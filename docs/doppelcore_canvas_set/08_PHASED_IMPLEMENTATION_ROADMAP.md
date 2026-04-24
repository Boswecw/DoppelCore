# Canvas 08 — Phased Implementation Roadmap

**Date and Time:** 2026-04-18 07:44:18

## Phase 0 — doctrine lock

Decide and record:

- DoppelCore is the canonical machine-truth kernel
- Registry remains the outer control shell
- Cortex remains extraction
- rendered docs are downstream products

## Phase 1 — contract foundation

Build:

- `records.rs`
- `contracts.rs`
- `posture.rs`
- `drift.rs`
- `manifests.rs`
- `errors.rs`

Outcome:

A compilable typed contract family exists.

## Phase 2 — extraction handoff integration

Define Cortex packet types and DoppelCore intake adapters.

Outcome:

DoppelCore can normalize bounded extraction packets into subjects, anchors, and evidence.

## Phase 3 — initial claim engine

Implement bounded derivation for:

- doc-system structure claims
- canonical output presence claims
- stale assembled documentation drift
- one route/service/workflow proving slice

Outcome:

First real machine-truth emission.

## Phase 4 — Registry storage and IPC

Add persistence and commands.

Outcome:

ForgeCommand can run a Doppel scan and read stored manifest data.

## Phase 5 — rendered projection

Generate:

- one backend review packet
- one UI slice
- one rendered Markdown output from machine records

Outcome:

Human-readable output becomes provably downstream.

## Phase 6 — differential scans and drift history

Add diff-aware reruns, historical comparison, and anchor drift tracking.

Outcome:

DoppelCore becomes useful as an ongoing mirror instead of a one-shot scan.

## Phase 7 — controlled expansion

Expand to additional route/service/workflow families, then repo-wide coverage.

Outcome:

Mirror coverage grows without collapsing discipline.
