# IN_PROGRESS_FEATURE_WORKLIST.md — OxReplay

## Active bootstrap worksets

1. `W001_REPO_BOOTSTRAP_AND_RUNTIME_STRATA`
   - status: complete
   - objective: lock repo skeleton, runtime strata, and first package map.
2. `W002_BUNDLE_AND_SCHEMA_RUNTIME`
   - status: complete
   - objective: stand up canonical bundle parsing, validation, and indexing.
3. `W003_ADAPTER_CAPABILITY_AND_CONFORMANCE_HARNESS`
   - status: complete
   - objective: validate adapter manifests and capability claims.
4. `W004_OXCALC_OXFML_ADAPTER_INTAKE_AND_REPLAY_PATH`
   - status: in_progress
   - objective: exercise initial adapters from the first two integrated lanes.
5. `W005_DNA_RECALC_CLI_SHELL_AND_PACK_EXPORT_BASELINE`
   - status: in_progress
   - objective: provide the first usable host shell and pack-facing replay export path.
6. `W006_WITNESS_DISTILLATION_AND_LIFECYCLE_GOVERNANCE_BASELINE`
   - status: in_progress
   - objective: stand up predicate-driven witness reduction and lifecycle/quarantine handling.

## Activation note
1. The Rust-first stack is now declared for the repo.
2. `W002` has now emitted retained validator fixtures and baseline outputs for the first bundle/runtime slice.
3. `W003` has now emitted retained conformance fixtures and baseline outputs, including current sibling-manifest acceptance and rejection cases.
4. `W004` is now active over the first retained `OxCalc` and `OxFml` replay intake baselines and the first shared diff control/mismatch runs.
5. `W005` is now active over the first usable `DNA ReCalc` host shell baselines for validate, replay, diff, explain, adapter validation, distill, witness-state, and pack export.
6. `W006` is now active over retained distillation and lifecycle-governance examples; broad adapter `C4` and `C5` claims remain later evidence lanes.

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
