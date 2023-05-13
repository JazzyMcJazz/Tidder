use actix_web::{delete, web, HttpResponse, Responder, HttpRequest};
use serde_json::json;
use crate::service::{elastic, security};


#[delete("/api/comment/{id}")]
pub async fn unpublish_comment(comment_id: web::Path<String>, req: HttpRequest) -> impl Responder {
    // XXX: Bad Practice! Should be moved to a middleware
    let (user_id, role) = match security::verify_user(&req) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "Unauthorized" })),
    };

    let comment = match elastic::get_comment_by_id(comment_id.to_string(), &false).await {
        Ok(comment) => comment,
        Err(msg) => return HttpResponse::NotFound().json(json!({ "status": "error", "message": msg })),
    };

    // Check if the user is the author of the post
    if !role.eq("admin") && !comment.author_id.eq(&user_id) {
        return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "Unauthorized" }));
    }

    // Delete the comment
    match elastic::delete_post(elastic::Index::Comment, comment_id.to_string()).await {
        Ok(_) => (),
        Err(msg) => return HttpResponse::InternalServerError().json(json!({ "status": "error", "message": msg })),
    };

    HttpResponse::Ok().json(json!({ "status": "ok" }))
}