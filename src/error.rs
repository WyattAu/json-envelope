use serde::Serialize;

/// API error details.
#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    /// Machine-readable error code.
    pub code: String,
    /// Human-readable error message.
    pub message: String,
    /// Optional additional details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}
