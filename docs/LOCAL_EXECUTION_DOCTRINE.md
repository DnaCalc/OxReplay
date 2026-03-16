# LOCAL_EXECUTION_DOCTRINE.md — OxReplay

## 1. Purpose
Set expectations for local execution and validation while `OxReplay` is still in bootstrap and early implementation mode.

## 2. Baseline rules
1. Prefer file- and CLI-oriented validation over hidden IDE-only workflows.
2. Keep adapter validation and bundle validation runnable without full downstream host integration.
3. Treat lane adapters as external semantic boundaries, not as code-import shortcuts.
4. Record capability impact and retained witness impact when local work changes Replay-governed surfaces.

## 3. Early local validation targets
1. schema or bundle validation,
2. manifest or capability validation,
3. diff or explain query validation,
4. witness lifecycle or quarantine-state validation,
5. adapter loading smoke tests.

## 4. Anti-drift rule
If a local implementation step requires lane-semantic interpretation not already declared by an adapter contract, stop and route it back to the lane repo or Foundation doctrine before proceeding.
