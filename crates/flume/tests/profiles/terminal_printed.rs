//! The terminal profile that prints the command (C.5; design 12): the
//! terminal seam's scenarios against it, its refusals among them, a tell
//! refused with nothing typed, the command status gives, the same
//! command printed whatever session its stream names (107), an agent
//! read stopped once its record says its session ended (134), attach
//! refused for either view with nothing run (187), and runs holding no
//! part, in attach's words, with nothing run (201).

use crate::common::window::StandIn;
use crate::common::*;
use flume::host::System;
use flume::profiles::store::state_dir::StateDir;
use flume::profiles::terminal::printed::Printed;
use flume::profiles::window::terminal::TerminalApp;
use flume_conformance::engine::world::{launch, launched, World};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::terminal::{self, TerminalHarness};
use flume_core::{Answer, Engine, StreamListed};
use flume_seams::*;
use std::sync::Arc;

struct Harness {
    scratch: Scratch,
    seam: Printed,
}

impl TerminalHarness for Harness {
    type Seam = Printed;

    fn seam(&self) -> &Printed {
        &self.seam
    }

    fn place(&self) -> PlaceRef {
        PlaceRef {
            repository: self.scratch.path("repository").to_string_lossy().into(),
            location: self
                .scratch
                .path("repository/places/fix--one")
                .to_string_lossy()
                .into(),
            branch: "fix/one".into(),
            base: "main".into(),
        }
    }

    fn folder(&self) -> String {
        self.scratch.path("state/fix--one").to_string_lossy().into()
    }

    fn command(&self) -> AgentCommand {
        AgentCommand {
            program: "agent".into(),
            prompt_args: vec!["begin the stage".into()],
            ..AgentCommand::default()
        }
    }

    fn sends(&self) -> bool {
        false
    }

    fn closes(&self) -> bool {
        false
    }

    fn stubborn(&self, _identity: &Identity) {}

    fn session(&self) -> Option<String> {
        None
    }

    fn shows(&self) -> bool {
        false
    }

    fn leave_typed(&self, _identity: &Identity, _text: &str) {}

    fn submitted(&self, _identity: &Identity) -> Vec<String> {
        vec![]
    }

    /// A person runs the printed command and sees any question it asks:
    /// the terminal reads no start held.
    fn held(&self) -> Option<AgentCommand> {
        None
    }

    fn never_up(&self) -> Option<AgentCommand> {
        None
    }

    fn makes(&self) -> bool {
        false
    }

    fn person_pane(&self, _place: &PlaceRef) {}

    fn move_pane(&self, _identity: &Identity) {}

    fn shows_clues(&self) -> bool {
        false
    }

    /// No session is the printed command's to reach.
    fn recipient(&self) -> Option<String> {
        None
    }

    fn received(&self, _session: &str) -> Vec<String> {
        vec![]
    }

    fn no_session(&self) -> String {
        "ops".into()
    }

    /// The printed command holds no session a person can be shown, and
    /// refuses both views (187).
    fn views(&self) -> Vec<View> {
        vec![]
    }

    fn parts(&self) -> Vec<Option<String>> {
        vec![Some("work".into()), None]
    }

    fn refuse_attach(&self, _view: View) -> bool {
        false
    }

    /// The printed command holds no part, `work` or its own (201).
    fn runs_parts(&self) -> Vec<(Option<String>, SessionRuns)> {
        [Some("work".to_string()), None]
            .into_iter()
            .map(|part| {
                (
                    part,
                    SessionRuns::NoPart {
                        reason: NO_SESSION.into(),
                    },
                )
            })
            .collect()
    }
}

/// Why the printed command attaches nothing and holds no part (187, 201).
const NO_SESSION: &str =
    "the printed command holds no session a person can be shown; its agents run in the commands \
     it printed";

#[test]
fn the_printed_command_passes_the_terminal_scenarios() {
    pass_all("terminal", &terminal::scenarios(), || Harness {
        scratch: Scratch::new("printed"),
        seam: Printed,
    });
}

