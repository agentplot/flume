//! A place reported as one trace of its stages and waits (216 to 220,
//! S129; core.56): each place's held state names its trace and its own
//! span at its launch, each start's span as its claim is written and each
//! wait's as a settle first reads it; each span is handed on once, by the
//! write that records its end, with the times the held state keeps; a
//! record goes at each change; every span and record names its place and
//! none of its words; an agent is given its place's facts and its start's
//! context while flume sends, outside its start's record; and a settle of
//! every place hands on the host's reading, which names no place.

use super::repeating_stages::{branch, code_begun, GROUPS};
use super::settling::deliver_fix;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_fakes::Plan;
use flume_seams::telemetry::{self as keys, Value};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "telemetry: S129: a place codes an hour, stops short needing an answer, waits forty minutes on its caller, is retried, delivers, and waits fifty minutes on the merge queue before it lands, each start and each wait a span beneath the place's",
            cites: &["S129", "217", "218", "219"],
            run: s129_a_place_as_one_trace,
        },
        Scenario {
            name: "telemetry: S129's words stay on the host: no span or record holds its brief's, its stop-short's or its retry's words or a path, and every one names its repository, kind, version, name, base, stream, intent and mark",
            cites: &["S129", "218"],
            run: s129_words_stay_on_the_host,
        },
        Scenario {
            name: "telemetry: the place's span has no parent and runs from its launch to its tidy, every other span of the place beneath it",
            cites: &["217"],
            run: the_places_span,
        },
        Scenario {
            name: "telemetry: a place dropped while its stage's agent works ends its start's span stopped and its place's span at the drop",
            cites: &["217", "52"],
            run: a_dropped_place,
        },
        Scenario {
            name: "telemetry: two settles read one stage's delivery at once, and the start's span goes once, from the settle whose write landed",
            cites: &["217", "75"],
            run: two_settles_read_one_end,
        },
        Scenario {
            name: "telemetry: a stage that delivers while no command runs has its span sent by the next settle after a restart, from its start to that settle",
            cites: &["217", "77"],
            run: a_span_read_after_a_restart,
        },
        Scenario {
            name: "telemetry: a start read lost ends its span lost at the settle that read it, naming its conversation",
            cites: &["217", "96"],
            run: a_lost_start,
        },
        Scenario {
            name: "telemetry: each repetition of a repeating stage is a span of its own naming its repetition",
            cites: &["217", "140"],
            run: a_repeating_stages_repetitions,
        },
        Scenario {
            name: "telemetry: a place waiting on its caller under no hold's words past its bound stalls, its stall a record under its wait and its wait's span naming that it stalled",
            cites: &["217", "130"],
            run: a_wait_that_stalls,
        },
        Scenario {
            name: "telemetry: a stage waiting for its stream's room is a wait on the host from the settle that left it waiting to the one that started it",
            cites: &["217", "70"],
            run: a_wait_for_room,
        },
        Scenario {
            name: "telemetry: an agent read asking a person is a wait on a person from the settle that read it asking to the one that read it working",
            cites: &["217", "90"],
            run: a_person_while_the_agent_asks,
        },
        Scenario {
            name: "telemetry: a stage running an hour has its start's record under its span, the span not yet handed on",
            cites: &["217"],
            run: a_stage_running_now,
        },
        Scenario {
            name: "telemetry: a caller's hold on the submission goes as a record under the place's span naming the move held, and no record holds the hold's words",
            cites: &["217", "218", "47"],
            run: a_hold_put,
        },
        Scenario {
            name: "telemetry: a line of flume's log goes as a warning in flume's own words",
            cites: &["218", "139"],
            run: a_warning_in_flumes_words,
        },
        Scenario {
            name: "telemetry: while flume sends, an agent is given its account's resource pairs, any of a key flume names taken out, then its place's facts with its stage and start, and a TRACEPARENT naming its start's span",
            cites: &["219", "32"],
            run: the_agents_variables,
        },
        Scenario {
            name: "telemetry: with traces off an agent is given its place's facts and no TRACEPARENT; sending nothing, neither",
            cites: &["219", "216"],
            run: traces_off,
        },
        Scenario {
            name: "telemetry: a command the agent runs reads a TRACEPARENT whose trace is its place's and whose parent is its start's span",
            cites: &["219", "S129"],
            run: a_command_nests_under_its_start,
        },
        Scenario {
            name: "telemetry: a settle of every place hands on the host's places by kind, state and whom they wait on and each stream's agents and room, naming no place; a settle of one place none",
            cites: &["220", "121", "208"],
            run: the_hosts_places,
        },
        Scenario {
            name: "telemetry: a delivered start's span, measured an hour, names its repository, kind, stage, stream and delivered",
            cites: &["220", "217"],
            run: a_starts_time_by_its_kind_and_stage,
        },
        Scenario {
            name: "telemetry: status and the list hand on nothing; the next settle that ends a stage hands on its span",
            cites: &["216", "56", "200"],
            run: status_sends_nothing,
        },
        Scenario {
            name: "held state: a launch names, in the place's first document, its trace, its place's span and the launch's time, and every later write keeps them",
            cites: &["217", "73"],
            run: a_launch_names_the_trace,
        },
        Scenario {
            name: "held state: a document written before is given its trace at its next write, its place's span from that write and its running start's from its start",
            cites: &["217", "77"],
            run: a_document_written_before,
        },
        Scenario {
            name: "held state: a start's span is held before its agent's command is built, the command's TRACEPARENT naming it",
            cites: &["217", "219"],
            run: a_starts_span_before_its_command,
        },
        Scenario {
            name: "start integrity: OTEL_RESOURCE_ATTRIBUTES and TRACEPARENT as flume sets them are in no start's record, and an agent is taken whether flume sends or not",
            cites: &["161", "216", "219"],
            run: the_ack_taken_whatever_telemetry_gave,
        },
        Scenario {
            name: "seam fakes: over fakes whose telemetry sends nothing, a place runs to its landing, the fake keeps every signal handed it, and no open carries OTEL_RESOURCE_ATTRIBUTES or TRACEPARENT",
            cites: &["216", "219", "106"],
            run: a_fake_that_sends_nothing,
        },
        Scenario {
            name: "seam fakes: over fakes whose telemetry sends traces, an open's TRACEPARENT names the span later handed for that start",
            cites: &["217", "219", "106"],
            run: a_fake_that_sends_traces,
        },
    ]
}

