//! The terminal seam's scenarios (B.5, 94 to 98, 122, 136 to 139, 7),
//! and a recipient's session reached by its name (43, 132).

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub trait TerminalHarness {
    type Seam: Terminal + ?Sized;

    fn seam(&self) -> &Self::Seam;

    fn place(&self) -> PlaceRef;

    /// The place's own folder, where the terminal keeps what it keeps of
    /// a start (128).
    fn folder(&self) -> String;

    /// A command that runs an agent which waits for input.
    fn command(&self) -> AgentCommand;

    /// Whether the terminal sends to a running agent at all.
    fn sends(&self) -> bool;

    /// Whether the terminal ends an agent at all, rather than leaving its
    /// end to a person (97, C.5).
    fn closes(&self) -> bool;

    /// The agent under `identity` will not end when closed.
    fn stubborn(&self, identity: &Identity);

    /// The session the scenario's own command runs in, as the terminal
    /// names it, or none where it names none.
    fn session(&self) -> Option<String>;

    /// Whether the terminal shows anything of an agent at all (136).
    fn shows(&self) -> bool;

    /// `text` left typed in the agent's input and never submitted.
    fn leave_typed(&self, identity: &Identity, text: &str);

    /// Each input the agent took as submitted, in order.
    fn submitted(&self, identity: &Identity) -> Vec<String>;

    /// A command whose program stops at a question it asks before it
    /// starts, which flume does not answer; none where the terminal
    /// cannot read a start held (35, 94).
    fn held(&self) -> Option<AgentCommand>;

    /// A command whose program never comes up; none where the terminal
    /// cannot read that it did not (35, 94).
    fn never_up(&self) -> Option<AgentCommand>;

    /// Whether the terminal makes anything to hold a place's agents,
    /// which its tidy closes (137, 138).
    fn makes(&self) -> bool;

    /// A person opens a pane of their own in what the terminal made for
    /// the place's repository and base.
    fn person_pane(&self, place: &PlaceRef);

    /// A person moves the agent's pane elsewhere in the terminal.
    fn move_pane(&self, identity: &Identity);

    /// Whether the terminal shows clues at all (139).
    fn shows_clues(&self) -> bool;

    /// A recipient's session the harness started, which is no agent of
    /// flume's, by the name the terminal gives it, waiting for input; none
    /// where the terminal reaches no session (43, 95).
    fn recipient(&self) -> Option<String>;

    /// Each input the recipient's session named `session` took as
    /// submitted, in order.
    fn received(&self, session: &str) -> Vec<String>;

    /// A session name the terminal holds no session by.
    fn no_session(&self) -> String;
}

fn identity(start: u32) -> Identity {
    Identity {
        repository: "0123456789ab".into(),
        place: "fix--one".into(),
        stage: "code".into(),
        start,
    }
}

fn opened<H: TerminalHarness>(harness: &H, start: u32) -> Result<Identity, String> {
    let who = identity(start);
    match harness.seam().open(
        &harness.place(),
        &harness.folder(),
        &harness.command(),
        &who,
        None,
        None,
    ) {
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
        Scenario {
            name: "terminal: screen gives what the terminal shows of the agent, or nothing",
            cites: &["136", "35"],
            run: screen_shows_the_agent::<H>,
        },
        Scenario {
            name: "terminal: send clears what the agent holds typed, and leaves nothing typed",
            cites: &["95", "38", "S43"],
            run: send_clears_what_is_typed::<H>,
        },
        Scenario {
            name: "terminal: open answers a start held at a question its program asks first, with the screen",
            cites: &["94", "35", "34", "S40"],
            run: open_answers_held::<H>,
        },
        Scenario {
            name: "terminal: open answers a program that did not come up within the bound, with the screen",
            cites: &["94", "35", "S45"],
            run: open_answers_not_come_up::<H>,
        },
        Scenario {
            name: "terminal: tidy closes what it made for a place gone, and its base's once nothing live is in it",
            cites: &["138", "137", "S47"],
            run: tidy_closes_what_it_made::<H>,
        },
        Scenario {
            name: "terminal: tidy keeps what a person's pane holds, named, and closes nothing it did not make",
            cites: &["138", "137", "S48"],
            run: tidy_keeps_what_a_person_holds::<H>,
        },
        Scenario {
            name: "terminal: show gives a place's clues or shows none, and nothing is read back",
            cites: &["139"],
            run: show_gives_clues::<H>,
        },
        Scenario {
            name: "terminal: an agent's pane moved elsewhere is found again, read and sent to there",
            cites: &["S46", "95", "96", "30"],
            run: a_moved_pane_found_again::<H>,
        },
        Scenario {
            name: "terminal: a recipient's session is sent to and read by its name, and a name no session holds is not found",
            cites: &["43", "95", "96", "98"],
            run: a_session_by_its_name::<H>,
        },
        Scenario {
            name: "terminal: an agent opened under a session name is reached by it, and no second agent carries it while it lives",
            cites: &["132", "94", "98", "43"],
            run: an_agent_under_a_session_name::<H>,
        },
    ]
}

