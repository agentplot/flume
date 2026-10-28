//! The terminal seam's scenarios (B.5, 94 to 98, 122, 7).

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub trait TerminalHarness {
    type Seam: Terminal + ?Sized;

    fn seam(&self) -> &Self::Seam;

    fn place(&self) -> PlaceRef;

    /// A command that runs an agent which waits for input.
    fn command(&self) -> AgentCommand;

    /// Whether the terminal sends to a running agent at all.
    fn sends(&self) -> bool;

    /// The agent under `identity` will not end when closed.
    fn stubborn(&self, identity: &Identity);

    /// The session the scenario's own command runs in, as the terminal
    /// names it, or none where it names none.
    fn session(&self) -> Option<String>;
}

fn identity(start: u32) -> Identity {
    Identity {
        repository: "/repository".into(),
        place: "fix--one".into(),
        stage: "code".into(),
        start,
    }
}

fn opened<H: TerminalHarness>(harness: &H, start: u32) -> Result<Identity, String> {
    let who = identity(start);
    match harness
        .seam()
        .open(&harness.place(), &harness.command(), &who, None)
    {
        Opened::Opened { .. } => Ok(who),
        other => Err(format!("open gave {other:?}")),
    }
}

// ---- the terminal seam: a profile unit appends its own scenarios at the list's end ----

pub fn scenarios<H: TerminalHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name: "terminal: open runs an agent in the place under its identity",
            cites: &["94", "30", "107"],
            run: open_runs_an_agent::<H>,
        },
        Scenario {
            name: "terminal: open never runs a second agent under a live identity",
            cites: &["98", "30", "33", "I2"],
            run: no_second_agent::<H>,
        },
        Scenario {
            name: "terminal: send delivers the text, or refuses; never leaves it typed",
            cites: &["95", "38", "53", "I5"],
            run: send_delivers_or_refuses::<H>,
        },
        Scenario {
            name: "terminal: state reads the agent, stopped once closed",
            cites: &["96", "97", "56"],
            run: state_then_close::<H>,
        },
        Scenario {
            name: "terminal: close never reports an end that did not happen",
            cites: &["98", "31"],
            run: close_never_lies::<H>,
        },
        Scenario {
            name: "terminal: watch states its bound, or that it keeps none",
            cites: &["122", "119"],
            run: watch_states_its_bound::<H>,
        },
        Scenario {
            name: "terminal: session names the session a command runs in, or none",
            cites: &["7"],
            run: session_names_the_callers::<H>,
        },
    ]
}

fn session_names_the_callers<H: TerminalHarness>(harness: &H) -> Outcome {
    let session = harness.seam().session();
    ensure_eq!(session, harness.session(), "the session");
    ensure!(
        session.as_ref().is_none_or(|name| !name.trim().is_empty()),
        "a session named by no name"
    );
    Ok(())
}

fn open_runs_an_agent<H: TerminalHarness>(harness: &H) -> Outcome {
    let who = opened(harness, 1)?;
    ensure!(
        harness.seam().state(&who) != AgentState::Stopped,
        "an opened agent reads stopped"
    );
    Ok(())
}

fn no_second_agent<H: TerminalHarness>(harness: &H) -> Outcome {
    let who = opened(harness, 1)?;
    ensure!(
        matches!(
            harness
                .seam()
                .open(&harness.place(), &harness.command(), &who, None),
            Opened::Refused { .. }
        ),
        "a second agent opened under a live identity"
    );
    Ok(())
}

fn send_delivers_or_refuses<H: TerminalHarness>(harness: &H) -> Outcome {
    let who = opened(harness, 1)?;
    let sent = harness.seam().send(&who, "words");
    if harness.sends() {
        ensure_eq!(sent, Sent::Sent, "a send");
    } else {
        ensure!(
            matches!(sent, Sent::Refused { .. }),
            "a terminal that cannot send sent"
        );
    }
    Ok(())
}

fn state_then_close<H: TerminalHarness>(harness: &H) -> Outcome {
    let who = opened(harness, 1)?;
    ensure_eq!(harness.seam().close(&who), Closed::Closed, "close");
    ensure_eq!(
        harness.seam().state(&who),
        AgentState::Stopped,
        "a closed agent"
    );
    opened(harness, 1).map(|_| ())
}

fn close_never_lies<H: TerminalHarness>(harness: &H) -> Outcome {
    let who = opened(harness, 2)?;
    harness.stubborn(&who);
    ensure!(
        matches!(harness.seam().close(&who), Closed::Refused { .. }),
        "an agent that did not end was reported closed"
    );
    ensure!(
        harness.seam().state(&who) != AgentState::Stopped,
        "an agent that did not end reads stopped"
    );
    Ok(())
}

fn watch_states_its_bound<H: TerminalHarness>(harness: &H) -> Outcome {
    match harness.seam().watch() {
        Watch::Bound { seconds } => {
            ensure!(seconds > 0, "a watch with no bound");
            Ok(())
        }
        Watch::None => Ok(()),
    }
}
