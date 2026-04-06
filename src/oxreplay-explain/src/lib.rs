#![forbid(unsafe_code)]

use oxreplay_abstractions::SeverityClass;
use oxreplay_diff::{MismatchKind, ReplayDiff, ReplayDiffReport};
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_scenario_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right_scenario_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mismatch_kind: Option<MismatchKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<SeverityClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_value: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right_value: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainReport {
    pub equivalent: bool,
    pub records: Vec<ExplainRecord>,
}

pub fn explain_diff(diff: &ReplayDiffReport) -> ExplainReport {
    if diff.equivalent {
        return ExplainReport {
            equivalent: true,
            records: vec![ExplainRecord {
                query_id: "explain-equivalent".to_string(),
                summary: "scenarios are equivalent on the current comparison surface".to_string(),
                left_scenario_id: None,
                right_scenario_id: None,
                mismatch_kind: None,
                severity: None,
                view_family: None,
                left_value: None,
                right_value: None,
                detail: None,
            }],
        };
    }

    ExplainReport {
        equivalent: false,
        records: diff
            .mismatches
            .iter()
            .enumerate()
            .map(|(index, mismatch)| explain_mismatch(index, mismatch))
            .collect(),
    }
}

fn explain_mismatch(index: usize, mismatch: &ReplayDiff) -> ExplainRecord {
    let summary = match mismatch.mismatch_kind {
        MismatchKind::ProjectionCoverageGap => format!(
            "comparison view family `{}` is missing on one side",
            mismatch
                .view_family
                .as_deref()
                .unwrap_or("unknown_view_family")
        ),
        _ => format!(
            "comparison diverged on `{}`",
            mismatch
                .view_family
                .as_deref()
                .unwrap_or("normalized_replay_events")
        ),
    };

    ExplainRecord {
        query_id: format!("explain-{}-{:02}", mismatch.left_scenario_id, index + 1),
        summary,
        left_scenario_id: Some(mismatch.left_scenario_id.clone()),
        right_scenario_id: Some(mismatch.right_scenario_id.clone()),
        mismatch_kind: Some(mismatch.mismatch_kind.clone()),
        severity: Some(mismatch.severity),
        view_family: mismatch.view_family.clone(),
        left_value: mismatch.left_value.clone(),
        right_value: mismatch.right_value.clone(),
        detail: mismatch.detail.clone(),
    }
}

#[cfg(test)]
mod tests {
    use oxreplay_abstractions::SeverityClass;
    use oxreplay_diff::{MismatchKind, ReplayDiff, ReplayDiffReport};

    use super::explain_diff;

    #[test]
    fn explain_reports_missing_view_family() {
        let report = ReplayDiffReport {
            equivalent: false,
            mismatches: vec![ReplayDiff {
                left_scenario_id: "left".to_string(),
                right_scenario_id: "right".to_string(),
                mismatch_kind: MismatchKind::ProjectionCoverageGap,
                severity: SeverityClass::Coverage,
                view_family: Some("formatting_view".to_string()),
                left_value: None,
                right_value: Some(serde_json::json!({
                    "number_format_code": "$#,##0.00"
                })),
                detail: Some(
                    "comparison view family `formatting_view` is missing on `left`".to_string(),
                ),
            }],
        };

        let explain = explain_diff(&report);

        assert!(!explain.equivalent);
        assert_eq!(explain.records.len(), 1);
        assert_eq!(
            explain.records[0].view_family.as_deref(),
            Some("formatting_view")
        );
        assert_eq!(
            explain.records[0].mismatch_kind,
            Some(MismatchKind::ProjectionCoverageGap)
        );
    }
}