fn a_session_by_its_name<H: TerminalHarness>(harness: &H) -> Outcome {
    let nobody = harness.no_session();
    ensure_eq!(
        harness.seam().session_state(&nobody),
        None,
        "the state of a name no session holds"
    );
    match harness.seam().send_to(&nobody, "words") {
        ToSession::NotFound => {}
        ToSession::Refused { .. } if !harness.sends() => {}
        other => return Err(format!("a send to a name no session holds gave {other:?}")),
    }
    let Some(session) = harness.recipient() else {
        return Ok(());
    };
    let state = harness.seam().session_state(&session);
    ensure!(
        state.is_some_and(|state| state != AgentState::Stopped),
        "the recipient's session read {state:?}"
    );
    let sent = harness.seam().send_to(&session, "a notice");
    if !harness.sends() {
        ensure!(
            matches!(sent, ToSession::Refused { .. }),
            "a terminal that cannot send sent {sent:?}"
        );
        return Ok(());
    }
    ensure_eq!(sent, ToSession::Sent, "the send to the recipient");
    ensure_eq!(
        harness.received(&session).last().cloned(),
        Some("a notice".to_string()),
        "what the recipient took"
    );
    Ok(())
}

fn an_agent_under_a_session_name<H: TerminalHarness>(harness: &H) -> Outcome {
    if !harness.sends() {
        return Ok(());
    }
    let name = "madswan/planner";
    let first = identity(1);
    let opened = harness.seam().open(
        &harness.place(),
        &harness.folder(),
        &harness.command(),
        &first,
        None,
        Some(name),
    );
    ensure!(
        matches!(opened, Opened::Opened { .. }),
        "open under a session name gave {opened:?}"
    );
    let state = harness.seam().session_state(name);
    ensure!(
        state.is_some_and(|state| state != AgentState::Stopped),
        "the session {name} read {state:?}"
    );
    ensure_eq!(
        harness.seam().send_to(name, "a notice"),
        ToSession::Sent,
        "the send by the session name"
    );
    ensure_eq!(
        harness.submitted(&first).last().cloned(),
        Some("a notice".to_string()),
        "what the agent took"
    );
    let second = harness.seam().open(
        &harness.place(),
        &harness.folder(),
        &harness.command(),
        &identity(2),
        None,
        Some(name),
    );
    ensure!(
        matches!(second, Opened::Refused { .. }),
        "a second agent opened under the live session name {name}: {second:?}"
    );
    Ok(())
}

fn open_answers_held<H: TerminalHarness>(harness: &H) -> Outcome {
    let Some(command) = harness.held() else {
        return Ok(());
    };
    let who = identity(1);
    match harness.seam().open(
        &harness.place(),
        &harness.folder(),
        &command,
        &who,
        None,
        None,
    ) {
        Opened::Held { screen } => {
            ensure!(!screen.trim().is_empty(), "a held start with no screen");
            Ok(())
        }
        other => Err(format!(
            "open of a program held at a question gave {other:?}"
        )),
    }
}

fn open_answers_not_come_up<H: TerminalHarness>(harness: &H) -> Outcome {
    let Some(command) = harness.never_up() else {
        return Ok(());
    };
    let who = identity(1);
    match harness.seam().open(
        &harness.place(),
        &harness.folder(),
        &command,
        &who,
        None,
        None,
    ) {
        Opened::NotComeUp { .. } => Ok(()),
        other => Err(format!(
            "open of a program that never comes up gave {other:?}"
        )),
    }
}

