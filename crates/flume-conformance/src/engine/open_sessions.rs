//! An agent kept open for its answer (42, 131, 198): a stage that ends
//! short needing an answer, found work from a stage before the kind's last
//! among it, keeps its agent live for that answer until its stop-short's
//! time and its `keep_open`, where its terminal hands a running agent
//! words; a settle reads the agent let go once its time has passed, its
//! caller stopped it, it is read stopped or brought back bare, or its
//! account is spent, records when and why, and ends it once quiet, the
//! stage waiting on its answer still (31, 70, 77, 96, 130, 134, 146).

use super::account_pools::{account, account_of, fill_first_at_homes};
use super::account_pools::{fix_in, on, pool, pool_at, reading_at, reads, stops_at_limit, window};
use super::agent_settings::kept_open;
use super::carried_conversations::{by_hand, carries_lost, carries_none, carries_to};
use super::carried_conversations::{resumed_prompt, retry_talk, start_of, talk, talking};
use super::repeating_stages::{branch, code_begun, GROUPS};
use super::settling::{change_at_verify, construct_begun, fix_begun};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Retry, WaitFor, Waits, ANSWER_RECORDED_WITHIN, KEEP_OPEN, POLL};
use flume_fakes::Plan;
use flume_seams::*;
use std::sync::{Arc, Mutex};

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "kept open: a code agent stopping short needing an answer at t, on a terminal that hands a running agent words, with no layer giving keep_open, stays live, its end recording it kept open until t and one hour",
            cites: &["42", "32", "95", "198"],
            run: an_hour_by_default,
        },
        Scenario {
            name: "kept open: the host's [agents.streams.work] keep_open of 10m keeps a code agent of a place in work open until ten minutes after its stop-short",
            cites: &["32", "143", "42", "198", "S104"],
            run: a_hosts_time_for_a_stream,
        },
        Scenario {
            name: "kept open: a launch giving code keep_open 0 keeps its asking agent open for no time: the settle its going quiet runs lets it go by its time and ends it, and code waits on its answer",
            cites: &["32", "42", "31", "130", "198"],
            run: kept_open_for_no_time,
        },
        Scenario {
            name: "kept open: under a terminal that hands no words to a running agent, a code agent's stop-short records no time kept open, and its agent is left live and unclosed as a stop-short leaves it there",
            cites: &["95", "53", "42", "198"],
            run: not_kept_open_under_a_terminal_that_hands_no_words,
        },
        Scenario {
            name: "kept open: a construct agent finding work, construct not the kind's last stage, is kept open for an answer until the hour a question's would be",
            cites: &["131", "42", "198"],
            run: found_work_from_a_stage_before_the_last,
        },
        Scenario {
            name: "run through: construct finding work starts no stage, and the place waits on an answer with its agent kept open for it, status giving that it found work and its words",
            cites: &["131", "42", "56", "198"],
            run: construct_says_it_found_work,
        },
        Scenario {
            name: "run through: verify, the kind's last stage, finding work is kept open for nothing, its agent ended once quiet and the fix round started with its findings",
            cites: &["131", "112", "31", "198"],
            run: found_work_from_the_last_stage_keeps_no_agent_open,
        },
        Scenario {
            name: "let go: an hour on the clock after a code agent stopped short, the agent quiet and another place waiting for room on a host at its bound, a settle lets it go by its time and ends it through the terminal, code waits on its answer since the stop-short, and the other place's code starts",
            cites: &["70", "130", "31", "42", "198", "S104"],
            run: the_time_passing,
        },
        Scenario {
            name: "let go: an asking agent working once its time has passed reads let go and is not ended until a settle reads it quiet",
            cites: &["31", "70", "198"],
            run: not_quiet_once_its_time_has_passed,
        },
        Scenario {
            name: "let go: an asking agent whose session ends within its hour is recorded let go, ended, at the next settle, closed by nothing of flume's, and its stage waits on its answer with no agent",
            cites: &["96", "134", "42", "130", "198", "S105"],
            run: the_agent_dying_within_its_time,
        },
        Scenario {
            name: "let go: another agent of the asking agent's account reaching its usage limit within the hour lets the asking agent go by its account spent at the next settle, which ends it once quiet",
            cites: &["146", "31", "32", "70", "198", "S105"],
            run: the_account_spent_within_its_time,
        },
        Scenario {
            name: "let go: an asking agent whose account passes its pool's threshold within the hour, and is not spent, stays kept open",
            cites: &["185", "145", "198"],
            run: past_its_pools_threshold_within_its_time,
        },
        Scenario {
            name: "let go: two settles reading an asking agent's time passed at once record it let go once and close its agent once",
            cites: &["74", "75", "31", "198"],
            run: two_settles_at_once,
        },
        Scenario {
            name: "let go: a research talk's agent kept open for its answer, brought back bare in its pane after a restart, reads let go agent_ended at the next settle, the bare agent left in its pane, sent nothing and not ended, and talk waits on its answer",
            cites: &["77", "45", "42", "130", "198"],
            run: brought_back_bare_in_a_conversations_pane,
        },
        Scenario {
            name: "let go: a code agent kept open for its answer, brought back bare and quiet in its pane after a restart, reads let go agent_ended at the next settle, and the bare agent is ended",
            cites: &["77", "S116", "31", "198"],
            run: brought_back_bare_in_another_stage,
        },
        Scenario {
            name: "answer: a retry of code with the words notes.md while its asking agent is kept open hands the same agent the question then the answer as one prompt, code begun again under its first start and started once, and the stop-short's end holds the answer by retry with when it was sent and received",
            cites: &["S4", "42", "49", "50", "53", "54", "198"],
            run: a_retrys_answer,
        },
        Scenario {
            name: "answer: a tell of notes.md to a place whose code waits on an answer, its agent kept open, hands the same agent the question then the answer, code begun again under its first start, and the stop-short's end holds the answer by tell",
            cites: &["42", "53", "55", "198"],
            run: a_tells_answer,
        },
        Scenario {
            name: "answer: the third repetition of a repeating code, kept open for its answer, takes a retry's question and answer in its session and carries on its third group under the same start, never its first",
            cites: &["142", "140", "42", "49", "198"],
            run: a_repeating_stages_answer,
        },
        Scenario {
            name: "answer: a code agent kept open that takes a tell's answer, commits and ends its session, a settle its quiet started reading it let go, agent_ended, before the tell reads the answer received, has the answer recorded with tell, and code, carried on under its first start, read delivered from its commit",
            cites: &["42", "96", "121", "198"],
            run: an_answer_taken_before_its_session_ended,
        },
        Scenario {
            name: "answer: one given while the asking agent asks a person in its pane, and one past the tell's bound while it works, are each refused with nothing typed and nothing recorded, and code still waits on its answer",
            cites: &["53", "54", "42", "198"],
            run: an_answer_the_agent_does_not_take,
        },
        Scenario {
            name: "answer: a retry with words once the agent's time kept open has passed, before any settle read it let go, lets the agent go and ends it once quiet, and a fresh agent starts whose prompt carries the question and the answer",
            cites: &["42", "49", "50", "31", "198"],
            run: an_answer_after_the_time_passed,
        },
        Scenario {
            name: "answer: a retry with no words while the asking agent is kept open and quiet ends it, and a fresh agent starts whose prompt carries the question alone",
            cites: &["49", "50", "31", "198"],
            run: a_retry_with_no_words_while_kept_open,
        },
        Scenario {
            name: "answer: a tell to a place whose code waits on an answer once its agent was let go is refused with nothing typed, naming the retry that answers it",
            cites: &["42", "53", "198", "S104"],
            run: a_tell_once_the_agent_is_let_go,
        },
        Scenario {
            name: "answer: a second stop-short from code's asking agent kept open while code still reads ended short, no act having handed it an answer, waits ten seconds for one to be recorded and is refused, code still waiting on its first answer",
            cites: &["41", "42", "53", "198"],
            run: a_second_stop_short_with_no_answer_handed,
        },
        Scenario {
            name: "let go: a retry of code with notes.md once its agent was let go by its time and ended starts code's second start, a fresh agent whose prompt carries the question, then the answer",
            cites: &["42", "49", "198", "S104"],
            run: a_late_answer,
        },
        Scenario {
            name: "let go: a retry with notes.md of a talk whose agent kept open was brought back bare in its pane waits while the bare agent runs its conversation, naming its process, and once it ends starts talk resuming its session on the account its lost start ran under, its prompt flume's words then the question and the answer",
            cites: &["49", "179", "207", "77", "198"],
            run: the_answer_carries_the_conversation,
        },
        Scenario {
            name: "let go: a talk's agent kept open and stopped by its caller reads let go, stopped, at the next settle, and a retry with notes.md starts a fresh agent carrying no conversation, its prompt the question and the answer",
            cites: &["49", "51", "42", "198"],
            run: stopped_by_its_caller,
        },
        Scenario {
            name: "carried on: a code agent answered in its session stopping short again with other words records a second end of code's first start under its own key, delivered once to each recipient, the first staying delivered, and the agent is kept open for keep_open from the second stop-short",
            cites: &["43", "74", "42", "198"],
            run: a_second_stop_short_in_one_session,
        },
        Scenario {
            name: "carried on: a code stage begun two hours before its agent received an answer in its session, its agent then working 20 minutes with nothing delivered under a 30-minute limit, is not read working past its limit, and is read so once it has worked 30 minutes since the answer",
            cites: &["44", "118", "42", "198"],
            run: limits_after_an_answer,
        },
        Scenario {
            name: "carried on: a code agent answered notes.md in its session then stopping at its account's usage limit, its program carrying no conversation, is followed by code's fresh start on the pool's next account carrying flume's words, then the question and the answer",
            cites: &["42", "146", "179", "198"],
            run: spent_after_an_answer,
        },
        Scenario {
            name: "carried on: code's second start, its words a question and its answer, beginning without the settings it recorded is followed by a fresh start carrying flume's words, then the question and the answer",
            cites: &["42", "161", "35", "198"],
            run: unsettled_after_a_retry,
        },
        Scenario {
            name: "end delivery: a code agent stopping short needing an answer at t, on a terminal that hands a running agent words, has its end delivered to ops giving its agent kept open until t and one hour, in the JSON form and in the notice's words",
            cites: &["43", "42", "198"],
            run: the_notice_of_a_question,
        },
        Scenario {
            name: "end delivery: a caller's wait on code, having delivered its stop-short, waits again once it answered in code's session: the wait does not answer the stop-short as a read, and answers code's next end once it is read",
            cites: &["43", "42", "198"],
            run: waiting_again_after_an_answer,
        },
    ]
}

