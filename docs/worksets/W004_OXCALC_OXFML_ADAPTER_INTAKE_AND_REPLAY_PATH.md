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