fn tidy_closes_what_it_made<H: TerminalHarness>(harness: &H) -> Outcome {
    let who = opened(harness, 1)?;
    if harness.closes() {
        ensure_eq!(harness.seam().close(&who), Closed::Closed, "close");
    }
    let tidy = harness.seam().tidy(&harness.place());
    ensure!(
        tidy.kept.is_empty(),
        "kept with nothing of a person's: {tidy:?}"
    );
    if harness.makes() {
        ensure!(
            !tidy.closed.is_empty(),
            "a terminal that makes closed nothing"
        );
    } else {
        ensure!(
            tidy.closed.is_empty(),
            "a terminal that makes nothing closed {tidy:?}"
        );
    }
    let again = harness.seam().tidy(&harness.place());
    ensure!(
        again.closed.is_empty() && again.kept.is_empty(),
        "a second tidy did something: {again:?}"
    );
    ensure!(
        harness.seam().kept().is_empty(),
        "kept: {:?}",
        harness.seam().kept()
    );
    Ok(())
}

fn tidy_keeps_what_a_person_holds<H: TerminalHarness>(harness: &H) -> Outcome {
    if !harness.makes() {
        return Ok(());
    }
    let who = opened(harness, 1)?;
    harness.person_pane(&harness.place());
    if harness.closes() {
        ensure_eq!(harness.seam().close(&who), Closed::Closed, "close");
    }
    let tidy = harness.seam().tidy(&harness.place());
    ensure!(
        !tidy.kept.is_empty(),
        "nothing kept for a person's pane: {tidy:?}"
    );
    ensure!(
        !harness.seam().kept().is_empty(),
        "kept names nothing a person's pane holds"
    );
    Ok(())
}

fn show_gives_clues<H: TerminalHarness>(harness: &H) -> Outcome {
    let who = opened(harness, 1)?;
    let clues = Clues {
        stage: "code".into(),
        state: "working".into(),
        agent: Some(who),
        base: BaseCount {
            running: 1,
            ..BaseCount::default()
        },
    };
    let shown = harness.seam().show(&harness.place(), &clues);
    if harness.shows_clues() {
        ensure_eq!(shown, Shown::Shown, "the clues");
    } else {
        ensure_eq!(
            shown,
            Shown::ShowsNone,
            "the clues of a terminal that shows none"
        );
    }
    Ok(())
}

fn a_moved_pane_found_again<H: TerminalHarness>(harness: &H) -> Outcome {
    if !harness.sends() {
        return Ok(());
    }
    let who = opened(harness, 1)?;
    harness.move_pane(&who);
    ensure!(
        harness.seam().state(&who) != AgentState::Stopped,
        "a moved agent reads stopped"
    );
    ensure_eq!(harness.seam().send(&who, "words"), Sent::Sent, "a send");
    ensure_eq!(
        harness.submitted(&who).last().cloned(),
        Some("words".to_string()),
        "what the moved agent took"
    );
    Ok(())
}

fn screen_shows_the_agent<H: TerminalHarness>(harness: &H) -> Outcome {
    let who = opened(harness, 1)?;
    let screen = harness.seam().screen(&who);
    if harness.shows() {
        ensure!(
            screen.is_some_and(|screen| !screen.trim().is_empty()),
            "a terminal that shows its agents showed nothing"
        );
    } else {
        ensure_eq!(screen, None, "the screen of a terminal that shows nothing");
    }
    Ok(())
}

fn send_clears_what_is_typed<H: TerminalHarness>(harness: &H) -> Outcome {
    if !harness.sends() {
        return Ok(());
    }
    let who = opened(harness, 1)?;
    harness.leave_typed(&who, "left typed ");
    ensure_eq!(harness.seam().send(&who, "words"), Sent::Sent, "a send");
    ensure_eq!(
        harness.submitted(&who).last().cloned(),
        Some("words".to_string()),
        "what the agent took after the send"
    );
    Ok(())
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
            harness.seam().open(
                &harness.place(),
                &harness.folder(),
                &harness.command(),
                &who,
                None,
                None
            ),
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
    if !harness.closes() {
        ensure!(
            matches!(harness.seam().close(&who), Closed::Refused { .. }),
            "a terminal that leaves an agent's end to a person reported it ended"
        );
        ensure!(
            harness.seam().state(&who) != AgentState::Stopped,
            "an agent no close ended reads stopped"
        );
        return Ok(());
    }
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
