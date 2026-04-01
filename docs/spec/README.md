# OxReplay Spec Index

This directory is the OxReplay-owned mutable spec set after bootstrap.

## Canonical local spec ownership
1. `docs/spec/OXREPLAY_SCOPE_AND_BOUNDARY.md`
2. `docs/spec/OXREPLAY_RUNTIME_STRATA_AND_PACKAGE_MAP.md`
3. `docs/spec/OXREPLAY_BUNDLE_WITNESS_AND_REGISTRY_MODEL.md`
4. `docs/spec/OXREPLAY_ADAPTER_AND_CONFORMANCE_MODEL.md`
5. `docs/spec/OXREPLAY_DNA_ONECALC_CONSUMPTION_MODEL.md`
6. `docs/spec/DNA_RECALC_HOST.md`
7. `docs/spec/OXREPLAY_IMPLEMENTATION_BASELINE.md`
8. `docs/spec/OXREPLAY_REPLAY_CLASS_AND_SCENARIO_REGISTER.md`
9. `docs/spec/OXREPLAY_CAPABILITY_AND_PACK_TRACEABILITY.md`
10. `docs/spec/OXREPLAY_INITIAL_ADAPTER_INTAKE_PLAN.md`
11. `docs/spec/DNA_RECALC_CLI_CONTRACT.md`
12. `docs/spec/OXREPLAY_WITNESS_LIFECYCLE_TRANSITIONS.md`
13. `docs/spec/OXREPLAY_OXXLOBS_OBSERVATION_SEAM.md`

## Consumed doctrine
Foundation remains higher-precedence doctrine owner for:
1. Replay architecture and governance,
2. pack and promotion rules,
3. repo and host topology,
4. lifecycle and registry policy.

Primary Foundation references:
1. `../../../Foundation/REPLAY_APPLIANCE.md`
2. `../../../Foundation/CHARTER.md`
3. `../../../Foundation/ARCHITECTURE_AND_REQUIREMENTS.md`
4. `../../../Foundation/OPERATIONS.md`

## Lane reference rule
Lane repos remain the source for lane-native adapter meaning and semantics-specific trace details.

Current integration note:
1. the landed `OxFml_V1` consumer seam is now treated locally as the ordinary OxFml replay intake boundary,
2. local helper- or fixture-family-specific OxFml intake paths are retained only as transitional support and historical evidence readers.

## Mirror policy
This repo may restate implementation-boundary detail, but may not create local doctrine that conflicts with Foundation or reassigns lane ownership.

## Consumer note
Use `docs/spec/OXREPLAY_DNA_ONECALC_CONSUMPTION_MODEL.md` when the downstream consumer is a non-`DNA ReCalc` host such as `DNA OneCalc`.

That document covers:
1. embeddable `OxReplay` surface catalog,
2. current conservative capability floor per lane,
3. mode gate discipline for `Replay`, `Diff`, `Explain`, `Distill`, and `Handoff`,
4. artifact-lineage obligations including lossy, registry-unpinned, and capture-incomplete inputs,
5. `OxXlObs` input labeling and interpretation rules,
6. UI visibility and control guidance,
7. scenario-library growth and promotion caution.

Supporting downstream-consumer detail also appears in:
1. `docs/spec/OXREPLAY_SCOPE_AND_BOUNDARY.md` Section 6,
2. `docs/spec/OXREPLAY_ADAPTER_AND_CONFORMANCE_MODEL.md` Sections 4 and 7,
3. `docs/spec/DNA_RECALC_HOST.md` Section 4,
4. `docs/spec/OXREPLAY_BUNDLE_WITNESS_AND_REGISTRY_MODEL.md` Section 5,
5. `docs/spec/OXREPLAY_OXXLOBS_OBSERVATION_SEAM.md` Section 10.

## Bootstrap reading path
For initial `OxReplay` work, read in this order after the repo root docs:
1. `docs/spec/OXREPLAY_SCOPE_AND_BOUNDARY.md`
2. `docs/spec/OXREPLAY_RUNTIME_STRATA_AND_PACKAGE_MAP.md`
3. `docs/spec/OXREPLAY_BUNDLE_WITNESS_AND_REGISTRY_MODEL.md`
4. `docs/spec/OXREPLAY_ADAPTER_AND_CONFORMANCE_MODEL.md`
5. `docs/spec/OXREPLAY_DNA_ONECALC_CONSUMPTION_MODEL.md`
6. `docs/spec/DNA_RECALC_HOST.md`
7. `docs/spec/OXREPLAY_IMPLEMENTATION_BASELINE.md`
8. `docs/spec/OXREPLAY_REPLAY_CLASS_AND_SCENARIO_REGISTER.md`
9. `docs/spec/OXREPLAY_CAPABILITY_AND_PACK_TRACEABILITY.md`
10. `docs/spec/OXREPLAY_INITIAL_ADAPTER_INTAKE_PLAN.md`
11. `docs/spec/DNA_RECALC_CLI_CONTRACT.md`
12. `docs/spec/OXREPLAY_WITNESS_LIFECYCLE_TRANSITIONS.md`
13. `docs/spec/OXREPLAY_OXXLOBS_OBSERVATION_SEAM.md`
