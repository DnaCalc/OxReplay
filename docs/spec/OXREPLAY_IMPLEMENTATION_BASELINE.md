# OXREPLAY_IMPLEMENTATION_BASELINE.md

## 1. Position
This document defines the active repo-local implementation baseline for `OxReplay`.

It is subordinate to Foundation doctrine and to this repo's charter and operations rules.
It does not authorize capability claims on its own.

## 2. Implementation direction
1. `OxReplay` is Rust-first.
2. The active executable workspace is a repo-root Cargo workspace.
3. The active code root is `src/`.
4. The implementation pattern should follow the closest family precedent from `OxCalc`: one Rust workspace with crate boundaries shaped by declared runtime strata.

## 3. Workspace layout
The initial workspace members are:
1. `src/oxreplay-abstractions`
2. `src/oxreplay-bundle`
3. `src/oxreplay-core`
4. `src/oxreplay-diff`
5. `src/oxreplay-explain`
6. `src/oxreplay-distill`
7. `src/oxreplay-governance`
8. `src/oxreplay-conformance`
9. `src/oxreplay-dnarecalc-cli`

## 4. Crate responsibility map
1. `oxreplay-abstractions`
   - shared ids, capability enums, registry refs, and cross-strata contract types
2. `oxreplay-bundle`
   - bundle manifest types, ingest validation, sidecar resolution hooks, and indexing contracts
3. `oxreplay-core`
   - normalized replay events, views, and orchestration context
4. `oxreplay-diff`
   - typed mismatch and comparison surfaces
5. `oxreplay-explain`
   - explanation queries and causal records
6. `oxreplay-distill`
   - preservation predicates, reduction outcomes, and reduction manifests
7. `oxreplay-governance`
   - registry and witness lifecycle types plus compatibility checks
8. `oxreplay-conformance`
   - adapter capability manifest validation and machine-readable conformance outputs
9. `oxreplay-dnarecalc-cli`
   - `DNA ReCalc` CLI shell over the shared runtime

## 5. Validation floor
The minimum local executable validation floor is:
1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. `cargo test --workspace`
4. `pwsh ./scripts/meta-check.ps1`

## 6. Local rules
1. `unsafe` is forbidden by default.
2. New crate boundaries should reflect runtime ownership rather than convenience imports.
3. Shared runtime crates must not depend upward on `DNA ReCalc`.
4. Adapter- or lane-specific meaning must remain outside shared runtime crates except through declared adapter contracts.
5. Historical non-Rust implementation trees are not part of the baseline for this repo.

## 7. Evolution rule
The crate graph may change as implementation widens, but:
1. crate changes must preserve the declared ownership split,
2. any split that encourages semantic leakage across the adapter boundary is incorrect,
3. later widening should extend this baseline rather than silently replacing it.
