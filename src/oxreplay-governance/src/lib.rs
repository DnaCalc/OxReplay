#![forbid(unsafe_code)]

use oxreplay_abstractions::LaneId;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WitnessLifecycleState {
    ExplanatoryOnly,
    RetainedLocal,
    RetainedShared,
    PromotedPack,
    Superseded,
    Quarantined,
    GcEligible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineReason {
    UnstableOracleOrPredicate,
    InsufficientCapture,
    MissingSourceArtifacts,
    AdapterFailure,
    SchemaIncompatibility,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessLifecycleRecord {
    pub witness_id: String,
    pub lane_id: LaneId,
    pub scenario_id: String,
    pub state: WitnessLifecycleState,
    pub quarantine_reason: Option<QuarantineReason>,
    pub supersedes: Option<String>,
    pub pack_eligible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleTransitionResult {
    pub accepted: bool,
    pub previous_state: WitnessLifecycleState,
    pub next_state: WitnessLifecycleState,
    pub record: WitnessLifecycleRecord,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WitnessLifecycleError {
    #[error("transition from {from:?} to {to:?} is not allowed")]
    InvalidTransition {
        from: WitnessLifecycleState,
        to: WitnessLifecycleState,
    },
}

pub fn transition_lifecycle(
    record: &WitnessLifecycleRecord,
    next_state: WitnessLifecycleState,
) -> Result<LifecycleTransitionResult, WitnessLifecycleError> {
    if !is_allowed_transition(&record.state, &next_state) {
        return Err(WitnessLifecycleError::InvalidTransition {
            from: record.state.clone(),
            to: next_state,
        });
    }

    let mut updated = record.clone();
    updated.state = next_state.clone();
    updated.pack_eligible = matches!(updated.state, WitnessLifecycleState::PromotedPack);
    if !matches!(updated.state, WitnessLifecycleState::Quarantined) {
        updated.quarantine_reason = None;
    }

    Ok(LifecycleTransitionResult {
        accepted: true,
        previous_state: record.state.clone(),
        next_state,
        record: updated,
        diagnostics: Vec::new(),
    })
}

fn is_allowed_transition(from: &WitnessLifecycleState, to: &WitnessLifecycleState) -> bool {
    matches!(
        (from, to),
        (
            WitnessLifecycleState::ExplanatoryOnly,
            WitnessLifecycleState::RetainedLocal
        ) | (
            WitnessLifecycleState::ExplanatoryOnly,
            WitnessLifecycleState::Quarantined
        ) | (
            WitnessLifecycleState::RetainedLocal,
            WitnessLifecycleState::RetainedShared
        ) | (
            WitnessLifecycleState::RetainedLocal,
            WitnessLifecycleState::Quarantined
        ) | (
            WitnessLifecycleState::RetainedLocal,
            WitnessLifecycleState::Superseded
        ) | (
            WitnessLifecycleState::RetainedShared,
            WitnessLifecycleState::PromotedPack
        ) | (
            WitnessLifecycleState::RetainedShared,
            WitnessLifecycleState::Quarantined
        ) | (
            WitnessLifecycleState::RetainedShared,
            WitnessLifecycleState::Superseded
        ) | (
            WitnessLifecycleState::PromotedPack,
            WitnessLifecycleState::Superseded
        ) | (
            WitnessLifecycleState::Quarantined,
            WitnessLifecycleState::RetainedLocal
        ) | (
            WitnessLifecycleState::Quarantined,
            WitnessLifecycleState::Superseded
        ) | (
            WitnessLifecycleState::Superseded,
            WitnessLifecycleState::GcEligible
        )
    )
}

#[cfg(test)]
mod tests {
    use super::{
        QuarantineReason, WitnessLifecycleRecord, WitnessLifecycleState, transition_lifecycle,
    };
    use oxreplay_abstractions::LaneId;

    fn record(state: WitnessLifecycleState) -> WitnessLifecycleRecord {
        WitnessLifecycleRecord {
            witness_id: "wit-001".to_string(),
            lane_id: LaneId("oxcalc".to_string()),
            scenario_id: "scenario-001".to_string(),
            state,
            quarantine_reason: None,
            supersedes: None,
            pack_eligible: false,
        }
    }

    #[test]
    fn explanatory_only_can_transition_to_retained_local() {
        let result = transition_lifecycle(
            &record(WitnessLifecycleState::ExplanatoryOnly),
            WitnessLifecycleState::RetainedLocal,
        )
        .expect("transition should succeed");

        assert_eq!(result.record.state, WitnessLifecycleState::RetainedLocal);
        assert!(!result.record.pack_eligible);
    }

    #[test]
    fn quarantined_record_clears_reason_when_released() {
        let mut quarantined = record(WitnessLifecycleState::Quarantined);
        quarantined.quarantine_reason = Some(QuarantineReason::InsufficientCapture);

        let result = transition_lifecycle(&quarantined, WitnessLifecycleState::RetainedLocal)
            .expect("transition should succeed");

        assert_eq!(result.record.quarantine_reason, None);
    }

    #[test]
    fn promoted_pack_is_pack_eligible() {
        let result = transition_lifecycle(
            &record(WitnessLifecycleState::RetainedShared),
            WitnessLifecycleState::PromotedPack,
        )
        .expect("transition should succeed");

        assert!(result.record.pack_eligible);
    }
}
