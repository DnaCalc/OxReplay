#![forbid(unsafe_code)]

use oxreplay_abstractions::{LaneId, RegistryRef};
use serde::{Deserialize, Serialize};

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
