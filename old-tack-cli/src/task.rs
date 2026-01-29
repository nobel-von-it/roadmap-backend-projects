use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Done,
    InProgress,
    Pending,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Done => write!(f, "done"),
            Status::InProgress => write!(f, "in-progress"),
            Status::Pending => write!(f, "pending"),
        }
    }
}

impl<T: AsRef<str>> From<T> for Status {
    fn from(s: T) -> Self {
        match s.as_ref().to_lowercase().as_str() {
            "done" => Status::Done,
            "inprogress" | "in-progress" => Status::InProgress,
            _ => Status::Pending,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub status: Status,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {} - {}", self.id, self.description, self.status)
    }
}

impl Default for Task {
    fn default() -> Self {
        let now = Local::now();
        Task {
            id: 0,
            description: String::new(),
            status: Status::Pending,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Task {
    pub fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            ..Default::default()
        }
    }

    pub fn update_text(&mut self, text: String) {
        self.description = text;
        self.updated_at = Local::now();
    }

    pub fn update_status(&mut self, status: Status) {
        self.status = status;
        self.updated_at = Local::now();
    }
}
