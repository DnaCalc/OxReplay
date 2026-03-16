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

## 3. Not this host
`DNA ReCalc` is not:
1. a spreadsheet proving host like `DNA OneCalc` or `DNA TreeCalc`,
2. a new semantics authority,
3. a universal sink for arbitrary logs with no bundle or adapter discipline.

## 4. Initial command families
The initial host should expect to cover:
1. ingest and validate,
2. replay,
3. diff,
4. explain,
5. distill,
6. adapter validation,
7. witness-state or lifecycle operations,
8. pack export.

## 5. UX boundary
1. CLI first.
2. Optional later UI over the same runtime surfaces.
3. Explanations should remain queryable and machine-usable, not only human prose.

## 6. Dependency rule
`DNA ReCalc` depends on `OxReplay`.

It should consume lane behavior through adapters and canonical bundles rather than by linking directly to lane-semantic internals.

## 7. First bootstrap goals
1. Provide a usable CLI shell over bundle validation.
2. Exercise initial `OxCalc` and `OxFml` adapters.
3. Surface typed diffs and causal explanations.
4. Delay broader UI ambitions until the shared runtime is stable.
