# NOTES_FOR_OXFML

## Purpose
Record `OxReplay` observations that materially affect `OxFml` adapter or replay-facing design.

## Core message
The current `OxFml` intake passes the local `C0` through `C3` validator floor and projects the first retained FEC commit case through the shared replay path, but `OxReplay` still depends on a fixture-specific intake shape that is narrower than the OxFml replay-facing direction now documented upstream.

The highest-value interface changes for `OxReplay` are:
1. publish machine-readable replay projection metadata, including source-case-id to shared-scenario alias bindings, instead of leaving that mapping private to `OxReplay`,
2. expose a narrower replay-projection entry surface as the preferred long-term consumer-facing path,
3. preserve more of the replay-relevant OxFml metadata that already exists in fixture and runtime artifacts, especially fence and library-context pin information.

## Current evidence
1. retained manifest-validation output: `docs/test-runs/w003-conformance-oxfml-replay-adapter-v1-baseline/`
2. retained replay intake output: `docs/test-runs/w004-oxfml-oxfml_fec_accept_publication_001-baseline/`
3. current source scenario anchor imported by `OxReplay`: `crates/oxfml_core/tests/fixtures/fec_commit_replay_cases.json` with case id `fec_001_accept`
4. current local `OxReplay` intake loader and alias map: `src/oxreplay-core/src/lib.rs`
5. current `DNA ReCalc` `OxFml` intake switch: `src/oxreplay-dnarecalc-cli/src/main.rs`
6. OxFml replay-facing preferred consumer direction: `docs/spec/OXFML_CONSUMER_INTERFACE_REARCHITECTURE_PLAN.md`
7. OxFml replay adapter contract and manifest: `docs/spec/OXFML_REPLAY_APPLIANCE_ADAPTER_V1.md`, `docs/spec/OXFML_REPLAY_ADAPTER_CAPABILITY_MANIFEST_V1.json`

## Interface implications
1. publish source-case-id to replay-scenario-id alias bindings in a machine-readable artifact next to the retained replay fixture families so `OxReplay` does not need a private hardcoded alias map
2. make the preferred Replay-facing contract a replay projection service or facade over OxFml-owned artifacts rather than a loose collection of proving-host helpers and fixture-family assumptions
3. include replay-relevant preserved metadata in that projection surface:
   - source schema id and source artifact family
   - source case id and shared scenario id
   - observed fence members when present, including `formula_token`, `snapshot_epoch`, `bind_hash`, `profile_version`, and `capability_view_key`
   - library-context snapshot ref when present
   - registry bindings, capability floor, and lifecycle metadata when applicable
4. identify the first supported projection families beyond the initial FEC commit family in a machine-readable way; the most useful next family for `OxReplay` is session lifecycle because the OxFml manifest already advertises it
5. keep the current capability stance honest:
   - `C0` through `C3` remain the accepted local floor,
   - `C4` remains scaffolded,
   - this interface suggestion does not ask OxFml to widen capability claims prematurely
6. keep normalized replay families additive and non-authoritative; `OxReplay` should continue consuming `OxFml` meaning through declared adapters only

## Minimum invariants
1. source case ids remain `OxFml`-authoritative even when `OxReplay` retains alias ids for cross-lane replay packets
2. shared scenario aliases must be published by `OxFml` as transport aids, not as replacements for source case ids
3. `OxReplay` must not infer broader replay-safe rewrite authority than `OxFml` has declared locally
4. fence and provider-pin metadata that matter to replay identity or explanation must not be silently dropped by the preferred projection surface
5. scaffolded `C4` remains scaffolded until retained lifecycle-aware evidence is exported and claimed explicitly

## Response processed locally
`OxFml` has now answered the main seam questions through:
1. `../OxFml/docs/upstream/NOTES_FOR_OXREPLAY.md`
2. `../OxFml/docs/spec/OXFML_CONSUMER_INTERFACE_AND_FACADE_CONTRACT_V1.md`

Current processed local read:
1. alias publication is expected inline in replay projection results,
2. the current replay facade shape is `ReplayProjectionRequest`, `ReplayProjectionService`, and `ReplayProjectionResult`,
3. the ordinary downstream consumer surface is now `consumer::replay` with `consumer::runtime` used when replay projection begins from runtime/session result objects,
4. `OxReplay` should now consume the landed `OxFml_V1` seam rather than treating it as a future target.

## Remaining implementation questions
1. what is the first retained shared replay packet after the accepted FEC-style projection that should be checked in locally
2. which session-lifecycle packet should become the first retained post-FEC shared replay intake
3. what additional local conformance checks should be added around preserved projection metadata
