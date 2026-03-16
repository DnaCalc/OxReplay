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

pub fn diff_summary(left: &ReplayScenario, right: &ReplayScenario) -> ReplayDiff {
    let mismatch_kind = if left.events.len() == right.events.len() {
        MismatchKind::ViewValue
    } else {
        MismatchKind::TraceEvent
    };

    ReplayDiff {
        left_scenario_id: left.scenario_id.clone(),
        right_scenario_id: right.scenario_id.clone(),
        mismatch_kind,
        severity: SeverityClass::Semantic,
    }
}
