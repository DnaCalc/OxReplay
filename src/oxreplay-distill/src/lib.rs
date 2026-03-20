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
    let outcome = if scenario.events.is_empty() {
        ReductionOutcome::Quarantined(QuarantineReason::InsufficientCapture)
    } else if predicate.predicate_id.contains("unstable") {
        ReductionOutcome::UnstablePredicate
    } else {
        ReductionOutcome::Stable
    };

    ReplayReductionManifest {
        witness_id: format!("witness-{}", scenario.scenario_id),
        source_scenario_id: scenario.scenario_id.clone(),
        predicate,
        outcome,
    }
}

#[cfg(test)]
mod tests {
    use super::{ReductionOutcome, ReplayPreservationPredicate, planned_reduction};
    use oxreplay_abstractions::LaneId;
    use oxreplay_core::{ReplayEvent, ReplayScenario};

    fn scenario_with_events() -> ReplayScenario {
        ReplayScenario {
            scenario_id: "scenario.test".to_string(),
            lane_id: LaneId("oxcalc".to_string()),
            events: vec![ReplayEvent {
                event_id: "evt-01".to_string(),
                source_label: "candidate_emitted".to_string(),
                normalized_family: "candidate.built".to_string(),
            }],
            registry_refs: Vec::new(),
        }
    }

    #[test]
    fn stable_predicate_stays_stable() {
        let reduction = planned_reduction(
            &scenario_with_events(),
            ReplayPreservationPredicate {
                predicate_id: "preserve-publication-path".to_string(),
                description: "stable".to_string(),
            },
        );

        assert_eq!(reduction.outcome, ReductionOutcome::Stable);
    }

    #[test]
    fn unstable_predicate_is_reported() {
        let reduction = planned_reduction(
            &scenario_with_events(),
            ReplayPreservationPredicate {
                predicate_id: "unstable-oracle-reference".to_string(),
                description: "unstable".to_string(),
            },
        );

        assert_eq!(reduction.outcome, ReductionOutcome::UnstablePredicate);
    }

    #[test]
    fn empty_scenario_is_quarantined() {
        let mut scenario = scenario_with_events();
        scenario.events.clear();

        let reduction = planned_reduction(
            &scenario,
            ReplayPreservationPredicate {
                predicate_id: "preserve-publication-path".to_string(),
                description: "empty".to_string(),
            },
        );

        assert_eq!(
            reduction.outcome,
            ReductionOutcome::Quarantined(
                oxreplay_governance::QuarantineReason::InsufficientCapture
            )
        );
    }
}
