# W003 — Adapter Capability And Conformance Harness

## Objective
Provide shared machinery to validate adapter manifests and capability claims.

## Scope
1. Manifest shape validation.
2. Capability-ladder validation.
3. Registry-version validation.
4. Conformance-result emission for CI and `DNA ReCalc`.

## Dependencies
1. `W001`
2. `W002`
3. Lane adapter expectations from integrated repos.

## Exit gate
1. Manifest contract is local-spec complete.
2. `C0` through `C5` handling is explicit.
3. Runtime does not reinterpret lane semantics.

## Expected capability impact
1. provides shared validation machinery for adapter capability claims from `C0` through `C5`
2. `W003` does not pre-claim any lane adapter capability level; it validates claims made elsewhere

## Expected pack impact
1. candidate evidence path for `PACK.replay.appliance`
2. downstream packs and hosts may rely on `W003` outputs only after retained conformance artifacts exist

## Environment Preconditions
1. `W002` validate/ingest surfaces are available or explicitly scoped for reuse
2. canonical registry-family expectations are frozen for the workset scope
3. the Rust-first implementation stack declared for `W002` remains valid for `W003`, or an explicit split is recorded before activation
4. manifest fixtures and expected denial cases are available from lane repos or local retained fixtures

## Evidence Layout
1. canonical emitted artifact root: `docs/test-corpus/` for retained manifest fixtures and `docs/test-runs/` for conformance outputs
2. checked-in versus ephemeral policy: baseline fixtures and machine-readable validation baselines are checked in; exploratory runs may be ephemeral until promoted
3. stable naming policy for baseline runs: `w003-conformance-<adapter-id>-baseline`

## Replay-Corpus Readiness
1. replay classes requiring corpus scenarios before activation: `manifest_shape_valid`, `manifest_shape_invalid`, `capability_claim_matrix`, `registry_version_match`
2. scenario ids satisfying them:
   - `manifest_shape_valid` -> `cap_manifest_minimal_valid_001`
   - `manifest_shape_invalid` -> `cap_manifest_schema_invalid_001`
   - `capability_claim_matrix` -> `cap_claim_matrix_floor_001`
   - `registry_version_match` -> `cap_registry_version_mismatch_001`
3. reserve or later lanes: lane-specific replay/diff scenarios and witness distillation

## Pack-Evidence Traceability
1. pack name: `PACK.replay.appliance`
2. replay classes: `manifest_shape_valid`, `manifest_shape_invalid`, `capability_claim_matrix`, `registry_version_match`
3. scenario ids or artifact paths:
   - `cap_manifest_minimal_valid_001` -> `docs/test-corpus/bundles/cap_manifest_minimal_valid_001/`
   - `cap_manifest_schema_invalid_001` -> `docs/test-corpus/bundles/cap_manifest_schema_invalid_001/`
   - `cap_claim_matrix_floor_001` -> `docs/test-corpus/bundles/cap_claim_matrix_floor_001/`
   - `cap_registry_version_mismatch_001` -> `docs/test-corpus/bundles/cap_registry_version_mismatch_001/`
   - retained human-readable runs -> `docs/test-runs/w003-conformance-<adapter-id>-baseline/`
