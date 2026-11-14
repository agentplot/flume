//! The terminal seam's scenarios (B.5, 94 to 98, 122, 136 to 139, 7),
//! a recipient's session reached by its name (43, 132), a part's agents
//! given by their names (132, C.5), a part shown to a person in each
//! view the terminal offers (187), whether a part runs (201), and
//! whether the terminal hands a running agent words (95).

use crate::engine::world::World;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Engine;
use flume_seams::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub trait TerminalHarness {
    type Seam: Terminal + ?Sized;

    fn seam(&self) -> &Self::Seam;

    /// The same terminal as `seam`, shared as the engine holds one.
    fn shared(&self) -> Arc<dyn Terminal>;

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

    /// A recipient's session the harness started, which is no agent of
    /// flume's, by the name the terminal gives it, reading working; none
    /// where the terminal can make none, by default (54, 214).
    fn working_recipient(&self) -> Option<String> {
        None
    }

    /// A session name the terminal holds no session by.
    fn no_session(&self) -> String;

    /// The views the terminal offers a person, each answered by attach;
    /// every other is refused (187).
    fn views(&self) -> Vec<View>;

    /// The parts attach is asked to show, a session by its name, started
    /// first where it does not run, or the terminal's own where none,
    /// each one the terminal can show in the views it offers (187).
    fn parts(&self) -> Vec<Option<String>>;

    /// attach refuses `view` from now, where the harness can make it;
    /// whether it did (187).
    fn refuse_attach(&self, view: View) -> bool;

    /// What the harness has the terminal's own configuration lack for
    /// flume's plugin, each in the profile's words, and so what setup is
    /// to give; none for a terminal with no plugin of flume's, as the
    /// printed command, by default (122).
    fn setup(&self) -> Vec<String> {
        vec![]
    }

    /// Each part runs is asked of, a session by its name or the
    /// terminal's own where none, with what runs is to answer of it, the
    /// harness scripting or starting each first; none where the profile's
    /// own tests hold its runs (201).
    fn runs_parts(&self) -> Vec<(Option<String>, SessionRuns)> {
        vec![]
    }

    /// Every call the terminal logged, in order, where it logs them as
    /// the fakes do, each runs as `runs <part> <part>…`, its own part
    /// `own`; none where it logs nothing (201).
    fn logged(&self) -> Option<Vec<String>> {
        None
    }

    /// An agent the harness started under `name` in the part `session`,
    /// which is no agent of flume's, waiting for input; whether it
    /// started one, none by default, for a terminal that holds no agent
    /// a name finds (132, C.5).
    fn named_agent(&self, _session: &str, _name: &str) -> bool {
        false
    }
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
        &OpenIn::Own,
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
            name: "terminal: the engine reads whether it runs attended the same under the fake terminal and this one, from the host's watch held, calling no terminal operation",
            cites: &["122", "208", "119"],
            run: the_same_reading_under_every_terminal::<H>,
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
            name: "terminal: send_to refuses a session that reads working, typing nothing, so a role's session is never typed into while it works",
            cites: &["54", "43", "95"],
            run: a_session_at_work::<H>,
        },
        Scenario {
            name: "terminal: an agent opened under a session name is reached by it, and no second agent carries it while it lives",
            cites: &["94", "98", "43", "107", "132"],
            run: an_agent_under_a_session_name::<H>,
        },
        Scenario {
            name: "terminal: a part's agents are given by their names, each with its state and a session name a send reaches it by, a name no agent carries is not given, and a part that does not run holds none and is not started",
            cites: &["132", "95", "96", "98"],
            run: an_agent_by_its_name::<H>,
        },
        Scenario {
            name: "terminal: attach gives a window's command for each part, shows the app, or refuses a view it does not offer",
            cites: &["187"],
            run: attach_by_view::<H>,
        },
        Scenario {
            name: "terminal: setup gives what its configuration lacks for flume's plugin, each in words, and none for a terminal with no plugin of flume's",
            cites: &["122", "72"],
            run: setup_gives_what_is_lacking::<H>,
        },
        Scenario {
            name: "terminal: runs reads whether each part runs, alone or several in one reading, by the name the terminal knows it by, or that it holds no part or could not read it, and starts nothing",
            cites: &["201"],
            run: runs_reads_each_part::<H>,
        },
        Scenario {
            name: "terminal: sends says whether it hands a running agent words, and one that does not refuses every send with nothing typed",
            cites: &["95", "53", "198"],
            run: sends_says_whether_it_hands_words::<H>,
        },
    ]
}