/// A phrase no signal holds, which the place's words carry.
const PHRASE: &str = "heron-blue lantern";

/// A path the place's words carry, which no signal holds.
const PATH: &str = "/home/someone/notes";

/// An hour.
const HOUR: Seconds = 60 * 60;

/// Every span handed on, by name, in order: each with its parent, times
/// and attributes.
struct Span {
    span: String,
    parent: Option<String>,
    from: Seconds,
    to: Seconds,
    attributes: Attributes,
}

fn spans_named(world: &World, name: &str) -> Vec<Span> {
    world
        .telemetry
        .signals()
        .into_iter()
        .filter_map(|signal| match signal {
            Signal::Span {
                span,
                parent,
                name: found,
                from,
                to,
                attributes,
                ..
            } if found == name => Some(Span {
                span,
                parent,
                from,
                to,
                attributes,
            }),
            _ => None,
        })
        .collect()
}

/// The records handed on under `event`, each with its span and
/// attributes.
fn records(world: &World, event: &str) -> Vec<(String, Attributes)> {
    world
        .telemetry
        .signals()
        .into_iter()
        .filter_map(|signal| match signal {
            Signal::Record {
                span,
                event: found,
                attributes,
                ..
            } if found == event => Some((span, attributes)),
            _ => None,
        })
        .collect()
}

fn text(value: &str) -> Value {
    Value::Text(value.into())
}

/// The value `key` names in `attributes`.
fn at<'a>(attributes: &'a Attributes, key: &str) -> Option<&'a Value> {
    attributes.get(key)
}

/// The place's trace as its held state names it.
fn trace_of(world: &World, id: &PlaceId) -> Result<TraceHeld, String> {
    world
        .held(id)
        .ok_or("the place is gone")?
        .trace
        .ok_or_else(|| "the place names no trace".to_string())
}

/// The start of `stage` of the place, as held.
fn start_of(world: &World, id: &PlaceId, index: usize) -> Result<StartHeld, String> {
    world.held(id).ok_or("the place is gone")?.stages[index]
        .start
        .clone()
        .ok_or_else(|| "the stage never started".to_string())
}

/// The fix `name` launched with `fix` changed as given, its brief holding
/// [`PHRASE`] and [`PATH`], and its code stage begun.
fn fix_with(
    world: &World,
    name: &str,
    change: impl FnOnce(&mut Launch),
) -> Result<PlaceId, String> {
    let mut fix = launch(
        "fix",
        name,
        Some(&format!("mend the {PHRASE} under {PATH}")),
    );
    change(&mut fix);
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", name);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    Ok(id)
}

