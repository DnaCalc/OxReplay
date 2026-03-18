#![forbid(unsafe_code)]

use oxreplay_bundle::{ValidationStatus, render_text_report, validate_bundle_at_path};

const HELP: &str = "\
dna-recalc <command> [options]

Commands:
  validate-bundle
  replay
  diff
  explain
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
        Some(
            "replay" | "diff" | "explain" | "validate-adapter" | "witness-state" | "pack-export",
        ) => {
            println!("DNA ReCalc CLI scaffold is active for this command family.");
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
