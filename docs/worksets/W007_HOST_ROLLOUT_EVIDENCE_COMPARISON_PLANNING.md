# W007 — Host Rollout Evidence Comparison Planning

## Objective
Plan the next `OxReplay` support tranche for retained host-rollout evidence consumed by `DNA OneCalc`, `DNA TreeCalc`, and `OxXlPlay` without creating TreeCalc-private shims or absorbing lane semantics.

## Scope
1. compare per-node values through declared typed comparison-view families,
2. compare table slices as a typed replay view family rather than as host-private JSON,
3. compare effective display through `effective_display_text` with render-context trust status,
4. compare execution outcome through the existing typed `execution_outcome` family with explicit `outcome_kind`, `outcome_stage`, `class_id`, and optional `lane_reason_code`,
5. plan dependency and invalidation evidence intake as replay-facing typed surfaces with source-lane provenance and capture-loss status,
6. retain `DNA TreeCalc` and `DNA OneCalc` artifacts only as replay-governed corpus inputs or host references, never as private shared-runtime shims.

## Dependencies
1. `W004_OXCALC_OXFML_ADAPTER_INTAKE_AND_REPLAY_PATH`
2. `W005_DNA_RECALC_CLI_SHELL_AND_PACK_EXPORT_BASELINE`
3. `W006_WITNESS_DISTILLATION_AND_LIFECYCLE_GOVERNANCE_BASELINE`
4. accepted `OxXlPlay` observation-source seam for replay-facing typed views
5. host or lane outputs that publish typed evidence surfaces with source identity and schema lineage

## Exit gate
1. the local spec set names the planned comparison families and scenario ids,
2. the downstream-host consumption model states that host artifacts must come through typed replay evidence surfaces,
3. the `OxXlPlay` outbound note records the required emitted observation families,
4. no document authorizes direct linkage to TreeCalc-private or OneCalc-private helper types,
5. the local runtime has retained normalized-replay fixtures proving comparison and explain output over the declared families.

## Expected capability impact
1. plans a `C2`/`C3` comparison and explain widening over typed host-rollout evidence,
2. does not claim a new adapter capability level for `OxXlPlay`, `DNA OneCalc`, or `DNA TreeCalc`,
3. does not change current `C4` or `C5` floors.
4. the executable fixture slice is local OxReplay comparison support only; producer-side capability remains blocked until upstream repos emit retained artifacts.

## Expected pack impact
1. planned support for `PACK.replay.appliance` through retained host-rollout replay inputs,
2. planned support for `PACK.diff.cross_engine.continuous` through per-family comparison evidence,
3. planned support for `PACK.trace.forensic_plane` through dependency, invalidation, and execution-outcome provenance,
4. no pack claim exists until retained replay, diff, and explain artifacts are emitted.

## Environment Preconditions
1. typed comparison-view families are present in input artifacts,
2. inputs preserve source lane or host identity, schema lineage, capture mode, projection status, and registry refs when applicable,
3. retained host artifacts are addressed through repo-relative paths or sibling evidence refs,
4. validation runs do not mutate checked-in baselines.

## Evidence Layout
1. planned retained corpus roots:
   - `docs/test-corpus/bundles/host_rollout_per_node_values_001/`
   - `docs/test-corpus/bundles/host_rollout_table_slice_001/`
   - `docs/test-corpus/bundles/host_rollout_table_update_oracle_001/`
   - `docs/test-corpus/bundles/host_rollout_effective_display_001/`
   - `docs/test-corpus/bundles/host_rollout_execution_outcome_001/`
   - `docs/test-corpus/bundles/host_rollout_dependency_invalidation_001/`
   - `docs/test-corpus/bundles/host_rollout_retained_artifacts_001/`
2. planned retained run roots:
   - `docs/test-runs/w007-host-rollout-<scenario-id>-baseline/`
3. current retained local fixture roots:
   - `docs/test-corpus/bundles/host_rollout_per_node_values_001/`
   - `docs/test-corpus/bundles/host_rollout_table_slice_001/`
   - `docs/test-corpus/bundles/host_rollout_effective_display_001/`
   - `docs/test-corpus/bundles/host_rollout_execution_outcome_001/`
   - `docs/test-corpus/bundles/host_rollout_dependency_invalidation_001/`
   - `docs/test-corpus/bundles/host_rollout_retained_artifacts_001/`