/// The fix `name` with its code stage begun.
fn fix_begun(world: &World, name: &str) -> Result<PlaceId, String> {
    fix_with(world, name, |_| {})
}

/// The start `start` of the place's code stage stops short needing an
/// answer, its words holding [`PHRASE`] and [`PATH`], and a settle reads it.
fn stops_short(world: &World, id: &PlaceId, start: u32) -> Outcome {
    let held = start_of(world, id, 0)?;
    ensure_eq!(held.n, start, "the start that stops short");
    let answer = world.engine.needs(
        &held.identity.to_string(),
        &held.secret.0,
        &format!("which way is the {PHRASE}? see {PATH}"),
        StopKind::NeedsAnswer,
    );
    ensure!(answer.refusal().is_none(), "the stop-short: {answer:?}");
    world.settle(id);
    Ok(())
}

/// S129 run over the fakes: a fix of the intent `accounts`, its mark
/// `FL-129`, whose agents are not kept open for an answer, so its retry
/// starts afresh; begun at the launch's time, stopped short an hour on,
/// retried forty minutes after, delivered, and landed fifty minutes after
/// its submission. Its id and the times of each step.
struct S129 {
    id: PlaceId,
    launched: Seconds,
    stopped: Seconds,
    retried: Seconds,
    submitted: Seconds,
    landed: Seconds,
}

fn s129(world: &World) -> Result<S129, String> {
    world.telemetry.set_sends(Sends::ALL);
    world.queue.plan_submits(vec![Plan::Hold]);
    let launched = world.host.now();
    let id = fix_with(world, "accounts", |fix| {
        fix.intent = Some("accounts".into());
        fix.mark = Some("FL-129".into());
        fix.every_stage = Settings {
            keep_open: Some("0".into()),
            ..Settings::default()
        };
    })?;
    world.host.advance(HOUR);
    let stopped = world.host.now();
    stops_short(world, &id, 1)?;
    world.settle(&id);
    world.host.advance(40 * 60);
    let retried = world.host.now();
    let answer = world.engine.retry(
        &id,
        Retry::Stage("code".into()),
        Some(format!("go by the {PHRASE}, as {PATH} says")),
        false,
    );
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    world.settle(&id);
    ensure_eq!(start_of(world, &id, 0)?.n, 2, "the retry's start");
    world.host.advance(HOUR);
    let submitted = world.host.now();
    deliver_fix(world, &id);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    world.host.advance(50 * 60);
    let landed = world.host.now();
    world.queue.decide(REPOSITORY, "fix/accounts", Plan::Land);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    Ok(S129 {
        id,
        launched,
        stopped,
        retried,
        submitted,
        landed,
    })
}

fn s129_a_place_as_one_trace(_: &()) -> Outcome {
    let world = World::new();
    let run = s129(&world)?;
    let place = spans_named(&world, "place");
    ensure_eq!(place.len(), 1, "the place's spans");
    let place = &place[0];
    ensure_eq!(place.parent, None, "the place's span's parent");
    ensure_eq!(
        (place.from, place.to),
        (run.launched, run.landed),
        "the place's span"
    );
    let code = spans_named(&world, "code");
    ensure_eq!(code.len(), 2, "code's spans");
    for span in &code {
        ensure_eq!(span.parent.as_ref(), Some(&place.span), "a start's parent");
    }
    ensure_eq!(
        at(&code[0].attributes, keys::START),
        Some(&Value::Number(1)),
        "start 1"
    );
    ensure_eq!(
        at(&code[0].attributes, keys::START_END),
        Some(&text("short")),
        "start 1's end"
    );
    ensure_eq!(
        at(&code[0].attributes, keys::START_NEEDS),
        Some(&text("answer")),
        "start 1 needs"
    );
    ensure_eq!(
        (code[0].from, code[0].to),
        (run.launched, run.stopped),
        "an hour coding"
    );
    ensure_eq!(
        at(&code[1].attributes, keys::START),
        Some(&Value::Number(2)),
        "start 2"
    );
    ensure_eq!(
        at(&code[1].attributes, keys::START_END),
        Some(&text("delivered")),
        "start 2's end"
    );
    ensure_eq!(code[1].from, run.retried, "start 2 from its retry");
    let caller = spans_named(&world, "wait on caller");
    ensure_eq!(caller.len(), 1, "the waits on the caller");
    ensure_eq!(
        (caller[0].from, caller[0].to),
        (run.stopped, run.retried),
        "forty minutes on the caller"
    );
    ensure_eq!(caller[0].to - caller[0].from, 40 * 60, "the caller's wait");
    ensure_eq!(
        at(&caller[0].attributes, keys::WAIT_MOVE),
        Some(&text("answer")),
        "the move"
    );
    let queue = spans_named(&world, "wait on merge-queue");
    ensure_eq!(queue.len(), 1, "the waits on the merge queue");
    ensure_eq!(
        (queue[0].from, queue[0].to),
        (run.submitted, run.landed),
        "fifty minutes on the merge queue"
    );
    ensure_eq!(
        at(&queue[0].attributes, keys::WAIT_OUTCOME),
        Some(&text("landed")),
        "how it ended"
    );
    for span in caller.iter().chain(&queue) {
        ensure_eq!(span.parent.as_ref(), Some(&place.span), "a wait's parent");
    }
    ensure!(!run.id.0.is_empty(), "the place's id");
    Ok(())
}

