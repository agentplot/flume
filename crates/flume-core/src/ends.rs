//! Delivering a place's ends (43, 95, core.11, core.25): each end the
//! held state records is sent once to each recipient the launch named, as
//! one line of notice through the terminal into the recipient's own
//! session, only while the terminal reads that session asking no
//! question, and recorded delivered on the terminal's answer sent, with
//! no reading back. The delivery is claimed in held state before the
//! send, as a prompt's sending is, so two settles at once send one
//! notice (75); a recipient asking, unread or not found, or a send
//! refused, leaves the end pending for a later settle, a wait or the
//! inbox. A recipient the host's configuration gives a standing agent is
//! reached in that agent alone, found by its name in its stream's
//! terminal and session at each delivery, none or more than one of the
//! name leaving the end pending with nothing claimed (132, 190, C.7;
//! core.28). A recipient no pane holds is sent no notice, its ends
//! pending until it collects them by the inbox or a wait; and no notice
//! carries a drop's or a done's end, which stands pending for the
//! place's recipients that are no role's address until each collects it
//! (43, 52, 114; core.25). A place whose work is over and whose worktree
//! is gone keeps its document alone, apart from a live place's, while an
//! end of it stands pending, and it goes once every end is delivered or
//! its caller drops it (3, 62).

use crate::prompt::group_name;
use crate::standing::Reached;
use crate::{Engine, Step};
use flume_seams::*;
use std::collections::BTreeMap;

/// How much of a brief's first line a notice gives (43; design 9).
pub const NOTICE_BRIEF: usize = 60;

/// The one line a notice is: the place, its brief's first line cut at
/// [`NOTICE_BRIEF`] characters, what ended, until when a stop-short's
/// agent is kept open for its answer, the end's key, and the command that
/// reads it whole: `flume inbox` where the place, as `held` gives it when
/// the end's delivery is recorded, waits on a move of its caller's, which
/// gives the question and how to answer it, and `flume status <place>`
/// otherwise; and `flume confirm` with the delivery's key. It is the
/// delivery's message, fixed when the delivery is recorded and sent as
/// recorded. It names no fact to act on blindly, and no question's words,
/// which are many lines (43, 198; weir 100; core.11, core.25; design 4,
/// 9).
pub fn notice(held: &HeldState, of: &EndOf) -> String {
    let place = &held.place;
    let short = place.short();
    let brief = place
        .brief_line()
        .map(|line| format!(" ({})", cut(line, NOTICE_BRIEF)))
        .unwrap_or_default();
    let read = if held.waiting.is_some() {
        "flume inbox".to_string()
    } else {
        format!("flume status {short}")
    };
    let confirm = place
        .delivery_key(of)
        .map(|key| format!("; confirm: flume confirm {key}"))
        .unwrap_or_default();
    format!(
        "flume: {short}{brief} {} [{}]; read: {read}{confirm}",
        end_words(of),
        of.end_key()
    )
}

/// `line` cut at `at` characters, an ellipsis saying where it was cut.
fn cut(line: &str, at: usize) -> String {
    let mut chars = line.chars();
    let head: String = chars.by_ref().take(at).collect();
    if chars.next().is_none() {
        head
    } else {
        format!("{}…", head.trim_end())
    }
}

/// What ended, in the words a notice and the inbox give it: what ended,
/// and for a stop-short whose agent was kept open for its answer when it
/// was read, until when (43, 198).
pub fn end_words(of: &EndOf) -> String {
    let what = what_ended(of);
    match of {
        EndOf::Stage {
            state: StageState::EndedShort,
            end,
            ..
        } => match end.open_until {
            Some(until) => format!("{what}, its agent kept open for the answer until {until}"),
            None => what,
        },
        _ => what,
    }
}

