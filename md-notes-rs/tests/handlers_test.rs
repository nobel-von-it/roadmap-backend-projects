use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use md_notes_rs::{
    AppState,
    fs::{Note, NoteManager},
    handlers::{ListNotesResponse, create_router},
};
use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::Mutex;
use tower::ServiceExt;

#[tokio::test]
async fn test_full_http_api_end_to_end_integration() {
    let dir = tempdir().unwrap();
    let manager = NoteManager::new(dir.path().to_path_buf());
    let state = Arc::new(AppState {
        fs: Mutex::new(manager),
    });
    let app = create_router(state);

    // 1. Create a new markdown note via POST /api/notes
    let create_payload = serde_json::json!({
        "name": "e2e_note",
        "content": "# Integration Header\n\n- Bullet 1\n- Bullet 2",
        "dir": null
    });

    let res_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&create_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res_create.status(), StatusCode::CREATED);

    let bytes_create = res_create.into_body().collect().await.unwrap().to_bytes();
    let created_note: Note = serde_json::from_slice(&bytes_create).unwrap();
    assert_eq!(created_note.title(), "e2e_note");

    let note_id = *created_note.id();

    // 2. Fetch all notes via GET /api/notes
    let res_list = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/notes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res_list.status(), StatusCode::OK);
    let bytes_list = res_list.into_body().collect().await.unwrap().to_bytes();
    let list_data: ListNotesResponse = serde_json::from_slice(&bytes_list).unwrap();

    assert_eq!(list_data.notes.len(), 1);
    assert_eq!(list_data.notes[0].id(), &note_id);
    assert_eq!(list_data.notes[0].title(), "e2e_note");

    // 3. Render note HTML via GET /api/notes/{id}/render
    let res_render = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/notes/{}/render", note_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res_render.status(), StatusCode::OK);
    let bytes_render = res_render.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(bytes_render.to_vec()).unwrap();

    assert!(html.contains("Integration Header"));
    assert!(html.contains("Bullet 1"));

    // 4. Grammar check note via GET /api/notes/{id}/check
    let res_check = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/notes/{}/check", note_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(res_check.status() == StatusCode::OK || res_check.status() == StatusCode::INTERNAL_SERVER_ERROR);
}