fn s129_words_stay_on_the_host(_: &()) -> Outcome {
    let world = World::new();
    s129(&world)?;
    let signals = world.telemetry.signals();
    ensure!(signals.len() > 10, "{} signals", signals.len());
    for signal in &signals {
        let shown = format!("{signal:?}");
        ensure!(
            !shown.contains("heron"),
            "a signal holds the place's words: {shown}"
        );
        ensure!(!shown.contains("/home"), "a signal holds a path: {shown}");
        ensure!(
            !shown.contains(REPOSITORY),
            "a signal holds the repository's path: {shown}"
        );
        let attributes = match signal {
            Signal::Span { attributes, .. } | Signal::Record { attributes, .. } => attributes,
            _ => continue,
        };
        for (key, value) in [
            (keys::REPOSITORY, text("repository")),
            (keys::KIND, text("fix")),
            (keys::KIND_VERSION, Value::Number(1)),
            (keys::PLACE, text("accounts")),
            (keys::BASE, text("main")),
            (keys::STREAM, text(DEFAULT_STREAM)),
            (keys::INTENT, text("accounts")),
            (keys::MARK, text("FL-129")),
        ] {
            ensure_eq!(at(attributes, key), Some(&value), "{key} of {shown}");
        }
    }
    Ok(())
}

fn the_places_span(_: &()) -> Outcome {
    let world = World::new();
    let launched = world.host.now();
    let id = fix_begun(&world, "one")?;
    let trace = trace_of(&world, &id)?;
    world.host.advance(HOUR);
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    let place = spans_named(&world, "place");
    ensure_eq!(place.len(), 1, "the place's spans");
    ensure_eq!(place[0].parent, None, "its parent");
    ensure_eq!(place[0].span, trace.span, "its span, as held");
    ensure_eq!(
        (place[0].from, place[0].to),
        (launched, launched + HOUR),
        "from its launch to its tidy"
    );
    for signal in world.telemetry.signals() {
        if let Signal::Span {
            trace: of, parent, ..
        } = &signal
        {
            ensure_eq!(of, &trace.trace, "a span's trace");
            ensure!(
                parent.is_none() || parent.as_ref() == Some(&trace.span),
                "a span beneath another: {signal:?}"
            );
        }
    }
    Ok(())
}

fn a_dropped_place(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    world.host.advance(600);
    let dropped = world.host.now();
    let answer = world.engine.drop_place(&id, true);
    ensure!(answer.refusal().is_none(), "the drop: {answer:?}");
    let code = spans_named(&world, "code");
    ensure_eq!(code.len(), 1, "code's spans");
    ensure_eq!(code[0].to, dropped, "the start's span ends at the drop");
    ensure_eq!(
        at(&code[0].attributes, keys::START_END),
        Some(&text("stopped")),
        "how it ended"
    );
    let place = spans_named(&world, "place");
    ensure_eq!(place.len(), 1, "the place's spans");
    ensure_eq!(place[0].to, dropped, "the place's span ends at the drop");
    Ok(())
}

fn two_settles_read_one_end(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_begun(&world, "one")?;
    deliver_fix(&world, &id);
    let (other, place) = (world.engine(), id.clone());
    world.store.before_next_write(move || {
        other.settle(Some(&place));
    });
    world.settle(&id);
    let code = spans_named(&world, "code");
    ensure_eq!(code.len(), 1, "code's spans");
    ensure_eq!(records(&world, "end").len(), 1, "code's end records");
    Ok(())
}

