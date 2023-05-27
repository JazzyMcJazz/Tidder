use crate::DbPool;
use crate::model::api::{Claims, UserRequest, QueryParams};
use crate::service::database;
use actix_web::HttpRequest;
use actix_web::web::Query;
use actix_multipart::form::tempfile::TempFile;
use jsonwebtoken::{encode, Header, DecodingKey, EncodingKey, Algorithm};
use chrono;
use bcrypt::verify;
use image::io::Reader;
use std::fs;
use std::io::Cursor;


pub async fn login(pool: DbPool, login: &UserRequest) -> Result<String, String> {

    
    let user = match database::find_user_by_username(pool, login.username.to_string()).await {
        Ok(user) => user,
        Err(e) => return Err(e.to_string()),
    };
    
    let password_matches = match verify(&login.password, &user.password) {
        Ok(password_matches) => password_matches,
        Err(_) => return Err("Invalid credentials".to_string()),
    };

    if !password_matches {
        return Err("Invalid credentials".to_string());
    }
    
    // Token claims (payload)
    let claims = Claims {
        sub: user.id.to_string(),
        iss: "tidders".to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::days(365)).timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
        username: user.username.to_string(),
        role: user.role,
    };
    
    // Create a JWT token
    let token = encode(
        &Header::new(Algorithm::RS256), 
        &claims, 
        &EncodingKey::from_rsa_pem(include_bytes!("../security/keystore/privkey.pem"))
            .expect("Failed to create JWT")
    ).expect("Failed to create JWT");

    Ok(token)
}

pub fn verify_user(req: &HttpRequest) -> Result<(String, String), &'static str> {
    // Get the cookie from the request
    let cookie = match req.cookie("identity") {
        Some(cookie) => cookie,
        None => return Err("Unauthorized"),
    };

    // Verify the JWT token
    let token = cookie.value();
    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_rsa_pem(include_bytes!("../security/keystore/pubkey.pem"))
            .expect("Failed to read public key"),
        &jsonwebtoken::Validation::new(Algorithm::RS256),
    );

    match token_data {
        Ok(token_data) => {
            // Verify the CSRF token
            let csrf = match req.cookie("csrf") {
                Some(csrf) => csrf,
                None => return Err("Unauthorized"),
            };

            if csrf.value() != token {
                return Err("Unauthorized");
            }

            Ok((token_data.claims.sub, token_data.claims.role))
        },
        Err(_) => Err("Unauthorized"),
    }
}

/**
 * Check if the show_all query parameter is set to true and return true if the user is an admin
 * @param query The query parameters
 * @param req The HTTP request
 */
pub fn will_show_all(query: Query<QueryParams>, req: &HttpRequest) -> bool {
    match query.show_all.unwrap_or(false) {
        true => match verify_user(req) {
            Ok((_, role)) => match role.eq("admin") {
                true => true,
                false => false,
            },
            Err(_) => false,
        },
        false => false,
    }
}

pub fn validate_image(image: TempFile, user_id: String) -> Result<(String, Vec<u8>), String> {
    
    // Validate file size
    if image.size > 5_000_000 {
        return Err("File size must be less than 5MB".to_string());
    }

    // Validate file name
    let re = regex::Regex::new(r"^[a-zA-Z0-9_]+\.(png|jpg|jpeg|gif)$").unwrap();
    if !re.is_match(&image.file_name.unwrap()) {
        return Err("Illegal file name".to_string());
    };

    // Save image in tmp directory so it can be processed as bytes.
    // It is saved with no extension so that the image library can guess the format
    let tmp_path = format!("tmp/{}", user_id);
    image.file.persist(&tmp_path).unwrap();
    
    // Read image from tmp directory
    let img = fs::read(&tmp_path).unwrap();
    let reader = Reader::new(Cursor::new(&img)).with_guessed_format().unwrap();
    
    // Validate mime type
    let extension = match reader.format() {
        Some(extension) => match extension {
            image::ImageFormat::Png => "png",
            image::ImageFormat::Jpeg => "jpeg",
            image::ImageFormat::Gif => "gif",
            _ => return {
                fs::remove_file(&tmp_path).unwrap();
                Err("Image must be a png, jpg or gif".to_string())
            },
        },
        None => {
            fs::remove_file(&tmp_path).unwrap();
            return Err("Image must be a png, jpg or gif".to_string())
        },
    };

    // Delete the image from the tmp directory
    fs::remove_file(&tmp_path).unwrap();

    // Add the extension to the file
    let filename = format!("{}.{}", user_id, extension);

    Ok((filename, img))
}