# HANDOFF_DNATREECALC_001_W056_TABLE_RETAINED_COMPARISON_ARTIFACTS

## Header
1. handoff id: `HANDOFF_DNATREECALC_001`
2. date: `2026-05-23`
3. from repo: `OxReplay`
4. to repo: `DnaTreeCalc`
5. related workset or feature id: `W007/W056`, `oxreplay-p1w.1`, `OxCalc calc-4vs8.26`

## Purpose
Record the retained producer artifacts needed before `OxReplay` can close the TreeCalc side of W056 table comparison evidence.

`OxReplay` already accepts declared comparison views and can compare retained JSON payloads without parsing structured-reference text or reimplementing table semantics. The current blocker is the absence of retained `DnaTreeCalc` normalized-replay artifacts for the W056 table scenarios.

## Proposed change
`DnaTreeCalc` should emit retained replay-facing artifacts for W056 table comparison scenarios with:

1. a canonical replay manifest or normalized-replay artifact that preserves `scenario_id`, `lane_id` or `source_host`, `source_schema_id`, `projection_status`, `capture_mode`, `capture_loss`, and `registry_refs` where applicable,
2. declared `comparison_views` rather than private host strings,
3. repo-relative artifact refs for any sidecars consumed by the view payloads,
4. source metadata that distinguishes direct capture from derived projection and names unavailable surfaces explicitly.

Required comparison-view families for the TreeCalc producer side:

1. `table_slice`
   - producer-declared table slice payload with table identity, row identity/order, column identity/order, header/data/totals region facts, and table-cell payloads,
   - no OxReplay parsing of structured-reference formula text or reconstruction of table semantics.
2. `per_node_value`
   - per-node and table-cell value facts with stable node/table/row/column identity and declared value payloads,
   - table-cell values may carry `comparison_value` envelopes and display text as nested declared facts, but OxReplay treats the family payload as retained evidence unless a shared value wire helper is admitted.
3. `effective_display_text`
   - effective display text where TreeCalc can state it, with render/projection trust status in source metadata.
4. `execution_outcome`
   - typed outcome payload with `outcome_kind`, `outcome_stage`, `class_id`, and optional `lane_reason_code`, or an explicit producer-side blocker if the class mapping is not yet available.
5. `dependency_evidence`
   - retained facts for row membership/order, column identity, header/data/totals regions, caller row context, referenced value facts, and structured-reference dependency edges where the producer owns them.
6. `invalidation_evidence`
   - retained facts for table/node rename, move, delete, row/column membership changes, header/totals changes, value changes, and structural rebind decisions where the producer owns them.
7. `retained_artifact_ref`
   - stable refs to the emitted producer artifacts, sidecars, source workspace/case ids, hashes where available, and any known capture/projection limits.

For cross-producer comparison against Excel-observed fixtures, publish a shared scenario alias or mapping that pairs the TreeCalc W056 scenario with the retained `OxXlPlay` structured table observation when appropriate.

## Current evidence
1. OxReplay W007 local mechanics fixtures:
   - `docs/test-corpus/bundles/host_rollout_table_slice_001/`
   - `docs/test-corpus/bundles/host_rollout_per_node_values_001/`
   - `docs/test-corpus/bundles/host_rollout_effective_display_001/`
   - `docs/test-corpus/bundles/host_rollout_execution_outcome_001/`
   - `docs/test-corpus/bundles/host_rollout_dependency_invalidation_001/`
   - `docs/test-corpus/bundles/host_rollout_retained_artifacts_001/`
2. OxReplay W007 retained runs:
   - `docs/test-runs/w007-host-rollout-host_rollout_table_slice_001-baseline/`
   - `docs/test-runs/w007-host-rollout-host_rollout_per_node_values_001-baseline/`
   - `docs/test-runs/w007-host-rollout-host_rollout_effective_display_001-baseline/`
   - `docs/test-runs/w007-host-rollout-host_rollout_execution_outcome_001-baseline/`
   - `docs/test-runs/w007-host-rollout-host_rollout_dependency_invalidation_001-baseline/`
   - `docs/test-runs/w007-host-rollout-host_rollout_retained_artifacts_001-baseline/`
3. Accepted Excel-observed structured table intake:
   - `../OxXlPlay/states/excel/xlplay_structured_reference_workbook_001/oxreplay-manifest.json`
   - `../OxXlPlay/states/excel/xlplay_structured_reference_workbook_001/views/normalized-replay.json`
   - `../OxXlPlay/states/excel/xlplay_structured_reference_workbook_001/views/table-slice.json`
   - `docs/test-runs/oxxlplay-seam-xlplay_structured_reference_workbook_001-baseline/`
4. Current DnaTreeCalc inspected table inputs are planning/workspace fixtures, not replay-facing producer artifacts:
   - `../DnaTreeCalc/docs/test-corpus/workspaces/tables.json`
   - `../DnaTreeCalc/docs/test-corpus/tables/structured-references.json`
   - `../DnaTreeCalc/docs/handovers/HANDOVER_OXCALC_table_node_model.md`

## Impact
1. capability impact: no new OxReplay adapter capability claim; this handoff defines the producer artifact requirements needed before W007/W056 TreeCalc intake can move beyond blocked.
2. pack impact: candidate evidence for `PACK.replay.appliance`, `PACK.diff.cross_engine.continuous`, and `PACK.trace.forensic_plane` once retained producer artifacts exist.
3. migration or fallback impact: until these artifacts exist, OxReplay can only report local mechanics fixtures plus the accepted OxXlPlay table-slice intake.
4. affected repos or hosts: `DnaTreeCalc`, `OxCalc`, `OxReplay`, `OxXlPlay`, `DNA ReCalc`, downstream `DNA OneCalc` comparison consumers.

## Requested response
1. emit retained W056 TreeCalc normalized-replay artifacts for the comparison-view families listed above, or state which families are intentionally unavailable,
2. preserve source lineage and capture/projection limits in machine-readable metadata,
3. provide a scenario alias/mapping to the corresponding Excel-observed fixture when cross-producer comparison is intended,
4. do not require OxReplay to parse private structured-reference strings, infer table semantics, or import host-private helper types.
