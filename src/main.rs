//! main.rs

use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

const PROGRAM: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// Default check interval expressed in seconds
const DEFAULT_CHECK_INTERVAL: u64 = 60;

fn main() -> ExitCode {
    eprintln!("{PROGRAM} {VERSION} - Minimalistic cross-platform mouse jiggler tool");
    eprintln!("Copyright (c) 2025-2026 {AUTHORS}");
    eprintln!();

    // Parse command line arguments
    let mut args = env::args_os();
    let argv0 = args.next().unwrap_or_else(|| OsString::from(PROGRAM));

    let prog = Path::new(&argv0)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(PROGRAM);

    let interval = match (args.next(), args.next()) {
        (None, _) => DEFAULT_CHECK_INTERVAL,
        (Some(arg), None) => arg.to_str().and_then(|s| s.parse().ok()).unwrap_or(0),
        _ => return usage(prog),
    };
    if interval == 0 {
        return usage(prog);
    }

    // Let's do it
    match jiggy::run(Duration::from_secs(interval)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("[!] Error: {err}");
            ExitCode::FAILURE
        }
    }
}

/// Print usage information and exit
fn usage(prog: &str) -> ExitCode {
    eprintln!("Usage:");
    eprintln!("{prog} [check_interval_in_secs] (default: {DEFAULT_CHECK_INTERVAL}s)");
    eprintln!("\nExamples:");
    eprintln!("{prog}");
    eprintln!("{prog} 30");

    ExitCode::FAILURE
}
