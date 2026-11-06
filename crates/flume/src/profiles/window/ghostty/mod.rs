//! The window profile through Ghostty (C.8; design 10), checked against
//! Ghostty 1.4.0. On macOS, Ghostty's command line starts no window, so
//! `open -na Ghostty.app` asks the display for a new one, `--env` giving
//! its process each variable the command needs, and `--args --title=<title>
//! -e <program> <arguments>` what it runs under the title Ghostty forces,
//! each a word of its own with no shell between; `open` is waited on
//! within the bound, its exit 0 opened and any other refused in its words.
//! Elsewhere `ghostty --title=<title> -e <program> <arguments>` is
//! started in a process group of its own, its input, output and errors
//! closed, under the host user's login environment with the command's
//! over it, and not waited on, the window being the person's (120, 188,
//! 189; core.13).

use crate::host::process::{program_path, run_bounded, Bounded};
use crate::host::{self, System};
use flume_seams::{ViewCommand, Window, WindowShown};
use std::collections::BTreeMap;
use std::process::{Command, Stdio};
use std::time::Duration;

/// The profile's name in the host's configuration, `terminal_app` (105,
/// 188).
pub const NAME: &str = "ghostty";

/// How long `open` is waited on (189).
pub const BOUND: Duration = Duration::from_secs(10);

/// The window Ghostty opens for a system.
#[derive(Clone, Debug)]
pub struct Ghostty {
    system: System,
    open: String,
    ghostty: String,
    login: Option<BTreeMap<String, String>>,
    bound: Duration,
}

impl Default for Ghostty {
    fn default() -> Self {
        Ghostty::new()
    }
}

impl Ghostty {
    /// The profile for the host's own system.
    pub fn new() -> Ghostty {
        Ghostty::for_system(System::of_host())
    }

    /// The profile for `system`, whatever the host runs, so a test holds
    /// both paths on any host.
    pub fn for_system(system: System) -> Ghostty {
        Ghostty {
            system,
            open: "open".into(),
            ghostty: NAME.into(),
            login: None,
            bound: BOUND,
        }
    }

    /// The same profile running `open` and `ghostty` as named, a test's
    /// stand-ins.
    pub fn with_programs(self, open: &str, ghostty: &str) -> Ghostty {
        Ghostty {
            open: open.into(),
            ghostty: ghostty.into(),
            ..self
        }
    }

    /// The same profile, `ghostty` started in exactly `variables` for the
    /// host user's login environment.
    pub fn with_login(self, variables: &[(&str, &str)]) -> Ghostty {
        Ghostty {
            login: Some(
                variables
                    .iter()
                    .map(|(name, value)| (name.to_string(), value.to_string()))
                    .collect(),
            ),
            ..self
        }
    }

    /// The same profile, `open` waited on for `bound`.
    pub fn with_bound(self, bound: Duration) -> Ghostty {
        Ghostty { bound, ..self }
    }

    /// The words Ghostty is given for `command` under `title`: the title
    /// forced, then the command after `-e`.
    fn ghostty_words(command: &ViewCommand, title: &str) -> Vec<String> {
        let mut words = vec![format!("--title={title}"), "-e".into()];
        words.push(command.program.clone());
        words.extend(command.arguments.iter().cloned());
        words
    }

    /// The window asked of the display through `open`, waited on within
    /// the bound.
    fn through_open(&self, command: &ViewCommand, title: &str) -> WindowShown {
        let mut open = Command::new(&self.open);
        open.args(["-na", "Ghostty.app"]);
        for (name, value) in &command.environment {
            open.arg("--env").arg(format!("{name}={value}"));
        }
        open.arg("--args")
            .args(Ghostty::ghostty_words(command, title));
        match run_bounded(open, self.bound) {
            Ok(Bounded::Exited { success: true, .. }) => WindowShown::Opened,
            Ok(Bounded::Exited { errors, .. }) => WindowShown::Refused {
                reason: if errors.is_empty() {
                    format!("{} -na Ghostty.app failed and said nothing", self.open)
                } else {
                    errors
                },
            },
            Ok(Bounded::Unanswered) => WindowShown::Refused {
                reason: format!(
                    "{} -na Ghostty.app did not answer within {} seconds",
                    self.open,
                    self.bound.as_secs_f32()
                ),
            },
            Err(error) => WindowShown::Refused {
                reason: format!("{} could not be run: {error}", self.open),
            },
        }
    }

    /// Ghostty started on its own, in a process group of its own under the
    /// login environment with the command's over it, and not waited on.
    fn started(&self, command: &ViewCommand, title: &str) -> WindowShown {
        let path = std::env::var("PATH").unwrap_or_default();
        let program = program_path(&self.ghostty, &path)
            .map(|found| found.to_string_lossy().into_owned())
            .unwrap_or_else(|| self.ghostty.clone());
        let login = match &self.login {
            Some(login) => login.clone(),
            None => host::login_environment(),
        };
        let mut ghostty = Command::new(&program);
        ghostty
            .args(Ghostty::ghostty_words(command, title))
            .env_clear()
            .envs(&login)
            .envs(&command.environment)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        std::os::unix::process::CommandExt::process_group(&mut ghostty, 0);
        match ghostty.spawn() {
            Ok(_) => WindowShown::Opened,
            Err(error) => WindowShown::Refused {
                reason: format!("{program} could not be started: {error}"),
            },
        }
    }
}

impl Window for Ghostty {
    fn show(&self, command: &ViewCommand, title: &str) -> WindowShown {
        match self.system {
            System::MacOs => self.through_open(command, title),
            System::Other => self.started(command, title),
        }
    }
}
