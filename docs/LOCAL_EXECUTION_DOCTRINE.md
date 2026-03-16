# LOCAL_EXECUTION_DOCTRINE.md — OxReplay

## 1. Purpose
Set expectations for local execution and validation while `OxReplay` is still in bootstrap and early implementation mode.

## 2. Rust-First Baseline
1. `OxReplay` is Rust-first with the active Cargo workspace rooted at `src/`.
2. Crate boundaries should follow the declared replay strata rather than convenience groupings that encourage semantic leakage.
3. The local validation floor for Rust changes is:
   - `cargo fmt --all --check`
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
   - `cargo test --workspace`
   - `pwsh ./scripts/meta-check.ps1`
4. `unsafe` remains forbidden in the Rust workspace unless a separate explicit doctrine decision authorizes a narrowly scoped exception.

## 3. Local Execution Lessons

### Lesson 1: Execution Packets Must Declare Their Artifact Root Up Front
If a workset expects emitted evidence, it must name the canonical artifact root before code begins.

Rule:
1. every implementation packet names one canonical artifact root,
2. the packet states whether the artifacts are checked in or ephemeral,
3. retained baseline naming is fixed before the first run is promoted.

### Lesson 2: Tracked Artifacts Must Use Repo-Relative Paths Only
Tracked evidence must remain portable across local checkouts and CI.

Rule:
1. checked-in artifacts use repo-relative paths only,
2. absolute paths are allowed only in transient local diagnostics that are not tracked.

### Lesson 3: Replay Classes Need Scenario IDs Before Harness Work Starts
Harness work should not discover its corpus after the code already exists.

Rule:
1. if a replay class is part of a workset gate, at least one scenario id is assigned before activation,
2. missing replay classes must remain explicit in the workset packet rather than being silently deferred.

### Lesson 4: Harness Output Shape Is Part Of The Contract
Validator, conformance, and host outputs are contract surfaces, not incidental developer logs.

Rule:
1. tests should assert emitted artifact shape as well as pass/fail behavior,
2. output normalization is part of the runtime contract.

### Lesson 5: One Stable Baseline Run Beats Many Ad Hoc Smokes
Closure evidence should center on one explicit retained baseline per execution wave.

Rule:
1. each evidence-bearing workset should designate one stable retained baseline run,
2. ad hoc smoke runs remain useful but do not satisfy gate evidence unless promoted.

### Lesson 6: Tool Availability Must Be Declared As An Execution Precondition
Formal or auxiliary tools are not implicitly available just because the repo mentions them.

Rule:
1. execution packets declare required tools, optional tools, and fallback evidence rules,
2. missing tools must be reported explicitly when fallback evidence is used.

### Lesson 7: Pack Names Alone Are Not Enough
Pack references without replay classes, scenario ids, and artifact paths are planning-only.

Rule:
1. planning packets should bind each named pack to replay classes, scenario ids, and artifact roots,
2. pack references without those bindings do not constitute readiness evidence.

## 4. Early local validation targets
1. bundle ingest and schema validation,
2. adapter manifest and capability validation,
3. replay/diff/explain query validation,
4. witness lifecycle and quarantine-state validation,
5. adapter loading and `DNA ReCalc` CLI smoke tests.

## 5. Anti-drift rule
If a local implementation step requires lane-semantic interpretation not already declared by an adapter contract, stop and route it back to the lane repo or Foundation doctrine before proceeding.