fn a_span_read_after_a_restart(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let started = world.host.now();
    let id = fix_begun(&world, "one")?;
    deliver_fix(&world, &id);
    world.host.advance(3 * HOUR);
    let read = world.host.now();
    world.engine().settle(None);
    let code = spans_named(&world, "code");
    ensure_eq!(code.len(), 1, "code's spans");
    ensure_eq!(
        (code[0].from, code[0].to),
        (started, read),
        "from its start to the settle that read it"
    );
    Ok(())
}

fn a_lost_start(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    let start = start_of(&world, &id, 0)?;
    world
        .agent
        .set_conversation(&start.identity, Some("conversation-1"));
    world.host.advance(600);
    let lost = world.host.now();
    world.terminal.kill(&start.identity);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "code"
    );
    let code = spans_named(&world, "code");
    ensure_eq!(code.len(), 1, "code's spans");
    ensure_eq!(code[0].to, lost, "ended at the settle that read it");
    ensure_eq!(
        at(&code[0].attributes, keys::START_END),
        Some(&text("lost")),
        "how"
    );
    ensure_eq!(
        at(&code[0].attributes, keys::SESSION_ID),
        Some(&text("conversation-1")),
        "its conversation"
    );
    Ok(())
}

fn a_repeating_stages_repetitions(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    for ticked in [2, 4, 5] {
        world.tick_groups(&branch("plan"), "plan", GROUPS, ticked);
        world.settle(&id);
    }
    let code = spans_named(&world, "code");
    ensure_eq!(code.len(), 3, "code's spans");
    let repetitions: Vec<Option<Value>> = code
        .iter()
        .map(|span| at(&span.attributes, keys::REPETITION).cloned())
        .collect();
    ensure_eq!(
        repetitions,
        vec![Some(text("1")), Some(text("2")), Some(text("3"))],
        "each names its repetition"
    );
    Ok(())
}

fn a_wait_that_stalls(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    stops_short(&world, &id, 1)?;
    world.host.advance(STALL_AFTER + 1);
    world.settle(&id);
    let stalls = records(&world, "stall");
    ensure_eq!(stalls.len(), 1, "the stall's records");
    let held = world.held(&id).ok_or("the place is gone")?;
    let wait = held
        .waits
        .iter()
        .find(|wait| wait.on == WaitParty::Caller)
        .ok_or("no wait on the caller")?;
    ensure_eq!(&stalls[0].0, &wait.span, "the stall under its wait");
    ensure!(wait.stalled, "the wait names no stall");
    let answer = world
        .engine
        .retry(&id, Retry::Stage("code".into()), None, true);
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    let caller = spans_named(&world, "wait on caller");
    ensure_eq!(caller.len(), 1, "the caller's waits");
    ensure_eq!(
        at(&caller[0].attributes, keys::WAIT_STALLED),
        Some(&Value::Flag(true)),
        "it stalled"
    );
    Ok(())
}

fn a_wait_for_room(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(1));
    world.queue.plan_submits(vec![Plan::Hold, Plan::Hold]);
    let one = fix_begun(&world, "one")?;
    launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    let two = World::id("fix", "two");
    world.settle(&two);
    ensure_eq!(
        world.stage_state(&two, "code"),
        Some(StageState::WaitingForRoom),
        "two's code"
    );
    let waited = world
        .held(&two)
        .and_then(|held| held.waits.first().map(|wait| wait.since))
        .ok_or("two holds no wait")?;
    world.host.advance(900);
    let started = world.host.now();
    deliver_fix(&world, &one);
    world.engine().settle(None);
    ensure!(
        world
            .stage_state(&two, "code")
            .is_some_and(StageState::running),
        "two's code did not start: {:?}",
        world.stage_state(&two, "code")
    );
    let host = spans_named(&world, "wait on host");
    ensure_eq!(host.len(), 1, "the waits on the host");
    ensure_eq!((host[0].from, host[0].to), (waited, started), "the wait");
    ensure_eq!(
        at(&host[0].attributes, keys::WAIT_MOVE),
        Some(&text("room")),
        "for room"
    );
    ensure_eq!(
        at(&host[0].attributes, keys::PLACE),
        Some(&text("two")),
        "two's"
    );
    Ok(())
}

