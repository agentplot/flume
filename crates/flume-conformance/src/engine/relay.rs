//! A place's live agent relayed to a caller that carries a person's
//! screen and keys (107, 221, 222): the place-relay scenarios over the
//! fakes, each driven by a scripted link of its own. The relay reaches
//! the current stage's live agent through the terminal its start
//! recorded (190), gives the screen at once and again only when it
//! changes, an unread read once (96, 98, 136), types the person's input
//! as it comes and never through a send (95), ends at its caller's close
//! or once the agent's terminal goes (96, 120), is refused for a place
//! with no live agent or a terminal that offers none (4, 70), and writes
//! and keeps nothing (56). The suite's clause list is read before 221
//! and 222, so each scenario names them in its words and cites the
//! clauses the list holds.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, RelayEnded, RelayLine, RelayLink, RelayNext};
use flume_seams::*;
use std::collections::VecDeque;
use std::time::Duration;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "place relay: the relay (221) of a fix whose code agent is live gives the link the agent's screen first and answers relayed, naming the place and the agent of code's first start, ended by the caller",
            cites: &["107", "190"],
            run: the_current_stages_agent,
        },
        Scenario {
            name: "place relay: a start recorded in another terminal is relayed (221) through that terminal, and the host's own is asked no relay",
            cites: &["190", "107"],
            run: a_start_recorded_in_another_terminal,
        },
        Scenario {
            name: "place relay: the relay (221) of a place whose first stage waits for room is refused naming the place and that it has no live agent, and no terminal is asked to relay",
            cites: &["4", "70"],
            run: a_place_waiting_for_room,
        },
        Scenario {
            name: "place relay: a terminal that offers no relay (222) refuses the relay in its own words, and the link is given no line",
            cites: &["4"],
            run: a_terminal_that_offers_no_relay,
        },
        Scenario {
            name: "place relay: a screen read three times and then changed is given once and then once more, and no more (221, 222)",
            cites: &["136"],
            run: a_screen_given_once_while_it_stands,
        },
        Scenario {
            name: "place relay: a read the terminal does not answer is given once as unread until a read answers, and the agent's state is read at once after it (221, 222)",
            cites: &["96", "98", "136"],
            run: a_read_the_terminal_does_not_answer,
        },
        Scenario {
            name: "place relay: a person's text typed by the relay (221, 222) goes onto what the agent holds typed with nothing cleared, enter submits both, and the terminal's send is asked nothing",
            cites: &["95", "107"],
            run: text_onto_what_was_left_typed,
        },
        Scenario {
            name: "place relay: an input the terminal refuses is given back as untyped in its words, and the relay (221, 222) goes on to type the next",
            cites: &["107"],
            run: an_input_the_terminal_refuses,
        },
        Scenario {
            name: "place relay: the relay (221) ends as its caller's when its link closes, or when the link takes no more lines",
            cites: &["120"],
            run: the_caller_closes,
        },
        Scenario {
            name: "place relay: an agent read stopped at the first tick of a new second of the host's clock ends the relay (221) gone, its screen still answering",
            cites: &["96"],
            run: the_agent_stops_while_its_screen_reads,
        },
        Scenario {
            name: "place relay: an agent closed while the relay (221) runs reads gone at the next read, which ends it gone",
            cites: &["96"],
            run: the_agent_closed,
        },
        Scenario {
            name: "place relay: a relay (221) writes and keeps nothing: the held state stands at its version, status answers as before, and no document holds what was typed or shown",
            cites: &["56", "120"],
            run: nothing_written,
        },
    ]
}

/// What a scripted link does when the relay asks it for its next: an
/// act run first, as what happens between two of its calls, or what it
/// answers.
enum Step {
    Do(Box<dyn FnOnce()>),
    Next(RelayNext),
}

/// A link of the scenario's own: it answers its steps in turn and the
/// caller's close once they are spent, keeps each line it was given and
/// each bound it was asked under, and takes no more lines past `takes`
/// where a scenario sets it.
struct Script {
    steps: VecDeque<Step>,
    given: Vec<RelayLine>,
    asked: Vec<Duration>,
    takes: Option<usize>,
}

impl Script {
    fn new(steps: Vec<Step>) -> Script {
        Script {
            steps: steps.into(),
            given: vec![],
            asked: vec![],
            takes: None,
        }
    }

    /// Ticks `count` times, then closes.
    fn ticks(count: usize) -> Script {
        Script::new((0..count).map(|_| Step::Next(RelayNext::Tick)).collect())
    }