/// The seam-contracts spec's scenarios of whether a terminal hands a
/// running agent words: it says so as the harness has it, the same each
/// time asked, and one that does not refuses a send to an agent it
/// opened, the agent taking nothing (53, 95, C.5).
fn sends_says_whether_it_hands_words<H: TerminalHarness>(harness: &H) -> Outcome {
    let sends = harness.seam().sends();
    ensure_eq!(sends, harness.sends(), "whether it sends");
    ensure_eq!(harness.seam().sends(), sends, "asked again");
    if sends {
        return Ok(());
    }
    let who = opened(harness, 1)?;
    ensure!(
        matches!(harness.seam().send(&who, "notes.md"), Sent::Refused { .. }),
        "a terminal that hands no words sent"
    );
    ensure!(
        harness.submitted(&who).is_empty(),
        "the agent took {:?}",
        harness.submitted(&who)
    );
    Ok(())
}

/// The seam-contracts spec's runs over the fake: each part answered as
/// the harness has it, in words where it holds no part or could not
/// read it; the parts asked together in one call answered each as alone,
/// in the order asked; nothing logged but each asking, one for each call,
/// so nothing opened; and each asked again answered the same, so nothing
/// started (201).
fn runs_reads_each_part<H: TerminalHarness>(harness: &H) -> Outcome {
    let parts = harness.runs_parts();
    if parts.is_empty() {
        return Ok(());
    }
    let of =
        |part: &Option<String>| format!("runs of {}", part.as_deref().unwrap_or("its own part"));
    for (part, due) in &parts {
        let answer = harness.seam().runs(&[part.as_deref()]);
        ensure_eq!(answer, vec![due.clone()], of(part));
        let words = match &answer[0] {
            SessionRuns::Runs { name } | SessionRuns::NotRunning { name } => name,
            SessionRuns::NoPart { reason } | SessionRuns::Unread { reason } => reason,
        };
        ensure!(
            !words.trim().is_empty(),
            "{} gave {answer:?}, with no words",
            of(part)
        );
    }
    let asked: Vec<Option<&str>> = parts.iter().map(|(part, _)| part.as_deref()).collect();
    let due: Vec<SessionRuns> = parts.iter().map(|(_, due)| due.clone()).collect();
    ensure_eq!(
        harness.seam().runs(&asked),
        due,
        "runs of every part in one call"
    );
    if let Some(logged) = harness.logged() {
        let named: Vec<&str> = asked.iter().map(|part| part.unwrap_or("own")).collect();
        let mut calls: Vec<String> = named.iter().map(|part| format!("runs {part}")).collect();
        calls.push(format!("runs {}", named.join(" ")));
        ensure_eq!(logged, calls, "what the terminal logged");
    }
    for (part, due) in &parts {
        ensure_eq!(
            harness.seam().runs(&[part.as_deref()]),
            vec![due.clone()],
            format!("{} asked again", of(part))
        );
    }
    Ok(())
}

