//! A person opens a stream's agents in its view (186; core.48): the
//! stream's own session, resolved as a start resolves its stream's, shown
//! through the stream's multiplexer in its view, the stream's else the
//! host's, else a terminal's: for a terminal view, a window its terminal
//! app opens running the terminal's attach under the stream's title,
//! else its name; for the app, the multiplexer's own app told to show it
//! (107, 187, 188). It takes no lock, writes no held state and settles
//! nothing, so a second asking opens a second window (core.17); and its
//! answer names the command's variables, never their values (core.30).

use crate::room::StreamRoom;
use crate::terminals::{uncarried, Setting};
use crate::{Answer, Engine, StreamOpened, TitleFrom, TitleHeld, ViewCommandStatus, WindowOutcome};
use flume_seams::*;

impl Engine {
    /// The stream `stream`'s session shown in its view, `default`'s where
    /// none is named; refused, naming why and opening nothing, for a
    /// stream the host does not declare, a multiplexer or a terminal app
    /// naming no profile, and a terminal's attach or a window's show
    /// refused in its own words (4, 107, 186 to 190).
    pub fn open_stream(&self, stream: Option<&str>) -> Answer {
        let name = stream.unwrap_or(DEFAULT_STREAM);
        let streams = self.host.streams();
        let Some(declared) = streams.get(name) else {
            let names: Vec<&str> = streams.keys().map(String::as_str).collect();
            return Answer::refused(format!(
                "the host declares no stream {name}; its streams are: {}",
                names.join(", ")
            ));
        };
        let display = self.host.display_for(name);
        let Some(terminal) = self.resolver.terminal(&display.multiplexer.profile) else {
            return Answer::refused(uncarried(
                name,
                Setting::Multiplexer,
                &display.multiplexer,
                &self.resolver.terminal_profiles(),
            ));
        };
        let session = StreamRoom {
            session: declared.session.clone(),
            ..StreamRoom::default()
        }
        .own_session(name);
        let opened = |title, terminal_app, command, window| {
            Answer::Opened(StreamOpened {
                stream: name.to_string(),
                multiplexer: display.multiplexer.clone(),
                session: session.clone(),
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
                let title = match &declared.title {
                    Some(title) => TitleHeld {
                        title: title.clone(),
                        from: TitleFrom::Stream,
                    },
                    None => TitleHeld {
                        title: name.to_string(),
                        from: TitleFrom::StreamName,
                    },
                };
                let Some(window) = self.resolver.window(&display.terminal_app.profile) else {
                    return Answer::refused(uncarried(
                        name,
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
