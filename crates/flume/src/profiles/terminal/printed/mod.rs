//! The terminal profile that prints the command (C.5; design 12): open
//! answers the command, the prompt carried in it, for a person to run in
//! the place, and status gives it with the agent; send and close are
//! refused, so a tell is refused and a prompt not received is never begun
//! (38, 53), as is every send to a recipient's session, which it holds
//! none of, so every end stays pending, read from status, a wait or the
//! inbox (43); state is unknown from the terminal, the core reading the
//! agent stopped once its own record says the session ended (134);
//! screen is nothing (136); tidy and show do nothing, as it makes
//! nothing and shows no clue (138, 139); there is no watch, so flume runs attended
//! (119); and it names no session (7). A command printed once for an
//! identity is printed for it never again, kept as a mark in the place's
//! own folder (98, 128). The command's environment is kept beside the
//! mark in a file the host's user alone may read, which the line names,
//! so no value of it, a secret's or a token's, is printed (32, 56).

use crate::shell;
use flume_seams::{
    AgentCommand, AgentState, Closed, Clues, Identity, Opened, PlaceRef, Sent, Shown, ShownAt,
    Terminal, TerminalTidy, ToSession, Watch,
};
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

/// The profile's name in the host's configuration, `[host] terminal`, and
/// the one bound where it names none (105).
pub const NAME: &str = "printed";

/// The terminal that prints each agent's command for a person to run.
#[derive(Clone, Copy, Debug, Default)]
pub struct Printed;

/// Where the command printed for `identity` is marked in the place's
/// folder.
fn mark(folder: &str, identity: &Identity) -> PathBuf {
    Path::new(folder)
        .join("printed")
        .join(identity.to_string().replace('/', "%"))
}

/// Where the environment of the command printed for `identity` is kept,
/// beside its mark: the line names the file and never a value, since a
/// value may be a secret (32, 56).
fn environment(folder: &str, identity: &Identity) -> PathBuf {
    let mut path = mark(folder, identity).into_os_string();
    path.push(".env");
    PathBuf::from(path)
}

/// The environment file's text: one `NAME=value` line a POSIX shell
/// reads per variable, each value quoted.
fn environment_text(command: &AgentCommand) -> String {
    command
        .env
        .iter()
        .map(|(name, value)| format!("{name}={}\n", shell::quote(value)))
        .collect()
}

/// The line a person runs: into the place, then a POSIX shell that reads
/// the command's environment from `env` over theirs and runs the program
/// with its arguments and the prompt (88, 38). No value of the
/// environment is in it, only the file's path; the person's own shell is
/// left as it was.
pub fn line(place: &PlaceRef, command: &AgentCommand, env: &Path) -> String {
    let mut words: Vec<String> = vec!["cd".into(), shell::quote(&place.location), "&&".into()];
    if !command.env.is_empty() {
        words.push(shell::words([
            "sh",
            "-c",
            "set -a && . \"$0\" && set +a && exec \"$@\"",
            &env.to_string_lossy(),
        ]));
    }
    words.push(shell::words(
        std::iter::once(command.program.as_str())
            .chain(command.args.iter().map(String::as_str))
            .chain(command.prompt_args.iter().map(String::as_str)),
    ));
    words.join(" ")
}

/// Writes `text` to a new file at `path` that the host's user alone may
/// read, refusing one that stands.
fn write_new(path: &Path, text: &str, private: bool) -> std::io::Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    if private {
        options.mode(0o600);
    }
    options.open(path)?.write_all(text.as_bytes())
}

impl Terminal for Printed {
    fn open(
        &self,
        place: &PlaceRef,
        folder: &str,
        command: &AgentCommand,
        identity: &Identity,
        _beside: Option<&str>,
        _session: Option<&str>,
    ) -> Opened {
        let env = environment(folder, identity);
        let printed = line(place, command, &env);
        let mark = mark(folder, identity);
        let made = mark
            .parent()
            .map_or(Ok(()), fs::create_dir_all)
            .and_then(|()| write_new(&mark, &printed, false))
            .and_then(|()| match fs::remove_file(&env) {
                Err(error) if error.kind() != ErrorKind::NotFound => Err(error),
                _ => Ok(()),
            })
            .and_then(|()| write_new(&env, &environment_text(command), true));
        match made {
            Ok(()) => Opened::Opened {
                carried: true,
                printed: Some(printed),
            },
            Err(error) if error.kind() == ErrorKind::AlreadyExists => Opened::Refused {
                reason: format!("a command was printed for {identity} already"),
            },
            Err(error) => Opened::Refused {
                reason: format!("{}: {error}", mark.display()),
            },
        }
    }

    fn send(&self, _agent: &Identity, _text: &str) -> Sent {
        Sent::Refused {
            reason: "the printed command sends nothing to a running agent; nothing was typed"
                .into(),
        }
    }

    fn state(&self, _agent: &Identity) -> AgentState {
        AgentState::Unknown
    }

    fn send_to(&self, session: &str, _text: &str) -> ToSession {
        ToSession::Refused {
            reason: format!(
                "the printed command sends nothing to a session, {session} among them; nothing was typed"
            ),
        }
    }

    /// The printed command holds no session to read.
    fn session_state(&self, _session: &str) -> Option<AgentState> {
        None
    }

    fn screen(&self, _agent: &Identity) -> Option<String> {
        None
    }

    fn close(&self, agent: &Identity) -> Closed {
        Closed::Refused {
            reason: format!(
                "the printed command ends no agent: {agent} ends where a person ran it"
            ),
        }
    }

    /// The printed command makes nothing to hold an agent, so it tidies
    /// nothing (138).
    fn tidy(&self, _place: &PlaceRef) -> TerminalTidy {
        TerminalTidy::default()
    }

    fn kept(&self) -> Vec<String> {
        vec![]
    }

    /// Nothing beside a person's own terminal shows a clue (139).
    fn show(&self, _place: &PlaceRef, _clues: &Clues) -> Shown {
        Shown::ShowsNone
    }

    /// Nothing shows a clue, so nothing is shown anywhere (139).
    fn shown_at(&self, _place: &PlaceRef, _agent: Option<&Identity>) -> ShownAt {
        ShownAt::default()
    }

    fn watch(&self) -> Watch {
        Watch::None
    }

    fn session(&self) -> Option<String> {
        None
    }
}
