//! A person opens a session of a stream's terminal in its view, by the
//! session's name and never a stream's (186, 199; core.48): a stream's
//! own session, resolved as a start resolves its stream's own, or a
//! session the stream names for a kind or a stage, shown through the
//! stream's multiplexer in its view, the stream's else the host's, else
//! a terminal's: for a terminal view, a window its terminal app opens
//! running the terminal's attach, under the stream's title, else its
//! name, for the stream's own session, and under the session's name for
//! one it names for a kind or a stage; for the app, the multiplexer's own
//! app told to show it (107, 187, 188). It takes no lock, writes no held
//! state and settles nothing, so a second asking opens a second window
//! (core.17); and its answer names the command's variables, never their
//! values (core.30).

use crate::answer::and_list;
use crate::room::StreamRoom;
use crate::terminals::{uncarried, Setting};
use crate::{Answer, Engine, StreamOpened, TitleFrom, TitleHeld, ViewCommandStatus, WindowOutcome};
use flume_seams::*;
use std::collections::BTreeMap;

/// The session an open shows, as the stream whose word gives it gives
/// it: the stream's own, or one it names for a kind or a stage alone.
struct Found {
    stream: String,
    session: SessionHeld,
    title: TitleHeld,
}

impl Engine {
    /// The session `session` shown in its stream's view, the host's
    /// default stream's own where none is named; refused, naming why and
    /// opening nothing, for a name no stream's word gives, naming each
    /// stream's sessions, a session two streams' words give, naming both,
    /// a multiplexer or a terminal app naming no profile, and a
    /// terminal's attach or a window's show refused in its own words (4,
    /// 107, 186 to 190, 199).
    pub fn open_session(&self, session: Option<&str>) -> Answer {
        let declared = self.host.streams();
        let streams: BTreeMap<String, StreamRoom> = declared
            .iter()
            .map(|(name, stream)| (name.clone(), StreamRoom::of(name, stream.clone())))
            .collect();
        let found = match session {
            None => {
                let room = streams.get(DEFAULT_STREAM).cloned().unwrap_or_default();
                Found {
                    stream: DEFAULT_STREAM.to_string(),
                    session: room.own_session(DEFAULT_STREAM),
                    title: own_title(DEFAULT_STREAM, declared.get(DEFAULT_STREAM)),
                }
            }
            Some(session) => match found(session, &streams, &declared) {
                Ok(found) => found,
                Err(refused) => return Answer::refused(refused),
            },
        };
        let named = streams.get(&found.stream).and_then(|room| {
            let name = found.session.name.as_deref()?;
            room.named_sessions(&found.stream, &self.host.configuration())
                .into_iter()
                .find(|named| named.name == name)
        });
        self.show_session(found, named)
    }

    /// The session `found` shows, in its stream's view, through its
    /// stream's multiplexer and terminal app (107, 186 to 190).
    fn show_session(&self, found: Found, named: Option<NamedSession>) -> Answer {
        let Found {
            stream: name,
            session,
            title,
        } = found;
        let display = self.host.display_for(&name);
        let Some(terminal) = self.resolver.terminal(&display.multiplexer.profile) else {
            return Answer::refused(uncarried(
                &name,
                Setting::Multiplexer,
                &display.multiplexer,
                &self.resolver.terminal_profiles(),
            ));
        };
        let opened = |title, terminal_app, command, window| {
            Answer::Opened(StreamOpened {
                stream: name.clone(),
                multiplexer: display.multiplexer.clone(),
                session: session.clone(),
                named: named.clone(),
                view: display.view.clone(),
                title,
                terminal_app,
                command,
                window,
            })
        };
        let profile = &display.multiplexer.profile;
        match display.view.view {
            View::App => match terminal.attach(session.name.as_deref(), View::App) {
                Attached::Shown => opened(None, None, None, WindowOutcome::App),
                Attached::Refused { reason } => Answer::refused(reason),
                Attached::Command { .. } => Answer::refused(format!(
                    "the terminal {profile} answered an attach for its app with a command, which only a window's attach answers"
                )),
            },
            View::Terminal => {
                let Some(window) = self.resolver.window(&display.terminal_app.profile) else {
                    return Answer::refused(uncarried(
                        &name,
                        Setting::TerminalApp,
                        &display.terminal_app,
                        &self.resolver.window_profiles(),
                    ));
                };
                let command = match terminal.attach(session.name.as_deref(), View::Terminal) {
                    Attached::Command { command } => command,
                    Attached::Refused { reason } => return Answer::refused(reason),
                    Attached::Shown => {
                        return Answer::refused(format!(
                            "the terminal {profile} answered an attach for a window with its app shown, which only an attach for its app answers"
                        ))
                    }
                };
                let outcome = match window.show(&command, &title.title) {
                    WindowShown::Opened => WindowOutcome::Opened,
                    WindowShown::Printed { line } => WindowOutcome::Printed { line },
                    WindowShown::Refused { reason } => return Answer::refused(reason),
                };
                opened(
                    Some(title),
                    Some(display.terminal_app.clone()),
                    Some(ViewCommandStatus::of(&command)),
                    outcome,
                )
            }
        }
    }
}

