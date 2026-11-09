//! The wait by place and the inbox (43, 119, 122, 208; design 9;
//! core.13, core.25). A wait names a place and its stage or its submission, and
//! the recipient it waits as: it answers at once with the newest end of
//! that stage's current start or that submission pending for the
//! recipient, recording it delivered by the wait; answers one already
//! delivered as a read, delivering nothing again; and otherwise follows
//! the host's state until such an end comes or its timeout passes. While
//! a caller keeps watch over the host it reads the store alone, and the
//! caller's settles move the place; while none does it settles every
//! place before each reading after its first, whatever terminal the
//! host binds, as every acting command does,
//! the first reading answering at once an end that stands pending
//! before a settle could deliver it by notice and tidy its place away. The inbox gives
//! every end pending for a recipient, of every place on the host, a
//! tidied place's kept document among them, or, naming a place, of the
//! documents status reads under that name alone, and records each
//! delivered to it in the same write that reads it, so two inboxes at
//! once deliver each end once (75). A recipient no pane holds collects its
//! ends so, a drop's or a done's among them (43, 62). A recipient's
//! default is the caller's
//! environment's, else the role a role's run serves, else the calling
//! session as the terminal names it, as a launch's (7, 132).

use crate::held::stop_short_waited;
use crate::{end_words, Answer, Collected, EndStatus, Engine, WaitingPlace};
use flume_seams::*;

/// How often a wait reads the host's state again (design 9).
pub const WAIT_POLL: Seconds = 2;

/// What a wait waits on: a stage's current start, or the submission.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WaitFor {
    Stage(String),
    Submission,
}

impl Engine {
    /// The recipient a command that takes one acts as where it names
    /// none: the caller's environment's, else the role of the place
    /// `here` names where that place is a role's run, else the calling
    /// session as the first terminal the host binds that names one names
    /// it (7, 132, 190; design 7).
    pub fn recipient(&self, here: Option<&PlaceId>) -> Option<String> {
        if let Some(recipient) = self.host.recipient() {
            return Some(recipient);
        }
        if let Some(role) = here
            .and_then(|place| self.store.read(place))
            .and_then(|read| read.state.role)
        {
            return Some(role);
        }
        self.caller_session()
    }

    /// Waits on the place's stage or submission as `recipient`, for up to
    /// `timeout` where one is given (43; design 9).
    pub fn wait(
        &self,
        place: &PlaceId,
        of: &WaitFor,
        recipient: &str,
        timeout: Option<Seconds>,
    ) -> Answer {
        let watched = !self.attended();
        let deadline = timeout.map(|timeout| self.host.now() + timeout);
        loop {
            match self.waited(place, of, recipient) {
                Err(reason) => return Answer::refused(reason),
                Ok(Some(answer)) => return answer,
                Ok(None) => {}
            }
            if !watched {
                self.settle_all();
                match self.waited(place, of, recipient) {
                    Err(reason) => return Answer::refused(reason),
                    Ok(Some(answer)) => return answer,
                    Ok(None) => {}
                }
            }
            let now = self.host.now();
            let pause = match deadline {
                Some(deadline) if now >= deadline => {
                    return Answer::Waited {
                        place: place.clone(),
                        recipient: recipient.into(),
                        end: None,
                        delivered: false,
                    }
                }
                Some(deadline) => WAIT_POLL.min(deadline - now),
                None => WAIT_POLL,
            };
            self.host.sleep(pause);
        }
    }

    /// One reading of a wait: its answer where an end of what it waits on
    /// stands, none while one may still come, or the refusal saying why
    /// none will.
    fn waited(
        &self,
        place: &PlaceId,
        of: &WaitFor,
        recipient: &str,
    ) -> Result<Option<Answer>, String> {
        let Some(held) = self.document(place) else {
            return Err(format!("no place {place} stands on this host"));
        };
        let Some(held) = self.recorded(vec![held]).pop() else {
            return Ok(None);
        };
        let Some(key) = newest(&held, of)? else {
            let over = !matches!(held.phase, Phase::Live | Phase::Making { .. });
            if over {
                return Err(format!(
                    "the work of the place {place} is over, and {} has no end to answer",
                    what(of)
                ));
            }
            return Ok(None);
        };
        let mut owing = self.owing();
        let Some(end) = held.ends.iter().find(|end| end.of.key() == key) else {
            return Ok(None);
        };
        let mut delivered = false;
        if end.pending_to(&held.place, &owing, recipient) {
            if let Some(delivery) = held.place.delivery_key(&end.of) {
                let now = self.host.now();
                match self
                    .hand_over(&[delivery], recipient, DeliveredBy::Wait, now, true)
                    .pop()
                {
                    Some(Ok(handed)) => delivered = handed,
                    Some(Err(why)) => return Err(why),
                    None => {}
                }
                owing = self.owing();
            }
        }
        Ok(Some(Answer::Waited {
            place: place.clone(),
            recipient: recipient.into(),
            end: Some(EndStatus::of(end, &held.place, &owing)),
            delivered,
        }))
    }

