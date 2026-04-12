#![forbid(unsafe_code)]

use serde_json::{Map, Value};

use oxreplay_bundle::{ValidationStatus, render_text_report, validate_bundle_at_path};
use oxreplay_conformance::{load_manifest_from_path, validate_manifest};
use oxreplay_core::{
    ReplayScenario, load_oxcalc_tracecalc_projection, load_oxfml_v1_replay_projection,
    load_replay_scenario_from_path,
};
use oxreplay_diff::diff_summary;
use oxreplay_distill::{ReductionOutcome, ReplayPreservationPredicate, planned_reduction};
use oxreplay_explain::explain_diff;
use oxreplay_governance::{WitnessLifecycleRecord, WitnessLifecycleState, transition_lifecycle};

const HELP: &str = "\
dna-recalc <command> [options]

Commands:
  validate-bundle
  replay
  diff
  explain
  distill
  validate-adapter
  witness-state
  pack-export
";

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next();

    match command.as_deref() {
        None | Some("help") | Some("--help") | Some("-h") => {
            print!("{HELP}");
        }
        Some("validate-bundle") => {
            let exit_code = run_validate_bundle(args.collect());
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        }
        Some("validate-adapter") => {
            let exit_code = run_validate_adapter(args.collect());
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        }
        Some("replay") => {
            let exit_code = run_replay(args.collect());
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        }
        Some("diff") => {
            let exit_code = run_diff(args.collect());
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        }
        Some("explain") => {
            let exit_code = run_explain(args.collect());
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        }
        Some("distill") => {
            let exit_code = run_distill(args.collect());
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        }
        Some("witness-state") => {
            let exit_code = run_witness_state(args.collect());
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        }
        Some("pack-export") => {
            let exit_code = run_pack_export(args.collect());
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        }
        Some(other) => {
            eprintln!("unknown command: {other}\n\n{HELP}");
            std::process::exit(2);
        }
    }
}

fn run_validate_bundle(args: Vec<String>) -> i32 {
    let mut bundle_path = None;
    let mut batch_index_path = None;
    let mut selection = BatchSelection::All;
    let mut format = String::from("text");

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--bundle" => bundle_path = iter.next(),
            "--batch-index" => batch_index_path = iter.next(),
            "--selection" => {
                let Some(value) = iter.next() else {
                    eprintln!("missing value for --selection");
                    return 2;
                };
                let Some(parsed) = parse_batch_selection(&value) else {
                    eprintln!("unsupported selection: {value}");
                    return 2;
                };
                selection = parsed;
            }
            "--format" => {
                if let Some(value) = iter.next() {
                    format = value;
                } else {
                    eprintln!("missing value for --format");
                    return 2;
                }
            }
            other => {
                eprintln!("unknown validate-bundle argument: {other}");
                return 2;
            }
        }
    }

    if bundle_path.is_some() && batch_index_path.is_some() {
        eprintln!("validate-bundle accepts either --bundle <path> or --batch-index <path>");
        return 2;
    }

    if let Some(batch_index_path) = batch_index_path {
        let batch = match load_batch_index(&batch_index_path) {
            Ok(batch) => batch,
            Err(code) => return code,
        };
        let selected_cases = select_batch_cases(&batch.cases, selection);
        let mut case_reports = Vec::new();
        let mut invalid_count = 0usize;

        for case in selected_cases {
            let Some(manifest_path) = case.oxreplay_manifest_path.as_deref() else {
                eprintln!(
                    "batch case `{}` is missing `oxreplay_manifest_path`",
                    case.case_id
                );
                return 4;
            };
            let report = match validate_bundle_at_path(manifest_path) {
                Ok(report) => report,
                Err(error) => {
                    eprintln!("batch case `{}`: {error}", case.case_id);
                    return 4;
                }
            };
            if report.status == ValidationStatus::Invalid {
                invalid_count += 1;
            }
            case_reports.push(serde_json::json!({
                "case_id": case.case_id,
                "status": case.status,
                "error": case.error,
                "output_dir": case.output_dir,
                "capture_path": case.capture_path,
                "oxreplay_manifest_path": case.oxreplay_manifest_path,
                "normalized_replay_path": case.normalized_replay_path,
                "validation": report,
            }));
        }

        let status = if invalid_count == 0 {
            ValidationStatus::Valid
        } else {
            ValidationStatus::Invalid
        };
        let output = serde_json::json!({
            "batch_id": batch.batch_id,
            "selection": selection.as_str(),
            "status": status,
            "case_count": case_reports.len(),
            "invalid_case_count": invalid_count,
            "cases": case_reports,
        });

        match format.as_str() {
            "json" => match serde_json::to_string_pretty(&output) {
                Ok(text) => println!("{text}"),
                Err(error) => {
                    eprintln!("failed to serialize batch validation output: {error}");
                    return 4;
                }
            },
            "text" => {
                println!("status: {:?}", status);
                println!("batch_id: {}", batch.batch_id);
                println!("selection: {}", selection.as_str());
                println!("case_count: {}", case_reports.len());
                println!("invalid_case_count: {}", invalid_count);
            }
            _ => {
                eprintln!("unsupported format: {format}");
                return 2;
            }
        }

        return match status {
            ValidationStatus::Valid => 0,
            ValidationStatus::Invalid => 1,
        };
    }

    let Some(bundle_path) = bundle_path else {
        eprintln!("validate-bundle requires --bundle <path> or --batch-index <path>");
        return 2;
    };

    let report = match validate_bundle_at_path(&bundle_path) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("{error}");
            return 4;
        }
    };

    match format.as_str() {
        "json" => match serde_json::to_string_pretty(&report) {
            Ok(text) => println!("{text}"),
            Err(error) => {
                eprintln!("failed to serialize report: {error}");
                return 4;
            }
        },
        "text" => {
            print!("{}", render_text_report(&report));
        }
        _ => {
            eprintln!("unsupported format: {format}");
            return 2;
        }
    }

    match report.status {
        ValidationStatus::Valid => 0,
        ValidationStatus::Invalid => 1,
    }
}

