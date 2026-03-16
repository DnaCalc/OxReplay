#![forbid(unsafe_code)]

use oxreplay_abstractions::CapabilityLevel;

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
    let command = std::env::args().nth(1);

    match command.as_deref() {
        None | Some("help") | Some("--help") | Some("-h") => {
            print!("{HELP}");
        }
        Some(
            "validate-bundle" | "replay" | "diff" | "explain" | "validate-adapter"
            | "witness-state" | "pack-export",
        ) => {
            println!(
                "DNA ReCalc CLI scaffold is active. Declared capability ladder size: {}.",
                CapabilityLevel::ALL.len()
            );
        }
        Some(other) => {
            eprintln!("unknown command: {other}\n\n{HELP}");
            std::process::exit(2);
        }
    }
}
