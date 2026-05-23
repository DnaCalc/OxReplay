#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use oxreplay_abstractions::{LaneId, RegistryRef};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub event_id: String,
    pub source_label: String,
    pub normalized_family: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayScenario {
    pub scenario_id: String,
    pub lane_id: LaneId,
    pub events: Vec<ReplayEvent>,
    pub registry_refs: Vec<RegistryRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comparison_views: Vec<ReplayComparisonView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayComparisonView {
    pub view_family: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayComparisonContract {
    pub comparison_family: String,
    pub equivalence_policy_id: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedOutcomeValue {
    pub outcome_kind: String,
    pub outcome_stage: String,
    pub class_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lane_reason_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub human_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_detail: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayRenderContext {
    pub context_id: String,
    pub context_kind: String,
    pub locale_tag: String,
    pub decimal_separator: String,
    pub thousands_separator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_separator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRenderContext {
    pub ref_id: Option<String>,
    pub context: ReplayRenderContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderContextResolution {
    Absent,
    Resolved(ResolvedRenderContext),
    Untrusted { reason: String },
}

impl RenderContextResolution {
    pub fn status_summary(&self) -> String {
        match self {
            Self::Absent => "render_context=absent".to_string(),
            Self::Resolved(resolved) => {
                let source = match resolved.ref_id.as_deref() {
                    Some(ref_id) => format!("ref=`{ref_id}`"),
                    None => "inline".to_string(),
                };
                let trust_class = resolved.context.trust_class.as_deref().unwrap_or("unknown");
                format!(
                    "render_context=resolved({source}, locale_tag=`{}`, decimal_separator=`{}`, thousands_separator=`{}`, trust_class=`{}`)",
                    resolved.context.locale_tag,
                    resolved.context.decimal_separator,
                    resolved.context.thousands_separator,
                    trust_class,
                )
            }
            Self::Untrusted { reason } => {
                format!("render_context=untrusted({reason})")
            }
        }
    }
}

impl ReplayComparisonView {
    pub fn normalized_family(&self) -> &str {
        normalized_comparison_view_family(&self.view_family)
    }

    pub fn comparison_contract(&self) -> ReplayComparisonContract {
        ReplayComparisonContract {
            comparison_family: self.normalized_family().to_string(),
            equivalence_policy_id: default_equivalence_policy_id(&self.view_family).to_string(),
            required: comparison_view_required(&self.view_family),
        }
    }
}

impl ReplayScenario {
    pub fn comparison_view_map(&self) -> BTreeMap<&str, &serde_json::Value> {
        self.comparison_views
            .iter()
            .map(|view| (view.view_family.as_str(), &view.value))
            .collect()
    }

    pub fn normalized_comparison_view_map(&self) -> BTreeMap<String, &ReplayComparisonView> {
        self.comparison_views
            .iter()
            .map(|view| (view.normalized_family().to_string(), view))
            .collect()
    }

    pub fn resolve_render_context(&self) -> RenderContextResolution {
        let Some(source_metadata) = self
            .source_metadata
            .as_ref()
            .and_then(serde_json::Value::as_object)
        else {
            return RenderContextResolution::Absent;
        };

        let inline_context = source_metadata.get("render_context");
        let context_ref = source_metadata.get("render_context_ref");
        match (inline_context, context_ref) {
            (Some(_), Some(_)) => RenderContextResolution::Untrusted {
                reason: "both inline `render_context` and `render_context_ref` are present"
                    .to_string(),
            },
            (Some(inline_context), None) => parse_render_context(inline_context)
                .map(|context| {
                    RenderContextResolution::Resolved(ResolvedRenderContext {
                        ref_id: None,
                        context,
                    })
                })
                .unwrap_or_else(|reason| RenderContextResolution::Untrusted { reason }),
            (None, Some(context_ref)) => resolve_render_context_ref(source_metadata, context_ref),
            (None, None) => RenderContextResolution::Absent,
        }
    }
}

pub fn normalized_comparison_view_family(view_family: &str) -> &str {
    match view_family {
        "comparison_value" | "worksheet_comparison_value" => "worksheet_comparison_value",
        "retained_host_artifact_ref" | "host_artifact_ref" => "retained_artifact_ref",
        _ => view_family,
    }
}

pub fn default_equivalence_policy_id(view_family: &str) -> &'static str {
    match normalized_comparison_view_family(view_family) {
        "worksheet_comparison_value" => "worksheet_value_exact",
        "per_node_value" => "per_node_value_json_exact",
        "table_slice" => "table_slice_json_exact",
        "table_update_oracle" => "table_update_oracle_json_exact",
        "effective_display_text" => "effective_display_text_exact",
        "visible_value_text" => "visible_value_text_exact",
        "execution_outcome" => "typed_outcome_class",
        "dependency_evidence" => "dependency_evidence_json_exact",
        "invalidation_evidence" => "invalidation_evidence_json_exact",
        "retained_artifact_ref" => "retained_artifact_ref_json_exact",
        _ => "view_json_exact",
    }
}

pub fn comparison_view_required(view_family: &str) -> bool {
    !matches!(
        normalized_comparison_view_family(view_family),
        "visible_value_text"
    )
}

fn resolve_render_context_ref(
    source_metadata: &serde_json::Map<String, serde_json::Value>,
    context_ref: &serde_json::Value,
) -> RenderContextResolution {
    let Some(ref_id) = context_ref.as_str() else {
        return RenderContextResolution::Untrusted {
            reason: "`render_context_ref` must be a string".to_string(),
        };
    };

    let Some(render_contexts) = source_metadata
        .get("render_contexts")
        .and_then(serde_json::Value::as_object)
    else {
        return RenderContextResolution::Untrusted {
            reason: format!(
                "`render_context_ref` `{ref_id}` did not resolve because `render_contexts` is missing"
            ),
        };
    };

    let Some(target) = render_contexts.get(ref_id) else {
        return RenderContextResolution::Untrusted {
            reason: format!("`render_context_ref` `{ref_id}` did not resolve"),
        };
    };

    if target
        .as_object()
        .is_some_and(|target| target.contains_key("render_context_ref"))
    {
        return RenderContextResolution::Untrusted {
            reason: format!(
                "`render_context_ref` `{ref_id}` points to another `render_context_ref`; one-hop resolution only"
            ),
        };
    }

    parse_render_context(target)
        .map(|context| {
            RenderContextResolution::Resolved(ResolvedRenderContext {
                ref_id: Some(ref_id.to_string()),
                context,
            })
        })
        .unwrap_or_else(|reason| RenderContextResolution::Untrusted { reason })
}

fn parse_render_context(value: &serde_json::Value) -> Result<ReplayRenderContext, String> {
    let context: ReplayRenderContext = serde_json::from_value(value.clone())
        .map_err(|error| format!("render_context is outside the admitted local seam: {error}"))?;

    if context.context_kind != "excel_render_context" {
        return Err(format!(
            "render_context declares unsupported context_kind `{}`",
            context.context_kind
        ));
    }

    Ok(context)
}

pub fn is_replay_ready(scenario: &ReplayScenario) -> bool {
    !scenario.scenario_id.trim().is_empty() && !scenario.events.is_empty()
}

#[derive(Debug, Error)]
pub enum ReplayScenarioLoadError {
    #[error("failed to read scenario source from `{path}`: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse scenario source from `{path}`: {source}")]
    Parse {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

#[derive(Debug, Deserialize)]
struct OxCalcTraceCalcScenario {
    scenario_id: String,
    expected: OxCalcExpected,
}

#[derive(Debug, Deserialize)]
struct OxCalcExpected {
    trace_labels: Vec<OxCalcTraceLabelCount>,
}

#[derive(Debug, Deserialize)]
struct OxCalcTraceLabelCount {
    label: String,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct OxFmlV1ReplayProjectionResult {
    source_artifact_family: String,
    #[serde(default)]
    source_case_id: Option<String>,
    #[serde(default)]
    source_case_ids: Vec<String>,
    #[serde(default)]
    shared_scenario_alias: Option<String>,
    formula_stable_id: String,
    #[serde(default)]
    session_id: Option<String>,
    #[serde(default)]
    witness_id: Option<String>,
    #[serde(default)]
    phase: Option<String>,
    #[serde(default)]
    commit_decision_kind: Option<String>,
    #[serde(default)]
    trace_event_kinds: Vec<String>,
}

pub fn load_oxcalc_tracecalc_projection(
    path: impl AsRef<Path>,
) -> Result<ReplayScenario, ReplayScenarioLoadError> {
    let path = path.as_ref();
    let path_text = path.display().to_string();
    let source = fs::read_to_string(path).map_err(|source| ReplayScenarioLoadError::Read {
        path: path_text.clone(),
        source,
    })?;
    let scenario: OxCalcTraceCalcScenario =
        serde_json::from_str(&source).map_err(|source| ReplayScenarioLoadError::Parse {
            path: path_text,
            source,
        })?;

    let mut events = Vec::new();
    for label in scenario.expected.trace_labels {
        for occurrence in 0..label.count {
            events.push(ReplayEvent {
                event_id: format!("{}-{:02}", label.label, occurrence + 1),
                source_label: label.label.clone(),
                normalized_family: normalize_oxcalc_label(&label.label).to_string(),
            });
        }
    }

    Ok(ReplayScenario {
        scenario_id: scenario.scenario_id,
        lane_id: LaneId("oxcalc".to_string()),
        events,
        registry_refs: vec![],
        comparison_views: vec![],
        source_metadata: None,
    })
}

pub fn load_replay_scenario_from_path(
    path: impl AsRef<Path>,
) -> Result<ReplayScenario, ReplayScenarioLoadError> {
    let path = path.as_ref();
    let path_text = path.display().to_string();
    let source = fs::read_to_string(path).map_err(|source| ReplayScenarioLoadError::Read {
        path: path_text.clone(),
        source,
    })?;

    serde_json::from_str(&source).map_err(|source| ReplayScenarioLoadError::Parse {
        path: path_text,
        source,
    })
}

pub fn load_oxfml_v1_replay_projection(
    path: impl AsRef<Path>,
) -> Result<ReplayScenario, ReplayScenarioLoadError> {
    let path = path.as_ref();
    let path_text = path.display().to_string();
    let source = fs::read_to_string(path).map_err(|source| ReplayScenarioLoadError::Read {
        path: path_text.clone(),
        source,
    })?;
    let raw_projection: serde_json::Value =
        serde_json::from_str(&source).map_err(|source| ReplayScenarioLoadError::Parse {
            path: path_text.clone(),
            source,
        })?;
    let projection: OxFmlV1ReplayProjectionResult = serde_json::from_value(raw_projection.clone())
        .map_err(|source| ReplayScenarioLoadError::Parse {
            path: path_text.clone(),
            source,
        })?;
    let scenario_id = select_oxfml_v1_scenario_id(&projection);
    let events = project_oxfml_v1_events(&scenario_id, &projection);
    let comparison_views = project_comparison_views(
        raw_projection
            .get("comparison_views")
            .cloned()
            .unwrap_or(serde_json::Value::Null),
    )
    .map_err(|source| ReplayScenarioLoadError::Parse {
        path: path_text,
        source,
    })?;

    Ok(ReplayScenario {
        scenario_id,
        lane_id: LaneId("oxfml".to_string()),
        events,
        registry_refs: vec![],
        comparison_views,
        source_metadata: Some(raw_projection),
    })
}

fn normalize_oxcalc_label(label: &str) -> &'static str {
    match label {
        "candidate_admitted" => "candidate.admitted",
        "candidate_recorded" | "candidate_emitted" => "candidate.built",
        "publication_committed" | "candidate_published" => "publication.committed",
        "candidate_rejected" => "reject.issued",
        _ => "oxcalc.local.unmapped",
    }
}

fn select_oxfml_v1_scenario_id(projection: &OxFmlV1ReplayProjectionResult) -> String {
    projection
        .shared_scenario_alias
        .clone()
        .or_else(|| projection.source_case_id.clone())
        .or_else(|| projection.source_case_ids.first().cloned())
        .or_else(|| projection.witness_id.clone())
        .or_else(|| {
            projection
                .session_id
                .as_ref()
                .map(|session_id| format!("oxfml.session.{session_id}"))
        })
        .unwrap_or_else(|| {
            format!(
                "oxfml.{}.{}",
                projection.source_artifact_family, projection.formula_stable_id
            )
        })
}

fn project_oxfml_v1_events(
    scenario_id: &str,
    projection: &OxFmlV1ReplayProjectionResult,
) -> Vec<ReplayEvent> {
    let mut events = Vec::new();

    if let Some(commit_decision_kind) = projection.commit_decision_kind.as_deref() {
        push_projected_event(
            &mut events,
            scenario_id,
            commit_decision_kind,
            normalize_oxfml_v1_decision(commit_decision_kind),
        );
    }

    if let Some(phase) = projection.phase.as_deref() {
        push_projected_event(
            &mut events,
            scenario_id,
            phase,
            normalize_oxfml_v1_phase(phase, projection.commit_decision_kind.as_deref()),
        );
    }

    for trace_event_kind in &projection.trace_event_kinds {
        push_projected_event(
            &mut events,
            scenario_id,
            trace_event_kind,
            normalize_oxfml_v1_trace_event(trace_event_kind),
        );
    }

    events
}

fn push_projected_event(
    events: &mut Vec<ReplayEvent>,
    scenario_id: &str,
    source_label: &str,
    normalized_family: &str,
) {
    if events
        .iter()
        .any(|event| event.normalized_family == normalized_family)
    {
        return;
    }

    events.push(ReplayEvent {
        event_id: format!("{scenario_id}-{:02}", events.len() + 1),
        source_label: source_label.to_string(),
        normalized_family: normalized_family.to_string(),
    });
}

fn normalize_oxfml_v1_decision(commit_decision_kind: &str) -> &'static str {
    match commit_decision_kind.to_ascii_lowercase().as_str() {
        "accepted" => "candidate.accepted",
        "rejected" => "reject.issued",
        _ => "oxfml.decision.unmapped",
    }
}

fn normalize_oxfml_v1_phase(phase: &str, commit_decision_kind: Option<&str>) -> &'static str {
    match phase.to_ascii_lowercase().as_str() {
        "open" => "session.opened",
        "executed" | "capabilityviewestablished" => "candidate.built",
        "committed" | "commitattempted" => "publication.committed",
        "committedorrejected" => match commit_decision_kind.map(str::to_ascii_lowercase) {
            Some(decision) if decision == "rejected" => "reject.issued",
            _ => "publication.committed",
        },
        "rejected" | "aborted" | "expired" | "terminated" => "session.terminated",
        _ => "oxfml.phase.unmapped",
    }
}

fn normalize_oxfml_v1_trace_event(trace_event_kind: &str) -> &'static str {
    let trace_event_kind = trace_event_kind.to_ascii_lowercase();

    if trace_event_kind.contains("spill") {
        "spill.observed"
    } else if trace_event_kind.contains("payload") {
        "publication.payload"
    } else if trace_event_kind.contains("reject") {
        "reject.issued"
    } else if trace_event_kind.contains("publish") || trace_event_kind.contains("commit") {
        "publication.committed"
    } else if trace_event_kind.contains("accepted") && trace_event_kind.contains("candidate") {
        "candidate.accepted"
    } else if trace_event_kind.contains("candidate")
        || trace_event_kind.contains("execut")
        || trace_event_kind.contains("capabilityviewestablished")
    {
        "candidate.built"
    } else if trace_event_kind.contains("open") {
        "session.opened"
    } else if trace_event_kind.contains("abort")
        || trace_event_kind.contains("expire")
        || trace_event_kind.contains("terminat")
    {
        "session.terminated"
    } else {
        "oxfml.trace.unmapped"
    }
}

fn project_comparison_views(
    raw_value: serde_json::Value,
) -> Result<Vec<ReplayComparisonView>, serde_json::Error> {
    if raw_value.is_null() {
        return Ok(Vec::new());
    }

    serde_json::from_value(raw_value)
}

#[cfg(test)]
mod tests {
    use super::{
        RenderContextResolution, ReplayComparisonView, ReplayEvent, ReplayScenario,
        comparison_view_required, default_equivalence_policy_id, is_replay_ready,
        load_oxcalc_tracecalc_projection, load_oxfml_v1_replay_projection,
        load_replay_scenario_from_path, normalized_comparison_view_family,
    };
    use oxreplay_abstractions::LaneId;
    use std::path::PathBuf;

    fn scenario_with_source_metadata(source_metadata: serde_json::Value) -> ReplayScenario {
        ReplayScenario {
            scenario_id: "scenario".to_string(),
            lane_id: LaneId("test".to_string()),
            events: vec![ReplayEvent {
                event_id: "scenario-01".to_string(),
                source_label: "event".to_string(),
                normalized_family: "candidate.built".to_string(),
            }],
            registry_refs: vec![],
            comparison_views: vec![],
            source_metadata: Some(source_metadata),
        }
    }

    #[test]
    fn projects_real_oxcalc_tracecalc_case() {
        let scenario = load_oxcalc_tracecalc_projection(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../OxCalc/docs/test-corpus/core-engine/tracecalc/hand-auditable/tc_accept_publish_001.json"))
        .expect("oxcalc fixture should load");

        assert_eq!(scenario.scenario_id, "tc_accept_publish_001");
        assert!(is_replay_ready(&scenario));
        assert_eq!(scenario.events.len(), 3);
    }

    #[test]
    fn loads_oxfml_v1_replay_projection_fixture() {
        let scenario =
            load_oxfml_v1_replay_projection(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
                "../../docs/test-corpus/bundles/oxfml_v1_replay_projection_001/projection.json",
            ))
            .expect("oxfml v1 projection should load");

        assert_eq!(scenario.scenario_id, "oxfml_fec_accept_publication_001");
        assert!(is_replay_ready(&scenario));
        assert_eq!(
            scenario
                .source_metadata
                .as_ref()
                .and_then(|value| value.get("source_case_id")),
            Some(&serde_json::Value::String("fec_001_accept".to_string()))
        );
        assert_eq!(
            scenario
                .source_metadata
                .as_ref()
                .and_then(|value| value.get("shared_scenario_alias")),
            Some(&serde_json::Value::String(
                "oxfml_fec_accept_publication_001".to_string()
            ))
        );
        assert_eq!(
            scenario
                .source_metadata
                .as_ref()
                .and_then(|value| value.get("registry_pin")),
            Some(&serde_json::Value::String(
                "reg-oxfml-2026-04-01".to_string()
            ))
        );
        assert_eq!(
            scenario
                .source_metadata
                .as_ref()
                .and_then(|value| value.get("library_context_snapshot_ref"))
                .and_then(|value| value.get("snapshot_id")),
            Some(&serde_json::Value::String("ctx-main".to_string()))
        );
        assert_eq!(
            scenario
                .events
                .iter()
                .map(|event| event.normalized_family.as_str())
                .collect::<Vec<_>>(),
            vec![
                "candidate.accepted",
                "publication.committed",
                "publication.payload",
            ]
        );
        assert!(scenario.comparison_views.is_empty());
    }

    #[test]
    fn loads_oxfml_v1_session_lifecycle_projection_fixture() {
        let scenario = load_oxfml_v1_replay_projection(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
                "../../docs/test-corpus/bundles/oxfml_v1_session_lifecycle_projection_001/projection.json",
            ),
        )
        .expect("oxfml v1 session lifecycle projection should load");

        assert_eq!(scenario.scenario_id, "oxfml_session_lifecycle_001");
        assert!(is_replay_ready(&scenario));
        assert_eq!(
            scenario
                .events
                .iter()
                .map(|event| event.normalized_family.as_str())
                .collect::<Vec<_>>(),
            vec![
                "candidate.accepted",
                "publication.committed",
                "session.opened",
                "candidate.built",
            ]
        );
        assert!(scenario.comparison_views.is_empty());
    }

    #[test]
    fn loads_oxfml_v1_xml_verification_comparison_views_projection_fixture() {
        let scenario = load_oxfml_v1_replay_projection(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
                "../../docs/test-corpus/bundles/oxfml_v1_xml_verification_comparison_views_projection_001/projection.json",
            ),
        )
        .expect("oxfml xml verification projection should load");

        assert_eq!(
            scenario.scenario_id,
            "oxfml_xml_verification_comparison_views_001"
        );
        assert_eq!(scenario.comparison_views.len(), 4);
        assert_eq!(
            scenario.comparison_views[1],
            ReplayComparisonView {
                view_family: "effective_display_text".to_string(),
                value: serde_json::Value::String("$3.00".to_string()),
            }
        );
    }

    #[test]
    fn loads_normalized_replay_scenario_fixture() {
        let scenario =
            load_replay_scenario_from_path(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
                "../../docs/test-corpus/bundles/crosslane_replay_identity_001/left.replay.json",
            ))
            .expect("normalized fixture should load");

        assert_eq!(scenario.scenario_id, "crosslane_replay_identity_001_left");
        assert!(is_replay_ready(&scenario));
        assert_eq!(scenario.events.len(), 2);
        assert!(scenario.comparison_views.is_empty());
        assert!(scenario.source_metadata.is_none());
    }

    #[test]
    fn loads_normalized_replay_comparison_views() {
        let scenario =
            load_replay_scenario_from_path(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
                "../../docs/test-corpus/bundles/crosslane_xml_view_family_gap_001/left.replay.json",
            ))
            .expect("normalized fixture should load");

        assert_eq!(
            scenario.comparison_views,
            vec![
                ReplayComparisonView {
                    view_family: "comparison_value".to_string(),
                    value: serde_json::json!({
                        "value_kind": "number",
                        "worksheet_value_class": "scalar",
                        "payload": "6"
                    }),
                },
                ReplayComparisonView {
                    view_family: "effective_display_text".to_string(),
                    value: serde_json::Value::String("6".to_string()),
                },
            ]
        );
    }

    #[test]
    fn loads_xlplay_vba_udf_addthem_oracle_fixture() {
        let scenario =
            load_replay_scenario_from_path(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
                "../../docs/test-corpus/bundles/xlplay_vba_udf_addthem_001/scenario.replay.json",
            ))
            .expect("xlplay vba udf fixture should load");

        assert_eq!(scenario.scenario_id, "xlplay_vba_udf_addthem_001");
        assert_eq!(scenario.lane_id.0, "oxxlplay");
        assert!(is_replay_ready(&scenario));
        assert_eq!(
            scenario
                .normalized_comparison_view_map()
                .get("worksheet_comparison_value")
                .expect("comparison value")
                .value,
            serde_json::json!({
                "kind": "number",
                "value": 5.0
            })
        );
        assert_eq!(
            scenario
                .source_metadata
                .as_ref()
                .and_then(|metadata| metadata.get("udf_admission"))
                .and_then(|admission| admission.get("return_type")),
            Some(&serde_json::json!("Double"))
        );
    }

    #[test]
    fn normalizes_comparison_family_contracts_for_value_and_outcome_views() {
        assert_eq!(
            normalized_comparison_view_family("comparison_value"),
            "worksheet_comparison_value"
        );
        assert_eq!(
            default_equivalence_policy_id("comparison_value"),
            "worksheet_value_exact"
        );
        assert!(comparison_view_required("comparison_value"));

        assert_eq!(
            normalized_comparison_view_family("execution_outcome"),
            "execution_outcome"
        );
        assert_eq!(
            normalized_comparison_view_family("retained_host_artifact_ref"),
            "retained_artifact_ref"
        );
        assert_eq!(
            normalized_comparison_view_family("authoring_outcome"),
            "authoring_outcome"
        );
        assert_eq!(
            default_equivalence_policy_id("per_node_value"),
            "per_node_value_json_exact"
        );
        assert_eq!(
            default_equivalence_policy_id("table_slice"),
            "table_slice_json_exact"
        );
        assert_eq!(
            default_equivalence_policy_id("table_update_oracle"),
            "table_update_oracle_json_exact"
        );
        assert_eq!(
            default_equivalence_policy_id("dependency_evidence"),
            "dependency_evidence_json_exact"
        );
        assert_eq!(
            default_equivalence_policy_id("invalidation_evidence"),
            "invalidation_evidence_json_exact"
        );
        assert_eq!(
            default_equivalence_policy_id("retained_host_artifact_ref"),
            "retained_artifact_ref_json_exact"
        );
        assert_eq!(
            default_equivalence_policy_id("execution_outcome"),
            "typed_outcome_class"
        );
        assert!(comparison_view_required("execution_outcome"));
        assert!(!comparison_view_required("visible_value_text"));
    }

    #[test]
    fn resolves_inline_render_context() {
        let scenario = scenario_with_source_metadata(serde_json::json!({
            "render_context": {
                "context_id": "ctx-inline",
                "context_kind": "excel_render_context",
                "locale_tag": "nl-NL",
                "decimal_separator": ",",
                "thousands_separator": ".",
                "trust_class": "unpinned"
            }
        }));

        let resolution = scenario.resolve_render_context();

        match resolution {
            RenderContextResolution::Resolved(resolved) => {
                assert_eq!(resolved.ref_id, None);
                assert_eq!(resolved.context.context_id, "ctx-inline");
                assert_eq!(resolved.context.locale_tag, "nl-NL");
            }
            other => panic!("expected resolved inline render context, got {other:?}"),
        }
    }

    #[test]
    fn resolves_one_hop_render_context_ref() {
        let scenario = scenario_with_source_metadata(serde_json::json!({
            "render_context_ref": "ctx-shared",
            "render_contexts": {
                "ctx-shared": {
                    "context_id": "ctx-shared",
                    "context_kind": "excel_render_context",
                    "locale_tag": "en-US",
                    "decimal_separator": ".",
                    "thousands_separator": ",",
                    "trust_class": "direct"
                }
            }
        }));

        let resolution = scenario.resolve_render_context();

        match resolution {
            RenderContextResolution::Resolved(resolved) => {
                assert_eq!(resolved.ref_id.as_deref(), Some("ctx-shared"));
                assert_eq!(resolved.context.locale_tag, "en-US");
            }
            other => panic!("expected resolved ref render context, got {other:?}"),
        }
    }

    #[test]
    fn marks_missing_render_context_ref_as_untrusted() {
        let scenario = scenario_with_source_metadata(serde_json::json!({
            "render_context_ref": "ctx-missing",
            "render_contexts": {}
        }));

        let resolution = scenario.resolve_render_context();

        assert_eq!(
            resolution,
            RenderContextResolution::Untrusted {
                reason: "`render_context_ref` `ctx-missing` did not resolve".to_string()
            }
        );
    }

    #[test]
    fn rejects_render_context_ref_to_ref_as_untrusted() {
        let scenario = scenario_with_source_metadata(serde_json::json!({
            "render_context_ref": "ctx-a",
            "render_contexts": {
                "ctx-a": {
                    "render_context_ref": "ctx-b"
                },
                "ctx-b": {
                    "context_id": "ctx-b",
                    "context_kind": "excel_render_context",
                    "locale_tag": "en-US",
                    "decimal_separator": ".",
                    "thousands_separator": ","
                }
            }
        }));

        let resolution = scenario.resolve_render_context();

        assert_eq!(
            resolution,
            RenderContextResolution::Untrusted {
                reason: "`render_context_ref` `ctx-a` points to another `render_context_ref`; one-hop resolution only".to_string()
            }
        );
    }

    #[test]
    fn loads_oxxlplay_spreadsheetml_replay_source_metadata() {
        let scenario =
            load_replay_scenario_from_path(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
                "../../docs/test-runs/oxxlplay-seam-xlplay_capture_spreadsheetml_formatting_001-baseline/replay.json",
            ))
            .expect("oxxlplay spreadsheetml replay should load");

        assert_eq!(
            scenario.scenario_id,
            "xlplay_capture_spreadsheetml_formatting_001"
        );
        assert_eq!(scenario.comparison_views.len(), 4);
        assert_eq!(
            scenario
                .source_metadata
                .as_ref()
                .and_then(|value| value.get("projection_status")),
            Some(&serde_json::Value::String("lossy".to_string()))
        );
        assert_eq!(
            scenario
                .source_metadata
                .as_ref()
                .and_then(|value| value.get("source_schema_id")),
            Some(&serde_json::Value::String(
                "oxxlplay.normalized_replay.v1".to_string()
            ))
        );
    }
}
