//! A carried start that does not begin (146, 161, 179; S79): where a
//! start that carries the conversation of its stage's agent stopped at
//! its account's usage limit would be read never begun, at its agent's
//! stop, at the stage's limit or on a prompt it did not receive, it is
//! recorded so on the place with what happened, its agent is sent nothing
//! more, and once quiet it is ended and its stage started afresh with
//! flume's words after the start that stopped, carrying nothing. No end
//! is delivered of it, no caller is waited on and no bound of the kind's
//! counted (31, 35, 112).

use crate::start::{StartReason, Starts};
use crate::{Engine, Step};
use flume_seams::*;

/// What happened to a carried start whose agent stopped before it said it
/// started (146, S79).
pub(crate) const ENDED_FIRST: &str =
    "the start that carried its conversation ended before it said it started";

/// What happened to a carried start whose agent did not say it started by
/// the stage's limit (35, 146).
pub(crate) const NOT_BY_THE_LIMIT: &str =
    "the start that carried its conversation did not say it started by the stage's limit";

/// What happened to a carried start whose agent did not receive its
/// prompt (38, 146).
pub(crate) const NOT_RECEIVED: &str =
    "the start that carried its conversation did not receive its prompt";

/// Whether `start` carries its stage's conversation (179).
pub(crate) fn carries(start: &StartHeld) -> bool {
    start
        .conversation
        .as_ref()
        .is_some_and(|conversation| conversation.carried)
}

/// The words a carried start that did not begin was recorded with, none
/// where it was not (146).
pub(crate) fn uncarried<'a>(held: &'a HeldState, start: &StartHeld) -> Option<&'a str> {
    held.uncarried
        .iter()
        .find(|read| read.stage == start.identity.stage && read.start == start.n)
        .map(|read| read.words.as_str())
}

/// The wait of a stage whose carried start `start` did not begin: after
/// the start whose agent stopped at its account's usage limit, carrying
/// nothing, with `why` (146, 179).
pub(crate) fn after_uncarried(start: &StartHeld, why: String) -> Option<AfterSpent> {
    let from = start.conversation.as_ref()?.from;
    Some(AfterSpent {
        start: Identity {
            start: from,
            ..start.identity.clone()
        },
        carry: false,
        why: Some(why),
    })
}

impl Engine {
    /// The carried start `start` of the stage at `index` read as not
    /// begun, as `words` say: recorded on the place once; once its agent
    /// is quiet, at once where it stopped, the agent ended and the stage
    /// started afresh with [`StartReason::Spent`]'s words after the start
    /// whose agent stopped at its account's usage limit, carrying nothing;
    /// true when the stage moved (31, 35, 112, 146, 179).
    pub(crate) fn read_uncarried(
        &self,
        id: &PlaceId,
        index: usize,
        start: &StartHeld,
        words: &str,
    ) -> bool {
        let now = self.host.now();
        let Some(words) = self.update(id, |held| {
            if let Some(read) = uncarried(held, start) {
                return Step::Keep(read.to_string());
            }
            held.uncarried.push(UncarriedStart {
                stage: held.stages[index].name.clone(),
                start: start.n,
                at: now,
                words: words.to_string(),
            });
            Step::Write(words.to_string())
        }) else {
            return false;
        };
        let reading = self.read_agent(id, start);
        if !reading.quiet() {
            return false;
        }
        let identity = &start.identity;
        if reading.state != AgentState::Stopped {
            if let Closed::Refused { reason } = self.terminal.close(identity) {
                self.failed_end(id, identity, &reason);
                return false;
            }
        }
        let again = StartReason::Spent.words();
        let carry = after_uncarried(start, words);
        {
            let _room = self.host.room();
            let order = self.queue().next_order();
            self.update(id, |held| {
                let Some(at) = held.index_of(identity) else {
                    return Step::Keep(());
                };
                let stage = &mut held.stages[at];
                if let Some(start) = stage.start.as_mut() {
                    start.live = false;
                    start.first_quiet = None;
                    start.past_limit = false;
                }
                stage.state = StageState::WaitingForRoom;
                stage.end = None;
                stage.room = Some(RoomWait {
                    order,
                    words: Some(again.clone()),
                    carry: carry.clone(),
                });
                Step::Write(())
            });
        }
        self.start_stage(id, index, Some(again));
        true
    }
}