fn run_validate_adapter(args: Vec<String>) -> i32 {
    let mut adapter_path = None;
    let mut format = String::from("text");

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--adapter" => adapter_path = iter.next(),
            "--format" => {
                if let Some(value) = iter.next() {
                    format = value;
                } else {
                    eprintln!("missing value for --format");
                    return 2;
                }
            }
            other => {
                eprintln!("unknown validate-adapter argument: {other}");
                return 2;
            }
        }
    }

    let Some(adapter_path) = adapter_path else {
        eprintln!("validate-adapter requires --adapter <path>");
        return 2;
    };

    let manifest = match load_manifest_from_path(&adapter_path) {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{error}");
            return 4;
        }
    };

    let report = validate_manifest(&manifest);

    match format.as_str() {
        "json" => match serde_json::to_string_pretty(&report) {
            Ok(text) => println!("{text}"),
            Err(error) => {
                eprintln!("failed to serialize report: {error}");
                return 4;
            }
        },
        "text" => {
            println!("accepted: {}", report.accepted);
            println!("adapter_id: {}", report.adapter_id.0);
            println!(
                "normalized_claimed_capabilities: {}",
                report.normalized_claimed_capabilities.len()
            );
            if report.diagnostics.is_empty() {
                println!("diagnostics: none");
            } else {
                println!("diagnostics:");
                for diagnostic in &report.diagnostics {
                    println!("  - {diagnostic}");
                }
            }
        }
        _ => {
            eprintln!("unsupported format: {format}");
            return 2;
        }
    }

    if report.accepted { 0 } else { 1 }
}

fn run_replay(args: Vec<String>) -> i32 {
    let scenario = match parse_replay_input(args) {
        Ok(scenario) => scenario,
        Err(code) => return code,
    };

    match serde_json::to_string_pretty(&scenario) {
        Ok(text) => println!("{text}"),
        Err(error) => {
            eprintln!("failed to serialize replay output: {error}");
            return 4;
        }
    }

    0
}

