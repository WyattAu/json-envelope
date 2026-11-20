# Pattern adoption

Uniform JSON API response envelope with a shared cross-implementation
conformance fixture.

| Pattern | Status | Evidence |
|---|---|---|
| P57 | adopted | tests/conformance.rs:1 (shared fixture tests/conformance_cases.json must decode identically to the TS @est/envelope implementation, case for case) |
| P64 | adopted | tests/proptest.rs:4 |
| P67 | partial | fuzz/fuzz_targets/fuzz_parse.rs (byte-level parse fuzz target; no structured generator) |
| P72 | adopted | src/envelope.rs:10 (ApiResponse uniform envelope: success flag, data, error, pagination) |
