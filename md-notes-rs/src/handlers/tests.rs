use super::*;
use crate::fs::NoteManager;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tempfile::tempdir;
use tokio::sync::Mutex;
use tower::ServiceExt; // for oneshot

fn setup_test_app() -> (Router, tempfile::TempDir) {
    let dir = tempdir().unwrap();
    let manager = NoteManager::new(dir.path().to_path_buf());
    let state = Arc::new(AppState {
        fs: Mutex::new(manager),
    });
    let app = create_router(state);
    (app, dir)
}

async fn response_to_string(response: axum::response::Response) -> String {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

// ============================================================================
// 1. Tests for create_note_handler (POST /api/notes)
// ============================================================================

#[tokio::test]
async fn test_create_note_handler_success() {
    let (app, _dir) = setup_test_app();

    let request_payload = serde_json::json!({
        "name": "my_first_note",
        "content": "# Title\nThis is a test note.",
        "dir": null
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&request_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body_str = response_to_string(response).await;
    let created_note: Note = serde_json::from_str(&body_str).unwrap();

    assert_eq!(created_note.title(), "my_first_note");
    assert_eq!(created_note.content(), "# Title\nThis is a test note.");
}

#[tokio::test]
async fn test_create_note_handler_with_custom_dir() {
    let (app, dir) = setup_test_app();
    let custom_subfolder = dir.path().join("sub_folder").to_str().unwrap().to_string();

    let request_payload = serde_json::json!({
        "name": "nested_note",
        "content": "Nested content",
        "dir": custom_subfolder
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&request_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body_str = response_to_string(response).await;
    let note: Note = serde_json::from_str(&body_str).unwrap();

    assert_eq!(note.title(), "nested_note");
    assert!(note.path().to_str().unwrap().contains("sub_folder"));
}

#[tokio::test]
async fn test_create_note_handler_duplicate_fails() {
    let (app, _dir) = setup_test_app();

    let request_payload = serde_json::json!({
        "name": "duplicate_test",
        "content": "Content",
        "dir": null
    });

    // First request - Success
    let res1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&request_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res1.status(), StatusCode::CREATED);

    // Second request with same name - Conflict / Error
    let res2 = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&request_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res2.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ============================================================================
// 2. Tests for list_notes_handler (GET /api/notes)
// ============================================================================

#[tokio::test]
async fn test_list_notes_handler_empty() {
    let (app, _dir) = setup_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/notes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_str = response_to_string(response).await;
    let list_response: ListNotesResponse = serde_json::from_str(&body_str).unwrap();

    assert!(list_response.notes.is_empty());
}

#[tokio::test]
async fn test_list_notes_handler_populated() {
    let (app, _dir) = setup_test_app();

    // Create 2 notes
    for i in 1..=2 {
        let payload = serde_json::json!({
            "name": format!("note_{}", i),
            "content": format!("content_{}", i),
            "dir": null
        });

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/notes")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
    }

    // List notes
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/notes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body_str = response_to_string(response).await;
    let list_response: ListNotesResponse = serde_json::from_str(&body_str).unwrap();

    assert_eq!(list_response.notes.len(), 2);
}

// ============================================================================
// 3. Tests for render_note_handler (GET /api/notes/{id}/render)
// ============================================================================

#[tokio::test]
async fn test_render_note_handler_success() {
    let (app, _dir) = setup_test_app();

    // Create note with markdown formatting
    let payload = serde_json::json!({
        "name": "render_me",
        "content": "# Hello World\n\nThis is **bold** text.",
        "dir": null
    });

    let create_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let created_note: Note = serde_json::from_str(&response_to_string(create_res).await).unwrap();
    let note_id = created_note.id();

    // Call render endpoint
    let render_res = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/notes/{}/render", note_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(render_res.status(), StatusCode::OK);
    let html_body = response_to_string(render_res).await;

    assert!(html_body.contains("<h1>Hello World</h1>") || html_body.contains("Hello World"));
    assert!(html_body.contains("bold"));
}

#[tokio::test]
async fn test_render_note_handler_not_found() {
    let (app, _dir) = setup_test_app();

    let random_id = Uuid::new_v4();
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/notes/{}/render", random_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ============================================================================
// 4. Tests for grammar_check_handler (GET /api/notes/{id}/check)
// ============================================================================

#[tokio::test]
async fn test_grammar_check_handler_not_found() {
    let (app, _dir) = setup_test_app();

    let random_id = Uuid::new_v4();
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/notes/{}/check", random_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_grammar_check_handler_existing_note() {
    let (app, _dir) = setup_test_app();

    let payload = serde_json::json!({
        "name": "grammar_test",
        "content": "Привет мир!",
        "dir": null
    });

    let create_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/notes")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let note: Note = serde_json::from_str(&response_to_string(create_res).await).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/notes/{}/check", note.id()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Since external LanguageTool server may or may not be running locally in test env,
    // the status code will be 200 (if server up) or 500 (if connection refused).
    // The test ensures the handler executes without panic.
    assert!(response.status() == StatusCode::OK || response.status() == StatusCode::INTERNAL_SERVER_ERROR);
}
