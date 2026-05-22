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
| `shared_view_family_diff` | `crosslane_xml_view_family_gap_001` | `docs/test-corpus/bundles/crosslane_xml_view_family_gap_001/` |
| `shared_view_family_diff_integrated` | `crosslane_xml_view_family_gap_integrated_001` | `docs/test-corpus/bundles/crosslane_xml_view_family_gap_integrated_001/` |
| `shared_view_family_diff_integrated_divergence` | `crosslane_xml_view_family_divergence_integrated_001` | `docs/test-corpus/bundles/crosslane_xml_view_family_divergence_integrated_001/` |

## 5. W005 DNA ReCalc CLI shell
| Replay class | Scenario id | Planned retained root |
|---|---|---|
| `cli_validate` | `host_validate_bundle_001` | `docs/test-corpus/bundles/host_validate_bundle_001/` |
| `cli_replay` | `host_replay_bundle_001` | `docs/test-corpus/bundles/host_replay_bundle_001/` |
| `cli_diff` | `host_diff_bundle_001` | `docs/test-corpus/bundles/host_diff_bundle_001/` |
| `cli_explain` | `host_explain_bundle_001` | `docs/test-corpus/bundles/host_explain_bundle_001/` |
| `cli_explain_view_family` | `host_explain_view_family_gap_001` | `docs/test-corpus/bundles/host_explain_view_family_gap_001/` |
| `cli_explain_view_family_integrated` | `host_explain_view_family_gap_integrated_001` | `docs/test-corpus/bundles/host_explain_view_family_gap_integrated_001/` |
| `cli_explain_view_family_integrated_divergence` | `host_explain_view_family_divergence_integrated_001` | `docs/test-corpus/bundles/host_explain_view_family_divergence_integrated_001/` |
| `cli_adapter_validate` | `host_validate_adapter_001` | `docs/test-corpus/bundles/host_validate_adapter_001/` |
| `pack_export` | `host_pack_export_001` | `docs/test-corpus/bundles/host_pack_export_001/` |

## 6. W006 Witness distillation and lifecycle
| Replay class | Scenario id | Planned retained root |
|---|---|---|
| `distill_stable` | `wit_distill_stable_001` | `docs/test-corpus/witnesses/wit_distill_stable_001/` |
| `distill_unstable` | `wit_distill_unstable_predicate_001` | `docs/test-corpus/witnesses/wit_distill_unstable_predicate_001/` |
| `quarantine_required` | `wit_quarantine_capture_insufficient_001` | `docs/test-corpus/witnesses/wit_quarantine_capture_insufficient_001/` |
| `lifecycle_transition` | `wit_lifecycle_transition_retained_local_001` | `states/lifecycle/wit_lifecycle_transition_retained_local_001/` |

## 7. Reserved and acknowledged OxXlPlay seam classes
| Replay class | Scenario id | Planned retained root |
|---|---|---|
| `xlplay_manifest_shape_valid` | `xlplay_manifest_minimal_valid_001` | reserved until `OxXlPlay` emits a retained manifest fixture |
| `xlplay_observation_bundle_valid` | `xlplay_capture_values_formulae_001` | acknowledged emitted scenario from `../OxXlPlay/states/excel/xlplay_capture_values_formulae_001/` |
| `xlplay_comparison_view_observation_ready` | `xlplay_capture_spreadsheetml_formatting_001` | acknowledged emitted scenario from `../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/` plus local retained seam baseline |
| `xlplay_vba_udf_oracle_first_slice` | `xlplay_vba_udf_addthem_001` | local retained first-slice scenario for `AddThem(Double, Double) As Double` Excel-oracle intake; live OxXlPlay capture will replace the planned-capture metadata when available |
| `xlplay_capture_loss_declared` | `xlplay_capture_loss_formula_unavailable_001` | acknowledged proposed first capture-loss scenario from `OxXlPlay` |
| `xlplay_diff_ready_against_dna` | `xlplay_diff_ready_against_dna_001` | reserved until `OxXlPlay` proposes the first Excel-vs-DNA comparison-ready scenario |
| `xlplay_structured_table_slice_observation_ready` | `xlplay_structured_reference_workbook_001` | acknowledged emitted scenario from `../OxXlPlay/states/excel/xlplay_structured_reference_workbook_001/` plus local retained seam baseline |

## 8. W007 host-rollout evidence planning
| Replay class | Scenario id | Planned retained root |
|---|---|---|
| `host_rollout_per_node_values` | `host_rollout_per_node_values_001` | `docs/test-corpus/bundles/host_rollout_per_node_values_001/` |
| `host_rollout_table_slice` | `host_rollout_table_slice_001` | `docs/test-corpus/bundles/host_rollout_table_slice_001/` |
| `host_rollout_effective_display` | `host_rollout_effective_display_001` | `docs/test-corpus/bundles/host_rollout_effective_display_001/` |
| `host_rollout_execution_outcome` | `host_rollout_execution_outcome_001` | `docs/test-corpus/bundles/host_rollout_execution_outcome_001/` |
| `host_rollout_dependency_invalidation` | `host_rollout_dependency_invalidation_001` | `docs/test-corpus/bundles/host_rollout_dependency_invalidation_001/` |
| `host_rollout_retained_artifacts` | `host_rollout_retained_artifacts_001` | `docs/test-corpus/bundles/host_rollout_retained_artifacts_001/` |

These roots now contain local OxReplay normalized-replay fixtures and retained diff/explain baselines for shared comparison mechanics. They remain producer-integration partial. `OxXlPlay` has published the first real retained `table_slice` artifact for `xlplay_structured_reference_workbook_001`; `DNA TreeCalc`, `DNA OneCalc`, dependency, invalidation, and retained-artifact-ref producer artifacts remain open.

## 9. Naming rule
Scenario ids must remain stable once a retained artifact root is created.

## 10. Widening rule
Later scenario families should:
1. add new ids rather than mutate existing ids silently,
2. record supersession explicitly if a scenario family is replaced,
3. keep workset-to-scenario binding auditable from planning through evidence.
