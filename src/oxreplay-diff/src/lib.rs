#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use oxreplay_abstractions::SeverityClass;
use oxreplay_core::ReplayScenario;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MismatchKind {
    ScenarioPresence,
    VisibleValue,
    EffectiveDisplayText,
    FormattingView,
    ConditionalFormattingView,
    ViewValue,
    ProjectionCoverageGap,
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
pub struct ReplayDiffReport {
    pub equivalent: bool,
    pub mismatches: Vec<ReplayDiff>,
}

pub fn diff_summary(left: &ReplayScenario, right: &ReplayScenario) -> ReplayDiffReport {
    let left_views = left.comparison_view_map();
    let right_views = right.comparison_view_map();

    if !left_views.is_empty() || !right_views.is_empty() {
        return diff_comparison_views(left, right, &left_views, &right_views);
    }

    diff_normalized_events(left, right)
}

fn diff_comparison_views(
    left: &ReplayScenario,
    right: &ReplayScenario,
    left_views: &std::collections::BTreeMap<&str, &serde_json::Value>,
    right_views: &std::collections::BTreeMap<&str, &serde_json::Value>,
) -> ReplayDiffReport {
    let mut families = BTreeSet::new();
    families.extend(left_views.keys().copied());
    families.extend(right_views.keys().copied());

    let mut mismatches = Vec::new();

    for family in ordered_view_families(&families) {
        match (
            left_views.get(family.as_str()),
            right_views.get(family.as_str()),
        ) {
            (Some(left_value), Some(right_value)) if left_value != right_value => {
                mismatches.push(ReplayDiff {
                    left_scenario_id: left.scenario_id.clone(),
                    right_scenario_id: right.scenario_id.clone(),
                    mismatch_kind: mismatch_kind_for_view_family(&family),
                    severity: severity_for_view_family(&family),
                    view_family: Some(family),
                    left_value: Some((*left_value).clone()),
                    right_value: Some((*right_value).clone()),
                    detail: Some("comparison view values diverged".to_string()),
                });
            }
            (Some(left_value), None) => mismatches.push(ReplayDiff {
                left_scenario_id: left.scenario_id.clone(),
                right_scenario_id: right.scenario_id.clone(),
                mismatch_kind: MismatchKind::ProjectionCoverageGap,
                severity: SeverityClass::Coverage,
                view_family: Some(family.clone()),
                left_value: Some((*left_value).clone()),
                right_value: None,
                detail: Some(format!(
                    "comparison view family `{family}` is missing on `{}`",
                    right.scenario_id
                )),
            }),
            (None, Some(right_value)) => mismatches.push(ReplayDiff {
                left_scenario_id: left.scenario_id.clone(),
                right_scenario_id: right.scenario_id.clone(),
                mismatch_kind: MismatchKind::ProjectionCoverageGap,
                severity: SeverityClass::Coverage,
                view_family: Some(family.clone()),
                left_value: None,
                right_value: Some((*right_value).clone()),
                detail: Some(format!(
                    "comparison view family `{family}` is missing on `{}`",
                    left.scenario_id
                )),
            }),
            _ => {}
        }
    }

    ReplayDiffReport {
        equivalent: mismatches.is_empty(),
        mismatches,
    }
}

fn diff_normalized_events(left: &ReplayScenario, right: &ReplayScenario) -> ReplayDiffReport {
    let left_families: Vec<_> = left
        .events
        .iter()
        .map(|event| event.normalized_family.as_str())
        .collect();
    let right_families: Vec<_> = right
        .events
        .iter()
        .map(|event| event.normalized_family.as_str())
        .collect();

    if left_families == right_families {
        return ReplayDiffReport {
            equivalent: true,
            mismatches: Vec::new(),
        };
    }

    ReplayDiffReport {
        equivalent: false,
        mismatches: vec![ReplayDiff {
            left_scenario_id: left.scenario_id.clone(),
            right_scenario_id: right.scenario_id.clone(),
            mismatch_kind: MismatchKind::TraceEvent,
            severity: SeverityClass::Semantic,
            view_family: None,
            left_value: Some(serde_json::json!(left_families)),
            right_value: Some(serde_json::json!(right_families)),
            detail: Some("normalized replay event families diverged".to_string()),
        }],
    }
}

fn ordered_view_families(families: &BTreeSet<&str>) -> Vec<String> {
    const PREFERRED: [&str; 4] = [
        "visible_value",
        "effective_display_text",
        "formatting_view",
        "conditional_formatting_view",
    ];

    let mut ordered = Vec::new();
    for family in PREFERRED {
        if families.contains(family) {
            ordered.push(family.to_string());
        }
    }

    for family in families {
        if !PREFERRED.contains(family) {
            ordered.push((*family).to_string());
        }
    }

    ordered
}

fn mismatch_kind_for_view_family(view_family: &str) -> MismatchKind {
    match view_family {
        "visible_value" => MismatchKind::VisibleValue,
        "effective_display_text" => MismatchKind::EffectiveDisplayText,
        "formatting_view" => MismatchKind::FormattingView,
        "conditional_formatting_view" => MismatchKind::ConditionalFormattingView,
        _ => MismatchKind::ViewValue,
    }
}

fn severity_for_view_family(view_family: &str) -> SeverityClass {
    match view_family {
        "visible_value" => SeverityClass::Semantic,
        _ => SeverityClass::Informational,
    }
}

#[cfg(test)]
mod tests {
    use oxreplay_abstractions::LaneId;
    use oxreplay_core::{ReplayComparisonView, ReplayEvent, ReplayScenario};

    use super::{MismatchKind, diff_summary};

    fn scenario(scenario_id: &str, comparison_views: Vec<ReplayComparisonView>) -> ReplayScenario {
        ReplayScenario {
            scenario_id: scenario_id.to_string(),
            lane_id: LaneId("test".to_string()),
            events: vec![ReplayEvent {
                event_id: format!("{scenario_id}-01"),
                source_label: "event".to_string(),
                normalized_family: "candidate.built".to_string(),
            }],
            registry_refs: vec![],
            comparison_views,
            source_metadata: None,
        }
    }

    #[test]
    fn classifies_display_divergence_explicitly() {
        let left = scenario(
            "left",
            vec![
                ReplayComparisonView {
                    view_family: "visible_value".to_string(),
                    value: serde_json::Value::String("6".to_string()),
                },
                ReplayComparisonView {
                    view_family: "effective_display_text".to_string(),
                    value: serde_json::Value::String("6".to_string()),
                },
            ],
        );
        let right = scenario(
            "right",
            vec![
                ReplayComparisonView {
                    view_family: "visible_value".to_string(),
                    value: serde_json::Value::String("6".to_string()),
                },
                ReplayComparisonView {
                    view_family: "effective_display_text".to_string(),
                    value: serde_json::Value::String("$6.00".to_string()),
                },
            ],
        );

        let report = diff_summary(&left, &right);

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].mismatch_kind,
            MismatchKind::EffectiveDisplayText
        );
        assert_eq!(
            report.mismatches[0].view_family.as_deref(),
            Some("effective_display_text")
        );
    }

    #[test]
    fn classifies_missing_view_family_as_projection_gap() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "visible_value".to_string(),
                value: serde_json::Value::String("6".to_string()),
            }],
        );
        let right = scenario(
            "right",
            vec![
                ReplayComparisonView {
                    view_family: "visible_value".to_string(),
                    value: serde_json::Value::String("6".to_string()),
                },
                ReplayComparisonView {
                    view_family: "formatting_view".to_string(),
                    value: serde_json::json!({
                        "number_format_code": "$#,##0.00"
                    }),
                },
            ],
        );

        let report = diff_summary(&left, &right);

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].mismatch_kind,
            MismatchKind::ProjectionCoverageGap
        );
        assert_eq!(
            report.mismatches[0].view_family.as_deref(),
            Some("formatting_view")
        );
    }
}
