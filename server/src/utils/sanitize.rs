use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use comrak::plugins::syntect::SyntectAdapter;

use crate::model::data::{Post, Comment};

const CODE_BLOCK_THEME: &str = "base16-eighties.dark";
const DELETED_TEXT: &str = "<p class=\"text-red-500\">deleted</p>";

pub fn sanitize_post(post: &mut Post, show_all: &bool) -> Post {
    
    // sanitize and convert markdown to html
    let adapter = SyntectAdapter::new(CODE_BLOCK_THEME);
    let options = ComrakOptions::default();
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);
    
    // If the post is deleted, replace the title and body with "deleted"
    if !show_all && post.deleted {
        post.title = DELETED_TEXT.to_string();
        post.body = DELETED_TEXT.to_string();
    } else {
        post.body = markdown_to_html_with_plugins(&post.body, &options, &plugins);
    }

    // If the entire body was sanitized away, replace it with "deleted"
    if post.body.trim().starts_with("<!-- raw HTML omitted -->") {
        post.body = DELETED_TEXT.to_string();
    }

    post.clone()
}

pub fn sanitize_comment(comment: &mut Comment, show_all: &bool) -> Comment {
    
    // convert markdown to html and sanitize
    let adapter = SyntectAdapter::new(CODE_BLOCK_THEME);
    let options = ComrakOptions::default();
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);
    
    if !show_all && comment.deleted {
        comment.body = DELETED_TEXT.to_string();
    } else {
        comment.body = markdown_to_html_with_plugins(&comment.body, &options, &plugins);
    }

    // If the entire body was sanitized away, replace it with "deleted"
    if comment.body.trim().starts_with("<!-- raw HTML omitted -->") {
        comment.body = DELETED_TEXT.to_string();
    }

    comment.clone()
}