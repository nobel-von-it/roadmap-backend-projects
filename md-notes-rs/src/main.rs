mod fs;
mod handlers;
mod md;

use handlers::*;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::{Result, anyhow};
use axum::{
    Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::fs::{NoteFS, NoteFSManager};

struct AppState {
    root_path: String,
    cached_notes: Vec<PathBuf>,
    fs: NoteFSManager,
}

#[derive(Deserialize, Serialize)]
struct MdNoteRequest {
    name: String,
    content: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber to log to stdout
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::info!("Initializing Markdown Notes Service...");

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(anyhow!("usage: md-notes-rs <root-path>"));
    }
    let root_path = args[1].clone();

    let fs = NoteFSManager;
    tracing::info!(root_dir = %root_path, "Scanning root directory for markdown notes...");
    let cached_notes = fs.scan_root_dir(&root_path).await;
    tracing::info!(
        cached_count = cached_notes.len(),
        "Scan complete. First 10 loaded: {:?}",
        cached_notes.iter().take(10).collect::<Vec<_>>()
    );

    let app_state = Arc::new(Mutex::new(AppState {
        root_path,
        cached_notes,
        fs,
    }));

    let router = Router::new()
        .route(
            "/api/notes",
            post(create_note_handler).get(list_notes_handler),
        )
        .route("/api/notes/{filename}/render", get(render_note_handler))
        .route("/api/grammar-check", post(grammar_check_handler))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = "127.0.0.1:8080";
    tracing::info!(bind_address = %addr, "Starting HTTP server...");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
