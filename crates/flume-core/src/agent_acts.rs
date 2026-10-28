//! What an agent's program asks of flume (41, 42, 110, 115): a stop-short,
//! taken only under the current start's identity and secret while its
//! agent is live, and the stop check, which keeps at work an agent that
//! would go quiet with its deliverable missing and nothing said.

use crate::start::Starts;
use crate::{Act, Answer, Engine, Step};
use flume_seams::*;

impl Engine {
    /// The place and stage index whose live current start `identity`
    /// names.
    fn current_start(&self, identity: &str) -> Option<(HeldState, usize)> {
        self.store.read_every().into_iter().find_map(|read| {
            let held = read.state;
            let (index, start) = held.live_start()?;
            (held.current == Some(index) && start.identity.to_string() == identity)
                .then_some((held, index))
        })
    }

    /// An agent's stop-short, with its identity, its start's secret and
    /// its words: the stage ends short with the words whole, and no next
    /// stage starts (41, 42).
    pub fn needs(&self, identity: &str, secret: &str, words: &str) -> Answer {
        self.settle_attended();
        let Some((held, index)) = self.current_start(identity) else {
            return Answer::refused(format!(
                "no live agent of a place's current stage is {identity}"
            ));
        };
        let start = held.stages[index].start.clone().expect("a live start");
        if start.secret.0 != secret {
            return Answer::refused(format!("the secret is not {identity}'s start's"));
        }
        if self.terminal.state(&start.identity) == AgentState::Stopped {
            return Answer::refused(format!("the agent {identity} is not live"));
        }
        let id = held.place.id.clone();
        let taken = self
            .update(&id, |held| {
                let Some(index) = held.index_of(&start.identity) else {
                    return Step::Keep(false);
                };
                let stage = &mut held.stages[index];
                if !stage.state.running() {
                    return Step::Keep(false);
                }
                stage.state = StageState::EndedShort;
                stage.end = Some(StageEnd {
                    head: None,
                    report: None,
                    words: Some(words.to_string()),
                });
                Step::Write(true)
            })
            .unwrap_or(false);
        if !taken {
            return Answer::refused(format!("the stage of {identity} is not running"));
        }
        self.acted(Act::Needs, &id)
    }

    /// The stop check: words naming the deliverable and the way to stop
    /// short for an agent of a stage that is not a conversation, about to
    /// go quiet with its deliverable missing, nothing said, no own work
    /// and no question open; none otherwise (110, 115).
    pub fn stop_check(&self, identity: &str) -> Answer {
        let words = self.stop_words(identity);
        Answer::StopCheck { words }
    }

    fn stop_words(&self, identity: &str) -> Option<String> {
        let (held, index) = self.current_start(identity)?;
        let place = &held.place;
        let declared = &place.kind.stages[index];
        if declared.conversation || !held.stages[index].state.running() {
            return None;
        }
        let start = held.stages[index].start.as_ref()?;
        let agent = self.agent(&start.program)?;
        if agent.own_work(&start.identity) != Reading::No
            || agent.asking(&start.identity) != Reading::No
        {
            return None;
        }
        let bound = self.bound(place).ok()?;
        if self.facts(&held, &bound, Some(index)).delivered {
            return None;
        }
        let change = bound.change.as_ref().map(|change| change.path(&place.name));
        let change = change.as_deref().unwrap_or("the place's change");
        let what = match declared.deliverables.first()? {
            Deliverable::Commits => format!(
                "commits: a commit on {} made since the stage started",
                place.branch
            ),
            Deliverable::ChangeWritten => format!(
                "change written: the change {change}, its planning complete, in a commit made since the stage started"
            ),
            Deliverable::TasksTicked => format!(
                "tasks ticked: every task of the change {change} ticked, in a commit made since the stage started"
            ),
            Deliverable::ChangeArchived => format!(
                "change archived: the change {change} archived on {}, in a commit made since the stage started",
                place.branch
            ),
            Deliverable::Report { path } => format!(
                "report: the file {} written since the stage started",
                self.report_path(place, path)
            ),
            Deliverable::CommittedReport { path } => format!(
                "committed report: the file {path} in a commit made since the stage started"
            ),
            Deliverable::Unreadable { .. } => return None,
        };
        Some(format!(
            "The {} stage's deliverable does not exist yet. It is {what}. Deliver it before you stop, or stop short: {}",
            declared.name,
            agent.stop_short(&start.identity)
        ))
    }
}
