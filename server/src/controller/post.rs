use actix_web::{get, post, delete, web, HttpResponse, Responder, HttpRequest};
use serde_json::json;
use crate::DbPool;
use crate::model::api::{CreatePostRequest, CreateCommentRequest, QueryParams};
use crate::model::data::{Post, Comment};
use crate::service::security::verify_user;
use crate::service::{elastic, security, database};
use crate::utils::form_validation::{validate_new_post, validate_new_comment};

#[get("/api/post/popular")]
pub async fn get_popular_posts(query: web::Query<QueryParams>, req: HttpRequest) -> impl Responder {

    let show_all = security::will_show_all(query, &req);

    // Fetch the posts from the database and return a JSON response
    let data = elastic::get_posts(&show_all).await;
    
    match data {
        Ok(data) => HttpResponse::Ok().json(json!({ "posts": data })),
        Err(msg) => HttpResponse::InternalServerError().json(json!({ "status": "error", "message": msg })),
    }
}

#[get("/api/post/me")]
pub async fn get_own_posts(req: HttpRequest) -> impl Responder {

    let cookie = req.cookie("csrf").unwrap();
    println!("Cookie: {:?}", cookie);

    // XXX: Bad Practice! Should be moved to a middleware
    let (user_id, _) = match security::verify_user(&req) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "Unauthorized" })),
    };

    let posts = match elastic::get_posts_by_user_id(user_id.clone()).await {
        Ok(posts) => posts,
        Err(_) => return HttpResponse::InternalServerError().json(json!({ "status": "error", "message": "Failed to fetch posts" })),
    };

    HttpResponse::Ok().json(json!({ "posts": posts }))
}

#[get("/api/post/{id}")]
pub async fn get_post_by_id(id: web::Path<String>, req: HttpRequest, query: web::Query<QueryParams>) -> impl Responder {

    // Convert the id to an integer
    let id = match id.parse::<String>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(json!({ "status": "error", "message": "Post not found" })),
    };
    
    let show_all = security::will_show_all(query, &req);

    let post = match elastic::get_post_by_id(id, &show_all).await {
        Ok(post) => post,
        Err(_) => return HttpResponse::InternalServerError().json(json!({ "status": "error", "message": "Failed to fetch post" })),
    };

    // only post author and admins can view unpublished posts
    if !post.published {
        match verify_user(&req) {
            Ok((user_id, role)) => {
                if !post.author_id.eq(&user_id) && !role.eq("admin")  {
                    return HttpResponse::NotFound().json(json!({ "status": "error", "message": "Not Found" }));
                }
            },
            Err(_) => {
                return HttpResponse::NotFound().json(json!({ "status": "error", "message": "Not Found" }));
            }
        }
    }
    
    let category = match elastic::get_category_by_id(&post.category_id).await {
        Ok(category) => category,
        Err(msg) => return HttpResponse::InternalServerError().json(json!({ "status": "error", "message": msg })),
    };

    HttpResponse::Ok().json(json!({ "category": category, "post": post }))

}

#[get("/api/post/{id}/comment")]
pub async fn get_comments_by_post_id(id: web::Path<String>, query: web::Query<QueryParams>, req: HttpRequest) -> impl Responder {

    let show_all = security::will_show_all(query, &req);

    let comments = elastic::get_comments_by_post_id(id.to_string(), &show_all).await;

    HttpResponse::Ok().json(json!({ "comments": comments }))
}

