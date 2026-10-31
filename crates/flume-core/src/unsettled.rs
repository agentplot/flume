//! A start whose agent began without its settings (35, 135, 161;
//! core.10): where the agent program's own word at its start, or at its
//! start's first prompt for a mode its program names only with a prompt,
//! shows it began otherwise than its start recorded, the start is
//! recorded so on the place by its variables' names alone and its mode,
//! its agent is sent no prompt, or none again, and once quiet it is ended
//! and its stage started afresh under the settings resolved then, counted
//! toward no bound of the kind's and waiting on no caller (31, 112). A
//! fresh start whose agent begins so again is never begun, its end naming
//! the same.

use crate::start::{StartReason, Starts};
use crate::{Engine, Step};
use flume_seams::*;

impl Engine {
    /// The starting start `start` of the stage at `index` read begun as
    /// `unsettled` says: recorded on the place once; once its agent is
    /// quiet, the agent ended and, where the stage's start before it
    /// began so too, the start never begun with its end naming how,
    /// else the stage started afresh with [`StartReason::Unsettled`]'s
    /// words; true when the stage moved (31, 35, 37, 43, 112, 161).
    pub(crate) fn read_unsettled(
        &self,
        id: &PlaceId,
        index: usize,
        start: &StartHeld,
        unsettled: Unsettled,
    ) -> bool {
        let now = self.host.now();
        let Some(written) = self.update(id, |held| {
            let stage = held.stages[index].name.clone();
            let again = held
                .unsettled
                .iter()
                .any(|read| read.stage == stage && read.start + 1 == start.n);
            if held
                .unsettled
                .iter()
                .any(|read| read.stage == stage && read.start == start.n)
            {
                return Step::Keep((held.clone(), again));
            }
            held.unsettled.push(UnsettledStart {
                stage,
                start: start.n,
                at: now,
                unsettled: unsettled.clone(),
            });
            Step::Write((held.clone(), again))
        }) else {
            return false;
        };
        let (held, again) = written;
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
        if again {
            let mut end = self.read_out(&held, start, true);
            end.unsettled = Some(Box::new(unsettled));
            self.end_start(
                id,
                identity,
                |start, state| {
                    start.live = false;
                    *state = StageState::NeverBegun;
                },
                Some(end),
            );
            return true;
        }
        let words = StartReason::Unsettled.words();
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
                }
                stage.state = StageState::WaitingForRoom;
                stage.end = None;
                stage.room = Some(RoomWait {
                    order,
                    words: Some(words.clone()),
                });
                Step::Write(())
            });
        }
        self.start_stage(id, index, Some(words));
        true
    }
}