/// The question every asking agent here stops short with.
pub(super) const QUESTION: &str = "Which file holds the log?";

/// The question and `answer` as a retry's prompt and an answer handed to
/// a session give them (42, 49).
fn asked(answer: &str) -> String {
    format!("The question: {QUESTION}\n\nThe answer: {answer}")
}

/// The act was taken.
pub(super) fn taken(answer: &flume_core::Answer) -> Outcome {
    match answer.refusal() {
        Some(reason) => Err(format!("the act was refused: {reason}")),
        None => Ok(()),
    }
}

/// The ends the place records of its stage `stage`'s start `start` ended
/// short, in the order they were read.
fn short_ends(
    world: &World,
    id: &PlaceId,
    stage: &str,
    start: u32,
) -> Result<Vec<EndHeld>, String> {
    Ok(world
        .held(id)
        .ok_or("no held state")?
        .ends
        .into_iter()
        .filter(|end| {
            matches!(
                &end.of,
                EndOf::Stage { stage: named, start: n, state: StageState::EndedShort, .. }
                    if named == stage && *n == start
            )
        })
        .collect())
}

/// The first end the place records of its stage `stage`'s start `start`
/// ended short.
fn short_end(world: &World, id: &PlaceId, stage: &str, start: u32) -> Result<EndHeld, String> {
    short_ends(world, id, stage, start)?
        .into_iter()
        .next()
        .ok_or_else(|| format!("no end of {stage}'s start {start} ended short"))
}

/// The answer notes.md reached the agent `who` of the fix's code, start 1,
/// kept open for it, by `act` at `at`: the question then the answer its
/// last words, code begun again under that start and started once, no
/// agent opened or closed for it, the stop-short's end holding the answer,
/// and the place waiting on its caller no more (42, 48, 49, 53).
fn answered_in_session(
    world: &World,
    id: &PlaceId,
    who: &Identity,
    act: Act,
    at: Seconds,
) -> Outcome {
    ensure_eq!(
        world.terminal.sent_to(who).last().cloned(),
        Some(asked("notes.md")),
        "the last words code's agent was sent"
    );
    let held = world.held(id).ok_or("no held state")?;
    let stage = &held.stages[0];
    let start = stage.start.clone().ok_or("code has no start")?;
    ensure_eq!(
        (
            stage.state,
            stage.starts,
            start.n,
            start.live,
            start.answered,
            stage.end.clone()
        ),
        (StageState::Begun, 1, 1, true, 1, None),
        "code, its starts, its start, whether it is live, its answers and its end"
    );
    ensure_eq!(
        (world.terminal.log.count("open"), closed(world)),
        (1, vec![]),
        "the agents opened and closed"
    );
    ensure_eq!(
        short_end(world, id, "code", 1)?.answer,
        Some(AnswerHeld {
            words: "notes.md".into(),
            act,
            at,
            session: Some(Handed {
                sent_at: at,
                received_at: at,
            }),
        }),
        "the answer the stop-short's end holds"
    );
    ensure!(
        world.status(id)?.waiting_on.is_none(),
        "the place still waits on its caller"
    );
    Ok(())
}

