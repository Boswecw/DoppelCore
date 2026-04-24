# Canvas 10 — Operating Rules and Naming Doctrine

**Date and Time:** 2026-04-18 07:44:18

## 1. Naming doctrine

Use these terms consistently:

- **DoppelCore** — canonical machine-truth kernel
- **Cortex** — bounded extraction subsystem
- **Registry** — control/governance shell
- **Manifest** — emitted machine bundle for a scan or slice
- **Render** — downstream human-readable output
- **Drift** — code-to-record divergence
- **Posture** — truth state of a record or aggregate

## 2. Core operator rules

### Rule A
No human-readable artifact outranks the machine manifest.

### Rule B
No claim without evidence.

### Rule C
No evidence without source reference.

### Rule D
No verified posture without bounded proof.

### Rule E
No UI-derived truth.

## 3. Release rule

A DoppelCore slice is not considered real merely because contracts compile.

A slice is only real when:

- extraction works
- records emit
- posture emits
- drift emits
- manifest emits
- Registry reads it
- rendered view stays downstream

## 4. Future-proofing rule

DoppelCore should remain useful whether it is called by:

- ForgeCommand Registry
- a CLI
- another internal app
- a batch scanner
- a future helper agent

This is why the kernel must be deterministic and transportable.

## 5. Final doctrine

The architecture to remember is:

> Cortex extracts.
> DoppelCore tells machine truth.
> Registry governs.
> Renders explain.
