# OXREPLAY_ADAPTER_AND_CONFORMANCE_MODEL.md

## 1. Position
This document defines how `OxReplay` should host adapters and capability validation without taking lane ownership away from the source repos.

## 2. Adapter contract rule
An adapter is the lane-declared bridge between lane-native artifacts and the normalized replay model.

`OxReplay` owns:
1. adapter interfaces,
2. adapter loading and execution runtime,
3. manifest parsing and capability validation,
4. conformance harnesses,
5. shared bundle emission and validation utilities.

Lane repos own:
1. source artifact meaning,
2. source schema truth,
3. closure and safe-rewrite authority for distillation,
4. semantic interpretation of lane-local payloads.

## 3. Supported adapter patterns
The runtime should support:
1. direct canonical-bundle emitters from lane repos,
2. narrow adapter packages referencing `OxReplay` abstractions,
3. plugin-style adapters loaded by `DNA ReCalc`,
4. conformance-only test adapters and fixtures.

## 4. Capability ladder
Shared capability levels are:
1. `C0.ingest_valid`
2. `C1.replay_valid`
3. `C2.diff_valid`
4. `C3.explain_valid`
5. `C4.distill_valid`
6. `C5.pack_valid`

`OxReplay` must treat capability claims as validated evidence surfaces, not mere metadata strings.

Downstream packs, hosts, and promotion packets must state the required capability level explicitly rather than assuming Replay support generically.

## 5. Bootstrap rollout baseline
Foundation rollout expectations for initial lane growth are:
1. `OxCalc` is the first lane expected to drive toward `C5.pack_valid` and the first proving ground for shared diff and witness-distillation flows.
2. `OxFml` should first prove ingest, replay, diff, and explain; distillation follows only after seam evidence stabilizes.
3. `OxFunc` joins later through narrower initial replay surfaces and later capability growth.
4. `OxVba` joins later through narrower initial conformance and host-policy replay surfaces.

## 6. Required adapter manifest content
1. adapter id and version,
2. lane id,
3. supported source schemas,
4. supported bundle schemas,
5. supported capture modes,
6. claimed capability levels,
7. known limits,
8. conformance artifact refs,
9. registry version refs.

## 7. Conformance runtime duties
`OxReplay` should provide shared machinery to:
1. validate manifest shape,
2. validate capability claims against shared rules,
3. surface registry-version mismatches,
4. report missing lifecycle support for distillation or pack claims,
5. emit machine-readable validation results for CI and `DNA ReCalc`.

## 8. Distillation boundary
`OxReplay` may execute reduction search, but:
1. preservation predicates remain explicit inputs,
2. closure rules come from the adapter,
3. safe rewrite transforms must be lane-declared,
4. unstable predicates or insufficient capture must produce explicit outcomes,
5. witness lifecycle and quarantine handling remain governed outputs.

## 9. Resulting rule
`OxReplay` hosts the adapter runtime and conformance machinery, but the lane repos remain the semantic owners of what their adapters mean.