/// The fix `name`'s code start `start`.
pub(super) fn code(name: &str, start: u32) -> Identity {
    World::identity("fix", name, "code", start)
}

/// The secret of the live start of the place's stage at `index`.
fn secret(world: &World, id: &PlaceId, index: usize) -> Result<String, String> {
    Ok(world
        .held(id)
        .and_then(|held| held.stages[index].start.clone())
        .ok_or("no start")?
        .secret
        .0)
}

/// The agent `who` of the place's stage at `index` stops short with
/// `words` as `kind`, taken; the time it stopped short.
pub(super) fn stops_short(
    world: &World,
    id: &PlaceId,
    index: usize,
    who: &Identity,
    words: &str,
    kind: StopKind,
) -> Result<Seconds, String> {
    let at = world.host.now();
    let answer = world
        .engine()
        .needs(&who.to_string(), &secret(world, id, index)?, words, kind);
    ensure!(
        answer.refusal().is_none(),
        "the stop-short was refused: {answer:?}"
    );
    Ok(at)
}

/// The fix `name` launched, its code agent stopped short with
/// [`QUESTION`] and settled; the place and the time it stopped short.
pub(super) fn asking(world: &World, name: &str) -> Result<(PlaceId, Seconds), String> {
    let id = fix_begun(world, name)?;
    let at = stops_short(
        world,
        &id,
        0,
        &code(name, 1),
        QUESTION,
        StopKind::NeedsAnswer,
    )?;
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::EndedShort),
        "code once it stopped short"
    );
    Ok((id, at))
}

/// The fix `name` launched for `recipients`, none of them a session the
/// terminal holds, so a wait delivers to them, its code agent stopped
/// short with [`QUESTION`] and settled; the place and the time it stopped
/// short.
fn asking_for(
    world: &World,
    name: &str,
    recipients: &[&str],
) -> Result<(PlaceId, Seconds), String> {
    let mut fix = launch("fix", name, Some("x"));
    fix.recipients = recipients.iter().map(|named| named.to_string()).collect();
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", name);
    world.settle(&id);
    let at = stops_short(
        world,
        &id,
        0,
        &code(name, 1),
        QUESTION,
        StopKind::NeedsAnswer,
    )?;
    world.settle(&id);
    Ok((id, at))
}

/// The end a wait answers, and whether it delivered it; none where it
/// answers none.
fn waited(answer: &flume_core::Answer) -> Result<Option<(EndHeld, bool)>, String> {
    match answer {
        flume_core::Answer::Waited { end, delivered, .. } => {
            Ok(end.clone().map(|end| (end.end, *delivered)))
        }
        other => Err(format!("the wait answered {other:?}")),
    }
}

/// The words of a stage's end, as the end records them.
fn words_of(end: &EndHeld) -> Option<String> {
    match &end.of {
        EndOf::Stage { end, .. } => end.words.clone(),
        _ => None,
    }
}

/// flume's words for a fresh start after a spent account (146).
const SPENT: &str = "The stage's earlier agent stopped at its account's usage limit, and this agent starts the stage afresh. What it committed is on the place's branch and what it left is in the place: read them and carry the stage on from there.";

/// flume's words for a fresh start after one begun without its settings
/// (161).
const UNSETTLED: &str = "The stage's earlier agent began without the settings its start recorded and was ended, and this agent starts the stage afresh. Anything it committed is on the place's branch and anything it left is in the place: read them and carry the stage on from there.";

/// The end the place's stage `stage` holds.
fn end_of(world: &World, id: &PlaceId, stage: &str) -> Result<StageEnd, String> {
    world
        .held(id)
        .ok_or("no held state")?
        .stages
        .into_iter()
        .find(|held| held.name == stage)
        .and_then(|held| held.end)
        .ok_or_else(|| format!("{stage} holds no end"))
}

/// Whether the start under `who` is live, as held state gives it.
fn live(world: &World, id: &PlaceId, who: &Identity) -> Result<bool, String> {
    world
        .held(id)
        .ok_or("no held state")?
        .stages
        .into_iter()
        .filter_map(|stage| stage.start)
        .find(|start| &start.identity == who)
        .map(|start| start.live)
        .ok_or_else(|| format!("no start is {who}"))
}

/// The agents the fake terminal was asked to close.
fn closed(world: &World) -> Vec<String> {
    world.terminal.log.of("close")
}

/// The place waits on a retry of `stage` for its answer, since `since`.
fn waits_on_answer(world: &World, id: &PlaceId, stage: &str, since: Seconds) -> Outcome {
    let waiting = world
        .status(id)?
        .waiting_on
        .ok_or("the place waits on nobody")?;
    ensure_eq!(
        (
            waiting.act,
            waiting.stage.as_deref(),
            waiting.why,
            waiting.since
        ),
        (Act::Retry, Some(stage), Waits::Answer, Some(since)),
        "the move the place waits on"
    );
    Ok(())
}

fn an_hour_by_default(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = asking(&world, "one")?;
    ensure_eq!(
        (
            end_of(&world, &id, "code")?.open_until,
            live(&world, &id, &code("one", 1))?,
            closed(&world)
        ),
        (Some(at + KEEP_OPEN), true, vec![]),
        "the time code's end keeps its agent open until, whether its agent is live, and the agents closed"
    );
    ensure_eq!(KEEP_OPEN, 60 * 60, "flume's own time kept open");
    Ok(())
}

fn a_hosts_time_for_a_stream(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    world.host.set_stream_settings("work", kept_open("10m"));
    let id = fix_in(&world, Some("work"), "one")?;
    let at = stops_short(
        &world,
        &id,
        0,
        &code("one", 1),
        QUESTION,
        StopKind::NeedsAnswer,
    )?;
    world.settle(&id);
    ensure_eq!(
        (
            end_of(&world, &id, "code")?.open_until,
            live(&world, &id, &code("one", 1))?
        ),
        (Some(at + 10 * 60), true),
        "the time code's end keeps its agent open until, and whether its agent is live"
    );
    Ok(())
}

