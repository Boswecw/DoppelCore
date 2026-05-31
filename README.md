# DoppelCore

DoppelCore is the canonical **machine-truth kernel** for a code-mirror system —
the nearest governed, machine-readable mirror of code reality.

- **Code** is the authority.
- **DoppelCore** mirrors it into typed, JSON-stable truth records.
- **Registry** governs, verifies, and enforces.
- **Human-readable documents** are deterministic rendered products, not the
  authority layer.

See `docs/doppelcore_canvas_set/` for the full doctrine and the phased roadmap
(Canvas 08).

## Status

This repository is a standalone Rust library. It currently provides:

- **Contract surface** (`contracts`, `records`, `comparison`, `correction`,
  `errors`) — the portable, JSON-stable types for subjects, anchors, evidence,
  claims, posture, drift, manifests, comparisons, and the correction fabric.
- **Phase 2 extraction intake** (`extraction`, `intake`) — Cortex extraction
  packet contracts and a normalization adapter that turns bounded extraction
  packets into canonical subjects, anchors, and evidence.

Not yet implemented (Canvas 08 roadmap): the claim engine (Phase 3), Registry
persistence/IPC (Phase 4), rendered projection (Phase 5), and differential
scans / drift history (Phase 6).

## Build and test

```bash
cargo check
cargo test
```

CI (`.github/workflows/ci.yml`) enforces `cargo fmt --check`,
`cargo clippy -D warnings`, and `cargo test` on every push and pull request.

## Layout

- `src/contracts.rs` — string enums (profile, subject / anchor / claim kinds, …)
- `src/records.rs` — canonical truth records and the manifest bundle
- `src/comparison.rs` — manifest-diff and drift-delta types
- `src/correction.rs` — correction-fabric contracts
- `src/extraction.rs` — Cortex extraction packet contracts
- `src/intake.rs` — extraction → canonical record normalization
- `src/errors.rs` — typed error surface
- `tests/wire_format.rs` — JSON wire-format guarantees
- `docs/doppelcore_canvas_set/` — doctrine and roadmap
