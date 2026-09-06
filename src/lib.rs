#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

//! Standard API response envelope for Rust services.
//!
//! Provides `ApiResponse<T>`, `ApiError`, and `PaginationMeta`.
//!
//! This crate is `no_std` + `alloc`. The optional `axum` feature adds an
//! `IntoResponse` adapter (axum itself is std-bound; linking it transitively
//! keeps this crate core-only).

extern crate alloc;

mod envelope;
mod error;

pub use envelope::{ApiResponse, PaginationMeta};
pub use error::ApiError;
