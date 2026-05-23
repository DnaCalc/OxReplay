# CURRENT_BLOCKERS.md — OxReplay

Status: active blockers present.

Last reviewed: 2026-05-23.

---

## Active Blockers

### BLK-REPLAY-003: OxFunc shared value-types wire helper not admitted yet

- **Status**: active
- **Impact**: final switch from the local `comparison_value` wire comparator seam to direct shared OxFunc-owned type reuse across `W004`, `W005`, and W007/W056 producer evidence
- **Current state**: `../OxFunc` now includes `crates/oxfunc_value_types`, but there is not yet an admitted serde or replay-wire helper surface that `OxReplay` can consume directly for the `comparison_value` family; `OxReplay` has isolated its comparator seam locally and is still normalizing replay JSON into typed comparisons inside `src/oxreplay-diff/src/lib.rs`. The local seam now accepts the declared `oxfunc_value_types.aligned_json.v1` envelope emitted by current `DnaTreeCalc`/`OxXlPlay` W056 artifacts, and retained table-update-oracle self-diff evidence is equivalent at `docs/test-runs/oxxlplay-seam-xlplay_table_update_oracle_001-baseline/`, but this remains a workaround rather than the final shared-value implementation.
- **Exact unblock steps**: admit the OxFunc-owned serde or replay-wire helper surface for published-formula-result comparison values, add the narrow dependency in `OxReplay`, and replace the local JSON-to-typed normalization helper in `src/oxreplay-diff/src/lib.rs` with direct shared-type comparison
- **Recommendation**: workaround
- **Opened**: 2026-04-14
- **Last reviewed**: 2026-05-23

### BLK-REPLAY-004: W056 third-pass producer artifact gaps

- **Status**: active
- **Impact**: full closure of `oxreplay-qb9` and W007/W056 third-pass table evidence intake
- **Current state**: `OxReplay` now retains W056 intake evidence for the landed `DnaTreeCalc` W056 producer artifact and current `OxXlPlay` structured-reference, WorkbookConstructionSpec, table-construction, and table-update-oracle artifacts at `docs/test-runs/w007-w056-table-third-pass-intake-baseline/`. The refreshed table-update-oracle artifact carries `execution_outcome.class_id`, self-compares cleanly, and now includes explicit empty data-body observations, first-row insert, last-row delete, empty-table column rename, current-row absence diagnostics, and multi-table/name/anchor collision availability. The structured-reference, WorkbookConstructionSpec, and table-construction artifacts still omit top-level `execution_outcome.class_id`, so their self-diff records remain typed outcome seam drift. DnaTreeCalc now activates empty-body tables through LiveOxCalc, but no retained DnaTreeCalc empty-body replay artifact is landed yet, so empty-body and first/last-row closure remain cross-producer partial rather than paired. No retained lifecycle callback artifact or full namespace/anchor/workspace cross-producer artifact has landed. `OxXlPlay` also still declares Excel dependency graph, dirty-set, and invalidation event-order evidence unavailable through the current COM capture path.
- **Exact unblock steps**: land producer artifacts with explicit `execution_outcome.class_id` for all W056 table evidence families; land retained DnaTreeCalc empty-body and first/last-row transition artifacts that can be paired with the refreshed OxXlPlay observations; land lifecycle callback replay artifacts; land namespace/anchor/workspace paired artifacts; and either emit direct or explicitly derived dependency/invalidation evidence from `OxXlPlay` with interpretation limits, or keep those lanes explicitly unavailable.
- **Recommendation**: workaround
- **Opened**: 2026-05-23

### BLK-REPLAY-002: OxCalc manifest C4 lifecycle gap

- **Status**: active
- **Impact**: `W003` sibling-manifest acceptance, `W004` honest capability intake, and any local acceptance of `OxCalc` `cap.C4.distill_valid`
- **Current state**: the retained `OxReplay` W003 conformance baseline rejects `../OxCalc/docs/spec/core-engine/CORE_ENGINE_REPLAY_ADAPTER_CAPABILITY_MANIFEST_V1.json` because it claims `cap.C4.distill_valid` without declaring lifecycle states
- **Exact unblock steps**: either add explicit lifecycle states to the `OxCalc` manifest and keep the `C4` claim, or downgrade the current claim to `C3` and keep `C4` as target or scaffolded until lifecycle evidence is exposed
- **Recommendation**: escalate
- **Opened**: 2026-03-18

---

## Resolved Blockers

### BLK-REPLAY-001: First implementation stack undeclared

- **Status**: resolved
- **Impact**: `W002` through `W006` activation packet quality
- **Current state**: Rust-first implementation direction is now declared, the active Cargo workspace root is `src/`, workspace checks are explicit, and the workset packets now reference the chosen stack
- **Exact unblock steps**: none; resolved by the Rust-first baseline update and execution-packet expansion
- **Recommendation**: workaround
- **Opened**: 2026-03-16
- **Resolved**: 2026-03-16

---

## Entry Template

```text
### BLK-REPLAY-NNN: <title>

- **Status**: active | resolved | closed
- **Impact**: <which worksets/features are blocked>
- **Current state**: <what has been attempted, what failed>
- **Exact unblock steps**: <specific actions needed>
- **Recommendation**: wait | escalate | workaround
- **Opened**: YYYY-MM-DD
- **Resolved**: YYYY-MM-DD (if applicable)
```
