# IN_PROGRESS_FEATURE_WORKLIST.md — OxReplay

## Active bootstrap worksets

1. `W001_REPO_BOOTSTRAP_AND_RUNTIME_STRATA`
   - status: planned
   - objective: lock repo skeleton, runtime strata, and first package map.
2. `W002_BUNDLE_AND_SCHEMA_RUNTIME`
   - status: planned
   - objective: stand up canonical bundle parsing, validation, and indexing.
3. `W003_ADAPTER_CAPABILITY_AND_CONFORMANCE_HARNESS`
   - status: planned
   - objective: validate adapter manifests and capability claims.
4. `W004_OXCALC_OXFML_ADAPTER_INTAKE_AND_REPLAY_PATH`
   - status: planned
   - objective: exercise initial adapters from the first two integrated lanes.
5. `W005_DNA_RECALC_CLI_SHELL_AND_PACK_EXPORT_BASELINE`
   - status: planned
   - objective: provide the first usable host shell and pack-facing replay export path.
6. `W006_WITNESS_DISTILLATION_AND_LIFECYCLE_GOVERNANCE_BASELINE`
   - status: planned
   - objective: stand up predicate-driven witness reduction and lifecycle/quarantine handling.

## Reserved follow-on lane entry
1. `OxCalc` remains the first lane expected to drive toward `C5.pack_valid`.
2. `OxFml` should first prove ingest, replay, diff, and explain before distillation is widened.
3. `OxFunc` and `OxVba` are later and narrower intake lanes; do not imply broad replay or pack-valid scope for them by default.

## Activation rule
Move a workset to `in_progress` only when:
1. scope is explicit,
2. dependencies are known,
3. capability and pack impact are named,
4. no lane-semantic ownership drift is introduced.
