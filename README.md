# json-envelope

> Standard API response envelope for Rust services.

> **Deprecated umbrella**: this crate is now a thin re-export shim over
> [`api-types`](https://github.com/WyattAu/api-types), which is the single
> envelope story (JSend envelope + `PaginationMeta` + `utoipa` derives +
> RFC 7807 + proptest/fuzz). New code should depend on `api-types` directly.
> This crate keeps publishing so existing users don't break.

## Quick Start

```rust
use json_envelope::ApiResponse;

// Success response
let response = ApiResponse::success(my_data);

// Error response (note the migrated constructor name)
let response = ApiResponse::<()>::error_with("NOT_FOUND", "Resource not found");

// Paginated response
let response = ApiResponse::paginated(items, meta);
```

## Migration to `api-types`

| `json-envelope` (old) | `api-types` (new) |
|------------------------|-------------------|
| `ApiResponse::success(data)` | unchanged |
| `ApiResponse::paginated(data, meta)` | unchanged |
| `ApiResponse::<()>::error(code, msg)` | `ApiResponse::<()>::error_with(code, msg)` |
| `ApiResponse::from(data)` | unchanged |
| `PaginationMeta { .. }` / `ApiError { .. }` | unchanged shapes |

## License

MIT OR Apache-2.0
