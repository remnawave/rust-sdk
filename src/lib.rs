pub mod api;
pub use api::*;

#[derive(Debug)]
pub struct ApiError {
    pub status_code: u16,
    pub url: String,
    pub request_body: Option<String>,
    pub response_body: String,
    pub response_headers: std::collections::HashMap<String, String>,
    pub timestamp: Option<String>,
    pub path: Option<String>,
    pub message: Option<String>,
    pub error_code: Option<String>,
    pub error: Option<String>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = &self.message {
            write!(
                f,
                "API Error [{}]: {} (code: {}) - {} | <{}>",
                self.status_code,
                msg,
                self.error_code.as_deref().unwrap_or("unknown"),
                self.error.as_deref().unwrap_or("unknown"),
                self.request_body.as_deref().unwrap_or("unknown")
            )
        } else {
            write!(
                f,
                "HTTP {} from {}: {} (code: {}) - {}",
                self.status_code,
                self.url,
                self.response_body,
                self.error_code.as_deref().unwrap_or("unknown"),
                self.error.as_deref().unwrap_or("unknown")
            )
        }
    }
}

impl std::error::Error for ApiError {}
