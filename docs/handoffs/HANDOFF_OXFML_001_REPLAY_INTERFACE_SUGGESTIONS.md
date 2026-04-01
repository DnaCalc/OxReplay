# HANDOFF_OXFML_001_REPLAY_INTERFACE_SUGGESTIONS

## Header
1. handoff id: `HANDOFF_OXFML_001`
2. date: `2026-04-01`
3. from repo: `OxReplay`
4. to repo: `OxFml`
5. related workset or feature id: `W004_OXCALC_OXFML_ADAPTER_INTAKE_AND_REPLAY_PATH`

## Purpose
Record the `OxReplay`-visible replay seam that OxFml has now published and that `OxReplay` now consumes.

## Consumed seam
`OxFml` now publishes a Replay-facing consumer surface and machine-readable projection metadata that `OxReplay` consumes through the canonical `OxFml_V1` seam.

The consumed change set is:
1. publish machine-readable source-case-id to shared-scenario-id alias bindings for retained replay-facing fixture families
2. publish the preferred replay projection entry surface as an OxFml-owned service or facade
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
4. publish the first supported projection families beyond the initial accepted packet, with `session_lifecycle_replay_cases.json` as the most useful next family for `OxReplay`
5. keep current capability claims unchanged in this interface wave:
   - accept the current `C0` through `C3` floor,
   - keep `C4` scaffolded,
   - do not treat packaging or projection improvements as capability promotion by themselves

## Current evidence
1. `OxReplay` now consumes retained OxFml replay packets through the `oxfml-v1-replay-projection` kind in `src/oxreplay-dnarecalc-cli/src/main.rs`
2. `OxReplay` now preserves the full retained projection packet in `src/oxreplay-core/src/lib.rs`
3. retained local intake baseline: `docs/test-runs/w004-oxfml-oxfml_fec_accept_publication_001-baseline/report.json`
4. retained local conformance baseline: `docs/test-runs/w003-conformance-oxfml-replay-adapter-v1-baseline/report.json`
5. current retained OxFml intake packet anchor: `docs/test-corpus/bundles/oxfml_v1_replay_projection_001/projection.json`
6. current OxFml replay-facing direction already points toward a replay facade in `../OxFml/docs/spec/OXFML_CONSUMER_INTERFACE_REARCHITECTURE_PLAN.md`
7. current OxFml replay adapter contract and capability manifest already define additive replay transport and an honest `C0` through `C3` floor in:
   - `../OxFml/docs/spec/OXFML_REPLAY_APPLIANCE_ADAPTER_V1.md`
   - `../OxFml/docs/spec/OXFML_REPLAY_ADAPTER_CAPABILITY_MANIFEST_V1.json`

## Impact
1. capability impact: none by itself; this handoff is about interface publication and metadata preservation, not a higher capability claim
2. pack impact: improves the traceability and reproducibility of retained replay intake for `PACK.replay.appliance` and later `PACK.diff.cross_engine.continuous`
3. affected repos or hosts: `OxFml`, `OxReplay`, `DNA ReCalc`, later `DNA OneCalc`, later `OxCalc` replay-aware intake

## Resolved reply
1. shared-scenario aliases are published inline in retained replay projection results
2. the preferred Replay-facing entry surface for downstream consumers is the replay projection seam
3. the next projection family after the accepted first packet should be session lifecycle
4. the preferred replay projection result preserves the metadata floor `OxReplay` requested

## Response processed by OxReplay
1. `OxFml` replied through `../OxFml/docs/upstream/NOTES_FOR_OXREPLAY.md`
2. `OxFml` promoted the replay-facing direction into the canonical draft packet `../OxFml/docs/spec/OXFML_CONSUMER_INTERFACE_AND_FACADE_CONTRACT_V1.md`
3. the current OxFml reply answers the main seam questions as follows:
   - replay projection is the preferred long-term consumer entry surface
   - machine-readable alias publication should be part of replay projection metadata
   - projection results should preserve source schema, source family, pin refs, fence members, registry bindings, capability floor, lifecycle metadata, and replay envelope refs
   - ordinary downstream public entry is `consumer::runtime`, `consumer::editor`, and `consumer::replay`
4. `OxReplay` now treats that reply as the current `OxFml_V1` seam baseline for local spec alignment and downstream implementation, while keeping current capability state unchanged
5. remaining open implementation-shaping questions are:
   - which retained OxFml V1 packet should be the first post-FEC shared replay intake checked in locally
   - which projection family should be the first post-FEC retained shared replay intake lane