fn kept_open_for_no_time(_: &()) -> Outcome {
    let world = World::new();
    let mut fix = launch("fix", "one", Some("x"));
    fix.settings.insert("code".into(), kept_open("0"));
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    let who = code("one", 1);
    let at = stops_short(&world, &id, 0, &who, QUESTION, StopKind::NeedsAnswer)?;
    // The agent goes quiet once it stopped short, and its going quiet
    // settles the place (121).
    world.settle(&id);
    let end = end_of(&world, &id, "code")?;
    ensure_eq!(
        (end.open_until, end.closed),
        (
            Some(at),
            Some(SessionClosed {
                at,
                by: ClosedBy::Time,
                ended_at: Some(at),
                left_in_pane: false,
            })
        ),
        "the time code's agent was kept open until, and its letting go"
    );
    ensure_eq!(
        (closed(&world), live(&world, &id, &who)?),
        (vec![format!("close {who}")], false),
        "the agents closed, and whether code's start is live"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::EndedShort),
        "code"
    );
    waits_on_answer(&world, &id, "code", at)
}

fn not_kept_open_under_a_terminal_that_hands_no_words(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.cannot_send();
    let (id, _) = asking(&world, "one")?;
    world.host.advance(2 * KEEP_OPEN);
    world.settle(&id);
    let end = end_of(&world, &id, "code")?;
    ensure_eq!(
        (
            end.open_until,
            end.closed,
            live(&world, &id, &code("one", 1))?,
            closed(&world)
        ),
        (None, None, true, vec![]),
        "the time code's agent is kept open until, its letting go, whether its agent is live, and the agents closed, two hours on"
    );
    Ok(())
}

fn found_work_from_a_stage_before_the_last(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    let construct = World::identity("change", "plan", "construct", 1);
    let at = stops_short(
        &world,
        &id,
        0,
        &construct,
        "The brief's log already exists on main.",
        StopKind::FoundWork,
    )?;
    world.settle(&id);
    let (question, asked) = asking(&world, "one")?;
    ensure_eq!(
        (
            end_of(&world, &id, "construct")?
                .open_until
                .map(|until| until - at),
            end_of(&world, &question, "code")?
                .open_until
                .map(|until| until - asked),
        ),
        (Some(KEEP_OPEN), Some(KEEP_OPEN)),
        "how long construct's finding and code's question keep their agents open"
    );
    ensure_eq!(
        (live(&world, &id, &construct)?, closed(&world)),
        (true, vec![]),
        "whether construct's agent is live, and the agents closed"
    );
    Ok(())
}

fn construct_says_it_found_work(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    let construct = World::identity("change", "plan", "construct", 1);
    let findings = "The brief's log already exists on main.";
    let at = stops_short(&world, &id, 0, &construct, findings, StopKind::FoundWork)?;
    world.settle(&id);
    let status = world.status(&id)?;
    ensure_eq!(
        (status.stages[0].state, status.stages[1].state),
        (StageState::EndedShort, StageState::NotReached),
        "construct and code"
    );
    ensure_eq!(
        status.stages[0].end.clone().and_then(|end| end.needs),
        Some(StopKind::FoundWork),
        "the kind status gives"
    );
    waits_on_answer(&world, &id, "construct", at)?;
    let words = status.waiting_on.and_then(|waiting| waiting.words);
    ensure_eq!(words.as_deref(), Some(findings), "the words");
    ensure_eq!(
        (
            end_of(&world, &id, "construct")?.open_until,
            live(&world, &id, &construct)?,
            world.terminal.log.count("open")
        ),
        (Some(at + KEEP_OPEN), true, 1),
        "the time construct's agent is kept open until, whether it is live, and the agents opened"
    );
    Ok(())
}

fn found_work_from_the_last_stage_keeps_no_agent_open(_: &()) -> Outcome {
    let world = World::new();
    let id = change_at_verify(&world, "plan")?;
    let verify = World::identity("change", "plan", "verify", 1);
    let findings = "Task 1.1 is ticked and its test fails.";
    stops_short(&world, &id, 2, &verify, findings, StopKind::FoundWork)?;
    ensure_eq!(
        end_of(&world, &id, "verify")?.open_until,
        None,
        "the time verify's end keeps its agent open until"
    );
    world.settle(&id);
    ensure!(
        closed(&world).contains(&format!("close {verify}")),
        "verify's agent was not closed: {:?}",
        closed(&world)
    );
    let code = World::identity("change", "plan", "code", 2);
    let prompt = world.prompt(&id, &code).ok_or("code did not start")?;
    ensure!(
        prompt.contains(findings),
        "the findings are not in {prompt}"
    );
    Ok(())
}

fn the_time_passing(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(1));
    let (one, at) = asking(&world, "one")?;
    launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    let two = World::id("fix", "two");
    world.settle(&two);
    ensure_eq!(
        world.stage_state(&two, "code"),
        Some(StageState::WaitingForRoom),
        "two's code while one's agent is kept open"
    );
    world.host.advance(KEEP_OPEN);
    world.settle(&one);
    let who = code("one", 1);
    ensure_eq!(
        end_of(&world, &one, "code")?.closed,
        Some(SessionClosed {
            at: at + KEEP_OPEN,
            by: ClosedBy::Time,
            ended_at: Some(at + KEEP_OPEN),
            left_in_pane: false,
        }),
        "one's agent's letting go"
    );
    ensure_eq!(
        (closed(&world), live(&world, &one, &who)?),
        (vec![format!("close {who}")], false),
        "the agents closed, and whether one's code start is live"
    );
    ensure_eq!(
        world.stage_state(&one, "code"),
        Some(StageState::EndedShort),
        "one's code"
    );
    waits_on_answer(&world, &one, "code", at)?;
    ensure!(
        world
            .stage_state(&two, "code")
            .is_some_and(StageState::running),
        "two's code did not start in the room one's agent freed: {:?}",
        world.stage_state(&two, "code")
    );
    Ok(())
}

fn not_quiet_once_its_time_has_passed(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = asking(&world, "one")?;
    let who = code("one", 1);
    world.terminal.set_state(&who, AgentState::Working);
    world.host.advance(KEEP_OPEN + 60);
    world.settle(&id);
    ensure_eq!(
        (
            end_of(&world, &id, "code")?.closed,
            closed(&world),
            live(&world, &id, &who)?
        ),
        (
            Some(SessionClosed {
                at: at + KEEP_OPEN + 60,
                by: ClosedBy::Time,
                ended_at: None,
                left_in_pane: false,
            }),
            vec![],
            true
        ),
        "the agent's letting go, the agents closed, and whether code's start is live while it works"
    );
    world.host.advance(60);
    world.terminal.set_state(&who, AgentState::Waiting);
    world.settle(&id);
    let end = end_of(&world, &id, "code")?;
    ensure_eq!(
        (
            end.closed.map(|closed| (closed.at, closed.ended_at)),
            closed(&world),
            live(&world, &id, &who)?
        ),
        (
            Some((at + KEEP_OPEN + 60, Some(at + KEEP_OPEN + 120))),
            vec![format!("close {who}")],
            false
        ),
        "when it was let go and ended, the agents closed, and whether code's start is live once quiet"
    );
    Ok(())
}

