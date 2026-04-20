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
    pub equivalence_policy_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
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
                equivalence_policy_id: None,
                required: None,
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

fn summary_for_mismatch(mismatch: &ReplayDiff) -> String {
    match mismatch.mismatch_kind {
        MismatchKind::ProjectionCoverageGap => format!(
            "comparison view family `{}` is missing on one side",
            mismatch
                .view_family
                .as_deref()
                .unwrap_or("unknown_view_family")
        ),
        MismatchKind::OutcomeValue => match mismatch.detail.as_deref() {
            Some(detail) if is_legacy_outcome_seam_drift(detail) => format!(
                "legacy typed outcome family `{}` is not admitted on the typed execution_outcome comparison path",
                mismatch
                    .view_family
                    .as_deref()
                    .unwrap_or("unknown_outcome_family")
            ),
            _ => format!(
                "typed outcome diverged on `{}`",
                mismatch
                    .view_family
                    .as_deref()
                    .unwrap_or("execution_outcome")
            ),
        },
        _ => {
            let base = format!(
                "comparison diverged on `{}`",
                mismatch
                    .view_family
                    .as_deref()
                    .unwrap_or("normalized_replay_events")
            );

            match mismatch
                .detail
                .as_deref()
                .and_then(classification_label_from_detail)
            {
                Some(label) => format!("{base} ({label})"),
                None => base,
            }
        }
    }
}

fn classification_label_from_detail(detail: &str) -> Option<&str> {
    const LABELS: [&str; 2] = ["near_equal_last_bit", "near_zero_residue"];

    let (label, _) = detail.split_once(':')?;
    LABELS.contains(&label).then_some(label)
}

fn is_legacy_outcome_seam_drift(detail: &str) -> bool {
    detail.starts_with("legacy typed outcome family `")
}