/// The seam-contracts spec's "An agent by its name": an agent the harness
/// started under `w-planner` in the part `w` is given there by that name,
/// once, with a state that is not stopped and a session name whose state
/// is read and to which a send goes, or is refused by a terminal that
/// cannot send; no agent is given by a name none carries; and the part
/// `idle`, which does not run, holds none and runs no more after it was
/// asked. A terminal that holds no agent a name finds answers each part
/// not read, in words (95, 96, 98, 132, C.5; core.13).
fn an_agent_by_its_name<H: TerminalHarness>(harness: &H) -> Outcome {
    let (part, name, idle, nobody) = ("w", "w-planner", "idle", "w-nobody");
    if !harness.named_agent(part, name) {
        for asked in [Some(part), Some(idle), None] {
            match harness.seam().agents_in(asked) {
                AgentsIn::Unread { reason } => ensure!(
                    !reason.trim().is_empty(),
                    "the agents of {asked:?} not read, with no words"
                ),
                other => {
                    return Err(format!(
                        "the agents of {asked:?}, in a terminal that holds no agent a name finds, gave {other:?}"
                    ))
                }
            }
        }
        return Ok(());
    }
    let agents = match harness.seam().agents_in(Some(part)) {
        AgentsIn::Read { agents } => agents,
        other => return Err(format!("the agents of {part} gave {other:?}")),
    };
    let found: Vec<&NamedAgent> = agents.iter().filter(|agent| agent.name == name).collect();
    let [agent] = found[..] else {
        return Err(format!(
            "{name} given {} times among {agents:?}",
            found.len()
        ));
    };
    ensure!(
        agent.state != AgentState::Stopped,
        "{name} given stopped: {agent:?}"
    );
    ensure!(
        agents
            .iter()
            .all(|agent| agent.name != nobody && !agent.at.trim().is_empty()),
        "the agents of {part}: {agents:?}"
    );
    let state = harness.seam().session_state(&agent.at);
    ensure!(
        state.is_some_and(|state| state != AgentState::Stopped),
        "{name}'s session {} read {state:?}",
        agent.at
    );
    let sent = harness.seam().send_to(&agent.at, "a notice");
    if harness.sends() {
        ensure_eq!(sent, ToSession::Sent, format!("the send to {name}"));
        ensure_eq!(
            harness.received(&agent.at).last().cloned(),
            Some("a notice".to_string()),
            format!("what {name} took")
        );
    } else {
        ensure!(
            matches!(sent, ToSession::Refused { .. }),
            "a terminal that cannot send sent {sent:?}"
        );
    }
    ensure_eq!(
        harness.seam().agents_in(Some(idle)),
        AgentsIn::Read { agents: vec![] },
        format!("the agents of {idle}, which does not run")
    );
    let runs = harness.seam().runs(&[Some(idle)]);
    ensure!(
        !matches!(runs[..], [SessionRuns::Runs { .. }]),
        "{idle} runs once its agents were asked: {runs:?}"
    );
    Ok(())
}

