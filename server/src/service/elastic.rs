use std::env;

use actix_web::{http::StatusCode};
use url::Url;
use serde_json::json;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use elasticsearch::{
    http::transport::{TransportBuilder, SingleNodeConnectionPool},
    params::Refresh,
    DeleteByQueryParts,
    Elasticsearch,
    SearchParts,
    UpdateParts,
    IndexParts,
    CountParts,
    GetParts,
};

use crate::model::data::{Category, Post, Comment};

pub enum Index {
    Post,
    Comment,
}

const CATEGORY_INDEX: &str = "tidder_category";
const COMMENT_INDEX: &str = "tidder_comment";
const POST_INDEX: &str = "tidder_post";

fn client() -> Elasticsearch {

    let url = env::var("ELASTIC_URL").expect("Missing ELASTIC_URL");
    let url = Url::parse(url.as_str()).expect("Invalid URL");
    let conn_pool = SingleNodeConnectionPool::new(url);

    let username = env::var("ELASTIC_USER").expect("Missing ELASTIC_USER");
    let password = env::var("ELASTIC_PASS").expect("Missing ELASTIC_PASS");
    let encoded = STANDARD.encode(format!("{}:{}", username, password));

    let header_name = elasticsearch::http::headers::AUTHORIZATION;
    let header_value = elasticsearch::http::headers::HeaderValue::from_str(
        &format!("Basic {}", encoded)
    ).expect("Invalid header value");

    let transport = TransportBuilder::new(conn_pool)
        .header(header_name, header_value)
        .disable_proxy()
        .build()
        .expect("Failed to create transport");

    Elasticsearch::new(transport)
}

pub async fn get_categories() -> Vec<Category> {

    let client = client();

    let response = client
        .search(SearchParts::Index(&[CATEGORY_INDEX]))
        .body(json!({
            "size": 10000,
            "query": {
                "match_all": {}
            },
        }))
        .send().await;
    
    let mut categories = match response {
        Ok(response) => {
            let body = response.json::<serde_json::Value>().await.unwrap();
            let categories: Vec<Category> = body.get("hits").unwrap().get("hits").unwrap().as_array().unwrap().iter().map(|hit| {
                let source = hit.get("_source").unwrap();
                let id = hit.get("_id").unwrap().as_str().unwrap().to_string();
                
                Category {
                    id,
                    name: source.get("name").unwrap().as_str().unwrap().to_string(),
                    posts: None,
                }
            }).collect();

            categories
        },
        _ => return vec![]
    };

    for category in categories.iter_mut() {
        let count = count_posts_by_category_id(category.id.to_string()).await;
        match count {
            Ok(count) => {
                category.posts = Some(count);
            },
            _ => return vec![]
        }
    }

    categories

}

pub async fn get_category_by_id(category_id: &String) -> Result<Category, &'static str> {
    let client = client();

    let response = client
        .get(GetParts::IndexId(CATEGORY_INDEX, category_id))
        .send().await;
    
    match response {
        Ok(response) => {
            let body = response.json::<serde_json::Value>().await.unwrap();
            let source = body.get("_source").unwrap();
            Ok(Category {
                id: category_id.to_string(),
                name: source.get("name").unwrap().as_str().unwrap().to_string(),
                posts: None,
            })
        },
        Err(_) => Err("Internal server error")
    }
}

pub async fn get_category_by_name(category_name: String) -> Result<Category, &'static str> {

    let client = client();

    let response = client
        .search(SearchParts::Index(&[CATEGORY_INDEX]))
        .body(json!({
            "query": {
                "match": {
                    "name": category_name
                }
            }
        }))
        .send().await;
    
    match response {
        Ok(response) => {
            let body = response.json::<serde_json::Value>().await.unwrap();
            let hits = body.get("hits").unwrap().get("hits").unwrap().as_array().unwrap();
            if hits.len() > 0 {
                let source = hits[0].get("_source").unwrap();
                let id = hits[0].get("_id").unwrap().as_str().unwrap().to_string();

                let category = Category {
                    id,
                    name: source.get("name").unwrap().as_str().unwrap().to_string(),
                    posts: None,
                };
                Ok(category)
            } else {
                Err("Not found")
            }
        },
        Err(_) => Err("Internal server error")
    }
}

pub async fn count_posts_by_category_id(category_id: String) -> Result<u64, &'static str> {
    
        let client = client();
    
        let response = client
            .count(CountParts::Index(&[POST_INDEX]))
            .body(json!({
                "query": {
                    "bool": {
                        "must": [
                            { "match": { "category_id": category_id } },
                            { "match": { "published": true } },
                            { "match": { "deleted": false } }
                        ]
                    }
                }
            }))
            .send()
            .await;
        
        match response {
            Ok(response) => {
                let body = response.json::<serde_json::Value>().await.unwrap();
                let count = body.get("count").unwrap().as_u64().unwrap();
                Ok(count)
            },
            Err(_) => Err("Internal server error")
        }
}