    /// The screens given, in order.
    fn screens(&self) -> Vec<String> {
        self.given
            .iter()
            .filter_map(|line| match line {
                RelayLine::Screen(screen) => Some(screen.clone()),
                _ => None,
            })
            .collect()
    }
}

impl RelayLink for Script {
    fn give(&mut self, line: &RelayLine) -> bool {
        if self.takes.is_some_and(|takes| self.given.len() >= takes) {
            return false;
        }
        self.given.push(line.clone());
        true
    }

    fn next(&mut self, every: Duration) -> RelayNext {
        self.asked.push(every);
        loop {
            match self.steps.pop_front() {
                Some(Step::Do(act)) => act(),
                Some(Step::Next(next)) => return next,
                None => return RelayNext::Closed,
            }
        }
    }
}

fn tick() -> Step {
    Step::Next(RelayNext::Tick)
}

fn typed(input: Typed) -> Step {
    Step::Next(RelayNext::Typed(input))
}

/// A fix named one launched, its code stage's first start live under the
/// fake terminal; its place and its agent.
fn a_live_fix(world: &World) -> Result<(PlaceId, Identity), String> {
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let id = World::id("fix", "one");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the fix's code stage"
    );
    Ok((id, World::identity("fix", "one", "code", 1)))
}

/// The relay's answer, or what it answered.
fn relayed(answer: &Answer) -> Result<(PlaceId, String, RelayEnded), String> {
    match answer {
        Answer::Relayed {
            place,
            agent,
            ended,
        } => Ok((place.clone(), agent.clone(), *ended)),
        other => Err(format!("the relay answered {other:?}")),
    }
}

fn the_current_stages_agent(_: &()) -> Outcome {
    let world = World::new();
    let (id, agent) = a_live_fix(&world)?;
    world.terminal.set_screen(&agent, Some("the code agent"));
    let mut link = Script::new(vec![]);
    let answer = world.engine().relay(&id, &mut link);
    ensure_eq!(
        relayed(&answer)?,
        (id.clone(), agent.to_string(), RelayEnded::Caller),
        "the answer"
    );
    ensure_eq!(
        link.given,
        vec![RelayLine::Screen("the code agent".into())],
        "the lines given"
    );
    ensure_eq!(
        link.asked,
        vec![Duration::from_millis(250)],
        "the bound the link was asked under, the terminal's"
    );
    ensure_eq!(
        world.terminal.log.of("relay").first().cloned(),
        Some(format!("relay {agent}")),
        "the terminal's relay"
    );
    Ok(())
}

fn a_start_recorded_in_another_terminal(_: &()) -> Outcome {
    let world = World::new();
    let other = world.other_terminal("other");
    world.host.set_host_display(Some("fake"), None, None);
    world.host.set_display("trial", Some("other"), None, None);
    launched(&world.engine.launch(&Launch {
        stream: Some("trial".into()),
        ..launch("fix", "one", Some("x"))
    }))?;
    let id = World::id("fix", "one");
    let agent = World::identity("fix", "one", "code", 1);
    ensure!(
        other.opened(&agent).is_some(),
        "the fix's agent did not open in other"
    );
    other.set_screen(&agent, Some("in other"));
    let mut link = Script::new(vec![typed(Typed::Text("through other".into()))]);
    let answer = world.engine().relay(&id, &mut link);
    ensure_eq!(
        relayed(&answer)?,
        (id, agent.to_string(), RelayEnded::Caller),
        "the answer"
    );
    ensure_eq!(
        link.given,
        vec![RelayLine::Screen("in other".into())],
        "the lines given"
    );
    ensure_eq!(
        other.typed(&agent).map(|(typed, _)| typed),
        Some("through other".to_string()),
        "what the agent holds typed in other"
    );
    ensure!(
        world.terminal.log.of("relay").is_empty(),
        "the host's own terminal was asked: {:?}",
        world.terminal.log.of("relay")
    );
    Ok(())
}

