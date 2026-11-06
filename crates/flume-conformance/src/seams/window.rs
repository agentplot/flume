//! The window seam's scenarios (B.8, 188, 189): a window running a
//! view's command under a title, the line a person runs where no window
//! is opened, or a refusal in the window program's words, within the
//! bound the profile states.

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;
use std::time::{Duration, Instant};

pub trait WindowHarness {
    type Seam: Window + ?Sized;

    fn seam(&self) -> &Self::Seam;

    /// A command a window runs: a program, its arguments and a variable
    /// it needs with its value.
    fn command(&self) -> ViewCommand;

    /// Whether the profile opens a window at all, rather than giving the
    /// line a person runs (188).
    fn opens(&self) -> bool;

    /// The windows opened, in order, each with the command it runs and
    /// its title, as the harness reads them back.
    fn opened(&self) -> Vec<(ViewCommand, String)>;

    /// The next show answers the line a person runs, where the harness
    /// can make it so; whether it did. A profile that opens no window
    /// gives the line at every show (188).
    fn will_print(&self) -> bool;

    /// The window program refuses the next show, saying the words given;
    /// none where the harness cannot make it refuse (188).
    fn will_refuse(&self) -> Option<String>;

    /// The window program does not answer the next show; the bound the
    /// profile states, or none where the harness cannot hold it (189).
    fn will_hang(&self) -> Option<Duration>;
}

const TITLE: &str = "Work Agents";

/// How much past its bound a show may answer, for the process's own
/// ending.
const SLACK: Duration = Duration::from_secs(5);

// ---- the window seam: a profile unit appends its own scenarios at the list's end ----

pub fn scenarios<H: WindowHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name: "window: show opens one window at each asking, running the command under the title",
            cites: &["188", "189"],
            run: one_window_at_each_asking::<H>,
        },
        Scenario {
            name: "window: show gives the line a person runs where it opens no window, naming no variable's value",
            cites: &["188", "105"],
            run: the_line_a_person_runs::<H>,
        },
        Scenario {
            name: "window: show refuses in the window program's words, opening nothing",
            cites: &["188"],
            run: refused_in_its_words::<H>,
        },
        Scenario {
            name: "window: show answers within the bound its profile states",
            cites: &["189"],
            run: answers_within_its_bound::<H>,
        },
    ]
}

fn one_window_at_each_asking<H: WindowHarness>(harness: &H) -> Outcome {
    let command = harness.command();
    let first = harness.seam().show(&command, TITLE);
    let second = harness.seam().show(&command, TITLE);
    if harness.opens() {
        ensure_eq!(first, WindowShown::Opened, "the first show");
        ensure_eq!(second, WindowShown::Opened, "the second show");
        let window = (command, TITLE.to_string());
        ensure_eq!(
            harness.opened(),
            vec![window.clone(), window],
            "the windows opened"
        );
    } else {
        for (which, answer) in [("first", first), ("second", second)] {
            ensure!(
                matches!(answer, WindowShown::Printed { .. }),
                "a profile that opens no window answered the {which} show {answer:?}"
            );
        }
        ensure!(
            harness.opened().is_empty(),
            "a profile that opens no window opened {:?}",
            harness.opened()
        );
    }
    Ok(())
}

fn the_line_a_person_runs<H: WindowHarness>(harness: &H) -> Outcome {
    if !harness.will_print() {
        return Ok(());
    }
    let command = harness.command();
    let WindowShown::Printed { line } = harness.seam().show(&command, TITLE) else {
        return Err("show gave no line where it opens no window".into());
    };
    for word in std::iter::once(&command.program).chain(&command.arguments) {
        ensure!(
            line.contains(word.as_str()),
            "the line {line:?} omits {word:?}"
        );
    }
    for (name, value) in &command.environment {
        ensure!(
            line.contains(name.as_str()),
            "the line {line:?} omits {name}"
        );
        ensure!(
            !line.contains(value.as_str()),
            "the line {line:?} gives {name}'s value"
        );
    }
    ensure!(
        harness.opened().is_empty(),
        "a show that gave the line opened {:?}",
        harness.opened()
    );
    Ok(())
}

fn refused_in_its_words<H: WindowHarness>(harness: &H) -> Outcome {
    let Some(words) = harness.will_refuse() else {
        return Ok(());
    };
    match harness.seam().show(&harness.command(), TITLE) {
        WindowShown::Refused { reason } => {
            ensure!(
                reason.contains(&words),
                "the refusal {reason:?} omits the program's words {words:?}"
            );
        }
        other => return Err(format!("a refused show answered {other:?}")),
    }
    ensure!(
        harness.opened().is_empty(),
        "a refused show opened {:?}",
        harness.opened()
    );
    Ok(())
}

fn answers_within_its_bound<H: WindowHarness>(harness: &H) -> Outcome {
    let Some(bound) = harness.will_hang() else {
        return Ok(());
    };
    let asked = Instant::now();
    let answer = harness.seam().show(&harness.command(), TITLE);
    let took = asked.elapsed();
    ensure!(
        took <= bound + SLACK,
        "show answered after {took:?}, past its bound of {bound:?}"
    );
    ensure!(
        matches!(answer, WindowShown::Refused { .. }),
        "a show whose program never answered gave {answer:?}"
    );
    Ok(())
}