/// What ended, in the words a notice gives it, a repetition's end naming
/// its task group (43, 142).
pub fn what_ended(of: &EndOf) -> String {
    match of {
        EndOf::Stage {
            stage, state, end, ..
        } => {
            let stage = match &end.group {
                Some(group) => format!(
                    "{stage} on {}",
                    group_name(group.number.as_deref(), group.title.as_deref())
                ),
                None => stage.clone(),
            };
            match state {
                StageState::Delivered => format!("{stage} delivered"),
                StageState::EndedShort if end.needs == Some(StopKind::FoundWork) => {
                    format!("{stage} found work")
                }
                StageState::EndedShort => format!("{stage} needs an answer"),
                StageState::Finished => format!("{stage} finished"),
                StageState::Lost if end.left_in_pane => format!(
                    "{stage} lost, and flume left the agent the terminal brought back in its pane"
                ),
                StageState::Lost => format!("{stage} lost"),
                StageState::NeverBegun => match &end.unsettled {
                    Some(unsettled) => {
                        format!("{stage} never begun, its agent began {}", unsettled.words())
                    }
                    None => format!("{stage} never begun"),
                },
                StageState::Stuck => format!("{stage} stuck"),
                other => format!("{stage} {other:?}"),
            }
        }
        EndOf::Submission { verdict, .. } => match verdict {
            Verdict::Landed { commit, .. } => format!("submission landed {commit}"),
            Verdict::Failed { .. } | Verdict::Conflict { .. } => "submission failed".into(),
            Verdict::Withdrawn => "submission withdrawn".into(),
            Verdict::Refused { .. } => "submission refused".into(),
        },
        EndOf::Stall { waiting, .. } => format!("stalled: {waiting}"),
        EndOf::Over { over } => over.word().into(),
    }
}

impl Engine {
    /// Sends each end of the place a notice carries, pending for a
    /// recipient whose session asks no question and to which nothing has
    /// handed it, once, as the message its delivery was recorded with,
    /// through the first terminal the host binds that holds the session,
    /// or, for a recipient the host's standing entries name, read once,
    /// into the one agent of the entry's name in its stream's terminal and
    /// session. The notice is claimed on the place before the send, so two
    /// settles at once send one; on the terminal's answer sent its
    /// hand-over is recorded through weir's `hand`, by `notice`, and the
    /// claim kept; a send not made takes the claim back; and a claim whose
    /// hand-over the host's deliveries lack is recorded at the next settle
    /// at the claim's time. No one agent asking no question found claims
    /// nothing (43, 75, 132, 190, C.7; weir 103; core.11, core.28; design
    /// 5).
    pub(crate) fn deliver_ends(&self, id: &PlaceId) {
        let Some(held) = self.with_token(id) else {
            return;
        };
        self.record_deliveries(std::slice::from_ref(&held));
        let mut owing = self.owing();
        if self.record_claims(&held, &owing) {
            owing = self.owing();
        }
        let mut standing = None;
        let mut states: BTreeMap<String, Option<Reached>> = BTreeMap::new();
        for end in held.ends.iter().filter(|end| end.noticed()) {
            let Some(delivery) = end.delivery(&held.place, &owing) else {
                continue;
            };
            for recipient in end.pending(&held.place, &owing) {
                if end.handed_to(&held.place, &owing, &recipient) {
                    continue;
                }
                let Some(Reached {
                    terminal,
                    state: AgentState::Working | AgentState::Waiting,
                    at,
                }) = states
                    .entry(recipient.clone())
                    .or_insert_with(|| {
                        let standing = standing.get_or_insert_with(|| self.standing());
                        self.reached(standing, &recipient)
                    })
                    .clone()
                else {
                    continue;
                };
                let key = end.of.key();
                let now = self.host.now();
                let claimed = self
                    .update(id, |held| {
                        let Some(end) = held.ends.iter_mut().find(|end| end.of.key() == key) else {
                            return Step::Keep(false);
                        };
                        if end
                            .noticing
                            .iter()
                            .any(|claim| claim.recipient == recipient)
                        {
                            return Step::Keep(false);
                        }
                        end.noticing.push(NoticeClaim {
                            recipient: recipient.clone(),
                            at: now,
                        });
                        Step::Write(true)
                    })
                    .unwrap_or(false);
                if !claimed {
                    continue;
                }
                let why = match terminal.send_to(&at, &delivery.message) {
                    ToSession::Sent => {
                        let handed = self.hand_over(
                            std::slice::from_ref(&delivery.key),
                            &recipient,
                            DeliveredBy::Notice,
                            now,
                            false,
                        );
                        for refused in handed.into_iter().filter_map(Result::err) {
                            self.host.log(&format!(
                                "the notice of {id}'s end {key} to {recipient} was sent, and its hand-over not recorded: {refused}"
                            ));
                        }
                        continue;
                    }
                    ToSession::NotFound => format!("no session is named {at}"),
                    ToSession::Refused { reason } => reason,
                };
                states.insert(recipient.clone(), None);
                self.host.log(&format!(
                    "the notice of {id}'s end {key} was not sent to {recipient}: {why}"
                ));
                self.update(id, |held| {
                    if let Some(end) = held.ends.iter_mut().find(|end| end.of.key() == key) {
                        end.noticing
                            .retain(|claim| !(claim.recipient == recipient && claim.at == now));
                    }
                    Step::Write(())
                });
            }
        }
    }

