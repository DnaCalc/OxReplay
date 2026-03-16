# OxReplay Worksets

Worksets are the execution units for `OxReplay`.

## Bootstrap sequence
1. `W001_REPO_BOOTSTRAP_AND_RUNTIME_STRATA.md`
2. `W002_BUNDLE_AND_SCHEMA_RUNTIME.md`
3. `W003_ADAPTER_CAPABILITY_AND_CONFORMANCE_HARNESS.md`
4. `W004_OXCALC_OXFML_ADAPTER_INTAKE_AND_REPLAY_PATH.md`
5. `W005_DNA_RECALC_CLI_SHELL_AND_PACK_EXPORT_BASELINE.md`
6. `W006_WITNESS_DISTILLATION_AND_LIFECYCLE_GOVERNANCE_BASELINE.md`

## Workset rule
Workset files in this directory are the execution packets for `OxReplay`.

Each workset should declare:
1. objective,
2. scope,
3. dependencies,
4. exit gate,
5. expected capability impact,
6. expected pack impact,
7. environment preconditions,
8. evidence layout,
9. replay-corpus readiness,
10. pack-evidence traceability.

If a workset is the first one expected to produce runtime code, its packet must also declare the initial implementation stack explicitly before the workset moves to `in_progress`.
