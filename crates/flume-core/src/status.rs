//! Status of one place or every place (56 to 62): every fact of a
//! deliverable, a change or a submission read where it lives when asked,
//! at one head per place; nothing written, sent or started (57, 58).

use crate::reading::{find_group, reads_change};
use crate::start::Starts;
use crate::{
    Act, AgentSettings, AgentStatus, Answer, EndStatus, Engine, GroupStatus, PlaceStatus,
    StageStatus, SubmissionStatus, WaitingOn, Waits,
};
use flume_seams::*;

impl Engine {
    /// Status of the place, or of every place on the host; a place tidied
    /// whose document stays for its pending ends among them (43, 62).
    pub fn status(&self, place: Option<&PlaceId>) -> Answer {
        let held: Vec<HeldState> = match place {
            Some(id) => match self.store.read(id) {
                Some(read) => vec![read.state],
                None => {
                    let tidied: Vec<HeldState> = self
                        .tidied_of(id)
                        .iter()
                        .filter_map(|id| self.store.read(id))
                        .map(|read| read.state)
                        .collect();
                    if tidied.is_empty() {
                        return Answer::refused(format!("no place {id} stands on this host"));
                    }
                    tidied
                }
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
            host: self.host_status(),
        }
    }

    /// Every fact of one place, as it stands when asked.
    pub(crate) fn place_status(&self, held: &HeldState) -> PlaceStatus {
        let place = &held.place;
        let mut cannot_read = vec![];
        let mut refused_acts = vec![];
        let current = held.current;
        let read_facts = !matches!(
            held.phase,
            Phase::Gone { .. } | Phase::Making { .. } | Phase::Tidied { .. }
        );
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
        let change = match facts.as_ref().and_then(|facts| facts.change.as_ref()) {
            Some(ChangeRead::Read { facts }) => Some(facts),
            _ => None,
        };
        let tasks = match facts.as_ref().and_then(|facts| facts.change.as_ref()) {
            Some(ChangeRead::Read { facts }) => Some(facts.tasks.clone()),
            Some(ChangeRead::CouldNotRead { reason }) => {
                cannot_read.push(format!("the change: {reason}"));
                None
            }
            None => None,
        };
        let agent = held.live_start().map(|(index, start)| {
            let reading = self.read_agent(&place.id, start);
            cannot_read.extend(reading.unread());
            let folder = self.store.folder(&place.id);
            let mut last_words = None;
            let mut notes = vec![];
            if let Some(agent) = self.agent(&start.program) {
                if let Guarded::Refused { acts } = agent.guard(&folder, &start.identity) {
                    refused_acts.extend(acts);
                }
                last_words = agent.last_words(&folder, &start.identity);
                notes = agent.notes(&folder, &start.identity);
            }
            let starting = held.stages[index].state == StageState::Starting;
            AgentStatus {
                identity: start.identity.to_string(),
                stage: place.kind.stages[index].name.clone(),
                program: start.program.clone(),
                settings: AgentSettings {
                    settings: start.settings.clone(),
                    env: start
                        .from
                        .keys()
                        .filter_map(|key| key.strip_prefix("env."))
                        .map(|name| (name.to_string(), flume_seams::SET.to_string()))
                        .collect(),
                    from: flume_seams::unvalued_from(&start.from),
                },
                state: reading.state,
                started: reading.started,
                asking: reading.asking,
                own_work: reading.own_work,
                past_limit: start.past_limit,
                screen: starting
                    .then(|| self.terminal.screen(&start.identity))
                    .flatten(),
                printed: start.printed.clone(),
                last_words,
                notes,
            }
        });
        let agent_state = agent.as_ref().map(|agent| agent.state);
        let submission = match (&bound, place.kind.lands) {
            (Some(bound), true) if read_facts => bound.merge_queue.as_ref().map(|queue| {
                let read = crate::submit::own_read(
                    held,
                    queue.read_submission(&place.repository, &place.branch),
                );
                if let SubmissionRead::CouldNotRead { reason } = &read {
                    cannot_read.push(format!("the merge queue: {reason}"));
                }
                if let Some(reason) = &held.submission.unread {
                    cannot_read.push(format!("the merge queue's inbox: {reason}"));
                }
                SubmissionStatus {
                    read,
                    submits: held.submission.submits,
                    send_backs: held.submission.send_backs,
                    verdict: held.submission.verdict.clone(),
                    waiting: held.submission.waiting.clone(),
                    held: held.submission_hold().is_some(),
                }
            }),
            _ => None,
        };
        if reads_change(&place.kind) && bound.as_ref().is_some_and(|bound| bound.change.is_none()) {
            cannot_read.push("the change: no change profile is bound".into());
        }
        let stages: Vec<StageStatus> = held
            .stages
            .iter()
            .zip(&place.kind.stages)
            .enumerate()
            .map(|(index, (stage, declared))| StageStatus {
                name: stage.name.clone(),
                state: stage.state,
                starts: stage.starts,
                end: stage.end.clone(),
                refused_part: stage.refused_part.clone(),
                refused_setting: stage.refused_setting.clone(),
                conversation: declared.conversation,
                group: change.and_then(|change| group_status(held, index, change)),
            })
            .collect();
        let group = current.and_then(|index| stages[index].group.clone());
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
            stages,
            current: current.map(|index| held.stages[index].name.clone()),
            agent,
            head: facts.as_ref().map(|facts| facts.head.clone()),
            delivered: with_start.and(facts.as_ref().map(|facts| facts.delivered)),
            tasks,
            submission,
            refused_acts,
            cannot_read,
            failures: held.failures.clone(),
            recipients: place.recipients.clone(),
            hold_words: place.hold_words.clone(),
            waiting_on: held.waiting.as_ref().map(|waiting| waiting.on.clone()),
            stalled: held.waiting.as_ref().is_some_and(|waiting| waiting.stalled),
            holds: held.holds.clone(),
            priority: held.priority,
            stream: place.stream.clone(),
            intent: place.intent.clone(),
            ends: held
                .ends
                .iter()
                .map(|end| EndStatus {
                    end: end.clone(),
                    pending_for: end.pending(&place.recipients),
                })
                .collect(),
            role: held.role.clone(),
            clues: self.place_clues(held, agent_state, group.as_ref()),
        }
    }
}

