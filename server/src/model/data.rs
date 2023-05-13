use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use r2d2_sqlite::rusqlite::Row;

use crate::utils::sanitize::{sanitize_post, sanitize_comment};

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub posts: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: Option<String>,
    pub author_name: String,
    pub author_id: String,
    pub category_id: String,
    pub category_name: String,
    pub title: String,
    pub body: String,
    pub upvotes: u32,
    pub downvotes: u32,
    pub published: bool,
    pub deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Post {
    pub fn from_json(source: &Value) -> Post {
        let id = source.get("_id").unwrap().as_str().unwrap().to_string();
        let source = source.get("_source").unwrap();

        Post {
            id: Some(id),
            title: source.get("title").unwrap().as_str().unwrap().to_string(),
            body: source.get("body").unwrap().as_str().unwrap().to_string(),
            author_name: source.get("author_name").unwrap().as_str().unwrap().to_string(),
            author_id: source.get("author_id").unwrap().as_str().unwrap().to_string(),
            category_id: source.get("category_id").unwrap().as_str().unwrap().to_string(),
            category_name: source.get("category_name").unwrap().as_str().unwrap().to_string(),
            created_at: source.get("created_at").unwrap().as_str().unwrap().to_string(),
            updated_at: source.get("updated_at").unwrap().as_str().unwrap().to_string(),
            upvotes: source.get("upvotes").unwrap().as_u64().unwrap() as u32,
            downvotes: source.get("downvotes").unwrap().as_u64().unwrap() as u32,
            published: source.get("published").unwrap().as_bool().unwrap(),
            deleted: source.get("deleted").unwrap().as_bool().unwrap(),
        }
    }
    pub fn sanitize(&mut self, show_all: &bool) -> Post {
        sanitize_post(self, show_all)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Comment {
    pub id: Option<String>,
    pub author_name: String,
    pub author_id: String,
    pub post_id: String,
    pub body: String,
    pub deleted: bool,
    pub upvotes: u32,
    pub downvotes: u32,
    pub created_at: String,
    pub updated_at: String,
}

impl Comment {
    pub fn from_json(source: &Value) -> Comment {
        let id = source.get("_id").unwrap().as_str().unwrap().to_string();
        let source = source.get("_source").unwrap();

        Comment {
            id: Some(id),
            author_name: source.get("author_name").unwrap().as_str().unwrap().to_string(),
            author_id: source.get("author_id").unwrap().as_str().unwrap().to_string(),
            post_id: source.get("post_id").unwrap().as_str().unwrap().to_string(),
            body: source.get("body").unwrap().as_str().unwrap().to_string(),
            upvotes: source.get("upvotes").unwrap().as_u64().unwrap() as u32,
            downvotes: source.get("downvotes").unwrap().as_u64().unwrap() as u32,
            created_at: source.get("created_at").unwrap().as_str().unwrap().to_string(),
            updated_at: source.get("updated_at").unwrap().as_str().unwrap().to_string(),
            deleted: source.get("deleted").unwrap().as_bool().unwrap(),
        }
    }
    pub fn sanitize(&mut self, show_all: &bool) -> Comment {
        sanitize_comment(self, show_all)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub username_lower: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub created_at: String,
    pub role: String,
}

impl User {
    pub fn from_db(row: &Row) -> User {
        User {
            id: row.get(0).unwrap(),
            username: row.get(1).unwrap(),
            username_lower: row.get(2).unwrap(),
            password: row.get(3).unwrap(),
            avatar_url: row.get(4).unwrap_or(None),
            created_at: row.get(5).unwrap(),
            role: row.get(6).unwrap(),
        }
    }
}
