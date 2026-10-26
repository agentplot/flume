//! flume's binary (design 13): it builds the core over the profiles built
//! in and writes the answer as JSON, the one form every surface gives
//! (61). It has no command yet; the commands arrive under
//! `src/commands/`.

mod profiles;

use flume_core::Answer;
use std::process::ExitCode;

/// The core's answer: every place's status once the host's profiles are
/// built in, or the refusal naming what is missing.
fn answer() -> Answer {
    match profiles::core() {
        Ok(core) => core.status(None),
        Err(reason) => Answer::refused(reason),
    }
}

fn main() -> ExitCode {
    let answer = answer();
    println!("{}", answer.to_json());
    if answer.refusal().is_some() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_answer_is_written_as_json_and_read_back() {
        let answer = answer();
        let back: Answer = serde_json::from_str(&answer.to_json()).expect("an answer's JSON");
        assert_eq!(back, answer);
    }

    #[test]
    fn with_no_profile_built_in_the_answer_names_what_is_missing() {
        let reason = answer().refusal().map(str::to_string).unwrap_or_default();
        assert!(reason.contains("terminal"), "{reason}");
    }

    #[test]
    fn profiles_holds_one_marker_per_seam() {
        let source = include_str!("profiles.rs");
        for seam in [
            "place",
            "kinds",
            "change",
            "agent",
            "terminal",
            "merge queue",
            "store",
        ] {
            let marker = format!("// ---- the {seam} seam's profiles: a profile unit appends its module line below ----");
            assert_eq!(
                source.matches(&marker).count(),
                1,
                "the {seam} seam's marker"
            );
        }
    }
}
