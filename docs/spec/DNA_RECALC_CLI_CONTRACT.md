# DNA_RECALC_CLI_CONTRACT.md

## 1. Position
This document defines the initial CLI contract for `DNA ReCalc`.

It refines `DNA_RECALC_HOST.md` into an activation-ready command surface for `W005`.

## 2. Command families
The baseline command families are:
1. `validate-bundle`
2. `replay`
3. `diff`
4. `explain`
5. `distill`
6. `validate-adapter`
7. `witness-state`
8. `pack-export`

## 3. Invocation shape
The initial executable name is:
1. `dna-recalc`

General invocation form:
1. `dna-recalc <command> [options]`

## 4. Input model
Common input flags should include:
1. `--bundle <path>`
2. `--adapter <path>`
3. `--kind <kind>` for replay, diff, explain, and distill inputs
4. `--case-id <id>` when a fixture family requires a stable case selection
5. `--predicate-id <id>` and `--predicate-description <text>` for distillation
6. `--format json|text`
7. `--output <path>`
8. `--run-id <id>` when a retained or transient run id is required

## 5. Output model
Rules:
1. `--format json` is the machine-usable baseline
2. `--format text` is human-readable convenience output
3. retained outputs use repo-relative artifact paths when checked in
4. command output should name the active scenario id, bundle id, adapter id, or witness id when applicable

## 6. Exit codes
1. `0` success
2. `1` validation or replay failure with a deterministic machine-readable outcome
3. `2` usage or argument error
4. `3` unsupported capability or incompatible adapter/bundle shape
5. `4` internal error where no deterministic replay-governed result could be produced

## 7. Command-specific baseline expectations
1. `validate-bundle`
   - validates schema compatibility, sidecars, registry refs, and bundle indexing floor
2. `replay`
   - executes deterministic replay over a canonical bundle
3. `diff`
   - compares candidate versus baseline replay surfaces using typed mismatch output
4. `explain`
   - returns causal explanation records for replay, diff, reject, or lifecycle questions
5. `distill`
   - emits predicate-bound reduction manifests and quarantine outcomes without moving distillation into hot-path replay
6. `validate-adapter`
   - validates adapter manifest shape, capability claims, registry refs, and declared limits
7. `witness-state`
   - queries or updates witness lifecycle state through governed transitions only
8. `pack-export`
   - emits pack-facing replay outputs without implying pack readiness unless evidence exists

## 8. Machine-readable floor
The machine-readable floor for baseline commands should include:
1. command id
2. status
3. capability impact
4. pack impact
5. artifact refs
6. scenario ids when relevant
7. typed validation, diff, explain, or lifecycle records

## 9. Non-goals
This pass does not define:
1. a GUI contract
2. free-form interactive debugging UX
3. lane-semantic direct-link shortcuts
