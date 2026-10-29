//! Delivering a place's ends (43, 95, core.11, core.25): each end the
//! held state records is sent once to each recipient the launch named, as
//! one line of notice through the terminal into the recipient's own
//! session, only while the terminal reads that session asking no
//! question, and recorded delivered on the terminal's answer sent, with
//! no reading back. The delivery is claimed in held state before the
//! send, as a prompt's sending is, so two settles at once send one
//! notice (75); a recipient asking, unread or not found, or a send
//! refused, leaves the end pending for a later settle, a wait or the
//! inbox. A place whose work is over and whose worktree is gone keeps
//! its document alone, apart from a live place's, while an end of it
//! stands pending, and it goes once every end is delivered or its caller
//! drops it (3, 62).

use crate::prompt::group_name;
use crate::{Engine, Step};
use flume_seams::*;
use std::collections::BTreeMap;

/// The one line a notice is: the place, what ended, and the command that
/// reads it whole (43, design 2).
pub fn notice(place: &PlaceRecord, of: &EndOf) -> String {
    let short = place.short();
    format!(
        "flume: {short} {}; read: flume status {short}",
        what_ended(of)
    )
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
                StageState::Lost => format!("{stage} lost"),
                StageState::NeverBegun => format!("{stage} never begun"),
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
    }
}

impl Engine {
    /// Sends each end of the place pending for a recipient whose session
    /// asks no question, once, claimed before the send and released where
    /// the send was not made (43, 75, core.11).
    pub(crate) fn deliver_ends(&self, id: &PlaceId) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        let mut states: BTreeMap<String, Option<AgentState>> = BTreeMap::new();
        for end in &held.ends {
            for recipient in end.pending(&held.place.recipients) {
                let state = *states
                    .entry(recipient.clone())
                    .or_insert_with(|| self.terminal.session_state(&recipient));
                if !matches!(state, Some(AgentState::Working | AgentState::Waiting)) {
                    continue;
                }
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
                let why = match self
                    .terminal
                    .send_to(&recipient, &notice(&held.place, &end.of))
                {
                    ToSession::Sent => continue,
                    ToSession::NotFound => format!("no session is named {recipient}"),
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
    /// end of it stands pending, naming the recipients it waits on; none
    /// kept where nothing is pending (43, 62).
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
        for stage in &mut kept.stages {
            if let Some(start) = stage.start.as_mut() {
                start.live = false;
            }
        }
        match self.store.write(&id, None, Some(&kept)) {
            Written::Written { .. } => Some(id),
            Written::Moved => self.store.read(&id).map(|_| id),
        }
    }

    /// A tidied place's document: removed once no end of it stands
    /// pending, and otherwise kept naming the recipients it waits on;
    /// true when it changed something (43, 62).
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
