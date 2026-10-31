//! The terminal profile that prints the command (C.5; design 12): the
//! terminal seam's scenarios against it, its refusals among them, a tell
//! refused with nothing typed, the command status gives, and an agent
//! read stopped once its record says its session ended (134).

use crate::common::*;
use flume::profiles::store::state_dir::StateDir;
use flume::profiles::terminal::printed::Printed;
use flume_conformance::engine::world::{launch, launched, World};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::terminal::{self, TerminalHarness};
use flume_core::{Answer, Engine};
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
}

#[test]
fn the_printed_command_passes_the_terminal_scenarios() {
    pass_all("terminal", &terminal::scenarios(), || Harness {
        scratch: Scratch::new("printed"),
        seam: Printed,
    });
}

/// An engine over `world`'s fakes with the printed command for its
/// terminal and the store in `scratch`'s state directory, where the
/// printed command keeps its marks.
fn printed(world: &World, scratch: &Scratch) -> Engine {
    Engine::new(
        world.resolver.clone(),
        Arc::new(Printed),
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
