# DoppelCore — Phase 8E Repo Init Slice

Date: 2026-04-24
Time: 2026-04-24 03:06 UTC

This slice starts DoppelCore as its own repo.

## Slice boundary

This slice does **only** the following:

- establishes a new standalone Rust library repo for `DoppelCore`
- moves the portable contract surface into the new repo
- keeps the repo free of UI, registry, self-healing, and LocalDb ownership
- proves the new repo is ready for `cargo test` and `cargo check`

## What is intentionally not in this slice

- no database adapter
- no sqlx migrations
- no Tauri ownership
- no direct Self-Healing commands
- no Registry orchestration
- no correction-fabric handoff
- no execution lane

## Why this is the right 8E move

The plan says DoppelCore should become its own internal ecosystem component.
Right now the truth-core contracts exist inside `Forge_Command`.
This slice starts the extraction without dragging cross-repo coupling into the first proof gate.

## Success criteria

- `cargo test` passes in `~/Forge/ecosystem/DoppelCore`
- `cargo check` passes in `~/Forge/ecosystem/DoppelCore`
- repo is ready for the next extraction slice
