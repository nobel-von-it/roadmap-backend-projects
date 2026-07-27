pub mod api;
pub mod fs;
pub mod handlers;
pub mod md;

use crate::fs::NoteManager;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub struct AppState {
    pub fs: Mutex<NoteManager>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MdNoteRequest {
    pub dir: Option<String>,
    pub name: String,
    pub content: Option<String>,
}
