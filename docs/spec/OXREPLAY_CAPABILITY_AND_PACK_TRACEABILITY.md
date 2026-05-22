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
| `C2.diff_valid` | `W004`, `W005` | `shared_diff`, `shared_view_family_diff`, `shared_view_family_diff_integrated`, `shared_view_family_diff_integrated_divergence`, `cli_diff` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `C3.explain_valid` | `W005` | `cli_explain`, `cli_explain_view_family`, `cli_explain_view_family_integrated`, `cli_explain_view_family_integrated_divergence` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| planned `C2`/`C3` host-rollout widening | `W007` | `host_rollout_per_node_values`, `host_rollout_table_slice`, `host_rollout_effective_display`, `host_rollout_execution_outcome`, `host_rollout_dependency_invalidation`, `host_rollout_retained_artifacts` | planned `docs/test-corpus/bundles/`, planned `docs/test-runs/` |
| `C4.distill_valid` | `W006` | `distill_stable`, `distill_unstable`, `quarantine_required`, `lifecycle_transition` | `docs/test-corpus/witnesses/`, `states/lifecycle/`, `docs/test-runs/` |
| `C5.pack_valid` | successor workset beyond `W006` | pack-specific bound set | pack-specific retained evidence roots |

## 3. Pack bindings
| Pack | Required workset floor | Required replay classes | Minimum retained roots |
|---|---|---|---|
| `PACK.replay.appliance` | `W002` through `W005` | `bundle_manifest_valid`, `bundle_manifest_invalid`, `sidecar_resolution`, `manifest_shape_valid`, `capability_claim_matrix`, `shared_replay`, `shared_view_family_diff`, `shared_view_family_diff_integrated`, `shared_view_family_diff_integrated_divergence`, `cli_validate`, `cli_replay`, `cli_adapter_validate`, `pack_export` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `PACK.diff.cross_engine.continuous` | `W004`, `W005` | `shared_diff`, `shared_view_family_diff`, `shared_view_family_diff_integrated`, `shared_view_family_diff_integrated_divergence`, `cli_diff`, `cli_explain_view_family`, `cli_explain_view_family_integrated`, `cli_explain_view_family_integrated_divergence` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `PACK.trace.forensic_plane` | `W006` | `distill_stable`, `lifecycle_transition` | `docs/test-corpus/witnesses/`, `states/lifecycle/`, `docs/test-runs/` |
| `PACK.reject.calculus` | `W006` when reject replay evidence is in scope | `quarantine_required`, reject-bearing `shared_diff` or host explain scenarios | `docs/test-corpus/witnesses/`, `docs/test-runs/` |
| `PACK.replay.appliance` | `W007` | `host_rollout_per_node_values`, `host_rollout_table_slice`, `host_rollout_effective_display`, `host_rollout_execution_outcome`, `host_rollout_dependency_invalidation`, `host_rollout_retained_artifacts` | planned `docs/test-corpus/bundles/`, planned `docs/test-runs/` |
| `PACK.diff.cross_engine.continuous` | `W007` | `host_rollout_per_node_values`, `host_rollout_table_slice`, `host_rollout_effective_display`, `host_rollout_execution_outcome`, `host_rollout_dependency_invalidation` | planned `docs/test-corpus/bundles/`, planned `docs/test-runs/` |
| `PACK.trace.forensic_plane` | `W007` | `host_rollout_dependency_invalidation`, `host_rollout_retained_artifacts` | planned `docs/test-corpus/bundles/`, planned `docs/test-runs/` |

## 4. Report-back rule
Every later completion or status report should be able to point from:
1. a capability level,
2. to the governing workset,
3. to the replay class,
4. to the stable scenario id,
5. to the retained artifact root.

## 5. Conservative claim rule
If any link in that chain is missing, the capability or pack reference remains planning-only.

## 6. W007 Planning Rule
1. The `W007` host-rollout rows are planning bindings only.
2. Planned comparison surfaces must arrive as typed replay evidence families, not as TreeCalc-private or OneCalc-private helper objects.
3. Planned families include `per_node_value`, `table_slice`, `effective_display_text`, `execution_outcome`, `dependency_evidence`, `invalidation_evidence`, and retained host artifact refs.
4. Final product verdict policy remains host-owned even when `OxReplay` compares the declared typed surfaces.
5. Current local W007 fixtures exercise shared comparison mechanics for those families, but they do not upgrade any producer capability floor until real upstream retained artifacts are emitted and replayed.
