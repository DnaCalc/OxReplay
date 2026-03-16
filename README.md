# OxReplay

OxReplay is the shared Replay appliance implementation repo and library family for DNA Calc.

It exists to implement the cross-lane replay substrate defined by Foundation doctrine without taking ownership of lane-native semantics away from `OxFunc`, `OxFml`, `OxCalc`, or `OxVba`.

## Core responsibilities
1. Canonical replay bundle parsing, validation, serialization, and indexing.
2. Shared replay runtime surfaces for replay, diff, explain, and witness distillation.
3. Adapter SDK, adapter loading, and adapter capability validation.
4. Registry and witness-lifecycle tooling for replay-governed artifacts.
5. Pack-facing replay export and validation support.
6. `DNA ReCalc` CLI and later UI host surfaces.

## Not this repo
1. Not a new semantics lane.
2. Not the owner of lane-local event semantics or reject taxonomies.
3. Not the owner of lane-local rewrite permissions or capture instrumentation meaning.
4. Not a spreadsheet product host.

## Startup docs
1. `AGENTS.md`
2. `CHARTER.md`
3. `OPERATIONS.md`
4. `docs/spec/README.md`

## Bootstrap workspace layout
1. `docs/spec/`
   - repo-owned mutable specs for shared replay runtime and host boundaries.
2. `docs/worksets/`
   - execution packets for staged delivery.
3. `docs/handoffs/`
   - structured cross-repo or Foundation-facing handoff packets.
4. `docs/upstream/`
   - outbound observation ledgers for sibling repos and hosts.
5. `docs/test-corpus/`
   - retained canonical bundles, witnesses, and related replay fixtures.
6. `docs/test-runs/`
   - retained human-readable validation runs.
7. `src/`, `tests/`, `tools/`, `scripts/`, `states/`, `formal/`
   - code, harness, state, and formalization roots to be populated as implementation advances.

## Bootstrap status
The repo starts doc-first. No implementation language or build graph is locked yet.
The runtime strata and ownership boundaries are fixed first so code can be added without semantic drift.

## Foundation alignment
Precedence and constitutional constraints are inherited from:
1. `../Foundation/CHARTER.md`
2. `../Foundation/ARCHITECTURE_AND_REQUIREMENTS.md`
3. `../Foundation/OPERATIONS.md`
4. `../Foundation/REPLAY_APPLIANCE.md`

## Dependency constitution
1. May depend on shared abstractions and replay schemas.
2. May host adapter SDK and conformance tooling.
3. Must not depend on lane-semantic internals outside declared adapter or test boundaries.
4. Must not become a second source of semantic truth.
