# Canvas 03 — Canonical Record Model

**Date and Time:** 2026-04-18 07:44:18

## 1. Record philosophy

A code mirror is only as good as its record model.

DoppelCore should not begin with giant universal schema ambitions. It should begin with a precise record family that can grow cleanly.

## 2. Foundational record families

## 2.1 SubjectRecord
Represents the thing being mirrored.

Examples:

- repository
- module
- route
- service
- workflow
- event path
- persistence boundary
- documentation artifact

Suggested fields:

- `subject_id`
- `subject_kind`
- `repo_id`
- `path`
- `display_name`
- `revision_ref`
- `source_profile`

## 2.2 AnchorRecord
Represents a stable machine address into source reality.

Examples:

- file path anchor
- symbol anchor
- route anchor
- function anchor
- SQL migration anchor

Suggested fields:

- `anchor_id`
- `subject_id`
- `anchor_kind`
- `path`
- `line_start`
- `line_end`
- `symbol_name`
- `anchor_hash`

## 2.3 ClaimRecord
Represents a bounded statement about behavior, structure, or role.

Examples:

- “this route writes to table X”
- “this service emits event Y”
- “this assembled artifact is stale”

Suggested fields:

- `claim_id`
- `subject_id`
- `claim_type`
- `statement`
- `truth_class`
- `posture`
- `evidence_refs`
- `confidence_policy`

## 2.4 EvidenceRecord
Represents what was directly observed.

Examples:

- file metadata
- symbol parse result
- route registration parse
- import graph fragment
- output timestamp check

Suggested fields:

- `evidence_id`
- `evidence_type`
- `probe_method`
- `source_ref`
- `captured_at`
- `digest`
- `determinism_class`

## 2.5 PostureRecord
Represents the truth posture of an aggregate or claim.

Canonical starter vocabulary:

- `verified`
- `deterministic`
- `inferred`
- `partial`
- `blocked`
- `stale`
- `unknown`
- `conflicted`
- `unassessable`

## 2.6 DriftRecord
Represents code-to-record divergence or proof degradation.

Examples:

- source changed after last mirror emission
- anchor hash mismatch
- canonical output older than source records
- unresolved symbol after refactor

## 2.7 ManifestRecord
Represents the emitted mirror bundle for a repo or slice.

Suggested fields:

- `manifest_id`
- `repo_id`
- `revision_ref`
- `profile_id`
- `record_counts`
- `posture_summary`
- `generated_at`
- `generator_version`

## 3. Determinism classes

Every significant fact must be marked by determinism class.

Starter classes:

- `deterministic`
- `heuristic`
- `operator_asserted`
- `mixed`

Canonical rule:

Deterministic facts may gate compliance directly. Heuristic facts may inform review, but must not silently become hard truth.

## 4. First bounded scope

V1 DoppelCore should focus on:

- repo identity
- documentation system structure
- canonical output presence/freshness
- route/service/workflow anchors for one proving slice
- evidence-backed claims for that slice

## 5. Machine product families

Recommended V1 output files:

- `subjects.json`
- `anchors.json`
- `claims.json`
- `evidence.json`
- `postures.json`
- `drift.json`
- `manifest.json`
