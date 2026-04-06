# NOTES_FOR_OXXLPLAY

## Purpose
Record `OxReplay` observations that materially affect `OxXlPlay` bundle-emission, adapter-contract, or replay-facing design.

## Core message
`OxReplay` has now processed the widened `OxXlPlay` SpreadsheetML acknowledgement and accepts the current dual-artifact seam: a rich source observation bundle plus an `OxReplay`-canonical replay manifest and widened normalized replay view with declared `comparison_views` and replay-facing `source_metadata`.

## Current evidence
1. local seam packet: `docs/spec/OXREPLAY_OXXLPLAY_OBSERVATION_SEAM.md`
2. local retained seam validation run: `docs/test-runs/oxxlplay-seam-xlplay_capture_values_formulae_001-baseline/`
3. local retained SpreadsheetML seam validation run: `docs/test-runs/oxxlplay-seam-xlplay_capture_spreadsheetml_formatting_001-baseline/`
4. sibling-emitted canonical manifest: `../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/oxreplay-manifest.json`
5. sibling-emitted normalized replay view: `../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/views/normalized-replay.json`
6. sibling handoff response: `../OxXlPlay/docs/handoffs/HANDOFF_SPREADSHEETML_VERIFICATION_UPDATE_001.md`

## Interface implications
1. the accepted first-pass model is dual-artifact:
   - the rich source observation bundle remains `OxXlPlay`-authoritative,
   - the `replay.bundle.v1` manifest is the replay-facing intake artifact
2. `lane_id = oxxlplay` is accepted locally as an observation-source intake id rather than a semantic-lane claim
3. `projection_status = lossy` is accepted for the current normalized replay view
4. an immediate formal adapter manifest is not required for the first accepted intake pass
5. keep Excel-driving behavior outside `OxReplay`; `OxReplay` should consume declared observation artifacts only
6. `comparison_views` publication for `visible_value`, `effective_display_text`, `formatting_view`, and `conditional_formatting_view` is now accepted on the SpreadsheetML family
7. replay-facing `source_metadata` is now accepted on the normalized replay artifact as the local carrier for projection status, capture-loss summary, interpretation limits, workbook identity, and family inventory

## Minimum invariants
1. `OxReplay` must not become the owner of Excel-driving logic
2. `OxXlPlay` must distinguish directly observed from inferred fields
3. capture-loss, downgraded instrumentation, and unavailable surfaces must be explicit
4. scenario ids and source observation ids must stay traceable through any local aliasing
5. value-sensitive observations preserved only inside string-encoded normalized families are bootstrap-only and must not become the long-term shared diff contract
6. comparison-view families must remain declared observation facts tied to captured surfaces, not synthetic downstream summaries

## Open questions
1. when should a formal `OxXlPlay` adapter manifest be added on top of the accepted canonical-manifest path
2. which registry families should be pinned first once Excel-origin diff or explain outputs become retained
3. should any per-family payload-harmonization contract be declared for `formatting_view` or `conditional_formatting_view` so cross-lane diffs can distinguish structural envelope mismatch from true semantic divergence more precisely
