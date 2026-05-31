# VERIFY

This repository is healthy when:

- `cargo test` passes
- `cargo check` passes
- `cargo fmt --all -- --check` reports no diffs
- `cargo clippy --all-targets -- -D warnings` is clean

CI runs all of these on every push and pull request.
