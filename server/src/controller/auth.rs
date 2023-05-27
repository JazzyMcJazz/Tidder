use std::env;

use actix_web::{ HttpResponse, get, post, cookie, web};
use regex::{Regex, RegexSet};
use serde_json::json;
use serde_derive::{Serialize};
use bcrypt::{DEFAULT_COST, hash};

use crate::DbPool;
use crate::model::api::UserRequest;
use crate::service::{security, database};

#[derive(Debug, Serialize)]
struct Claims {
    sub: String, 
    iss: String,
    exp: usize,
    iat: usize,
}

#[get("/api/pubkey")]
pub async fn pubkey() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(include_str!("../security/keystore/pubkey.pem"))
}

#[get("/api/users")]
pub async fn get_users(pool: DbPool) -> HttpResponse {
    let users = match database::find_users(pool).await {
        Ok(users) => users,
        Err(e) => return HttpResponse::InternalServerError().json(json!({ "status": "error", "message": e.to_string() })),
    };

    HttpResponse::Ok().json(json!({ "users": users }))
}

#[post("/api/register")]
pub async fn register(pool: DbPool, form: web::Form<UserRequest>) -> HttpResponse {

    let re_username = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    let re_password = RegexSet::new(&[r"[A-Z]", r"[a-z]", r"\d", r#"[!@#$%^&*(),.?\":{}|<>]"#]).unwrap();

    // Validate username
    let username = match re_username.is_match(&form.username) {
        true => &form.username,
        false => return HttpResponse::BadRequest().json(json!({ "status": "error", "message": "Username can only contain letters, numbers and underscores" })),
    };
    
    // Validate and hash password
    let password = match 
        &form.password.len() >= &8 && 
        &form.password.len() <= &64 &&
        re_password.is_match(&form.password) {
            true => hash(&form.password, DEFAULT_COST).expect("Failed to hash password"),
            false => return HttpResponse::BadRequest().json(json!({ "status": "error", "message": "Password must be between 8 and 64 characters long and contain at least one uppercase letter, one lowercase letter, one digit and one special character" })),
    };

    let user_id = match database::save_user(pool.clone(), username.to_string(), password).await {
        Ok(id) => id,
        Err(e) => {
            if e.eq("UNIQUE constraint failed: users.username") || e.eq("UNIQUE constraint failed: users.username_lower") {
                return HttpResponse::Conflict().json(json!({ "status": "error", "message": "Username already exists" }));
            } else {
                eprintln!("Error saving user: {}", e);
                return HttpResponse::InternalServerError().json(json!({ "status": "error", "message": "Something went wrong" }));
            }
        }
    }; 
    
    let token = match security::login(pool, &form.into_inner()).await {
        Ok(token) => token,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "Invalid credentials" })),
    };

    // Create a secure cookie with the JWT token
    let cookie = cookie::Cookie::build("identity", &token)
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(cookie::SameSite::Strict)
        .max_age(cookie::time::Duration::days(365))
        .finish();

    HttpResponse::Created()
        .cookie(cookie)
        .json(json!({ "status": "ok", "user_id": user_id }))
    
}

#[post("/api/login")]
pub async fn login(pool: DbPool, form: web::Form<UserRequest>) -> HttpResponse {

    let form = form.into_inner();

    let token = match security::login(pool, &form).await {
        Ok(token) => token,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "Invalid credentials" })),
    };

    // Create a secure cookie with the JWT token
    let cookie = cookie::Cookie::build("identity", &token)
        .domain(env::var("COOKIE_DOMAIN").expect("COOKIE_DOMAIN not set"))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(cookie::SameSite::Strict)
        .max_age(cookie::time::Duration::days(365))
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({ "status": "ok", "username": form.username }))
}

#[post("/api/logout")]
pub async fn logout() -> HttpResponse {
    // Clear the cookie
    let identity_cookie = cookie::Cookie::build("identity", "")
        .domain(env::var("COOKIE_DOMAIN").expect("COOKIE_DOMAIN not set"))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(cookie::SameSite::Strict)
        .max_age(cookie::time::Duration::MIN)
        .finish();

    HttpResponse::Ok()
        .cookie(identity_cookie)
        .json(json!({ "status": "ok" }))
}