fn run_diff(args: Vec<String>) -> i32 {
    let batch_request = match parse_batch_comparison_request(&args, "diff") {
        Ok(request) => request,
        Err(code) => return code,
    };
    if let Some(batch_request) = batch_request {
        let constant = match load_scenario_by_kind(
            &batch_request.constant_path,
            &batch_request.constant_kind,
        ) {
            Ok(scenario) => scenario,
            Err(code) => return code,
        };

        let mut equivalent = true;
        let mut cases = Vec::new();
        for case in batch_request.cases {
            let replay_path = match case.normalized_replay_path.as_deref() {
                Some(path) => path,
                None => {
                    eprintln!(
                        "batch case `{}` is missing `normalized_replay_path`",
                        case.case_id
                    );
                    return 4;
                }
            };
            let batch_scenario = match load_scenario_by_kind(replay_path, "normalized-replay") {
                Ok(scenario) => scenario,
                Err(code) => return code,
            };
            let report = match batch_request.batch_side {
                BatchSide::Left => diff_summary(&batch_scenario, &constant),
                BatchSide::Right => diff_summary(&constant, &batch_scenario),
            };
            if !report.equivalent {
                equivalent = false;
            }
            cases.push(serde_json::json!({
                "case_id": case.case_id,
                "status": case.status,
                "error": case.error,
                "output_dir": case.output_dir,
                "capture_path": case.capture_path,
                "oxreplay_manifest_path": case.oxreplay_manifest_path,
                "normalized_replay_path": case.normalized_replay_path,
                "diff": report,
            }));
        }

        let output = serde_json::json!({
            "batch_id": batch_request.batch_id,
            "selection": batch_request.selection.as_str(),
            "batch_side": batch_request.batch_side.as_str(),
            "constant_path": batch_request.constant_path,
            "constant_kind": batch_request.constant_kind,
            "equivalent": equivalent,
            "case_count": cases.len(),
            "cases": cases,
        });

        match serde_json::to_string_pretty(&output) {
            Ok(text) => println!("{text}"),
            Err(error) => {
                eprintln!("failed to serialize diff output: {error}");
                return 4;
            }
        }

        return if equivalent { 0 } else { 1 };
    }

    let parsed = match parse_diff_inputs(args) {
        Ok(parsed) => parsed,
        Err(code) => return code,
    };

    let report = diff_summary(&parsed.0, &parsed.1);

    match serde_json::to_string_pretty(&report) {
        Ok(text) => println!("{text}"),
        Err(error) => {
            eprintln!("failed to serialize diff output: {error}");
            return 4;
        }
    }

    if report.equivalent { 0 } else { 1 }
}

