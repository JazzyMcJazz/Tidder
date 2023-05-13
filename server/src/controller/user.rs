use actix_web::{ HttpResponse, get, HttpRequest};
use serde_json::json;

use crate::DbPool;
use crate::service::{security, database};

#[get("/api/user/me")]
pub async fn get_self(pool: DbPool, req: HttpRequest) -> HttpResponse {
    let (user_id, _) = match security::verify_user(&req) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "Unauthorized" })),
    };

    let user = match database::find_user_by_id(pool, user_id).await {
        Ok(user) => user,
        Err(e) => return HttpResponse::InternalServerError().json(json!({ "status": "error", "message": e.to_string() })),
    };

    HttpResponse::Ok().json(json!({ "user": {
        "id": user.id,
        "username": user.username,
        "avatar_url": user.avatar_url,
    }}))
}