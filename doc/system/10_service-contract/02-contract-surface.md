            # Contract Surface

            **Document version:** 2.0 (2026-06-22) - canonical compliance migration

            The repo-owned contract surface lives in `src/` and is organized around contracts, records, correction, comparison, and error types.

The current boundary is library-contract publication. DoppelCore may expose portable types and validation helpers, but this document does not admit database adapters, Tauri commands, registry orchestration, or correction-fabric handoff behavior.
