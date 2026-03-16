# W005 — DNA ReCalc CLI Shell And Pack Export Baseline

## Objective
Provide the first usable `DNA ReCalc` shell over `OxReplay`.

## Scope
1. CLI command family shell.
2. Bundle validate and replay entry points.
3. Diff and explain entry points.
4. Adapter validation entry points.
5. Pack-facing replay export baseline.

## Dependencies
1. `W002`
2. `W003`
3. `W004`

## Exit gate
1. `DNA ReCalc` host role is executable in repo-local terms.
2. Pack-facing output path is explicit.
3. Host remains a replay surface, not a semantics owner.
