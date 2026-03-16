# OXREPLAY_RUNTIME_STRATA_AND_PACKAGE_MAP.md

## 1. Position
This document translates the repo mission into an initial runtime-strata and workspace model without freezing the implementation language or final package graph prematurely.

## 2. Intended strata
The initial shared-runtime split is:
1. `Abstractions`
   - adapter-facing ids, manifest shapes, registry refs, lifecycle refs, and stable cross-strata contracts.
2. `Bundle`
   - canonical bundle parsing, validation, serialization, sidecar loading, and indexing.
3. `Core`
   - normalized replay runtime types, orchestration context, and source-preserving identity carriers.
4. `Diff`
   - typed mismatch objects and diff planning over normalized replay state.
5. `Explain`
   - causal-query and explanation surfaces over replay and diff outputs.
6. `Distill`
   - predicate-bound witness reduction search, reduction manifests, and witness-bundle emission.
7. `Governance`
   - registry handling, witness lifecycle handling, and compatibility checks.
8. `Conformance`
   - adapter capability validation, harness loading, and machine-readable validation outputs.
9. `DNA ReCalc`
   - host shell over the shared runtime, CLI first.

## 3. Physical workspace model
The repo layout should map work approximately as follows:
1. `src/`
   - active Rust Cargo workspace for shared runtime and host code, stratified by the logical model above.
2. `tests/`
   - unit, scenario, conformance, and retained regression tests aligned to the same strata.
3. `tools/`
   - developer utilities, validation helpers, and fixture or report tooling that should not ship as runtime code.
4. `scripts/`
   - local wrappers, convenience entry points, and repeatable repo bootstrap or validation commands.
5. `states/`
   - checked-in registry, lifecycle, or other retained state artifacts when they are part of the evidence model.
6. `formal/`
   - formal models or notes for reduction, compatibility, or lifecycle invariants when applicable.

### 3.1 Rust workspace baseline
The active implementation direction is Rust-first.

The initial Cargo workspace should live at repo root and include `src/` members named:
1. `src/oxreplay-abstractions`
2. `src/oxreplay-bundle`
3. `src/oxreplay-core`
4. `src/oxreplay-diff`
5. `src/oxreplay-explain`
6. `src/oxreplay-distill`
7. `src/oxreplay-governance`
8. `src/oxreplay-conformance`
9. `src/oxreplay-dnarecalc-cli`

Rust workspace rules:
1. keep crate boundaries aligned to the strata and ownership split,
2. forbid `unsafe` by default,
3. treat `cargo fmt`, `cargo clippy`, and `cargo test` as the minimum local validation floor,
4. do not introduce a parallel non-Rust implementation tree for the same runtime scope.

## 4. Boundary rules
1. Host layers may depend downward on shared runtime layers.
2. Shared runtime layers must not depend upward on host shells.
3. Conformance may depend on shared abstractions and harness utilities, but must not become a new semantics source.
4. Lane-specific fixtures may be consumed in tests or adapters, not hard-coded into shared runtime interpretation.
5. If a package split encourages semantic leakage across the adapter boundary, the split is incorrect even if it compiles cleanly.

## 5. Bootstrap implementation rule
The logical strata are stable before the final package map is stable.

This means:
1. directory names may change,
2. crate or assembly boundaries may change,
3. internal helper placement may change,
4. but the ownership split must remain intact.

## 6. Artifact roots
Initial retained artifact roots should prefer:
1. `docs/test-corpus/` for canonical retained bundles and witnesses,
2. `docs/test-runs/` for retained run outputs and summaries,
3. `states/registry/` for registry snapshots or pinned compatibility state,
4. `states/lifecycle/` for retained witness lifecycle examples when needed.

## 7. First bootstrap slices
1. lock the strata names and boundary model,
2. stand up empty or minimal code roots without implying capability claims,
3. keep the technology stack open until the first implementation workset declares it explicitly,
4. ensure every later code addition can be located in the workspace without reopening scope debates.
