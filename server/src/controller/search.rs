use actix_web::{get, web::Query, HttpResponse};
use serde_json::json;
use crate::{service::elastic, model::api::QueryParams};

#[get("/api/search")]
pub async fn search(query: Query<QueryParams>) -> HttpResponse {

    let query = match &query.q {
        Some(query) => query,
        None => return HttpResponse::BadRequest().json(json!({ "status": "error", "message": "Missing query parameter" })),
    };

    let data = elastic::search(&query).await;
    
    match data {
        Ok(data) => HttpResponse::Ok().json(json!({ "categories": data.0, "posts": data.1 })),
        Err(msg) => HttpResponse::InternalServerError().json(json!({ "status": "error", "message": msg })),
    }
}