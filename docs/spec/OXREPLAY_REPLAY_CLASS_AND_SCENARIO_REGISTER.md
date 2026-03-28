# OXREPLAY_REPLAY_CLASS_AND_SCENARIO_REGISTER.md

## 1. Position
This document is the planning register for the first replay classes and scenario ids used by `OxReplay` worksets.

It assigns stable scenario ids before harness work starts.
The ids below are planning commitments until the retained artifacts exist.

## 2. W002 Bundle and schema runtime
| Replay class | Scenario id | Planned retained root |
|---|---|---|
| `bundle_manifest_valid` | `rb_manifest_minimal_valid_001` | `docs/test-corpus/bundles/rb_manifest_minimal_valid_001/` |
| `bundle_manifest_invalid` | `rb_manifest_schema_invalid_001` | `docs/test-corpus/bundles/rb_manifest_schema_invalid_001/` |
| `sidecar_resolution` | `rb_sidecar_resolution_roundtrip_001` | `docs/test-corpus/bundles/rb_sidecar_resolution_roundtrip_001/` |
| `bundle_indexing` | `rb_bundle_index_projection_001` | `docs/test-corpus/bundles/rb_bundle_index_projection_001/` |

## 3. W003 Adapter capability and conformance
| Replay class | Scenario id | Planned retained root |
|---|---|---|
| `manifest_shape_valid` | `cap_manifest_minimal_valid_001` | `docs/test-corpus/bundles/cap_manifest_minimal_valid_001/` |
| `manifest_shape_invalid` | `cap_manifest_schema_invalid_001` | `docs/test-corpus/bundles/cap_manifest_schema_invalid_001/` |
| `capability_claim_matrix` | `cap_claim_matrix_floor_001` | `docs/test-corpus/bundles/cap_claim_matrix_floor_001/` |
| `registry_version_match` | `cap_registry_version_mismatch_001` | `docs/test-corpus/bundles/cap_registry_version_mismatch_001/` |

## 4. W004 OxCalc and OxFml adapter intake
| Replay class | Scenario id | Planned retained root |
|---|---|---|
| `oxcalc_intake` | `oxcalc_tracecalc_accept_publish_001` | sibling import from `../OxCalc` plus local retained intake note |
| `oxfml_intake` | `oxfml_fec_accept_publication_001` | sibling import from `../OxFml` plus local retained intake note |
| `shared_replay` | `crosslane_replay_identity_001` | `docs/test-corpus/bundles/crosslane_replay_identity_001/` |
| `shared_diff` | `crosslane_diff_mismatch_001` | `docs/test-corpus/bundles/crosslane_diff_mismatch_001/` |

## 5. W005 DNA ReCalc CLI shell
| Replay class | Scenario id | Planned retained root |
|---|---|---|
| `cli_validate` | `host_validate_bundle_001` | `docs/test-corpus/bundles/host_validate_bundle_001/` |
| `cli_replay` | `host_replay_bundle_001` | `docs/test-corpus/bundles/host_replay_bundle_001/` |
| `cli_diff` | `host_diff_bundle_001` | `docs/test-corpus/bundles/host_diff_bundle_001/` |
| `cli_explain` | `host_explain_bundle_001` | `docs/test-corpus/bundles/host_explain_bundle_001/` |
| `cli_adapter_validate` | `host_validate_adapter_001` | `docs/test-corpus/bundles/host_validate_adapter_001/` |
| `pack_export` | `host_pack_export_001` | `docs/test-corpus/bundles/host_pack_export_001/` |

## 6. W006 Witness distillation and lifecycle
| Replay class | Scenario id | Planned retained root |
|---|---|---|
| `distill_stable` | `wit_distill_stable_001` | `docs/test-corpus/witnesses/wit_distill_stable_001/` |
| `distill_unstable` | `wit_distill_unstable_predicate_001` | `docs/test-corpus/witnesses/wit_distill_unstable_predicate_001/` |
| `quarantine_required` | `wit_quarantine_capture_insufficient_001` | `docs/test-corpus/witnesses/wit_quarantine_capture_insufficient_001/` |
| `lifecycle_transition` | `wit_lifecycle_transition_retained_local_001` | `states/lifecycle/wit_lifecycle_transition_retained_local_001/` |

## 7. Reserved OxXlObs seam classes
| Replay class | Scenario id | Planned retained root |
|---|---|---|
| `xlobs_manifest_shape_valid` | `xlobs_manifest_minimal_valid_001` | reserved until `OxXlObs` emits a retained manifest fixture |
| `xlobs_observation_bundle_valid` | `xlobs_capture_values_formulae_001` | acknowledged emitted scenario from `../OxXlObs/states/excel/xlobs_capture_values_formulae_001/` |
| `xlobs_capture_loss_declared` | `xlobs_capture_loss_formula_unavailable_001` | acknowledged proposed first capture-loss scenario from `OxXlObs` |
| `xlobs_diff_ready_against_dna` | `xlobs_diff_ready_against_dna_001` | reserved until `OxXlObs` proposes the first Excel-vs-DNA comparison-ready scenario |

## 8. Naming rule
Scenario ids must remain stable once a retained artifact root is created.

## 9. Widening rule
Later scenario families should:
1. add new ids rather than mutate existing ids silently,
2. record supersession explicitly if a scenario family is replaced,
3. keep workset-to-scenario binding auditable from planning through evidence.
