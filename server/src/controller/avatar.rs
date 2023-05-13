use std::fs;

use actix_multipart::{
    form::{
        tempfile::TempFile,
        MultipartForm,
    },
    
};
use actix_web::{post, HttpResponse, HttpRequest, web::Query, get};
use serde_derive::Deserialize;
use serde_json::json;

use crate::{service::{security, database}, DbPool, utils::convert};
// use crate::utils::convert;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    file: TempFile,
}

#[post("/api/upload/avatar")]
async fn upload_avatar(pool: DbPool, MultipartForm(form): MultipartForm<UploadForm>, req: HttpRequest) -> HttpResponse {

    let user_id = match security::verify_user(&req) {
        Ok((user_id, _)) => user_id,
        Err(_) => return HttpResponse::Unauthorized().json(json!({ "status": "error", "message": "Unauthorized" })),
    };

    let filename = uuid::Uuid::new_v4().to_string();
    let file = form.file;
    
    let (filename, file) = match security::validate_image(file, filename) {
        Ok(data) => data,
        Err(msg) => return HttpResponse::BadRequest().json(json!({ "status": "error", "message": msg })),
    };
    
    
    // save the new avatar to the avatar directory
    let avatar_dir = "public/avatar";
    let avatar_path = format!("{}/{}", avatar_dir, &filename);
    fs::write(&avatar_path, file).unwrap();

    // delete the old avatar if it wasn't overwritten
    let old_avatar = database::find_user_by_id(pool.clone(), user_id.clone()).await.expect("Failed to fetch user").avatar_url;
    match old_avatar {
        Some(old_avatar) => {
            let old_avatar = old_avatar.split("/").last().unwrap();
            let old_avatar_path = format!("{}/{}", avatar_dir, old_avatar);
            if old_avatar_path != avatar_path {
                fs::remove_file(old_avatar_path).unwrap();
            }
        },
        None => (),
    }

    // generate the url for the avatar   
    let avatar_url = req.url_for("avatars", [format!("{}", filename)]).unwrap();
    let avatar_url = convert::url_to_string(avatar_url);
    database::update_user_avatar(pool, user_id, avatar_url).await.expect("Failed to update user avatar");

    HttpResponse::Ok().into()
}

#[derive(Debug, Deserialize)]
struct AvatarQuery {
    user_ids: String,
}

#[get("/api/avatar")]
async fn get_avatar_urls(pool: DbPool, query: Query<AvatarQuery>) -> HttpResponse {
    
    let user_ids = query.user_ids.split(",");
    let user_ids = user_ids.map(|id| id.parse::<String>().unwrap()).collect::<Vec<_>>();
    let urls = database::find_avatars_by_user_ids(pool, user_ids).await.expect("Failed to fetch avatar urls");
    let urls = urls.into_iter().map(|(user_id, avatar_url)| {
        json!({
            "user_id": user_id,
            "avatar_url": avatar_url,
        })
    }).collect::<Vec<_>>();

    HttpResponse::Ok().json(json!({ "urls": urls }))
}