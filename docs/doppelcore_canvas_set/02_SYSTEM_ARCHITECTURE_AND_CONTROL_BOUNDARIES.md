# Canvas 02 — System Architecture and Control Boundaries

**Date and Time:** 2026-04-18 07:44:18

## 1. Architectural position

DoppelCore sits between bounded extraction and governance.

```text
Repo / commit / filesystem reality
        ↓
Cortex extraction and bounded probes
        ↓
DoppelCore canonical truth records
        ↓
Registry control, compliance, remediation, verification
        ↓
Rendered human outputs and operator views
```

## 2. Layer responsibilities

## Layer A — Cortex

Cortex is responsible for:

- walking files
- reading safe text and structure
- parsing syntax-level facts
- building extraction packets
- never silently becoming semantic authority

## Layer B — DoppelCore

DoppelCore is responsible for:

- defining canonical record contracts
- converting extraction evidence into governed truth records
- attaching posture and drift
- separating deterministic and inferred classes
- emitting manifests and machine products

## Layer C — Registry

Registry is responsible for:

- deciding what to scan
- selecting scan profiles
- running compliance workflows
- managing designation and governance state
- handling remediation and verification
- publishing results

## Layer D — Human surface

Human surfaces are responsible for:

- reviewability
- comprehension
- operator navigation
- summaries and diagrams

Human surfaces are not responsible for truth authorship.

## 3. Control boundary rules

### Rule A — Cortex cannot publish final truth
Cortex may emit extraction packets and bounded evidence only.

### Rule B — DoppelCore cannot silently mutate repos
DoppelCore is read/evaluate/emit only.

### Rule C — Registry owns action authority
Any write, scaffold, remediation, verification, or approval lives in Registry.

### Rule D — UI cannot invent posture
Frontend must consume backend-owned posture and determinism classes.

## 4. Why this split matters

This split prevents two common failures:

- extraction nodes pretending to understand more than they do
- governance shells pretending documentation is authoritative when it is only rendered output

## 5. Canonical naming doctrine

Recommended naming inside ForgeCommand:

- `cortex` = bounded extraction subsystem
- `doppelcore` = machine truth kernel
- `registry` = governance/control shell
- `render` = human output generation layer

## 6. Minimal internal module shape

Suggested internal module group:

```text
src-tauri/src/doppelcore/
  mod.rs
  records.rs
  contracts.rs
  posture.rs
  drift.rs
  manifests.rs
  derive.rs
  emit.rs
  profiles.rs
  errors.rs
```
