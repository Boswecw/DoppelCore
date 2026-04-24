# Canvas 04 — Scan Pipeline and Evidence Flow

**Date and Time:** 2026-04-18 07:44:18

## 1. Pipeline objective

The pipeline must convert repo reality into machine truth without letting inference outrun proof.

## 2. Pipeline stages

## Stage 0 — Scan request
Registry issues a bounded scan request.

Inputs:

- governed repo path
- revision or working tree reference
- profile
- slice selector
- operator intent

## Stage 1 — Cortex extraction
Cortex performs bounded reads and emits extraction packets.

Possible packet types:

- file index packet
- safe text packet
- symbol packet
- route registration packet
- import graph packet
- documentation tree packet

## Stage 2 — DoppelCore normalization
DoppelCore converts raw extraction into normalized subjects and anchors.

## Stage 3 — Claim derivation
DoppelCore derives bounded claims from evidence.

This stage must never collapse ambiguity.

## Stage 4 — Posture and drift evaluation
DoppelCore determines:

- posture per claim
- drift per anchor or artifact
- aggregate mirror posture

## Stage 5 — Manifest emission
DoppelCore emits a machine bundle that Registry can store, compare, or publish.

## Stage 6 — Registry consumption
Registry uses the outputs for:

- compliance evaluation
- remediation planning
- verification comparison
- operator review
- rendered output generation

## 3. Evidence rules

### Rule 1 — evidence is immutable per run
Evidence records should be append-only within a scan run.

### Rule 2 — source refs are mandatory
Every claim must point back to evidence and anchors.

### Rule 3 — unknowns are valid output
No data is better than fake certainty.

### Rule 4 — scan profiles must be explicit
A lightweight documentation profile and a deeper code-mirror profile are different things.

## 4. Suggested scan profiles

### `doc_system_v1`
Focus on protocol structure and canonical documentation evidence.

### `route_service_slice_v1`
Focus on one route, one service, one workflow, one persistence path, one event path.

### `repo_mirror_foundation_v1`
Focus on repo-wide subject and anchor emission without full semantic claims.

## 5. Product retention

Recommended retention posture:

- keep latest successful manifest
- keep prior manifest for diff comparison
- keep evidence by run id
- retain drift records across runs for historical trending