/// The session `session` among every stream's own and every session a
/// stream names for a kind or a stage, with its stream, where it came
/// from and its window's title: a stream's own as its own, though the
/// stream names it for a kind too; else one it names for a kind, from
/// `stream-kind`, else for a stage alone, from `stream-stage`. Refused
/// naming each stream's sessions where no stream's word gives it, a
/// stream's name that is not one of its sessions' among them, and naming
/// both streams where two streams' words give it (4, 186, 199).
fn found(
    session: &str,
    streams: &BTreeMap<String, StreamRoom>,
    declared: &BTreeMap<String, Stream>,
) -> Result<Found, String> {
    let giving: Vec<(&String, &StreamRoom, Vec<String>)> = streams
        .iter()
        .filter_map(|(name, room)| {
            let words: Vec<String> = room
                .words(name)
                .into_iter()
                .filter(|(named, _)| named == session)
                .map(|(_, word)| word)
                .collect();
            (!words.is_empty()).then_some((name, room, words))
        })
        .collect();
    match giving.as_slice() {
        [] => Err(no_such_session(session, streams)),
        [(name, room, _)] => {
            let own = room.own_session(name);
            if own.name.as_deref() == Some(session) {
                return Ok(Found {
                    stream: (*name).clone(),
                    session: own,
                    title: own_title(name, declared.get(*name)),
                });
            }
            let for_kind = room
                .kinds
                .values()
                .any(|table| table.session.as_deref() == Some(session));
            Ok(Found {
                stream: (*name).clone(),
                session: SessionHeld {
                    name: Some(session.to_string()),
                    from: if for_kind {
                        SessionFrom::StreamKind
                    } else {
                        SessionFrom::StreamStage
                    },
                },
                title: TitleHeld {
                    title: session.to_string(),
                    from: TitleFrom::Session,
                },
            })
        }
        several => {
            let whose: Vec<String> = several
                .iter()
                .map(|(name, _, words)| format!("the stream {name}'s, {}", and_list(words)))
                .collect();
            Err(format!(
                "the session {session} is given by {} streams' words, {}, and a person opens \
                 by its name a session one stream's word gives alone; mend the host's \
                 configuration so one stream gives it",
                several.len(),
                and_list(&whose)
            ))
        }
    }
}

/// Why `session` opens nothing: no stream's word gives it, with each
/// stream's sessions, and, where it is a stream's name, that a person
/// opens one of its sessions, never a stream (4, 186, 199).
fn no_such_session(session: &str, streams: &BTreeMap<String, StreamRoom>) -> String {
    let each: Vec<String> = streams
        .iter()
        .map(|(name, room)| {
            let mut sessions: Vec<String> = vec![];
            if room.own_session(name).name.is_none() {
                sessions.push("the terminal's own, opened by no name".to_string());
            }
            for (named, _) in room.words(name) {
                if !sessions.contains(&named) {
                    sessions.push(named);
                }
            }
            format!("the stream {name} gives {}", and_list(&sessions))
        })
        .collect();
    let stream = if streams.contains_key(session) {
        format!(
            "; {session} is a stream's name, and a person opens one of a stream's sessions by \
             the session's name, never a stream"
        )
    } else {
        String::new()
    };
    format!(
        "no stream's word gives a session {session}{stream}; each stream's sessions: {}",
        each.join("; ")
    )
}

/// The title of a window on the stream `name`'s own session: the
/// stream's `title`, else its name (143, 186).
fn own_title(name: &str, declared: Option<&Stream>) -> TitleHeld {
    match declared.and_then(|stream| stream.title.clone()) {
        Some(title) => TitleHeld {
            title,
            from: TitleFrom::Stream,
        },
        None => TitleHeld {
            title: name.to_string(),
            from: TitleFrom::StreamName,
        },
    }
}

impl ViewCommandStatus {
    /// The command by its program, its arguments and its variables'
    /// names alone (core.30).
    fn of(command: &ViewCommand) -> ViewCommandStatus {
        ViewCommandStatus {
            program: command.program.clone(),
            arguments: command.arguments.clone(),
            environment: command.environment.keys().cloned().collect(),
        }
    }
}
