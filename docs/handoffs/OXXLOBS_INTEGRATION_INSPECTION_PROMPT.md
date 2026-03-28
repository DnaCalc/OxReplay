# OxXlObs Integration Inspection Prompt

Use this prompt in `OxXlObs` when inspecting the integration seam with `OxReplay`.

```text
You are inspecting the OxXlObs to OxReplay integration seam.

Goal:
Determine whether OxXlObs can emit the right observation artifacts for OxReplay ingestion without pushing Excel-driving logic or semantic ownership into OxReplay.

Read in this order from OxReplay:
1. README.md
2. docs/spec/OXREPLAY_SCOPE_AND_BOUNDARY.md
3. docs/spec/OXREPLAY_BUNDLE_WITNESS_AND_REGISTRY_MODEL.md
4. docs/spec/OXREPLAY_ADAPTER_AND_CONFORMANCE_MODEL.md
5. docs/spec/OXREPLAY_OXXLOBS_OBSERVATION_SEAM.md
6. docs/spec/DNA_RECALC_CLI_CONTRACT.md
7. docs/upstream/NOTES_FOR_OXXLOBS.md
8. docs/handoffs/HANDOFF_OXXLOBS_001_OBSERVATION_SEAM.md

Inspect in OxReplay:
1. the retained bundle and witness roots under docs/test-corpus/
2. the retained run baselines under docs/test-runs/
3. the current dna-recalc command families in src/oxreplay-dnarecalc-cli/

Questions to answer:
1. Can OxXlObs emit direct canonical replay bundles from the first pass, or is a narrower observation-contract phase needed first?
2. Which provenance fields must OxXlObs preserve so OxReplay can replay, diff, explain, and later distill Excel observations honestly?
3. Which fields are directly observed, inferred, unavailable, unstable, or intentionally omitted?
4. What should the first minimal valid OxXlObs fixture family be?
5. What should the first capture-loss or downgraded-instrumentation fixture family be?
6. What should the first comparison-ready scenario against a DNA Calc lane be?
7. What adapter id, lane id, source schema ids, and scenario-id strategy should OxXlObs use?

Required output:
1. a short seam assessment
2. a proposed first artifact family list
3. a proposed provenance field list
4. any contract gaps that still need OxReplay-side clarification
5. a recommended first retained scenario set with stable ids

Hard constraints:
1. Do not make OxReplay the owner of Excel-driving behavior.
2. Do not treat Excel as a semantics-owning lane.
3. Do not hide capture-loss or uncertainty.
4. Keep the first pass observation-first and replay-ready.
```
