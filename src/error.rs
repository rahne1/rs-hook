use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebhookError {
    #[error("HTTP error: {0}")]
    Http(#[from] hyper::Error),

    #[error("HTTP status error: {status}")]
    Status { status: hyper::StatusCode, body: String },

    #[error("JSON serialization error: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TLS error: {0}")]
    Tls(#[from] native_tls::Error),

    #[error("Tokio TLS handshake error: {0}")]
    TokioTls(String),

    #[error("Invalid webhook URL")]
    InvalidUrl,

    #[error("Request error: {0}")]
    Request(String),

    #[error("Content too long: {0} characters (max 6000)")]
    ContentTooLong(usize),
}

pub type Result<T> = std::result::Result<T, WebhookError>;
