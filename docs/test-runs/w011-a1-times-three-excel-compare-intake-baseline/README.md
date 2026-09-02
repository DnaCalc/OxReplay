# w011-a1-times-three-excel-compare-intake-baseline

- Date: 2026-09-02
- Origin: DnaTreeCalc W011 bead `dtc-j7n8.12` (Wave 2b, OxReplay value-diff DNA vs Excel on the W011 fixture)
- Execution state: `blocked` (typed gaps `oxreplay-5nn` in this repo, `dtc-j7n8.21` in DnaTreeCalc; `CURRENT_BLOCKERS.md` `BLK-REPLAY-005`)
- Scope status: `scope_partial` (intake of both Excel-observed states is exercised; the DNA-vs-Excel verdict is NOT produced), `target_partial`, `integration_completeness: partial`
- Capability impact: none (shared validate/replay/diff surfaces only; no adapter capability claim)
- Pack impact: none (candidate `PACK.replay.appliance` intake evidence only)
- Reviewed inbound observations: `../OxXlPlay/docs/upstream/` has no `NOTES_FOR_OXREPLAY.md`; the OxXlPlay run note `../OxXlPlay/docs/test-runs/W011_A1_TIMES_THREE_EXCEL_COMPARE.md` was read and its honest limits carried forward below.

## What this run is
`DNA ReCalc` intake of the two OxXlPlay retained states captured by `dtc-j7n8.11`
(real Excel 16.0 build 20326 observing the W011 fixture before and after the DNA save):

| State | Excel observed | workbook_fingerprint |
|---|---|---|
| `../OxXlPlay/states/excel/xlplay_w011_a1_times_three_pre_edit_001/` | Sheet1!A1 = 7, Sheet1!B1 = 21, B1 formula `=A1*3` | `sha256:6e7f61c3...` (= `DnaTreeCalc/fixtures/w011/a1_times_three.xlsx`) |
| `../OxXlPlay/states/excel/xlplay_w011_a1_times_three_dna_saved_001/` | Sheet1!A1 = 10, Sheet1!B1 = 30, B1 formula `=A1*3` | `sha256:6ba5af8d...` (= `DnaTreeCalc/target/w011/a1_times_three_saved.xlsx`, DNA-saved through OxDoc) |

Commands are in `command.txt` (cwd is this repo; every invocation ran with `--offline`).
The OxXlPlay states were read only; nothing under `../OxXlPlay` was mutated
(Validation Non-Mutation Rule, `OPERATIONS.md` 13.3).

## Results
| Artifact | Exit | Result |
|---|---|---|
| `pre-edit.validate-bundle.json` | 0 | `valid`; indexes 7 sidecars and 3 views (`normalized-replay`, `execution-outcome`, `comparison-value`) |
| `dna-saved.validate-bundle.json` | 0 | `valid`; same index |
| `pre-edit.replay.json` | 0 | 3 events (A1 value, B1 value, B1 formula `==A1*3`); `comparison_views` = `comparison_value` (number 7.0) + `execution_outcome` (`accepted_execution` / `value_published`) |
| `dna-saved.replay.json` | 0 | 3 events; `comparison_views` = `comparison_value` (number 10.0) + `execution_outcome` (`accepted_execution` / `value_published`) |
| `pre-edit.self-diff.json`, `dna-saved.self-diff.json` | 0 | `equivalent: true` (mechanics sanity: the aligned-JSON envelope parses inside the local comparator seam) |
| `cross-state.diff.json` | 1 | `equivalent: false`; **exactly one** mismatch: `worksheet_comparison_value` 7.0 vs 10.0 (`worksheet_value_exact`, semantic) |
| `cross-state.explain.json` | 1 | one record, `comparison diverged on worksheet_comparison_value` |

## The typed gap (why there is no verdict)
The cross-state diff is the discriminating run. Both captures observed Sheet1!B1
directly (21 before, 30 after) and the B1 value is the campaign's decisive
requirement (post-save cached B1 = 30, not the stale 21). Yet OxReplay reports
only the A1 divergence and says **nothing about B1**, because:

1. OxXlPlay's normalized-replay v2 projects only the first `cell_value` surface
   into `comparison_views[comparison_value]`; the event
   `normalized_family` strings (`excel.surface.cell_value.direct:Sheet1!B1:comparison_value`)
   no longer carry the value, and `diff_summary` ignores events once
   `comparison_views` are present.
2. The per-surface values do exist in `views/comparison-value.json`
   (`surfaces[]` with `locator` + `oxfunc_value_types.aligned_json.v1` envelope),
   which `validate-bundle` indexes but no `--kind` loads.
3. There is no DnaTreeCalc-emitted replay-facing artifact for this fixture at all
   (DnaTreeCalc publishes normalized-replay views only for W056). The DNA-side
   values exist as host-core test assertions and as the cached `<v>` parts of the
   DNA-saved package; OxReplay cannot read `.xlsx` and must not hand-transcribe
   values into a fixture and call the result a verdict.

An "equivalent" verdict produced through the current path would therefore be
blind to B1. Per the bead, the gap is filed and the verdict is withheld:
`oxreplay-5nn` (this repo: multi-surface loader / multi-cell family),
`dtc-j7n8.21` (DnaTreeCalc: emit the producer artifact from the reopened saved
bytes, both cells), `BLK-REPLAY-005`, and an outbound observation in
`docs/upstream/NOTES_FOR_OXXLPLAY.md`.

## Honest limits carried forward
1. One workbook, two cells, one formula; not campaign-wide Excel fidelity, not bit-exactness.
2. The OxXlPlay driver runs `CalculateFullRebuild()` after open, so Excel's B1 = 30 is a
   recalculation that agrees with DNA's published 30; the cached-30 witness remains the
   package part plus DnaTreeCalc's `save_after_edit_reopens_with_cached_30`.
3. `execution_outcome` compares equal across the two states by design (both `value_published`);
   it does not discriminate values.
