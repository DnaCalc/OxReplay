#![forbid(unsafe_code)]

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayView {
    pub view_family: String,
    pub artifact_path: String,
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
    #[error("requested case `{case_id}` was not found in `{path}`")]
    MissingCase { path: String, case_id: String },
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
struct OxFmlFecCommitCase {
    case_id: String,
    expected: OxFmlExpected,
}

#[derive(Debug, Deserialize)]
struct OxFmlExpected {
    decision: String,
    #[serde(default)]
    published_payload: Option<String>,
    #[serde(default)]
    spill_event_kind: Option<String>,
    #[serde(default)]
    reject_code: Option<String>,
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

pub fn load_oxfml_fec_projection(
    path: impl AsRef<Path>,
    case_id: &str,
) -> Result<ReplayScenario, ReplayScenarioLoadError> {
    let path = path.as_ref();
    let path_text = path.display().to_string();
    let source = fs::read_to_string(path).map_err(|source| ReplayScenarioLoadError::Read {
        path: path_text.clone(),
        source,
    })?;
    let cases: Vec<OxFmlFecCommitCase> =
        serde_json::from_str(&source).map_err(|source| ReplayScenarioLoadError::Parse {
            path: path_text.clone(),
            source,
        })?;
    let case = cases
        .into_iter()
        .find(|candidate| candidate.case_id == case_id)
        .ok_or_else(|| ReplayScenarioLoadError::MissingCase {
            path: path_text,
            case_id: case_id.to_string(),
        })?;

    let mut events = Vec::new();
    match case.expected.decision.as_str() {
        "accepted" => {
            events.push(ReplayEvent {
                event_id: format!("{case_id}-candidate"),
                source_label: "AcceptedCandidateResultBuilt".to_string(),
                normalized_family: "candidate.accepted".to_string(),
            });
            events.push(ReplayEvent {
                event_id: format!("{case_id}-publication"),
                source_label: "CommitAccepted".to_string(),
                normalized_family: "publication.committed".to_string(),
            });
        }
        "rejected" => {
            events.push(ReplayEvent {
                event_id: format!("{case_id}-reject"),
                source_label: case
                    .expected
                    .reject_code
                    .clone()
                    .unwrap_or_else(|| "RejectIssued".to_string()),
                normalized_family: "reject.issued".to_string(),
            });
        }
        _ => {}
    }

    if let Some(spill_event_kind) = case.expected.spill_event_kind {
        events.push(ReplayEvent {
            event_id: format!("{case_id}-spill"),
            source_label: spill_event_kind,
            normalized_family: "spill.observed".to_string(),
        });
    }

    if let Some(published_payload) = case.expected.published_payload {
        events.push(ReplayEvent {
            event_id: format!("{case_id}-payload"),
            source_label: published_payload,
            normalized_family: "publication.payload".to_string(),
        });
    }

    Ok(ReplayScenario {
        scenario_id: map_oxfml_case_id(case_id).to_string(),
        lane_id: LaneId("oxfml".to_string()),
        events,
        registry_refs: vec![],
        source_metadata: None,
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
            path: path_text,
            source,
        })?;
    let scenario_id = select_oxfml_v1_scenario_id(&projection);
    let events = project_oxfml_v1_events(&scenario_id, &projection);

    Ok(ReplayScenario {
        scenario_id,
        lane_id: LaneId("oxfml".to_string()),
        events,
        registry_refs: vec![],
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

fn map_oxfml_case_id(case_id: &str) -> &str {
    match case_id {
        "fec_001_accept" => "oxfml_fec_accept_publication_001",
        "fec_002_formula_token_reject" => "oxfml_fec_formula_token_reject_001",
        "fec_003_capability_view_reject" => "oxfml_fec_capability_view_reject_001",
        _ => case_id,
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
    } else if trace_event_kind.contains("commit") && trace_event_kind.contains("accept") {
        "publication.committed"
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

#[cfg(test)]
mod tests {
    use super::{
        is_replay_ready, load_oxcalc_tracecalc_projection, load_oxfml_fec_projection,
        load_oxfml_v1_replay_projection, load_replay_scenario_from_path,
    };
    use std::path::PathBuf;

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
    fn projects_real_oxfml_fec_case() {
        let scenario = load_oxfml_fec_projection(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
                "../../../OxFml/crates/oxfml_core/tests/fixtures/fec_commit_replay_cases.json",
            ),
            "fec_001_accept",
        )
        .expect("oxfml fixture should load");

        assert_eq!(scenario.scenario_id, "oxfml_fec_accept_publication_001");
        assert!(is_replay_ready(&scenario));
        assert!(scenario.events.len() >= 2);
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
        assert!(scenario.source_metadata.is_none());
    }
}
