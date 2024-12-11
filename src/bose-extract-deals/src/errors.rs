use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("Request: {0}")]
    ReqWest(#[from] reqwest::Error),
    #[error("JSON serialization/deserialization: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("SQLite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("HTTP status code: {0}")]
    HttpStatusCode(String),
    #[error("General: {0}")]
    General(String),
}
