# DEC-001 — Rust-First Workspace Baseline

## Decision
Adopt a Rust-first implementation baseline for `OxReplay`.

The active implementation lives under `src/` as a repo-root Cargo workspace.
Crate boundaries follow the declared replay strata and `DNA ReCalc` host surface.

## Date
2026-03-16

## Scope
1. repo-local implementation language and build graph
2. top-level workspace layout
3. local validation floor for executable work
4. relationship to sibling repo patterns

## Alternatives considered
1. defer the language choice longer
   - rejected because `W002` needed an explicit stack before activation
2. use a `crates/` root like `OxFml` and `OxVba`
   - rejected because `OxReplay` already declared `src/` as the code root and `OxCalc` shows the closest family fit for a Rust workspace under `src/`
3. maintain a parallel non-Rust tree during bootstrap
   - rejected because it would split evidence and reopen layout debates

## Result
1. `Cargo.toml` at repo root defines the workspace
2. workspace members live under `src/oxreplay-*`
3. `scripts/meta-check.ps1` is the local one-command validation floor
4. Rust workspace checks are:
   - `cargo fmt --all --check`
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
   - `cargo test --workspace`
5. `unsafe` remains forbidden by default

## Initial crate split
1. `oxreplay-abstractions`
2. `oxreplay-bundle`
3. `oxreplay-core`
4. `oxreplay-diff`
5. `oxreplay-explain`
6. `oxreplay-distill`
7. `oxreplay-governance`
8. `oxreplay-conformance`
9. `oxreplay-dnarecalc-cli`

## Impact
1. resolves `BLK-REPLAY-001`
2. moves `W002` to the active implementation packet
3. gives later worksets one stable layout and validation floor
4. does not, by itself, claim runtime capability or pack readiness