    /// The document a wait reads for the place: of those its name reads,
    /// its live one, else the newest kept for its pending ends once it was
    /// tidied (43, 62).
    fn document(&self, place: &PlaceId) -> Option<HeldState> {
        self.documents_of(place)
            .into_iter()
            .max_by_key(|held| match held.phase {
                Phase::Tidied { at, .. } => at,
                _ => Seconds::MAX,
            })
    }

    /// Collects every end pending for `recipient` of every place on the
    /// host, or, where `place` names one, of the documents status reads
    /// under its name alone, each recorded delivered by the inbox in the
    /// write that reads it, and gives every place of those that waits on
    /// its caller and on `recipient`, marking nothing for it; refused,
    /// marking nothing, where no document stands under the name. An act
    /// on places, so while no caller keeps watch it settles every place
    /// first; a name's documents are read before that settle, so it
    /// collects what the name read and nothing a settle tidied since, a
    /// document it removed giving nothing (43, 62, 119, 130, 193, 208;
    /// design 9).
    pub fn inbox(&self, recipient: &str, place: Option<&PlaceId>) -> Answer {
        self.inbox_by(recipient, place, DeliveredBy::Inbox)
    }

    /// The inbox, each end it gives handed over `by` its way (43; weir
    /// 103).
    fn inbox_by(&self, recipient: &str, place: Option<&PlaceId>, by: DeliveredBy) -> Answer {
        let named: Option<Vec<PlaceId>> = match place {
            Some(id) => {
                let named: Vec<PlaceId> = self
                    .documents_of(id)
                    .into_iter()
                    .map(|held| held.place.id)
                    .collect();
                if named.is_empty() {
                    return Answer::refused(format!("no place {id} stands on this host"));
                }
                Some(named)
            }
            None => None,
        };
        self.settle_attended();
        let documents: Vec<HeldState> = match named {
            Some(ids) => ids
                .iter()
                .filter_map(|id| self.store.read(id))
                .map(|read| read.state)
                .collect(),
            None => self
                .store
                .read_every()
                .into_iter()
                .map(|read| read.state)
                .collect(),
        };
        let documents = self.recorded(documents);
        let owing = self.owing();
        let mut waiting = vec![];
        let mut pending: Vec<(&HeldState, &EndHeld, String)> = vec![];
        for held in &documents {
            waiting.extend(waits_on(held, recipient));
            for end in &held.ends {
                if !end.pending_to(&held.place, &owing, recipient) {
                    continue;
                }
                if let Some(delivery) = held.place.delivery_key(&end.of) {
                    pending.push((held, end, delivery));
                }
            }
        }
        let keys: Vec<String> = pending.iter().map(|(_, _, key)| key.clone()).collect();
        let handed = if keys.is_empty() {
            vec![]
        } else {
            self.hand_over(&keys, recipient, by, self.host.now(), true)
        };
        let owing = if keys.is_empty() { owing } else { self.owing() };
        let mut ends = vec![];
        for ((held, end, _), handed) in pending.into_iter().zip(handed) {
            match handed {
                Ok(true) => {}
                Ok(false) => continue,
                Err(why) => {
                    self.host.log(&format!(
                        "an end of {} was not handed to {recipient} by the inbox: {why}",
                        held.place.id
                    ));
                    continue;
                }
            }
            let place = &held.place;
            ends.push(Collected {
                place: PlaceId::new(place.location.clone()),
                kind: place.kind.name.clone(),
                name: place.name.clone(),
                end: EndStatus::of(end, place, &owing),
            });
        }
        Answer::Inbox {
            recipient: recipient.into(),
            ends,
            waiting,
        }
    }
}

