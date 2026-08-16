use std::{fs, path::PathBuf};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::Path,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use deadpool_sqlite::Config;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Deserialize, Serialize)]
struct URL {
    id: u64,
    url: String,
    short_url: String,
    created_at: String,
    updated_at: String,
    accessed_at: String,
    access_count: u64,
}

#[derive(Deserialize, Serialize)]
struct CreateUrlRequest {
    url: String,
}

type DB = deadpool_sqlite::Pool;

trait ShortenerAPI {
    async fn create_url(url: String) -> Result<URL>;
    async fn get_url(url: String) -> Result<URL>;
    async fn update_url(url: String, short_url: String) -> Result<URL>;
    async fn delete_url(url: String) -> Result<URL>;
    async fn stats_url(url: String) -> Result<URL>;
    async fn list_urls() -> Result<Vec<URL>>;
}

async fn create_url_handler(Json(req): Json<CreateUrlRequest>) -> impl IntoResponse {}
async fn get_url_handler(Path(url): Path<String>) -> impl IntoResponse {}
async fn update_url_handler(
    Path(url): Path<String>,
    Json(req): Json<CreateUrlRequest>,
) -> impl IntoResponse {
}
async fn delete_url_handler(Path(url): Path<String>) -> impl IntoResponse {}
async fn stats_url_handler(Path(url): Path<String>) -> impl IntoResponse {}
async fn list_urls_handler() -> impl IntoResponse {}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("Starting URL shortener API...");

    let db_path = PathBuf::from("./db.db");
    if !db_path.exists() {
        fs::create_dir_all(db_path.parent().unwrap())?;
        fs::File::create(&db_path)?;
    }

    let db = Config::new(db_path);

    let app = Router::new()
        .route("/shorten", post(create_url_handler).get(list_urls_handler))
        .route(
            "/shorten/{url}",
            delete(delete_url_handler)
                .put(update_url_handler)
                .delete(delete_url_handler),
        )
        .route("/shorten/{url}/stats", get(stats_url_handler))
        .with_state(db);

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
