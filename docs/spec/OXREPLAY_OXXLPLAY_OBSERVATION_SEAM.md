# OXREPLAY_OXXLPLAY_OBSERVATION_SEAM.md

## 1. Position
This document defines the initial `OxReplay` integration seam for `OxXlPlay`.

`OxXlPlay` is treated as an Excel observation harness and bundle-emission source, not as a semantics-owning lane in the same sense as `OxCalc` or `OxFml`.

## 2. Seam objective
The seam exists so `OxXlPlay` can emit replay-ready evidence that `OxReplay` can:
1. ingest and validate,
2. replay and diff against DNA Calc lanes,
3. explain and retain as governed evidence,
4. later distill into minimized witnesses when the capture surface is sufficient.

## 3. Boundary rule
`OxReplay` owns:
1. canonical bundle validation and indexing,
2. normalized replay runtime surfaces,
3. diff, explain, witness, lifecycle, and host mechanics,
4. adapter manifest validation and conformance rules.

`OxXlPlay` owns:
1. Excel driving or observation harness behavior,
2. source observation schema truth,
3. declared meaning of Excel-observed fields,
4. capture-loss declarations and uncertainty annotations at the observation boundary.

`OxReplay` must not:
1. drive Excel directly,
2. infer missing observation meaning from partial logs,
3. reinterpret Excel behavior as semantic doctrine,
4. silently discard capture-loss or uncertainty markers emitted by `OxXlPlay`.

## 4. Preferred integration shapes
The seam should support these shapes in descending preference:
1. direct canonical replay-bundle emission from `OxXlPlay`,
2. `OxXlPlay` observation bundles plus a narrow declared adapter projection,
3. plugin-style adapter loading by `DNA ReCalc` if bundle emission is not yet direct.

Preferred rule:
1. emit replay-ready bundles as early as possible,
2. keep Excel-driving specifics outside `OxReplay`,
3. use adapter manifests only for declared projection and capability boundaries.

Current acknowledged model from `OxXlPlay`:
1. a rich `OxXlPlay` observation bundle remains the source-observation contract,
2. an `OxReplay`-canonical `replay.bundle.v1` manifest is emitted as the replay-facing intake artifact over that richer bundle,
3. a normalized replay view may be emitted as a first-pass lossy projection for ingest and replay-path activation.

## 5. Required preserved provenance
For `OxXlPlay`-originating retained artifacts, `OxReplay` should expect preserved fields for:
1. source repo or adapter identity,
2. observation source kind `excel`,
3. Excel product surface where relevant, such as desktop or automation path,
4. Excel build, version, and channel when available,
5. host environment facts needed for reproducibility,
6. workbook or input fingerprint,
7. scenario id or observation run id,
8. trigger recipe or action sequence,
9. calculation mode and relevant host settings when observable,
10. observed surface inventory such as values, formulas, errors, or timing envelopes,
11. capture-loss, downgraded-instrumentation, unavailable-surface, or nondeterminism markers,
12. registry and lifecycle pinning when the evidence is retained.

## 6. Observation-specific contract expectations
The first `OxXlPlay` handoff into `OxReplay` should make these surfaces explicit:
1. what was directly observed versus inferred,
2. which workbook surfaces are in scope for equality or diff checks,
3. which host settings materially affect reproducibility,
4. which surfaces are intentionally omitted from the first pass,
5. what counts as deterministic enough for retained replay evidence,
6. how capture-loss is encoded when Excel does not expose a surface.

## 7. Initial retained acceptance packet
The first useful `OxXlPlay` delivery into `OxReplay` should include:
1. one adapter manifest or equivalent declared observation contract,
2. one minimal valid observation bundle seed,
3. one deliberately capture-incomplete or downgraded observation example,
4. one differential comparison-ready scenario against a DNA Calc lane,
5. explicit source ids and retained scenario aliases if aliasing is required.

## 8. First replay classes reserved for `OxXlPlay`
The initial reserved replay classes for `OxXlPlay` intake are:
1. `xlplay_manifest_shape_valid`
2. `xlplay_observation_bundle_valid`
3. `xlplay_capture_loss_declared`
4. `xlplay_diff_ready_against_dna`

These are reserved planning classes only until retained artifacts exist.

## 9. Processed response status
`OxReplay` has now locally validated both the first acknowledged `OxXlPlay` intake artifacts and the widened SpreadsheetML comparison-view artifacts:
1. first canonical manifest: `../OxXlPlay/states/excel/xlplay_capture_values_formulae_001/oxreplay-manifest.json`
2. first normalized replay view: `../OxXlPlay/states/excel/xlplay_capture_values_formulae_001/views/normalized-replay.json`
3. widened SpreadsheetML canonical manifest: `../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/oxreplay-manifest.json`
4. widened SpreadsheetML normalized replay view: `../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/views/normalized-replay.json`
5. local retained widened seam baseline: `docs/test-runs/oxxlplay-seam-xlplay_capture_spreadsheetml_formatting_001-baseline/`

