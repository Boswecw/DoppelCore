# Canvas 09 — Risks, Failure Modes, and Anti-Patterns

**Date and Time:** 2026-04-18 07:44:18

## 1. Main risks

### Risk A — semantic inflation
The system begins with bounded facts but slowly starts presenting guesses as truth.

### Risk B — Cortex overload
Cortex gets overloaded with truth-authority responsibilities and loses its bounded identity.

### Risk C — Registry dilution
Registry becomes a vague dashboard instead of a control shell with explicit decision authority.

### Risk D — render inversion
The prettiest output becomes the thing humans trust most, even when machine posture says partial or stale.

### Risk E — schema explosion
Too many record kinds appear before the first proving slice is stable.

## 2. Anti-patterns

Do not do these:

- do not let agents write canonical truth directly
- do not treat confidence as proof
- do not hide `unknown` because it looks ugly
- do not let frontend derive severity or truth class on its own
- do not attempt whole-repo behavioral omniscience in V1
- do not mix repo mutation with mirror derivation

## 3. Protective rules

### Rule 1
Every claim must have evidence refs.

### Rule 2
Every machine bundle must have a manifest.

### Rule 3
Every rendered output must be traceable to machine records.

### Rule 4
Every heuristic must be explicitly marked.

### Rule 5
Every drift state must be visible.

## 4. Honest posture language

Use hard vocabulary:

- `verified`
- `deterministic`
- `partial`
- `stale`
- `blocked`
- `unknown`
- `conflicted`

Avoid fake-safe language like:

- “probably correct”
- “appears complete” without evidence
- “fully mirrored” without manifest proof
