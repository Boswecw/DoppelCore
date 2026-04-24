# Canvas 05 — Registry Integration Plan

**Date and Time:** 2026-04-18 07:44:18

## 1. Registry role after DoppelCore

Registry remains the outer governed shell.

It still owns:

- discovery
- intake
- designation
- compliance scheduling
- remediation planning
- apply flow
- verification flow
- operator decisions
- repo-state rollup

DoppelCore does not replace these.

## 2. Integration strategy

## Step 1 — add DoppelCore as a sibling backend subsystem
Do not jam DoppelCore logic into existing registry files at first.

Add a separate module with clean contracts.

## Step 2 — emit DoppelCore products from current compliance runs
Each compliance run should produce machine products, even if V1 scope is mostly documentation-governance truth.

## Step 3 — make Registry read DoppelCore manifests
Registry views should consume machine-truth outputs instead of scattered file checks wherever practical.

## Step 4 — shift rendered docs to projection status
Compiled `SYSTEM.md` remains required, but its authority becomes derivative.

## 3. Immediate mapping from current Registry concepts

### Existing Registry concept → DoppelCore concept

- governed system → `SubjectRecord`
- designation and canonical output target → anchored repo/document subject facts
- compliance finding → `ClaimRecord` + `PostureRecord`
- filesystem proof → `EvidenceRecord`
- stale assembled doc state → `DriftRecord`
- compliance run summary → `ManifestRecord`

## 4. What Registry should still decide

Registry should continue deciding:

- when a scan is allowed
- which profile is valid
- whether remediation is operator-only
- whether proof gate passes
- what gets published in UI

## 5. Required IPC and storage additions

Suggested additions:

- `registry_run_doppel_scan_for_system`
- `registry_get_doppel_manifest`
- `registry_list_doppel_drift`
- `registry_get_subject_detail`

Suggested persistence groups:

- `reg_doppel_runs`
- `reg_doppel_subjects`
- `reg_doppel_anchors`
- `reg_doppel_claims`
- `reg_doppel_evidence`
- `reg_doppel_drift`
- `reg_doppel_manifests`

## 6. Honesty rule

A rendered view may not claim a repo slice is mirrored unless the corresponding DoppelCore manifest exists and has a non-false posture basis.
