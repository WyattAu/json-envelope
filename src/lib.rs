#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Standard API response envelope for Rust services.
//!
//! Provides `ApiResponse<T>`, `ApiError`, and `PaginationMeta` with
//! `IntoResponse` implementation for Axum.

mod envelope;
mod error;

pub use envelope::{ApiResponse, PaginationMeta};
pub use error::ApiError;
