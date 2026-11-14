//! A place's trace, read from the writes of its held state (217 to 220;
//! design 7, 8): one write's signals are what changed between the state
//! it read and the state it writes, each span named in the held state as
//! it opens and handed on once, from the write that records its end. The
//! signals name the place by its facts and none of its words or paths
//! (218; core.56).

use flume_seams::telemetry::{self as keys, Value};
use flume_seams::{
    Attributes, HeldState, HoldOn, HostReading, IntentKey, Over, Phase, PlacesStanding, Seconds,
    Signal, StageState, StartHeld, StopKind, StreamAgents, TraceHeld, Verdict, WaitHeld, WaitParty,
    Waits,
};

/// The resource attributes an agent of flume's is given: its place's
/// facts, its stage and its start, in this order (219; design 9).
const AGENTS_KEYS: [&str; 10] = [
    keys::REPOSITORY,
    keys::KIND,
    keys::KIND_VERSION,
    keys::PLACE,
    keys::BASE,
    keys::STREAM,
    keys::INTENT,
    keys::MARK,
    keys::STAGE,
    keys::START,
];

/// The variable an agent's telemetry reads its resource from, and the one
/// it reads its parent span from (219).
pub(crate) const RESOURCE_VARIABLE: &str = "OTEL_RESOURCE_ATTRIBUTES";
pub(crate) const PARENT_VARIABLE: &str = "TRACEPARENT";

impl crate::Engine {
    /// Gives the agent of the start under `identity`, whose command's
    /// environment is `env`, its place's facts in `OTEL_RESOURCE_ATTRIBUTES`
    /// after the pairs its environment, else the login environment, gives
    /// there, any pair of a key flume names taken out of those, and, while
    /// traces go, `TRACEPARENT` naming its start's span in its place's
    /// trace; nothing while flume sends nothing. Neither is in the start's
    /// record (161, 219; core.10, core.30).
    pub(crate) fn give_telemetry(
        &self,
        id: &flume_seams::PlaceId,
        identity: &flume_seams::Identity,
        env: &mut std::collections::BTreeMap<String, String>,
    ) {
        let sends = self.telemetry.sends();
        if !sends.any() {
            return;
        }
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        let Some(start) = held
            .stages
            .iter()
            .filter_map(|stage| stage.start.as_ref())
            .find(|start| start.identity == *identity)
        else {
            return;
        };
        let given = env
            .get(RESOURCE_VARIABLE)
            .cloned()
            .or_else(|| self.host.login(RESOURCE_VARIABLE));
        let facts = stage_attributes(&place_attributes(&held), &identity.stage, start.n);
        env.insert(RESOURCE_VARIABLE.into(), resource(given.as_deref(), &facts));
        match (&held.trace, &start.span) {
            (Some(trace), Some(span)) if sends.traces => {
                env.insert(
                    PARENT_VARIABLE.into(),
                    format!("00-{}-{span}-01", trace.trace),
                );
            }
            _ => {}
        }
    }
}

impl crate::Engine {
    /// Writes `line`, flume's own words for what fails nothing, to the
    /// host's log, and hands it on as a warning, which the binary scrubs
    /// of each path and each value shaped as a credential (139, 218;
    /// core.56).
    pub(crate) fn log(&self, line: &str) {
        self.host.log(line);
        self.telemetry.send(Signal::Warning {
            at: self.host.now(),
            words: line.to_string(),
        });
    }

    /// Hands on the host's reading at a settle of every place: the places
    /// standing, counted by kind, state in the list's words and whom they
    /// wait on, a tidied place's kept document none of them; and each
    /// stream's agents running and room, as status reads the room (220,
    /// 121, 208).
    pub(crate) fn read_host(&self) {
        let mut counted: std::collections::BTreeMap<(String, String, Option<String>), u64> =
            std::collections::BTreeMap::new();
        for read in self.store.read_every() {
            let held = &read.state;
            if matches!(held.phase, Phase::Tidied { .. }) {
                continue;
            }
            let (state, on) = self.state_of_place(held);
            let on = on.map(|on| party_of(on).to_string());
            *counted
                .entry((held.place.kind.name.clone(), state, on))
                .or_default() += 1;
        }
        let places = counted
            .into_iter()
            .map(|((kind, state, on), count)| PlacesStanding {
                kind,
                state,
                on,
                count,
            })
            .collect();
        let streams = self
            .queue()
            .streams
            .iter()
            .map(|(stream, room)| StreamAgents {
                stream: stream.clone(),
                running: room.running,
                room: room.room(),
            })
            .collect();
        self.telemetry.send(Signal::Host(HostReading {
            at: self.host.now(),
            places,
            streams,
        }));
    }
}

/// Whom a place waits on as a signal names it (217, 220).
fn party_of(on: crate::WaitsOn) -> &'static str {
    match on {
        crate::WaitsOn::Caller => WaitParty::Caller.word(),
        crate::WaitsOn::Host => WaitParty::Host.word(),
        crate::WaitsOn::MergeQueue => WaitParty::MergeQueue.word(),
        crate::WaitsOn::Session => "session",
        crate::WaitsOn::Places => "places",
    }
}

/// `OTEL_RESOURCE_ATTRIBUTES` as an agent of flume's is given it: the
/// pairs `given` holds, each whose key flume names taken out, then the
/// place's `facts` in [`AGENTS_KEYS`]' order, each value percent-encoded
/// as OpenTelemetry's resource reads it (219; design 9).
fn resource(given: Option<&str>, facts: &Attributes) -> String {
    let mut pairs: Vec<String> = given
        .unwrap_or_default()
        .split(',')
        .filter(|pair| !pair.trim().is_empty())
        .filter(|pair| {
            let key = pair.split('=').next().unwrap_or_default().trim();
            !AGENTS_KEYS.contains(&key)
        })
        .map(str::to_string)
        .collect();
    for key in AGENTS_KEYS {
        if let Some(value) = facts.get(key) {
            pairs.push(format!("{key}={}", encode(&value.to_string())));
        }
    }
    pairs.join(",")
}

/// A resource attribute's value, percent-encoded where the variable's
/// own syntax would read it (W3C baggage octets), as sluice encodes its
/// own (219).
fn encode(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        match byte {
            b',' | b'=' | b';' | b'%' | b'"' | b'\\' | 0..=0x20 | 0x7f.. => {
                encoded.push_str(&format!("%{byte:02X}"));
            }
            _ => encoded.push(byte as char),
        }
    }
    encoded
}

/// The hex digits of a trace's id and of a span's (217).
pub(crate) const TRACE_DIGITS: usize = 32;
pub(crate) const SPAN_DIGITS: usize = 16;

/// A wait a held state reads: whom it waits on, for which move, and on
/// which stage, none where it waits on the submission or the place
/// (217; design 7).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WaitRead {
    pub on: WaitParty,
    pub for_move: String,
    pub stage: Option<String>,
}

impl WaitRead {
    fn is(&self, held: &WaitHeld) -> bool {
        self.on == held.on && self.for_move == held.for_move && self.stage == held.stage
    }
}

/// The waits `held` reads while it stands, in design 7's order: its
/// caller's move, a person's at a dialog its program shows; each stage
/// waiting for room on the host; the merge queue from a submission sent
/// until it is read landed or refused; and a person while a live start's
/// agent was read asking. None once the place's work is over (217; 70,
/// 90, 111, 130).
pub(crate) fn waits_of(held: &HeldState) -> Vec<WaitRead> {
    if !matches!(held.phase, Phase::Making { .. } | Phase::Live) {
        return vec![];
    }
    let mut waits = vec![];
    if let Some(waiting) = &held.waiting {
        let (on, for_move) = match waiting.on.why {
            Waits::Dialog => (WaitParty::Person, "dialog".to_string()),
            why => (WaitParty::Caller, word_of(why)),
        };
        waits.push(WaitRead {
            on,
            for_move,
            stage: waiting.on.stage.clone(),
        });
    }
    for stage in &held.stages {
        if stage.state == StageState::WaitingForRoom {
            waits.push(WaitRead {
                on: WaitParty::Host,
                for_move: "room".into(),
                stage: Some(stage.name.clone()),
            });
        }
    }
    let submission = &held.submission;
    if submission.submits > 0 && !submission.due && submission.verdict.is_none() {
        waits.push(WaitRead {
            on: WaitParty::MergeQueue,
            for_move: "landing".into(),
            stage: None,
        });
    }
    for stage in &held.stages {
        if let Some(start) = stage.start.as_ref().filter(|start| start.live) {
            if start.asking_since.is_some() && stage.state.running() {
                waits.push(WaitRead {
                    on: WaitParty::Person,
                    for_move: "asking".into(),
                    stage: Some(stage.name.clone()),
                });
            }
        }
    }
    waits
}

