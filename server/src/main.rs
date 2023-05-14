use actix_web::{App, HttpServer, http, web, delete, HttpResponse, middleware};
use actix_cors::Cors;
use actix_files as a_fs;
use controller::post::{get_own_posts, publish_post};
use http::header;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::fs;
use std::env;
use dotenv::dotenv;

// routes
use crate::controller::{
    auth::{
        pubkey, 
        register, 
        login, 
        logout
    },
    user::{
        get_self
    },
    category::{
        get_categories,
        get_category_by_id,
        get_posts_by_category_id,
    },
    post::{
        get_popular_posts, 
        get_post_by_id, 
        get_comments_by_post_id,
        create_post,
        create_comment,
        unpublish_post
    },
    comment::{
        unpublish_comment
    },
    search::{
        search
    },
    avatar::{
        upload_avatar,
        get_avatar_urls
    },
};

mod model;
mod controller;
mod service;
mod utils;

type DbPool = web::Data<Pool<SqliteConnectionManager>>;

#[delete("/api/flush")]
pub async fn flush() -> HttpResponse {
    service::elastic::flush_data().await;
    HttpResponse::Ok().into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
   
    let db_file = "db.db";
    
    // Create the database directory if it doesn't exist
    let db_dir = "database";
    if !fs::metadata(db_dir).is_ok() {
        fs::create_dir(db_dir).expect("Failed to create database directory");
    }

    // Create the tmp folder if it doesn't exist (used to store uploaded images temporarily while they are being processed)
    let tmp_dir = "tmp";
    if !fs::metadata(tmp_dir).is_ok() {
        fs::create_dir(tmp_dir).expect("Failed to create tmp directory");
    }

    // Create the avatar directory if it doesn't exist
    let avatar_dir = "public/avatar"; 
    if !fs::metadata(avatar_dir).is_ok() {
        fs::create_dir_all(avatar_dir).expect(format!("Failed to create {} directory", avatar_dir).as_str());
    }

    // Open a connection pool to the database
    let manager: SqliteConnectionManager = SqliteConnectionManager::file(format!("{}/{}", db_dir, db_file));
    let pool: Pool<SqliteConnectionManager> = r2d2::Pool::builder()
        .build(manager) 
        .expect("Failed to create pool.");

    // Initialize the database (create tables etc.)
    service::database::init(pool.clone()).await.unwrap();

    // Start the HTTP server
    HttpServer::new(move || {

        // CORS
        let cors = Cors::default()
            .allowed_origin(env::var("CLIENT_URL").expect("CLIENT_URL must be set").as_str())
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION, 
                header::ACCEPT, 
                header::CONTENT_TYPE,
                header::HeaderName::from_static("x-sveltekit-action"),
            ])
            .supports_credentials()
            .max_age(3600);

        // Security headers
        let security_headers = middleware::DefaultHeaders::new()
            .add((header::CACHE_CONTROL, "no-cache"))
            .add((header::CONTENT_SECURITY_POLICY, "frame-ancestors 'none'"))
            .add((header::CONTENT_TYPE, "application/json"))
            .add((header::STRICT_TRANSPORT_SECURITY, "max-age=31536000"))
            .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
            .add((header::X_FRAME_OPTIONS, "DENY"));

        // Routes
        App::new()
            .wrap(cors)
            .wrap(security_headers)
            .app_data(web::Data::new(pool.clone()))
            .service(a_fs::Files::new("/public", "./public").show_files_listing())
            .service(web::resource("/public/avatar/{filename}").name("avatars").route(web::get().to(|| HttpResponse::Ok())))
            .service(login)
            .service(register)
            .service(logout) 
            .service(pubkey) 
            .service(get_popular_posts)
            .service(get_own_posts)
            .service(get_post_by_id)
            .service(get_posts_by_category_id)
            .service(get_categories)
            .service(get_category_by_id)
            .service(get_comments_by_post_id)
            .service(get_avatar_urls)
            .service(search)
            .service(get_self)
            .service(create_post)
            .service(create_comment)
            .service(publish_post)
            .service(unpublish_post)
            .service(unpublish_comment)
            .service(upload_avatar)
            // .service(get_users) // Dev endpoint, remove in production
            // .service(flush) // Dev endpoint, remove in production
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}