fn the_agent_dying_within_its_time(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    let who = code("one", 1);
    world.agent.ends_after_stop_short(&who);
    let at = stops_short(&world, &id, 0, &who, QUESTION, StopKind::NeedsAnswer)?;
    world.agent.stopped_short(&who);
    world.host.advance(10 * 60);
    world.settle(&id);
    ensure_eq!(
        end_of(&world, &id, "code")?.closed,
        Some(SessionClosed {
            at: at + 10 * 60,
            by: ClosedBy::AgentEnded,
            ended_at: Some(at + 10 * 60),
            left_in_pane: false,
        }),
        "the agent's letting go"
    );
    ensure_eq!(
        (
            closed(&world),
            live(&world, &id, &who)?,
            world.stage_state(&id, "code")
        ),
        (vec![], false, Some(StageState::EndedShort)),
        "the agents closed, whether code's start is live, and code"
    );
    ensure!(
        world.status(&id)?.agent.is_none(),
        "status gives code an agent"
    );
    waits_on_answer(&world, &id, "code", at)
}

fn the_account_spent_within_its_time(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", None), account("work-2", None)],
    );
    world.host.set_host_pool(Some("work"));
    let one = fix_in(&world, None, "one")?;
    let who = code("one", 1);
    let at = stops_short(&world, &one, 0, &who, QUESTION, StopKind::NeedsAnswer)?;
    world.settle(&one);
    let two = fix_in(&world, None, "two")?;
    ensure_eq!(
        (
            account_of(&world, &one, "code"),
            account_of(&world, &two, "code")
        ),
        (on("work", "work-1"), on("work", "work-1")),
        "the accounts one's and two's code run on"
    );
    world.host.advance(20 * 60);
    stops_at_limit(&world, "two", 1, Some(at + 5 * 3600));
    world.settle(&two);
    ensure!(
        live(&world, &one, &who)?,
        "one's agent was ended by two's settle"
    );
    world.settle(&one);
    ensure_eq!(
        end_of(&world, &one, "code")?.closed,
        Some(SessionClosed {
            at: at + 20 * 60,
            by: ClosedBy::AccountSpent,
            ended_at: Some(at + 20 * 60),
            left_in_pane: false,
        }),
        "one's agent's letting go"
    );
    ensure!(
        closed(&world).contains(&format!("close {who}")),
        "one's agent was not closed: {:?}",
        closed(&world)
    );
    ensure_eq!(
        (live(&world, &one, &who)?, world.stage_state(&one, "code")),
        (false, Some(StageState::EndedShort)),
        "whether one's code start is live, and one's code"
    );
    waits_on_answer(&world, &one, "code", at)
}

fn past_its_pools_threshold_within_its_time(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    pool_at(
        &world,
        "work",
        Strategy::FillFirst,
        Some(50),
        vec![
            reading_at(account("work-1", None)),
            reading_at(account("work-2", None)),
        ],
    );
    let resets = Some(now + 5 * 3600);
    reads(&world, "work-1", now, vec![window("5-hour", 1_000, resets)]);
    let id = fix_in(&world, None, "one")?;
    let who = code("one", 1);
    stops_short(&world, &id, 0, &who, QUESTION, StopKind::NeedsAnswer)?;
    world.settle(&id);
    world.host.advance(10 * 60);
    reads(
        &world,
        "work-1",
        now + 10 * 60,
        vec![window("5-hour", 9_000, resets)],
    );
    world.settle(&id);
    ensure_eq!(
        (
            end_of(&world, &id, "code")?.closed,
            live(&world, &id, &who)?,
            closed(&world)
        ),
        (None, true, vec![]),
        "the agent's letting go, whether code's start is live, and the agents closed, its account at 90% under a threshold of 50"
    );
    Ok(())
}

fn two_settles_at_once(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = asking(&world, "one")?;
    world.host.advance(KEEP_OPEN);
    let other = world.engine();
    let settled = id.clone();
    world.store.before_next_write(move || {
        other.settle(Some(&settled));
    });
    world.settle(&id);
    let who = code("one", 1);
    ensure_eq!(
        (
            end_of(&world, &id, "code")?.closed,
            closed(&world),
            live(&world, &id, &who)?
        ),
        (
            Some(SessionClosed {
                at: at + KEEP_OPEN,
                by: ClosedBy::Time,
                ended_at: Some(at + KEEP_OPEN),
                left_in_pane: false,
            }),
            vec![format!("close {who}")],
            false
        ),
        "the agent's letting go, the agents closed, and whether code's start is live"
    );
    Ok(())
}

fn brought_back_bare_in_a_conversations_pane(_: &()) -> Outcome {
    let world = World::new();
    let id = talking(&world)?;
    let at = stops_short(&world, &id, 0, &talk(1), QUESTION, StopKind::NeedsAnswer)?;
    world.settle(&id);
    world.host.advance(10 * 60);
    world.host.restart();
    world.terminal.forget_all();
    world.agent.bring_back_bare(&talk(1));
    let sent = world.terminal.sent_to(&talk(1)).len();
    world.engine().settle(None);
    world.settle(&id);
    ensure_eq!(
        end_of(&world, &id, "talk")?.closed,
        Some(SessionClosed {
            at: at + 10 * 60,
            by: ClosedBy::AgentEnded,
            ended_at: None,
            left_in_pane: true,
        }),
        "talk's agent's letting go"
    );
    ensure_eq!(
        (
            closed(&world),
            world.terminal.sent_to(&talk(1)).len(),
            live(&world, &id, &talk(1))?,
            world.stage_state(&id, "talk")
        ),
        (vec![], sent, true, Some(StageState::EndedShort)),
        "the agents closed, what talk's agent was sent, whether talk's start is live, and talk"
    );
    waits_on_answer(&world, &id, "talk", at)
}

fn brought_back_bare_in_another_stage(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = asking(&world, "one")?;
    let who = code("one", 1);
    world.host.advance(10 * 60);
    world.host.restart();
    world.terminal.forget_all();
    world.agent.bring_back_bare(&who);
    world.engine().settle(None);
    ensure_eq!(
        end_of(&world, &id, "code")?.closed,
        Some(SessionClosed {
            at: at + 10 * 60,
            by: ClosedBy::AgentEnded,
            ended_at: Some(at + 10 * 60),
            left_in_pane: false,
        }),
        "code's agent's letting go"
    );
    ensure_eq!(
        (
            closed(&world),
            live(&world, &id, &who)?,
            world.stage_state(&id, "code")
        ),
        (
            vec![format!("close {who}")],
            false,
            Some(StageState::EndedShort)
        ),
        "the agents closed, whether code's start is live, and code"
    );
    waits_on_answer(&world, &id, "code", at)
}

