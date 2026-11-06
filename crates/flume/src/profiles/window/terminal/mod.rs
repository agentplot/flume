//! The window profile through Terminal.app (C.8; design 11), checked
//! against Terminal 2.15 on macOS 27. `osascript` runs a script, each line
//! an `-e` of its own, telling Terminal to `do script` its first argument,
//! the command's line, in a new window, and to show its second, the
//! title, as that tab's custom title; the window's title shows it beside
//! what the person's Terminal profile adds. The line `exec`s the command,
//! so it takes the place of Terminal's shell, `/usr/bin/env` giving it the
//! variables it needs. `osascript` is waited on within the bound, its exit
//! 0 opened and any other refused in its words. Terminal.app runs on
//! macOS alone, and elsewhere every show is refused (188, 189; core.13).

use crate::host::process::{run_bounded, Bounded};
use crate::host::System;
use crate::shell;
use flume_seams::{ViewCommand, Window, WindowShown};
use std::process::Command;
use std::time::Duration;

/// The profile's name in the host's configuration, `terminal_app` (105,
/// 188).
pub const NAME: &str = "terminal";

/// How long `osascript` is waited on (189).
pub const BOUND: Duration = Duration::from_secs(10);

/// The script `osascript` runs, one line to each `-e`, its first argument
/// the command's line and its second the title.
pub const SCRIPT: [&str; 8] = [
    "on run argv",
    "tell application \"Terminal\"",
    "set opened to do script (item 1 of argv)",
    "set custom title of opened to (item 2 of argv)",
    "set title displays custom title of opened to true",
    "activate",
    "end tell",
    "end run",
];

/// The window Terminal.app opens, on macOS.
#[derive(Clone, Debug)]
pub struct TerminalApp {
    system: System,
    osascript: String,
    bound: Duration,
}

impl Default for TerminalApp {
    fn default() -> Self {
        TerminalApp::new()
    }
}

impl TerminalApp {
    /// The profile for the host's own system.
    pub fn new() -> TerminalApp {
        TerminalApp::for_system(System::of_host())
    }

    /// The profile for `system`, whatever the host runs, so a test holds
    /// both paths on any host.
    pub fn for_system(system: System) -> TerminalApp {
        TerminalApp {
            system,
            osascript: "osascript".into(),
            bound: BOUND,
        }
    }

    /// The same profile running `osascript` as named, a test's stand-in.
    pub fn with_program(self, osascript: &str) -> TerminalApp {
        TerminalApp {
            osascript: osascript.into(),
            ..self
        }
    }

    /// The same profile, `osascript` waited on for `bound`.
    pub fn with_bound(self, bound: Duration) -> TerminalApp {
        TerminalApp { bound, ..self }
    }
}

/// The line Terminal's window runs for `command`: `exec`, then
/// `/usr/bin/env` and each variable it needs where it needs any, then its
/// program and arguments, each word quoted for a POSIX shell where it
/// holds anything a shell reads (188).
pub fn line(command: &ViewCommand) -> String {
    let mut words = vec!["exec".to_string()];
    if !command.environment.is_empty() {
        words.push("/usr/bin/env".into());
        for (name, value) in &command.environment {
            words.push(shell::argument(&format!("{name}={value}")));
        }
    }
    words.push(shell::argument(&command.program));
    words.extend(command.arguments.iter().map(|word| shell::argument(word)));
    words.join(" ")
}

impl Window for TerminalApp {
    fn show(&self, command: &ViewCommand, title: &str) -> WindowShown {
        if self.system != System::MacOs {
            return WindowShown::Refused {
                reason: "Terminal.app runs only on macOS".into(),
            };
        }
        let mut osascript = Command::new(&self.osascript);
        for script in SCRIPT {
            osascript.args(["-e", script]);
        }
        osascript.arg(line(command)).arg(title);
        match run_bounded(osascript, self.bound) {
            Ok(Bounded::Exited { success: true, .. }) => WindowShown::Opened,
            Ok(Bounded::Exited { errors, .. }) => WindowShown::Refused {
                reason: if errors.is_empty() {
                    format!("{} failed and said nothing", self.osascript)
                } else {
                    errors
                },
            },
            Ok(Bounded::Unanswered) => WindowShown::Refused {
                reason: format!(
                    "{} did not answer within {} seconds",
                    self.osascript,
                    self.bound.as_secs_f32()
                ),
            },
            Err(error) => WindowShown::Refused {
                reason: format!("{} could not be run: {error}", self.osascript),
            },
        }
    }
}
