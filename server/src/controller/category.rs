use actix_web::{get, HttpResponse, Responder, web, HttpRequest};
use serde_json::json;

use crate::{service::{elastic, security}, model::api::QueryParams};

#[get("/api/category")]
pub async fn get_categories() -> impl Responder {
    let categories = elastic::get_categories().await;
    HttpResponse::Ok().json(json!({ "categories": categories }))
}

#[get("/api/category/{category_id}")]
pub async fn get_category_by_id(category_id: web::Path<String>) -> impl Responder {
   let category_id = match category_id.parse::<String>() {
         Ok(category_id) => category_id,
         Err(_) => return HttpResponse::BadRequest().json(json!({ "status": "error", "message": "Category not found" })),
   };

   match elastic::get_category_by_id(&category_id).await {
      Ok(category) => HttpResponse::Ok().json(json!({ "category": category })),
      Err(_) => HttpResponse::InternalServerError().json(json!({ "status": "error", "message": "Failed to fetch category" })),
   }
}

#[get("/api/category/{id}/posts")]
pub async fn get_posts_by_category_id(id: web::Path<String>, query: web::Query<QueryParams>, req: HttpRequest) -> impl Responder {
    // Convert the id to an integer
    let id = match id.parse::<String>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json(json!({ "status": "error", "message": "Category not found" })),
    };

    let is_admin = match security::verify_user(&req) {
        Ok((_, role)) => match role.eq("admin") {
            true => true,
            false => false,
        },
        Err(_) => false,
    };

    let show_all = is_admin && query.show_all.unwrap_or(false);

    let posts = elastic::get_posts_by_category_id(id, &show_all).await;

    HttpResponse::Ok().json(json!({ "posts": posts }))
}