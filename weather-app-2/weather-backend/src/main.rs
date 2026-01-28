mod api;
mod cache;
mod handlers;
mod models;
mod storage;

use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{Router, routing::post};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::cache::{CacheService, RedisCache};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenvy::dotenv()?;

    let redis_cache = RedisCache::new("redis://127.0.0.1/").expect("Failed to create redis client");
    log::info!(
        "Redis cache keys: {}",
        redis_cache.get_all_keys().await?.join(", ")
    );

    let cache = Arc::new(CacheService::new(redis_cache));

    let static_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
    let router = Router::new()
        .route("/api/weather", post(handlers::get_current_temperature))
        .nest_service("/static", ServeDir::new(static_path))
        .with_state(cache);

    let listener = TcpListener::bind("0.0.0.0:3001").await?;

    axum::serve(listener, router).await?;

    Ok(())
}
