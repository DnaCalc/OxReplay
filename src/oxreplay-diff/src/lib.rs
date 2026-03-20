#![forbid(unsafe_code)]

use oxreplay_abstractions::SeverityClass;
use oxreplay_core::ReplayScenario;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MismatchKind {
    ScenarioPresence,
    ViewValue,
    RejectKind,
    TraceEvent,
    CounterValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayDiff {
    pub left_scenario_id: String,
    pub right_scenario_id: String,
    pub mismatch_kind: MismatchKind,
    pub severity: SeverityClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayDiffReport {
    pub equivalent: bool,
    pub mismatches: Vec<ReplayDiff>,
}

pub fn diff_summary(left: &ReplayScenario, right: &ReplayScenario) -> ReplayDiffReport {
    let left_families: Vec<_> = left
        .events
        .iter()
        .map(|event| event.normalized_family.as_str())
        .collect();
    let right_families: Vec<_> = right
        .events
        .iter()
        .map(|event| event.normalized_family.as_str())
        .collect();

    if left_families == right_families {
        return ReplayDiffReport {
            equivalent: true,
            mismatches: Vec::new(),
        };
    }

    let mismatch_kind = if left.events.len() == right.events.len() {
        MismatchKind::ViewValue
    } else {
        MismatchKind::TraceEvent
    };

    ReplayDiffReport {
        equivalent: false,
        mismatches: vec![ReplayDiff {
            left_scenario_id: left.scenario_id.clone(),
            right_scenario_id: right.scenario_id.clone(),
            mismatch_kind,
            severity: SeverityClass::Semantic,
        }],
    }
}