#[test]
fn attach_under_the_printed_command_is_refused_for_either_view() {
    // 187, S91: no session a person can be shown, for a window or an app.
    for view in [View::Terminal, View::App] {
        for part in [Some("work"), None] {
            assert_eq!(
                Printed.attach(part, view),
                Attached::Refused {
                    reason: NO_SESSION.into()
                },
                "attach {part:?} for {}",
                view.name()
            );
        }
    }
}

/// Every file under `folder`, by its path.
fn files_under(folder: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut found = vec![];
    let mut todo = vec![folder.to_path_buf()];
    while let Some(directory) = todo.pop() {
        for entry in std::fs::read_dir(&directory)
            .into_iter()
            .flatten()
            .flatten()
        {
            let path = entry.path();
            if path.is_dir() {
                todo.push(path);
            } else {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

#[test]
fn attach_under_the_printed_command_runs_nothing() {
    // 187, S91: `work` under the printed command opened through the core
    // for a window and then for its app, each refused in the printed
    // command's words, and nothing run: Terminal.app's `osascript`, the
    // stream's terminal app, asked for no window, and nothing written
    // where the printed command keeps the commands it prints.
    let scratch = Scratch::new("printed-attach");
    let world = World::new();
    let osascript = StandIn::new(&scratch.path("stand-in"), "osascript");
    world.resolver.with_window(
        "terminal",
        Arc::new(TerminalApp::for_system(System::MacOs).with_program(&osascript.path())),
    );
    world.host.set_stream("work", Some(Stream::default()));
    let engine = printed(&world, &scratch);
    world
        .host
        .set_host_display(Some("printed"), None, Some("terminal"));
    let state = scratch.path("state");
    let before = files_under(&state);
    for view in [View::Terminal, View::App] {
        world.host.set_display("work", None, Some(view), None);
        assert_eq!(
            engine.open_session(Some("work")).refusal(),
            Some(
                "the printed command holds no session a person can be shown; its agents run in \
                 the commands it printed"
            ),
            "the open of work for {}",
            view.name()
        );
    }
    assert!(osascript.runs_settled().is_empty(), "a window program ran");
    assert_eq!(world.window.log.count("show"), 0, "the fake window's shows");
    assert_eq!(files_under(&state), before, "the printed command's marks");
}

#[test]
fn runs_under_the_printed_command_holds_no_part_and_runs_nothing() {
    // 187, 201: `work` and its own part each answered that it holds no
    // part, in attach's words; and the list over the printed command
    // gives each stream `no session`, its window program asked for
    // nothing and nothing written where the printed command keeps the
    // commands it prints.
    for part in [Some("work"), None] {
        assert_eq!(
            Printed.runs(part),
            SessionRuns::NoPart {
                reason: NO_SESSION.into()
            },
            "runs of {part:?}"
        );
    }
    let scratch = Scratch::new("printed-runs");
    let world = World::new();
    let osascript = StandIn::new(&scratch.path("stand-in"), "osascript");
    world.resolver.with_window(
        "terminal",
        Arc::new(TerminalApp::for_system(System::MacOs).with_program(&osascript.path())),
    );
    world.host.set_stream("work", Some(Stream::default()));
    let engine = printed(&world, &scratch);
    world
        .host
        .set_host_display(Some("printed"), None, Some("terminal"));
    let state = scratch.path("state");
    let before = files_under(&state);
    let Answer::List { streams, .. } = engine.list() else {
        panic!("the list answered otherwise");
    };
    let lines: Vec<(&str, &str, Option<&SessionRuns>)> = streams
        .iter()
        .map(
            |StreamListed {
                 stream, line, runs, ..
             }| { (stream.as_str(), line.as_str(), runs.as_ref()) },
        )
        .collect();
    let none = SessionRuns::NoPart {
        reason: NO_SESSION.into(),
    };
    assert_eq!(
        lines,
        vec![
            (DEFAULT_STREAM, "default · no session", Some(&none)),
            ("work", "work · no session", Some(&none)),
        ],
        "the streams' lines"
    );
    assert!(osascript.runs_settled().is_empty(), "a window program ran");
    assert_eq!(files_under(&state), before, "the printed command's marks");
}

/// An engine over `world`'s fakes with the printed command for its
/// terminal, the host's multiplexer and the resolver's `printed`, and the
/// store in `scratch`'s state directory, where the printed command keeps
/// its marks (190).
fn printed(world: &World, scratch: &Scratch) -> Engine {
    let printed: Arc<dyn Terminal> = Arc::new(Printed);
    world.resolver.with_terminal("printed", printed.clone());
    world.host.set_host_display(Some("printed"), None, None);
    Engine::new(
        world.resolver.clone(),
        printed,
        Arc::new(StateDir::at(scratch.path("state/flume"))),
        world.history.clone(),
        world.host.clone(),
        world.defaults.clone(),
    )
}

#[test]
fn the_command_status_gives_carries_the_prompt() {
    let scratch = Scratch::new("printed-status");
    let world = World::new();
    let answer = printed(&world, &scratch).launch(&launch("fix", "one", Some("bump the pin")));
    let place = launched(&answer).expect("the launch");
    let agent = place.agent.expect("an agent in the launch's answer");
    let command = agent.printed.expect("no command printed");
    assert!(
        command.starts_with(&format!("cd {} && ", World::id("fix", "one").0)),
        "the command runs elsewhere than the place: {command}"
    );
    assert!(
        command.contains("bump the pin"),
        "the command carries not the prompt: {command}"
    );
    assert_eq!(agent.state, AgentState::Unknown, "the agent's state");
    assert_eq!(agent.screen, None, "the screen");
}

#[test]
fn a_session_names_nothing_in_the_printed_command() {
    // 107, C.5, design 3: the command printed for an agent of a stream
    // naming its session is the one printed for the default stream's,
    // and status still gives the session as resolved.
    let commands: Vec<(String, Option<SessionHeld>)> = [None, Some("office")]
        .into_iter()
        .map(|session| {
            let scratch = Scratch::new("printed-session");
            let world = World::new();
            if let Some(session) = session {
                world.host.set_stream_session(DEFAULT_STREAM, Some(session));
            }
            let place = launched(&printed(&world, &scratch).launch(&launch(
                "fix",
                "one",
                Some("bump the pin"),
            )))
            .expect("the launch");
            let agent = place.agent.expect("an agent in the launch's answer");
            let printed = agent.printed.expect("no command printed");
            (
                printed.replace(&scratch.root.display().to_string(), "<scratch>"),
                agent.session,
            )
        })
        .collect();
    assert_eq!(commands[0].0, commands[1].0, "the commands printed");
    assert_eq!(
        commands[1].1,
        Some(SessionHeld {
            name: Some("office".into()),
            from: SessionFrom::Stream,
        }),
        "the agent's session"
    );
}

#[test]
fn a_tell_under_the_printed_command_is_refused_with_nothing_typed() {
    let scratch = Scratch::new("printed-tell");
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.set_started(&code, Reading::Yes);
    launched(&printed(&world, &scratch).launch(&launch("fix", "one", Some("x"))))
        .expect("the launch");
    let told = printed(&world, &scratch).tell(&id, "go on", Some(30));
    assert!(
        told.refusal().is_some(),
        "a tell under the printed command was taken: {told:?}"
    );
    assert!(world.terminal.sent_to(&code).is_empty(), "words were typed");
}

#[test]
fn an_agent_whose_record_says_its_session_ended_reads_stopped() {
    let scratch = Scratch::new("printed-ended");
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.set_started(&code, Reading::No);
    launched(&printed(&world, &scratch).launch(&launch("fix", "one", Some("x"))))
        .expect("the launch");
    printed(&world, &scratch).settle(Some(&id));
    let state = |engine: Engine| match engine.status(Some(&id)) {
        Answer::Status { places, .. } => places[0].stages[0].state,
        other => panic!("status answered {other:?}"),
    };
    assert_eq!(
        state(printed(&world, &scratch)),
        StageState::Starting,
        "a start whose agent the terminal cannot read"
    );
    world.agent.set_ended(&code, Reading::Yes);
    printed(&world, &scratch).settle(Some(&id));
    assert_eq!(
        state(printed(&world, &scratch)),
        StageState::NeverBegun,
        "a start whose agent's record says its session ended with no word that it started"
    );
}
