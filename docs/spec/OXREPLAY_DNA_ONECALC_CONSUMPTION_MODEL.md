# OXREPLAY_DNA_ONECALC_CONSUMPTION_MODEL.md

## 1. Position
This document defines how `DNA OneCalc` should consume `OxReplay` as shared replay infrastructure.

It is the authoritative local consumer note for a downstream proving host that is not `DNA ReCalc`.
It does not redefine Foundation replay doctrine, lane-semantic ownership, or the `DNA ReCalc` host contract.

When this document and the Foundation `DNA_ONECALC_SCOPE_AND_SPEC.md` note disagree, the Foundation note takes precedence for replay governance and host-topology questions.

## 2. Consumer identity rule
`DNA OneCalc` is a spreadsheet proving host that consumes `OxReplay` for:
1. bundle validation,
2. replay execution,
3. diff,
4. explain,
5. witness handling,
6. scenario-library growth,
7. comparison against retained Excel observation artifacts from `OxXlPlay`,
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
3. local `DNA OneCalc` UI affordances must not be described as the `DNA ReCalc` contract,
4. `DNA OneCalc` documentation should never use the name `DNA ReCalc` to describe its own replay features.

## 4. Embeddable `OxReplay` surface catalog
`DNA OneCalc` may embed or invoke the following shared runtime strata directly.

| `OxReplay` stratum | Embeddable | OneCalc use | Constraint |
|---|---|---|---|
| `Abstractions` | yes | adapter ids, manifest types, registry refs, lifecycle refs | read-only consumption; do not extend shared id families locally |
| `Bundle` | yes | bundle parsing, validation, sidecar resolution, indexing | all bundles must go through canonical validation |
| `Core` | yes | normalized replay runtime types, orchestration context | do not locally reinterpret normalized events |
| `Diff` | yes | typed mismatch comparison over normalized replay state | do not locally define mismatch kinds outside the shared registry |
| `Explain` | yes | causal-query and explanation surfaces over replay/diff outputs | explanation truth comes from the adapter, not from host inference |
| `Distill` | yes, when the active lane floor supports it | predicate-bound witness reduction, reduction manifests | preservation predicates and closure rules come from adapters; distillation remains offline and predicate-bound |
| `Governance` | yes | registry handling, witness lifecycle, compatibility checks | lifecycle transitions must follow the governed transition model |
| `Conformance` | yes for validation; not for host UI claims | adapter manifest validation, capability claim checking | conformance results inform the host but do not become product marketing claims |
| `DNA ReCalc` | no, do not embed the `DNA ReCalc` host shell | — | `DNA OneCalc` builds its own host shell; it should not wrap or re-expose `DNA ReCalc` as a subsystem |

Rule:
1. embedding a shared stratum means calling its public interfaces, not copying its internals,
2. the host must treat shared runtime outputs as authoritative and not locally override them,
3. if the host needs behavior that a shared stratum does not yet support, that gap should be routed upstream rather than patched locally.

## 5. Current conservative upstream floor
`DNA OneCalc` should assume only the following current local `OxReplay` floor.

| Source surface | Conservative assumption for `DNA OneCalc` today | Evidence anchors |
|---|---|---|
| `OxFml` | accepted local adapter floor through `C3.explain_valid`; treat `C4` and beyond as later evidence lanes | `docs/test-runs/w003-conformance-oxfml-replay-adapter-v1-baseline/report.json`, `docs/upstream/NOTES_FOR_OXFML.md`, `docs/IN_PROGRESS_FEATURE_WORKLIST.md` |
| `OxFunc` | no accepted local replay-intake floor yet; consume current function semantics through `OxFml` and lane-native contracts rather than assuming direct `OxReplay` capability | `docs/IN_PROGRESS_FEATURE_WORKLIST.md`, `docs/upstream/NOTES_FOR_OXFUNC.md` |
| `OxXlPlay` | accepted observation-source seam: source observation bundle plus canonical `replay.bundle.v1` manifest and widened normalized replay view with declared `comparison_views` and `source_metadata`; still treat it as a `lossy` observation intake, not as a broad equivalence or formal adapter-capability claim | `docs/spec/OXREPLAY_OXXLPLAY_OBSERVATION_SEAM.md`, `docs/test-runs/oxxlplay-seam-xlplay_capture_values_formulae_001-baseline/`, `docs/test-runs/oxxlplay-seam-xlplay_capture_spreadsheetml_formatting_001-baseline/`, `../OxXlPlay/docs/test-runs/W007_FIRST_CROSS_REPO_REPLAY_AND_DIFF_CONSUMPTION.md` |
| `OxVba` | later and narrower lane; no accepted local replay capability floor yet | `docs/IN_PROGRESS_FEATURE_WORKLIST.md`, `docs/upstream/NOTES_FOR_OXVBA.md` |