4. current retained run roots:
   - `docs/test-runs/w007-host-rollout-host_rollout_per_node_values_001-baseline/`
   - `docs/test-runs/w007-host-rollout-host_rollout_table_slice_001-baseline/`
   - `docs/test-runs/w007-host-rollout-host_rollout_table_update_oracle_001-baseline/`
   - `docs/test-runs/w007-host-rollout-host_rollout_effective_display_001-baseline/`
   - `docs/test-runs/w007-host-rollout-host_rollout_execution_outcome_001-baseline/`
   - `docs/test-runs/w007-host-rollout-host_rollout_dependency_invalidation_001-baseline/`
   - `docs/test-runs/w007-host-rollout-host_rollout_retained_artifacts_001-baseline/`
   - `docs/test-runs/oxxlplay-seam-xlplay_structured_reference_workbook_001-baseline/`
5. checked-in versus ephemeral policy:
   - promoted host-rollout fixtures and baseline reports are checked in,
   - exploratory host exports remain ephemeral until they satisfy source-preservation and typed-surface requirements.

## Current Executable Slice
1. `OxReplay` compares the declared W007 families as retained comparison views from normalized replay artifacts.
2. `per_node_value`, `table_slice`, `dependency_evidence`, `invalidation_evidence`, and `retained_artifact_ref` currently use exact JSON comparison with family-specific policy ids and mismatch labels.
3. `effective_display_text` keeps render-context trust detail.
4. `execution_outcome` keeps typed outcome-class comparison and rejects legacy outcome-family shims.
5. This slice does not infer TreeCalc reference semantics, table semantics, invalidation semantics, or OneCalc host policy; it compares retained payloads only.
6. The `OxXlPlay` structured-reference workbook scenario now provides the first real retained upstream `table_slice` artifact that validates and replays through the local normalized-replay intake path.
7. The `DnaTreeCalc` W056 table producer scenario now provides the first retained TreeCalc table artifact accepted by OxReplay validation/replay intake: `../DnaTreeCalc/docs/test-runs/w056-table-structured-references-001/` plus retained OxReplay intake output at `docs/test-runs/dnatreecalc-w056-table-structured-references-001-baseline/`.
8. The `OxXlPlay` table-update oracle scenario now validates and replays through OxReplay at `docs/test-runs/oxxlplay-seam-xlplay_table_update_oracle_001-baseline/`. OxReplay admits `table_update_oracle` as an opaque exact-JSON comparison family with policy `table_update_oracle_json_exact`; this policy compares retained observation payloads only and does not infer Excel or TreeCalc table semantics. Its retained self-diff/explain still records remaining comparison blockers: top-level `comparison_value` depends on the shared value-wire decision, and `execution_outcome` needs `class_id`.
9. Real upstream integration remains partial until matched cross-producer diff/explain evidence exists for TreeCalc and Excel-observed table/update artifacts, and until shared value/outcome envelope gaps are resolved or explicitly typed as exclusions. Current evidence is accepted intake plus the local exact retained-oracle comparison policy, not a broad adapter capability claim.

## Replay-Corpus Readiness
1. planned replay classes:
   - `host_rollout_per_node_values`
   - `host_rollout_table_slice`
   - `host_rollout_table_update_oracle`
   - `host_rollout_effective_display`
   - `host_rollout_execution_outcome`
   - `host_rollout_dependency_invalidation`
   - `host_rollout_retained_artifacts`
2. planned scenario ids:
   - `host_rollout_per_node_values_001`
   - `host_rollout_table_slice_001`
   - `host_rollout_table_update_oracle_001`
   - `host_rollout_effective_display_001`
   - `host_rollout_execution_outcome_001`
   - `host_rollout_dependency_invalidation_001`
   - `host_rollout_retained_artifacts_001`
3. reserve or later lanes:
   - registry-pinned witness promotion over host-rollout evidence,
   - distillation of host evidence beyond retained-local examples,
   - final product verdict policy.

## Pack-Evidence Traceability
1. `PACK.replay.appliance`
   - planned retained input validation and replay runs for each host-rollout scenario.
2. `PACK.diff.cross_engine.continuous`
   - planned diff and explain baselines over per-node value, table-slice, display, outcome, dependency, and invalidation view families.
3. `PACK.trace.forensic_plane`
   - planned lineage, dependency, invalidation, source-metadata, render-context, and capture-loss evidence.
