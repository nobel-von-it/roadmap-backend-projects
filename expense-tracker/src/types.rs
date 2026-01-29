use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Expense {
    pub id: usize,
    pub amount: f64,
    pub description: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Default for Expense {
    fn default() -> Self {
        let now = Local::now();
        Self {
            id: 0,
            amount: 0.0,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Expense {
    pub fn new(amount: f64, description: Option<String>) -> Self {
        Self {
            amount,
            description,
            ..Default::default()
        }
    }
}