fn a_place_waiting_for_room(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(1));
    for name in ["one", "two"] {
        launched(&world.engine.launch(&launch("fix", name, Some("x"))))?;
    }
    let id = World::id("fix", "two");
    ensure!(
        world
            .terminal
            .opened(&World::identity("fix", "two", "code", 1))
            .is_none(),
        "the second fix's agent opened"
    );
    world.terminal.log.clear();
    let mut link = Script::ticks(1);
    let reason = refused(&world.engine().relay(&id, &mut link))?;
    ensure!(
        reason.contains(&id.to_string()) && reason.contains("no live agent"),
        "the refusal: {reason}"
    );
    ensure!(link.given.is_empty(), "lines given: {:?}", link.given);
    ensure!(
        world.terminal.log.calls().is_empty(),
        "the terminal was asked: {:?}",
        world.terminal.log.calls()
    );
    Ok(())
}

fn a_terminal_that_offers_no_relay(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = a_live_fix(&world)?;
    world.terminal.refuse_relay(Some("no relay here"));
    let mut link = Script::ticks(1);
    ensure_eq!(
        refused(&world.engine().relay(&id, &mut link))?,
        "no relay here".to_string(),
        "the refusal"
    );
    ensure!(link.given.is_empty(), "lines given: {:?}", link.given);
    Ok(())
}

fn a_screen_given_once_while_it_stands(_: &()) -> Outcome {
    let world = World::new();
    let (id, agent) = a_live_fix(&world)?;
    world.terminal.set_screen(&agent, Some("two"));
    world.terminal.relay_screens(&["one", "one", "one"]);
    let mut link = Script::ticks(4);
    relayed(&world.engine().relay(&id, &mut link))?;
    ensure_eq!(
        link.given,
        vec![
            RelayLine::Screen("one".into()),
            RelayLine::Screen("two".into())
        ],
        "the lines given"
    );
    ensure_eq!(
        world.terminal.log.of("relay").len(),
        6,
        "the relay asked once and read five times"
    );
    Ok(())
}

fn a_read_the_terminal_does_not_answer(_: &()) -> Outcome {
    let world = World::new();
    let (id, agent) = a_live_fix(&world)?;
    world.terminal.set_screen(&agent, Some("one"));
    world.terminal.relay_screens(&["one"]);
    world.terminal.relay_unread("slow");
    world.terminal.relay_unread("slow");
    world.terminal.log.clear();
    let mut link = Script::ticks(3);
    relayed(&world.engine().relay(&id, &mut link))?;
    ensure_eq!(
        link.given,
        vec![
            RelayLine::Screen("one".into()),
            RelayLine::Unread("slow".into())
        ],
        "the lines given"
    );
    let calls = world.terminal.log.calls();
    let second = calls
        .iter()
        .enumerate()
        .filter(|(_, call)| *call == "relay read")
        .nth(1)
        .map(|(at, _)| at)
        .ok_or("no second read")?;
    ensure_eq!(
        calls.get(second + 1).cloned(),
        Some(format!("state {agent}")),
        "the call after the second read"
    );
    Ok(())
}

fn text_onto_what_was_left_typed(_: &()) -> Outcome {
    let world = World::new();
    let (id, agent) = a_live_fix(&world)?;
    world.terminal.leave_typed(&agent, "draft");
    world.terminal.log.clear();
    let mut link = Script::new(vec![
        typed(Typed::Text(" more".into())),
        typed(Typed::Keys(vec![Key::bare(KeyName::Enter)])),
    ]);
    relayed(&world.engine().relay(&id, &mut link))?;
    ensure_eq!(
        world.terminal.sent_to(&agent).last().cloned(),
        Some("draft more".to_string()),
        "what the agent took as submitted"
    );
    ensure_eq!(
        world.terminal.typed(&agent),
        Some((String::new(), 0)),
        "what the agent holds typed, and how many times it was cleared"
    );
    let calls = world.terminal.log.calls();
    ensure!(
        !calls.iter().any(|call| call.starts_with("send")),
        "the terminal's send was asked: {calls:?}"
    );
    ensure_eq!(
        calls
            .iter()
            .filter(|call| call.starts_with("relay text") || call.starts_with("relay keys"))
            .cloned()
            .collect::<Vec<_>>(),
        vec!["relay text".to_string(), "relay keys enter".to_string()],
        "what the relay typed, no ctrl+c before the text"
    );
    Ok(())
}

