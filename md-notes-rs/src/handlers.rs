use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    AppState, MdNoteRequest,
    fs::{NoteFS, get_file_name},
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct MdNote {
    path: String,
    title: String,
    content: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ListNotesResponse {
    notes: Vec<MdNote>,
}

#[tracing::instrument]
pub async fn render_note_handler() {
    tracing::info!("Render note handler called");
}

#[tracing::instrument(skip(state, new_note))]
pub async fn create_note_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(new_note): Json<MdNoteRequest>,
) -> impl IntoResponse {
    tracing::debug!("Acquiring AppState lock...");
    let fs = match state.lock() {
        Ok(guard) => {
            tracing::debug!(
                cached_count = guard.cached_notes.len(),
                "AppState lock acquired successfully"
            );
            guard.fs
        }
        Err(err) => {
            tracing::error!(error = %err, "Failed to acquire AppState lock (Mutex poisoned)");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    let title = new_note.name.clone();
    let mut tmp_path = new_note.name.clone();
    if !tmp_path.starts_with("./") {
        tmp_path = format!("./{}", tmp_path);
    }
    if !tmp_path.ends_with(".md") {
        tmp_path = format!("{}.md", tmp_path);
    }
    let path = PathBuf::from(tmp_path);
    match fs
        .create(
            path.parent().unwrap().to_path_buf(),
            title,
            new_note.content.unwrap_or_default(),
        )
        .await
    {
        Ok(_) => {
            if let Ok(mut guard) = state.lock() {
                guard.cached_notes.push(path.clone());
                tracing::info!(
                    "Note created successfully. New size: {}",
                    guard.cached_notes.len()
                );
                // TODO: replace resopnse with the new note
                (StatusCode::CREATED, "Note created successfully").into_response()
            } else {
                tracing::error!("Failed to acquire AppState lock (Mutex poisoned)");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
        Err(err) => {
            tracing::error!(error = %err, "Failed to create note");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

#[tracing::instrument(skip(state))]
pub async fn list_notes_handler(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    tracing::debug!("Acquiring AppState lock...");
    let (cached_notes, fs) = match state.lock() {
        Ok(guard) => {
            tracing::debug!(
                cached_count = guard.cached_notes.len(),
                "AppState lock acquired successfully"
            );
            (guard.cached_notes.clone(), guard.fs)
        }
        Err(err) => {
            tracing::error!(error = %err, "Failed to acquire AppState lock (Mutex poisoned)");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
    };

    tracing::debug!("Extracting filenames from cached paths...");
    let cached_names = cached_notes
        .iter()
        .filter_map(|path| get_file_name(path).ok())
        .collect::<Vec<_>>();

    tracing::debug!(
        count = cached_notes.len(),
        "Reading note contents asynchronously from disk..."
    );
    let cached_contents_fut = cached_notes.iter().map(|name| fs.get_content_by_path(name));
    let cached_contents = futures::future::join_all(cached_contents_fut)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    if cached_names.len() != cached_contents.len() {
        tracing::error!(
            names_count = cached_names.len(),
            contents_count = cached_contents.len(),
            "Integrity check failed: Mismatch between names count and contents read count"
        );
        return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
    }

    let notes = cached_names
        .into_iter()
        .zip(cached_notes.iter())
        .zip(cached_contents)
        .map(|((name, path), content)| MdNote {
            path: path.to_str().unwrap_or_default().to_string(),
            title: name,
            content,
        })
        .collect::<Vec<_>>();

    tracing::info!(
        notes_retrieved = notes.len(),
        "Successfully compiled notes list"
    );
    Json(ListNotesResponse { notes }).into_response()
}

#[tracing::instrument]
pub async fn grammar_check_handler() {
    tracing::info!("Grammar check handler called");
}