Important non-dependency note:
1. `OxCalc` remains a seam-reference repo for `DNA OneCalc`,
2. it is not part of the initial `DNA OneCalc` runtime dependency set,
3. the active local `OxCalc` blocker on `C4.distill_valid` does not change that runtime split.

## 6. Mode gate discipline
The Foundation `DNA_ONECALC_SCOPE_AND_SPEC.md` defines named workbench modes with explicit capability gates. This section maps those modes to the `OxReplay` surfaces that must be present for each mode to be honestly enabled.

| OneCalc mode | Required `OxReplay` surface | Minimum capability floor | Platform rule | Current honest state |
|---|---|---|---|---|
| `Replay` | accepted lane adapter intake through `Bundle` and `Core`; validated replay execution | `C1.replay_valid` for the active lane | all hosts that can read retained artifacts | honest for `OxFml` through `C3`; `OxFunc` and `OxVba` not yet accepted |
| `Diff` | typed diff surface through `Diff` stratum; comparable replay artifacts from at least two sources; prefer per-family comparison where `comparison_views` are published | `C2.diff_valid` for the active lane | same as replay-capable hosts | honest for `OxFml`; the current integrated XML lane now consumes real OxFml and OxXlPlay `comparison_views` and emits typed family divergence over a still-`lossy` OxXlPlay observation input |
| `Explain` | causal-query surface through `Explain` stratum; adapter-backed explanation records; per-family divergence and coverage-gap records when `comparison_views` are published | `C3.explain_valid` for the active lane | same as replay-capable hosts | honest for `OxFml` through `C3`; the current integrated XML lane now emits per-family divergence explain records over retained OxFml and OxXlPlay artifacts |
| `Distill` | predicate-bound reduction through `Distill` stratum; adapter-declared closure rules and preservation predicates | `C4.distill_valid` for the active lane | only where the active lane adapter supports it | not yet honest for any lane from OneCalc perspective; hide or mark experimental |
| `Handoff` | lineage-complete replay artifacts with provenance, seam pins, and capability floor | no additional OxReplay floor beyond what the source mode required | all hosts | depends on the source mode floor; handoff must carry exact provisional pins |

Gate enforcement rule:
1. if the required `OxReplay` surface is not present or the capability floor is not met, the mode must be hidden, disabled with a visible reason, or shown as provisional with the exact gap visible,
2. `DNA OneCalc` must not present a mode as available when the underlying `OxReplay` capability evidence does not justify it,
3. the UI should surface the relied-upon capability level explicitly when the mode is active.

## 7. Artifact-lineage obligations
When `DNA OneCalc` artifacts rely on `OxReplay`, they must preserve:
1. replay bundle id,
2. source lane id,
3. adapter id and version,
4. source schema lineage,
5. capture mode,
6. projection status,
7. registry refs when present,
8. witness lifecycle state when present,
9. retained artifact refs for replay, diff, explain, or distill outputs,
10. capability level actually relied upon.

### 7.1 Lossy and registry-unpinned input obligations
When a retained upstream artifact arrives without registry refs or with an explicitly lossy projection:

1. treat the intake as provisional rather than silently upgrading it to a registry-pinned surface,
2. surface the `lossy` or `provisional` status explicitly in retained artifacts and UI state,
3. do not make broad witness, pack, or semantic-equivalence claims over a lossy or registry-unpinned observation projection unless retained conformance evidence says that is acceptable for the exact claim being made,
4. if registry pinning is later added to an upstream source, the downstream artifact should record when it transitioned from unpinned to pinned rather than silently back-dating the pinning.

