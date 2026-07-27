use std::{path::PathBuf, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use uuid::Uuid;

use crate::{
    AppState, MdNoteRequest,
    api::languagetool::check_grammar,
    fs::{Note, NoteOperation},
    md::render_md_to_html,
};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ListNotesResponse {
    pub notes: Vec<Note>,
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route(
            "/api/notes",
            post(create_note_handler).get(list_notes_handler),
        )
        .route("/api/notes/{filename}/render", get(render_note_handler))
        .route("/api/grammar-check", post(grammar_check_handler))
        .route("/api/notes/{filename}/check", get(grammar_check_handler))
        .with_state(app_state)
}

pub async fn index_handler() -> impl IntoResponse {
    Html(include_str!("../../static/index.html"))
}

#[tracing::instrument(skip(state, id))]
pub async fn render_note_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let fs = state.fs.lock().await;
    match fs.get_note(&id).await {
        Ok(note) => match render_md_to_html(note.content()).await {
            Ok(html) => {
                tracing::info!("Note rendered successfully");
                (StatusCode::OK, html).into_response()
            }
            Err(err) => {
                tracing::error!(error = %err, "Failed to render note");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        },
        Err(err) => {
            tracing::error!(error = %err, "Failed to fetch note");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

#[tracing::instrument(skip(state, new_note))]
pub async fn create_note_handler(
    State(state): State<Arc<AppState>>,
    Json(new_note): Json<MdNoteRequest>,
) -> impl IntoResponse {
    tracing::debug!("Acquiring AppState lock...");
    let mut fs = state.fs.lock().await;

    let dir = match new_note.dir {
        Some(d) => {
            let path = PathBuf::from(d);
            if path.is_relative() {
                fs.root().join(path)
            } else {
                path
            }
        }
        None => fs.root().to_path_buf(),
    };

    match fs.create_note(&dir, new_note.name, new_note.content).await {
        Ok(note) => {
            tracing::info!("Note created successfully. New size: {}", fs.cache().len());
            (StatusCode::CREATED, Json(note)).into_response()
        }
        Err(err) => {
            tracing::error!(error = %err, "Failed to create note");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

#[tracing::instrument(skip(state))]
pub async fn list_notes_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    tracing::debug!("Acquiring AppState lock...");
    let fs = state.fs.lock().await;

    tracing::debug!("Extracting filenames from cached paths...");
    let cached_ids = fs.cache().keys().cloned().collect::<Vec<_>>();
    let cached_notes_fut = cached_ids.iter().map(|id| fs.get_note(id));
    let cached_notes = futures::future::join_all(cached_notes_fut)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    tracing::debug!(
        count = cached_notes.len(),
        "Reading note contents asynchronously from disk..."
    );
    Json(ListNotesResponse {
        notes: cached_notes,
    })
    .into_response()
}

#[tracing::instrument(skip(state, id))]
pub async fn grammar_check_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let fs = state.fs.lock().await;
    match fs.get_note(&id).await {
        Ok(note) => match check_grammar(note.content()).await {
            Ok(text) => (StatusCode::OK, Json(text)).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        },
        Err(err) => {
            tracing::error!(error = %err, "Failed to find note for grammar check");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

#[cfg(test)]
mod tests;
