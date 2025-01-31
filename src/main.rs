use std::env;
use std::path::Path;
use std::process;
use std::time::Duration;

const PROGRAM: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default check interval expressed in seconds
const DEFAULT_CHECK_INTERVAL: u64 = 60;

fn main() {
    println!("{PROGRAM} {VERSION} - Minimalistic cross-platform mouse jiggler");
    println!("Copyright (c) 2025 Marco Ivaldi <raptor@0xdeadbeef.info>");
    println!();

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    let prog = Path::new(&args[0])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap_or(PROGRAM);

    let interval = match args.len() {
        1 => DEFAULT_CHECK_INTERVAL,
        2 => args[1].parse().unwrap_or(0),
        _ => 0,
    };
    if interval == 0 {
        usage(prog);
    }

    // Let's do it
    match jiggy::run(Duration::from_secs(interval)) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("[!] Error: {err}");
            process::exit(1);
        }
    }
}

/// Print usage information and exit
fn usage(prog: &str) {
    println!("Usage:");
    println!("$ {prog} <check_interval_in_secs> (default: {DEFAULT_CHECK_INTERVAL}s)");
    println!("\nExamples:");
    println!("$ {prog}");
    println!("$ {prog} 30");

    process::exit(1);
}
