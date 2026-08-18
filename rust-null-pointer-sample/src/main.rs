use std::env;

use rust_null_pointer_sample::batch::{run_panic_mode, run_safe_mode};
use rust_null_pointer_sample::null_like::direct_null_deref;

fn print_help() {
    println!("Usage:");
    println!("  --mode <safe|panic|null-deref>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mode = "safe".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--mode" if i + 1 < args.len() => {
                mode = args[i + 1].clone();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    let values = vec![Some("alpha".to_string()), None, Some("gamma".to_string())];

    match mode.as_str() {
        "panic" => {
            println!("Running panic mode (will panic on None.unwrap())...");
            let lengths = run_panic_mode(&values);
            println!("Lengths: {:?}", lengths);
        }
        "null-deref" => {
            println!("Running null-deref mode (intentional UB/crash demonstration)...");
            let value = direct_null_deref();
            println!("Read value from null pointer (undefined behavior): {}", value);
        }
        "safe" => {
            println!("Running safe mode...");
            let report = run_safe_mode(&values);
            println!("Success count: {}", report.success_count);
            println!("Failure count: {}", report.failure_count);
            println!("Lengths: {:?}", report.lengths);
            if !report.errors.is_empty() {
                println!("Errors:");
                for e in &report.errors {
                    println!("  - {}", e);
                }
            }
        }
        _ => {
            eprintln!("invalid mode: {}", mode);
            std::process::exit(1);
        }
    }
}
