use std::{path::PathBuf, sync::Arc};

use anyhow::{Result, anyhow};
use md_notes_rs::{
    AppState,
    fs::{Cache, NoteManager},
    handlers::create_router,
};
use tokio::{net::TcpListener, sync::Mutex};

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

    let mut fs = NoteManager::new(PathBuf::from(&root_path));
    fs.load_cache().await?;
    tracing::info!(root_dir = %root_path, "Scanning root directory for markdown notes...");

    tracing::info!(
        cached_count = fs.cache().len(),
        "Scan complete. First 10 loaded: {:?}",
        fs.cache().iter().take(10).collect::<Vec<_>>()
    );

    let app_state = Arc::new(AppState { fs: Mutex::new(fs) });

    let router = create_router(app_state).layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = "127.0.0.1:8080";
    tracing::info!(bind_address = %addr, "Starting HTTP server...");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