/// Why a place waits on its caller, in its serde form (56, 130).
fn word_of(why: Waits) -> String {
    serde_json::to_value(why)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_default()
}

/// What a write's signals are made with besides its two states: the
/// write's time, a new id of so many hex digits from the host's source
/// of secrets, and the conversation a start's agent runs as the agent
/// seam reads it (217, 218).
pub(crate) struct Tracing<'a> {
    pub now: Seconds,
    pub id: &'a mut dyn FnMut(usize) -> String,
    pub conversation: &'a dyn Fn(&StartHeld) -> Option<String>,
}

/// The signals one write makes, from `was`, the state it read, none for
/// a launch's first write, to `next`, the state it writes, which it
/// changes as it names them: the place's trace where it has none, from
/// the write's time; a span for each start that runs and has none, with
/// a `start` record; each start whose stage left running, or that a fresh
/// start took over, its span ended with an `end` record; a `stuck` record
/// for a stage that became stuck; a `hold` record for each hold put and
/// `hold.ended` for each gone; each wait read gone ended, with its span
/// and a `wait.ended` record, and each read new opened with a span and a
/// `wait.begun` record; a `stall` record once the place is read stalled;
/// and the place's span once its work is over. Nothing goes once the
/// place's span went (217, 218; design 7).
pub(crate) fn signals_between(
    was: Option<&HeldState>,
    next: &mut HeldState,
    tracing: Tracing<'_>,
) -> Vec<Signal> {
    let Tracing {
        now,
        id,
        conversation,
    } = tracing;
    if next
        .trace
        .as_ref()
        .is_some_and(|trace| trace.ended.is_some())
    {
        return vec![];
    }
    let trace = next
        .trace
        .get_or_insert_with(|| TraceHeld {
            trace: id(TRACE_DIGITS),
            span: id(SPAN_DIGITS),
            from: now,
            ended: None,
        })
        .clone();
    let place = place_attributes(next);
    let mut signals = vec![];
    let record = |span: &str, event: &str, mut attributes: Attributes| {
        attributes.insert(keys::EVENT_NAME.into(), event.into());
        Signal::Record {
            trace: trace.trace.clone(),
            span: span.to_string(),
            at: now,
            event: event.to_string(),
            attributes,
        }
    };
    let span = |span: &str, name: &str, from: Seconds, attributes: Attributes| Signal::Span {
        trace: trace.trace.clone(),
        span: span.to_string(),
        parent: Some(trace.span.clone()),
        name: name.to_string(),
        from,
        to: now,
        attributes,
    };
    let over = place_over(&next.phase);

    for index in 0..next.stages.len() {
        let was_stage = was.and_then(|was| was.stages.get(index));
        let was_start = was_stage.and_then(|stage| stage.start.as_ref());
        let was_running = was_stage.is_some_and(|stage| stage.state.running());
        let stage = &mut next.stages[index];
        let same = match (was_start, stage.start.as_ref()) {
            (Some(was), Some(now)) => was.n == now.n && was.identity == now.identity,
            _ => false,
        };
        if was_running && !same {
            if let Some(was_start) = was_start {
                let span_id = was_start.span.clone().unwrap_or_else(|| id(SPAN_DIGITS));
                let attributes = start_attributes(&place, &stage.name, was_start, conversation)
                    .with(keys::START_END, "replaced");
                signals.push(span(
                    &span_id,
                    &stage.name,
                    was_start.started_at,
                    attributes,
                ));
                signals.push(record(
                    &span_id,
                    "end",
                    stage_attributes(&place, &stage.name, was_start.n)
                        .with(keys::START_END, "replaced"),
                ));
            }
        }
        let state = stage.state;
        let name = stage.name.clone();
        let end = stage.end.as_ref().and_then(|end| end.needs);
        let Some(start) = stage.start.as_mut() else {
            continue;
        };
        let live_now = state.running() && !over;
        if start.span.is_none() && (state.running() || (was_running && same)) {
            let named = id(SPAN_DIGITS);
            start.span = Some(named.clone());
            signals.push(record(
                &named,
                "start",
                stage_attributes(&place, &name, start.n),
            ));
        }
        let Some(start_span) = start.span.clone() else {
            continue;
        };
        if ((was_running && same) || (!same && state.running())) && !live_now {
            let (end_word, needs) = end_of(state, end);
            let mut attributes = start_attributes(&place, &name, start, conversation)
                .with(keys::START_END, end_word);
            let mut ended =
                stage_attributes(&place, &name, start.n).with(keys::START_END, end_word);
            if let Some(needs) = needs {
                attributes = attributes.with(keys::START_NEEDS, needs);
                ended = ended.with(keys::START_NEEDS, needs);
            }
            signals.push(span(&start_span, &name, start.started_at, attributes));
            signals.push(record(&start_span, "end", ended));
        }
        let was_stuck = was_stage.is_some_and(|stage| stage.state == StageState::Stuck);
        if state == StageState::Stuck && !was_stuck && same {
            signals.push(record(
                &start_span,
                "stuck",
                stage_attributes(&place, &name, start.n),
            ));
        }
    }

    let was_holds = was.map(|was| was.holds.as_slice()).unwrap_or_default();
    let same_hold =
        |a: &flume_seams::Hold, b: &flume_seams::Hold| a.on == b.on && a.since == b.since;
    for hold in &next.holds {
        if !was_holds.iter().any(|was| same_hold(was, hold)) {
            signals.push(record(
                &trace.span,
                "hold",
                hold_attributes(&place, &hold.on),
            ));
        }
    }
    for hold in was_holds {
        if !next.holds.iter().any(|now| same_hold(now, hold)) {
            signals.push(record(
                &trace.span,
                "hold.ended",
                hold_attributes(&place, &hold.on),
            ));
        }
    }

    let read = waits_of(next);
    let outcome = outcome_of(next);
    let mut open = vec![];
    for wait in std::mem::take(&mut next.waits) {
        if read.iter().any(|read| read.is(&wait)) {
            open.push(wait);
            continue;
        }
        let mut attributes = wait_attributes(&place, &wait);
        if wait.on == WaitParty::MergeQueue {
            if let Some(outcome) = outcome {
                attributes = attributes.with(keys::WAIT_OUTCOME, outcome);
            }
        }
        let name = format!("wait on {}", wait.on.word());
        signals.push(span(&wait.span, &name, wait.since, attributes));
        signals.push(record(&wait.span, "wait.ended", wait_named(&place, &wait)));
    }
    for wait in &read {
        if open.iter().any(|open| wait.is(open)) {
            continue;
        }
        let opened = WaitHeld {
            span: id(SPAN_DIGITS),
            on: wait.on,
            for_move: wait.for_move.clone(),
            stage: wait.stage.clone(),
            since: now,
            held: false,
            stalled: false,
        };
        signals.push(record(
            &opened.span,
            "wait.begun",
            wait_named(&place, &opened),
        ));
        open.push(opened);
    }
    let stalled = next.waiting.as_ref().is_some_and(|waiting| waiting.stalled);
    for wait in &mut open {
        wait.held |= held_over(next, wait);
        wait.stalled |= stalled && wait.on == WaitParty::Caller;
    }
    let was_stalled = was
        .and_then(|was| was.waiting.as_ref())
        .is_some_and(|waiting| waiting.stalled);
    if stalled && !was_stalled {
        let under = open
            .iter()
            .find(|wait| wait.on == WaitParty::Caller)
            .map_or(trace.span.clone(), |wait| wait.span.clone());
        let mut attributes = place.clone();
        if let Some(stage) = next.waiting.as_ref().and_then(|w| w.on.stage.clone()) {
            attributes = attributes.with(keys::STAGE, stage);
        }
        signals.push(record(&under, "stall", attributes));
    }
    next.waits = open;

    if over {
        let to = now;
        if let Some(held) = next.trace.as_mut() {
            held.ended = Some(to);
        }
        signals.push(Signal::Span {
            trace: trace.trace.clone(),
            span: trace.span.clone(),
            parent: None,
            name: "place".into(),
            from: trace.from,
            to,
            attributes: place.clone(),
        });
    }
    signals
}