impl Engine {
    /// The task group of the place's current stage where it repeats, as
    /// status gives it, read from the change at the branch's head (58,
    /// 139, 142).
    pub(crate) fn current_group(&self, held: &HeldState) -> Option<GroupStatus> {
        let index = held.current?;
        let place = &held.place;
        if !place.kind.stages[index].repeats {
            return None;
        }
        let change = self.bound(place).ok()?.change?;
        let head = self
            .history
            .head(&place.repository, &place.branch)
            .unwrap_or_else(|| place.base_revision.clone());
        match change.read_change(&place.repository, &head, &place.name) {
            ChangeRead::Read { facts } => group_status(held, index, &facts),
            ChangeRead::CouldNotRead { .. } => None,
        }
    }
}

/// The task group status gives the stage at `index` where it repeats,
/// read from `facts`, the change at the head: the group its repetition
/// works on, or ended short, stuck, lost, never begun or stopped on,
/// found there by its number and title; or, for a stage on hold, waiting
/// for room, refused or between repetitions, the group its next
/// repetition would take, the first holding a task unticked. None for a
/// stage that does not repeat, has not reached its repetitions or has
/// delivered, and for a repetition given no group (56, 58, 142;
/// core.15).
pub(crate) fn group_status(
    held: &HeldState,
    index: usize,
    facts: &ChangeFacts,
) -> Option<GroupStatus> {
    if !held.place.kind.stages[index].repeats {
        return None;
    }
    let stage = &held.stages[index];
    let at = match stage.state {
        StageState::Starting
        | StageState::Started
        | StageState::Begun
        | StageState::Stuck
        | StageState::Lost
        | StageState::NeverBegun
        | StageState::Stopped
        | StageState::EndedShort => {
            let given = stage.start.as_ref()?.group.as_ref()?;
            find_group(facts, given)?
        }
        StageState::OnHold
        | StageState::WaitingForRoom
        | StageState::WaitingOnReadiness
        | StageState::Refused
        | StageState::Repeating => facts.first_unticked_group()?,
        StageState::NotReached | StageState::Delivered | StageState::Finished => return None,
    };
    let group = &facts.groups[at];
    Some(GroupStatus {
        place: at + 1,
        of: facts.groups.len(),
        number: group.number.clone(),
        title: group.title.clone(),
    })
}

