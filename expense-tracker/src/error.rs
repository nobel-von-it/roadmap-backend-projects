use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExpenseError {
    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("ParseInt error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("ParseFloat error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Unknown command")]
    UnknownCommand,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Empty description")]
    EmptyDescription,
    #[error("Expense not found with id: {0}")]
    ExpenseNotFound(usize),
    #[error("Invalid month: {0}")]
    InvalidMonth(u8),
}
