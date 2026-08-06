use std::env;

use rust_sample::app::config::AppConfig;
use rust_sample::app::run;

fn print_help() {
    println!("rust-sample usage:");
    println!("  --mode <quick|normal|deep>");
    println!("  --items <N>");
    println!("  --seed <N>");
}

fn parse_args() -> AppConfig {
    let args: Vec<String> = env::args().collect();
    let mut mode = "normal".to_string();
    let mut items: usize = 20;
    let mut seed: u64 = 1;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            "--mode" => {
                if i + 1 < args.len() {
                    mode = args[i + 1].clone();
                    i += 1;
                }
            }
            "--items" => {
                if i + 1 < args.len() {
                    if let Ok(v) = args[i + 1].parse::<usize>() {
                        items = v;
                    }
                    i += 1;
                }
            }
            "--seed" => {
                if i + 1 < args.len() {
                    if let Ok(v) = args[i + 1].parse::<u64>() {
                        seed = v;
                    }
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    AppConfig { mode, items, seed }
}

fn main() {
    let config = parse_args();
    match run(config) {
        Ok(report) => {
            println!("{}", report.render_text());
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}