fn a_person_while_the_agent_asks(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    let start = start_of(&world, &id, 0)?;
    world.host.advance(60);
    let asked = world.host.now();
    world.agent.set_asking(&start.identity, Reading::Yes);
    world.settle(&id);
    world.host.advance(600);
    let answered = world.host.now();
    world.agent.set_asking(&start.identity, Reading::No);
    world.settle(&id);
    let person = spans_named(&world, "wait on person");
    ensure_eq!(person.len(), 1, "the waits on a person");
    ensure_eq!(
        (person[0].from, person[0].to),
        (asked, answered),
        "the wait"
    );
    ensure_eq!(
        at(&person[0].attributes, keys::WAIT_MOVE),
        Some(&text("asking")),
        "while it asks"
    );
    Ok(())
}

fn a_stage_running_now(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    world.host.advance(HOUR);
    world.settle(&id);
    let start = start_of(&world, &id, 0)?;
    let span = start.span.ok_or("the start names no span")?;
    let started = records(&world, "start");
    ensure_eq!(started.len(), 1, "the start's records");
    ensure_eq!(&started[0].0, &span, "under the start's span");
    ensure!(
        spans_named(&world, "code").is_empty(),
        "the start's span went while it runs"
    );
    Ok(())
}

fn a_hold_put(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    let trace = trace_of(&world, &id)?;
    let words = format!("until the {PHRASE} is signed off");
    let answer = world.engine.hold(&id, HoldOn::Submission, &words, None);
    ensure!(answer.refusal().is_none(), "the hold: {answer:?}");
    let held = records(&world, "hold");
    ensure_eq!(held.len(), 1, "the hold's records");
    ensure_eq!(&held[0].0, &trace.span, "under the place's span");
    ensure_eq!(
        at(&held[0].1, keys::WAIT_MOVE),
        Some(&text("submission")),
        "the move held"
    );
    for signal in world.telemetry.signals() {
        ensure!(
            !format!("{signal:?}").contains("heron"),
            "a signal holds the hold's words: {signal:?}"
        );
    }
    Ok(())
}

fn a_warning_in_flumes_words(_: &()) -> Outcome {
    let world = World::new();
    world.store.refuse_host_writes(true);
    world.engine().settle_every(60);
    let logged = world.host.logged.calls();
    ensure!(!logged.is_empty(), "nothing was logged");
    let warnings: Vec<String> = world
        .telemetry
        .signals()
        .into_iter()
        .filter_map(|signal| match signal {
            Signal::Warning { words, .. } => Some(words),
            _ => None,
        })
        .collect();
    ensure_eq!(warnings, logged, "each line as a warning");
    Ok(())
}

/// The command the terminal opened the place's code agent with.
fn opened(world: &World, id: &PlaceId) -> Result<(StartHeld, AgentCommand), String> {
    let start = start_of(world, id, 0)?;
    let (command, _) = world
        .terminal
        .opened(&start.identity)
        .ok_or("the agent was not opened")?;
    Ok((start, command))
}

fn the_agents_variables(_: &()) -> Outcome {
    let world = World::new();
    world.telemetry.set_sends(Sends::ALL);
    world.host.set_pool(
        "work",
        Some(Pool {
            strategy: Strategy::FillFirst,
            threshold: None,
            accounts: vec![Account {
                name: "work-1".into(),
                program: "agent".into(),
                env: [(
                    "OTEL_RESOURCE_ATTRIBUTES".to_string(),
                    "team=ops,flume.stage=x".to_string(),
                )]
                .into(),
                agents: None,
                usage: None,
            }],
        }),
    );
    world.host.set_host_pool(Some("work"));
    let id = fix_begun(&world, "one")?;
    let (start, command) = opened(&world, &id)?;
    let resource = command
        .env
        .get("OTEL_RESOURCE_ATTRIBUTES")
        .ok_or("no OTEL_RESOURCE_ATTRIBUTES")?;
    ensure!(
        resource.starts_with("team=ops,"),
        "the account's pairs first: {resource}"
    );
    ensure_eq!(
        resource.matches("flume.stage=").count(),
        1,
        "flume.stage once"
    );
    ensure!(
        resource.contains("flume.stage=code") && resource.contains("flume.start=1"),
        "the start's stage and number: {resource}"
    );
    ensure!(
        resource.contains("flume.place=one") && resource.contains("flume.kind=fix"),
        "the place's facts: {resource}"
    );
    let trace = trace_of(&world, &id)?;
    ensure_eq!(
        command.env.get("TRACEPARENT").cloned(),
        Some(format!(
            "00-{}-{}-01",
            trace.trace,
            start.span.clone().ok_or("no span")?
        )),
        "the start's TRACEPARENT"
    );
    Ok(())
}

