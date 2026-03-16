# OXREPLAY_CAPABILITY_AND_PACK_TRACEABILITY.md

## 1. Position
This document maps capability levels and replay-governed packs to the first `OxReplay` worksets, replay classes, and evidence roots.

The mappings below are planning bindings.
They are not capability claims by themselves.

## 2. Capability ladder bindings
| Capability level | Minimum workset floor | Required replay classes | Required evidence roots |
|---|---|---|---|
| `C0.ingest_valid` | `W002`, `W003` | `bundle_manifest_valid`, `manifest_shape_valid` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `C1.replay_valid` | `W004` | `oxcalc_intake`, `oxfml_intake`, `shared_replay` | `docs/test-corpus/bundles/`, lane-import roots, `docs/test-runs/` |
| `C2.diff_valid` | `W004`, `W005` | `shared_diff`, `cli_diff` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `C3.explain_valid` | `W005` | `cli_explain` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `C4.distill_valid` | `W006` | `distill_stable`, `distill_unstable`, `quarantine_required`, `lifecycle_transition` | `docs/test-corpus/witnesses/`, `states/lifecycle/`, `docs/test-runs/` |
| `C5.pack_valid` | successor workset beyond `W006` | pack-specific bound set | pack-specific retained evidence roots |

## 3. Pack bindings
| Pack | Required workset floor | Required replay classes | Minimum retained roots |
|---|---|---|---|
| `PACK.replay.appliance` | `W002` through `W005` | `bundle_manifest_valid`, `bundle_manifest_invalid`, `sidecar_resolution`, `manifest_shape_valid`, `capability_claim_matrix`, `shared_replay`, `cli_validate`, `cli_replay`, `cli_adapter_validate`, `pack_export` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `PACK.diff.cross_engine.continuous` | `W004`, `W005` | `shared_diff`, `cli_diff` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `PACK.trace.forensic_plane` | `W006` | `distill_stable`, `lifecycle_transition` | `docs/test-corpus/witnesses/`, `states/lifecycle/`, `docs/test-runs/` |
| `PACK.reject.calculus` | `W006` when reject replay evidence is in scope | `quarantine_required`, reject-bearing `shared_diff` or host explain scenarios | `docs/test-corpus/witnesses/`, `docs/test-runs/` |

## 4. Report-back rule
Every later completion or status report should be able to point from:
1. a capability level,
2. to the governing workset,
3. to the replay class,
4. to the stable scenario id,
5. to the retained artifact root.

## 5. Conservative claim rule
If any link in that chain is missing, the capability or pack reference remains planning-only.
