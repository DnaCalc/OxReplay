# NOTES_FOR_OXXLPLAY

## Purpose
Record `OxReplay` observations that materially affect `OxXlPlay` bundle-emission, adapter-contract, or replay-facing design.

## Core message
`OxReplay` has now processed the first `OxXlPlay` acknowledgement and accepts the current dual-artifact seam: a rich source observation bundle plus an `OxReplay`-canonical replay manifest and first-pass normalized replay view.

## Current evidence
1. local seam packet: `docs/spec/OXREPLAY_OXXLPLAY_OBSERVATION_SEAM.md`
2. local retained seam validation run: `docs/test-runs/oxxlplay-seam-xlplay_capture_values_formulae_001-baseline/`
3. sibling-emitted canonical manifest: `../OxXlPlay/states/excel/xlplay_capture_values_formulae_001/oxreplay-manifest.json`
4. sibling-emitted normalized replay view: `../OxXlPlay/states/excel/xlplay_capture_values_formulae_001/views/normalized-replay.json`

## Interface implications
1. the accepted first-pass model is dual-artifact:
   - the rich source observation bundle remains `OxXlPlay`-authoritative,
   - the `replay.bundle.v1` manifest is the replay-facing intake artifact
2. `lane_id = oxxlplay` is accepted locally as an observation-source intake id rather than a semantic-lane claim
3. `projection_status = lossy` is accepted for the current normalized replay view
4. an immediate formal adapter manifest is not required for the first accepted intake pass
5. keep Excel-driving behavior outside `OxReplay`; `OxReplay` should consume declared observation artifacts only

## Minimum invariants
1. `OxReplay` must not become the owner of Excel-driving logic
2. `OxXlPlay` must distinguish directly observed from inferred fields
3. capture-loss, downgraded instrumentation, and unavailable surfaces must be explicit
4. scenario ids and source observation ids must stay traceable through any local aliasing
5. value-sensitive observations preserved only inside string-encoded normalized families are bootstrap-only and must not become the long-term shared diff contract

## Open questions
1. what is the first retained Excel-vs-DNA comparison-ready scenario that should enter `OxReplay`
2. when should a formal `OxXlPlay` adapter manifest be added on top of the accepted canonical-manifest path
3. which registry families should be pinned first once Excel-origin diff or explain outputs become retained
