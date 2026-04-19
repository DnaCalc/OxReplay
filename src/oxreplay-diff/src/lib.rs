#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use oxreplay_abstractions::SeverityClass;
use oxreplay_core::{ReplayComparisonView, ReplayScenario, TypedOutcomeValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MismatchKind {
    ScenarioPresence,
    ComparisonValue,
    VisibleValue,
    EffectiveDisplayText,
    FormattingView,
    ConditionalFormattingView,
    OutcomeValue,
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
pub struct ReplayDiffReport {
    pub equivalent: bool,
    pub mismatches: Vec<ReplayDiff>,
}

pub fn diff_summary(left: &ReplayScenario, right: &ReplayScenario) -> ReplayDiffReport {
    let left_views = left.normalized_comparison_view_map();
    let right_views = right.normalized_comparison_view_map();

    if !left_views.is_empty() || !right_views.is_empty() {
        return diff_comparison_views(left, right, &left_views, &right_views);
    }

    diff_normalized_events(left, right)
}

fn diff_comparison_views(
    left: &ReplayScenario,
    right: &ReplayScenario,
    left_views: &std::collections::BTreeMap<String, &ReplayComparisonView>,
    right_views: &std::collections::BTreeMap<String, &ReplayComparisonView>,
) -> ReplayDiffReport {
    let mut families = BTreeSet::new();
    families.extend(left_views.keys().cloned());
    families.extend(right_views.keys().cloned());

    let left_has_legacy_outcome_family = left_views
        .keys()
        .any(|family| is_legacy_outcome_family(family));
    let right_has_legacy_outcome_family = right_views
        .keys()
        .any(|family| is_legacy_outcome_family(family));

    let mut mismatches = Vec::new();

    for family in ordered_view_families(&families) {
        if is_legacy_outcome_family(&family) {
            mismatches.push(legacy_outcome_family_mismatch(
                left,
                right,
                &family,
                left_views.get(&family).copied(),
                right_views.get(&family).copied(),
            ));
            continue;
        }

        match (left_views.get(&family), right_views.get(&family)) {
            (Some(left_view), Some(right_view)) => {
                let left_contract = left_view.comparison_contract();
                let right_contract = right_view.comparison_contract();
                if left_contract.equivalence_policy_id != right_contract.equivalence_policy_id {
                    mismatches.push(ReplayDiff {
                        left_scenario_id: left.scenario_id.clone(),
                        right_scenario_id: right.scenario_id.clone(),
                        mismatch_kind: mismatch_kind_for_view_family(&family),
                        severity: SeverityClass::Instrumentation,
                        view_family: Some(family.clone()),
                        equivalence_policy_id: None,
                        required: Some(left_contract.required || right_contract.required),
                        left_value: Some(left_view.value.clone()),
                        right_value: Some(right_view.value.clone()),
                        detail: Some(format!(
                            "comparison view family `{family}` declared incompatible equivalence policies (left=`{}`, right=`{}`)",
                            left_contract.equivalence_policy_id, right_contract.equivalence_policy_id
                        )),
                    });
                    continue;
                }

                let comparison = comparison_view_values(&family, left_view, right_view);
                if comparison.equivalent {
                    continue;
                }

                mismatches.push(ReplayDiff {
                    left_scenario_id: left.scenario_id.clone(),
                    right_scenario_id: right.scenario_id.clone(),
                    mismatch_kind: mismatch_kind_for_view_family(&family),
                    severity: severity_for_view_family(&family),
                    view_family: Some(family.clone()),
                    equivalence_policy_id: Some(left_contract.equivalence_policy_id),
                    required: Some(left_contract.required || right_contract.required),
                    left_value: Some(left_view.value.clone()),
                    right_value: Some(right_view.value.clone()),
                    detail: Some(comparison.detail),
                });
            }
            (Some(left_view), None) => {
                if family == "execution_outcome" && right_has_legacy_outcome_family {
                    continue;
                }

                let contract = left_view.comparison_contract();
                if !contract.required {
                    continue;
                }

                mismatches.push(ReplayDiff {
                    left_scenario_id: left.scenario_id.clone(),
                    right_scenario_id: right.scenario_id.clone(),
                    mismatch_kind: MismatchKind::ProjectionCoverageGap,
                    severity: SeverityClass::Coverage,
                    view_family: Some(family.clone()),
                    equivalence_policy_id: Some(contract.equivalence_policy_id),
                    required: Some(true),
                    left_value: Some(left_view.value.clone()),
                    right_value: None,
                    detail: Some(format!(
                        "comparison view family `{family}` is missing on `{}`",
                        right.scenario_id
                    )),
                });
            }
            (None, Some(right_view)) => {
                if family == "execution_outcome" && left_has_legacy_outcome_family {
                    continue;
                }

                let contract = right_view.comparison_contract();
                if !contract.required {
                    continue;
                }

                mismatches.push(ReplayDiff {
                    left_scenario_id: left.scenario_id.clone(),
                    right_scenario_id: right.scenario_id.clone(),
                    mismatch_kind: MismatchKind::ProjectionCoverageGap,
                    severity: SeverityClass::Coverage,
                    view_family: Some(family.clone()),
                    equivalence_policy_id: Some(contract.equivalence_policy_id),
                    required: Some(true),
                    left_value: None,
                    right_value: Some(right_view.value.clone()),
                    detail: Some(format!(
                        "comparison view family `{family}` is missing on `{}`",
                        left.scenario_id
                    )),
                });
            }
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
            equivalence_policy_id: None,
            required: None,
            left_value: Some(serde_json::json!(left_families)),
            right_value: Some(serde_json::json!(right_families)),
            detail: Some("normalized replay event families diverged".to_string()),
        }],
    }
}

fn is_legacy_outcome_family(view_family: &str) -> bool {
    matches!(
        view_family,
        "authoring_outcome" | "bind_outcome" | "publication_outcome"
    )
}

fn legacy_outcome_family_mismatch(
    left: &ReplayScenario,
    right: &ReplayScenario,
    family: &str,
    left_view: Option<&ReplayComparisonView>,
    right_view: Option<&ReplayComparisonView>,
) -> ReplayDiff {
    ReplayDiff {
        left_scenario_id: left.scenario_id.clone(),
        right_scenario_id: right.scenario_id.clone(),
        mismatch_kind: MismatchKind::OutcomeValue,
        severity: SeverityClass::Instrumentation,
        view_family: Some(family.to_string()),
        equivalence_policy_id: None,
        required: Some(true),
        left_value: left_view.map(|view| view.value.clone()),
        right_value: right_view.map(|view| view.value.clone()),
        detail: Some(format!(
            "legacy typed outcome family `{family}` is not admitted on the typed execution_outcome comparison path; publish `execution_outcome` with explicit `outcome_kind`, `outcome_stage`, `class_id`, and optional `lane_reason_code`"
        )),
    }
}

fn ordered_view_families(families: &BTreeSet<String>) -> Vec<String> {
    const PREFERRED: [&str; 6] = [
        "worksheet_comparison_value",
        "effective_display_text",
        "visible_value_text",
        "execution_outcome",
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
        if !PREFERRED.contains(&family.as_str()) {
            ordered.push(family.clone());
        }
    }

    ordered
}

fn mismatch_kind_for_view_family(view_family: &str) -> MismatchKind {
    match view_family {
        "worksheet_comparison_value" => MismatchKind::ComparisonValue,
        "effective_display_text" => MismatchKind::EffectiveDisplayText,
        "visible_value_text" => MismatchKind::VisibleValue,
        "execution_outcome" => MismatchKind::OutcomeValue,
        "formatting_view" => MismatchKind::FormattingView,
        "conditional_formatting_view" => MismatchKind::ConditionalFormattingView,
        _ => MismatchKind::ViewValue,
    }
}

fn severity_for_view_family(view_family: &str) -> SeverityClass {
    match view_family {
        "worksheet_comparison_value" | "execution_outcome" => SeverityClass::Semantic,
        _ => SeverityClass::Informational,
    }
}

fn detail_for_view_family(view_family: &str) -> String {
    match view_family {
        "worksheet_comparison_value" => "typed comparison values diverged".to_string(),
        "execution_outcome" => "typed outcome classes diverged".to_string(),
        _ => "comparison view values diverged".to_string(),
    }
}

struct ViewComparison {
    equivalent: bool,
    detail: String,
}

fn comparison_view_values(
    view_family: &str,
    left: &ReplayComparisonView,
    right: &ReplayComparisonView,
) -> ViewComparison {
    match view_family {
        "worksheet_comparison_value" => comparison_value_equal(&left.value, &right.value),
        "execution_outcome" => outcome_value_equal(left, right),
        _ => ViewComparison {
            equivalent: left.value == right.value,
            detail: detail_for_view_family(view_family),
        },
    }
}

fn comparison_value_equal(left: &serde_json::Value, right: &serde_json::Value) -> ViewComparison {
    // Switch point: replace this local replay-wire normalization with direct
    // OxFunc-owned serde/wire helpers once that surface is admitted for reuse.
    match (
        parse_typed_comparison_value(left),
        parse_typed_comparison_value(right),
    ) {
        (Ok(left), Ok(right)) => ViewComparison {
            equivalent: left == right,
            detail: detail_for_typed_comparison_divergence(&left, &right),
        },
        (Err(left_error), Err(right_error)) => ViewComparison {
            equivalent: false,
            detail: format!(
                "comparison_value envelopes are outside the admitted local seam (left: {}; right: {})",
                left_error.as_message(),
                right_error.as_message()
            ),
        },
        (Err(left_error), Ok(_)) => ViewComparison {
            equivalent: false,
            detail: format!(
                "left comparison_value envelope is outside the admitted local seam: {}",
                left_error.as_message()
            ),
        },
        (Ok(_), Err(right_error)) => ViewComparison {
            equivalent: false,
            detail: format!(
                "right comparison_value envelope is outside the admitted local seam: {}",
                right_error.as_message()
            ),
        },
    }
}

fn detail_for_typed_comparison_divergence(
    left: &TypedComparisonValue,
    right: &TypedComparisonValue,
) -> String {
    classify_numeric_divergence(left, right)
        .unwrap_or_else(|| detail_for_view_family("worksheet_comparison_value"))
}

fn classify_numeric_divergence(
    left: &TypedComparisonValue,
    right: &TypedComparisonValue,
) -> Option<String> {
    let (TypedComparisonValue::Number(left_bits), TypedComparisonValue::Number(right_bits)) =
        (left, right)
    else {
        return None;
    };

    let left = f64::from_bits(*left_bits);
    let right = f64::from_bits(*right_bits);
    if !left.is_finite() || !right.is_finite() {
        return None;
    }

    let abs_delta = (left - right).abs();
    if ordered_float_bits(*left_bits).abs_diff(ordered_float_bits(*right_bits)) == 1 {
        return Some(format!(
            "near_equal_last_bit: finite numeric comparison values differ by 1 ULP (left={left:?}, right={right:?}, abs_delta={abs_delta:?})"
        ));
    }

    if (left == 0.0 && right != 0.0 || right == 0.0 && left != 0.0) && abs_delta <= f64::EPSILON {
        return Some(format!(
            "near_zero_residue: one side is exact zero and the other is a tiny finite residue below f64::EPSILON (left={left:?}, right={right:?}, abs_delta={abs_delta:?})"
        ));
    }

    None
}

fn ordered_float_bits(bits: u64) -> u64 {
    const SIGN_BIT: u64 = 1_u64 << 63;

    if bits & SIGN_BIT == 0 {
        bits | SIGN_BIT
    } else {
        !bits
    }
}

fn outcome_value_equal(
    left: &ReplayComparisonView,
    right: &ReplayComparisonView,
) -> ViewComparison {
    match (
        parse_typed_outcome_value(left),
        parse_typed_outcome_value(right),
    ) {
        (Ok(left), Ok(right)) => ViewComparison {
            equivalent: left.outcome_kind == right.outcome_kind && left.class_id == right.class_id,
            detail: if left.outcome_kind == right.outcome_kind && left.class_id == right.class_id {
                detail_for_view_family("execution_outcome")
            } else {
                format!(
                    "typed outcome classes diverged (left_stage=`{}`, right_stage=`{}`, left_outcome_kind=`{}`, right_outcome_kind=`{}`, left_class_id=`{}`, right_class_id=`{}`)",
                    left.outcome_stage,
                    right.outcome_stage,
                    left.outcome_kind,
                    right.outcome_kind,
                    left.class_id,
                    right.class_id,
                )
            },
        },
        (Err(left_error), Err(right_error)) => ViewComparison {
            equivalent: false,
            detail: format!(
                "execution_outcome envelopes are outside the admitted local seam (left: {left_error}; right: {right_error})"
            ),
        },
        (Err(left_error), Ok(_)) => ViewComparison {
            equivalent: false,
            detail: format!(
                "left execution_outcome envelope is outside the admitted local seam: {left_error}"
            ),
        },
        (Ok(_), Err(right_error)) => ViewComparison {
            equivalent: false,
            detail: format!(
                "right execution_outcome envelope is outside the admitted local seam: {right_error}"
            ),
        },
    }
}

fn parse_typed_outcome_value(
    view: &ReplayComparisonView,
) -> Result<TypedOutcomeValue, &'static str> {
    let object = view.value.as_object().ok_or("expected object payload")?;

    let outcome_kind = object
        .get("outcome_kind")
        .and_then(serde_json::Value::as_str)
        .ok_or("missing outcome_kind")?
        .to_string();
    let outcome_stage = object
        .get("outcome_stage")
        .and_then(serde_json::Value::as_str)
        .ok_or("missing outcome_stage")?
        .to_string();
    let class_id = object
        .get("class_id")
        .and_then(serde_json::Value::as_str)
        .ok_or("missing class_id")?
        .to_string();

    Ok(TypedOutcomeValue {
        outcome_kind,
        outcome_stage,
        class_id,
        lane_reason_code: object
            .get("lane_reason_code")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string),
        human_summary: object
            .get("human_summary")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string),
        raw_detail: object.get("raw_detail").cloned(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComparisonValueParseError {
    MissingDeclaredKind,
    UnsupportedDeclaredKind,
    InvalidLogicalPayload,
    InvalidNumberPayload,
    InvalidTextPayload,
    InvalidErrorPayload,
    MissingReferencePayload,
    MissingRichPayload,
    MissingArrayPayload,
    InvalidArrayPayload,
    ArrayShapeMismatch,
}

impl ComparisonValueParseError {
    fn as_message(self) -> &'static str {
        match self {
            Self::MissingDeclaredKind => "missing declared comparison_value kind",
            Self::UnsupportedDeclaredKind => "unsupported comparison_value kind",
            Self::InvalidLogicalPayload => "invalid logical payload",
            Self::InvalidNumberPayload => "invalid number payload",
            Self::InvalidTextPayload => "invalid text payload",
            Self::InvalidErrorPayload => "invalid error payload",
            Self::MissingReferencePayload => "missing reference payload",
            Self::MissingRichPayload => "missing rich payload",
            Self::MissingArrayPayload => "missing array payload",
            Self::InvalidArrayPayload => "invalid array payload",
            Self::ArrayShapeMismatch => "declared array shape does not match payload",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypedComparisonValue {
    Blank,
    Logical(bool),
    Number(u64),
    Text(String),
    Error(String),
    Reference(serde_json::Value),
    Rich(serde_json::Value),
    Array(TypedArrayValue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TypedArrayValue {
    rows: Option<usize>,
    cols: Option<usize>,
    items: Vec<TypedComparisonValue>,
}

fn parse_typed_comparison_value(
    value: &serde_json::Value,
) -> Result<TypedComparisonValue, ComparisonValueParseError> {
    match value {
        serde_json::Value::Null => Ok(TypedComparisonValue::Blank),
        serde_json::Value::Bool(boolean) => Ok(TypedComparisonValue::Logical(*boolean)),
        serde_json::Value::Number(number) => parse_json_number(number)
            .map(TypedComparisonValue::Number)
            .ok_or(ComparisonValueParseError::InvalidNumberPayload),
        serde_json::Value::String(text) => Ok(TypedComparisonValue::Text(text.clone())),
        serde_json::Value::Array(items) => {
            parse_nested_array(items).map(TypedComparisonValue::Array)
        }
        serde_json::Value::Object(object) => parse_typed_object(object),
    }
}

fn parse_typed_object(
    object: &serde_json::Map<String, serde_json::Value>,
) -> Result<TypedComparisonValue, ComparisonValueParseError> {
    let kind = object
        .get("value_kind")
        .or_else(|| object.get("kind"))
        .or_else(|| object.get("type"))
        .and_then(serde_json::Value::as_str)
        .map(|value| value.to_ascii_lowercase())
        .ok_or(ComparisonValueParseError::MissingDeclaredKind)?;

    match kind.as_str() {
        "blank" | "blanklike" | "empty" => Ok(TypedComparisonValue::Blank),
        "logical" | "bool" | "boolean" => extract_logical_value(object)
            .map(TypedComparisonValue::Logical)
            .ok_or(ComparisonValueParseError::InvalidLogicalPayload),
        "number" => extract_number_value(object)
            .map(TypedComparisonValue::Number)
            .ok_or(ComparisonValueParseError::InvalidNumberPayload),
        "text" | "string" => extract_text_value(object)
            .map(TypedComparisonValue::Text)
            .ok_or(ComparisonValueParseError::InvalidTextPayload),
        "error" => extract_error_value(object)
            .map(TypedComparisonValue::Error)
            .ok_or(ComparisonValueParseError::InvalidErrorPayload),
        "reference" => extract_payload_value(object)
            .cloned()
            .map(TypedComparisonValue::Reference)
            .ok_or(ComparisonValueParseError::MissingReferencePayload),
        "rich" | "rich_value" | "richvalue" => extract_payload_value(object)
            .cloned()
            .map(TypedComparisonValue::Rich)
            .ok_or(ComparisonValueParseError::MissingRichPayload),
        "array" => parse_array_object(object).map(TypedComparisonValue::Array),
        _ => Err(ComparisonValueParseError::UnsupportedDeclaredKind),
    }
}

fn parse_array_object(
    object: &serde_json::Map<String, serde_json::Value>,
) -> Result<TypedArrayValue, ComparisonValueParseError> {
    let rows = extract_dimension(object, &["rows", "row_count"]);
    let cols = extract_dimension(object, &["cols", "columns", "col_count"]);
    let payload =
        extract_payload_value(object).ok_or(ComparisonValueParseError::MissingArrayPayload)?;

    let (derived_rows, derived_cols, items) = match payload {
        serde_json::Value::Array(items) => {
            if items.iter().all(serde_json::Value::is_array) {
                let flattened = flatten_nested_array(items)?;
                let derived_cols = items
                    .first()
                    .and_then(serde_json::Value::as_array)
                    .map(std::vec::Vec::len)
                    .unwrap_or(0);
                (items.len(), derived_cols, flattened)
            } else {
                (
                    1,
                    items.len(),
                    items
                        .iter()
                        .map(parse_typed_comparison_value)
                        .collect::<Result<Vec<_>, _>>()?,
                )
            }
        }
        serde_json::Value::Object(nested) => {
            let nested_payload = extract_payload_value(nested)
                .ok_or(ComparisonValueParseError::MissingArrayPayload)?;
            match nested_payload {
                serde_json::Value::Array(items) => {
                    if items.iter().all(serde_json::Value::is_array) {
                        let flattened = flatten_nested_array(items)?;
                        let derived_cols = items
                            .first()
                            .and_then(serde_json::Value::as_array)
                            .map(std::vec::Vec::len)
                            .unwrap_or(0);
                        (items.len(), derived_cols, flattened)
                    } else {
                        (
                            1,
                            items.len(),
                            items
                                .iter()
                                .map(parse_typed_comparison_value)
                                .collect::<Result<Vec<_>, _>>()?,
                        )
                    }
                }
                _ => return Err(ComparisonValueParseError::InvalidArrayPayload),
            }
        }
        _ => return Err(ComparisonValueParseError::InvalidArrayPayload),
    };

    if rows.is_some_and(|declared_rows| declared_rows != derived_rows)
        || cols.is_some_and(|declared_cols| declared_cols != derived_cols)
    {
        return Err(ComparisonValueParseError::ArrayShapeMismatch);
    }

    Ok(TypedArrayValue {
        rows: Some(rows.unwrap_or(derived_rows)),
        cols: Some(cols.unwrap_or(derived_cols)),
        items,
    })
}

fn extract_dimension(
    object: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<usize> {
    for key in keys {
        if let Some(value) = object.get(*key)
            && let Some(number) = value.as_u64()
        {
            return usize::try_from(number).ok();
        }
    }
    object
        .get("shape")
        .and_then(serde_json::Value::as_object)
        .and_then(|shape| {
            keys.iter()
                .find_map(|key| shape.get(*key).and_then(serde_json::Value::as_u64))
        })
        .and_then(|number| usize::try_from(number).ok())
}

fn extract_payload_value(
    object: &serde_json::Map<String, serde_json::Value>,
) -> Option<&serde_json::Value> {
    ["payload", "value", "items", "elements", "cells", "values"]
        .into_iter()
        .find_map(|key| object.get(key))
}

fn extract_logical_value(object: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
    if let Some(value) = object.get("logical").and_then(serde_json::Value::as_bool) {
        return Some(value);
    }
    match extract_payload_value(object)? {
        serde_json::Value::Bool(value) => Some(*value),
        serde_json::Value::String(value) if value.eq_ignore_ascii_case("true") => Some(true),
        serde_json::Value::String(value) if value.eq_ignore_ascii_case("false") => Some(false),
        _ => None,
    }
}

fn extract_number_value(object: &serde_json::Map<String, serde_json::Value>) -> Option<u64> {
    object
        .get("number")
        .or_else(|| object.get("numeric_value"))
        .or_else(|| object.get("published_value"))
        .or_else(|| extract_payload_value(object))
        .and_then(parse_number_value)
}

fn extract_text_value(object: &serde_json::Map<String, serde_json::Value>) -> Option<String> {
    if let Some(value) = object.get("text").and_then(serde_json::Value::as_str) {
        return Some(value.to_string());
    }
    if let Some(value) = object
        .get("utf16_code_units")
        .and_then(decode_utf16_code_units)
    {
        return Some(value);
    }
    match extract_payload_value(object)? {
        serde_json::Value::String(value) => Some(value.clone()),
        serde_json::Value::Object(payload) => payload
            .get("utf16_code_units")
            .and_then(decode_utf16_code_units),
        _ => None,
    }
}

fn extract_error_value(object: &serde_json::Map<String, serde_json::Value>) -> Option<String> {
    object
        .get("error_kind")
        .and_then(serde_json::Value::as_str)
        .or_else(|| {
            object
                .get("worksheet_error_code")
                .and_then(serde_json::Value::as_str)
        })
        .or_else(|| object.get("error_code").and_then(serde_json::Value::as_str))
        .or_else(|| object.get("code").and_then(serde_json::Value::as_str))
        .or_else(|| extract_payload_value(object).and_then(serde_json::Value::as_str))
        .map(normalize_error_code)
}

fn parse_number_value(value: &serde_json::Value) -> Option<u64> {
    match value {
        serde_json::Value::Number(number) => parse_json_number(number),
        serde_json::Value::String(number) => number.parse::<f64>().ok().map(f64::to_bits),
        serde_json::Value::Object(object) => {
            extract_payload_value(object).and_then(parse_number_value)
        }
        _ => None,
    }
}

fn parse_json_number(number: &serde_json::Number) -> Option<u64> {
    number.as_f64().map(f64::to_bits)
}

fn decode_utf16_code_units(value: &serde_json::Value) -> Option<String> {
    let code_units = value.as_array()?;
    let mut units = Vec::with_capacity(code_units.len());
    for code_unit in code_units {
        let code_unit = code_unit.as_u64()?;
        let code_unit = u16::try_from(code_unit).ok()?;
        units.push(code_unit);
    }

    String::from_utf16(&units).ok()
}

fn normalize_error_code(value: &str) -> String {
    let compact = value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_uppercase();

    match compact.as_str() {
        "NULL" => "Null".to_string(),
        "DIV0" => "Div0".to_string(),
        "VALUE" => "Value".to_string(),
        "REF" => "Ref".to_string(),
        "NAME" => "Name".to_string(),
        "NUM" => "Num".to_string(),
        "NA" => "NA".to_string(),
        "GETTINGDATA" => "GettingData".to_string(),
        "SPILL" => "Spill".to_string(),
        "CALC" => "Calc".to_string(),
        "FIELD" => "Field".to_string(),
        "BLOCKED" => "Blocked".to_string(),
        "UNKNOWN" => "Unknown".to_string(),
        _ => value.to_string(),
    }
}

fn parse_nested_array(
    items: &[serde_json::Value],
) -> Result<TypedArrayValue, ComparisonValueParseError> {
    let is_matrix = items.iter().all(serde_json::Value::is_array);
    if !is_matrix {
        return Ok(TypedArrayValue {
            rows: Some(1),
            cols: Some(items.len()),
            items: items
                .iter()
                .map(parse_typed_comparison_value)
                .collect::<Result<Vec<_>, _>>()?,
        });
    }

    Ok(TypedArrayValue {
        rows: Some(items.len()),
        cols: Some(
            items
                .first()
                .and_then(serde_json::Value::as_array)
                .map(std::vec::Vec::len)
                .unwrap_or(0),
        ),
        items: flatten_nested_array(items)?,
    })
}

fn flatten_nested_array(
    items: &[serde_json::Value],
) -> Result<Vec<TypedComparisonValue>, ComparisonValueParseError> {
    let cols = items
        .first()
        .and_then(serde_json::Value::as_array)
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let mut flattened = Vec::new();
    for row in items {
        let row = row
            .as_array()
            .ok_or(ComparisonValueParseError::InvalidArrayPayload)?;
        if row.len() != cols {
            return Err(ComparisonValueParseError::ArrayShapeMismatch);
        }
        for item in row {
            flattened.push(parse_typed_comparison_value(item)?);
        }
    }
    Ok(flattened)
}

#[cfg(test)]
mod tests {
    use oxreplay_abstractions::{LaneId, SeverityClass};
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
                    view_family: "comparison_value".to_string(),
                    value: serde_json::json!({
                        "value_kind": "number",
                        "worksheet_value_class": "scalar",
                        "payload": "6"
                    }),
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
                    view_family: "comparison_value".to_string(),
                    value: serde_json::json!({
                        "value_kind": "number",
                        "worksheet_value_class": "scalar",
                        "payload": "6.0"
                    }),
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
    fn classifies_typed_comparison_value_divergence_explicitly() {
        let left = scenario(
            "left",
            vec![
                ReplayComparisonView {
                    view_family: "comparison_value".to_string(),
                    value: serde_json::json!({
                        "value_kind": "number",
                        "worksheet_value_class": "scalar",
                        "payload": "6"
                    }),
                },
                ReplayComparisonView {
                    view_family: "effective_display_text".to_string(),
                    value: serde_json::Value::String("$6.00".to_string()),
                },
            ],
        );
        let right = scenario(
            "right",
            vec![
                ReplayComparisonView {
                    view_family: "comparison_value".to_string(),
                    value: serde_json::json!({
                        "value_kind": "number",
                        "worksheet_value_class": "scalar",
                        "payload": "7"
                    }),
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
            MismatchKind::ComparisonValue
        );
        assert_eq!(
            report.mismatches[0].view_family.as_deref(),
            Some("worksheet_comparison_value")
        );
        assert_eq!(
            report.mismatches[0].detail.as_deref(),
            Some("typed comparison values diverged")
        );
    }

    #[test]
    fn classifies_one_ulp_numeric_divergence_without_relaxing_equivalence() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "number",
                    "payload": "14.206699082890463"
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "number",
                    "payload": "14.206699082890465"
                }),
            }],
        );

        let report = diff_summary(&left, &right);
        let detail = report.mismatches[0]
            .detail
            .as_deref()
            .expect("comparison_value mismatch detail");

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].mismatch_kind,
            MismatchKind::ComparisonValue
        );
        assert!(detail.starts_with("near_equal_last_bit:"));
        assert!(detail.contains("left=14.206699082890463"));
        assert!(detail.contains("right=14.206699082890465"));
    }

    #[test]
    fn classifies_tiny_numeric_zero_residue_without_relaxing_equivalence() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "number",
                    "payload": "5.551115123125783e-17"
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "number",
                    "payload": "0.0"
                }),
            }],
        );

        let report = diff_summary(&left, &right);
        let detail = report.mismatches[0]
            .detail
            .as_deref()
            .expect("comparison_value mismatch detail");

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].mismatch_kind,
            MismatchKind::ComparisonValue
        );
        assert!(detail.starts_with("near_zero_residue:"));
        assert!(detail.contains("left=5.551115123125783e-17"));
        assert!(detail.contains("right=0.0"));
    }

    #[test]
    fn treats_aligned_comparison_value_and_effective_display_text_as_equivalent() {
        let left = scenario(
            "left",
            vec![
                ReplayComparisonView {
                    view_family: "comparison_value".to_string(),
                    value: serde_json::json!({
                        "value_kind": "number",
                        "worksheet_value_class": "scalar",
                        "payload": "6"
                    }),
                },
                ReplayComparisonView {
                    view_family: "effective_display_text".to_string(),
                    value: serde_json::Value::String("$6.00".to_string()),
                },
            ],
        );
        let right = scenario(
            "right",
            vec![
                ReplayComparisonView {
                    view_family: "comparison_value".to_string(),
                    value: serde_json::json!({
                        "kind": "number",
                        "worksheet_value_class": "scalar",
                        "value": "6.0"
                    }),
                },
                ReplayComparisonView {
                    view_family: "effective_display_text".to_string(),
                    value: serde_json::Value::String("$6.00".to_string()),
                },
            ],
        );

        let report = diff_summary(&left, &right);

        assert!(report.equivalent);
        assert!(report.mismatches.is_empty());
    }

    #[test]
    fn treats_aligned_text_string_and_utf16_payloads_as_equivalent() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "kind": "text",
                    "text": "c"
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "kind": "text",
                    "utf16_code_units": [99]
                }),
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(report.equivalent);
        assert!(report.mismatches.is_empty());
    }

    #[test]
    fn treats_aligned_error_code_and_worksheet_error_code_as_equivalent() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "kind": "error",
                    "code": "NA"
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "kind": "error",
                    "worksheet_error_code": "na"
                }),
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(report.equivalent);
        assert!(report.mismatches.is_empty());
    }

    #[test]
    fn treats_numeric_string_forms_as_equal_for_comparison_value() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "number",
                    "worksheet_value_class": "scalar",
                    "payload": "6"
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "number",
                    "worksheet_value_class": "scalar",
                    "payload": "6.0"
                }),
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(report.equivalent);
        assert!(report.mismatches.is_empty());
    }

    #[test]
    fn compares_comparison_value_arrays_by_shape_and_recursive_content() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "array",
                    "rows": 2,
                    "cols": 2,
                    "payload": [
                        [
                            { "value_kind": "number", "payload": "1" },
                            { "value_kind": "number", "payload": "2.0" }
                        ],
                        [
                            { "value_kind": "text", "payload": "A" },
                            { "value_kind": "logical", "payload": true }
                        ]
                    ]
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "array",
                    "shape": { "rows": 2, "cols": 2 },
                    "values": [
                        [
                            { "value_kind": "number", "payload": "1.0" },
                            { "value_kind": "number", "payload": "2" }
                        ],
                        [
                            { "value_kind": "text", "payload": "A" },
                            { "value_kind": "logical", "payload": true }
                        ]
                    ]
                }),
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(report.equivalent);
        assert!(report.mismatches.is_empty());
    }

    #[test]
    fn does_not_misclassify_reference_envelopes_as_arrays() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "reference",
                    "payload": ["Sheet1!A1", "Sheet1!A2"]
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "array",
                    "payload": ["Sheet1!A1", "Sheet1!A2"]
                }),
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].detail.as_deref(),
            Some("typed comparison values diverged")
        );
    }

    #[test]
    fn rejects_identical_malformed_typed_number_envelopes_as_seam_drift() {
        let malformed = serde_json::json!({
            "value_kind": "number",
            "payload": "not-a-number"
        });
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: malformed.clone(),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: malformed,
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].detail.as_deref(),
            Some(
                "comparison_value envelopes are outside the admitted local seam (left: invalid number payload; right: invalid number payload)"
            )
        );
    }

    #[test]
    fn surfaces_mixed_parse_comparison_value_seam_drift() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "number",
                    "payload": "6"
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "number",
                    "payload": "not-a-number"
                }),
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].detail.as_deref(),
            Some(
                "right comparison_value envelope is outside the admitted local seam: invalid number payload"
            )
        );
    }

    #[test]
    fn rejects_declared_array_shape_mismatches_as_seam_drift() {
        let malformed = serde_json::json!({
            "value_kind": "array",
            "rows": 2,
            "cols": 2,
            "payload": [
                [
                    { "value_kind": "number", "payload": "1" }
                ]
            ]
        });
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: malformed.clone(),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: malformed,
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].detail.as_deref(),
            Some(
                "comparison_value envelopes are outside the admitted local seam (left: declared array shape does not match payload; right: declared array shape does not match payload)"
            )
        );
    }

    #[test]
    fn ignores_optional_visible_value_text_when_only_one_side_publishes_it() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "visible_value_text".to_string(),
                value: serde_json::Value::String("preview".to_string()),
            }],
        );
        let right = scenario("right", vec![]);

        let report = diff_summary(&left, &right);

        assert!(report.equivalent);
        assert!(report.mismatches.is_empty());
    }

    #[test]
    fn classifies_visible_value_text_divergence_when_both_sides_publish_it() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "visible_value_text".to_string(),
                value: serde_json::Value::String("6".to_string()),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "visible_value_text".to_string(),
                value: serde_json::Value::String("$6.00".to_string()),
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].mismatch_kind,
            MismatchKind::VisibleValue
        );
        assert_eq!(
            report.mismatches[0].view_family.as_deref(),
            Some("visible_value_text")
        );
        assert_eq!(
            report.mismatches[0].equivalence_policy_id.as_deref(),
            Some("visible_value_text_exact")
        );
        assert_eq!(report.mismatches[0].required, Some(false));
    }

    #[test]
    fn treats_typed_outcome_equivalence_as_cross_stage_not_value_equality() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "execution_outcome".to_string(),
                value: serde_json::json!({
                    "outcome_kind": "rejected",
                    "outcome_stage": "authoring",
                    "class_id": "input_rejected",
                    "lane_reason_code": "excel_programmatic_authoring_rejected",
                    "human_summary": "Excel rejected the authored formula"
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "execution_outcome".to_string(),
                value: serde_json::json!({
                    "outcome_kind": "rejected",
                    "outcome_stage": "bind",
                    "class_id": "input_rejected",
                    "lane_reason_code": "oxfml_bind_boundary_rejected",
                    "human_summary": "Bind boundary rejected the candidate"
                }),
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(report.equivalent);
        assert!(report.mismatches.is_empty());
    }

    #[test]
    fn rejects_non_equivalent_typed_outcome_classes() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "execution_outcome".to_string(),
                value: serde_json::json!({
                    "outcome_kind": "rejected",
                    "outcome_stage": "authoring",
                    "class_id": "input_rejected",
                    "lane_reason_code": "excel_programmatic_authoring_rejected"
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "execution_outcome".to_string(),
                value: serde_json::json!({
                    "outcome_kind": "executed",
                    "outcome_stage": "execution",
                    "class_id": "value_published",
                    "lane_reason_code": "oxfml_execution_succeeded"
                }),
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].mismatch_kind,
            MismatchKind::OutcomeValue
        );
        assert_eq!(
            report.mismatches[0].view_family.as_deref(),
            Some("execution_outcome")
        );
        assert_eq!(
            report.mismatches[0].equivalence_policy_id.as_deref(),
            Some("typed_outcome_class")
        );
        assert!(
            report.mismatches[0]
                .detail
                .as_deref()
                .is_some_and(|detail| detail.contains("left_stage=`authoring`")
                    && detail.contains("right_stage=`execution`")
                    && detail.contains("left_class_id=`input_rejected`")
                    && detail.contains("right_class_id=`value_published`"))
        );
    }

    #[test]
    fn rejects_execution_outcome_without_explicit_stage() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "execution_outcome".to_string(),
                value: serde_json::json!({
                    "outcome_kind": "rejected",
                    "class_id": "input_rejected",
                    "lane_reason_code": "excel_programmatic_authoring_rejected"
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "execution_outcome".to_string(),
                value: serde_json::json!({
                    "outcome_kind": "rejected",
                    "outcome_stage": "bind",
                    "class_id": "input_rejected",
                    "lane_reason_code": "oxfml_bind_boundary_rejected"
                }),
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].mismatch_kind,
            MismatchKind::OutcomeValue
        );
        assert_eq!(
            report.mismatches[0].detail.as_deref(),
            Some(
                "left execution_outcome envelope is outside the admitted local seam: missing outcome_stage"
            )
        );
    }

    #[test]
    fn rejects_legacy_outcome_family_as_typed_outcome_seam_drift() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "authoring_outcome".to_string(),
                value: serde_json::json!({
                    "outcome_kind": "rejected",
                    "outcome_stage": "authoring",
                    "class_id": "input_rejected",
                    "lane_reason_code": "legacy_authoring"
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![ReplayComparisonView {
                view_family: "execution_outcome".to_string(),
                value: serde_json::json!({
                    "outcome_kind": "rejected",
                    "outcome_stage": "authoring",
                    "class_id": "input_rejected",
                    "lane_reason_code": "normalized_execution"
                }),
            }],
        );

        let report = diff_summary(&left, &right);

        assert!(!report.equivalent);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(
            report.mismatches[0].mismatch_kind,
            MismatchKind::OutcomeValue
        );
        assert_eq!(
            report.mismatches[0].severity,
            SeverityClass::Instrumentation
        );
        assert_eq!(
            report.mismatches[0].view_family.as_deref(),
            Some("authoring_outcome")
        );
        assert_eq!(report.mismatches[0].equivalence_policy_id, None);
        assert_eq!(report.mismatches[0].required, Some(true));
        assert_eq!(
            report.mismatches[0].detail.as_deref(),
            Some(
                "legacy typed outcome family `authoring_outcome` is not admitted on the typed execution_outcome comparison path; publish `execution_outcome` with explicit `outcome_kind`, `outcome_stage`, `class_id`, and optional `lane_reason_code`"
            )
        );
    }

    #[test]
    fn classifies_missing_view_family_as_projection_gap() {
        let left = scenario(
            "left",
            vec![ReplayComparisonView {
                view_family: "comparison_value".to_string(),
                value: serde_json::json!({
                    "value_kind": "number",
                    "worksheet_value_class": "scalar",
                    "payload": "6"
                }),
            }],
        );
        let right = scenario(
            "right",
            vec![
                ReplayComparisonView {
                    view_family: "comparison_value".to_string(),
                    value: serde_json::json!({
                        "value_kind": "number",
                        "worksheet_value_class": "scalar",
                        "payload": "6.0"
                    }),
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