impl Engine {
    /// The inbox run as the calling session the terminal names, never as
    /// the caller's environment's recipient or a role's run's address, so a
    /// person's session never takes a role's ends; none where no terminal
    /// the host binds names one, as under the printed command (43, 190,
    /// C.4; design 9).
    pub fn session_inbox(&self) -> Option<Answer> {
        let session = self.caller_session()?;
        Some(self.inbox_by(&session, None, DeliveredBy::SessionStart))
    }
}

/// The words a session flume did not start is given at its start, as its
/// added context, from the inbox it ran there: each end the inbox
/// collected, and each place that waits on the session with the
/// question, its key, its agent kept open and until when, and the three
/// ways on: answer it with the key, by a retry or, while its agent is kept
/// open, a tell; hold the move with words; or leave it to the answerer
/// behind its stream at its stall. None where the inbox gives nothing (43,
/// 130, 193, 198, C.4; design 9).
pub fn session_words(answer: &Answer) -> Option<String> {
    let Answer::Inbox {
        recipient,
        ends,
        waiting,
    } = answer
    else {
        return None;
    };
    if ends.is_empty() && waiting.is_empty() {
        return None;
    }
    let mut lines = vec![format!(
        "flume, at this session's start: what waits on it, {recipient}, from `flume inbox`."
    )];
    if !ends.is_empty() {
        lines.push(String::new());
        lines.push("Ends delivered to this session now:".into());
        for collected in ends {
            let short = format!("{}--{}", collected.kind, collected.name);
            lines.push(format!(
                "- {short} {} [{}]; read: flume status {short}",
                end_words(&collected.end.end.of),
                collected.end.key
            ));
        }
    }
    for place in waiting {
        lines.push(String::new());
        lines.extend(waiting_words(place));
    }
    Some(lines.join("\n"))
}

/// One waiting place in [`session_words`]: what it waits on and since
/// when, the question by its key, its agent kept open or let go, its hold,
/// and the three ways on (130, 198; design 9).
fn waiting_words(place: &WaitingPlace) -> Vec<String> {
    let short = format!("{}--{}", place.kind, place.name);
    let on = &place.waiting_on;
    let brief = place
        .brief
        .as_ref()
        .map(|brief| format!(" ({brief})"))
        .unwrap_or_default();
    let since = on
        .since
        .map(|since| format!(" since {since}"))
        .unwrap_or_default();
    let mut lines = vec![format!(
        "{short}{brief}, in the stream {}, waits on {}{since}.",
        place.stream,
        on.move_words()
    )];
    match &place.stop_short {
        Some(stop_short) => {
            let needs = match stop_short.needs {
                Some(StopKind::FoundWork) => "found work",
                _ => "needs an answer",
            };
            let asking = stop_short
                .addressee
                .as_ref()
                .map(|addressee| format!(", asking {addressee}"))
                .unwrap_or_default();
            lines.push(format!(
                "Its stop-short {} {needs}{asking}:",
                stop_short.key
            ));
            let words = stop_short.words.as_deref().unwrap_or("(no words)");
            lines.extend(words.lines().map(|line| format!("  {line}")));
        }
        None => {
            if let Some(words) = &on.words {
                lines.push(format!("Its words: {words}"));
            }
        }
    }
    if let Some(until) = on.open_until {
        lines.push(format!(
            "Its agent is kept open for the answer until {until}."
        ));
    } else if let Some(closed) = &on.closed {
        lines.push(format!(
            "Its agent kept open for the answer was let go at {}; a retry starts a fresh agent with the question and the answer.",
            closed.at
        ));
    }
    if let Some(hold) = &place.hold {
        let bound = hold
            .bound
            .map(|bound| format!(" for {bound} seconds"))
            .unwrap_or_default();
        lines.push(format!(
            "The move is on hold since {}{bound}: {}",
            hold.since,
            hold.words.as_deref().unwrap_or("(no words)")
        ));
    }
    lines.push("Ways on:".into());
    let answers = place
        .stop_short
        .as_ref()
        .filter(|_| on.why == Waits::Answer);
    match (answers, on.act, on.stage.as_deref()) {
        (Some(stop_short), _, Some(stage)) => {
            lines.push(format!(
                "- answer it, naming its key: flume retry {short} --stage {stage} --answers {} --words \"<answer>\"",
                stop_short.key
            ));
            if on.open_until.is_some() {
                lines.push(format!(
                    "  or, while its agent is kept open: flume tell {short} --answers {} \"<answer>\"",
                    stop_short.key
                ));
            }
        }
        (_, Act::Done, _) => lines.push(format!("- make the move: flume done {short}")),
        (_, Act::Answer, _) => {
            lines.push("- make the move: answer the dialog in its agent's own pane".into());
        }
        (_, _, Some(stage)) => {
            lines.push(format!(
                "- make the move: flume retry {short} --stage {stage}"
            ));
        }
        (_, _, None) => {
            lines.push(format!("- make the move: flume retry {short} --submission"));
        }
    }
    lines.push(format!(
        "- hold the move with your words, saying who has it: flume hold {short} \"<words>\" --waiting --for <time>"
    ));
    lines.push("- leave it: once it stalls, the answerer behind its stream has it.".into());
    lines
}