fn traces_off(_: &()) -> Outcome {
    let world = World::new();
    world.telemetry.set_sends(Sends {
        traces: false,
        ..Sends::ALL
    });
    let id = fix_begun(&world, "one")?;
    let (_, command) = opened(&world, &id)?;
    ensure!(
        command.env.contains_key("OTEL_RESOURCE_ATTRIBUTES"),
        "no resource"
    );
    ensure!(!command.env.contains_key("TRACEPARENT"), "a TRACEPARENT");
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    let (_, command) = opened(&world, &id)?;
    ensure!(
        !command.env.contains_key("OTEL_RESOURCE_ATTRIBUTES")
            && !command.env.contains_key("TRACEPARENT"),
        "sending nothing, the agent is given {:?}",
        command.env.keys().collect::<Vec<_>>()
    );
    Ok(())
}

fn a_command_nests_under_its_start(_: &()) -> Outcome {
    let world = World::new();
    world.telemetry.set_sends(Sends::ALL);
    let id = fix_begun(&world, "one")?;
    let (start, command) = opened(&world, &id)?;
    let parent = command.env.get("TRACEPARENT").ok_or("no TRACEPARENT")?;
    let parts: Vec<&str> = parent.split('-').collect();
    ensure_eq!(parts.len(), 4, "a W3C traceparent: {parent}");
    let trace = trace_of(&world, &id)?;
    ensure_eq!(parts[1], trace.trace.as_str(), "the place's trace");
    ensure_eq!(
        Some(parts[2].to_string()),
        start.span,
        "the start's span as the parent"
    );
    Ok(())
}

fn the_hosts_places(_: &()) -> Outcome {
    let world = World::new();
    let one = fix_begun(&world, "one")?;
    let two = fix_begun(&world, "two")?;
    stops_short(&world, &two, 1)?;
    world.telemetry.clear();
    world.settle(&one);
    let readings = |world: &World| -> Vec<HostReading> {
        world
            .telemetry
            .signals()
            .into_iter()
            .filter_map(|signal| match signal {
                Signal::Host(reading) => Some(reading),
                _ => None,
            })
            .collect()
    };
    ensure!(
        readings(&world).is_empty(),
        "a settle of one place read the host"
    );
    world.engine().settle(None);
    let read = readings(&world);
    ensure_eq!(read.len(), 1, "the host's readings");
    ensure_eq!(
        read[0].places,
        vec![
            PlacesStanding {
                kind: "fix".into(),
                state: "waiting on an answer".into(),
                on: Some("caller".into()),
                count: 1,
            },
            PlacesStanding {
                kind: "fix".into(),
                state: "working".into(),
                on: None,
                count: 1,
            },
        ],
        "the places standing"
    );
    ensure_eq!(
        read[0].streams,
        vec![StreamAgents {
            stream: DEFAULT_STREAM.into(),
            running: 2,
            room: None,
        }],
        "each stream's agents"
    );
    let shown = format!("{read:?}");
    ensure!(
        !shown.contains("fix--") && !shown.contains(REPOSITORY),
        "the reading names a place: {shown}"
    );
    Ok(())
}

fn a_starts_time_by_its_kind_and_stage(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_begun(&world, "one")?;
    world.host.advance(HOUR);
    deliver_fix(&world, &id);
    world.settle(&id);
    let code = spans_named(&world, "code");
    ensure_eq!(code.len(), 1, "code's spans");
    ensure_eq!(code[0].to - code[0].from, HOUR, "an hour");
    for (key, value) in [
        (keys::REPOSITORY, text("repository")),
        (keys::KIND, text("fix")),
        (keys::STAGE, text("code")),
        (keys::STREAM, text(DEFAULT_STREAM)),
        (keys::START_END, text("delivered")),
    ] {
        ensure_eq!(at(&code[0].attributes, key), Some(&value), "{key}");
    }
    Ok(())
}

fn status_sends_nothing(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_begun(&world, "one")?;
    deliver_fix(&world, &id);
    world.telemetry.clear();
    world.engine().status(Some(&id));
    world.engine().status(None);
    world.engine().list(false);
    world.engine().list(true);
    ensure!(
        world.telemetry.signals().is_empty(),
        "status or the list handed on {:?}",
        world.telemetry.signals()
    );
    world.settle(&id);
    ensure_eq!(
        spans_named(&world, "code").len(),
        1,
        "code's span at the settle"
    );
    Ok(())
}

