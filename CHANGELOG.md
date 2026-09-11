# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

### Added
- `no_std` support (core + alloc). The `axum` feature is now a real,
  declared optional dependency (`axum` 0.8, `json` feature) instead of an
  undeclared cfg that never activated — the advertised `IntoResponse`
  adapter is now actually available with `--features axum`.

### Changed
- `serde`/`serde_json` build without their std default features; `thiserror`
  and `http` (unused) removed; `serde_json` is a dev-dependency for
  tests/benches.

## [0.1.0] - 2026-09-01

### Added
- Standard API response envelope — ApiResponse<T>, ApiError, PaginationMeta.
- Published to crates.io (2026-09-01).
