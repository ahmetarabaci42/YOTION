use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(String),
    Validation(String),
    Encryption(String),
    NotFound(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(_) => write!(f, "Database error occurred"),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::Encryption(_) => write!(f, "Encryption error occurred"),
            AppError::NotFound(resource) => write!(f, "{} not found", resource),
            AppError::Internal(_) => write!(f, "Internal server error"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        match err {
            rusqlite::Error::QueryReturnedNoRows => {
                AppError::NotFound("Resource".to_string())
            }
            rusqlite::Error::SqliteFailure(_, Some(msg)) if msg.contains("UNIQUE") => {
                AppError::Validation("A record with this value already exists".to_string())
            }
            _ => AppError::Database(format!("Database operation failed")),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(format!("Internal error"))
    }
}

pub type AppResult<T> = Result<T, AppError>;