fn a_launch_names_the_trace(_: &()) -> Outcome {
    let world = World::new();
    let launched = world.host.now();
    let id = fix_begun(&world, "one")?;
    let trace = trace_of(&world, &id)?;
    ensure_eq!(trace.trace.len(), 32, "the trace's digits");
    ensure_eq!(trace.span.len(), 16, "the place's span's digits");
    ensure!(
        trace
            .trace
            .chars()
            .chain(trace.span.chars())
            .all(|c| c.is_ascii_hexdigit()),
        "hex digits: {trace:?}"
    );
    ensure_eq!(trace.from, launched, "from the launch");
    world.host.advance(600);
    world.settle(&id);
    ensure_eq!(trace_of(&world, &id)?, trace, "a later write keeps it");
    Ok(())
}

fn a_document_written_before(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_begun(&world, "one")?;
    let mut before = world.held(&id).ok_or("the place")?;
    before.trace = None;
    before.waits.clear();
    let started = before.stages[0]
        .start
        .as_mut()
        .map(|start| {
            start.span = None;
            start.started_at
        })
        .ok_or("no start")?;
    world.store.put(&id, before);
    world.telemetry.clear();
    world.host.advance(HOUR);
    let read = world.host.now();
    deliver_fix(&world, &id);
    world.settle(&id);
    let trace = trace_of(&world, &id)?;
    ensure_eq!(trace.from, read, "the place's span from that write");
    let code = spans_named(&world, "code");
    ensure_eq!(code.len(), 1, "code's spans");
    ensure_eq!(
        (code[0].from, code[0].to),
        (started, read),
        "from its start"
    );
    Ok(())
}

fn a_starts_span_before_its_command(_: &()) -> Outcome {
    let world = World::new();
    world.telemetry.set_sends(Sends::ALL);
    let id = fix_begun(&world, "one")?;
    let (start, command) = opened(&world, &id)?;
    let span = start.span.ok_or("no span")?;
    ensure!(
        command
            .env
            .get("TRACEPARENT")
            .is_some_and(|parent| parent.contains(&span)),
        "the command was built before the span was held: {:?}",
        command.env.get("TRACEPARENT")
    );
    ensure!(
        world.agent.log.count("command") >= 1,
        "the agent seam was not asked for a command"
    );
    Ok(())
}

fn the_ack_taken_whatever_telemetry_gave(_: &()) -> Outcome {
    for sends in [Sends::ALL, Sends::default()] {
        let world = World::new();
        world.telemetry.set_sends(sends);
        let id = fix_begun(&world, "one")?;
        let start = start_of(&world, &id, 0)?;
        for name in ["OTEL_RESOURCE_ATTRIBUTES", "TRACEPARENT"] {
            ensure!(
                !start.start_record.env.contains_key(name),
                "{name} in the record, sending {sends:?}"
            );
        }
    }
    Ok(())
}

/// A fix run to its landing over the world's fakes, delivered and
/// landed.
fn landed(world: &World) -> Result<PlaceId, String> {
    let id = fix_begun(world, "one")?;
    deliver_fix(world, &id);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    Ok(id)
}

fn a_fake_that_sends_nothing(_: &()) -> Outcome {
    let world = World::new();
    let id = landed(&world)?;
    ensure!(!world.telemetry.spans().is_empty(), "the fake kept no span");
    let identity = World::identity("fix", "one", "code", 1);
    let (command, _) = world.terminal.opened(&identity).ok_or("not opened")?;
    for name in ["OTEL_RESOURCE_ATTRIBUTES", "TRACEPARENT"] {
        ensure!(
            !command.env.contains_key(name),
            "{name} given to {id}'s agent"
        );
    }
    Ok(())
}

fn a_fake_that_sends_traces(_: &()) -> Outcome {
    let world = World::new();
    world.telemetry.set_sends(Sends {
        traces: true,
        ..Sends::default()
    });
    landed(&world)?;
    let identity = World::identity("fix", "one", "code", 1);
    let (command, _) = world.terminal.opened(&identity).ok_or("not opened")?;
    let parent = command.env.get("TRACEPARENT").ok_or("no TRACEPARENT")?;
    let code = spans_named(&world, "code");
    ensure_eq!(code.len(), 1, "code's spans");
    ensure!(
        parent.contains(&code[0].span),
        "{parent} names no span handed for the start"
    );
    Ok(())
}