#[post("/api/post")]
pub async fn create_post(pool: DbPool, form: web::Form<CreatePostRequest>, query: web::Query<QueryParams>, req: HttpRequest) -> HttpResponse {
    
    // XXX: Bad Practice! Should be moved to a middleware
    let (user_id, _) = match security::verify_user(&req) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "Unauthorized" })),
    };

    let user = match database::find_user_by_id(pool, user_id.clone()).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "User not found" })),
    };

    // Validate the form
    match validate_new_post(&form) {
        Ok(_) => (),
        Err(msg) => return HttpResponse::BadRequest().json(json!({ "status": "error", "message": msg })),
    }

    // Get the category or create a new one
    let category = match (&form.new_category, &form.category_id) {
        (Some(new_category), _) => match elastic::index_category(new_category.clone()).await {
            Ok(category) => category,
            Err(msg) => return HttpResponse::InternalServerError().json(json!({ "status": "error", "message": msg })),
        },
        (_, Some(category_id)) => match elastic::get_category_by_id(category_id).await {
            Ok(category) => category,
            Err(msg) => return HttpResponse::InternalServerError().json(json!({ "status": "error", "message": msg })),
        },
        _ => return HttpResponse::InternalServerError().json(json!({ "status": "error", "message": "Failed to create category" })),
    };

    let now = chrono::Utc::now().to_rfc3339();
    let is_draft = query.draft.unwrap_or(false);

    let post = Post {
        id: None,
        author_id: user_id,
        author_name: user.username,
        category_id: category.id,
        category_name: category.name,
        title: form.title.clone(),
        body: form.body.clone(),
        upvotes: 0,
        downvotes: 0,
        published: !is_draft,
        created_at: now.clone(),
        updated_at: now,
        deleted: false,
    };

    match elastic::index_post(post).await {
        Ok(post) => HttpResponse::Created().json(json!({ "category_id": post.category_id, "post_id": post.id })),
        Err(msg) => HttpResponse::InternalServerError().json(json!({ "status": "error", "message": msg })),
    }
}

#[post("api/post/{id}/publish")]
pub async fn publish_post(id: web::Path<String>, req: HttpRequest) -> HttpResponse {
    
    // XXX: Bad Practice! Should be moved to a middleware
    let (user_id, _) = match security::verify_user(&req) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "Unauthorized" })),
    };

    let post = match elastic::get_post_by_id(id.clone(), &false).await {
        Ok(post) => post,
        Err(_) => return HttpResponse::NotFound().json(json!({ "status": "error", "message": "Post not found" })),
    };

    // Check if the user the author of the post
    if !post.author_id.eq(&user_id) {
        return HttpResponse::Forbidden().json(json!({ "status": "error", "message": "Forbidden" }));
    }

    match elastic::publish_post(id.to_string()).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "success", "message": "Post published" })),
        Err(msg) => HttpResponse::InternalServerError().json(json!({ "status": "error", "message": msg })),
    }
}

#[post("/api/post/{id}/comment")]
pub async fn create_comment(pool: DbPool, id: web::Path<String>, form: web::Form<CreateCommentRequest>, req: HttpRequest) -> HttpResponse {
    
    // XXX: Bad Practice! Should be moved to a middleware
    let (user_id, _) = match security::verify_user(&req) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "Unauthorized" })),
    };

    let user = match database::find_user_by_id(pool, user_id.clone()).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "User not found" })),
    };

    // Validate the form
    match validate_new_comment(&form) {
        Ok(_) => (),
        Err(msg) => return HttpResponse::BadRequest().json(json!({ "status": "error", "message": msg })),
    }

    // Get the post
    let post = match elastic::get_post_by_id(id.clone(), &false).await {
        Ok(post) => post,
        Err(_) => return HttpResponse::NotFound().json(json!({ "status": "error", "message": "Post not found" })),
    };

    let now = chrono::Utc::now().to_rfc3339();
    let comment = Comment {
        id: None,
        author_id: user_id,
        author_name: user.username,
        post_id: post.id.unwrap(),
        body: form.body.clone(),
        deleted: false,
        upvotes: 0,
        downvotes: 0,
        created_at: now.clone(),
        updated_at: now,
    };

    match elastic::index_comment(comment).await {
        Ok(comment) => HttpResponse::Created().json(json!({ "post_id": comment.post_id, "comment_id": comment.id })),
        Err(msg) => HttpResponse::InternalServerError().json(json!({ "status": "error", "message": msg })),
    }
}

#[delete("/api/post/{id}")]
pub async fn unpublish_post(id: web::Path<String>, req: HttpRequest) -> HttpResponse {
    
    // XXX: Bad Practice! Should be moved to a middleware
    let (user_id, role) = match security::verify_user(&req) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "Unauthorized" })),
    };

    let post = match elastic::get_post_by_id(id.clone(), &false).await {
        Ok(post) => post,
        Err(_) => return HttpResponse::NotFound().json(json!({ "status": "error", "message": "Post not found" })),
    };

    // Check if the user is an admin or the author of the post
    if !role.eq("admin") && !post.author_id.eq(&user_id) {
        return HttpResponse::Forbidden().json(json!({ "status": "error", "message": "Forbidden" }));
    }

    match elastic::delete_post(elastic::Index::Post, id.clone()).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "success", "message": "Post deleted" })),
        Err(msg) => HttpResponse::InternalServerError().json(json!({ "status": "error", "message": msg })),
    }
}