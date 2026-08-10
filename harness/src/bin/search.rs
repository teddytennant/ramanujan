//! Run a search and, on success, write a Lean certificate.
//!
//! Usage:
//!   ramanujan capset <n> [iterations] [seed]
//!   ramanujan covering <v> <k> <t> <blocks> [iterations] [seed]

use ramanujan::island::search;
use ramanujan::problem::Problem;
use ramanujan::problems::{CapSet, CoveringDesign};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: ramanujan capset <n> [iterations] [seed]\n       \
                 ramanujan covering <v> <k> <t> <blocks> [iterations] [seed]";

    match args.first().map(String::as_str) {
        Some("capset") => {
            let n: u32 = parse(args.get(1), "n");
            let iters: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(50_000);
            let seed: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
            let problem = CapSet::new(n);
            let found = search(&problem, seed, iters);
            report(&problem, found.score, found.published_best, seed, iters);
            emit(&problem.name(), problem.emit_lean(&found.object));
        }
        Some("covering") => {
            let v: u32 = parse(args.get(1), "v");
            let k: u32 = parse(args.get(2), "k");
            let t: u32 = parse(args.get(3), "t");
            let b: usize = parse(args.get(4), "blocks");
            let iters: u64 = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(200_000);
            let seed: u64 = args.get(6).and_then(|s| s.parse().ok()).unwrap_or(1);
            let problem = CoveringDesign::new(v, k, t, b);
            let found = search(&problem, seed, iters);
            let total = problem.target_count();
            println!(
                "{}: covered {}/{} {}-subsets with {} blocks (seed {}, {} iterations)",
                problem.name(),
                found.score,
                total,
                t,
                b,
                seed,
                iters
            );
            if problem.is_complete(&found.object) {
                println!("  complete covering: C({v},{k},{t}) <= {b}");
                emit(&problem.name(), problem.emit_lean(&found.object));
            } else {
                println!("  incomplete: no certificate emitted");
            }
        }
        _ => {
            eprintln!("{usage}");
            std::process::exit(2);
        }
    }
}

fn parse<T: std::str::FromStr>(arg: Option<&String>, what: &str) -> T {
    match arg.and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => {
            eprintln!("missing or invalid <{what}>");
            std::process::exit(2);
        }
    }
}

fn report(problem: &dyn ProblemName, score: i64, published: Option<i64>, seed: u64, iters: u64) {
    println!(
        "{}: best {} (seed {}, {} iterations)",
        problem.name_of(),
        score,
        seed,
        iters
    );
    match published {
        Some(best) if score > best => {
            println!("  EXCEEDS published best of {best}; verify the baseline before claiming anything")
        }
        Some(best) if score == best => println!("  matches published best of {best}"),
        Some(best) => println!("  below published best of {best}"),
        // Silence here would invite reading the number as a record. It is not one.
        None => println!("  no published baseline declared; not a record claim"),
    }
}

/// Minimal object-safe view of a problem's name, so `report` can stay generic
/// without dragging the associated type along.
trait ProblemName {
    fn name_of(&self) -> String;
}

impl<P: Problem> ProblemName for P {
    fn name_of(&self) -> String {
        self.name()
    }
}

fn emit(name: &str, lean: String) {
    let dir = std::path::Path::new("../lean/Certs/Generated");
    if let Err(e) = std::fs::create_dir_all(dir) {
        eprintln!("could not create {}: {e}", dir.display());
        return;
    }
    let file = dir.join(format!("{}.lean", to_module(name)));
    match std::fs::write(&file, lean) {
        Ok(()) => println!("  certificate written to {}", file.display()),
        Err(e) => eprintln!("  could not write certificate: {e}"),
    }
}

/// `capset-4` becomes `Capset4`, which is a legal Lean module name.
fn to_module(name: &str) -> String {
    let mut out = String::new();
    let mut upper = true;
    for ch in name.chars() {
        if ch == '-' || ch == '_' {
            upper = true;
        } else if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}
