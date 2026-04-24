# Canvas 06 — Cortex Boundary and Cooperation Model

**Date and Time:** 2026-04-21 00:00:00

## 1. Why this canvas exists

The easiest failure here is still role collapse, but the collapse risk has changed.

The old risk was that Cortex would become extractor, interpreter, truth engine, and renderer all at once.

The new risk is broader:

* Cortex becomes more than extraction
* Centipede becomes unbounded semantic authority
* DoppelCore becomes a vague duplicate of intake logic
* Registry starts consuming mixed, non-normalized products

This canvas exists to preserve a clean chain of authority between **extraction**, **intake/reconciliation**, **machine-truth formation**, and **registry governance**.

## 2. Updated boundary model

The updated model is:

```text
Code
  -> Cortex extraction packets
  -> Centipede intake + reconciliation
  -> DoppelCore truth records / claims / posture / drift
  -> Registry governance / verification / enforcement
  -> rendered human surfaces
```

This replaces the older direct cooperation model:

```text
Cortex packet -> DoppelCore normalization -> Registry decisioning
```

The reason for the change is architectural maturity:

* Cortex is now upstream extraction
* Centipede is now the stronger intake and reconciliation spine
* DoppelCore remains the machine-truth kernel
* Registry remains the outer control shell

## 3. Cortex charter

Cortex remains the bounded extraction subsystem.

Allowed responsibilities:

* walk files
* gather safe text metadata
* parse syntax-level structures
* emit extraction packets
* prepare handoff data
* support bounded extraction profiles

Not allowed by default:

* final truth authority
* silent semantic upgrading
* governance decision making
* repo mutation during extraction
* direct registry decisioning
* direct proposal authority
* direct self-healing action authority

Cortex is upstream signal production, not canonical truth.

## 4. Centipede charter relative to Cortex

Centipede is now the governed intake and reconciliation layer.

Allowed responsibilities:

* receive upstream producer packets
* structure intake evidence into governed reconciliation products
* retain run identity and provenance
* record lane admission outcomes
* retain decision traces
* reconcile multi-source intake into operator-meaningful incident formation
* serve bounded downstream read models for operator surfaces

Not allowed by default:

* final canonical machine-truth authority
* silent mutation of registry truth
* direct execution authority
* hidden proposal approval
* unbounded semantic reinterpretation outside governed contracts

Centipede is the intake spine, not the final truth kernel.

## 5. DoppelCore charter relative to Centipede

DoppelCore remains the inner canonical machine-truth layer.

Its responsibilities are:

* receive bounded, provenance-carrying products from Centipede
* normalize them into canonical truth records
* maintain governed claims, posture, drift, and code-mirror fidelity
* preserve machine-readable truth for registry consumption
* support deterministic rendered human surfaces

DoppelCore is where the system crosses from **intake evidence** into **machine-truth representation**.

That means DoppelCore should not be bypassed by either Cortex or Centipede when canonical truth is being formed.

## 6. Registry cooperation pattern

Registry remains the outer governance and enforcement shell.

Registry consumes **DoppelCore truth products**, not raw Cortex extraction and not ad hoc Centipede intake packets.

That preserves a disciplined boundary:

* Cortex produces bounded extraction evidence
* Centipede reconciles and structures intake evidence
* DoppelCore forms machine-truth records
* Registry enforces governance and verification
* human-readable documents remain rendered surfaces only

## 7. Why this is now the clean split

This split gives each layer a distinct identity:

* **Cortex** stays reusable as extraction infrastructure
* **Centipede** becomes the robust intake/reconciliation spine
* **DoppelCore** becomes the machine-truth kernel
* **Registry** remains the governance shell

That is cleaner than either of these bad alternatives:

### Bad alternative A

Cortex becomes extraction + interpretation + truth + governance

### Bad alternative B

Centipede becomes intake + truth + approval + registry logic

### Bad alternative C

DoppelCore becomes just another name for the intake layer

All three create authority drift and make the mirror less trustworthy.

## 8. Current implementation truth note

Current ForgeCommand implementation has not yet fully reached this target-state doctrine.

At present, Centipede in repo code is still at the ledger/read-surface stage:

* durable recording of run creation
* lane admission outcomes
* decision traces
* bounded recent-run and per-run detail hydration

Weighted reconciliation and fuller operator coordination are still explicitly beyond the current tranche boundary.

So this canvas defines the intended doctrinal direction, not a claim that the full target-state is already implemented.

## 9. Recommendation

Keep Cortex as extraction.
Keep Centipede as intake and reconciliation.
Build DoppelCore as the machine-truth kernel.
Keep Registry as the outer governance shell.

Do not collapse those roles.

The desired end-state is:

```text
Code
  -> Cortex extraction
  -> Centipede intake/reconciliation
  -> DoppelCore truth records
  -> Registry governance/verification
  -> rendered human surfaces
```