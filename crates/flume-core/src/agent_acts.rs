//! What an agent's program asks of flume (41, 42, 110, 115, 131): a
//! stop-short, taken only under the current start's identity and secret
//! while its agent is live, saying whether it needs an answer or found
//! work, and the stop check, which keeps at work an agent that
//! would go quiet with its deliverable missing and nothing said.

use crate::prompt::group_text;
use crate::reading::find_group;
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

    /// An agent's stop-short, with its identity, its start's secret, its
    /// words and whether it needs an answer or found work: the stage ends
    /// short with the words whole and the kind, and no next stage starts,
    /// except that found work from the kind's last stage starts its fix
    /// round once the agent is quiet (41, 42, 131).
    pub fn needs(&self, identity: &str, secret: &str, words: &str, kind: StopKind) -> Answer {
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
        let agent = self.agent(&start.program);
        let folder = self.store.folder(&held.place.id);
        if self.state_of(agent.as_deref(), &folder, &start.identity) == AgentState::Stopped {
            return Answer::refused(format!("the agent {identity} is not live"));
        }
        let id = held.place.id.clone();
        let made = self.made_since(&id, &start);
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
                    words: Some(words.to_string()),
                    needs: Some(kind),
                    made: made.clone(),
                    ..StageEnd::default()
                });
                Step::Write(true)
            })
            .unwrap_or(false);
        if !taken {
            return Answer::refused(format!("the stage of {identity} is not running"));
        }
        self.acted(Act::Needs, &id)
    }

    /// The place's own folder, where the agent of the live current start
    /// `identity` names keeps its record, given only with that start's
    /// secret: the folder an agent program's acks and marks are written in
    /// (128, 135).
    pub fn agent_folder(&self, identity: &str, secret: &str) -> Result<String, String> {
        let Some((held, index)) = self.current_start(identity) else {
            return Err(format!(
                "no live agent of a place's current stage is {identity}"
            ));
        };
        let start = held.stages[index].start.as_ref().expect("a live start");
        if start.secret.0 != secret {
            return Err(format!("the secret is not {identity}'s start's"));
        }
        Ok(self.store.folder(&held.place.id))
    }

    /// The stop check: words naming the deliverable, each thing the stage
    /// declares it makes with no line, and the way to stop short, for an
    /// agent of a stage that is not a conversation, about to go quiet
    /// with its deliverable missing or a thing unmade, nothing said, no
    /// own work and no question open; none otherwise (110, 115, 129).
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
        let folder = self.store.folder(&place.id);
        if agent.own_work(&folder, &start.identity) != Reading::No
            || agent.asking(&folder, &start.identity) != Reading::No
        {
            return None;
        }
        let bound = self.bound(place).ok()?;
        let facts = self.facts(&held, &bound, Some(index));
        if facts.delivered {
            return None;
        }
        let stop_short = agent.stop_short(&start.identity);
        let unmade = (!facts.unmade.is_empty()).then(|| {
            format!(
                "It makes {} besides, and {} holds no line naming {} since the stage started: write one JSON line for each thing, its name and what it is, with its address, or the branch and revision that bring it up.",
                facts.unmade.join(", "),
                self.made_path(&place.id),
                if facts.unmade.len() == 1 { "it" } else { "each" },
            )
        });
        if facts.deliverable {
            return Some(format!(
                "The {} stage's deliverable is not yet made. {} Do so before you stop, or stop short: {stop_short}",
                declared.name,
                unmade?,
            ));
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
            Deliverable::TasksTicked => match start.group.as_ref().filter(|_| declared.repeats) {
                Some(group) => group_words(group, facts.change.as_ref(), change),
                None => format!(
                    "tasks ticked: every task of the change {change} ticked, in a commit made since the stage started"
                ),
            },
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
        let unmade = unmade
            .map(|unmade| format!(" {unmade}"))
            .unwrap_or_default();
        Some(format!(
            "The {} stage's deliverable does not exist yet. It is {what}.{unmade} Deliver it before you stop, or stop short: {stop_short}",
            declared.name,
        ))
    }
}

/// What a repetition delivers, in the stop check's words: its group, by
/// its number and title, every task of it ticked in a commit made since
/// it started, and the tasks of it left unticked at the head; or, where
/// the change no longer holds the group, that its heading is to stay as
/// it was (110, 140).
fn group_words(group: &GroupHeld, change: Option<&ChangeRead>, path: &str) -> String {
    let named = group_text(Some(group));
    let named = named.lines().next().unwrap_or_default();
    let facts = match change {
        Some(ChangeRead::Read { facts }) => Some(facts),
        _ => None,
    };
    let left = facts.and_then(|facts| {
        find_group(facts, group).map(|at| {
            facts
                .tasks_of(at)
                .filter(|task| !task.ticked)
                .map(|task| task.text.clone())
                .collect::<Vec<_>>()
        })
    });
    let left = match left {
        Some(left) if left.is_empty() => String::new(),
        Some(left) => format!(". Left unticked: {}", left.join("; ")),
        None if facts.is_some() => format!(
            ". The change no longer holds {named}: leave its heading as it was, and tick its tasks: {}",
            group.tasks.join("; ")
        ),
        None => String::new(),
    };
    format!(
        "tasks ticked: every task of your task group, {named}, of the change {path} ticked, in a commit made since the stage started{left}"
    )
}