pub async fn index_category(category_name: String) -> Result<Category, &'static str> {
    
        let client = client();

        // check if category exists
        let category = get_category_by_name(category_name.clone()).await;

        match category {
            Ok(_) => return Err("Category already exists"),
            _ => {}
        }
    
        let response = client
            .index(IndexParts::IndexId(CATEGORY_INDEX, ""))
            .body(json!({ "name": category_name }))
            .refresh(Refresh::True)
            .send().await;
    
        match response {
            Ok(response) => {
                let body = response.json::<serde_json::Value>().await.unwrap();
                Ok(Category {
                    id: body.get("_id").unwrap().as_str().unwrap().to_string(),
                    name: category_name,
                    posts: None,
                })
            },
            Err(_) => Err("Internal server error")
        }
}

pub async fn index_post(post: Post) -> Result<Post, &'static str> {

    let client = client();

    let response = client
        .index(IndexParts::IndexId(POST_INDEX, ""))
        .body(json!(post))
        .refresh(Refresh::True)
        .send().await;

    match response {
        Ok(response) => {
            let body = response.json::<serde_json::Value>().await.unwrap();
            let id = body.get("_id").unwrap().as_str().unwrap().to_string();
            Ok(Post {
                id: Some(id),
                ..post
            })
        },
        Err(_) => Err("Internal server error")
    }
}

pub async fn publish_post(post_id: String) -> Result<(), &'static str> {

    let client = client();

    let response = client
        .update(UpdateParts::IndexId(POST_INDEX, &post_id))
        .body(json!({
            "doc": {
                "published": true
            }
        }))
        .send().await;

    match response {
        Ok(_) => Ok(()),
        Err(_) => Err("Internal server error")
    }
}

pub async fn delete_post(index: Index, post_id: String) -> Result<(), &'static str> {

    let client = client();

    let index = match index {
        Index::Post => POST_INDEX,
        Index::Comment => COMMENT_INDEX,
    };

    let response = client
        .update(UpdateParts::IndexId(index, &post_id))
        .body(json!({
            "doc": {
                "deleted": true
            }
        }))
        .send().await;

    match response {
        Ok(_) => Ok(()),
        Err(_) => Err("Internal server error")
    }
}

pub async fn index_comment(comment: Comment) -> Result<Comment, &'static str> {

    let client = client();

    let response = client
        .index(IndexParts::IndexId(COMMENT_INDEX, ""))
        .body(json!(comment))
        .refresh(Refresh::True)
        .send().await;

    match response {
        Ok(response) => {
            let body = response.json::<serde_json::Value>().await.unwrap();
            let id = body.get("_id").unwrap().as_str().unwrap().to_string();
            Ok(Comment {
                id: Some(id),
                ..comment
            })
        },
        Err(_) => Err("Internal server error")
    }
}

pub async fn search(query: &String) -> Result<(Vec<Category>, Vec<Post>), &'static str> {
    
        let client = client();
    
        let response = client
            .search(SearchParts::Index(&[POST_INDEX, CATEGORY_INDEX]))
            .body(json!({
                "size": 10000,
                "query": {
                    "multi_match": {
                        "query": query,

                        "fields": [
                            "title", 
                            "body",
                            "author_name",
                            "name",
                        ],
                        "fuzziness": "2",
                    }
                },
            }))
            .send().await;
    
        match response {
            Ok(response) => {
                let body = response.json::<serde_json::Value>().await.unwrap();
                let hits = body.get("hits").unwrap().get("hits").unwrap().as_array().unwrap();
                let mut categories = Vec::new();
                let mut posts = Vec::new();
                for hit in hits {
                    let source = hit.get("_source").unwrap();
                    let id = hit.get("_id").unwrap().as_str().unwrap().to_string();
    
                    if hit.get("_index").unwrap().as_str().unwrap() == "tidder_category" {
                        let category = Category {
                            id,
                            name: source.get("name").unwrap().as_str().unwrap().to_string(),
                            posts: None,
                        };
                        categories.push(category);
                        continue;
                    }
                    
                    let post = Post::from_json(hit).sanitize(&false);

                    if post.deleted || !post.published {
                        continue;
                    }

                    posts.push(post);
                }
    
                Ok((categories, posts))
            },
            Err(_) => Err("Internal server error")
        }
}

pub async fn flush_data() -> () {
    let client = client();

    let result = client
        .delete_by_query(DeleteByQueryParts::Index(&[POST_INDEX]))
        .body(json!({
            "query": {
                "match_all": {}
            }
        }))
        .send().await;

    // Just to mute compiler warnings
    if result.is_err() {
        return ();
    }

    let result = client
        .delete_by_query(DeleteByQueryParts::Index(&[CATEGORY_INDEX]))
        .body(json!({
            "query": {
                "match_all": {}
            }
        }))
        .send().await;

    // Just to mute compiler warnings
    if result.is_err() {
        return ();
    }

    let result = client
        .delete_by_query(DeleteByQueryParts::Index(&[COMMENT_INDEX]))
        .body(json!({
            "query": {
                "match_all": {}
            }
        }))
        .send().await;
    
    // Just to mute compiler warnings
    match result {
        Ok(_) => (),
        Err(_) => ()
    }
}