### 7.2 Capture-loss and downgraded-instrumentation obligations
When a source observation declares capture-loss, downgraded instrumentation, or unavailable surfaces:

1. preserve those markers in retained downstream artifacts,
2. surface them in the UI whenever they affect the reliability of a comparison, diff, or explain result,
3. do not treat a capture-incomplete observation as equivalent to a capture-complete observation for diff or witness purposes without explicit acknowledgment,
4. the comparison reliability badge should reflect the capture-loss state.

### 7.3 Comparison reliability badge
Every comparison surface shown in the UI or retained in artifacts should carry a reliability badge derived from its current evidence shape:
1. `direct` where the compared surface is directly observed or directly produced with no declared projection loss,
2. `derived` where the compared surface is a declared downstream derivation over retained direct facts,
3. `lossy` where the current retained view explicitly drops or normalizes facts,
4. `provisional` where the compared surface depends on a still-provisional seam or capability floor.

### 7.4 Comparison-view family rule
When an active adapter or observation seam publishes `comparison_views`, `DNA OneCalc` should prefer per-family comparison over raw replay-event comparison for product-facing diff and explain.

Baseline families for the current XML verification lane are:
1. `visible_value`
2. `effective_display_text`
3. `formatting_view`
4. `conditional_formatting_view`

Interpretation rule:
1. a divergence on one of those families should remain typed to that family in retained diff and explain artifacts,
2. if one side lacks a required family, the retained result should carry a projection coverage gap instead of presenting it as a semantic mismatch,
3. absence of a family still remains an upstream product constraint; it does not widen the local capability floor by itself,
4. when both sides publish a family but their JSON envelopes differ, `DNA OneCalc` must preserve the typed mismatch instead of collapsing it into a missing-family story.

## 8. `OxXlPlay` input labeling and interpretation
When `DNA OneCalc` consumes `OxXlPlay`-originated artifacts through `OxReplay`:

### 8.1 Labeling rule
1. the source lane should be labeled as `oxxlplay` with observation source kind `excel`,
2. the projection status should be labeled `lossy` when the current normalized replay view is explicitly lossy,
3. the capture mode should be carried as declared by `OxXlPlay`, typically `excel_black_box_observation`,
4. Excel build, version, channel, workbook fingerprint, and trigger-recipe provenance should be surfaced when available,
5. capture-loss, downgraded-instrumentation, unavailable-surface, or nondeterminism markers from `OxXlPlay` must be carried through `OxReplay` into `DNA OneCalc` retained artifacts.

### 8.2 Interpretation rule
1. the current `OxXlPlay` normalized replay view is useful for replay-path activation, declared family comparison, and provenance-carrying intake through `source_metadata`,
2. it is still not the right basis for broad semantic equivalence, formatting-complete parity, or registry-heavy witness claims,
3. `comparison_views` and `source_metadata` should be consumed as declared surfaces rather than reconstructed from raw normalized family strings,
4. `DNA OneCalc` should not present the current `OxXlPlay`-backed comparison as Excel parity truth where the underlying view is declared lossy.

### 8.3 Platform rule
1. live Excel-backed comparison remains Windows-only because the current live `OxXlPlay` capture path remains Windows-only,
2. non-Windows hosts may still replay, diff, and explain retained `OxXlPlay` artifacts through `OxReplay`,
3. the `DNA OneCalc` UI must surface the Windows-only restriction when the mode involves live Excel observation.

## 9. UI visibility and control rule
`DNA OneCalc` may expose:
1. replay capture state,
2. replay validation status,
3. diff and explain results,
4. witness lifecycle and distill controls,
5. scenario-library controls over retained replay evidence.

### 9.1 Required visible replay facts
The UI should make these replay facts visible when they affect interpretation:
1. capability floor actually relied upon per active lane,
2. source lane or observation source identity,
3. capture mode,
4. projection status including `lossy` when applicable,
5. capture-loss or downgraded-instrumentation markers,
6. Windows-only availability for live Excel comparison,
7. registry pinning status when registry-governed outputs are displayed,
8. witness lifecycle state and quarantine reason when present,
9. comparison reliability badge,
10. provisional or experimental status for modes that do not yet meet their declared floor.

