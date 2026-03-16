#![forbid(unsafe_code)]

use oxreplay_core::ReplayScenario;
use oxreplay_governance::QuarantineReason;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayPreservationPredicate {
    pub predicate_id: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReductionOutcome {
    Stable,
    UnstablePredicate,
    Quarantined(QuarantineReason),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayReductionManifest {
    pub witness_id: String,
    pub source_scenario_id: String,
    pub predicate: ReplayPreservationPredicate,
    pub outcome: ReductionOutcome,
}

pub fn planned_reduction(
    scenario: &ReplayScenario,
    predicate: ReplayPreservationPredicate,
) -> ReplayReductionManifest {
    ReplayReductionManifest {
        witness_id: format!("witness-{}", scenario.scenario_id),
        source_scenario_id: scenario.scenario_id.clone(),
        predicate,
        outcome: ReductionOutcome::Stable,
    }
}
