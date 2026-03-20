#![forbid(unsafe_code)]

use oxreplay_bundle::{ValidationStatus, render_text_report, validate_bundle_at_path};
use oxreplay_conformance::{load_manifest_from_path, validate_manifest};
use oxreplay_core::{
    ReplayScenario, load_oxcalc_tracecalc_projection, load_oxfml_fec_projection,
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
    let mut format = String::from("text");

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--bundle" => bundle_path = iter.next(),
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

    let Some(bundle_path) = bundle_path else {
        eprintln!("validate-bundle requires --bundle <path>");
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
    let mut case_id = None;
    let mut predicate_id = None;
    let mut predicate_description = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--bundle" => bundle_path = iter.next(),
            "--kind" => kind = iter.next(),
            "--case-id" => case_id = iter.next(),
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
        eprintln!("distill requires --kind <oxcalc-tracecalc|oxfml-fec-commit|normalized-replay>");
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

    let scenario = match load_scenario_by_kind(&bundle_path, &kind, case_id.as_deref()) {
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
    let mut case_id = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--bundle" => bundle_path = iter.next(),
            "--kind" => kind = iter.next(),
            "--case-id" => case_id = iter.next(),
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
        eprintln!("replay requires --kind <oxcalc-tracecalc|oxfml-fec-commit|normalized-replay>");
        return Err(2);
    };

    load_scenario_by_kind(&bundle_path, &kind, case_id.as_deref())
}

fn parse_diff_inputs(args: Vec<String>) -> Result<(ReplayScenario, ReplayScenario), i32> {
    let mut left = None;
    let mut left_kind = None;
    let mut left_case_id = None;
    let mut right = None;
    let mut right_kind = None;
    let mut right_case_id = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--left" => left = iter.next(),
            "--left-kind" => left_kind = iter.next(),
            "--left-case-id" => left_case_id = iter.next(),
            "--right" => right = iter.next(),
            "--right-kind" => right_kind = iter.next(),
            "--right-case-id" => right_case_id = iter.next(),
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

    let left = load_scenario_by_kind(&left, &left_kind, left_case_id.as_deref())?;
    let right = load_scenario_by_kind(&right, &right_kind, right_case_id.as_deref())?;
    Ok((left, right))
}

fn load_scenario_by_kind(
    path: &str,
    kind: &str,
    case_id: Option<&str>,
) -> Result<ReplayScenario, i32> {
    match kind {
        "oxcalc-tracecalc" => load_oxcalc_tracecalc_projection(path).map_err(|error| {
            eprintln!("{error}");
            4
        }),
        "normalized-replay" => load_replay_scenario_from_path(path).map_err(|error| {
            eprintln!("{error}");
            4
        }),
        "oxfml-fec-commit" => {
            let Some(case_id) = case_id else {
                eprintln!("oxfml-fec-commit requires --case-id <id>");
                return Err(2);
            };
            load_oxfml_fec_projection(path, case_id).map_err(|error| {
                eprintln!("{error}");
                4
            })
        }
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