fn run_explain(args: Vec<String>) -> i32 {
    let batch_request = match parse_batch_comparison_request(&args, "explain") {
        Ok(request) => request,
        Err(code) => return code,
    };
    if let Some(batch_request) = batch_request {
        let constant = match load_scenario_by_kind(
            &batch_request.constant_path,
            &batch_request.constant_kind,
        ) {
            Ok(scenario) => scenario,
            Err(code) => return code,
        };

        let mut equivalent = true;
        let mut cases = Vec::new();
        for case in batch_request.cases {
            let replay_path = match case.normalized_replay_path.as_deref() {
                Some(path) => path,
                None => {
                    eprintln!(
                        "batch case `{}` is missing `normalized_replay_path`",
                        case.case_id
                    );
                    return 4;
                }
            };
            let batch_scenario = match load_scenario_by_kind(replay_path, "normalized-replay") {
                Ok(scenario) => scenario,
                Err(code) => return code,
            };
            let diff = match batch_request.batch_side {
                BatchSide::Left => diff_summary(&batch_scenario, &constant),
                BatchSide::Right => diff_summary(&constant, &batch_scenario),
            };
            let explain = explain_diff(&diff);
            if !diff.equivalent {
                equivalent = false;
            }
            cases.push(serde_json::json!({
                "case_id": case.case_id,
                "status": case.status,
                "error": case.error,
                "output_dir": case.output_dir,
                "capture_path": case.capture_path,
                "oxreplay_manifest_path": case.oxreplay_manifest_path,
                "normalized_replay_path": case.normalized_replay_path,
                "diff": diff,
                "explain": explain,
            }));
        }

        let output = serde_json::json!({
            "batch_id": batch_request.batch_id,
            "selection": batch_request.selection.as_str(),
            "batch_side": batch_request.batch_side.as_str(),
            "constant_path": batch_request.constant_path,
            "constant_kind": batch_request.constant_kind,
            "equivalent": equivalent,
            "case_count": cases.len(),
            "cases": cases,
        });

        match serde_json::to_string_pretty(&output) {
            Ok(text) => println!("{text}"),
            Err(error) => {
                eprintln!("failed to serialize explain output: {error}");
                return 4;
            }
        }

        return if equivalent { 0 } else { 1 };
    }

    let parsed = match parse_diff_inputs(args) {
        Ok(parsed) => parsed,
        Err(code) => return code,
    };

    let diff = diff_summary(&parsed.0, &parsed.1);
    let explain = explain_diff(&diff);

    match serde_json::to_string_pretty(&explain) {
        Ok(text) => println!("{text}"),
        Err(error) => {
            eprintln!("failed to serialize explain output: {error}");
            return 4;
        }
    }

    if diff.equivalent { 0 } else { 1 }
}

fn run_distill(args: Vec<String>) -> i32 {
    let mut bundle_path = None;
    let mut kind = None;
    let mut predicate_id = None;
    let mut predicate_description = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--bundle" => bundle_path = iter.next(),
            "--kind" => kind = iter.next(),
            "--predicate-id" => predicate_id = iter.next(),
            "--predicate-description" => predicate_description = iter.next(),
            other => {
                eprintln!("unknown distill argument: {other}");
                return 2;
            }
        }
    }

    let Some(bundle_path) = bundle_path else {
        eprintln!("distill requires --bundle <path>");
        return 2;
    };
    let Some(kind) = kind else {
        eprintln!(
            "distill requires --kind <oxcalc-tracecalc|oxfml-v1-replay-projection|normalized-replay>"
        );
        return 2;
    };
    let Some(predicate_id) = predicate_id else {
        eprintln!("distill requires --predicate-id <id>");
        return 2;
    };
    let Some(predicate_description) = predicate_description else {
        eprintln!("distill requires --predicate-description <text>");
        return 2;
    };

    let scenario = match load_scenario_by_kind(&bundle_path, &kind) {
        Ok(scenario) => scenario,
        Err(code) => return code,
    };

    let predicate = ReplayPreservationPredicate {
        predicate_id,
        description: predicate_description,
    };
    let manifest = planned_reduction(&scenario, predicate);

    match serde_json::to_string_pretty(&manifest) {
        Ok(text) => println!("{text}"),
        Err(error) => {
            eprintln!("failed to serialize reduction manifest: {error}");
            return 4;
        }
    }

    match manifest.outcome {
        ReductionOutcome::Stable => 0,
        ReductionOutcome::UnstablePredicate | ReductionOutcome::Quarantined(_) => 1,
    }
}

