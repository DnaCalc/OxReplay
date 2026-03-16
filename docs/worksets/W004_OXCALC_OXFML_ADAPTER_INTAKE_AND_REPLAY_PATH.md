# W004 — OxCalc/OxFml Adapter Intake And Replay Path

## Objective
Exercise the first real cross-lane shared replay path through the `OxCalc` and `OxFml` adapter surfaces.

## Scope
1. Intake of the current `OxCalc` replay surface.
2. Intake of the current `OxFml` replay surface.
3. Shared replay and diff flow over those adapters.
4. First retained validation fixtures or witness handles for both lanes.

## Dependencies
1. `W002`
2. `W003`
3. Current integrated adapter surfaces in `OxCalc` and `OxFml`.

## Exit gate
1. `OxCalc` and `OxFml` can both pass through the shared runtime path.
2. Differences between lane-local and shared-runtime responsibilities are explicit.
3. Next extraction candidates for `OxReplay` are named.

## Expected capability impact
1. no capability level is pre-claimed; `W004` measures and records the currently supported `OxCalc` and `OxFml` adapter levels
2. any resulting capability claim must be lane-specific and evidence-backed

## Expected pack impact
1. candidate evidence input for `PACK.replay.appliance`
2. candidate evidence input for `PACK.diff.cross_engine.continuous`
3. no pack claim exists until retained lane scenarios and replay outputs are linked

## Environment Preconditions
1. inbound `NOTES_FOR_OXREPLAY.md` ledgers are re-checked at activation time
2. current `OxCalc` and `OxFml` adapter surfaces are available from the sibling repos
3. shared fixture-sharing and handoff expectations are explicit before retained evidence is copied or mirrored
4. the initial implementation stack declared in `W002` is in force

## Evidence Layout
1. canonical emitted artifact root: `docs/test-corpus/bundles/` for retained lane bundles, `docs/test-corpus/witnesses/` when retained witness handles are promoted, and `docs/test-runs/` for replay or diff outputs
2. checked-in versus ephemeral policy: retained cross-lane fixtures and promoted witness handles are checked in; exploratory comparison runs may remain ephemeral until promoted
3. stable naming policy for baseline runs: `w004-<lane-id>-<scenario-id>-baseline`

## Replay-Corpus Readiness
1. replay classes requiring corpus scenarios before activation: `oxcalc_intake`, `oxfml_intake`, `shared_replay`, `shared_diff`
2. scenario ids satisfying them:
   - `oxcalc_intake` -> `oxcalc_tracecalc_accept_publish_001`
   - `oxfml_intake` -> `oxfml_fec_accept_publication_001`
   - `shared_replay` -> `crosslane_replay_identity_001`
   - `shared_diff` -> `crosslane_diff_mismatch_001`
3. reserve or later lanes: shared distillation, lifecycle promotion, and broad pack-export claims

## Pack-Evidence Traceability
1. pack name: `PACK.replay.appliance`, `PACK.diff.cross_engine.continuous`
2. replay classes: `oxcalc_intake`, `oxfml_intake`, `shared_replay`, `shared_diff`
3. scenario ids or artifact paths:
   - `oxcalc_tracecalc_accept_publish_001` -> lane source import from `../OxCalc`
   - `oxfml_fec_accept_publication_001` -> lane source import from `../OxFml`
   - `crosslane_replay_identity_001` -> `docs/test-corpus/bundles/crosslane_replay_identity_001/`
   - `crosslane_diff_mismatch_001` -> `docs/test-corpus/bundles/crosslane_diff_mismatch_001/`
   - retained human-readable runs -> `docs/test-runs/w004-<lane-id>-<scenario-id>-baseline/`
