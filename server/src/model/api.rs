use serde_derive::{Deserialize, Serialize};

use crate::model::data::Post;

//////////////////
// REQUEST DTOs //
//////////////////

#[derive(Debug, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}
 
#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub new_category: Option<String>,
    pub category_id: Option<String>,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub body: String,
}

//////////////////////
/// RESPONSES DTOs ///
//////////////////////

#[derive(Debug, Deserialize, Serialize)]
pub struct GetPostsResponse {
    pub posts: Vec<Post>,
}

// Other (should probably be moved to another file)

#[derive(Debug, Deserialize)]
pub struct QueryParams {
   pub q: Option<String>,
   pub show_all: Option<bool>,
   pub draft: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, 
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
    pub username: String,
    pub role: String,
}