fn run_witness_state(args: Vec<String>) -> i32 {
    let mut record_path = None;
    let mut next_state = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--record" => record_path = iter.next(),
            "--next-state" => next_state = iter.next(),
            other => {
                eprintln!("unknown witness-state argument: {other}");
                return 2;
            }
        }
    }

    let Some(record_path) = record_path else {
        eprintln!("witness-state requires --record <path>");
        return 2;
    };

    let record_text = match std::fs::read_to_string(&record_path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("failed to read lifecycle record: {error}");
            return 4;
        }
    };

    let record: WitnessLifecycleRecord = match serde_json::from_str(&record_text) {
        Ok(record) => record,
        Err(error) => {
            eprintln!("failed to parse lifecycle record: {error}");
            return 4;
        }
    };

    if let Some(next_state) = next_state {
        let Some(parsed_state) = parse_lifecycle_state(&next_state) else {
            eprintln!("unsupported lifecycle state: {next_state}");
            return 2;
        };

        match transition_lifecycle(&record, parsed_state) {
            Ok(result) => match serde_json::to_string_pretty(&result) {
                Ok(text) => println!("{text}"),
                Err(error) => {
                    eprintln!("failed to serialize transition result: {error}");
                    return 4;
                }
            },
            Err(error) => {
                eprintln!("{error}");
                return 1;
            }
        }

        return 0;
    }

    match serde_json::to_string_pretty(&record) {
        Ok(text) => println!("{text}"),
        Err(error) => {
            eprintln!("failed to serialize lifecycle record: {error}");
            return 4;
        }
    }

    0
}

fn run_pack_export(args: Vec<String>) -> i32 {
    let mut bundle_path = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--bundle" => bundle_path = iter.next(),
            other => {
                eprintln!("unknown pack-export argument: {other}");
                return 2;
            }
        }
    }

    let Some(bundle_path) = bundle_path else {
        eprintln!("pack-export requires --bundle <path>");
        return 2;
    };

    let report = match validate_bundle_at_path(&bundle_path) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("{error}");
            return 4;
        }
    };

    let export = serde_json::json!({
        "command_id": "pack-export",
        "status": report.status,
        "bundle_id": report.bundle_id,
        "scenario_id": report.scenario_id,
        "pack_impact": report.pack_impact,
        "artifact_refs": [
            { "path": report.manifest_path }
        ]
    });

    match serde_json::to_string_pretty(&export) {
        Ok(text) => println!("{text}"),
        Err(error) => {
            eprintln!("failed to serialize pack-export output: {error}");
            return 4;
        }
    }

    match report.status {
        ValidationStatus::Valid => 0,
        ValidationStatus::Invalid => 1,
    }
}

fn parse_replay_input(args: Vec<String>) -> Result<ReplayScenario, i32> {
    let mut bundle_path = None;
    let mut kind = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--bundle" => bundle_path = iter.next(),
            "--kind" => kind = iter.next(),
            other => {
                eprintln!("unknown replay argument: {other}");
                return Err(2);
            }
        }
    }

    let Some(bundle_path) = bundle_path else {
        eprintln!("replay requires --bundle <path>");
        return Err(2);
    };
    let Some(kind) = kind else {
        eprintln!(
            "replay requires --kind <oxcalc-tracecalc|oxfml-v1-replay-projection|normalized-replay>"
        );
        return Err(2);
    };

    load_scenario_by_kind(&bundle_path, &kind)
}

fn parse_diff_inputs(args: Vec<String>) -> Result<(ReplayScenario, ReplayScenario), i32> {
    let mut left = None;
    let mut left_kind = None;
    let mut right = None;
    let mut right_kind = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--left" => left = iter.next(),
            "--left-kind" => left_kind = iter.next(),
            "--right" => right = iter.next(),
            "--right-kind" => right_kind = iter.next(),
            other => {
                eprintln!("unknown diff/explain argument: {other}");
                return Err(2);
            }
        }
    }

    let Some(left) = left else {
        eprintln!("diff/explain requires --left <path>");
        return Err(2);
    };
    let Some(left_kind) = left_kind else {
        eprintln!("diff/explain requires --left-kind <kind>");
        return Err(2);
    };
    let Some(right) = right else {
        eprintln!("diff/explain requires --right <path>");
        return Err(2);
    };
    let Some(right_kind) = right_kind else {
        eprintln!("diff/explain requires --right-kind <kind>");
        return Err(2);
    };

    let left = load_scenario_by_kind(&left, &left_kind)?;
    let right = load_scenario_by_kind(&right, &right_kind)?;
    Ok((left, right))
}