fn a_retrys_answer(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = asking(&world, "one")?;
    let at = world.host.now();
    taken(&world.engine.retry(
        &id,
        Retry::Stage("code".into()),
        Some("notes.md".into()),
        false,
    ))?;
    answered_in_session(&world, &id, &code("one", 1), Act::Retry, at)
}

fn a_tells_answer(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = asking(&world, "one")?;
    let at = world.host.now();
    taken(&world.engine.tell(&id, "notes.md", None))?;
    answered_in_session(&world, &id, &code("one", 1), Act::Tell, at)
}

/// A settle's reading of a stage's agent kept open: its close, and
/// whether its start is live.
type LetGo = (Option<SessionClosed>, bool);

fn an_answer_taken_before_its_session_ended(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let (id, _) = asking(&world, "one")?;
    let who = code("one", 1);
    world.agent.receives_at(&who, None);
    let read: Arc<Mutex<Option<LetGo>>> = Arc::default();
    let (git, agent, terminal, store) = (
        world.git.clone(),
        world.agent.clone(),
        world.terminal.clone(),
        world.store.clone(),
    );
    let (settler, place, taker, seen) = (world.engine(), id.clone(), who.clone(), read.clone());
    world.host.at_next_sleep(move || {
        // While the tell waits to read the answer received, the agent
        // takes it, commits, and its session ends; the settle its quiet
        // started reads it let go.
        agent.receives_at(&taker, Some(1));
        git.commit(REPOSITORY, "fix/one", &["fix.rs"]);
        terminal.kill(&taker);
        agent.set_ended(&taker, Reading::Yes);
        settler.settle(Some(&place));
        let held = store.held(&place);
        let stage = held.as_ref().map(|held| &held.stages[0]);
        *seen.lock().expect("the settle's reading") = stage.map(|stage| {
            (
                stage.end.as_ref().and_then(|end| end.closed),
                stage.start.as_ref().is_some_and(|start| start.live),
            )
        });
    });
    let at = world.host.now();
    taken(&world.engine.tell(&id, "notes.md", None))?;
    ensure_eq!(
        read.lock().map_err(|_| "the settle's reading")?.clone(),
        Some((
            Some(SessionClosed {
                at: at + POLL,
                by: ClosedBy::AgentEnded,
                ended_at: Some(at + POLL),
                left_in_pane: false,
            }),
            false
        )),
        "the settle's letting go of code's agent before the tell recorded its answer, and whether code's start was live"
    );
    ensure_eq!(
        short_end(&world, &id, "code", 1)?.answer,
        Some(AnswerHeld {
            words: "notes.md".into(),
            act: Act::Tell,
            at,
            session: Some(Handed {
                sent_at: at,
                received_at: at + POLL,
            }),
        }),
        "the answer the stop-short's end holds"
    );
    let held = world.held(&id).ok_or("no held state")?;
    let stage = &held.stages[0];
    ensure_eq!(
        (
            stage.state,
            stage.starts,
            stage.start.as_ref().map(|start| (start.n, start.live, start.answered)),
            world.terminal.log.count("open")
        ),
        (StageState::Delivered, 1, Some((1, false, 1)), 1),
        "code, its starts, its start's number, whether it is live and its answers, and the agents opened"
    );
    Ok(())
}

fn a_repeating_stages_answer(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    world.tick_groups(&branch("plan"), "plan", GROUPS, 4);
    world.settle(&id);
    let third = World::identity("change", "plan", "code", 3);
    let group = |world: &World| -> Result<(Identity, Option<String>), String> {
        let start = world.held(&id).ok_or("no held state")?.stages[1]
            .start
            .clone()
            .ok_or("code has no start")?;
        Ok((start.identity, start.group.and_then(|group| group.number)))
    };
    ensure_eq!(
        group(&world)?,
        (third.clone(), Some("3".to_string())),
        "code's third repetition and its group"
    );
    stops_short(&world, &id, 1, &third, QUESTION, StopKind::NeedsAnswer)?;
    world.settle(&id);
    taken(&world.engine.retry(
        &id,
        Retry::Stage("code".into()),
        Some("notes.md".into()),
        false,
    ))?;
    ensure_eq!(
        world.terminal.sent_to(&third).last().cloned(),
        Some(asked("notes.md")),
        "the last words the third repetition's agent was sent"
    );
    ensure_eq!(
        (
            world.stage_state(&id, "code"),
            group(&world)?,
            world.terminal.log.count("open")
        ),
        (Some(StageState::Begun), (third, Some("3".to_string())), 4),
        "code, its repetition and its group, and the agents opened, construct's and code's three repetitions'"
    );
    Ok(())
}

fn an_answer_the_agent_does_not_take(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = asking(&world, "one")?;
    let who = code("one", 1);
    let sent = world.terminal.sent_to(&who).len();
    world.agent.set_asking(&who, Reading::Yes);
    world.terminal.set_state(&who, AgentState::Asking);
    refused(
        &world
            .engine
            .retry(&id, Retry::Current, Some("notes.md".into()), false),
    )?;
    world.agent.set_asking(&who, Reading::No);
    world.terminal.set_state(&who, AgentState::Working);
    let told = world.host.now();
    refused(&world.engine.tell(&id, "notes.md", Some(60)))?;
    ensure!(
        world.host.now() - told >= 60,
        "the tell gave up before its bound"
    );
    let start = world.held(&id).ok_or("no held state")?.stages[0]
        .start
        .clone()
        .ok_or("code has no start")?;
    ensure_eq!(
        (
            world.terminal.sent_to(&who).len(),
            world.stage_state(&id, "code"),
            short_end(&world, &id, "code", 1)?.answer,
            start.answered,
            closed(&world)
        ),
        (sent, Some(StageState::EndedShort), None, 0, vec![]),
        "what code's agent was sent, code, the answer its end holds, its start's answers, and the agents closed"
    );
    waits_on_answer(&world, &id, "code", at)
}

fn an_answer_after_the_time_passed(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = asking(&world, "one")?;
    world.host.advance(KEEP_OPEN);
    taken(
        &world
            .engine
            .retry(&id, Retry::Current, Some("notes.md".into()), false),
    )?;
    let first = code("one", 1);
    ensure_eq!(
        closed(&world),
        vec![format!("close {first}")],
        "the agents closed"
    );
    let prompt = world
        .prompt(&id, &code("one", 2))
        .ok_or("code's second start did not open")?;
    ensure!(
        prompt.contains(&asked("notes.md")),
        "the question then the answer are not in {prompt}"
    );
    Ok(())
}