### 9.2 Full replay visibility direction
`DNA OneCalc` should treat replay as a first-class product plane, not optional garnish.

Every meaningful session should be capable of becoming retained evidence, and every retained evidence item should be capable of becoming an upstream work request.

The intended visibility spine is:
1. scenario authoring with explicit host profile and input bindings,
2. scenario execution with replay capture,
3. retained replay evidence per executed scenario,
4. comparison against retained Excel or other replay evidence,
5. typed diff with mismatch classification,
6. causal explanation with adapter-sourced records,
7. witness retention with lifecycle state,
8. handoff packet emission with exact seam pins and capability floor.

### 9.3 Control surface guidance
`DNA OneCalc` should expose these `OxReplay`-backed controls:
1. validate bundle before replay,
2. select and execute replay scenario,
3. select comparison targets and invoke diff,
4. query explain for specific mismatch or reject records,
5. invoke distill when the lane floor supports it (otherwise hide or disable),
6. inspect and manage witness lifecycle state,
7. export handoff packets with full lineage,
8. browse and manage the retained scenario library.

## 10. Scenario-library growth rule
The intended growth path is:
1. author scenario,
2. run scenario,
3. emit retained replay evidence,
4. compare against retained Excel or other replay evidence,
5. explain mismatch,
6. retain witness,
7. emit upstream handoff.

`DNA OneCalc` should treat the replay corpus as provenance-bearing retained evidence, not as disposable UI cache state.

### 10.1 Scenario promotion caution
The first scenario families promoted into comparison and replay spines should favor:
1. `OxFml` lanes whose host and replay artifacts are already deterministic and typed,
2. `OxFunc` rows with stable semantic closure or explicit doc-modeled seam contracts consumed through `OxFml`,
3. `OxXlPlay` scenarios with retained provenance-rich bundle emission and no hidden capture assumptions.

Avoid promoting these as product-claim families unless explicitly marked provisional:
1. lanes or adapters with no accepted local `OxReplay` capability floor,
2. broad conditional-formatting or data-validation lanes beyond the current restricted-carrier floor,
3. Excel-comparison claims that depend on the current lossy replay projection as if it were complete semantic equivalence truth.

### 10.2 Current caution
1. the first `OxXlPlay` normalized replay view is useful for replay-path activation and coarse comparison wiring,
2. it is not yet the right basis for broad semantic equivalence, formatting-complete parity, or registry-heavy witness claims,
3. the `OxFml` replay floor is honest through `C3.explain_valid` but `C4.distill_valid` and `C5.pack_valid` remain later evidence lanes,
4. `OxFunc` and `OxVba` direct replay intake remain later narrower lanes.

## 11. Current local limits
The following limits remain explicit:
1. this repo still has no app-facing `DNA OneCalc` host contract analogous to `DNA_RECALC_HOST.md`,
2. `OxFunc` has no accepted local direct replay-intake floor yet,
3. `OxVba` has no accepted local replay-intake floor yet,
4. the `OxXlPlay` seam still lacks a formal adapter capability manifest and richer registry-pinned diff structure,
5. the current `OxXlPlay` replay-facing normalized view remains explicitly `lossy`,
6. broad lane `C4` or `C5` claims remain later evidence lanes unless retained conformance says otherwise,
7. `DNA OneCalc` currently consumes replay as shared infrastructure rather than through a dedicated app-facing host contract; that gap is acknowledged upstream documentation debt,
8. the shared runtime can now carry typed comparison-view families and both the OxFml and OxXlPlay XML verification lanes now publish them, but the current cross-lane XML comparison still remains partial because the retained family payloads diverge and the OxXlPlay side remains explicitly lossy.

## 12. Resulting rule
`DNA OneCalc` should use `OxReplay` as shared replay infrastructure, not as a substitute product host, lane semantics owner, or replacement for `DNA ReCalc`.