fn an_input_the_terminal_refuses(_: &()) -> Outcome {
    let world = World::new();
    let (id, agent) = a_live_fix(&world)?;
    world.terminal.set_screen(&agent, Some("ready"));
    world.terminal.refuse_next_typed("no such key");
    let mut link = Script::new(vec![
        typed(Typed::Keys(vec![Key::bare(KeyName::F(12))])),
        typed(Typed::Text("yes".into())),
    ]);
    let answer = world.engine().relay(&id, &mut link);
    ensure_eq!(relayed(&answer)?.2, RelayEnded::Caller, "what ended it");
    ensure_eq!(
        link.given,
        vec![
            RelayLine::Screen("ready".into()),
            RelayLine::Untyped("no such key".into())
        ],
        "the lines given"
    );
    ensure_eq!(
        world.terminal.typed(&agent).map(|(typed, _)| typed),
        Some("yes".to_string()),
        "what the agent holds typed"
    );
    Ok(())
}

fn the_caller_closes(_: &()) -> Outcome {
    let world = World::new();
    let (id, agent) = a_live_fix(&world)?;
    let mut link = Script::ticks(1);
    ensure_eq!(
        relayed(&world.engine().relay(&id, &mut link))?,
        (id.clone(), agent.to_string(), RelayEnded::Caller),
        "a link that closes after one tick"
    );
    world.terminal.set_screen(&agent, Some("first"));
    world.terminal.relay_screens(&["first", "second"]);
    let mut full = Script::ticks(5);
    full.takes = Some(1);
    ensure_eq!(
        relayed(&world.engine().relay(&id, &mut full))?,
        (id, agent.to_string(), RelayEnded::Caller),
        "a link that takes no more lines past its first"
    );
    ensure_eq!(
        full.given,
        vec![RelayLine::Screen("first".into())],
        "the lines it took"
    );
    ensure_eq!(full.asked.len(), 1, "the link asked for its next once");
    Ok(())
}

fn the_agent_stops_while_its_screen_reads(_: &()) -> Outcome {
    let world = World::new();
    let (id, agent) = a_live_fix(&world)?;
    world
        .terminal
        .schedule(&agent, world.host.now() + 1, AgentState::Stopped);
    let host = world.host.clone();
    let mut link = Script::new(vec![
        tick(),
        Step::Do(Box::new(move || host.advance(1))),
        tick(),
        tick(),
    ]);
    ensure_eq!(
        relayed(&world.engine().relay(&id, &mut link))?,
        (id, agent.to_string(), RelayEnded::Gone),
        "the answer"
    );
    ensure_eq!(link.asked.len(), 2, "the link asked for its next twice");
    ensure_eq!(
        world
            .terminal
            .log
            .of("relay")
            .iter()
            .filter(|call| *call == "relay read")
            .count(),
        3,
        "the reads, each answering a screen"
    );
    Ok(())
}

fn the_agent_closed(_: &()) -> Outcome {
    let world = World::new();
    let (id, agent) = a_live_fix(&world)?;
    let terminal = world.terminal.clone();
    let closed = agent.clone();
    let mut link = Script::new(vec![
        tick(),
        Step::Do(Box::new(move || {
            terminal.close(&closed);
        })),
        tick(),
        tick(),
    ]);
    ensure_eq!(
        relayed(&world.engine().relay(&id, &mut link))?,
        (id, agent.to_string(), RelayEnded::Gone),
        "the answer"
    );
    ensure_eq!(link.asked.len(), 2, "the link asked for its next twice");
    Ok(())
}

fn nothing_written(_: &()) -> Outcome {
    let world = World::new();
    let (id, agent) = a_live_fix(&world)?;
    world
        .terminal
        .set_screen(&agent, Some("the agent shows secret-screen"));
    let before = world.store.read(&id).ok_or("no held state")?;
    let status = world.status(&id)?;
    world.store.log.clear();
    let mut link = Script::new(vec![typed(Typed::Text("secret-typed".into())), tick()]);
    relayed(&world.engine().relay(&id, &mut link))?;
    ensure!(
        link.screens()
            .iter()
            .any(|screen| screen.contains("secret-screen")),
        "no screen given holds secret-screen: {:?}",
        link.given
    );
    ensure_eq!(
        world.store.log.calls(),
        vec![format!("read {id}")],
        "the store's calls during the relay"
    );
    let after = world.store.read(&id).ok_or("no held state after")?;
    ensure_eq!(after.version, before.version, "the held state's version");
    ensure_eq!(
        after.state,
        before.state,
        "the held state, its acts among it"
    );
    ensure_eq!(world.status(&id)?, status, "the place's status");
    for secret in ["secret-typed", "secret-screen"] {
        ensure!(
            world
                .store
                .every_text()
                .iter()
                .all(|text| !text.contains(secret)),
            "a document of the store holds {secret}"
        );
    }
    Ok(())
}