fn a_retry_with_no_words_while_kept_open(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = asking(&world, "one")?;
    taken(&world.engine.retry(&id, Retry::Current, None, false))?;
    let first = code("one", 1);
    ensure_eq!(
        closed(&world),
        vec![format!("close {first}")],
        "the agents closed"
    );
    let prompt = world
        .prompt(&id, &code("one", 2))
        .ok_or("code's second start did not open")?;
    ensure!(
        prompt.contains(&format!("The question: {QUESTION}")) && !prompt.contains("The answer:"),
        "the question alone is not in {prompt}"
    );
    Ok(())
}

fn a_tell_once_the_agent_is_let_go(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = asking(&world, "one")?;
    world.host.advance(KEEP_OPEN);
    world.settle(&id);
    let sendings = world.terminal.log.count("send");
    let reason = refused(&world.engine.tell(&id, "notes.md", None))?;
    ensure!(
        reason.contains(&format!("flume retry {id} --words")),
        "the refusal names not the retry: {reason}"
    );
    ensure_eq!(
        world.terminal.log.count("send"),
        sendings,
        "the sendings after the tell"
    );
    waits_on_answer(&world, &id, "code", at)
}

fn a_second_stop_short_with_no_answer_handed(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = asking(&world, "one")?;
    let before = world.host.now();
    let answer = world.engine().needs(
        &code("one", 1).to_string(),
        &secret(&world, &id, 0)?,
        "And its text?",
        StopKind::NeedsAnswer,
    );
    let reason = refused(&answer)?;
    ensure!(
        reason.contains("is not running"),
        "the refusal's words: {reason}"
    );
    ensure_eq!(
        world.host.now() - before,
        ANSWER_RECORDED_WITHIN,
        "how long the stop-short waited for an answer to be recorded"
    );
    ensure_eq!(
        end_of(&world, &id, "code")?.words,
        Some(QUESTION.to_string()),
        "code's end's words"
    );
    waits_on_answer(&world, &id, "code", at)
}

fn a_late_answer(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = asking(&world, "one")?;
    world.host.advance(KEEP_OPEN);
    world.settle(&id);
    taken(&world.engine.retry(
        &id,
        Retry::Stage("code".into()),
        Some("notes.md".into()),
        false,
    ))?;
    let second = code("one", 2);
    let prompt = world
        .prompt(&id, &second)
        .ok_or("code's second start did not open")?;
    ensure!(
        prompt.contains(&asked("notes.md")),
        "the question then the answer are not in {prompt}"
    );
    ensure_eq!(
        world.held(&id).map(|held| held.stages[0].starts),
        Some(2),
        "code's starts"
    );
    Ok(())
}

fn the_answer_carries_the_conversation(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-1");
    let id = talking(&world)?;
    stops_short(&world, &id, 0, &talk(1), QUESTION, StopKind::NeedsAnswer)?;
    world.settle(&id);
    world.host.advance(10 * 60);
    world.host.restart();
    world.terminal.forget_all();
    world.agent.bring_back_bare(&talk(1));
    world.engine().settle(None);
    ensure_eq!(
        end_of(&world, &id, "talk")?
            .closed
            .map(|closed| (closed.by, closed.left_in_pane)),
        Some((ClosedBy::AgentEnded, true)),
        "talk's agent's letting go"
    );
    world.agent.live_runs("s-1", &[by_hand()]);
    let status = retry_talk(&world, &id, Some("notes.md"), false)?;
    let waiting = status
        .waits_on_session
        .ok_or("the retry's answer says talk waits on nothing")?;
    ensure_eq!(
        (
            waiting.stage.as_str(),
            waiting.processes,
            world.stage_state(&id, "talk"),
            closed(&world)
        ),
        (
            "talk",
            vec![by_hand()],
            Some(StageState::WaitingForRoom),
            vec![]
        ),
        "what talk waits on, talk, and the agents closed after the retry"
    );
    world.agent.live_ended("s-1");
    world.terminal.kill(&talk(1));
    world.engine().settle(None);
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(start.n, 2, "talk's start once the bare agent ended");
    carries_lost(&world, &start, "work-1")?;
    ensure_eq!(
        world.prompt(&id, &talk(2)),
        Some(resumed_prompt(&world, &talk(2), Some(&asked("notes.md")))),
        "the carried start's prompt"
    );
    Ok(())
}

fn stopped_by_its_caller(_: &()) -> Outcome {
    let world = World::new();
    let id = talking(&world)?;
    let at = stops_short(&world, &id, 0, &talk(1), QUESTION, StopKind::NeedsAnswer)?;
    world.settle(&id);
    world.host.advance(10 * 60);
    taken(&world.engine.stop(&id, false))?;
    world.settle(&id);
    ensure_eq!(
        end_of(&world, &id, "talk")?.closed,
        Some(SessionClosed {
            at: at + 10 * 60,
            by: ClosedBy::Stopped,
            ended_at: Some(at + 10 * 60),
            left_in_pane: false,
        }),
        "talk's agent's letting go"
    );
    retry_talk(&world, &id, Some("notes.md"), false)?;
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(start.n, 2, "talk's start after the retry");
    carries_none(&world, &start)?;
    let prompt = world
        .prompt(&id, &talk(2))
        .ok_or("talk's second start did not open")?;
    ensure!(
        prompt.contains(&asked("notes.md")),
        "the question then the answer are not in {prompt}"
    );
    Ok(())
}

fn a_second_stop_short_in_one_session(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let (id, _) = asking_for(&world, "one", &["ops"])?;
    taken(
        &world
            .engine
            .retry(&id, Retry::Current, Some("notes.md".into()), false),
    )?;
    world.host.advance(5 * 60);
    let who = code("one", 1);
    let again = stops_short(&world, &id, 0, &who, "And its text?", StopKind::NeedsAnswer)?;
    world.settle(&id);
    world.settle(&id);
    let ends = short_ends(&world, &id, "code", 1)?;
    let read: Vec<(Option<String>, bool, Vec<String>)> = ends
        .iter()
        .map(|end| {
            (
                words_of(end),
                end.answer.is_some(),
                end.deliveries
                    .iter()
                    .map(|delivery| delivery.recipient.clone())
                    .collect(),
            )
        })
        .collect();
    ensure_eq!(
        read,
        vec![
            (Some(QUESTION.to_string()), true, vec!["ops".to_string()]),
            (Some("And its text?".to_string()), false, vec!["ops".to_string()]),
        ],
        "code's first start's ends ended short: their words, whether each holds an answer, and whom each was delivered to"
    );
    ensure!(
        ends[0].of.key() != ends[1].of.key(),
        "the two ends share the key {}",
        ends[0].of.key()
    );
    ensure_eq!(world.terminal.notices("ops").len(), 2, "the notices to ops");
    ensure_eq!(
        (
            end_of(&world, &id, "code")?.open_until,
            live(&world, &id, &who)?,
            world.terminal.log.count("open")
        ),
        (Some(again + KEEP_OPEN), true, 1),
        "the time code's agent is kept open until, whether it is live, and the agents opened"
    );
    Ok(())
}

