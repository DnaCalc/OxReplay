# NOTES_FOR_OXCALC

## Purpose
Record `OxReplay` observations that materially affect `OxCalc` adapter or replay-facing design.

## Core message
The current `OxCalc` replay adapter manifest now fails the retained `OxReplay` W003 validator because it claims `cap.C4.distill_valid` without declaring lifecycle states.

## Current evidence
1. retained manifest-validation output: `docs/test-runs/w003-conformance-oxcalc-tracecalc-replay-adapter-baseline/`
2. retained replay intake output: `docs/test-runs/w004-oxcalc-oxcalc_tracecalc_accept_publish_001-baseline/`
3. current source scenario anchor imported by `OxReplay`: `docs/test-corpus/core-engine/tracecalc/hand-auditable/tc_accept_publish_001.json`

## Interface implications
1. if `OxCalc` wants to keep the local `cap.C4.distill_valid` claim, expose explicit lifecycle states in the adapter manifest
2. if lifecycle states are not yet ready for export, move `cap.C4.distill_valid` out of `claimed_capability_levels`
3. keep the current alias mapping explicit: `tc_accept_publish_001` remains the source id and `OxReplay` currently mirrors it as `oxcalc_tracecalc_accept_publish_001`

## Minimum invariants
1. `OxReplay` must not silently weaken the `C4` floor to accept manifests with no lifecycle states
2. source labels and source scenario ids remain `OxCalc`-authoritative even when `OxReplay` adds normalized event families
3. any future `C5` movement must remain evidence-backed and separate from the current `C4` manifest issue

## Open questions
1. should `OxCalc` publish lifecycle states now, or intentionally stage the current manifest at `C3`
2. should the scenario-id alias be published in a machine-readable way from `OxCalc` rather than only being retained locally in `OxReplay`
