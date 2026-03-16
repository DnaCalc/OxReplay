#![forbid(unsafe_code)]

use oxreplay_abstractions::LaneId;
use serde::{Deserialize, Serialize};

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
