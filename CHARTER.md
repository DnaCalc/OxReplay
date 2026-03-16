# CHARTER.md — OxReplay Charter

## 1. Mission
OxReplay defines, implements, and proves the shared Replay appliance runtime for DNA Calc.

It owns the reusable replay substrate for bundle validation, replay execution, diff/explain, witness distillation, adapter capability validation, and the `DNA ReCalc` host surface while preserving lane ownership of semantics.

## 2. Precedence
When guidance conflicts, precedence is:
1. `../Foundation/CHARTER.md`
2. `../Foundation/ARCHITECTURE_AND_REQUIREMENTS.md`
3. `../Foundation/OPERATIONS.md`
4. `../Foundation/REPLAY_APPLIANCE.md`
5. this `CHARTER.md`
6. this repo `OPERATIONS.md`

## 3. Scope
In scope:
1. Canonical replay bundle runtime and validation surfaces.
2. Shared runtime types for replay, diff, explain, and witness distillation.
3. Adapter SDK, loader, capability-manifest validation, and conformance harnesses.
4. Registry and witness-lifecycle runtime support.
5. `DNA ReCalc` as the replay host over the shared runtime.

Out of scope:
1. Lane-native semantic ownership for `OxFunc`, `OxFml`, `OxCalc`, or `OxVba`.
2. Lane-local event-family truth and reject taxonomy authority.
3. Lane-local reduction rewrite permissions.
4. Spreadsheet proving-host semantics.
5. UI/product doctrine outside replay-host concerns.

## 4. Ownership boundary rule
1. `OxReplay` may normalize and operate on lane artifacts through declared adapter contracts.
2. `OxReplay` may not reinterpret lane-native artifacts outside those declared adapter contracts.
3. Lane repos remain authoritative for semantic meaning; `OxReplay` owns shared mechanics only.

## 5. `DNA ReCalc` rule
1. `DNA ReCalc` is the host surface over `OxReplay`.
2. It is not part of the spreadsheet host progression ladder.
3. It must not become a second semantic authority.

## 6. Clean-room rule
Allowed sources:
1. public specifications and documentation,
2. published research,
3. reproducible black-box observations,
4. Foundation-promoted replay doctrine and lane-provided clean-room handoff artifacts.

Disallowed sources:
1. proprietary or restricted sources,
2. reverse engineering of internals,
3. decompilation/disassembly of Excel internals.

## 7. Definition of done
A shared-runtime change is done only when:
1. repo-local spec text is updated,
2. relevant Foundation doctrine links still hold,
3. capability and pack impact are stated,
4. affected adapter or host conformance evidence is updated,
5. the change does not widen `OxReplay` into lane-semantic ownership.
