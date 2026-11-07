//! An agent kept open for its answer (42, 131, 198): a stage that ends
//! short needing an answer, found work from a stage before the kind's last
//! among it, keeps its agent live for that answer until its stop-short's
//! time and its `keep_open`, where its terminal hands a running agent
//! words; a settle reads the agent let go once its time has passed, its
//! caller stopped it, it is read stopped or brought back bare, or its
//! account is spent, records when and why, and ends it once quiet, the
//! stage waiting on its answer still (31, 70, 77, 96, 130, 134, 146).

use super::account_pools::{account, account_of};
use super::account_pools::{fix_in, on, pool, pool_at, reading_at, reads, stops_at_limit, window};
use super::agent_settings::kept_open;
use super::carried_conversations::{talk, talking};
use super::settling::{change_at_verify, construct_begun, fix_begun};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Waits, KEEP_OPEN};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "kept open: a code agent stopping short needing an answer at t, on a terminal that hands a running agent words, with no layer giving keep_open, stays live, its end recording it kept open until t and one hour",
            cites: &["42", "32", "95"],
            run: an_hour_by_default,
        },
        Scenario {
            name: "kept open: the host's [agents.streams.work] keep_open of 10m keeps a code agent of a place in work open until ten minutes after its stop-short",
            cites: &["32", "143", "42"],
            run: a_hosts_time_for_a_stream,
        },
        Scenario {
            name: "kept open: a launch giving code keep_open 0 keeps its asking agent open for no time: the settle its going quiet runs lets it go by its time and ends it, and code waits on its answer",
            cites: &["32", "42", "31", "130"],
            run: kept_open_for_no_time,
        },
        Scenario {
            name: "kept open: under a terminal that hands no words to a running agent, a code agent's stop-short records no time kept open, and its agent is left live and unclosed as a stop-short leaves it there",
            cites: &["95", "53", "42"],
            run: not_kept_open_under_a_terminal_that_hands_no_words,
        },
        Scenario {
            name: "kept open: a construct agent finding work, construct not the kind's last stage, is kept open for an answer until the hour a question's would be",
            cites: &["131", "42"],
            run: found_work_from_a_stage_before_the_last,
        },
        Scenario {
            name: "run through: construct finding work starts no stage, and the place waits on an answer with its agent kept open for it, status giving that it found work and its words",
            cites: &["131", "42", "56"],
            run: construct_says_it_found_work,
        },
        Scenario {
            name: "run through: verify, the kind's last stage, finding work is kept open for nothing, its agent ended once quiet and the fix round started with its findings",
            cites: &["131", "112", "31"],
            run: found_work_from_the_last_stage_keeps_no_agent_open,
        },
        Scenario {
            name: "let go: an hour on the clock after a code agent stopped short, the agent quiet and another place waiting for room on a host at its bound, a settle lets it go by its time and ends it through the terminal, code waits on its answer since the stop-short, and the other place's code starts",
            cites: &["70", "130", "31", "42"],
            run: the_time_passing,
        },
        Scenario {
            name: "let go: an asking agent working once its time has passed reads let go and is not ended until a settle reads it quiet",
            cites: &["31", "70"],
            run: not_quiet_once_its_time_has_passed,
        },
        Scenario {
            name: "let go: an asking agent whose session ends within its hour is recorded let go, ended, at the next settle, closed by nothing of flume's, and its stage waits on its answer with no agent",
            cites: &["96", "134", "42", "130"],
            run: the_agent_dying_within_its_time,
        },
        Scenario {
            name: "let go: another agent of the asking agent's account reaching its usage limit within the hour lets the asking agent go by its account spent at the next settle, which ends it once quiet",
            cites: &["146", "31", "32", "70"],
            run: the_account_spent_within_its_time,
        },
        Scenario {
            name: "let go: an asking agent whose account passes its pool's threshold within the hour, and is not spent, stays kept open",
            cites: &["185", "145"],
            run: past_its_pools_threshold_within_its_time,
        },
        Scenario {
            name: "let go: two settles reading an asking agent's time passed at once record it let go once and close its agent once",
            cites: &["74", "75", "31"],
            run: two_settles_at_once,
        },
        Scenario {
            name: "let go: a research talk's agent kept open for its answer, brought back bare in its pane after a restart, reads let go agent_ended at the next settle, the bare agent left in its pane, sent nothing and not ended, and talk waits on its answer",
            cites: &["77", "45", "42", "130"],
            run: brought_back_bare_in_a_conversations_pane,
        },
        Scenario {
            name: "let go: a code agent kept open for its answer, brought back bare and quiet in its pane after a restart, reads let go agent_ended at the next settle, and the bare agent is ended",
            cites: &["77", "S116", "31"],
            run: brought_back_bare_in_another_stage,
        },
    ]
}

/// The question every asking agent here stops short with.
const QUESTION: &str = "Which file holds the log?";

/// The fix `name`'s code start `start`.
fn code(name: &str, start: u32) -> Identity {
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
fn stops_short(
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
fn asking(world: &World, name: &str) -> Result<(PlaceId, Seconds), String> {
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