Accepted first-pass seam answers:
1. `lane_id = oxxlplay` is acceptable as an observation-source intake id and does not imply semantic ownership of Excel behavior
2. `projection_status = lossy` is the right first-pass declaration for the current normalized replay view
3. an empty `registry_refs` list is acceptable for the first intake pass while no registry-dependent capability claim is being made
4. the direct canonical-manifest path is acceptable without an immediate formal adapter manifest; a formal adapter manifest becomes useful when the projection surface or capability claim surface broadens
5. encoding observed values into normalized replay-family strings is acceptable only as a bootstrap activation surface, not as the long-term shared diff contract
6. the SpreadsheetML family may now publish machine-readable `comparison_views` for `visible_value`, `effective_display_text`, `formatting_view`, and `conditional_formatting_view`
7. the replay-facing normalized view may now publish `source_metadata` preserving projection status, capture-loss summary, interpretation limits, workbook identity, and family inventory

Still provisional:
1. a retained Excel-vs-DNA comparison-ready scenario now exists inside `OxReplay`, but it remains a lossy observation-versus-projection comparison surface rather than broad Excel parity truth
2. value-sensitive differential structure has widened into declared comparison families, but the current `formatting_view` and `conditional_formatting_view` envelopes still diverge structurally across lanes and should not be treated as harmonized semantic shapes yet
3. registry pinning should be added once emitted outputs depend on canonical mismatch, severity, capability, or lifecycle families
4. the current widened SpreadsheetML formatting and conditional-formatting surfaces remain `derived` and explicitly `lossy`, not direct Excel parity truth

## 10. `DNA OneCalc` comparison use
`DNA OneCalc` may consume retained `OxXlPlay` artifacts through `OxReplay` for:
1. replay against retained Excel observation artifacts,
2. diff and explain over retained observation inputs,
3. witness creation and scenario-library growth,
4. replay-visible comparison controls in its own UI.

### 10.1 Current consumer rule
1. `DNA OneCalc` should treat the current `OxXlPlay` replay-facing normalized view as a first-pass observation projection,
2. it should not treat that projection as broad semantic equivalence truth,
3. it must keep source observation identity, projection status, and capture-loss visible when those affect interpretation,
4. `DNA OneCalc` should consume published `comparison_views` and replay-facing `source_metadata` directly through `OxReplay` rather than inferring them from raw normalized event strings.

### 10.2 Labeling rule
When `DNA OneCalc` retains or displays `OxXlPlay`-originated artifacts consumed through `OxReplay`:
1. the source lane should be labeled `oxxlplay` with observation source kind `excel`,
2. the projection status should be labeled `lossy` when the current normalized replay view is explicitly lossy,
3. the capture mode should be carried as declared by `OxXlPlay`, typically `excel_black_box_observation`,
4. Excel build, version, channel, workbook fingerprint, and trigger-recipe provenance should be surfaced when available,
5. capture-loss, downgraded-instrumentation, unavailable-surface, or nondeterminism markers must be carried through into retained downstream artifacts and UI state.

### 10.3 Comparison reliability badge
Comparisons that involve `OxXlPlay`-originated inputs should carry a reliability badge:
1. `direct` only when the compared observation surface is directly captured with no declared projection loss,
2. `lossy` when the current retained view explicitly drops or normalizes facts,
3. `provisional` when the comparison depends on a still-provisional seam, capability floor, or registry-unpinned input.

The badge must be visible in both the UI and retained comparison artifacts.

### 10.4 Interpretation caution
1. value-sensitive differential structure should widen beyond string-encoded normalized families before broad equivalence claims are made,
2. `DNA OneCalc` should not present `OxXlPlay`-backed comparison as Excel parity truth where the underlying view is declared lossy,
3. the current seam lacks a formal adapter capability manifest and richer registry-pinned diff structure; that absence is a real product constraint, not a cosmetic gap.

### 10.5 Platform rule
1. live Excel-backed comparison remains Windows-only because the current live `OxXlPlay` capture path remains Windows-only,
2. non-Windows hosts may still replay, diff, and explain retained `OxXlPlay` artifacts through `OxReplay`,
3. the `DNA OneCalc` UI must surface the Windows-only restriction when the active mode involves live Excel observation.

## 11. Resulting rule
The `OxXlPlay` seam is ready when `OxXlPlay` can hand `OxReplay` declared, provenance-rich, replay-ready observation artifacts without forcing `OxReplay` to absorb Excel-driving logic or semantic ownership.
