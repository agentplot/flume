//! The terminal profile that prints the command (C.5; design 12): open
//! answers the command, the prompt carried in it, for a person to run in
//! the place, and status gives it with the agent; send and close are
//! refused, so a tell is refused and a prompt not received is never begun
//! (38, 53); state is unknown from the terminal, the core reading the
//! agent stopped once its own record says the session ended (134);
//! screen is nothing (136); there is no watch, so flume runs attended
//! (119); and it names no session (7). A command printed once for an
//! identity is printed for it never again, kept as a mark in the place's
//! own folder (98, 128).

use crate::shell;
use flume_seams::{
    AgentCommand, AgentState, Closed, Identity, Opened, PlaceRef, Sent, Terminal, Watch,
};
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
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

/// The line a person runs: into the place, the command's environment
/// over theirs, the program with its arguments and the prompt (88, 38).
pub fn line(place: &PlaceRef, command: &AgentCommand) -> String {
    let mut words: Vec<String> = vec!["cd".into(), shell::quote(&place.location), "&&".into()];
    words.extend(
        command
            .env
            .iter()
            .map(|(name, value)| format!("{name}={}", shell::quote(value))),
    );
    words.push(shell::words(
        std::iter::once(command.program.as_str())
            .chain(command.args.iter().map(String::as_str))
            .chain(command.prompt_args.iter().map(String::as_str)),
    ));
    words.join(" ")
}

impl Terminal for Printed {
    fn open(
        &self,
        place: &PlaceRef,
        folder: &str,
        command: &AgentCommand,
        identity: &Identity,
        _beside: Option<&str>,
    ) -> Opened {
        let printed = line(place, command);
        let mark = mark(folder, identity);
        let made = mark
            .parent()
            .map_or(Ok(()), fs::create_dir_all)
            .and_then(|()| OpenOptions::new().write(true).create_new(true).open(&mark))
            .and_then(|mut file| file.write_all(printed.as_bytes()));
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

    fn watch(&self) -> Watch {
        Watch::None
    }

    fn session(&self) -> Option<String> {
        None
    }
}