fn load_scenario_by_kind(path: &str, kind: &str) -> Result<ReplayScenario, i32> {
    match kind {
        "oxcalc-tracecalc" => load_oxcalc_tracecalc_projection(path).map_err(|error| {
            eprintln!("{error}");
            4
        }),
        "normalized-replay" => load_replay_scenario_from_path(path).map_err(|error| {
            eprintln!("{error}");
            4
        }),
        "oxfml-v1-replay-projection" => load_oxfml_v1_replay_projection(path).map_err(|error| {
            eprintln!("{error}");
            4
        }),
        other => {
            eprintln!("unsupported replay kind: {other}");
            Err(2)
        }
    }
}

fn parse_lifecycle_state(state: &str) -> Option<WitnessLifecycleState> {
    match state {
        "explanatory_only" => Some(WitnessLifecycleState::ExplanatoryOnly),
        "retained_local" => Some(WitnessLifecycleState::RetainedLocal),
        "retained_shared" => Some(WitnessLifecycleState::RetainedShared),
        "promoted_pack" => Some(WitnessLifecycleState::PromotedPack),
        "superseded" => Some(WitnessLifecycleState::Superseded),
        "quarantined" => Some(WitnessLifecycleState::Quarantined),
        "gc_eligible" => Some(WitnessLifecycleState::GcEligible),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BatchSelection {
    All,
    MismatchOnly,
}

impl BatchSelection {
    fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::MismatchOnly => "mismatch-only",
        }
    }
}

#[derive(Clone)]
struct BatchCaseIndexEntry {
    case_id: String,
    status: String,
    error: Option<String>,
    output_dir: Option<String>,
    capture_path: Option<String>,
    oxreplay_manifest_path: Option<String>,
    normalized_replay_path: Option<String>,
}

struct BatchIndex {
    batch_id: String,
    cases: Vec<BatchCaseIndexEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BatchSide {
    Left,
    Right,
}

impl BatchSide {
    fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

struct BatchComparisonRequest {
    batch_id: String,
    selection: BatchSelection,
    batch_side: BatchSide,
    constant_path: String,
    constant_kind: String,
    cases: Vec<BatchCaseIndexEntry>,
}

fn parse_batch_selection(value: &str) -> Option<BatchSelection> {
    match value {
        "all" => Some(BatchSelection::All),
        "mismatch-only" => Some(BatchSelection::MismatchOnly),
        _ => None,
    }
}

fn load_batch_index(path: &str) -> Result<BatchIndex, i32> {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("failed to read batch index `{path}`: {error}");
            return Err(4);
        }
    };
    let value: Value = match serde_json::from_str(&text) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("failed to parse batch index `{path}`: {error}");
            return Err(4);
        }
    };
    let Some(root) = value.as_object() else {
        eprintln!("batch index `{path}` must be a JSON object");
        return Err(4);
    };
    let Some(batch_id) = root.get("batch_id").and_then(Value::as_str) else {
        eprintln!("batch index `{path}` is missing `batch_id`");
        return Err(4);
    };
    let Some(case_values) = root.get("cases").and_then(Value::as_array) else {
        eprintln!("batch index `{path}` is missing `cases`");
        return Err(4);
    };

    let mut cases = Vec::new();
    for case_value in case_values {
        let Some(case) = case_value.as_object() else {
            eprintln!("batch index `{path}` contains a non-object case entry");
            return Err(4);
        };
        let Some(case_id) = case.get("case_id").and_then(Value::as_str) else {
            eprintln!("batch index `{path}` contains a case without `case_id`");
            return Err(4);
        };
        let Some(status) = case.get("status").and_then(Value::as_str) else {
            eprintln!("batch index `{path}` case `{case_id}` is missing `status`");
            return Err(4);
        };
        cases.push(BatchCaseIndexEntry {
            case_id: case_id.to_string(),
            status: status.to_string(),
            error: get_optional_string(case, "error"),
            output_dir: get_optional_string(case, "output_dir"),
            capture_path: get_optional_string(case, "capture_path"),
            oxreplay_manifest_path: get_optional_string(case, "oxreplay_manifest_path"),
            normalized_replay_path: get_optional_string(case, "normalized_replay_path"),
        });
    }

    Ok(BatchIndex {
        batch_id: batch_id.to_string(),
        cases,
    })
}

