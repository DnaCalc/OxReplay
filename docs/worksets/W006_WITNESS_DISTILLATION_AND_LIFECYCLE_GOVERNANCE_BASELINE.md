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