fn explain_mismatch(index: usize, mismatch: &ReplayDiff) -> ExplainRecord {
    let summary = summary_for_mismatch(mismatch);

    ExplainRecord {
        query_id: format!("explain-{}-{:02}", mismatch.left_scenario_id, index + 1),
        summary,
        left_scenario_id: Some(mismatch.left_scenario_id.clone()),
        right_scenario_id: Some(mismatch.right_scenario_id.clone()),
        mismatch_kind: Some(mismatch.mismatch_kind.clone()),
        severity: Some(mismatch.severity),
        view_family: mismatch.view_family.clone(),
        equivalence_policy_id: mismatch.equivalence_policy_id.clone(),
        required: mismatch.required,
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
                equivalence_policy_id: Some("view_json_exact".to_string()),
                required: Some(true),
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
        assert_eq!(
            explain.records[0].equivalence_policy_id.as_deref(),
            Some("view_json_exact")
        );
        assert_eq!(explain.records[0].required, Some(true));
    }

    #[test]
    fn explain_preserves_render_context_detail_for_display_divergence() {
        let report = ReplayDiffReport {
            equivalent: false,
            mismatches: vec![ReplayDiff {
                left_scenario_id: "left".to_string(),
                right_scenario_id: "right".to_string(),
                mismatch_kind: MismatchKind::EffectiveDisplayText,
                severity: SeverityClass::Informational,
                view_family: Some("effective_display_text".to_string()),
                equivalence_policy_id: Some("effective_display_text_exact".to_string()),
                required: Some(true),
                left_value: Some(serde_json::json!("1,23")),
                right_value: Some(serde_json::json!("1.23")),
                detail: Some(
                    "comparison view values diverged (render_context_diverged(locale_tag left=`nl-NL` right=`en-US`); render_context=resolved(inline, locale_tag=`nl-NL`, decimal_separator=`,`, thousands_separator=`.`, trust_class=`unpinned`); render_context=resolved(inline, locale_tag=`en-US`, decimal_separator=`.`, thousands_separator=`,`, trust_class=`direct`))".to_string(),
                ),
            }],
        };

        let explain = explain_diff(&report);

        assert!(!explain.equivalent);
        assert_eq!(explain.records.len(), 1);
        assert_eq!(
            explain.records[0].summary,
            "comparison diverged on `effective_display_text`"
        );
        assert!(explain.records[0].detail.as_deref().is_some_and(|detail| {
            detail.contains("render_context_diverged(locale_tag left=`nl-NL` right=`en-US`)")
                && detail.contains("render_context=resolved(inline, locale_tag=`nl-NL`")
                && detail.contains("render_context=resolved(inline, locale_tag=`en-US`")
        }));
    }

    #[test]
    fn explain_surfaces_numeric_mismatch_shape_in_summary() {
        let report = ReplayDiffReport {
            equivalent: false,
            mismatches: vec![ReplayDiff {
                left_scenario_id: "left".to_string(),
                right_scenario_id: "right".to_string(),
                mismatch_kind: MismatchKind::ComparisonValue,
                severity: SeverityClass::Semantic,
                view_family: Some("worksheet_comparison_value".to_string()),
                equivalence_policy_id: Some("worksheet_value_exact".to_string()),
                required: Some(true),
                left_value: Some(serde_json::json!(14.206699082890463_f64)),
                right_value: Some(serde_json::json!(14.206699082890465_f64)),
                detail: Some(
                    "near_equal_last_bit: finite numeric comparison values differ by 1 ULP (left=14.206699082890463, right=14.206699082890465, abs_delta=1.7763568394002505e-15)".to_string(),
                ),
            }],
        };

        let explain = explain_diff(&report);

        assert!(!explain.equivalent);
        assert_eq!(explain.records.len(), 1);
        assert_eq!(
            explain.records[0].summary,
            "comparison diverged on `worksheet_comparison_value` (near_equal_last_bit)"
        );
        assert_eq!(
            explain.records[0].equivalence_policy_id.as_deref(),
            Some("worksheet_value_exact")
        );
        assert_eq!(explain.records[0].required, Some(true));
        assert_eq!(
            explain.records[0].detail.as_deref(),
            Some(
                "near_equal_last_bit: finite numeric comparison values differ by 1 ULP (left=14.206699082890463, right=14.206699082890465, abs_delta=1.7763568394002505e-15)"
            )
        );
    }

    #[test]
    fn explain_surfaces_typed_outcome_summary() {
        let report = ReplayDiffReport {
            equivalent: false,
            mismatches: vec![ReplayDiff {
                left_scenario_id: "left".to_string(),
                right_scenario_id: "right".to_string(),
                mismatch_kind: MismatchKind::OutcomeValue,
                severity: SeverityClass::Semantic,
                view_family: Some("execution_outcome".to_string()),
                equivalence_policy_id: Some("typed_outcome_class".to_string()),
                required: Some(true),
                left_value: Some(serde_json::json!({
                    "outcome_kind": "rejected",
                    "outcome_stage": "authoring",
                    "class_id": "input_rejected"
                })),
                right_value: Some(serde_json::json!({
                    "outcome_kind": "executed",
                    "outcome_stage": "execution",
                    "class_id": "value_published"
                })),
                detail: Some(
                    "typed outcome classes diverged (left_stage=`authoring`, right_stage=`execution`, left_outcome_kind=`rejected`, right_outcome_kind=`executed`, left_class_id=`input_rejected`, right_class_id=`value_published`)".to_string(),
                ),
            }],
        };

        let explain = explain_diff(&report);

        assert!(!explain.equivalent);
        assert_eq!(explain.records.len(), 1);
        assert_eq!(
            explain.records[0].summary,
            "typed outcome diverged on `execution_outcome`"
        );
        assert_eq!(
            explain.records[0].equivalence_policy_id.as_deref(),
            Some("typed_outcome_class")
        );
        assert_eq!(explain.records[0].required, Some(true));
    }

    #[test]
    fn explain_surfaces_legacy_outcome_family_seam_drift() {
        let report = ReplayDiffReport {
            equivalent: false,
            mismatches: vec![ReplayDiff {
                left_scenario_id: "left".to_string(),
                right_scenario_id: "right".to_string(),
                mismatch_kind: MismatchKind::OutcomeValue,
                severity: SeverityClass::Instrumentation,
                view_family: Some("authoring_outcome".to_string()),
                equivalence_policy_id: None,
                required: Some(true),
                left_value: Some(serde_json::json!({
                    "outcome_kind": "rejected",
                    "outcome_stage": "authoring",
                    "class_id": "input_rejected"
                })),
                right_value: None,
                detail: Some(
                    "legacy typed outcome family `authoring_outcome` is not admitted on the typed execution_outcome comparison path; publish `execution_outcome` with explicit `outcome_kind`, `outcome_stage`, `class_id`, and optional `lane_reason_code`".to_string(),
                ),
            }],
        };

        let explain = explain_diff(&report);

        assert!(!explain.equivalent);
        assert_eq!(explain.records.len(), 1);
        assert_eq!(
            explain.records[0].summary,
            "legacy typed outcome family `authoring_outcome` is not admitted on the typed execution_outcome comparison path"
        );
        assert_eq!(explain.records[0].required, Some(true));
        assert_eq!(explain.records[0].equivalence_policy_id, None);
    }
}
