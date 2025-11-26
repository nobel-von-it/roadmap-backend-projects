mod handlers;
mod html;
mod models;
mod utils;

use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

use crate::html::HtmlManager;

#[tokio::main]
async fn main() -> Result<()> {
    let hm = match HtmlManager::load() {
        Ok(hm) => hm,
        Err(e) => {
            eprintln!("html manager load failed: {}", e);
            return Err(e);
        }
    };
    let app = Router::new()
        .route("/article/{id}", get(handlers::user::get::article))
        .route("/home", get(handlers::user::get::articles))
        .route(
            "/edit/{id}",
            get(handlers::admin::get::edit_article).post(handlers::admin::post::edit_article),
        )
        .route(
            "/new",
            get(handlers::admin::get::new_article).post(handlers::admin::post::new_article),
        )
        .route("/delete/{id}", post(handlers::admin::post::delete_article))
        .route("/admin", get(handlers::admin::get::admin_panel))
        .with_state(hm);
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
