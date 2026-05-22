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
7. `W007_HOST_ROLLOUT_EVIDENCE_COMPARISON_PLANNING`
   - status: in_progress
   - objective: plan typed retained evidence surfaces for host rollout comparisons across per-node values, table slices, effective display, execution outcome, dependency/invalidation evidence, and retained TreeCalc/OneCalc artifacts.
   - current baseline: planning text now reserves host-rollout replay classes and scenario ids; runtime comparison evidence remains open until typed artifacts and retained diff/explain baselines exist.

## Activation note
1. The Rust-first stack is now declared for the repo.
2. `W002` has now emitted retained validator fixtures and baseline outputs for the first bundle/runtime slice.
3. `W003` has now emitted retained conformance fixtures and baseline outputs, including current sibling-manifest acceptance and rejection cases.
4. `W004` is now active over the first retained `OxCalc` and `OxFml` replay intake baselines and the first shared diff control/mismatch runs.
   - current note: the local OxFml replay intake now uses the landed `OxFml_V1` projection seam as its only consumed interface.
   - current note: shared diff now has a local mechanics fixture for XML-style comparison families, a retained pre-publication integrated coverage-gap baseline, and a retained post-publication integrated baseline that consumes real OxFml and OxXlPlay `comparison_views`.
5. `W005` is now active over the first usable `DNA ReCalc` host shell baselines for validate, replay, diff, explain, adapter validation, distill, witness-state, and pack export.
   - current note: retained `DNA ReCalc` replay, diff, and explain evidence for OxFml has been refreshed to the preferred `oxfml-v1-replay-projection` intake path where applicable.
   - current note: retained `DNA ReCalc` explain output now emits machine-readable per-view-family records when the compared artifacts publish `comparison_views`.
   - current note: the current honest integrated XML comparison-view result over real OxFml and OxXlPlay publications is now typed per-family divergence across `comparison_value`, `effective_display_text`, `formatting_view`, and `conditional_formatting_view`, while the OxXlPlay side remains explicitly `lossy` and provenance-rich through `source_metadata`.
6. `W006` is now active over retained distillation and lifecycle-governance examples; broad adapter `C4` and `C5` claims remain later evidence lanes.
7. `W007` is now active as a planning lane for host-rollout evidence comparison. It does not authorize TreeCalc-private shims or upgrade any adapter capability claim.
   - current note: OxReplay-local retained normalized-replay fixtures now exercise exact comparison and explain labeling for `per_node_value`, `table_slice`, `effective_display_text`, `execution_outcome`, `dependency_evidence`, `invalidation_evidence`, and `retained_artifact_ref`; the first real `OxXlPlay` retained structured-table `table_slice` artifact has now been consumed for validation/replay intake, while broader producer integration remains blocked for TreeCalc/OneCalc artifacts and for dependency, invalidation, and retained-artifact-ref families.

## Reserved follow-on lane entry
1. `OxCalc` remains the first lane expected to drive toward `C5.pack_valid`.
2. `OxFml` should first prove ingest, replay, diff, and explain before distillation is widened.
3. `OxFunc` and `OxVba` are later and narrower intake lanes; do not imply broad replay or pack-valid scope for them by default.

## Downstream host note
1. `DNA OneCalc` consumes current `OxReplay` surfaces as shared infrastructure, not as a second replay-host contract.
2. The current honest floor for that consumer is accepted `OxFml` `C0` through `C3` plus the first accepted `OxXlPlay` observation-source seam.
3. Direct `OxFunc` and `OxVba` replay intake remain later narrower lanes.
4. Host rollout evidence for `DNA OneCalc` and `DNA TreeCalc` must enter through typed replay evidence surfaces with source lineage, not through private host helper types.
5. Current local W007 evidence compares OxReplay-owned retained normalized-replay artifacts only; it is not a capability claim for `DNA OneCalc`, `DNA TreeCalc`, or `OxXlPlay`.

## Activation rule
Move a workset to `in_progress` only when:
1. scope is explicit,
2. dependencies are known,
3. capability and pack impact are named,
4. no lane-semantic ownership drift is introduced.
