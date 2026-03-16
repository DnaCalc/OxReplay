# W002 — Bundle And Schema Runtime

## Objective
Stand up the first canonical bundle parsing, validation, and indexing surface in `OxReplay`.

## Scope
1. Bundle manifest ingestion.
2. Schema-version validation.
3. Sidecar resolution and indexing.
4. Machine-readable validation output.

## Dependencies
1. `W001`
2. Foundation Replay bundle doctrine.

## Exit gate
1. Bundle validation surface is spec-locked.
2. Indexing expectations are explicit.
3. Capability impact on `PACK.replay.appliance` is stated.
