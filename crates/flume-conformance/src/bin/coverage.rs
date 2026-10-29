//! The coverage check, as the moon task `conformance:coverage` runs it:
//! it fails naming every clause, invariant or scenario of the statement
//! no scenario cites but those no change has built yet, every scenario
//! citing one of those, every scenario citing nothing, and every
//! citation of no clause (106, section 12).

use flume_conformance::{clauses, coverage, suite};
use std::process::ExitCode;

fn main() -> ExitCode {
    let scenarios = suite::every();
    let problems = coverage::check(&clauses::all(), &clauses::unbuilt(), &scenarios);
    if problems.is_empty() {
        let left: Vec<String> = clauses::unbuilt()
            .iter()
            .map(|(clause, change)| format!("{clause} to {change}"))
            .collect();
        println!(
            "every clause of Parts A and B, I1 to I8 and S1 to S49 at {} is cited by the suite's {} scenarios{}",
            clauses::REVISION,
            scenarios.len(),
            if left.is_empty() {
                String::new()
            } else {
                format!("; {} left", left.join(", "))
            }
        );
        return ExitCode::SUCCESS;
    }
    eprintln!(
        "{} problems with the suite's citations of the statement at {}:",
        problems.len(),
        clauses::REVISION
    );
    for problem in &problems {
        eprintln!("  {problem}");
    }
    ExitCode::FAILURE
}
