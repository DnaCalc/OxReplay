# HANDOFF_OXFML_001_REPLAY_INTERFACE_SUGGESTIONS

## Header
1. handoff id: `HANDOFF_OXFML_001`
2. date: `2026-04-01`
3. from repo: `OxReplay`
4. to repo: `OxFml`
5. related workset or feature id: `W004_OXCALC_OXFML_ADAPTER_INTAKE_AND_REPLAY_PATH`

## Purpose
State the `OxReplay`-visible interface changes that would reduce current private coupling to OxFml replay inputs while staying inside the declared OxFml semantic boundary.

## Proposed change
`OxFml` should publish a narrower Replay-facing consumer surface and enough machine-readable projection metadata that `OxReplay` can stop depending on private fixture assumptions.

The suggested change set is:
1. publish machine-readable source-case-id to shared-scenario-id alias bindings for retained replay-facing fixture families
2. publish the first preferred replay projection entry surface as an OxFml-owned service or facade rather than leaving Replay consumers to discover raw helpers or fixture families ad hoc
3. preserve replay-relevant metadata in that projection surface, including:
   - source schema id
   - source artifact family
   - source case id
   - shared scenario id
   - observed fence members when present
   - library-context snapshot ref when present
   - registry bindings
   - capability floor
   - lifecycle metadata when applicable
4. publish the first supported projection families beyond `fec_commit_replay_cases.json`, with `session_lifecycle_replay_cases.json` as the most useful next family for `OxReplay`
5. keep current capability claims unchanged in this interface wave:
   - accept the current `C0` through `C3` floor,
   - keep `C4` scaffolded,
   - do not treat packaging or projection improvements as capability promotion by themselves

## Current evidence
1. `OxReplay` currently hardcodes OxFml case-id alias mapping in `src/oxreplay-core/src/lib.rs`
2. `OxReplay` loaded the initial retained OxFml baseline through the `oxfml-fec-commit` kind in `src/oxreplay-dnarecalc-cli/src/main.rs`; that path is now retained as transitional support rather than the preferred seam
3. retained local intake baseline: `docs/test-runs/w004-oxfml-oxfml_fec_accept_publication_001-baseline/report.json`
4. retained local conformance baseline: `docs/test-runs/w003-conformance-oxfml-replay-adapter-v1-baseline/report.json`
5. current OxFml source fixture anchor: `../OxFml/crates/oxfml_core/tests/fixtures/fec_commit_replay_cases.json`
6. current OxFml replay-facing direction already points toward a replay facade in `../OxFml/docs/spec/OXFML_CONSUMER_INTERFACE_REARCHITECTURE_PLAN.md`
7. current OxFml replay adapter contract and capability manifest already define additive replay transport and an honest `C0` through `C3` floor in:
   - `../OxFml/docs/spec/OXFML_REPLAY_APPLIANCE_ADAPTER_V1.md`
   - `../OxFml/docs/spec/OXFML_REPLAY_ADAPTER_CAPABILITY_MANIFEST_V1.json`

## Impact
1. capability impact: none by itself; this handoff is about interface publication and metadata preservation, not a higher capability claim
2. pack impact: improves the traceability and reproducibility of retained replay intake for `PACK.replay.appliance` and later `PACK.diff.cross_engine.continuous`
3. migration or fallback impact:
   - preferred path: OxFml publishes machine-readable alias and replay projection metadata plus a narrower replay-facing entry surface
   - fallback path: if the replay facade is not ready yet, publish the alias and projection metadata first and keep current helper entrypoints as transitional support
4. affected repos or hosts: `OxFml`, `OxReplay`, `DNA ReCalc`, later `DNA OneCalc`, later `OxCalc` replay-aware intake

## Requested response
1. confirm whether `OxFml` wants to publish shared-scenario aliases as a dedicated descriptor artifact or inline in retained replay fixture families
2. confirm the preferred first Replay-facing entry surface for downstream consumers before full facade packaging lands
3. identify the first projection family after FEC commit that `OxReplay` should target next
4. confirm the minimum metadata floor that OxFml wants retained in the preferred replay projection result

## Response processed by OxReplay
1. `OxFml` replied through `../OxFml/docs/upstream/NOTES_FOR_OXREPLAY.md`
2. `OxFml` promoted the replay-facing direction into the canonical draft packet `../OxFml/docs/spec/OXFML_CONSUMER_INTERFACE_AND_FACADE_CONTRACT_V1.md`
3. the current OxFml reply answers the main seam questions as follows:
   - replay projection is the preferred long-term consumer entry surface
   - machine-readable alias publication should be part of replay projection metadata
   - projection results should preserve source schema, source family, pin refs, fence members, registry bindings, capability floor, lifecycle metadata, and replay envelope refs
   - ordinary downstream public entry is `consumer::runtime`, `consumer::editor`, and `consumer::replay`, while helper and adapter projection paths are implementation substrate or transitional support
4. `OxReplay` now treats that reply as the current `OxFml_V1` seam baseline for local spec alignment and downstream implementation, while keeping current capability state unchanged
5. remaining open implementation-shaping questions are:
   - which retained OxFml V1 packet should be the first post-FEC shared replay intake checked in locally
   - which projection family should be the first post-FEC retained shared replay intake lane
