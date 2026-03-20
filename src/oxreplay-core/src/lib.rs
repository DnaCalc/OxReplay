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

#[cfg(test)]
mod tests {
    use super::{
        is_replay_ready, load_oxcalc_tracecalc_projection, load_oxfml_fec_projection,
        load_replay_scenario_from_path,
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
    fn loads_normalized_replay_scenario_fixture() {
        let scenario =
            load_replay_scenario_from_path(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
                "../../docs/test-corpus/bundles/crosslane_replay_identity_001/left.replay.json",
            ))
            .expect("normalized fixture should load");

        assert_eq!(scenario.scenario_id, "crosslane_replay_identity_001_left");
        assert!(is_replay_ready(&scenario));
        assert_eq!(scenario.events.len(), 2);
    }
}