    /// Records, at its time, the hand-over of each notice claimed on the
    /// place whose hand-over the host's deliveries lack, its send answered
    /// and its record never written, so it is sent no second time; true
    /// where it recorded any (43, 75; weir 103; design 5).
    fn record_claims(&self, held: &HeldState, owing: &Owing) -> bool {
        let mut recorded = false;
        for end in &held.ends {
            let Some(delivery) = end.delivery(&held.place, owing) else {
                continue;
            };
            for claim in &end.noticing {
                let handed = delivery
                    .handovers
                    .iter()
                    .any(|handover| handover.recipient == claim.recipient);
                let owed = delivery
                    .owed
                    .iter()
                    .any(|owed| owed.recipient == claim.recipient);
                if handed || !owed {
                    continue;
                }
                let answers = self.hand_over(
                    std::slice::from_ref(&delivery.key),
                    &claim.recipient,
                    DeliveredBy::Notice,
                    claim.at,
                    false,
                );
                for answer in answers {
                    match answer {
                        Ok(_) => recorded = true,
                        Err(why) => self.host.log(&format!(
                            "the notice claimed of {}'s end to {} was not recorded: {why}",
                            held.place.id, claim.recipient
                        )),
                    }
                }
            }
        }
        recorded
    }

    /// The document of a place whose worktree and branch are gone, kept
    /// apart from a live place's under its id and the tidy's time while an
    /// end of it stands pending, naming the recipients it waits on, and
    /// holding no line of what its stages made; none kept where nothing
    /// is pending (43, 62, 129).
    pub(crate) fn keep_for_ends(&self, held: &HeldState, over: &Over) -> Option<PlaceId> {
        let pending = held.pending_for(&self.owing());
        if pending.is_empty() {
            return None;
        }
        let at = match &held.phase {
            Phase::Gone { at: Some(at), .. } => *at,
            _ => self.host.now(),
        };
        let id = PlaceId::new(format!("{}@{at}", held.place.location));
        let mut kept = held.clone();
        kept.place.id = id.clone();
        kept.phase = Phase::Tidied {
            over: over.clone(),
            at,
            pending_for: pending,
        };
        // The made lines go with the place's folder, so the document kept
        // for its ends holds none of them (62, 129; core.23).
        for stage in &mut kept.stages {
            if let Some(start) = stage.start.as_mut() {
                start.live = false;
            }
            if let Some(end) = stage.end.as_mut() {
                end.made.clear();
            }
        }
        match self.store.write(&id, None, Some(&kept)) {
            Written::Written { .. } => Some(id),
            Written::Moved => self.store.read(&id).map(|_| id),
        }
    }

    /// A tidied place's document: removed once no end of it stands
    /// pending, and otherwise kept naming the recipients it waits on; true
    /// when it changed something (43, 62).
    pub(crate) fn settle_tidied(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let pending = read.state.pending_for(&self.owing());
        if pending.is_empty() {
            return self.remove_held(id);
        }
        self.update(id, |held| match &mut held.phase {
            Phase::Tidied { pending_for, .. } if *pending_for != pending => {
                *pending_for = pending.clone();
                Step::Write(true)
            }
            _ => Step::Keep(false),
        })
        .unwrap_or(false)
    }

    /// The documents a place's name reads, as status and the inbox read
    /// them: its live document, else those kept for it once it was
    /// tidied, `<location>@<at>` naming one; none where no document
    /// stands under the name (43, 62; core.14).
    pub(crate) fn documents_of(&self, id: &PlaceId) -> Vec<HeldState> {
        if let Some(read) = self.store.read(id) {
            return vec![read.state];
        }
        self.tidied_of(id)
            .iter()
            .filter_map(|id| self.store.read(id))
            .map(|read| read.state)
            .collect()
    }

    /// The tidied documents of the place `id` names: the one under that
    /// id, and those kept for the place at that location (43, 62).
    pub(crate) fn tidied_of(&self, id: &PlaceId) -> Vec<PlaceId> {
        self.store
            .read_every()
            .into_iter()
            .filter(|read| matches!(read.state.phase, Phase::Tidied { .. }))
            .filter(|read| read.state.place.id == *id || read.state.place.location == id.0)
            .map(|read| read.state.place.id)
            .collect()
    }
}
