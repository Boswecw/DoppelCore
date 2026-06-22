        # DoppelCore - Compiled System Reference

        **Designation:** DOP
        **Document role:** Canonical compiled technical reference for the DoppelCore Rust contract library
        **Source:** `doc/system/`
        **Build command:** `bash doc/system/BUILD.sh`
        **Document version:** 2.0 (2026-06-22) - canonical compliance migration
        **Protocol:** BDS Documentation Protocol v2.0; BDS Repo Documentation System Canonical Compliance Standard

        > **Generated artifact warning:** `doc/DOPSYSTEM.md` is assembled output. Edit
        > the source modules under `doc/system/` and rebuild. Hand edits to the
        > compiled artifact are overwritten by the next build.

        Assembly contract:

        - Command: `bash doc/system/BUILD.sh`
        - Validation: `bash doc/system/validate_snapshots.sh` runs during assembly
        - Primary output: `doc/DOPSYSTEM.md`

        This `doc/system/` tree is the canonical source of truth for DoppelCore. It uses
        explicit **truth classes**: canonical facts define repo role, authority
        boundaries, contract behavior, runtime behavior, and verification doctrine;
        snapshot facts are dated, audit-derived counts and current implementation
        inventory that may drift between audits.

        | Part | File | Contents |
        | --- | --- | --- |
        | §1 | `00_overview/01-overview.md` | 01 Overview |
| §2 | `10_service-contract/02-contract-surface.md` | 02 Contract Surface |
| §3 | `20_runtime/03-runtime-boundary.md` | 03 Runtime Boundary |
| §4 | `30_dependencies/04-dependencies.md` | 04 Dependencies |
| §5 | `40_governance/05-governance.md` | 05 Governance |
| §6 | `50_operations/06-verification.md` | 06 Verification |
| §7 | `99_appendices/90-appendices.md` | 90 Appendices |

        ## Quick Assembly

        ```bash
        bash doc/system/BUILD.sh
        ```

---

            # Overview

            **Document version:** 2.0 (2026-06-22) - canonical compliance migration

            DoppelCore is a standalone Rust library repo for the portable truth-core contract surface that was extracted from Forge_Command.

The current slice establishes the repo as an internal ecosystem component without pulling in UI ownership, registry orchestration, Self-Healing command ownership, LocalDb ownership, or execution-lane behavior.

This documentation tree is a canonical starting point for repo-local system truth. It does not replace the authored planning material under `docs/doppelcore_canvas_set/`.

---

            # Contract Surface

            **Document version:** 2.0 (2026-06-22) - canonical compliance migration

            The repo-owned contract surface lives in `src/` and is organized around contracts, records, correction, comparison, and error types.

The current boundary is library-contract publication. DoppelCore may expose portable types and validation helpers, but this document does not admit database adapters, Tauri commands, registry orchestration, or correction-fabric handoff behavior.

---

            # Runtime Boundary

            **Document version:** 2.0 (2026-06-22) - canonical compliance migration

            DoppelCore is not a standalone runtime in the current slice. It has no direct execution lane and no service process documented by this source tree.

Runtime claims must be introduced only after executable proof lands in the repository and the system chapters are rebuilt from source.

---

            # Dependencies

            **Document version:** 2.0 (2026-06-22) - canonical compliance migration

            The repository is a Rust library crate. Dependency truth is owned by `Cargo.toml` and `Cargo.lock`.

Any dependency inventory in generated system docs is a snapshot fact and must be refreshed from the manifests before release claims are made.

---

            # Governance

            **Document version:** 2.0 (2026-06-22) - canonical compliance migration

            The slice boundary is intentionally narrow: establish a standalone library and avoid first-slice cross-repo coupling.

The repo must remain free of UI, registry, Self-Healing, LocalDb, and execution ownership until a later governed extraction slice admits those responsibilities.

---

            # Verification

            **Document version:** 2.0 (2026-06-22) - canonical compliance migration

            The README names the first proof gate as:

```bash
cargo test
cargo check
```

Run these commands from the repository root before claiming the library is ready for the next extraction slice.

---

            # Appendices

            **Document version:** 2.0 (2026-06-22) - canonical compliance migration

            Supporting authored material lives in:

- `APPLY.md`
- `VERIFY.md`
- `docs/doppelcore_canvas_set/`

Those files remain source evidence. This compiled system reference summarizes the current canonical repo boundary.
