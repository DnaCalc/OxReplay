# W005 — DNA ReCalc CLI Shell And Pack Export Baseline

## Objective
Provide the first usable `DNA ReCalc` shell over `OxReplay`.

## Scope
1. CLI command family shell.
2. Bundle validate and replay entry points.
3. Diff and explain entry points.
4. Adapter validation entry points.
5. Pack-facing replay export baseline.

## Dependencies
1. `W002`
2. `W003`
3. `W004`

## Exit gate
1. `DNA ReCalc` host role is executable in repo-local terms.
2. Pack-facing output path is explicit.
3. Host remains a replay surface, not a semantics owner.

## Expected capability impact
1. exposes already-validated replay surfaces through the host shell
2. `W005` does not create new lane capability claims by itself

## Expected pack impact
1. candidate host-facing export path for `PACK.replay.appliance`
2. no pack claim exists until retained host-run artifacts and export outputs are linked

## Environment Preconditions
1. `W002` through `W004` provide runnable validation, replay, diff, and adapter-intake surfaces
2. CLI command-family shape is named before activation
3. adapter loading remains deterministic and file- or CLI-oriented
4. the host does not link directly to lane-semantic internals

## Evidence Layout
1. canonical emitted artifact root: `docs/test-runs/` for retained CLI smoke runs and `docs/test-corpus/bundles/` for checked-in example inputs
2. checked-in versus ephemeral policy: baseline CLI runs and export examples are checked in when promoted; ad hoc developer runs may be ephemeral
3. stable naming policy for baseline runs: `w005-dnarecalc-<command-family>-baseline`

## Replay-Corpus Readiness
1. replay classes requiring corpus scenarios before activation: `cli_validate`, `cli_replay`, `cli_diff`, `cli_explain`, `cli_adapter_validate`, `pack_export`
2. scenario ids satisfying them:
   - `cli_validate` -> `host_validate_bundle_001`
   - `cli_replay` -> `host_replay_bundle_001`
   - `cli_diff` -> `host_diff_bundle_001`
   - `cli_explain` -> `host_explain_bundle_001`
   - `cli_adapter_validate` -> `host_validate_adapter_001`
   - `pack_export` -> `host_pack_export_001`
3. reserve or later lanes: optional UI flows and widened host integration

## Pack-Evidence Traceability
1. pack name: `PACK.replay.appliance`
2. replay classes: `cli_validate`, `cli_replay`, `cli_diff`, `cli_explain`, `cli_adapter_validate`, `pack_export`
3. scenario ids or artifact paths:
   - `host_validate_bundle_001` -> `docs/test-corpus/bundles/host_validate_bundle_001/`
   - `host_replay_bundle_001` -> `docs/test-corpus/bundles/host_replay_bundle_001/`
   - `host_diff_bundle_001` -> `docs/test-corpus/bundles/host_diff_bundle_001/`
   - `host_explain_bundle_001` -> `docs/test-corpus/bundles/host_explain_bundle_001/`
   - `host_validate_adapter_001` -> `docs/test-corpus/bundles/host_validate_adapter_001/`
   - `host_pack_export_001` -> `docs/test-corpus/bundles/host_pack_export_001/`
   - retained human-readable runs -> `docs/test-runs/w005-dnarecalc-<command-family>-baseline/`
