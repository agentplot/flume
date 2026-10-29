//! flume's binary (design 13): it builds the core over the profiles built
//! in and writes the answer as JSON, the one form every surface gives
//! (61). It has no command yet, and answers every place's status; the
//! commands arrive under `src/commands/`.

use flume::profiles;
use flume_core::Answer;
use std::process::ExitCode;

/// The core's answer: every place's status on this host, or the refusal
/// naming why the core could not be built.
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
    use flume::host::config::HostConfig;
    use flume::profiles::{core_over, BuiltIn};
    use flume_core::Answer;

    #[test]
    fn the_answer_is_written_as_json_and_read_back() {
        let state = std::env::temp_dir().join(format!("flume-main-{}", std::process::id()));
        let core = core_over(&state, HostConfig::default(), None, BuiltIn::new())
            .expect("the core over an empty state directory");
        let answer = core.status(None);
        let _ = std::fs::remove_dir_all(&state);
        let back: Answer = serde_json::from_str(&answer.to_json()).expect("an answer's JSON");
        assert_eq!(back, answer);
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
