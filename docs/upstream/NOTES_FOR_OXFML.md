# NOTES_FOR_OXFML

## Purpose
Record `OxReplay` observations that materially affect `OxFml` adapter or replay-facing design.

## Core message
The current `OxFml` intake passes the local `C0` through `C3` validator floor and projects the first retained FEC commit case through the shared replay path, but the case-id to shared-scenario alias remains local to `OxReplay`.

## Current evidence
1. retained manifest-validation output: `docs/test-runs/w003-conformance-oxfml-replay-adapter-v1-baseline/`
2. retained replay intake output: `docs/test-runs/w004-oxfml-oxfml_fec_accept_publication_001-baseline/`
3. current source scenario anchor imported by `OxReplay`: `crates/oxfml_core/tests/fixtures/fec_commit_replay_cases.json` with case id `fec_001_accept`

## Interface implications
1. keep the current alias mapping explicit: `fec_001_accept` is the source case id and `OxReplay` currently mirrors it as `oxfml_fec_accept_publication_001`
2. if `OxFml` later claims `cap.C4.distill_valid`, expose lifecycle states in the manifest at the same time
3. keep normalized replay families additive and non-authoritative; `OxReplay` should continue consuming `OxFml` meaning through declared adapters only

## Minimum invariants
1. source case ids remain `OxFml`-authoritative even when `OxReplay` retains alias ids for cross-lane replay packets
2. `OxReplay` must not infer broader replay-safe rewrite authority than `OxFml` has declared locally
3. scaffolded `C4` remains scaffolded until retained lifecycle-aware evidence is exported

## Open questions
1. should `OxFml` publish machine-readable shared-scenario aliases alongside the fixture families
2. what is the first `OxFml` fixture family after FEC commit that should be promoted into retained shared replay intake