/// Whether the place's work is over as its span reads it: dropped or
/// done, or, for one that landed, removed or tidied (217).
fn place_over(phase: &Phase) -> bool {
    matches!(
        phase,
        Phase::Over {
            over: Over::Dropped | Over::Done,
            ..
        } | Phase::Gone { .. }
            | Phase::Tidied { .. }
    )
}

/// How a start whose stage now stands in `state` ended, and whether one
/// ended short needs an answer or found work (217; design 7).
fn end_of(state: StageState, needs: Option<StopKind>) -> (&'static str, Option<&'static str>) {
    match state {
        StageState::Delivered | StageState::Repeating => ("delivered", None),
        StageState::EndedShort => (
            "short",
            Some(match needs.unwrap_or_default() {
                StopKind::NeedsAnswer => "answer",
                StopKind::FoundWork => "found",
            }),
        ),
        StageState::Finished => ("finished", None),
        StageState::Lost => ("lost", None),
        StageState::NeverBegun => ("never-begun", None),
        StageState::WaitingForRoom => ("replaced", None),
        _ => ("stopped", None),
    }
}

/// How the submission a merge queue's wait waited on ended, as the
/// state that ends the wait reads it (111, 217).
fn outcome_of(held: &HeldState) -> Option<&'static str> {
    match &held.submission.verdict {
        Some(Verdict::Landed { .. }) => Some("landed"),
        Some(Verdict::Failed { .. }) => Some("failed"),
        Some(Verdict::Conflict { .. }) => Some("conflict"),
        Some(Verdict::Withdrawn) => Some("withdrawn"),
        Some(Verdict::Refused { .. }) => Some("refused"),
        None => match &held.phase {
            Phase::Over {
                over: Over::Landed { .. },
                ..
            } => Some("landed"),
            _ => None,
        },
    }
}

/// Whether a hold stands over `wait`'s move: a hold on its stage, the
/// submission's over the merge queue's, and a hold on its caller's move
/// or the stage its caller's move is on hold for (130, 217).
fn held_over(held: &HeldState, wait: &WaitHeld) -> bool {
    match wait.on {
        WaitParty::Caller => {
            held.waiting_hold().is_some()
                || held
                    .waiting
                    .as_ref()
                    .is_some_and(|waiting| waiting.on.why == Waits::OnHold)
        }
        WaitParty::MergeQueue => held.holds.iter().any(|hold| hold.on == HoldOn::Submission),
        WaitParty::Host | WaitParty::Person => wait
            .stage
            .as_deref()
            .is_some_and(|stage| held.hold_on(stage).is_some()),
    }
}

/// An attribute set gaining one more.
trait With {
    fn with(self, key: &str, value: impl Into<Value>) -> Self;
}

impl With for Attributes {
    fn with(mut self, key: &str, value: impl Into<Value>) -> Attributes {
        self.insert(key.to_string(), value.into());
        self
    }
}

/// The repository as a signal names it: its address, else its folder's
/// last name, never its path (218; core.55, core.56).
pub(crate) fn repository_named(held: &HeldState) -> String {
    held.place.address.clone().unwrap_or_else(|| {
        held.place
            .repository
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .unwrap_or_default()
            .to_string()
    })
}

/// What every span and record names of its place: its repository, kind
/// and version, name, base, stream, intent and mark (218).
pub(crate) fn place_attributes(held: &HeldState) -> Attributes {
    let place = &held.place;
    let repository = repository_named(held);
    let intent = match &place.intent {
        IntentKey::Launch { name, .. } => name.clone(),
        IntentKey::Base { base, .. } => format!("base:{repository}:{base}"),
    };
    let mut attributes = Attributes::new()
        .with(keys::REPOSITORY, repository)
        .with(keys::KIND, place.kind.name.clone())
        .with(keys::KIND_VERSION, place.kind.version)
        .with(keys::PLACE, place.name.clone())
        .with(keys::BASE, place.base.clone())
        .with(keys::STREAM, place.stream.name.clone())
        .with(keys::INTENT, intent);
    if let Some(mark) = &place.mark {
        attributes = attributes.with(keys::MARK, mark.clone());
    }
    attributes
}

/// The place's facts with the stage and start a signal concerns.
fn stage_attributes(place: &Attributes, stage: &str, start: u32) -> Attributes {
    place
        .clone()
        .with(keys::STAGE, stage)
        .with(keys::START, start)
}

/// What a start's span names: its stage and number, its repetition where
/// its stage repeats, its program, the conversation its agent runs and
/// the account it runs under (217, 218).
fn start_attributes(
    place: &Attributes,
    stage: &str,
    start: &StartHeld,
    conversation: &dyn Fn(&StartHeld) -> Option<String>,
) -> Attributes {
    let mut attributes =
        stage_attributes(place, stage, start.n).with(keys::PROGRAM, start.program.clone());
    if let Some(group) = &start.group {
        let repetition = group
            .number
            .clone()
            .unwrap_or_else(|| (group.index + 1).to_string());
        attributes = attributes.with(keys::REPETITION, repetition);
    }
    if let Some(session) = conversation(start) {
        attributes = attributes.with(keys::SESSION_ID, session);
    }
    if let Some(account) = &start.account {
        attributes = attributes.with(keys::ACCOUNT, format!("{}/{}", account.pool, account.name));
    }
    attributes
}

/// What a wait's records name: the place, whom it waits on and for which
/// move, and its stage where it is on one (217).
fn wait_named(place: &Attributes, wait: &WaitHeld) -> Attributes {
    let mut attributes = place
        .clone()
        .with(keys::WAIT_ON, wait.on.word())
        .with(keys::WAIT_MOVE, wait.for_move.clone());
    if let Some(stage) = &wait.stage {
        attributes = attributes.with(keys::STAGE, stage.clone());
    }
    attributes
}

/// What a wait's span names: its records' facts, and whether a hold
/// stood over it and whether it stalled (217).
fn wait_attributes(place: &Attributes, wait: &WaitHeld) -> Attributes {
    wait_named(place, wait)
        .with(keys::WAIT_HELD, wait.held)
        .with(keys::WAIT_STALLED, wait.stalled)
}

