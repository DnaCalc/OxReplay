# W006 — Witness Distillation And Lifecycle Governance Baseline

## Objective
Stand up predicate-driven witness reduction and the first lifecycle or quarantine handling surfaces without moving distillation into hot-path replay.

## Scope
1. Distillation runtime baseline.
2. Reduction-manifest emission.
3. Witness lifecycle state handling and quarantine outcomes.
4. Registry and capability interactions required for witness promotion decisions.
5. Retained example witnesses and lifecycle outputs.

## Dependencies
1. `W002`
2. `W003`
3. `W004`
4. `W005`

## Exit gate
1. Distillation remains offline and predicate-bound.
2. Witness lifecycle outcomes are explicit and machine-usable.
3. Quarantine or unstable-oracle outcomes are represented rather than hidden.
4. No lane-local rewrite authority has been absorbed into shared runtime code.

## Expected capability impact
1. candidate shared machinery for adapter `C4.distill_valid` handling
2. `W006` does not claim `C4` or `C5` for any lane until retained distillation and lifecycle evidence exists

## Expected pack impact
1. candidate evidence path for `PACK.trace.forensic_plane`
2. candidate evidence path for `PACK.reject.calculus` when reject replay evidence and lifecycle state are part of the claim
3. no pack claim exists until retained witness, quarantine, or lifecycle artifacts are linked

## Environment Preconditions
1. `W004` provides retained or reproducible lane scenarios suitable for reduction
2. `W005` or an equivalent local entry path can execute reduction and lifecycle flows deterministically
3. preservation predicates, closure rules, and safe rewrites come from adapters rather than shared runtime invention
4. the initial implementation stack declared in `W002` remains valid for reduction and lifecycle tooling

## Evidence Layout
1. canonical emitted artifact root: `docs/test-corpus/witnesses/` for retained witness bundles, `states/lifecycle/` for retained lifecycle-state examples, and `docs/test-runs/` for reduction or quarantine outputs
2. checked-in versus ephemeral policy: promoted witnesses, lifecycle examples, and baseline reduction outputs are checked in; exploratory reduction runs may be ephemeral until promoted
3. stable naming policy for baseline runs: `w006-<lane-id>-<scenario-id>-baseline`

## Replay-Corpus Readiness
1. replay classes requiring corpus scenarios before activation: `distill_stable`, `distill_unstable`, `quarantine_required`, `lifecycle_transition`
2. scenario ids satisfying them:
   - `distill_stable` -> `wit_distill_stable_001`
   - `distill_unstable` -> `wit_distill_unstable_predicate_001`
   - `quarantine_required` -> `wit_quarantine_capture_insufficient_001`
   - `lifecycle_transition` -> `wit_lifecycle_transition_retained_local_001`
3. reserve or later lanes: widened pack-promotion claims and non-baseline reduction strategies

## Pack-Evidence Traceability
1. pack name: `PACK.trace.forensic_plane`, `PACK.reject.calculus` when applicable
2. replay classes: `distill_stable`, `distill_unstable`, `quarantine_required`, `lifecycle_transition`
3. scenario ids or artifact paths:
   - `wit_distill_stable_001` -> `docs/test-corpus/witnesses/wit_distill_stable_001/`
   - `wit_distill_unstable_predicate_001` -> `docs/test-corpus/witnesses/wit_distill_unstable_predicate_001/`
   - `wit_quarantine_capture_insufficient_001` -> `docs/test-corpus/witnesses/wit_quarantine_capture_insufficient_001/`
   - `wit_lifecycle_transition_retained_local_001` -> `states/lifecycle/wit_lifecycle_transition_retained_local_001/`
   - retained human-readable runs -> `docs/test-runs/w006-<lane-id>-<scenario-id>-baseline/`