fn attach_by_view<H: TerminalHarness>(harness: &H) -> Outcome {
    let offered = harness.views();
    let parts = harness.parts();
    ensure!(!parts.is_empty(), "no part to attach");
    let mut commands = vec![];
    for part in &parts {
        for view in [View::Terminal, View::App] {
            let answer = harness.seam().attach(part.as_deref(), view);
            let of = format!("attach {part:?} for {}", view.name());
            match (offered.contains(&view), view, answer) {
                (true, View::Terminal, Attached::Command { command }) => {
                    ensure!(
                        !command.program.trim().is_empty(),
                        "{of} gave a command with no program"
                    );
                    commands.push(command);
                }
                (true, View::App, Attached::Shown) => {}
                (false, _, Attached::Refused { reason }) => {
                    ensure!(!reason.trim().is_empty(), "{of} refused with no words");
                }
                (offers, _, answer) => {
                    return Err(format!(
                        "{of}, a view the terminal {}, gave {answer:?}",
                        if offers { "offers" } else { "does not offer" }
                    ));
                }
            }
        }
    }
    for (at, command) in commands.iter().enumerate() {
        ensure!(
            !commands[..at].contains(command),
            "two parts were given the one command {command:?}"
        );
    }
    let first = parts[0].as_deref();
    for view in [View::Terminal, View::App] {
        if harness.refuse_attach(view) {
            ensure!(
                matches!(harness.seam().attach(first, view), Attached::Refused { .. }),
                "attach for {} answered where it was to refuse",
                view.name()
            );
        }
    }
    Ok(())
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

fn a_session_at_work<H: TerminalHarness>(harness: &H) -> Outcome {
    let Some(session) = harness.working_recipient() else {
        return Ok(());
    };
    ensure_eq!(
        harness.seam().session_state(&session),
        Some(AgentState::Working),
        "the recipient's session"
    );
    let before = harness.received(&session);
    match harness.seam().send_to(&session, "a notice") {
        ToSession::Refused { reason } if !harness.sends() || reason.contains("works") => {}
        other => return Err(format!("a send to a session at work gave {other:?}")),
    }
    ensure_eq!(
        harness.received(&session),
        before,
        "what the session at work took"
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
        &OpenIn::Own,
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
        &OpenIn::Own,
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
        &OpenIn::Own,
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
        &OpenIn::Own,
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
        line: "code · run 1 · tasks 2/5 · working".into(),
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
                &OpenIn::Own,
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

fn setup_gives_what_is_lacking<H: TerminalHarness>(harness: &H) -> Outcome {
    let lacking = harness.setup();
    let setup = harness.seam().setup();
    ensure_eq!(setup, lacking, "what the configuration lacks");
    ensure!(
        setup.iter().all(|words| !words.trim().is_empty()),
        "a setup with no words: {setup:?}"
    );
    ensure_eq!(harness.seam().setup(), setup, "setup asked again");
    Ok(())
}

/// A terminal that counts every call made of it, passing each on.
struct Counted {
    terminal: Arc<dyn Terminal>,
    calls: Arc<AtomicUsize>,
}

impl Counted {
    fn call(&self) -> &dyn Terminal {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.terminal.as_ref()
    }
}

impl Terminal for Counted {
    fn open(
        &self,
        place: &PlaceRef,
        folder: &str,
        command: &AgentCommand,
        identity: &Identity,
        at: &OpenIn,
        session: Option<&str>,
    ) -> Opened {
        self.call()
            .open(place, folder, command, identity, at, session)
    }

    fn send(&self, agent: &Identity, text: &str) -> Sent {
        self.call().send(agent, text)
    }

    fn sends(&self) -> bool {
        self.call().sends()
    }

    fn state(&self, agent: &Identity) -> AgentState {
        self.call().state(agent)
    }

    fn send_to(&self, session: &str, text: &str) -> ToSession {
        self.call().send_to(session, text)
    }

    fn session_state(&self, session: &str) -> Option<AgentState> {
        self.call().session_state(session)
    }

    fn screen(&self, agent: &Identity) -> Option<String> {
        self.call().screen(agent)
    }

    fn answer_trust(&self, agent: &Identity) -> bool {
        self.call().answer_trust(agent)
    }

    fn close(&self, agent: &Identity) -> Closed {
        self.call().close(agent)
    }

    fn tidy(&self, place: &PlaceRef) -> TerminalTidy {
        self.call().tidy(place)
    }

    fn kept(&self) -> Vec<String> {
        self.call().kept()
    }

    fn show(&self, place: &PlaceRef, clues: &Clues) -> Shown {
        self.call().show(place, clues)
    }

    fn shown_at(&self, place: &PlaceRef, agent: Option<&Identity>) -> ShownAt {
        self.call().shown_at(place, agent)
    }

    fn setup(&self) -> Vec<String> {
        self.call().setup()
    }

    fn session(&self) -> Option<String> {
        self.call().session()
    }

    fn attach(&self, session: Option<&str>, view: View) -> Attached {
        self.call().attach(session, view)
    }

    fn runs(&self, sessions: &[Option<&str>]) -> Vec<SessionRuns> {
        self.call().runs(sessions)
    }
}

/// Whether the engine runs attended under `terminal`, with no watch held,
/// a watch of a minute just written, and that watch three minutes on,
/// over one host's store and clock; and how many calls were made of the
/// terminal to read them.
fn readings(terminal: Arc<dyn Terminal>) -> (Vec<bool>, usize) {
    let world = World::new();
    let calls = Arc::new(AtomicUsize::new(0));
    let counted: Arc<dyn Terminal> = Arc::new(Counted {
        terminal,
        calls: calls.clone(),
    });
    let engine = Engine::new(
        world.resolver.clone(),
        counted,
        world.store.clone(),
        world.history.clone(),
        world.host.clone(),
        world.defaults.clone(),
    );
    let mut read = vec![engine.attended()];
    let watched = Watched {
        began: world.host.now(),
        bound: 60,
    };
    world
        .store
        .write_host(&HostKey::Watch, None, Some(&HostDocument::Watch(watched)));
    read.push(engine.attended());
    world.host.advance(3 * 60);
    read.push(engine.attended());
    (read, calls.load(Ordering::SeqCst))
}

/// The seam-contracts spec's "The same reading under every terminal":
/// the engine asked whether it runs attended under the fake terminal,
/// then under this one, with the same host's watch held, answers the
/// same, read from the host's watch alone, and no terminal operation is
/// called to answer it (119, 122, 208).
fn the_same_reading_under_every_terminal<H: TerminalHarness>(harness: &H) -> Outcome {
    let world = World::new();
    let (fake, fake_calls) = readings(world.terminal.clone());
    let (this, these_calls) = readings(harness.shared());
    ensure_eq!(
        fake,
        vec![true, false, true],
        "attended under the fake terminal: no watch, a watch kept, a watch lapsed"
    );
    ensure_eq!(this, fake, "attended under this terminal");
    ensure_eq!(
        (fake_calls, these_calls),
        (0, 0),
        "calls made of the terminals to read it"
    );
    Ok(())
}
