use std::env;

use rust_div_zero_sample::batch::{run_panic_mode, run_safe_mode};

fn print_help() {
    println!("Usage:");
    println!("  --mode <safe|panic>");
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

    let numerators = vec![10, 20, 30, 40, 50];
    let denominators = vec![2, 4, 0, 5, 10];

    match mode.as_str() {
        "panic" => {
            println!("Running panic mode (will panic on division by zero)...");
            let values = run_panic_mode(&numerators, &denominators);
            println!("Computed values: {:?}", values);
        }
        "safe" => {
            println!("Running safe mode...");
            let report = run_safe_mode(&numerators, &denominators);
            println!("Success count: {}", report.success_count);
            println!("Failure count: {}", report.failure_count);
            println!("Values: {:?}", report.values);
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