fn limits_after_an_answer(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    let who = code("one", 1);
    world.terminal.set_state(&who, AgentState::Working);
    world.host.advance(2 * 3600);
    stops_short(&world, &id, 0, &who, QUESTION, StopKind::NeedsAnswer)?;
    world.terminal.set_state(&who, AgentState::Waiting);
    world.settle(&id);
    taken(
        &world
            .engine
            .retry(&id, Retry::Current, Some("notes.md".into()), false),
    )?;
    let past = |world: &World| -> Result<bool, String> {
        Ok(world.held(&id).ok_or("no held state")?.stages[0]
            .start
            .clone()
            .ok_or("code has no start")?
            .past_limit)
    };
    world.terminal.set_state(&who, AgentState::Working);
    world.host.advance(20 * 60);
    world.settle(&id);
    ensure!(
        !past(&world)?,
        "code read working past its limit 20 minutes after its answer"
    );
    world.host.advance(10 * 60);
    world.settle(&id);
    ensure!(
        past(&world)?,
        "code not read working past its limit 30 minutes after its answer"
    );
    Ok(())
}

fn spent_after_an_answer(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", None), account("work-2", None)],
    );
    world.host.set_host_pool(Some("work"));
    let id = fix_in(&world, None, "one")?;
    let who = code("one", 1);
    stops_short(&world, &id, 0, &who, QUESTION, StopKind::NeedsAnswer)?;
    world.settle(&id);
    taken(
        &world
            .engine
            .retry(&id, Retry::Current, Some("notes.md".into()), false),
    )?;
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 5 * 3600));
    world.settle(&id);
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-2"),
        "the account code's fresh start runs on"
    );
    let prompt = world
        .prompt(&id, &code("one", 2))
        .ok_or("code's fresh start did not open")?;
    ensure!(
        prompt.contains(&format!("{SPENT}\n\n{}", asked("notes.md"))),
        "flume's words, then the question and the answer, are not in {prompt}"
    );
    Ok(())
}

fn unsettled_after_a_retry(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![Account {
            name: "work-1".into(),
            program: "agent".into(),
            env: [("AGENT_HOME".to_string(), "/s/work-1".to_string())]
                .into_iter()
                .collect(),
            agents: None,
            usage: None,
        }],
    );
    world.host.set_host_pool(Some("work"));
    let id = fix_in(&world, None, "one")?;
    stops_short(
        &world,
        &id,
        0,
        &code("one", 1),
        QUESTION,
        StopKind::NeedsAnswer,
    )?;
    world.settle(&id);
    world.host.advance(KEEP_OPEN);
    world.settle(&id);
    world.agent.set_started_without(
        &code("one", 2),
        Unsettled {
            missing: vec!["AGENT_HOME".into()],
            ..Unsettled::default()
        },
    );
    taken(
        &world
            .engine
            .retry(&id, Retry::Current, Some("notes.md".into()), false),
    )?;
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world
            .held(&id)
            .and_then(|held| held.stages[0].start.clone())
            .map(|start| start.n),
        Some(3),
        "code's start after its second began without its settings"
    );
    let prompt = world
        .prompt(&id, &code("one", 3))
        .ok_or("code's third start did not open")?;
    ensure!(
        prompt.contains(&format!("{UNSETTLED}\n\n{}", asked("notes.md"))),
        "flume's words, then the question and the answer, are not in {prompt}"
    );
    Ok(())
}

fn the_notice_of_a_question(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let (id, at) = asking_for(&world, "one", &["ops"])?;
    let until = at + KEEP_OPEN;
    let ends: Vec<_> = world
        .status(&id)?
        .ends
        .into_iter()
        .filter(|end| words_of(&end.end).as_deref() == Some(QUESTION))
        .collect();
    let read: Vec<(Option<Seconds>, Vec<String>, Vec<String>)> = ends
        .iter()
        .map(|end| {
            (
                end.open_until,
                end.end
                    .deliveries
                    .iter()
                    .map(|delivery| delivery.recipient.clone())
                    .collect(),
                end.pending_for.clone(),
            )
        })
        .collect();
    ensure_eq!(
        read,
        vec![(Some(until), vec!["ops".to_string()], vec![])],
        "code's stop-short: until when its agent is kept open, whom it was delivered to, and whom it is pending for"
    );
    let json = serde_json::to_value(&ends[0]).map_err(|error| error.to_string())?;
    ensure_eq!(
        json["open_until"].clone(),
        serde_json::json!(until),
        "until when its agent is kept open, in the JSON form"
    );
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![format!(
            "flume: fix--one code needs an answer, its agent kept open for the answer until {until}; read: flume status fix--one"
        )],
        "the notices to ops"
    );
    Ok(())
}

fn waiting_again_after_an_answer(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = asking_for(&world, "one", &["ops"])?;
    let code_stage = WaitFor::Stage("code".into());
    let first = waited(&world.engine().wait(&id, &code_stage, "ops", Some(0)))?;
    ensure_eq!(
        first.map(|(end, delivered)| (words_of(&end), delivered)),
        Some((Some(QUESTION.to_string()), true)),
        "the first wait's end and whether it delivered it"
    );
    taken(
        &world
            .engine
            .retry(&id, Retry::Current, Some("notes.md".into()), false),
    )?;
    let again = world.engine().wait(&id, &code_stage, "ops", Some(60));
    ensure!(
        waited(&again)?.is_none(),
        "the wait after the answer answered {again:?}"
    );
    stops_short(
        &world,
        &id,
        0,
        &code("one", 1),
        "And its text?",
        StopKind::NeedsAnswer,
    )?;
    let next = waited(&world.engine().wait(&id, &code_stage, "ops", Some(0)))?;
    ensure_eq!(
        next.map(|(end, delivered)| (words_of(&end), delivered)),
        Some((Some("And its text?".to_string()), true)),
        "the wait's end once code stopped short again, and whether it delivered it"
    );
    Ok(())
}
