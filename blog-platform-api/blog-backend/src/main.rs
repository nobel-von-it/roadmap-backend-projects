use std::path::PathBuf;

use anyhow::Result;
use axum::{
    Form, Router,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::{get, post},
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DBBlogPost {
    id: u64,
    title: String,
    content: String,
    category: String,
    tags: Vec<String>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlogPost {
    title: String,
    content: String,
    category: String,
    tags: Vec<String>,
}

async fn new_post(Form(new_post): Form<BlogPost>) -> impl IntoResponse {
    // id autoincrement
    // time in business logic (chrono)
    // TODO: "DB insert "
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn get_post(uri: Uri) -> impl IntoResponse {
    let (_, id) = uri.path().split_once("/").unwrap();
    let id: u64 = id.parse().unwrap();

    // TODO: "DB select where id = {id}"
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn get_all_posts() -> impl IntoResponse {
    // TODO: "DB select *"
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn put_post(uri: Uri, Form(put_post): Form<BlogPost>) -> impl IntoResponse {
    let (_, id) = uri.path().split_once("/").unwrap();
    let id: u64 = id.parse().unwrap();

    // TODO: "DB update where id = {id}"
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn delete_post(uri: Uri) -> impl IntoResponse {
    let (_, id) = uri.path().split_once("/").unwrap();
    let id: u64 = id.parse().unwrap();

    // TODO: "DB delete where id = {id}"
    StatusCode::INTERNAL_SERVER_ERROR
}

#[tokio::main]
async fn main() -> Result<()> {
    let static_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");

    let db = 0; // TODO: implement db service

    let router = Router::new()
        .route("/posts", post(new_post).get(get_all_posts))
        .route(
            "/posts/:id",
            get(get_post).put(put_post).delete(delete_post),
        )
        .nest_service("/", ServeDir::new(static_path))
        .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, router).await?;

    Ok(())
}