fn get_optional_string(object: &Map<String, Value>, key: &str) -> Option<String> {
    object
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn select_batch_cases(
    cases: &[BatchCaseIndexEntry],
    selection: BatchSelection,
) -> Vec<BatchCaseIndexEntry> {
    cases
        .iter()
        .filter(|case| match selection {
            BatchSelection::All => true,
            BatchSelection::MismatchOnly => {
                case.status.eq_ignore_ascii_case("mismatch")
                    || case.status.eq_ignore_ascii_case("mismatched")
            }
        })
        .cloned()
        .collect()
}

fn parse_batch_comparison_request(
    args: &[String],
    command_name: &str,
) -> Result<Option<BatchComparisonRequest>, i32> {
    let mut batch_index_path = None;
    let mut selection = BatchSelection::All;
    let mut left = None;
    let mut left_kind = None;
    let mut right = None;
    let mut right_kind = None;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--batch-index" => batch_index_path = iter.next().cloned(),
            "--selection" => {
                let Some(value) = iter.next() else {
                    eprintln!("missing value for --selection");
                    return Err(2);
                };
                let Some(parsed) = parse_batch_selection(value) else {
                    eprintln!("unsupported selection: {value}");
                    return Err(2);
                };
                selection = parsed;
            }
            "--left" => left = iter.next().cloned(),
            "--left-kind" => left_kind = iter.next().cloned(),
            "--right" => right = iter.next().cloned(),
            "--right-kind" => right_kind = iter.next().cloned(),
            _ => {}
        }
    }

    let Some(batch_index_path) = batch_index_path else {
        return Ok(None);
    };

    let batch = load_batch_index(&batch_index_path)?;
    let cases = select_batch_cases(&batch.cases, selection);

    if let (Some(constant_path), Some(constant_kind), None, None) = (
        left.clone(),
        left_kind.clone(),
        right.clone(),
        right_kind.clone(),
    ) {
        return Ok(Some(BatchComparisonRequest {
            batch_id: batch.batch_id,
            selection,
            batch_side: BatchSide::Right,
            constant_path,
            constant_kind,
            cases,
        }));
    }

    if let (None, None, Some(constant_path), Some(constant_kind)) = (
        left,
        left_kind,
        right.clone(),
        right_kind.clone(),
    ) {
        return Ok(Some(BatchComparisonRequest {
            batch_id: batch.batch_id,
            selection,
            batch_side: BatchSide::Left,
            constant_path,
            constant_kind,
            cases,
        }));
    }

    eprintln!(
        "{command_name} with --batch-index requires exactly one constant side: either --left/--left-kind or --right/--right-kind"
    );
    Err(2)
}

#[cfg(test)]
mod tests {
    use super::{BatchSelection, BatchSide, parse_batch_comparison_request, parse_batch_selection};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn parse_batch_selection_accepts_supported_values() {
        assert_eq!(parse_batch_selection("all"), Some(BatchSelection::All));
        assert_eq!(
            parse_batch_selection("mismatch-only"),
            Some(BatchSelection::MismatchOnly)
        );
        assert_eq!(parse_batch_selection("mismatch_only"), None);
    }

    #[test]
    fn parse_batch_request_uses_left_constant_side() {
        let batch_index_path = write_batch_index(
            "batch-parse-left",
            r#"{
  "batch_id": "batch-parse-left",
  "cases": [
    {
      "case_id": "case-001",
      "status": "matched",
      "error": null,
      "output_dir": "out/case-001",
      "capture_path": "out/case-001/capture.json",
      "oxreplay_manifest_path": "out/case-001/oxreplay-manifest.json",
      "normalized_replay_path": "out/case-001/normalized-replay.json"
    },
    {
      "case_id": "case-002",
      "status": "mismatch",
      "error": "value diverged",
      "output_dir": "out/case-002",
      "capture_path": "out/case-002/capture.json",
      "oxreplay_manifest_path": "out/case-002/oxreplay-manifest.json",
      "normalized_replay_path": "out/case-002/normalized-replay.json"
    }
  ]
}"#,
        );
        let args = vec![
            "--batch-index".to_string(),
            batch_index_path.display().to_string(),
            "--selection".to_string(),
            "mismatch-only".to_string(),
            "--left".to_string(),
            "left.json".to_string(),
            "--left-kind".to_string(),
            "normalized-replay".to_string(),
        ];

