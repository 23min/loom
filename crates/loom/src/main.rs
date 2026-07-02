//! The `loom` CLI. `loom verify <overlay-dir> [--prop <id>]` runs the overlay runner
//! (M-0016/AC-2); it discovers each property and writes a gap report beside it.

use std::path::Path;
use std::process::ExitCode;

const USAGE: &str = "usage: loom verify <overlay-dir> [--prop <id>]";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("verify") => run_verify(&args[2..]),
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn run_verify(args: &[String]) -> ExitCode {
    let mut overlay: Option<&str> = None;
    let mut prop: Option<&str> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--prop" => {
                i += 1;
                match args.get(i) {
                    Some(v) => prop = Some(v),
                    None => {
                        eprintln!("--prop requires a value\n{USAGE}");
                        return ExitCode::from(2);
                    }
                }
            }
            other if overlay.is_none() => overlay = Some(other),
            other => {
                eprintln!("unexpected argument: {other}\n{USAGE}");
                return ExitCode::from(2);
            }
        }
        i += 1;
    }

    let Some(overlay) = overlay else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };

    match loom::runner::verify(Path::new(overlay), prop) {
        Ok(reports) => {
            for r in &reports {
                println!("loom verify: {} -> {}", r.property, r.path.display());
            }
            if reports.is_empty() {
                eprintln!("loom verify: no properties found under {overlay}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("loom verify: {e}");
            ExitCode::FAILURE
        }
    }
}
