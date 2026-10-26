//! Status of one place or every place (56 to 62): every fact of a
//! deliverable, a change or a submission read where it lives when asked,
//! at one head per place; nothing written, sent or started (57, 58).

use crate::reading::reads_change;
use crate::start::Starts;
use crate::{AgentStatus, Answer, Engine, PlaceStatus, StageStatus, SubmissionStatus};
use flume_seams::*;

impl Engine {
    /// Status of the place, or of every place on the host.
    pub fn status(&self, place: Option<&PlaceId>) -> Answer {
        let held: Vec<HeldState> = match place {
            Some(id) => match self.store.read(id) {
                Some(read) => vec![read.state],
                None => return Answer::refused(format!("no place {id} stands on this host")),
            },
            None => self
                .store
                .read_every()
                .into_iter()
                .map(|read| read.state)
                .collect(),
        };
        Answer::Status {
            places: held.iter().map(|held| self.place_status(held)).collect(),
        }
    }

    /// Every fact of one place, as it stands when asked.
    pub(crate) fn place_status(&self, held: &HeldState) -> PlaceStatus {
        let place = &held.place;
        let mut cannot_read = vec![];
        let mut refused_acts = vec![];
        let current = held.current;
        let read_facts = !matches!(held.phase, Phase::Gone { .. } | Phase::Making { .. });
        let bound = match self.bound(place) {
            Ok(bound) => Some(bound),
            Err(reason) => {
                cannot_read.push(reason);
                None
            }
        };
        let with_start = current.filter(|index| {
            held.stages[*index].start.is_some() && !place.kind.stages[*index].conversation
        });
        let facts = bound
            .as_ref()
            .filter(|_| read_facts)
            .map(|bound| self.facts(held, bound, with_start));
        let tasks = match facts.as_ref().and_then(|facts| facts.change.as_ref()) {
            Some(ChangeRead::Read { facts }) => Some(facts.tasks.clone()),
            Some(ChangeRead::CouldNotRead { reason }) => {
                cannot_read.push(format!("the change: {reason}"));
                None
            }
            None => None,
        };
        let agent = held.live_start().map(|(index, start)| {
            let reading = self.read_agent(start);
            cannot_read.extend(reading.unread());
            if let Some(agent) = self.agent(&start.program) {
                if let Guarded::Refused { acts } = agent.guard(&start.identity) {
                    refused_acts.extend(acts);
                }
            }
            AgentStatus {
                identity: start.identity.to_string(),
                stage: place.kind.stages[index].name.clone(),
                program: start.program.clone(),
                settings: start.settings.clone(),
                state: reading.state,
                asking: reading.asking,
                own_work: reading.own_work,
                past_limit: start.past_limit,
            }
        });
        let submission = match (&bound, place.kind.lands) {
            (Some(bound), true) if read_facts => bound.merge_queue.as_ref().map(|queue| {
                let read = queue.read_submission(&place.repository, &place.branch);
                if let SubmissionRead::CouldNotRead { reason } = &read {
                    cannot_read.push(format!("the merge queue: {reason}"));
                }
                SubmissionStatus {
                    read,
                    submits: held.submission.submits,
                    send_backs: held.submission.send_backs,
                    verdict: held.submission.verdict.clone(),
                    waiting: held.submission.waiting.clone(),
                    held: place.hold_submission,
                }
            }),
            _ => None,
        };
        if reads_change(&place.kind) && bound.as_ref().is_some_and(|bound| bound.change.is_none()) {
            cannot_read.push("the change: no change profile is bound".into());
        }
        PlaceStatus {
            place: place.id.clone(),
            repository: place.repository.clone(),
            kind: place.kind.name.clone(),
            version: place.kind.version,
            declared: place.declared,
            name: place.name.clone(),
            branch: place.branch.clone(),
            base: place.base.clone(),
            mark: place.mark.clone(),
            lands: place.kind.lands,
            phase: held.phase.clone(),
            readiness: held.readiness.clone(),
            stages: held
                .stages
                .iter()
                .zip(&place.kind.stages)
                .map(|(stage, declared)| StageStatus {
                    name: stage.name.clone(),
                    state: stage.state,
                    starts: stage.starts,
                    end: stage.end.clone(),
                    refused_part: stage.refused_part.clone(),
                    conversation: declared.conversation,
                })
                .collect(),
            current: current.map(|index| held.stages[index].name.clone()),
            agent,
            head: facts.as_ref().map(|facts| facts.head.clone()),
            delivered: with_start.and(facts.as_ref().map(|facts| facts.delivered)),
            tasks,
            submission,
            refused_acts,
            cannot_read,
            failures: held.failures.clone(),
        }
    }
}
