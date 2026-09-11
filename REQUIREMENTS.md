# Requirements — json-envelope

Numbered, testable requirements. Every requirement maps to at least one named
test or doc-comment contract; security-relevant items cite threat-model rows.

Scope: JSON API envelope (`json-envelope`) — `ApiResponse`, `ApiError`, `PaginationMeta` with serde roundtrip contract

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-JE-001 | `ApiResponse::success`/`error` serialize to the documented JSON shapes; `success` discriminant is always present | MUST |
| REQ-JE-002 | `ApiError` carries code/message (+opt-in details) and builders validate invariants | MUST |
| REQ-JE-003 | Serde impls are feature-gated (`serde_impl`); `--no-default-features` builds compile | MUST |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-JE-100 | Deserialization of hostile JSON yields typed errors, never panics (property-tested) | MUST |

## Observability & API hygiene

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-JE-900 | All fallible public APIs return typed errors; production `unwrap`/`expect` is denied or explicitly justified with an invariant comment | MUST |
| REQ-JE-901 | Public items carry doc comments with runnable examples where practical | SHOULD |
