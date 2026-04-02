# OXREPLAY_INITIAL_ADAPTER_INTAKE_PLAN.md

## 1. Position
This document defines the first adapter-intake plan for `OxCalc`, `OxFml`, and the initial `OxXlPlay` observation seam.

It is a local planning companion for `W004`.
It does not redefine lane adapter semantics.
For downstream hosts such as `DNA OneCalc`, read this file as current intake ordering and current evidence floor, not as a host-facing API contract.

## 2. Intake sequence
1. intake `OxCalc` first because Foundation already treats it as the first expected lane toward higher replay capability growth
2. intake `OxFml` second in the same workset, with initial emphasis on ingest/replay/diff/explain rather than distillation
3. prepare the `OxXlPlay` seam as an observation-source contract before direct intake begins
4. defer `OxFunc` and `OxVba` to later narrower lanes after the first two lanes have stabilized inside `OxReplay`
5. downstream hosts like `DNA OneCalc` should not read this ordering as permission to assume direct `OxFunc` or `OxVba` replay support before local intake evidence exists

## 3. OxCalc intake expectations
`OxReplay` expects these source surfaces from `OxCalc`:
1. replay-facing scenario ids and replay-class bindings
2. source labels plus normalized event-family projection
3. retained views, reject sets, counters, and diff surfaces
4. adapter capability manifest and registry pin

Current planning scenario:
1. `oxcalc_tracecalc_accept_publish_001`

Current retained source and observations:
1. source anchor: `../OxCalc/docs/test-corpus/core-engine/tracecalc/hand-auditable/tc_accept_publish_001.json`
2. current alias mapping: `tc_accept_publish_001` -> `oxcalc_tracecalc_accept_publish_001`
3. current conformance result in `OxReplay`: manifest loads, but current `cap.C4.distill_valid` claim is rejected locally until lifecycle states are declared

Questions to answer during activation:
1. which retained `TraceCalc` artifacts should be mirrored locally versus referenced remotely
2. whether `engine_diff` is already sufficient for shared diff intake or requires a narrower projection pass
3. which `OxCalc` capability level is honestly claimable at intake time

## 4. OxFml intake expectations
`OxReplay` expects these source surfaces from `OxFml`:
1. typed session, candidate, commit, and reject projections
2. fixture-family import discipline with stable scenario ids
3. adapter capability manifest and registry pin
4. explicit current limits on replay-safe transforms and distillation claims

Current planning scenario:
1. `oxfml_fec_accept_publication_001`

Current retained source and observations:
1. retained intake anchor: `docs/test-corpus/bundles/oxfml_v1_replay_projection_001/projection.json`
2. current shared scenario alias carried by the retained intake packet: `oxfml_fec_accept_publication_001`
3. current conformance result in `OxReplay`: manifest passes the local `C0` through `C3` validator floor and keeps `C4` scaffolded
4. current OxFml reply to `OxReplay`: `../OxFml/docs/upstream/NOTES_FOR_OXREPLAY.md`
5. current canonical `OxFml_V1` consumer packet for this seam: `../OxFml/docs/spec/OXFML_CONSUMER_INTERFACE_AND_FACADE_CONTRACT_V1.md`

Current consumed `OxFml_V1` seam from OxFml:
1. the preferred OxFml -> `OxReplay` entry surface is the replay facade rather than broad helper or fixture-family dependence
2. the current replay-facing packet family is:
   - `ReplayProjectionRequest`
   - `ReplayProjectionService`
   - `ReplayProjectionResult`
3. the preferred ordinary public module entry for downstream consumers is `oxfml_core::consumer::replay`, with `consumer::runtime` used only when projection begins from runtime or session result objects
4. the consumed projection result should preserve:
   - source case id
   - shared scenario alias when present
   - source schema id and source artifact family
   - pinned `LibraryContextSnapshotRef` when present
   - typed query bundle carriage and replay-relevant captured packet truth when present
   - registry bindings and lifecycle metadata when applicable
   - lifecycle metadata when applicable
   - canonical replay envelope refs and sidecar refs
5. shared-scenario alias publication is expected inline in projection results rather than as a separate alias sidecar
6. this seam direction is packaging and metadata publication only; it does not widen the current accepted OxFml capability floor beyond `C3`

Questions to answer during activation:
1. which retained OxFml V1 projection packets should be checked in locally as stable replay-governed evidence beyond the first accepted packet
2. which session-lifecycle packet should act as the first retained shared replay import after the accepted FEC-style commit projection
3. what the first shared explain surface should consume once replay projection results become the preferred intake packet
4. what remains local-only evidence versus retained shared replay evidence as V1 packet coverage expands

## 5. Shared intake invariants
1. `OxReplay` consumes declared adapter meaning; it does not reinterpret lane semantics
2. source scenario ids remain authoritative
3. local retained imports must preserve source-lane identity and schema lineage
4. any normative pressure discovered during intake must route back through outbound notes or handoff packets

## 6. OxXlPlay seam expectations
`OxReplay` expects these source surfaces from `OxXlPlay`:
1. replay-ready observation artifacts rather than opaque automation logs
2. explicit direct-observation versus inference markers
3. Excel build, version, channel, workbook fingerprint, and trigger-recipe provenance
4. capture-loss, downgraded-instrumentation, unavailable-surface, or nondeterminism declarations when applicable
5. either direct canonical replay-bundle emission or a narrow declared observation-contract phase

Reserved first seam classes:
1. `xlplay_manifest_shape_valid`
2. `xlplay_observation_bundle_valid`
3. `xlplay_capture_loss_declared`
4. `xlplay_diff_ready_against_dna`

Acknowledged current emitted shapes from `OxXlPlay`:
1. source observation bundle: `../OxXlPlay/states/excel/xlplay_capture_values_formulae_001/bundle.json`
2. canonical replay manifest: `../OxXlPlay/states/excel/xlplay_capture_values_formulae_001/oxreplay-manifest.json`
3. normalized replay view: `../OxXlPlay/states/excel/xlplay_capture_values_formulae_001/views/normalized-replay.json`
4. accepted first-pass seam choices:
   - `lane_id`: `oxxlplay`
   - `capture_mode`: `excel_black_box_observation`
   - `projection_status`: `lossy`
   - `source_schema`: `oxxlplay.replay_bundle_seed.v1`
   - empty `registry_refs` for the first non-claiming intake pass

Questions to answer during activation:
1. what is the first deterministic enough workbook or scenario family for retained comparison
2. which observation surfaces are in the first equality and diff envelope
3. when should a formal adapter manifest be added beyond the accepted canonical-manifest path
4. when should registry pinning begin for retained Excel-origin diff and explain outputs

## 7. Planned outbound observation triggers
Write or update outbound notes if intake discovers:
1. missing adapter manifest fields needed by shared validation
2. missing source identity fields needed by shared replay/explain
3. lane-local naming drift that requires explicit adapter normalization
4. lifecycle or registry requirements that are not yet exposed by the lane repo

Current triggered outbound notes:
1. `NOTES_FOR_OXCALC.md` for the current `C4` lifecycle-state gap and retained alias mapping
2. `NOTES_FOR_OXFML.md` for the first shared replay intake expectations and preserved projection metadata floor
3. `NOTES_FOR_OXXLPLAY.md` for the initial observation-to-replay seam expectations

Current processed inbound reply:
1. `../OxFml/docs/upstream/NOTES_FOR_OXREPLAY.md` is now treated locally as the current `OxFml_V1` replay consumer seam for downstream implementation
