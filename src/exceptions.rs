use thiserror::Error;

#[derive(Error, Debug)]
pub enum APIRequestError {
    #[error("API request failed: {0}")]
    RequestFailed(String),
    
    #[error("Invalid JSON response: {0}")]
    InvalidJson(String),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

