use std::{env, fs, process};

use orbis::{SimulationEnvelope, parse_program, simulate_program};

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
        "simulate-report" => match simulate_program(&program) {
            Ok(report) => println!("{}", SimulationEnvelope::success(&args[1], report).to_json()),
            Err(error) => println!("{}", SimulationEnvelope::failure(&args[1], error.to_string()).to_json()),
        },
        _ => print_usage(),
    }

    Ok(())
}

fn print_usage() {
    eprintln!("usage: sekai <simulate|simulate-json|simulate-report> <file.sk>");
}
