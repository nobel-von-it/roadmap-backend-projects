mod api;
mod cache;
mod handlers;
mod models;
mod storage;

use std::sync::{Arc, RwLock};

use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::cache::CacheService;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let cache = Arc::new(RwLock::new(CacheService::new()));

    let router = Router::new()
        .route("/", get(handlers::get_homepage))
        .route("/api/weather", post(handlers::get_current_temperature))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(cache);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, router).await?;

    Ok(())
}
