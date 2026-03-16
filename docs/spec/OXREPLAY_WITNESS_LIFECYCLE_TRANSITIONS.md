# OXREPLAY_WITNESS_LIFECYCLE_TRANSITIONS.md

## 1. Position
This document defines the initial witness lifecycle transition model used by `W006`.

It is a local operational companion to Foundation lifecycle doctrine.

## 2. Lifecycle states
The initial governed states are:
1. `explanatory-only`
2. `retained-local`
3. `retained-shared`
4. `promoted-pack`
5. `superseded`
6. `quarantined`
7. `gc-eligible`

## 3. Transition floor
Allowed baseline transitions are:
1. `explanatory-only -> quarantined`
2. `explanatory-only -> retained-local`
3. `retained-local -> retained-shared`
4. `retained-local -> superseded`
5. `retained-local -> quarantined`
6. `retained-shared -> promoted-pack`
7. `retained-shared -> superseded`
8. `retained-shared -> quarantined`
9. `quarantined -> retained-local` when the quarantine reason is cleared by fresh evidence
10. `superseded -> gc-eligible`

## 4. Forbidden transitions
1. `explanatory-only -> promoted-pack`
2. `quarantined -> promoted-pack`
3. any transition that drops supersession lineage
4. any transition that removes quarantine reason history silently

## 5. Quarantine reasons
The initial structured reasons are:
1. `unstable_oracle_or_predicate`
2. `insufficient_capture`
3. `missing_source_artifacts`
4. `adapter_failure`
5. `schema_incompatibility`

## 6. Required retained fields
Lifecycle records should retain:
1. witness id
2. source lane
3. source scenario id
4. current lifecycle state
5. quarantine reason when present
6. supersession lineage when present
7. pack-eligibility state

## 7. Artifact roots
Baseline retained roots are:
1. `docs/test-corpus/witnesses/`
2. `states/lifecycle/`
3. `docs/test-runs/`

## 8. Working rule
Lifecycle state governs retention, promotion, quarantine, and GC policy.
It never rewrites the semantic meaning of the source witness artifact.
