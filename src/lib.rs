#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Standard API response envelope for Rust services.
//!
//! > **Deprecated umbrella**: this crate is now a thin re-export shim over
//! > [`api-types`](https://github.com/WyattAu/api-types), which is the single
//! > envelope story (JSend envelope + `PaginationMeta` + `utoipa` derives +
//! > RFC 7807 + proptest/fuzz). New code should depend on `api-types`
//! > directly. This crate keeps publishing so existing users don't break.
//!
//! # Migration
//!
//! | `json-envelope` (old) | `api-types` (new) |
//! |------------------------|-----------------|
//! | `ApiResponse::success(data)` | unchanged |
//! | `ApiResponse::paginated(data, meta)` | unchanged |
//! | `ApiResponse::<()>::error(code, msg)` | `ApiResponse::<()>::error_with(code, msg)` or `ApiResponse::error(ApiError::new(code, msg))` |
//! | `ApiResponse::from(data)` | unchanged |
//! | `PaginationMeta { page, per_page, total, total_pages }` | unchanged |
//! | `ApiError { code, message, details }` | unchanged (plus `ApiError::new` / `with_details` builders) |
//!
//! New in `api-types`: the strict JSend envelope [`JSend`](api_types::JSend),
//! `ApiListResponse`, RFC 7807 `ProblemDetail`, and the `openapi` feature.

pub use api_types::*;
