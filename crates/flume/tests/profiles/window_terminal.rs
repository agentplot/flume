//! The window profile through Terminal.app (C.8; design 11): the window
//! seam's scenarios against it built for macOS over a stand-in
//! `osascript`, on any host; the script's lines each after an `-e`, then
//! the line and the title, each one argument; the line's `/usr/bin/env`
//! and its quoting; automation not allowed refused in `osascript`'s words;
//! and, built for another system, every show refused with nothing run
//! (188, 189).

use crate::common::window::{shell_words, Run, StandIn};
use crate::common::*;
use flume::host::System;
use flume::profiles::window::terminal::{line, TerminalApp, SCRIPT};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::window::{self, WindowHarness};
use flume_seams::*;
use std::collections::BTreeMap;
use std::time::Duration;

/// The bound a test's profile states for `osascript`.
const BOUND: Duration = Duration::from_secs(2);

struct Harness {
    _scratch: Scratch,
    stand_in: StandIn,
    seam: TerminalApp,
}

impl Harness {
    fn new(system: System) -> Harness {
        let scratch = Scratch::new("window-terminal");
        let stand_in = StandIn::new(&scratch.path("stand-in"), "osascript");
        let seam = TerminalApp::for_system(system)
            .with_program(&stand_in.path())
            .with_bound(BOUND);
        Harness {
            _scratch: scratch,
            stand_in,
            seam,
        }
    }
}

/// The window `run` opened, as its words give it back: the line read as
/// a shell reads it, and the title.
fn window_of(run: &Run) -> (ViewCommand, String) {
    let script: Vec<String> = SCRIPT
        .iter()
        .flat_map(|line| ["-e".to_string(), line.to_string()])
        .collect();
    assert_eq!(run.words[..script.len()], script[..], "the script's lines");
    let [line, title] = &run.words[script.len()..] else {
        panic!("osascript given {:?} after the script", run.words);
    };
    let mut words = shell_words(line).into_iter().peekable();
    assert_eq!(words.next().as_deref(), Some("exec"));
    let mut environment = BTreeMap::new();
    if words.peek().map(String::as_str) == Some("/usr/bin/env") {
        words.next();
        while let Some((name, value)) = words.peek().and_then(|word| word.split_once('=')) {
            environment.insert(name.to_string(), value.to_string());
            words.next();
        }
    }
    let program = words.next().expect("the command's program");
    (
        ViewCommand {
            program,
            arguments: words.collect(),
            environment,
        },
        title.clone(),
    )
}

impl WindowHarness for Harness {
    type Seam = TerminalApp;

    fn seam(&self) -> &TerminalApp {
        &self.seam
    }

    fn command(&self) -> ViewCommand {
        command(Some("/scratch/a home"))
    }

    fn opens(&self) -> bool {
        true
    }

    fn opened(&self) -> Vec<(ViewCommand, String)> {
        self.stand_in.runs().iter().map(window_of).collect()
    }

    fn will_print(&self) -> bool {
        false
    }

    fn will_refuse(&self) -> Option<String> {
        let words = "Not authorized to send Apple events to Terminal.";
        self.stand_in.refuse_next(words);
        Some(words.into())
    }

    fn will_hang(&self) -> Option<Duration> {
        self.stand_in.hang_next();
        Some(BOUND)
    }
}

/// herdr's attach for the session `w`, needing the sessions' home where
/// one is given.
fn command(home: Option<&str>) -> ViewCommand {
    ViewCommand {
        program: "/bin/herdr".into(),
        arguments: vec!["session".into(), "attach".into(), "w".into()],
        environment: home
            .map(|home| BTreeMap::from([("XDG_CONFIG_HOME".into(), home.into())]))
            .unwrap_or_default(),
    }
}

#[test]
fn terminal_on_macos_passes_the_window_scenarios() {
    pass_all("window", &window::scenarios(), || {
        Harness::new(System::MacOs)
    });
}

#[test]
fn the_words_osascript_is_given() {
    // 188, C.8: the script's lines each after an -e, then the line and
    // the title, each one argument.
    let harness = Harness::new(System::MacOs);
    assert_eq!(
        harness.seam.show(&command(None), "Office Herdr"),
        WindowShown::Opened
    );
    let runs = harness.stand_in.runs();
    assert_eq!(runs.len(), 1);
    let mut expected: Vec<String> = SCRIPT
        .iter()
        .flat_map(|line| ["-e".to_string(), line.to_string()])
        .collect();
    expected.push("exec /bin/herdr session attach w".into());
    expected.push("Office Herdr".into());
    assert_eq!(runs[0].words, expected);
    assert!(SCRIPT.contains(&"set opened to do script (item 1 of argv)"));
    assert!(SCRIPT.contains(&"set custom title of opened to (item 2 of argv)"));
    assert!(SCRIPT.contains(&"set title displays custom title of opened to true"));
}

#[test]
fn a_command_that_needs_a_variable() {
    // 188; core.30: the home quoted, and no answer naming its value.
    let command = command(Some("/scratch/a home"));
    assert_eq!(
        line(&command),
        "exec /usr/bin/env 'XDG_CONFIG_HOME=/scratch/a home' /bin/herdr session attach w"
    );
    assert_eq!(
        line(&ViewCommand {
            environment: BTreeMap::from([("XDG_CONFIG_HOME".into(), "/scratch/home".into())]),
            ..command.clone()
        }),
        "exec /usr/bin/env XDG_CONFIG_HOME=/scratch/home /bin/herdr session attach w"
    );
    let harness = Harness::new(System::MacOs);
    let answer = harness.seam.show(&command, "Office Herdr");
    assert_eq!(answer, WindowShown::Opened);
    assert!(!format!("{answer:?}").contains("a home"));
}

#[test]
fn terminals_automation_not_allowed() {
    // 188: refused in osascript's words.
    let harness = Harness::new(System::MacOs);
    let words = "Not authorized to send Apple events to Terminal.";
    harness.stand_in.refuse_next(words);
    assert_eq!(
        harness.seam.show(&command(None), "Office Herdr"),
        WindowShown::Refused {
            reason: words.into()
        }
    );
    assert!(harness.stand_in.runs().is_empty());
}

#[test]
fn not_macos() {
    // 188: refused, saying Terminal.app runs only on macOS, nothing run.
    let harness = Harness::new(System::Other);
    let WindowShown::Refused { reason } = harness.seam.show(&command(None), "Office Herdr") else {
        panic!("Terminal.app opened a window off macOS");
    };
    assert!(
        reason.contains("Terminal.app runs only on macOS"),
        "{reason}"
    );
    assert!(harness.stand_in.runs().is_empty());
}
