# HANDOFF_DNAONECALC_001 — XML View-Family Comparison Response

## Header
1. handoff id: `HANDOFF_DNAONECALC_001`
2. date: `2026-04-06`
3. from repo: `OxReplay`
4. to repo: `DnaOneCalc`
5. related workset or feature id: `W004`, `W005`

## Purpose
Respond to `DnaOneCalc`'s XML verification request by declaring the local `OxReplay` comparison-view contract and the retained evidence now proving it.

## Proposed change
`OxReplay` now accepts machine-readable `comparison_views` on replay-facing artifacts and uses them to:
1. classify divergence on `visible_value`, `effective_display_text`, `formatting_view`, and `conditional_formatting_view` explicitly,
2. classify a missing required family as `projection_coverage_gap` instead of a plain semantic mismatch,
3. emit explain records that name the diverged or missing family without requiring host-local reinterpretation.

The local contract shape is:
1. compared artifacts may publish `comparison_views` as an array of `{ "view_family": <id>, "value": <json> }`,
2. `Diff` will compare each family independently when either side publishes `comparison_views`,
3. `Explain` will emit machine-readable records per mismatch with `mismatch_kind`, `severity`, `view_family`, `left_value`, `right_value`, and `detail`.

## Current evidence
1. local shared-runtime loader: `src/oxreplay-core/src/lib.rs`
2. local diff classifier: `src/oxreplay-diff/src/lib.rs`
3. local explain report emitter: `src/oxreplay-explain/src/lib.rs`
4. retained local mechanics fixture: `docs/test-corpus/bundles/crosslane_xml_view_family_gap_001/`
5. retained OxFml intake fixture carrying landed comparison views: `docs/test-corpus/bundles/oxfml_v1_xml_verification_comparison_views_projection_001/`
6. retained OxXlPlay widened seam baseline carrying landed comparison views and replay-facing source metadata: `docs/test-runs/oxxlplay-seam-xlplay_capture_spreadsheetml_formatting_001-baseline/`
7. retained pre-publication integrated diff baseline: `docs/test-runs/w004-shared-crosslane_xml_view_family_gap_integrated_001-baseline/`
8. retained pre-publication integrated explain baseline: `docs/test-runs/w005-dnarecalc-explain-view-family-gap-integrated-baseline/`
9. retained post-publication integrated diff baseline: `docs/test-runs/w004-shared-crosslane_xml_view_family_divergence_integrated_001-baseline/`
10. retained post-publication integrated explain baseline: `docs/test-runs/w005-dnarecalc-explain-view-family-divergence-integrated-baseline/`

## Impact
1. capability impact: no new lane capability claim; this is a shared `C2`/`C3` diff and explain mechanics widening that activates only when upstream artifacts publish `comparison_views`
2. pack impact: candidate evidence widening for `PACK.diff.cross_engine.continuous` and `PACK.replay.appliance`
3. migration or fallback impact: OxFml and OxXlPlay now both publish the four XML comparison-view families for the retained integrated path; the current integrated XML baseline has moved from missing-family coverage gaps to typed per-family divergence, and `DnaOneCalc` should preserve that typed result while also surfacing the OxXlPlay-side `lossy` and provenance qualifiers from `source_metadata`
4. affected repos or hosts: `DnaOneCalc`, `OxFml`, `OxXlPlay`, `DNA ReCalc`

## Requested response
1. `DnaOneCalc` should preserve the typed per-family mismatch kinds now emitted when both sides publish `visible_value`, `effective_display_text`, `formatting_view`, and `conditional_formatting_view`
2. `DnaOneCalc` may import the per-record explain JSON directly when the richer shape is present
3. `DnaOneCalc` should continue to treat future missing-family cases as `projection_coverage_gap`, but the current retained integrated XML path is no longer a missing-family case
4. `DnaOneCalc` should surface the OxXlPlay-side `projection_status`, `capture_loss`, `interpretation_limits`, and other replay-facing `source_metadata` qualifiers alongside these typed comparison results
