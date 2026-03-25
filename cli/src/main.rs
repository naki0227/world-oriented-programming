use std::{env, fs, process};

use orbis::{analyze_program, parse_program, simulate_program, simulate_program_envelope};

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 {
        print_usage();
        return Ok(());
    }

    let source = fs::read_to_string(&args[1])?;
    let program = parse_program(&source)?;
    match args[0].as_str() {
        "simulate" => {
            let report = simulate_program(&program)?;
            println!("sekai simulate {}", &args[1]);
            for snapshot in report.snapshots {
                println!("t={:.3}", snapshot.time);
                for sphere in snapshot.spheres {
                    println!(
                        "  {} position={} velocity={}",
                        sphere.name, sphere.position, sphere.velocity
                    );
                }
            }
        }
        "simulate-json" => {
            let report = simulate_program(&program)?;
            println!("{}", report.to_json(&args[1]));
        }
        "simulate-report" => {
            let envelope = simulate_program_envelope(&program, &args[1]);
            println!("{}", envelope.to_json());
        }
        "analyze" => {
            let inventory = analyze_program(&program)?;
            println!("{}", inventory.to_json(&args[1]));
        }
        _ => print_usage(),
    }

    Ok(())
}

fn print_usage() {
    eprintln!("usage: sekai <simulate|simulate-json|simulate-report|analyze> <file.sk>");
}
