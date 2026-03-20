#![forbid(unsafe_code)]

use oxreplay_diff::ReplayDiffReport;
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

pub fn explain_diff(diff: &ReplayDiffReport) -> ExplainRecord {
    if diff.equivalent {
        return ExplainRecord {
            query_id: "explain-equivalent".to_string(),
            summary: "scenarios are equivalent on the current normalized replay surface"
                .to_string(),
        };
    }

    let first = &diff.mismatches[0];
    ExplainRecord {
        query_id: format!("explain-{}", first.left_scenario_id),
        summary: format!(
            "mismatch {:?} between {} and {}",
            first.mismatch_kind, first.left_scenario_id, first.right_scenario_id
        ),
    }
}
