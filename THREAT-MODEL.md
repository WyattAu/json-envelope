# Threat Model — json-envelope

Reference: STRIDE. Scope: the crate's public API surface. Trust boundary:
(1) bytes/inputs entering public constructors and parsers, (2) concurrent
callers sharing interior state. json-envelope is an in-process library — it opens
no sockets and inherits the embedding process's trust domain.

Purpose: JSON API envelope (`json-envelope`) — `ApiResponse`, `ApiError`, `PaginationMeta` with serde roundtrip contract

## Assets

| ID | Asset | Exposed via |
|----|-------|-------------|
| A1 | wire compatibility of envelope shapes (semver surface) | hostile input, concurrent callers |
| A2 | integrity of error codes/messages | hostile input, concurrent callers |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Residual risk |
|---|--------|----------|---------|------------|---------------|
| T1 | Envelope shape drift breaks consumers | Tampering | `serde derives` | shape pinned by roundtrip property tests and semver-checks baseline | documented |
| T2 | Hostile JSON accepted into invariants | Spoofing | `deserialize` | serde validation + typed errors; proptest over random JSON | documented |
| T3 | `details` leaks sensitive data | Info disclosure | ``ApiError.details`` | opt-in field; callers control contents (documented residual risk) | documented |

## Repudiation

The crate keeps no audit trail; attribution of calls to callers is out of
scope for an in-process library.

## Out of Scope

- Network transport security (the crate never opens sockets).
- Storage-host compromise: an attacker who controls the host can bypass all
  in-process mitigations.
- Denial of service via resource exhaustion of the host process beyond the
  bounds enforced above.

Reviewed: 2026-09-11
