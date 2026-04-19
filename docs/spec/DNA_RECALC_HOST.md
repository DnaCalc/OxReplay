# DNA_RECALC_HOST.md

## 1. Position
This document defines the initial repo-local scope of `DNA ReCalc` as the host surface over `OxReplay`.

## 2. Host role
`DNA ReCalc` is the replay appliance host for:
1. bundle ingest and validation,
2. replay execution,
3. diff and explain queries,
4. witness distillation,
5. adapter capability validation,
6. pack-facing replay export and witness lifecycle operations.

It is the generic replay host reference surface for `OxReplay`.

## 3. Not this host
`DNA ReCalc` is not:
1. a spreadsheet proving host like `DNA OneCalc` or `DNA TreeCalc`,
2. a new semantics authority,
3. a universal sink for arbitrary logs with no bundle or adapter discipline.

## 4. Relationship to other hosts
A non-`DNA ReCalc` host such as `DNA OneCalc` may:
1. call `OxReplay` libraries,
2. embed shared runtime strata as in-process dependencies,
3. embed replay, diff, explain, or witness views in its own UI,
4. retain `OxReplay` outputs as part of its own scenario or handoff model,
5. use capability validation and conformance outputs to gate its own product modes.
6. consume `OxReplay` comparison/equivalence outputs without taking final corpus-verdict ownership away from the host.

That does not make it `DNA ReCalc`.

Working rule:
1. `DNA ReCalc` remains the canonical shared replay-host contract and CLI reference surface,
2. `DNA OneCalc` remains a separate proving host that consumes shared replay mechanics,
3. any app-facing `DNA OneCalc` replay UX is a host-local projection over `OxReplay`, not a rewrite of the `DNA ReCalc` host contract,
4. a downstream host must not embed or re-expose the `DNA ReCalc` host shell as a subsystem.

For the detailed non-`DNA ReCalc` downstream-host consumption model, see `docs/spec/OXREPLAY_DNA_ONECALC_CONSUMPTION_MODEL.md`.

## 5. Initial command families
The initial host should expect to cover:
1. ingest and validate,
2. replay,
3. diff,
4. explain,
5. distill,
6. adapter validation,
7. witness-state or lifecycle operations,
8. pack export.

## 6. UX boundary
1. CLI first.
2. Optional later UI over the same runtime surfaces.
3. Explanations should remain queryable and machine-usable, not only human prose.

## 7. Dependency rule
`DNA ReCalc` depends on `OxReplay`.

It should consume lane behavior through adapters and canonical bundles rather than by linking directly to lane-semantic internals.

## 8. First bootstrap goals
1. Provide a usable CLI shell over bundle validation.
2. Exercise initial `OxCalc` and `OxFml` adapters.
3. Surface typed diffs and causal explanations.
4. Keep final host verdict policy separate from shared comparison/equivalence mechanics.
5. Delay broader UI ambitions until the shared runtime is stable.

## 9. Current OxFml seam read
For `OxFml`, `DNA ReCalc` should consume the landed `OxFml_V1` replay projection seam as the host-facing replay intake.

Working rule:
1. ordinary OxFml replay intake should consume the replay projection packet through the `OxFml_V1` seam,
2. `DNA ReCalc` should preserve surfaced OxFml replay metadata rather than collapsing it into a thinner local alias-only projection.
