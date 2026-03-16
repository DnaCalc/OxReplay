# W003 — Adapter Capability And Conformance Harness

## Objective
Provide shared machinery to validate adapter manifests and capability claims.

## Scope
1. Manifest shape validation.
2. Capability-ladder validation.
3. Registry-version validation.
4. Conformance-result emission for CI and `DNA ReCalc`.

## Dependencies
1. `W001`
2. `W002`
3. Lane adapter expectations from integrated repos.

## Exit gate
1. Manifest contract is local-spec complete.
2. `C0` through `C5` handling is explicit.
3. Runtime does not reinterpret lane semantics.
