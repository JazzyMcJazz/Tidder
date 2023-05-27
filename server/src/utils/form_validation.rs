use actix_web::web;
use regex;

use crate::model::api::{CreatePostRequest, CreateCommentRequest};

pub fn validate_new_post(form: &web::Form<CreatePostRequest>) -> Result<(), &'static str> {
    // title regex
    let re_title = regex::Regex::new(r"^[a-zA-Z0-9_ ]+$").unwrap();
    
    // post length too short
    if form.body.len() < 10 {
        return Err("Post body must be at least 10 characters long");

    // post length too long
    } else if form.body.len() > 10000 {
        return Err("Post body can be at most 10000 characters long");
    
    // title contains invalid characters
    } else if !re_title.is_match(&form.title) {
        return Err("Title can only contain letters, numbers, spaces and underscores");

    // title length too short
    } else if form.title.len() < 5 {
        return Err("Title must be at least 5 characters long");

    // title length too long
    } else if form.title.len() > 100 {
        return Err("Title must be less than 100 characters long");

    // neither category_id nor new_category is specified
    } else if form.new_category.is_none() && form.category_id.is_none() {
        return Err("Category is required");

    // both category_id and new_category are specified
    } else if form.new_category.is_some() && form.category_id.is_some() {
        return Err("Cannot specify both category and new_category");

    // new_category length too short
    } else if form.new_category.is_some() && form.new_category.clone().unwrap().len() < 3 {
        return Err("New category name must be at least 3 characters long");
    
    // new_category length too long
    } else if form.new_category.is_some() && form.new_category.clone().unwrap().len() > 50 {
        return Err("New category name must be less than 50 characters long");
    
    // new_category contains invalid characters
    } else if form.new_category.is_some() && !re_title.is_match(&form.new_category.clone().unwrap()) {
        return Err("New category name can only contain letters, numbers, spaces and underscores");
    }

    Ok(())
}

pub fn validate_new_comment(form: &web::Form<CreateCommentRequest>) -> Result<(), &'static str> {
    // comment length too short
    if form.body.len() < 1 {
        return Err("Body must be at least 1 characters long");

    // comment length too long
    } else if form.body.len() > 10000 {
        return Err("Body must be less than 10000 characters long");
    }

    Ok(())
}