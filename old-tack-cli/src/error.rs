use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskCliError {
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse Error: {0}")]
    ParseJsonError(#[from] serde_json::Error),
    #[error("Parse Error: {0}")]
    ParseClapError(#[from] clap::Error),
    #[error("Already Exists: {0}")]
    AlreadyExists(String),
    #[error("Incorrect Input: {0}")]
    IncorrectInput(String),
    #[error("Not Found: {0}")]
    NotFound(String),
}
