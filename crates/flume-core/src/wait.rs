//! The wait by place and the inbox (43, 119, 122; design 9; core.13,
//! core.25). A wait names a place and its stage or its submission, and
//! the recipient it waits as: it answers at once with the newest end of
//! that stage's current start or that submission pending for the
//! recipient, recording it delivered by the wait; answers one already
//! delivered as a read, delivering nothing again; and otherwise follows
//! the host's state until such an end comes or its timeout passes. Under
//! a terminal that keeps watch it reads the store alone, and the watch
//! moves the place; under one that keeps none it settles every place
//! before each reading after its first, as every acting command does,
//! the first reading answering at once an end that stands pending
//! before a settle could deliver it by notice and tidy its place away. The inbox gives
//! every end pending for a recipient, of every place on the host, a
//! tidied place's kept document among them, and records each delivered
//! to it in the same write that reads it, so two inboxes at once deliver
//! each end once (75). A recipient's default is the caller's
//! environment's, else the role a role's run serves, else the calling
//! session as the terminal names it, as a launch's (7, 132).

use crate::{Answer, Collected, EndStatus, Engine, Step};
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
    /// session as the terminal names it (7, 132; design 7).
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
        self.terminal.session()
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
        let watched = self.terminal.watch() != Watch::None;
        let deadline = timeout.map(|timeout| self.host.now() + timeout);
        loop {
            match self.waited(place, of, recipient) {
                Err(reason) => return Answer::refused(reason),
                Ok(Some(answer)) => return answer,
                Ok(None) => {}
            }
            if !watched {
                self.settle_every();
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
        let Some(read) = self.document(place) else {
            return Err(format!("no place {place} stands on this host"));
        };
        let held = read.state;
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
        let id = held.place.id.clone();
        let pending = held.place.recipients.iter().any(|named| named == recipient)
            && held
                .ends
                .iter()
                .any(|end| end.of.key() == key && !end.delivered_to(recipient));
        let mut delivered = false;
        let mut answered = held;
        if pending {
            let now = self.host.now();
            let Some(written) = self.update(&id, |held| {
                let Some(end) = held.ends.iter_mut().find(|end| end.of.key() == key) else {
                    return Step::Keep(None);
                };
                if end.delivered_to(recipient) {
                    return Step::Keep(Some((held.clone(), false)));
                }
                end.deliveries.push(Delivery {
                    recipient: recipient.into(),
                    at: now,
                    by: DeliveredBy::Wait,
                });
                Step::Write(Some((held.clone(), true)))
            }) else {
                return Ok(None);
            };
            let Some((held, by_this)) = written else {
                return Ok(None);
            };
            answered = held;
            delivered = by_this;
        }
        let Some(end) = answered.ends.iter().find(|end| end.of.key() == key) else {
            return Ok(None);
        };
        Ok(Some(Answer::Waited {
            place: place.clone(),
            recipient: recipient.into(),
            end: Some(EndStatus {
                pending_for: end.pending(&answered.place.recipients),
                end: end.clone(),
            }),
            delivered,
        }))
    }

    /// The document a wait reads for the place: its live one, else the
    /// newest kept for its pending ends once it was tidied (43, 62).
    fn document(&self, place: &PlaceId) -> Option<Versioned> {
        if let Some(read) = self.store.read(place) {
            return Some(read);
        }
        self.tidied_of(place)
            .iter()
            .filter_map(|id| self.store.read(id))
            .max_by_key(|read| match read.state.phase {
                Phase::Tidied { at, .. } => at,
                _ => 0,
            })
    }

    /// Collects every end pending for `recipient` of every place on the
    /// host, each recorded delivered by the inbox in the write that reads
    /// it; an act on places, so under a terminal that keeps no watch it
    /// settles every place first (43, 119; design 9).
    pub fn inbox(&self, recipient: &str) -> Answer {
        self.settle_attended();
        let mut ends = vec![];
        for read in self.store.read_every() {
            let held = read.state;
            if !held.place.recipients.iter().any(|named| named == recipient)
                || held.ends.iter().all(|end| end.delivered_to(recipient))
            {
                continue;
            }
            let now = self.host.now();
            let collected = self
                .update(&held.place.id, |held| {
                    let mut collected = vec![];
                    for end in &mut held.ends {
                        if end.delivered_to(recipient) {
                            continue;
                        }
                        end.deliveries.push(Delivery {
                            recipient: recipient.into(),
                            at: now,
                            by: DeliveredBy::Inbox,
                        });
                        collected.push(EndStatus {
                            pending_for: end.pending(&held.place.recipients),
                            end: end.clone(),
                        });
                    }
                    if collected.is_empty() {
                        Step::Keep(collected)
                    } else {
                        Step::Write(collected)
                    }
                })
                .unwrap_or_default();
            let place = &held.place;
            ends.extend(collected.into_iter().map(|end| Collected {
                place: PlaceId::new(place.location.clone()),
                kind: place.kind.name.clone(),
                name: place.name.clone(),
                end,
            }));
        }
        Answer::Inbox {
            recipient: recipient.into(),
            ends,
        }
    }
}

/// The key of the newest end of what a wait waits on: of the stage's
/// current start, or of the place's current submission; none where it
/// has none yet, refused where the place has no such stage or its kind
/// does not land.
fn newest(held: &HeldState, of: &WaitFor) -> Result<Option<String>, String> {
    let place = &held.place.id;
    let ends = &held.ends;
    let found = match of {
        WaitFor::Stage(name) => {
            let Some(stage) = held.stages.iter().find(|stage| &stage.name == name) else {
                return Err(format!("the place {place} has no stage {name}"));
            };
            let start = stage.start.as_ref().map_or(stage.starts, |start| start.n);
            ends.iter().rev().find(|end| {
                matches!(&end.of, EndOf::Stage { stage, start: n, .. } if stage == name && *n == start)
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