/// The move a live place waits on its caller for, why, since when and
/// with what words, read from its held state alone; none where it waits
/// on nobody, its next step being flume's or an agent's, or the host's
/// room (56, 70, 130).
pub(crate) fn waiting_on(held: &HeldState) -> Option<WaitingOn> {
    if held.phase != Phase::Live {
        return None;
    }
    let place = &held.place;
    let stages = &place.kind.stages;
    let last = held.stages.len().checked_sub(1)?;
    let waits = |act, stage: Option<usize>, why, since, words| {
        Some(WaitingOn {
            act,
            stage: stage.map(|index| held.stages[index].name.clone()),
            why,
            since,
            words,
        })
    };
    let submission = &held.submission;
    if let Some(words) = &submission.waiting {
        let found = held.stages[last].state == StageState::EndedShort
            && held.stages[last].end.as_ref().and_then(|end| end.needs)
                == Some(StopKind::FoundWork);
        let failed = matches!(
            submission.verdict,
            Some(Verdict::Failed { .. } | Verdict::Conflict { .. })
        );
        if found || failed {
            let target = stages[last]
                .send_back_to
                .as_ref()
                .and_then(|name| held.stages.iter().position(|stage| &stage.name == name))
                .unwrap_or(last);
            return waits(
                Act::Retry,
                Some(target),
                Waits::PastBound,
                submission.waiting_since,
                Some(words.clone()),
            );
        }
        return waits(
            Act::Retry,
            None,
            Waits::Submission,
            submission.waiting_since,
            Some(words.clone()),
        );
    }
    if let Some(index) = held.stages.iter().position(|stage| {
        matches!(
            stage.state,
            StageState::OnHold | StageState::WaitingOnReadiness
        )
    }) {
        let stage = &held.stages[index];
        return match stage.state {
            StageState::OnHold => waits(
                Act::Retry,
                Some(index),
                Waits::OnHold,
                stage.since,
                held.hold_on(&stage.name)
                    .and_then(|hold| hold.words.clone()),
            ),
            _ => waits(
                Act::Retry,
                Some(index),
                Waits::Unready,
                stage.since,
                held.readiness
                    .failed()
                    .map(|step| format!("{}: {}", step.step, step.words)),
            ),
        };
    }
    let index = held.current?;
    let stage = &held.stages[index];
    let retry = |why, words| waits(Act::Retry, Some(index), why, stage.since, words);
    match stage.state {
        StageState::EndedShort => {
            let end = stage.end.as_ref();
            let found = end.and_then(|end| end.needs) == Some(StopKind::FoundWork);
            if found && index == last {
                return None;
            }
            retry(Waits::Answer, end.and_then(|end| end.words.clone()))
        }
        StageState::Lost => retry(Waits::Lost, None),
        StageState::NeverBegun => retry(Waits::NeverBegun, None),
        StageState::Refused => retry(
            Waits::Refused,
            stage
                .refused_part
                .clone()
                .or_else(|| stage.refused_setting.clone()),
        ),
        StageState::Stopped => retry(Waits::Stopped, None),
        StageState::Delivered | StageState::Finished if index == last => {
            if !place.kind.lands {
                waits(Act::Done, None, Waits::Delivered, stage.since, None)
            } else if let Some(hold) = held.submission_hold().filter(|_| submission.due) {
                waits(
                    Act::Retry,
                    None,
                    Waits::OnHold,
                    stage.since,
                    hold.words.clone(),
                )
            } else {
                None
            }
        }
        _ => None,
    }
}
