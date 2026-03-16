# W002 — Bundle And Schema Runtime

## Objective
Stand up the first canonical bundle parsing, validation, and indexing surface in `OxReplay`.

## Scope
1. Bundle manifest ingestion.
2. Schema-version validation.
3. Sidecar resolution and indexing.
4. Machine-readable validation output.

## Dependencies
1. `W001`
2. Foundation Replay bundle doctrine.

## Exit gate
1. Bundle validation surface is spec-locked.
2. Indexing expectations are explicit.
3. Capability impact on `PACK.replay.appliance` is stated.

## Initial implementation stack declaration
1. status: declared
2. language/runtime: Rust 2024 edition
3. build graph: repo-root Cargo workspace
4. package roots:
   - `src/oxreplay-abstractions`
   - `src/oxreplay-bundle`
   - `src/oxreplay-core`
   - `src/oxreplay-diff`
   - `src/oxreplay-explain`
   - `src/oxreplay-distill`
   - `src/oxreplay-governance`
   - `src/oxreplay-conformance`
   - `src/oxreplay-dnarecalc-cli`
5. test runner:
   - `cargo test --workspace`
6. local validation floor:
   - `cargo fmt --all --check`
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
   - `cargo test --workspace`
   - `pwsh ./scripts/meta-check.ps1`

## Expected capability impact
1. establishes shared validate/ingest machinery later used by adapter `C0.ingest_valid` conformance
2. `W002` does not, by itself, claim any adapter capability level

## Expected pack impact
1. candidate validate/ingest surface for `PACK.replay.appliance`
2. no pack claim exists until retained artifacts and validator outputs are linked

## Environment Preconditions
1. the Rust-first implementation stack declared in this packet remains the active stack for the repo
2. canonical bundle-schema references are pinned for the validator scope
3. sidecar-resolution policy is explicit before code begins
4. validation remains runnable through file- or CLI-oriented flows
5. baseline commands are executable from repo root through Cargo and `scripts/meta-check.ps1`

## Evidence Layout
1. canonical emitted artifact root: `docs/test-corpus/bundles/` for retained fixtures and `docs/test-runs/` for validation outputs
2. checked-in versus ephemeral policy: retained baseline fixtures are checked in; ad hoc validation runs may be ephemeral unless promoted
3. stable naming policy for baseline runs: `w002-bundle-<scenario-id>-baseline`

## Replay-Corpus Readiness
1. replay classes requiring corpus scenarios before activation: `bundle_manifest_valid`, `bundle_manifest_invalid`, `sidecar_resolution`
2. scenario ids satisfying them:
   - `bundle_manifest_valid` -> `rb_manifest_minimal_valid_001`
   - `bundle_manifest_invalid` -> `rb_manifest_schema_invalid_001`
   - `sidecar_resolution` -> `rb_sidecar_resolution_roundtrip_001`
3. reserve or later lanes: witness reduction, lifecycle transitions, and pack-export flows

## Pack-Evidence Traceability
1. pack name: `PACK.replay.appliance`
2. replay classes: `bundle_manifest_valid`, `bundle_manifest_invalid`, `sidecar_resolution`, `bundle_indexing`
3. scenario ids or artifact paths:
   - `rb_manifest_minimal_valid_001` -> `docs/test-corpus/bundles/rb_manifest_minimal_valid_001/`
   - `rb_manifest_schema_invalid_001` -> `docs/test-corpus/bundles/rb_manifest_schema_invalid_001/`
   - `rb_sidecar_resolution_roundtrip_001` -> `docs/test-corpus/bundles/rb_sidecar_resolution_roundtrip_001/`
   - `rb_bundle_index_projection_001` -> `docs/test-corpus/bundles/rb_bundle_index_projection_001/`
   - retained human-readable runs -> `docs/test-runs/w002-bundle-<scenario-id>-baseline/`
