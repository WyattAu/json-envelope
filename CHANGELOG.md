# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

### Changed
- This crate is now a thin re-export shim over `api-types` (the single
  envelope story: JSend envelope + `PaginationMeta` + `utoipa` + RFC 7807 +
  proptest/fuzz). `ApiResponse::success`, `ApiResponse::paginated`,
  `From<T>`, `PaginationMeta`, and the `ApiError` struct shape are unchanged;
  `ApiResponse::error(code, msg)` becomes `ApiResponse::error_with(code, msg)`
  (or `ApiResponse::error(ApiError::new(code, msg))`). The `axum`/`openapi`
  features now forward to `api-types`. Benches and fuzz targets moved to
  `api-types`. `no_std` support is dropped (the shim links `api-types`, which
  is std).

## [0.1.0] - 2026-09-01

### Added
- Standard API response envelope — ApiResponse<T>, ApiError, PaginationMeta.
- Published to crates.io (2026-09-01).
