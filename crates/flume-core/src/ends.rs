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
//! core.28). No notice is sent to a place's caller as its caller: the
//! end of the place's work, its landing, drop or done, waits for it until
//! it collects it by the inbox or a wait (43; core.25). A place whose
//! work is over and whose worktree is gone keeps its document alone,
//! apart from a live place's, while an end of it stands pending for a
//! recipient or for its caller, and it goes once every end is delivered
//! or its caller drops it (3, 62).

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
/// the notice is sent, waits on a move of its caller's, which gives the
/// question and how to answer it, and `flume status <place>` otherwise.
/// It names no fact to act on blindly, and no question's words, which
/// are many lines (43, 198; core.11, core.25; design 9).
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
    format!(
        "flume: {short}{brief} {} [{}]; read: {read}",
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
    /// Sends each end of the place pending for a recipient whose session
    /// asks no question, once, through the first terminal the host binds
    /// that holds the session, or, for a recipient the host's standing
    /// entries name, read once, into the one agent of the entry's name in
    /// its stream's terminal and session; claimed before the send and
    /// released where the send was not made, and claimed not at all where
    /// no one agent asking no question is found (43, 75, 132, 190, C.7;
    /// core.11, core.28).
    pub(crate) fn deliver_ends(&self, id: &PlaceId) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        let mut standing = None;
        let mut states: BTreeMap<String, Option<Reached>> = BTreeMap::new();
        for end in &held.ends {
            for recipient in end.pending(&held.place.recipients) {
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
                        if end.delivered_to(&recipient) {
                            return Step::Keep(false);
                        }
                        end.deliveries.push(Delivery {
                            recipient: recipient.clone(),
                            at: now,
                            by: DeliveredBy::Notice,
                        });
                        Step::Write(true)
                    })
                    .unwrap_or(false);
                if !claimed {
                    continue;
                }
                let why = match terminal.send_to(&at, &notice(&held, &end.of)) {
                    ToSession::Sent => continue,
                    ToSession::NotFound => format!("no session is named {at}"),
                    ToSession::Refused { reason } => reason,
                };
                states.insert(recipient.clone(), None);
                self.host.log(&format!(
                    "the notice of {id}'s end {key} was not sent to {recipient}: {why}"
                ));
                self.update(id, |held| {
                    if let Some(end) = held.ends.iter_mut().find(|end| end.of.key() == key) {
                        end.deliveries.retain(|delivery| {
                            !(delivery.recipient == recipient
                                && delivery.at == now
                                && delivery.by == DeliveredBy::Notice)
                        });
                    }
                    Step::Write(())
                });
            }
        }
    }

    /// The document of a place whose worktree and branch are gone, kept
    /// apart from a live place's under its id and the tidy's time while an
    /// end of it stands pending, naming those it waits on, its caller
    /// among them, and
    /// holding no line of what its stages made; none kept where nothing
    /// is pending (43, 62, 129).
    pub(crate) fn keep_for_ends(&self, held: &HeldState, over: &Over) -> Option<PlaceId> {
        let pending = held.pending_for();
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
    /// pending, and otherwise kept naming those it waits on, its caller
    /// among them; true when it changed something (43, 62).
    pub(crate) fn settle_tidied(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let pending = read.state.pending_for();
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