///NOTE:########################################################//
// NOTE:# Sanitize text body in all functions below this point #//
///NOTE:########################################################//

pub async fn get_posts_by_category_id(category_id: String, show_all: &bool) -> Vec<Post> {

    let client = client();

    // If show_all is true, we don't need to match by deleted
    let query = if *show_all {
        json!([
            { "match": { "category_id": category_id } },
        ])
    } else {
        json!([
            { "match": { "category_id": category_id } },
            { "match": { "published": true } },
            { "match": { "deleted": false } }
        ])
    };

    let response = client
        .search(SearchParts::Index(&[POST_INDEX]))
        .body(json!({
            "size": 10000,
            "sort": [
                { "upvotes": "desc" },
                { "downvotes": "asc" },
                { "created_at": "desc" },
            ],
            "query": {
                // match by category_id and published
                "bool": {
                    "must": query
                }
            }
        }))
        .send().await;
    
    match response {
        Ok(response) => {

            let posts = response.json::<serde_json::Value>().await.unwrap();
            let posts: Vec<Post> = posts.get("hits").unwrap().get("hits").unwrap().as_array().unwrap().iter().map(|hit| {
                Post::from_json(hit).sanitize(show_all)
            }).collect();

            posts
        },
        _ => return vec![]
    }
}

pub async fn get_posts(show_all: &bool) -> Result<Vec<Post>, &'static str> {
    
        let client = client();

        let query = if *show_all {
            json!({
                "match_all": {}
            })
        } else {
            json!({
                "bool": {
                    "must": [
                        { "match": { "deleted": false } },
                        { "match": { "published": true } },
                    ]
                }
            })
        };
    
        let response = client
            .search(SearchParts::Index(&[POST_INDEX]))
            .body(json!({
                "size": 10000,
                "sort": [
                    { "upvotes": "desc" },
                    { "downvotes": "asc" },
                    { "created_at": "desc" },
                ],
                "query": query,
            }))
            .send().await;
        
        match response {
            Ok(response) => {

                let body = response.json::<serde_json::Value>().await.unwrap();
                let posts: Vec<Post> = body.get("hits").unwrap().get("hits").unwrap().as_array().unwrap().iter().map(|hit| {
                    Post::from_json(hit).sanitize(show_all)
                }).collect();
    
                Ok(posts)
            },
            Err(_) => Err("Internal server error")
        }
}

pub async fn get_post_by_id(id: String, show_all: &bool) -> Result<Post, (StatusCode, &'static str)> {

    let client = client();

    let response = client
        .get(GetParts::IndexId(POST_INDEX, &id))
        .send().await;
    
    match response {
        Ok(response) => {
            let source = response.json::<serde_json::Value>().await.unwrap();
            Ok(Post::from_json(&source).sanitize(show_all))
        },
        Err(_) => Err((StatusCode::NOT_FOUND, "Not Found"))
    }
}

pub async fn get_posts_by_user_id(user_id: String) -> Result<Vec<Post>, &'static str> {
    
    let client = client();

    let response = client
        .search(SearchParts::Index(&[POST_INDEX]))
        .body(json!({
            "size": 10000,
            "sort": [{ "created_at": "desc" }],
            "query": {
                "match": {
                    "author_id": user_id
                }
            }
        }))
        .send().await;
    
    match response {
        Ok(response) => {

            let body = response.json::<serde_json::Value>().await.unwrap();
            let posts: Vec<Post> = body.get("hits").unwrap().get("hits").unwrap().as_array().unwrap().iter().map(|hit| {
                Post::from_json(hit).sanitize(&false)
            }).collect();

            Ok(posts)
        },
        Err(_) => Err("Internal server error")
    }
}

pub async fn get_comments_by_post_id(post_id: String, show_all: &bool) -> Vec<Comment> {
    
    let client = client();

    let response = client
        .search(SearchParts::Index(&[COMMENT_INDEX]))
        .body(json!({
            "size": 10000,
            "sort": [
                { "published": "desc"},
                { "upvotes": "desc" },
                { "downvotes": "asc" },
                { "created_at": "asc" },
            ],
            "query": {
                "match": {
                    "post_id": post_id
                }
            }
        }))
        .send().await;
    
    match response {
        Ok(response) => {

            let body = response.json::<serde_json::Value>().await.unwrap();
            let comments: Vec<Comment> = body.get("hits").unwrap().get("hits").unwrap().as_array().unwrap().iter().map(|hit| {
                Comment::from_json(hit).sanitize(show_all)
            }).collect();

            comments
        },
        _ => vec![]
    }
}

pub async fn get_comment_by_id(comment_id: String, show_all: &bool) -> Result<Comment, &'static str> {
    
        let client = client();
    
        let response = client
            .get(GetParts::IndexId(COMMENT_INDEX, &comment_id))
            .send().await;
        
        match response {
            Ok(response) => {
                let source = response.json::<serde_json::Value>().await.unwrap();
                Ok(Comment::from_json(&source).sanitize(show_all))
            },
            Err(_) => Err("Not Found")
        }
}