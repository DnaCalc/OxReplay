#![forbid(unsafe_code)]

use oxreplay_diff::ReplayDiff;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainQuery {
    pub query_id: String,
    pub target_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainRecord {
    pub query_id: String,
    pub summary: String,
}

pub fn explain_diff(diff: &ReplayDiff) -> ExplainRecord {
    ExplainRecord {
        query_id: format!("explain-{}", diff.left_scenario_id),
        summary: format!(
            "mismatch {:?} between {} and {}",
            diff.mismatch_kind, diff.left_scenario_id, diff.right_scenario_id
        ),
    }
}
