//! The coverage check (106, section 12): every clause of Parts A and B,
//! every invariant and every scenario of section 11 is cited by a
//! scenario of the suite, every scenario cites something, and every
//! citation names a clause, invariant or scenario of the statement.

use crate::scenario::Cited;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    /// A clause, invariant or scenario no scenario cites.
    Uncited(String),
    /// A scenario that cites nothing.
    CitesNothing(String),
    /// A scenario citing what the statement does not hold.
    NoSuchClause { scenario: String, cite: String },
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Problem::Uncited(clause) => write!(f, "{clause} is cited by no scenario"),
            Problem::CitesNothing(scenario) => {
                write!(f, "the scenario \"{scenario}\" cites nothing")
            }
            Problem::NoSuchClause { scenario, cite } => write!(
                f,
                "the scenario \"{scenario}\" cites {cite}, which the statement does not hold"
            ),
        }
    }
}

/// Every problem with `scenarios`' citations of `clauses`.
pub fn check(clauses: &[String], scenarios: &[Cited]) -> Vec<Problem> {
    let known: BTreeSet<&str> = clauses.iter().map(String::as_str).collect();
    let mut cited = BTreeSet::new();
    let mut problems = vec![];
    for scenario in scenarios {
        if scenario.cites.is_empty() {
            problems.push(Problem::CitesNothing(scenario.name.clone()));
        }
        for cite in &scenario.cites {
            if known.contains(cite.as_str()) {
                cited.insert(cite.as_str());
            } else {
                problems.push(Problem::NoSuchClause {
                    scenario: scenario.name.clone(),
                    cite: cite.clone(),
                });
            }
        }
    }
    problems.extend(
        clauses
            .iter()
            .filter(|clause| !cited.contains(clause.as_str()))
            .map(|clause| Problem::Uncited(clause.clone())),
    );
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clauses() -> Vec<String> {
        ["69", "70", "71", "I1", "S1"].map(String::from).to_vec()
    }

    fn scenario(name: &str, cites: &[&str]) -> Cited {
        Cited {
            name: name.into(),
            cites: cites.iter().map(|cite| cite.to_string()).collect(),
        }
    }

    #[test]
    fn every_clause_cited_passes() {
        let scenarios = [
            scenario("one", &["69", "70"]),
            scenario("two", &["71", "I1", "S1"]),
        ];
        assert_eq!(check(&clauses(), &scenarios), vec![]);
    }

    #[test]
    fn a_clause_left_out_is_named() {
        let scenarios = [scenario("one", &["69", "71", "I1", "S1"])];
        assert_eq!(
            check(&clauses(), &scenarios),
            vec![Problem::Uncited("70".into())]
        );
        assert_eq!(
            Problem::Uncited("70".into()).to_string(),
            "70 is cited by no scenario"
        );
    }

    #[test]
    fn a_citation_of_no_clause_names_the_scenario_and_the_number() {
        let scenarios = [scenario("one", &["69", "70", "71", "I1", "S1", "100"])];
        let problems = check(&clauses(), &scenarios);
        assert_eq!(
            problems,
            vec![Problem::NoSuchClause {
                scenario: "one".into(),
                cite: "100".into()
            }]
        );
        let words = problems[0].to_string();
        assert!(words.contains("one") && words.contains("100"), "{words}");
    }

    #[test]
    fn a_scenario_citing_nothing_is_named() {
        let scenarios = [
            scenario("one", &["69", "70", "71", "I1", "S1"]),
            scenario("bare", &[]),
        ];
        assert_eq!(
            check(&clauses(), &scenarios),
            vec![Problem::CitesNothing("bare".into())]
        );
    }

    #[test]
    fn the_statements_100_is_no_clause() {
        let scenarios = [scenario("one", &["100"])];
        assert!(
            check(&crate::clauses::all(), &scenarios).contains(&Problem::NoSuchClause {
                scenario: "one".into(),
                cite: "100".into()
            })
        );
    }
}