/// What a hold's record names: the place and the move held, a stage by
/// its name, never the hold's words (217, 218).
fn hold_attributes(place: &Attributes, on: &HoldOn) -> Attributes {
    match on {
        HoldOn::Stage { stage } => place
            .clone()
            .with(keys::WAIT_MOVE, "stage")
            .with(keys::STAGE, stage.clone()),
        HoldOn::Submission => place.clone().with(keys::WAIT_MOVE, "submission"),
        HoldOn::Waiting => place.clone().with(keys::WAIT_MOVE, "waiting"),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::Step;
    use flume_seams::{
        AccountFrom, AccountHeld, Act, Binding, Dialog, Hold, Host, Identity, Prompt, Readiness,
        Revision, Secret, Settings, StageEnd, StageHeld, StartRecord, SubmissionHeld, Waiting,
        WaitingOn, STALL_AFTER,
    };
    use std::collections::BTreeMap;

    /// A phrase no signal holds, which the place's words carry.
    pub(crate) const PHRASE: &str = "a distinctive heron-blue phrase";

    /// The place `fix--one` of `/home/someone/r`, live, with one stage,
    /// `code`, not yet reached, its brief holding [`PHRASE`] and a path.
    pub(crate) fn held() -> HeldState {
        let mut place = crate::tests::record(Binding::default());
        place.repository = "/home/someone/r".into();
        place.location = "/home/someone/r/places/fix--one".into();
        place.brief = Some(format!("mend it: {PHRASE} in /home/someone/r/src"));
        place.mark = Some("FL-7".into());
        place.kind.stages = vec![code_stage()];
        HeldState {
            place,
            phase: Phase::Live,
            readiness: Readiness::default(),
            stages: vec![StageHeld {
                name: "code".into(),
                state: StageState::NotReached,
                starts: 0,
                refused_part: None,
                refused_setting: None,
                end: None,
                start: None,
                since: None,
                room: None,
                made_from: None,
            }],
            current: Some(0),
            submission: SubmissionHeld::default(),
            failures: vec![],
            priority: 0,
            shown: None,
            ends: vec![],
            role: None,
            holds: vec![],
            waiting: None,
            stall_after: STALL_AFTER,
            unwoken: BTreeMap::new(),
            moves: vec![],
            unsettled: vec![],
            uncarried: vec![],
            terminals: vec![],
            forecast: None,
            fixes: vec![],
            trace: None,
            waits: vec![],
        }
    }

    /// The stage `code`, delivering commits.
    fn code_stage() -> flume_seams::Stage {
        flume_seams::Stage {
            name: "code".into(),
            prompt: "{{brief}}{{needs}}".into(),
            conversation: false,
            deliverables: vec![flume_seams::Deliverable::Commits],
            agents: vec![Settings::default()],
            waits: false,
            quiet_limit: None,
            send_back_to: None,
            send_back_limit: None,
            makes: vec![],
            repeats: false,
        }
    }

    /// Start `n` of `code`, made at `at`, its words and prompt holding
    /// [`PHRASE`] and a path, under the account `work/work-1`.
    pub(crate) fn start(n: u32, at: Seconds) -> StartHeld {
        StartHeld {
            n,
            started_at: at,
            start_head: Revision::new("c1"),
            made_from: 0,
            words: Some(format!("{PHRASE}: see /home/someone/notes")),
            identity: Identity {
                repository: "0a1b2c3d4e5f".into(),
                place: "fix--one".into(),
                stage: "code".into(),
                start: n,
            },
            secret: Secret(format!("secret-{n}")),
            program: "agent".into(),
            settings: Settings::default(),
            from: BTreeMap::new(),
            account: Some(AccountHeld {
                pool: "work".into(),
                name: "work-1".into(),
                from: AccountFrom::Strategy,
            }),
            start_record: StartRecord::default(),
            record: None,
            prompt: Prompt {
                text: format!("the brief: {PHRASE}"),
                ..Prompt::default()
            },
            live: true,
            begun_at: Some(at),
            first_quiet: None,
            last_head: Revision::new("c1"),
            last_head_move: at,
            past_limit: false,
            printed: None,
            group: None,
            may_name: vec![],
            opening: None,
            dialog: None,
            conversation: None,
            session: None,
            label: None,
            terminal: None,
            own: None,
            answered: 0,
            span: None,
            asking_since: None,
        }
    }

    /// `held` with its stage in `state` under start `n`, made at `at`.
    fn running(mut held: HeldState, state: StageState, n: u32, at: Seconds) -> HeldState {
        let stage = &mut held.stages[0];
        stage.state = state;
        stage.starts = n;
        let keep = stage.start.as_ref().filter(|start| start.n == n).cloned();
        stage.start = Some(keep.unwrap_or_else(|| start(n, at)));
        held
    }

    /// `held` waiting on its caller for `why` on `code` since `at`.
    fn waiting(mut held: HeldState, why: Waits, at: Seconds) -> HeldState {
        held.waiting = Some(Waiting {
            on: WaitingOn {
                act: if why == Waits::Dialog {
                    Act::Answer
                } else {
                    Act::Retry
                },
                stage: Some("code".into()),
                why,
                since: Some(at),
                words: Some(format!("{PHRASE} at /home/someone/r")),
                open_until: None,
                closed: None,
            },
            stalled: false,
        });
        held
    }

    /// Ids counted from one, each of `digits` hex digits.
    pub(crate) fn counted() -> impl FnMut(usize) -> String {
        let mut n = 0u64;
        move |digits| {
            n += 1;
            format!("{n:0digits$x}")
        }
    }

    /// The signals of one write from `was` to `next` at `now`, the
    /// conversation `s-<n>` for start `n`; `next` as it is written.
    fn write(
        was: Option<&HeldState>,
        next: &mut HeldState,
        now: Seconds,
        id: &mut dyn FnMut(usize) -> String,
    ) -> Vec<Signal> {
        let conversation = |start: &StartHeld| Some(format!("s-{}", start.n));
        signals_between(
            was,
            next,
            Tracing {
                now,
                id,
                conversation: &conversation,
            },
        )
    }

    /// Each signal's event, or `span <name>` for a span.
    fn named(signals: &[Signal]) -> Vec<String> {
        signals
            .iter()
            .map(|signal| match signal {
                Signal::Span { name, .. } => format!("span {name}"),
                Signal::Record { event, .. } => event.clone(),
                Signal::Host(_) => "host".into(),
                Signal::Warning { .. } => "warning".into(),
            })
            .collect()
    }

    /// The span named `name` among `signals`: its parent, times and
    /// attributes.
    fn span_named<'a>(
        signals: &'a [Signal],
        name: &str,
    ) -> (&'a Option<String>, Seconds, Seconds, &'a Attributes) {
        signals
            .iter()
            .find_map(|signal| match signal {
                Signal::Span {
                    name: found,
                    parent,
                    from,
                    to,
                    attributes,
                    ..
                } if found == name => Some((parent, *from, *to, attributes)),
                _ => None,
            })
            .unwrap_or_else(|| panic!("no span {name}: {signals:?}"))
    }

    fn text(value: &str) -> Value {
        Value::Text(value.into())
    }

    #[test]
    fn the_waits_a_state_reads() {
        // Design 7's second table: the caller's move, a person's at a
        // dialog, the host for room, the merge queue from a submission
        // to its outcome, and a person while a live start's agent asks;
        // none once the place's work is over (217).
        let wait = |on, for_move: &str, stage: Option<&str>| WaitRead {
            on,
            for_move: for_move.into(),
            stage: stage.map(str::to_string),
        };
        assert_eq!(waits_of(&held()), vec![]);
        for (why, word) in [
            (Waits::Answer, "answer"),
            (Waits::OnHold, "on_hold"),
            (Waits::Lost, "lost"),
            (Waits::NeverBegun, "never_begun"),
            (Waits::Unready, "unready"),
            (Waits::Refused, "refused"),
            (Waits::Stopped, "stopped"),
            (Waits::PastBound, "past_bound"),
            (Waits::Submission, "submission"),
            (Waits::Delivered, "delivered"),
            (Waits::Unsubmitted, "unsubmitted"),
        ] {
            assert_eq!(
                waits_of(&waiting(held(), why, 10)),
                vec![wait(WaitParty::Caller, word, Some("code"))],
                "{why:?}"
            );
        }
        let mut dialog = waiting(held(), Waits::Dialog, 10);
        dialog = running(dialog, StageState::Starting, 1, 5);
        dialog.stages[0].start.as_mut().unwrap().dialog = Some(Dialog {
            words: "Trust this folder?".into(),
            since: 10,
        });
        assert_eq!(
            waits_of(&dialog),
            vec![wait(WaitParty::Person, "dialog", Some("code"))]
        );
        let mut room = held();
        room.stages[0].state = StageState::WaitingForRoom;
        assert_eq!(
            waits_of(&room),
            vec![wait(WaitParty::Host, "room", Some("code"))]
        );
        let mut queued = held();
        queued.submission.submits = 1;
        assert_eq!(
            waits_of(&queued),
            vec![wait(WaitParty::MergeQueue, "landing", None)]
        );
        queued.submission.due = true;
        assert_eq!(waits_of(&queued), vec![], "due and not yet sent");
        queued.submission.due = false;
        queued.submission.verdict = Some(Verdict::Failed {
            words: "a check failed".into(),
        });
        assert_eq!(waits_of(&queued), vec![], "read refused");
        let mut asking = running(held(), StageState::Begun, 1, 5);
        asking.stages[0].start.as_mut().unwrap().asking_since = Some(20);
        assert_eq!(
            waits_of(&asking),
            vec![wait(WaitParty::Person, "asking", Some("code"))]
        );
        asking.phase = Phase::Over {
            over: Over::Dropped,
            kept: None,
        };
        assert_eq!(waits_of(&asking), vec![], "the work is over");
    }

    #[test]
    fn a_start_its_end_and_its_wait() {
        // S129's first start: it runs, stops short needing an answer, the
        // place waits forty minutes on its caller, and the retry's start
        // delivers; each start a span beneath the place's, the wait a
        // span of its own (217, S129).
        let mut ids = counted();
        let launched = held();
        let mut first = launched.clone();
        let made = write(None, &mut first, 100, &mut ids);
        assert_eq!(named(&made), Vec::<String>::new());
        let trace = first.trace.clone().expect("the launch names the trace");
        assert_eq!((trace.trace.len(), trace.span.len()), (32, 16));
        assert_eq!((trace.from, trace.ended), (100, None));

        let mut started = running(first.clone(), StageState::Started, 1, 160);
        let signals = write(Some(&first), &mut started, 160, &mut ids);
        assert_eq!(named(&signals), ["start"]);
        let span = started.stages[0].start.as_ref().unwrap().span.clone();
        assert_eq!(span.as_ref().map(String::len), Some(16));

        let mut short = running(started.clone(), StageState::EndedShort, 1, 160);
        short.stages[0].start.as_mut().unwrap().live = false;
        short.stages[0].end = Some(StageEnd {
            needs: Some(StopKind::NeedsAnswer),
            words: Some(format!("{PHRASE}: which way?")),
            ..StageEnd::default()
        });
        short = waiting(short, Waits::Answer, 3760);
        let signals = write(Some(&started), &mut short, 3760, &mut ids);
        assert_eq!(named(&signals), ["span code", "end", "wait.begun"]);
        let (parent, from, to, attributes) = span_named(&signals, "code");
        assert_eq!(parent.as_deref(), Some(trace.span.as_str()));
        assert_eq!((from, to), (160, 3760));
        assert_eq!(attributes[keys::START_END], text("short"));
        assert_eq!(attributes[keys::START_NEEDS], text("answer"));
        assert_eq!(attributes[keys::START], Value::Number(1));
        assert_eq!(attributes[keys::STAGE], text("code"));
        assert_eq!(attributes[keys::PROGRAM], text("agent"));
        assert_eq!(attributes[keys::SESSION_ID], text("s-1"));
        assert_eq!(attributes[keys::ACCOUNT], text("work/work-1"));
        assert_eq!(short.waits.len(), 1);
        assert_eq!(
            (short.waits[0].on, short.waits[0].since),
            (WaitParty::Caller, 3760)
        );

        let mut kept = short.clone();
        kept.priority = 3;
        assert_eq!(
            named(&write(Some(&short), &mut kept, 4000, &mut ids)),
            Vec::<String>::new(),
            "a write that changes no wait or start sends nothing"
        );

        let mut retried = running(kept.clone(), StageState::Started, 2, 6160);
        retried.waiting = None;
        retried.stages[0].end = None;
        let signals = write(Some(&kept), &mut retried, 6160, &mut ids);
        assert_eq!(
            named(&signals),
            ["start", "span wait on caller", "wait.ended"]
        );
        let (parent, from, to, attributes) = span_named(&signals, "wait on caller");
        assert_eq!(parent.as_deref(), Some(trace.span.as_str()));
        assert_eq!(to - from, 40 * 60, "forty minutes on the caller");
        assert_eq!(attributes[keys::WAIT_MOVE], text("answer"));
        assert_eq!(attributes[keys::WAIT_HELD], Value::Flag(false));
        assert!(retried.waits.is_empty());

        let mut delivered = running(retried.clone(), StageState::Delivered, 2, 6160);
        delivered.stages[0].start.as_mut().unwrap().live = false;
        let signals = write(Some(&retried), &mut delivered, 9760, &mut ids);
        assert_eq!(named(&signals), ["span code", "end"]);
        let (_, from, to, attributes) = span_named(&signals, "code");
        assert_eq!((from, to), (6160, 9760));
        assert_eq!(attributes[keys::START], Value::Number(2));
        assert_eq!(attributes[keys::START_END], text("delivered"));
        assert!(!attributes.contains_key(keys::START_NEEDS));
    }

    #[test]
    fn a_wait_whose_move_changes_ends_and_another_begins() {
        // A wait whose move changes is a new one: the first ends and the
        // next begins at the write that read the change; a hold over it
        // and its stall are named on its span (217, 130).
        let mut ids = counted();
        let mut answer = waiting(held(), Waits::Answer, 100);
        write(None, &mut answer, 100, &mut ids);
        let mut stalled = answer.clone();
        stalled.waiting.as_mut().unwrap().stalled = true;
        let signals = write(Some(&answer), &mut stalled, 400, &mut ids);
        assert_eq!(named(&signals), ["stall"]);
        let Signal::Record { span, .. } = &signals[0] else {
            panic!("a record");
        };
        assert_eq!(span, &stalled.waits[0].span, "the stall under its wait");
        assert!(stalled.waits[0].stalled);

        let mut lost = waiting(stalled.clone(), Waits::Lost, 500);
        let signals = write(Some(&stalled), &mut lost, 500, &mut ids);
        assert_eq!(
            named(&signals),
            ["span wait on caller", "wait.ended", "wait.begun"]
        );
        let (_, from, to, attributes) = span_named(&signals, "wait on caller");
        assert_eq!((from, to), (100, 500));
        assert_eq!(attributes[keys::WAIT_MOVE], text("answer"));
        assert_eq!(attributes[keys::WAIT_STALLED], Value::Flag(true));
        assert_eq!(lost.waits.len(), 1);
        assert_eq!(
            (lost.waits[0].for_move.as_str(), lost.waits[0].since),
            ("lost", 500)
        );
        assert!(!lost.waits[0].stalled);

        let mut held_on = lost.clone();
        held_on.holds.push(Hold {
            on: HoldOn::Waiting,
            words: Some(format!("{PHRASE} first")),
            since: 600,
            bound: None,
            lapsed: None,
        });
        let signals = write(Some(&lost), &mut held_on, 600, &mut ids);
        assert_eq!(named(&signals), ["hold"]);
        let Signal::Record { attributes, .. } = &signals[0] else {
            panic!("a record");
        };
        assert_eq!(attributes[keys::WAIT_MOVE], text("waiting"));
        assert!(held_on.waits[0].held);

        let mut roomless = held_on.clone();
        roomless.waiting = None;
        roomless.holds.clear();
        roomless.stages[0].state = StageState::WaitingForRoom;
        let signals = write(Some(&held_on), &mut roomless, 700, &mut ids);
        assert_eq!(
            named(&signals),
            [
                "hold.ended",
                "span wait on caller",
                "wait.ended",
                "wait.begun"
            ]
        );
        let (_, _, _, attributes) = span_named(&signals, "wait on caller");
        assert_eq!(attributes[keys::WAIT_HELD], Value::Flag(true));
        assert_eq!(roomless.waits[0].on, WaitParty::Host);
    }

    #[test]
    fn the_places_span_ends_at_a_drop_and_at_a_landings_tidy() {
        // The place's span, with no parent, from its launch to its drop,
        // its working start's ended stopped at the drop; and from its
        // launch to its tidy for one that landed, its merge queue's wait
        // ended landed at the settle that read the landing. Nothing goes
        // after (217).
        let mut ids = counted();
        let mut launched = held();
        write(None, &mut launched, 100, &mut ids);
        let trace = launched.trace.clone().unwrap();
        let mut working = running(launched.clone(), StageState::Begun, 1, 200);
        write(Some(&launched), &mut working, 200, &mut ids);
        let mut dropped = running(working.clone(), StageState::Stopped, 1, 200);
        dropped.stages[0].start.as_mut().unwrap().live = false;
        dropped.phase = Phase::Over {
            over: Over::Dropped,
            kept: None,
        };
        let signals = write(Some(&working), &mut dropped, 900, &mut ids);
        assert_eq!(named(&signals), ["span code", "end", "span place"]);
        let (_, from, to, attributes) = span_named(&signals, "code");
        assert_eq!((from, to), (200, 900));
        assert_eq!(attributes[keys::START_END], text("stopped"));
        let (parent, from, to, _) = span_named(&signals, "place");
        assert_eq!((parent, from, to), (&None, 100, 900));
        assert_eq!(dropped.trace.as_ref().unwrap().ended, Some(900));
        assert!(matches!(
            &signals[2],
            Signal::Span { span, .. } if *span == trace.span
        ));
        let mut gone = dropped.clone();
        gone.phase = Phase::Gone {
            over: Over::Dropped,
            at: Some(950),
        };
        assert!(write(Some(&dropped), &mut gone, 950, &mut ids).is_empty());

        let mut ids = counted();
        let mut submitted = held();
        write(None, &mut submitted, 100, &mut ids);
        let mut queued = running(submitted.clone(), StageState::Delivered, 1, 100);
        queued.submission.submits = 1;
        let signals = write(Some(&submitted), &mut queued, 1000, &mut ids);
        assert_eq!(named(&signals), ["wait.begun"]);
        let mut landed = queued.clone();
        landed.submission.verdict = Some(Verdict::Landed {
            commit: Revision::new("c9"),
            place_removed: false,
            branch_removed: false,
        });
        landed.phase = Phase::Over {
            over: Over::Landed {
                commit: Revision::new("c9"),
            },
            kept: None,
        };
        let signals = write(Some(&queued), &mut landed, 4000, &mut ids);
        assert_eq!(named(&signals), ["span wait on merge-queue", "wait.ended"]);
        let (_, from, to, attributes) = span_named(&signals, "wait on merge-queue");
        assert_eq!((from, to), (1000, 4000), "fifty minutes on the merge queue");
        assert_eq!(attributes[keys::WAIT_OUTCOME], text("landed"));
        let mut tidied = landed.clone();
        tidied.phase = Phase::Gone {
            over: Over::Landed {
                commit: Revision::new("c9"),
            },
            at: Some(4100),
        };
        let signals = write(Some(&landed), &mut tidied, 4100, &mut ids);
        assert_eq!(named(&signals), ["span place"]);
        let (parent, from, to, _) = span_named(&signals, "place");
        assert_eq!((parent, from, to), (&None, 100, 4100));
    }

    #[test]
    fn no_signal_holds_words_or_a_path() {
        // Every span and record names the place's repository, kind and
        // version, name, base, stream, intent and mark; none holds a word
        // of a brief, a start's words, a wait's, a hold's or a dialog's,
        // or a path (218; core.56).
        let mut ids = counted();
        let mut signals = vec![];
        let mut was = held();
        signals.extend(write(None, &mut was, 100, &mut ids));
        let mut steps = vec![];
        let mut dialog = waiting(
            running(was.clone(), StageState::Starting, 1, 110),
            Waits::Dialog,
            110,
        );
        dialog.stages[0].start.as_mut().unwrap().dialog = Some(Dialog {
            words: format!("{PHRASE} in /home/someone"),
            since: 110,
        });
        steps.push(dialog);
        let mut short = running(held(), StageState::EndedShort, 1, 110);
        short.stages[0].end = Some(StageEnd {
            words: Some(format!("{PHRASE} at /home/someone/r/src/lib.rs")),
            needs: Some(StopKind::FoundWork),
            ..StageEnd::default()
        });
        short = waiting(short, Waits::Answer, 200);
        short.holds.push(Hold {
            on: HoldOn::Submission,
            words: Some(format!("{PHRASE} until /home/someone/ok")),
            since: 200,
            bound: None,
            lapsed: None,
        });
        steps.push(short);
        let mut dropped = steps.last().unwrap().clone();
        dropped.phase = Phase::Over {
            over: Over::Dropped,
            kept: None,
        };
        steps.push(dropped);
        for (n, mut next) in steps.into_iter().enumerate() {
            next.trace = was.trace.clone();
            next.waits = was.waits.clone();
            if let (Some(start), Some(span)) = (
                next.stages[0].start.as_mut(),
                was.stages[0]
                    .start
                    .as_ref()
                    .and_then(|start| start.span.clone()),
            ) {
                start.span = Some(span);
            }
            signals.extend(write(Some(&was), &mut next, 200 + n as u64, &mut ids));
            was = next;
        }
        assert!(signals.len() >= 8, "{signals:?}");
        for signal in &signals {
            let shown = format!("{signal:?}");
            assert!(!shown.contains("heron"), "{shown}");
            assert!(!shown.contains("/home"), "{shown}");
            assert!(!shown.contains("Trust"), "{shown}");
            let attributes = match signal {
                Signal::Span { attributes, .. } | Signal::Record { attributes, .. } => attributes,
                other => panic!("{other:?}"),
            };
            for (key, value) in [
                (keys::REPOSITORY, text("r")),
                (keys::KIND, text("fix")),
                (keys::KIND_VERSION, Value::Number(1)),
                (keys::PLACE, text("one")),
                (keys::BASE, text("main")),
                (keys::STREAM, text("default")),
                (keys::INTENT, text("one")),
                (keys::MARK, text("FL-7")),
            ] {
                assert_eq!(attributes.get(key), Some(&value), "{key} of {shown}");
            }
        }
    }

    #[test]
    fn an_intent_by_its_base_and_a_repository_by_its_address() {
        // A launch that named no intent is `base:<repository>:<base>`, and
        // a repository with an address is named by it (218; core.55).
        let mut by_base = held();
        by_base.place.intent = IntentKey::Base {
            repository: "0a1b2c3d4e5f".into(),
            base: "main".into(),
            digest: "d".into(),
        };
        assert_eq!(
            place_attributes(&by_base)[keys::INTENT],
            text("base:r:main")
        );
        by_base.place.address = Some("acme/widgets".into());
        let attributes = place_attributes(&by_base);
        assert_eq!(attributes[keys::REPOSITORY], text("acme/widgets"));
        assert_eq!(attributes[keys::INTENT], text("base:acme/widgets:main"));
    }
    /// The repository the fakes' world binds.
    pub(crate) const REPOSITORY: &str = "/home/someone/r";

    /// The fakes an engine runs over here, for one repository on `main`
    /// whose kind `fix` has the one stage `code`, delivering commits, its
    /// agents the fake agent's, and the telemetry that keeps every signal.
    #[allow(dead_code)]
    pub(crate) struct World {
        pub git: std::sync::Arc<flume_fakes::FakeGit>,
        pub queue: std::sync::Arc<flume_fakes::FakeMergeQueue>,
        pub host: std::sync::Arc<flume_fakes::FakeHost>,
        pub store: std::sync::Arc<flume_fakes::FakeStore>,
        pub agent: std::sync::Arc<flume_fakes::FakeAgent>,
        pub terminal: std::sync::Arc<flume_fakes::FakeTerminal>,
        pub telemetry: std::sync::Arc<flume_fakes::FakeTelemetry>,
        resolver: std::sync::Arc<flume_fakes::FakeResolver>,
        history: std::sync::Arc<flume_fakes::FakeHistory>,
    }

    impl World {
        pub(crate) fn new() -> World {
            use flume_fakes::*;
            use flume_seams::{Kind, KindSettings};
            use std::sync::Arc;
            let git = FakeGit::new();
            git.init(REPOSITORY, "main");
            let host = Arc::new(FakeHost::default());
            host.set_recipient(Some("caller"));
            let place = Arc::new(FakePlace::new(git.clone()));
            let kinds = Arc::new(FakeKinds::default());
            kinds.set_kinds(
                REPOSITORY,
                vec![Kind {
                    name: "fix".into(),
                    version: 1,
                    branch_prefix: "fix".into(),
                    starts_unready: true,
                    lands: true,
                    agents: KindSettings::default(),
                    label: None,
                    stages: vec![code_stage()],
                    fix: None,
                    fix_limit: None,
                }],
            );
            kinds.set_binding(
                REPOSITORY,
                Binding {
                    place: Some("trees".into()),
                    change: None,
                    merge_queue: Some("queue".into()),
                },
            );
            let queue = Arc::new(FakeMergeQueue::new(git.clone()));
            let terminal = Arc::new(FakeTerminal::new(host.clone()));
            let agent = Arc::new(FakeAgent::new(terminal.clone()));
            let resolver = Arc::new(FakeResolver::new(kinds));
            resolver.bind_no_tool_place(place.clone());
            resolver.bind_place("trees", place);
            resolver.bind_merge_queue("queue", queue.clone());
            resolver.bind_agent("agent", agent.clone());
            resolver.with_terminal("fake", terminal.clone());
            World {
                history: Arc::new(FakeHistory::new(git.clone())),
                git,
                queue,
                host,
                store: Arc::new(FakeStore::default()),
                agent,
                terminal,
                telemetry: Arc::new(FakeTelemetry::default()),
                resolver,
            }
        }

        /// An engine over the fakes, as a command builds one, handing its
        /// signals to the world's telemetry.
        pub(crate) fn engine(&self) -> crate::Engine {
            crate::Engine::new(
                self.resolver.clone(),
                self.terminal.clone(),
                self.store.clone(),
                self.history.clone(),
                self.host.clone(),
                crate::Defaults {
                    settings: Settings {
                        program: Some("agent".into()),
                        ..Settings::default()
                    },
                    send_backs: 2,
                },
            )
            .with_telemetry(self.telemetry.clone())
        }

        /// The place `fix--<name>` launched with a brief holding
        /// [`PHRASE`], its id.
        pub(crate) fn launch(&self, name: &str) -> flume_seams::PlaceId {
            let answer = self.engine().launch(&flume_seams::Launch {
                repository: REPOSITORY.into(),
                kind: "fix".into(),
                name: name.into(),
                base: "main".into(),
                brief: Some(format!("mend it: {PHRASE}")),
                ..flume_seams::Launch::default()
            });
            assert!(
                matches!(answer, crate::Answer::Launched { .. }),
                "{answer:?}"
            );
            flume_seams::PlaceId::new(flume_fakes::place_location(
                REPOSITORY,
                &format!("fix--{name}"),
            ))
        }
    }

    #[test]
    fn a_launch_names_its_trace_before_any_start() {
        // The place's first document names its trace, its place's span and
        // the launch's time, before its stage starts; the start's span is
        // named as its claim is written, and its record goes under it in
        // the place's trace (217; core.16).
        let world = World::new();
        let first: std::sync::Arc<std::sync::Mutex<Option<HeldState>>> = Default::default();
        let (store, kept) = (world.store.clone(), first.clone());
        world.store.before_next_write(move || {
            let (store, kept) = (store.clone(), kept.clone());
            store.clone().before_next_write(move || {
                let created = store.places().first().and_then(|id| store.held(id));
                *kept.lock().unwrap() = created;
            });
        });
        let launched_at = world.host.now();
        let id = world.launch("one");
        let created = first.lock().unwrap().clone().expect("the first document");
        let trace = created.trace.clone().expect("named at the launch");
        assert_eq!((trace.trace.len(), trace.span.len()), (32, 16));
        assert!(trace.trace.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!((trace.from, trace.ended), (launched_at, None));
        assert!(created.stages.iter().all(|stage| stage.start.is_none()));

        let held = world.store.held(&id).expect("the place");
        assert_eq!(
            held.trace,
            Some(trace.clone()),
            "every later write keeps it"
        );
        let span = held.stages[0]
            .start
            .as_ref()
            .and_then(|start| start.span.clone())
            .expect("the start's span");
        let started: Vec<Signal> = world
            .telemetry
            .signals()
            .into_iter()
            .filter(|signal| matches!(signal, Signal::Record { event, .. } if event == "start"))
            .collect();
        assert_eq!(started.len(), 1, "{:?}", world.telemetry.signals());
        let Signal::Record {
            trace: of,
            span: under,
            ..
        } = &started[0]
        else {
            unreachable!()
        };
        assert_eq!((of, under), (&trace.trace, &span));
    }

    #[test]
    fn a_write_refused_for_its_version_sends_nothing() {
        // Two settles read one change at once: the one whose write lands
        // sends its signals, and the one refused for its version sends
        // none and, reading again, finds nothing more to send (217;
        // core.17).
        let world = World::new();
        let id = world.launch("one");
        world.telemetry.clear();
        let engine = world.engine();
        let (place, racing) = (id.clone(), world.engine());
        world.store.before_next_write(move || {
            racing.update(&place, |held| {
                held.stages[0].state = StageState::Stuck;
                Step::Write(())
            });
        });
        let mut decided = 0;
        engine.update(&id, |held| {
            decided += 1;
            held.stages[0].state = StageState::Stuck;
            Step::Write(())
        });
        assert_eq!(decided, 2, "read again after the refused write");
        let events = world.telemetry.events();
        assert_eq!(
            events.iter().filter(|event| *event == "stuck").count(),
            1,
            "{events:?}"
        );

        world.telemetry.clear();
        world.store.refuse_next_write_of(&id);
        let mut decided = 0;
        engine.update(&id, |held| {
            decided += 1;
            held.priority = 9;
            held.holds.push(flume_seams::Hold {
                on: HoldOn::Submission,
                words: None,
                since: 1,
                bound: None,
                lapsed: None,
            });
            Step::Write(())
        });
        assert_eq!(decided, 2);
        assert_eq!(
            world.telemetry.events(),
            ["hold"],
            "once, from the write that landed"
        );
    }
    /// The place `fix--<name>` launched and settled until its stage
    /// began, its id and its start's identity.
    fn begun(world: &World, name: &str) -> (flume_seams::PlaceId, Identity) {
        let id = world.launch(name);
        world.engine().settle(Some(&id));
        let held = world.store.held(&id).expect("the place");
        assert_eq!(held.stages[0].state, StageState::Begun, "{held:?}");
        let identity = held.stages[0].start.as_ref().unwrap().identity.clone();
        (id, identity)
    }

    #[test]
    fn a_persons_wait_opens_and_ends_at_the_settles_that_read_it() {
        // A settle that reads the agent asking a person writes since when,
        // which opens the place's wait on a person; the settle that reads
        // it working again clears it, which ends the wait (217, 90).
        let world = World::new();
        let (id, identity) = begun(&world, "one");
        world.telemetry.clear();
        world.host.advance(60);
        let asked = world.host.now();
        world.agent.set_asking(&identity, flume_seams::Reading::Yes);
        world.engine().settle(Some(&id));
        let held = world.store.held(&id).unwrap();
        let start = held.stages[0].start.as_ref().unwrap();
        assert_eq!(start.asking_since, Some(asked));
        assert_eq!(held.waits.len(), 1, "{:?}", held.waits);
        assert_eq!(
            (held.waits[0].on, held.waits[0].for_move.as_str()),
            (WaitParty::Person, "asking")
        );
        assert_eq!(world.telemetry.events(), ["wait.begun"]);

        world.host.advance(120);
        world.engine().settle(Some(&id));
        assert_eq!(
            world.store.held(&id).unwrap().stages[0]
                .start
                .as_ref()
                .unwrap()
                .asking_since,
            Some(asked),
            "a settle that reads it asking again keeps since when"
        );
        world.host.advance(600);
        let answered = world.host.now();
        world.agent.set_asking(&identity, flume_seams::Reading::No);
        world.engine().settle(Some(&id));
        let held = world.store.held(&id).unwrap();
        assert_eq!(held.stages[0].start.as_ref().unwrap().asking_since, None);
        assert!(held.waits.is_empty());
        let spans = world.telemetry.spans();
        let (_, from, to, attributes) = span_named(&spans, "wait on person");
        assert_eq!((from, to), (asked, answered));
        assert_eq!(attributes[keys::WAIT_MOVE], text("asking"));
        assert_eq!(attributes[keys::STAGE], text("code"));
    }
    #[test]
    fn the_agents_resource_keeps_its_own_pairs_before_the_places() {
        // While flume sends, the agent is given its account's pairs, any of
        // a key flume names taken out, then its place's facts with its
        // stage and start, once each, and, while traces go, a TRACEPARENT
        // naming its start's span in its place's trace; neither is in its
        // start's record. With traces off, no TRACEPARENT; sending
        // nothing, neither (219; core.10, core.30).
        use flume_seams::{Account, Pool, Sends, Strategy};
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
                    env: BTreeMap::from([(
                        RESOURCE_VARIABLE.to_string(),
                        "team=ops,flume.stage=x".to_string(),
                    )]),
                    agents: None,
                    usage: None,
                }],
            }),
        );
        world.host.set_host_pool(Some("work"));
        let id = world.launch("one");
        let held = world.store.held(&id).unwrap();
        let start = held.stages[0].start.clone().unwrap();
        let (command, _) = world.terminal.opened(&start.identity).expect("opened");
        assert_eq!(
            command.env[RESOURCE_VARIABLE],
            "team=ops,flume.repository=r,flume.kind=fix,flume.kind.version=1,\
             flume.place=one,flume.base=main,flume.stream=default,flume.intent=base:r:main,\
             flume.stage=code,flume.start=1"
        );
        let trace = held.trace.clone().unwrap();
        assert_eq!(
            command.env[PARENT_VARIABLE],
            format!("00-{}-{}-01", trace.trace, start.span.clone().unwrap())
        );
        for name in [RESOURCE_VARIABLE, PARENT_VARIABLE] {
            assert!(!start.start_record.env.contains_key(name), "{name}");
        }
        assert!(
            start.from.contains_key(&format!("env.{RESOURCE_VARIABLE}")),
            "the account's variable named with its layer: {:?}",
            start.from
        );

        let world = World::new();
        world.telemetry.set_sends(Sends {
            traces: false,
            ..Sends::ALL
        });
        world
            .host
            .set_login(RESOURCE_VARIABLE, Some("team=ops,flume.place=other"));
        let id = world.launch("two");
        let start = world.store.held(&id).unwrap().stages[0]
            .start
            .clone()
            .unwrap();
        let (command, _) = world.terminal.opened(&start.identity).unwrap();
        assert!(
            command.env[RESOURCE_VARIABLE].starts_with("team=ops,flume.repository=r,"),
            "the login environment's pairs first: {}",
            command.env[RESOURCE_VARIABLE]
        );
        assert_eq!(
            command.env[RESOURCE_VARIABLE]
                .matches("flume.place=")
                .count(),
            1
        );
        assert!(!command.env.contains_key(PARENT_VARIABLE));

        let world = World::new();
        let id = world.launch("three");
        let start = world.store.held(&id).unwrap().stages[0]
            .start
            .clone()
            .unwrap();
        let (command, _) = world.terminal.opened(&start.identity).unwrap();
        assert!(!command.env.contains_key(RESOURCE_VARIABLE));
        assert!(!command.env.contains_key(PARENT_VARIABLE));
    }

    #[test]
    fn a_value_is_encoded_where_the_variables_syntax_would_read_it() {
        assert_eq!(encode("fix/one"), "fix/one");
        assert_eq!(encode("a,b=c d"), "a%2Cb%3Dc%20d");
    }
    #[test]
    fn the_hosts_reading_names_no_place() {
        // A settle of every place hands on one reading of the host: the
        // places standing by kind, state and whom they wait on, and each
        // stream's agents running and room; a settle of one place none
        // (220, 121, 208).
        use flume_seams::{PlaceId, Stream};
        let world = World::new();
        world.host.set_stream(
            "work",
            Some(Stream {
                agents: Some(4),
                ..Stream::default()
            }),
        );
        let (id, _) = begun(&world, "alder");
        for name in ["birch", "cedar"] {
            let mut waits = waiting(held(), Waits::Answer, world.host.now());
            waits.place.id = PlaceId::new(format!("/data/places/x/fix--{name}"));
            waits.place.name = name.into();
            waits.place.stream.name = "work".into();
            waits.stages[0].state = StageState::EndedShort;
            world.store.put(&waits.place.id.clone(), waits);
        }
        world.telemetry.clear();
        world.engine().settle(Some(&id));
        let hosts = |world: &World| -> Vec<HostReading> {
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
        assert!(
            hosts(&world).is_empty(),
            "a settle of one place reads no host"
        );
        world.telemetry.clear();
        world.engine().settle(None);
        let read = hosts(&world);
        assert_eq!(read.len(), 1, "{:?}", world.telemetry.signals());
        assert_eq!(
            read[0].places,
            vec![
                PlacesStanding {
                    kind: "fix".into(),
                    state: "waiting on an answer".into(),
                    on: Some("caller".into()),
                    count: 2,
                },
                PlacesStanding {
                    kind: "fix".into(),
                    state: "working".into(),
                    on: None,
                    count: 1,
                },
            ]
        );
        assert_eq!(
            read[0].streams,
            vec![
                StreamAgents {
                    stream: "default".into(),
                    running: 1,
                    room: None,
                },
                StreamAgents {
                    stream: "work".into(),
                    running: 0,
                    room: Some(4),
                },
            ]
        );
        let shown = format!("{read:?}");
        for word in [
            "alder", "birch", "cedar", "/data", REPOSITORY, "work-1", "FL-7",
        ] {
            assert!(!shown.contains(word), "{word} in {shown}");
        }
        world.telemetry.clear();
        world.engine().settle_all();
        assert_eq!(hosts(&world).len(), 1, "settle_all reads the host once");
    }
    #[test]
    fn a_log_line_goes_as_a_warning() {
        // Each line flume writes to the host's log, in its own words, goes
        // as a warning too, at the time it was written (139, 218).
        let world = World::new();
        let id = world.launch("one");
        world.telemetry.clear();
        world.engine().log(&format!(
            "the place {id} was not tidied in a terminal: gone"
        ));
        assert_eq!(
            world.host.logged.calls(),
            [format!("the place {id} was not tidied in a terminal: gone")]
        );
        assert_eq!(
            world.telemetry.signals(),
            [Signal::Warning {
                at: world.host.now(),
                words: format!("the place {id} was not tidied in a terminal: gone"),
            }]
        );
        world.telemetry.clear();
        world.store.refuse_host_writes(true);
        world.engine().settle_every(60);
        let warnings: Vec<String> = world
            .telemetry
            .signals()
            .into_iter()
            .filter_map(|signal| match signal {
                Signal::Warning { words, .. } => Some(words),
                _ => None,
            })
            .collect();
        assert_eq!(warnings, world.host.logged.calls()[1..], "every line goes");
        assert!(
            warnings
                .iter()
                .any(|line| line.starts_with("the host's watch was not written")),
            "{warnings:?}"
        );
    }
}
