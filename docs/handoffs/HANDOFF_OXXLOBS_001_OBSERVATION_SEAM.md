# HANDOFF_OXXLOBS_001_OBSERVATION_SEAM

## Header
1. handoff id: `HANDOFF_OXXLOBS_001`
2. date: `2026-03-20`
3. from repo: `OxReplay`
4. to repo: `OxXlObs`
5. related workset or feature id: `OxXlObs initial integration seam`

## Purpose
State the initial `OxReplay` to `OxXlObs` contract for Excel observation artifacts entering the Replay appliance.

## Proposed change
`OxXlObs` should target replay-ready observation artifact emission from the start.

The preferred order is:
1. direct canonical replay-bundle emission,
2. otherwise a declared observation bundle plus narrow projection contract,
3. avoid free-form or opaque automation logs as the primary retained output.

`OxXlObs` should preserve:
1. Excel build, version, and channel when available,
2. workbook or input fingerprint,
3. trigger recipe,
4. calculation mode and material host settings,
5. observed surface inventory,
6. capture-loss, downgraded instrumentation, unavailable-surface, and nondeterminism markers.

## Current evidence
1. seam packet: `docs/spec/OXREPLAY_OXXLOBS_OBSERVATION_SEAM.md`
2. outbound note: `docs/upstream/NOTES_FOR_OXXLOBS.md`
3. current `DNA ReCalc` and retained baseline surfaces under `docs/test-corpus/` and `docs/test-runs/`

## Impact
1. capability impact: enables future `OxXlObs` `C0` and later capability validation without widening `OxReplay` semantics ownership
2. pack impact: establishes the provenance floor required before any Excel-backed retained evidence can support replay-governed packs
3. migration or fallback impact: if direct canonical bundle emission is delayed, a narrow observation-contract phase is acceptable as an intermediate step
4. affected repos or hosts: `OxXlObs`, `OxReplay`, `DNA ReCalc`, later `OxCalc` differential runs

## Requested response
1. acknowledge whether `OxXlObs` will target direct canonical bundle emission first or a narrower observation-contract phase first
2. confirm the first emitted provenance fields and observation surfaces
3. return one minimal retained fixture family and one first comparison-ready scenario proposal

## Response processed by OxReplay
1. `OxXlObs` responded through `../OxXlObs/docs/upstream/NOTES_FOR_OXREPLAY.md`
2. the accepted first-pass model is dual-artifact:
   - a rich source observation bundle remains the source contract,
   - an `OxReplay`-canonical `replay.bundle.v1` manifest is emitted for shared-runtime intake
3. `OxReplay` locally verified the emitted canonical manifest and normalized replay view against the current runtime
4. the remaining open seam item is the first retained Excel-vs-DNA comparison-ready scenario and the point at which a formal adapter manifest becomes worthwhile
