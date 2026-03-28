# OXREPLAY_SCOPE_AND_BOUNDARY.md

## 1. Position
This document defines what `OxReplay` is allowed to own and what it must leave to Foundation and the lane repos.

## 2. Repo purpose
`OxReplay` is the shared implementation substrate for the Replay appliance.

It exists to provide reusable mechanics for:
1. bundle validation and indexing,
2. normalized replay runtime surfaces,
3. diff and explain execution,
4. witness distillation,
5. registry and lifecycle tooling,
6. adapter capability validation,
7. `DNA ReCalc`,
8. downstream host consumption through declared shared-runtime surfaces.

## 3. In scope
1. Shared replay abstractions and runtime types.
2. Bundle parsing, serialization, validation, and indexing.
3. Replay execution over canonical bundles.
4. Shared mismatch, explain, and reduction runtime surfaces.
5. Registry snapshot handling and witness lifecycle mechanics.
6. Adapter SDK, loader, and conformance harnesses.
7. Pack-facing export mechanics for replay-governed packs.
8. Shared runtime surfaces that a non-`DNA ReCalc` host may embed or invoke without taking semantic ownership.

## 4. Out of scope
1. Semantic ownership of evaluator, function, coordinator, or VBA behavior.
2. Lane-local event-family truth.
3. Lane-local reject taxonomy authority.
4. Lane-local safe-rewrite policy for distillation.
5. Spreadsheet product-host behavior outside replay-host concerns.

## 5. Ownership split
1. Foundation owns doctrine and governance.
2. Lane repos own semantic meaning.
3. `OxReplay` owns shared runtime mechanics.
4. `DNA ReCalc` is the generic host surface over those mechanics.
5. Downstream product hosts may consume those mechanics, but they remain separate hosts with their own UI, persistence, and orchestration policy.

## 6. Non-`DNA ReCalc` host consumer rule
A downstream spreadsheet proving host such as `DNA OneCalc` may:
1. call `OxReplay` libraries or runtime services directly,
2. emit canonical replay artifacts over its own scenarios or retained comparisons,
3. surface replay, diff, explain, witness, and scenario-library controls in its own UI.

It may not:
1. redefine itself as `DNA ReCalc`,
2. bypass adapters or canonical bundle contracts,
3. move replay doctrine or witness-governance authority out of Foundation,
4. use shared-runtime convenience to absorb lane-semantic meaning.

## 7. Module boundary model
The intended initial module split is:
1. `Abstractions`
2. `Bundle`
3. `Core`
4. `Diff`
5. `Explain`
6. `Distill`
7. `Governance`
8. `Conformance`
9. `DNA ReCalc`

This split is a starting model, not a frozen package map.

## 8. Dependency constitution
Allowed:
1. dependence on shared schemas and local abstractions,
2. adapter loading through declared contracts,
3. lane-provided fixtures or plugins for conformance.

Forbidden without explicit override:
1. importing lane-semantic internals into shared runtime interpretation,
2. turning adapter helpers into semantic-core dependencies,
3. letting `DNA ReCalc` become the semantic authority instead of the replay host,
4. letting a downstream product host bypass shared replay contracts by linking directly to lane-semantic internals.

## 9. First bootstrap goals
1. Stand up bundle and schema validation.
2. Validate adapter capability manifests.
3. Load and exercise initial `OxCalc` and `OxFml` adapters.
4. Prove the first shared diff and explain flows.
5. Delay broader extraction until the shared surface is empirically stable.
