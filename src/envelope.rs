use serde::Serialize;

/// Standard API response envelope.
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    /// Whether the request was successful.
    pub success: bool,
    /// The response data (present on success).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// The error details (present on failure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
    /// Pagination metadata (present for list endpoints).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMeta>,
}

/// Pagination metadata for list responses.
#[derive(Debug, Clone, Serialize)]
pub struct PaginationMeta {
    /// Current page number (1-based).
    pub page: u32,
    /// Items per page.
    pub per_page: u32,
    /// Total number of items.
    pub total: u64,
    /// Total number of pages.
    pub total_pages: u32,
}

impl<T: Serialize> ApiResponse<T> {
    /// Create a successful response with data.
    pub fn success(data: T) -> Self {
        Self { success: true, data: Some(data), error: None, pagination: None }
    }

    /// Create a paginated response.
    pub fn paginated(data: Vec<T>, meta: PaginationMeta) -> Self {
        Self { success: true, data: Some(data), error: None, pagination: Some(meta) }
    }
}

impl ApiResponse<()> {
    /// Create an error response.
    pub fn error(code: &str, message: &str) -> Self {
        Self { success: false, data: None, error: Some(ApiError { code: code.to_string(), message: message.to_string(), details: None }), pagination: None }
    }
}

impl<T: Serialize> From<T> for ApiResponse<T> {
    fn from(data: T) -> Self {
        Self::success(data)
    }
}

#[cfg(feature = "axum")]
impl<T: Serialize> axum::response::IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let status = if self.success { 200 } else { 400 };
        (axum::http::StatusCode::from_u16(status).unwrap(), axum::Json(self)).into_response()
    }
}