        let parsed = parse_batch_comparison_request(&args, "diff")
            .expect("parse should succeed")
            .expect("batch request should be detected");

        assert_eq!(parsed.batch_id, "batch-parse-left");
        assert_eq!(parsed.selection, BatchSelection::MismatchOnly);
        assert_eq!(parsed.batch_side, BatchSide::Right);
        assert_eq!(parsed.constant_path, "left.json");
        assert_eq!(parsed.constant_kind, "normalized-replay");
        assert_eq!(parsed.cases.len(), 1);
        assert_eq!(parsed.cases[0].case_id, "case-002");
        assert_eq!(
            parsed.cases[0].normalized_replay_path.as_deref(),
            Some("out/case-002/normalized-replay.json")
        );
    }

    #[test]
    fn parse_batch_request_uses_right_constant_side() {
        let batch_index_path = write_batch_index(
            "batch-parse-right",
            r#"{
  "batch_id": "batch-parse-right",
  "cases": [
    {
      "case_id": "case-010",
      "status": "matched",
      "error": null,
      "output_dir": "out/case-010",
      "capture_path": "out/case-010/capture.json",
      "oxreplay_manifest_path": "out/case-010/oxreplay-manifest.json",
      "normalized_replay_path": "out/case-010/normalized-replay.json"
    }
  ]
}"#,
        );
        let args = vec![
            "--batch-index".to_string(),
            batch_index_path.display().to_string(),
            "--right".to_string(),
            "right.json".to_string(),
            "--right-kind".to_string(),
            "oxfml-v1-replay-projection".to_string(),
        ];

        let parsed = parse_batch_comparison_request(&args, "explain")
            .expect("parse should succeed")
            .expect("batch request should be detected");

        assert_eq!(parsed.batch_id, "batch-parse-right");
        assert_eq!(parsed.selection, BatchSelection::All);
        assert_eq!(parsed.batch_side, BatchSide::Left);
        assert_eq!(parsed.constant_path, "right.json");
        assert_eq!(parsed.constant_kind, "oxfml-v1-replay-projection");
        assert_eq!(parsed.cases.len(), 1);
        assert_eq!(parsed.cases[0].case_id, "case-010");
    }

    #[test]
    fn parse_batch_request_rejects_both_constant_sides() {
        let batch_index_path = write_batch_index(
            "batch-parse-invalid",
            r#"{
  "batch_id": "batch-parse-invalid",
  "cases": [
    {
      "case_id": "case-999",
      "status": "mismatch",
      "error": null,
      "output_dir": "out/case-999",
      "capture_path": "out/case-999/capture.json",
      "oxreplay_manifest_path": "out/case-999/oxreplay-manifest.json",
      "normalized_replay_path": "out/case-999/normalized-replay.json"
    }
  ]
}"#,
        );
        let args = vec![
            "--batch-index".to_string(),
            batch_index_path.display().to_string(),
            "--left".to_string(),
            "left.json".to_string(),
            "--left-kind".to_string(),
            "normalized-replay".to_string(),
            "--right".to_string(),
            "right.json".to_string(),
            "--right-kind".to_string(),
            "normalized-replay".to_string(),
        ];

        let parsed = parse_batch_comparison_request(&args, "diff");
        assert!(matches!(parsed, Err(2)));
    }

    fn write_batch_index(name: &str, body: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "oxreplay-dnarecalc-cli-{name}-{}-{unique}.json",
            std::process::id()
        ));
        fs::write(&path, body).expect("batch index fixture should be written");
        path
    }
}
