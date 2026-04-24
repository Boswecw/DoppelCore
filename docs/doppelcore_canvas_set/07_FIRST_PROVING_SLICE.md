# Canvas 07 — First Proving Slice

**Date and Time:** 2026-04-18 07:44:18

## 1. Purpose

The first proving slice should prove that DoppelCore can emit a truthful mirror bundle from a bounded real repo slice and that Registry can consume it without fiction.

## 2. Slice boundaries

Use exactly:

- 1 route
- 1 service
- 1 workflow
- 1 persistence path
- 1 event path
- 1 documentation artifact

This is enough to prove the model without drowning in scope.

## 3. Required outputs

For that slice, emit:

- one `SubjectRecord` bundle
- stable anchors for the route/service/workflow/persistence/event/doc artifact
- at least five evidence records
- at least five claims
- posture for each claim
- at least one drift evaluation
- one manifest

## 4. Required demonstrations

### Demo A — deterministic proof
Show that canonical doc-system checks still emit correctly through DoppelCore.

### Demo B — behavioral slice proof
Show that a bounded route/service/workflow relationship can be represented by anchors, claims, and evidence.

### Demo C — drift honesty
Change one source file or one canonical artifact timestamp and show drift emission.

### Demo D — rendered output honesty
Generate a simple human review view from the manifest and prove that it adds no uncited fact.

## 5. Acceptance criteria

The proving slice is accepted only if:

- machine products are emitted deterministically
- claims cite anchors and evidence
- posture is visible
- unknowns are not hidden
- Registry can read the manifest
- a human render can be generated from the same record set

## 6. Rejection criteria

Reject the slice if:

- narrative appears without machine backing
- claims exist without evidence
- drift is silently ignored
- UI invents severity or posture
- the slice depends on unconstrained agent narration
