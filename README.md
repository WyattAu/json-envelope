# json-envelope

> Standard API response envelope for Rust services.

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)

## Quick Start

```rust
use json-envelope::ApiResponse;

// Success response
let response = ApiResponse::success(my_data);

// Error response
let response = ApiResponse::<()>::error("NOT_FOUND", "Resource not found");

// Paginated response
let response = ApiResponse::paginated(items, meta);
```

## License

MIT OR Apache-2.0
