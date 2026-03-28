# OXREPLAY_DNA_ONECALC_CONSUMPTION_MODEL.md

## 1. Position
This document defines how `DNA OneCalc` should consume `OxReplay` as shared replay infrastructure.

It is a local consumer note for a downstream proving host.
It does not redefine Foundation replay doctrine, lane-semantic ownership, or the `DNA ReCalc` host contract.

## 2. Consumer rule
`DNA OneCalc` is a spreadsheet proving host that consumes `OxReplay` for:
1. replay capture,
2. replay validation,
3. diff,
4. explain,
5. witness handling,
6. scenario-library growth,
7. comparison against retained Excel observation artifacts from `OxXlObs`,
8. replay-visible UI state and user controls.

`DNA OneCalc` may:
1. call `OxReplay` library or runtime surfaces directly,
2. emit canonical replay bundles or normalized replay views for executed scenarios,
3. invoke validation, replay, diff, explain, distill, witness-state, or pack-export flows over declared inputs,
4. project those results into its own UI, persistence, and handoff model.

`DNA OneCalc` may not:
1. present itself as `DNA ReCalc`,
2. bypass lane adapters or canonical bundle contracts,
3. claim replay capability beyond retained `OxReplay` evidence,
4. move replay doctrine, registry governance, or witness-lifecycle policy out of Foundation and into the host.

## 3. Relationship to `DNA ReCalc`
`DNA ReCalc` remains the generic replay host surface over `OxReplay`.

The split is:
1. `DNA ReCalc` is the shared replay host and CLI reference surface,
2. `DNA OneCalc` is a separate spreadsheet proving host that embeds or invokes shared replay mechanics,
3. the same `OxReplay` runtime may sit under both hosts without collapsing them into one host identity.

Working rule:
1. if `DNA OneCalc` needs app-facing replay UX, it should build that UX over `OxReplay`,
2. if a generic replay operator surface is needed, `DNA ReCalc` remains the canonical host reference,
3. local `DNA OneCalc` UI affordances must not be described as the `DNA ReCalc` contract.

## 4. Current conservative upstream floor
`DNA OneCalc` should assume only the following current local `OxReplay` floor.

| Source surface | Conservative assumption for `DNA OneCalc` today | Evidence anchors |
|---|---|---|
| `OxFml` | accepted local adapter floor through `C3.explain_valid`; treat `C4` and beyond as later evidence lanes | `docs/test-runs/w003-conformance-oxfml-replay-adapter-v1-baseline/report.json`, `docs/upstream/NOTES_FOR_OXFML.md`, `docs/IN_PROGRESS_FEATURE_WORKLIST.md` |
| `OxFunc` | no accepted local replay-intake floor yet; consume current function semantics through `OxFml` and lane-native contracts rather than assuming direct `OxReplay` capability | `docs/IN_PROGRESS_FEATURE_WORKLIST.md`, `docs/upstream/NOTES_FOR_OXFUNC.md` |
| `OxXlObs` | accepted first-pass observation-source seam: source observation bundle plus canonical `replay.bundle.v1` manifest and first normalized replay view; treat it as a `lossy` observation intake, not as a broad equivalence or formal adapter-capability claim | `docs/spec/OXREPLAY_OXXLOBS_OBSERVATION_SEAM.md`, `docs/test-runs/oxxlobs-seam-xlobs_capture_values_formulae_001-baseline/`, `../OxXlObs/docs/test-runs/W007_FIRST_CROSS_REPO_REPLAY_AND_DIFF_CONSUMPTION.md` |
| `OxVba` | later and narrower lane; no accepted local replay capability floor yet | `docs/IN_PROGRESS_FEATURE_WORKLIST.md`, `docs/upstream/NOTES_FOR_OXVBA.md` |

Important non-dependency note:
1. `OxCalc` remains a seam-reference repo for `DNA OneCalc`,
2. it is not part of the initial `DNA OneCalc` runtime dependency set,
3. the active local `OxCalc` blocker on `C4.distill_valid` does not change that runtime split.

## 5. Artifact use rule
`DNA OneCalc` may keep its own product artifacts such as scenario documents, scenario runs, comparisons, and handoff packets.

When those artifacts rely on `OxReplay`, they should preserve:
1. replay bundle id,
2. source lane id,
3. adapter id and version,
4. source schema lineage,
5. capture mode,
6. projection status,
7. registry refs when present,
8. witness lifecycle state when present,
9. retained artifact refs for replay, diff, explain, or distill outputs.

If an upstream replay surface is explicitly lossy or registry-unpinned, `DNA OneCalc` should surface that explicitly in retained artifacts and UI state rather than hiding it.

## 6. UI and control rule
`DNA OneCalc` may expose:
1. replay capture state,
2. replay validation status,
3. diff and explain results,
4. witness lifecycle and distill controls,
5. scenario-library controls over retained replay evidence.

The UI should make these replay facts visible when they affect interpretation:
1. capability floor actually relied upon,
2. source lane or observation source,
3. capture mode,
4. projection status,
5. capture-loss or downgraded-instrumentation markers,
6. Windows-only availability for live Excel comparison.

Platform rule:
1. retained `OxXlObs` artifacts may be replayed or diffed anywhere `DNA OneCalc` runs,
2. live Excel-backed comparison remains Windows-only because the current live observation path remains Windows-only in `OxXlObs`.

## 7. Scenario-library growth rule
The intended growth path is:
1. author scenario,
2. run scenario,
3. emit retained replay evidence,
4. compare against retained Excel or other replay evidence,
5. explain mismatch,
6. retain witness,
7. emit upstream handoff.

`DNA OneCalc` should treat the replay corpus as provenance-bearing retained evidence, not as disposable UI cache state.

Current caution:
1. the first `OxXlObs` normalized replay view is useful for replay-path activation and coarse comparison wiring,
2. it is not yet the right basis for broad semantic equivalence, formatting-complete parity, or registry-heavy witness claims.

## 8. Current local limits
The following limits remain explicit:
1. this repo still has no app-facing `DNA OneCalc` host contract analogous to `DNA_RECALC_HOST.md`,
2. `OxFunc` has no accepted local direct replay-intake floor yet,
3. `OxVba` has no accepted local replay-intake floor yet,
4. the `OxXlObs` seam still lacks a formal adapter capability manifest and richer registry-pinned diff structure,
5. the current `OxXlObs` replay-facing normalized view remains explicitly `lossy`,
6. broad lane `C4` or `C5` claims remain later evidence lanes unless retained conformance says otherwise.

## 9. Resulting rule
`DNA OneCalc` should use `OxReplay` as shared replay infrastructure, not as a substitute product host, lane semantics owner, or replacement for `DNA ReCalc`.