/// The place `held` as the inbox gives it waiting on `recipient`: where it
/// waits on its caller, and `recipient` is a recipient of it, was named by
/// the stop-short the move answers, or was given a stall of the move as the
/// answerer behind its stream; none otherwise (43, 130, 193; design 9).
fn waits_on(held: &HeldState, recipient: &str) -> Option<WaitingPlace> {
    let on = &held.waiting.as_ref()?.on;
    let place = &held.place;
    let stop_short = stop_short_waited(held, on);
    let named = stop_short
        .as_ref()
        .and_then(|stop_short| stop_short.addressee.as_deref())
        == Some(recipient);
    // A stall of this move, not of one the place made before it: its
    // since is the move's, or a lapsed hold's after it (130).
    let answered = held.ends.iter().any(|end| {
        matches!(
            &end.of,
            EndOf::Stall { since, answerer: Some(answerer), .. }
                if answerer == recipient && on.since.is_none_or(|from| *since >= from)
        )
    });
    let reached = place.recipients.iter().any(|named| named == recipient);
    if !(reached || named || answered) {
        return None;
    }
    Some(WaitingPlace {
        place: PlaceId::new(place.location.clone()),
        kind: place.kind.name.clone(),
        name: place.name.clone(),
        stream: place.stream.name.clone(),
        brief: place.brief_line().map(str::to_string),
        waiting_on: on.clone(),
        stop_short,
        hold: held.waiting_hold().cloned(),
    })
}

/// The key of the newest end of what a wait waits on: of the stage's
/// current start since the last answer it took in its session, or of the
/// place's current submission; none where it has none yet, refused where
/// the place has no such stage or its kind does not land.
fn newest(held: &HeldState, of: &WaitFor) -> Result<Option<String>, String> {
    let place = &held.place.id;
    let ends = &held.ends;
    let found = match of {
        WaitFor::Stage(name) => {
            let Some(stage) = held.stages.iter().find(|stage| &stage.name == name) else {
                return Err(format!("the place {place} has no stage {name}"));
            };
            // An end of the current start read after the last answer it
            // took in its session: a wait after an answer never answers the
            // stop-short that answer answered (42, 43).
            let (start, answered) = stage
                .start
                .as_ref()
                .map_or((stage.starts, 0), |start| (start.n, start.answered));
            ends.iter().rev().find(|end| {
                matches!(
                    &end.of,
                    EndOf::Stage { stage, start: n, answered: count, .. }
                        if stage == name && *n == start && *count == answered
                )
            })
        }
        WaitFor::Submission => {
            if !held.place.kind.lands {
                return Err(format!(
                    "the kind {} does not land, so the place {place} has no submission",
                    held.place.kind.name
                ));
            }
            let submit = held.submission.submits;
            ends.iter()
                .rev()
                .find(|end| matches!(&end.of, EndOf::Submission { submit: n, .. } if *n == submit))
        }
    };
    Ok(found.map(|end| end.of.key()))
}

/// What a wait waits on, in words.
fn what(of: &WaitFor) -> String {
    match of {
        WaitFor::Stage(stage) => format!("its stage {stage}"),
        WaitFor::Submission => "its submission".into(),
    }
}
