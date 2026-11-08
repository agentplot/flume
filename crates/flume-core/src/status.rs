//! Status of one place or every place (56 to 62): every fact of a
//! deliverable, a change or a submission read where it lives when asked,
//! at one head per place; nothing written, sent or started (57, 58).
//! The places are read side by side, each repository's merge queue once
//! for all its places asked, and each live agent once for its place and
//! its base's count (58, 139, C.6). A place's kind is named by its
//! identity and where it came from as its launch read them, none for a
//! place whose held state keeps neither, its `declared` then saying its
//! source (19, 21).

use crate::base::{base_place, submission_open};
use crate::held::waiting_now;
use crate::list::waits_on;
use crate::live::waits_on_session;
use crate::reading::{find_group, reads_change};
use crate::settings::program_name;
use crate::start::Starts;
use crate::{
    side_by_side, Act, AgentSettings, AgentStatus, Answer, Bound, EffortStatus, EndStatus, Engine,
    GroupStatus, ModeStatus, MoveStatus, OwnFileStatus, OwnStatus, PlaceStatus, SessionAnswer,
    SessionWait, SourceStatus, StageStatus, SubmissionStatus, WaitingOn, Waits,
};
use flume_seams::*;
use std::collections::BTreeMap;
use std::sync::Arc;

/// The effort `start` passed, with its layer, and `variable`, the one its
/// program reads over it, where its account's environment names it, by
/// its name alone; none where no layer gave an effort (56, 197; core.30).
fn effort_status(start: &StartHeld, variable: Option<&'static str>) -> Option<EffortStatus> {
    let word = start.settings.effort.as_ref()?;
    let given = start.from.get("effort")?;
    let outranked_by = variable
        .filter(|name| {
            start
                .from
                .get(&format!("env.{name}"))
                .is_some_and(|given| given.layer == Layer::Account)
        })
        .map(str::to_string);
    Some(EffortStatus {
        word: word.clone(),
        layer: given.layer,
        passed: word.clone(),
        read_back: false,
        outranked_by,
    })
}

/// Whether status reads a place's facts where they live: not for a place
/// being made, gone or tidied (56, 62).
fn reads_facts(held: &HeldState) -> bool {
    !matches!(
        held.phase,
        Phase::Gone { .. } | Phase::Making { .. } | Phase::Tidied { .. }
    )
}

/// One repository's ask of the merge queue its places' binding names:
/// the places whose submissions it reads, each by its index with its
/// branch.
struct Ask {
    repository: String,
    profile: String,
    queue: Arc<dyn MergeQueue>,
    places: Vec<(usize, String)>,
}

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
        let mut submissions = self.submissions(&held);
        let read: Vec<(&HeldState, Option<ReadAcross>)> = held
            .iter()
            .enumerate()
            .map(|(at, held)| (held, submissions.remove(&at)))
            .collect();
        let places = side_by_side(&read, |(held, submission)| {
            self.place_status_read(held, submission.clone())
        });
        let agents: BTreeMap<PlaceId, AgentState> = places
            .iter()
            .filter_map(|status| Some((status.place.clone(), status.agent.as_ref()?.state)))
            .collect();
        Answer::Status {
            places,
            host: self.host_status(&agents),
        }
    }

    /// The submission of each of `held` that status reads, by its index:
    /// one that lands, whose facts are read and whose binding names a
    /// merge queue; each repository's merge queue asked once for all its
    /// places, the repositories side by side (58, 99, C.6).
    fn submissions(&self, held: &[HeldState]) -> BTreeMap<usize, ReadAcross> {
        let mut asks: Vec<Ask> = vec![];
        for (at, held) in held.iter().enumerate() {
            let place = &held.place;
            let Some(profile) = place.binding.merge_queue.as_ref() else {
                continue;
            };
            if !place.kind.lands || !reads_facts(held) {
                continue;
            }
            let Ok(Bound {
                merge_queue: Some(queue),
                ..
            }) = self.bound(place)
            else {
                continue;
            };
            let branch = (at, place.branch.clone());
            match asks
                .iter_mut()
                .find(|ask| ask.repository == place.repository && &ask.profile == profile)
            {
                Some(ask) => ask.places.push(branch),
                None => asks.push(Ask {
                    repository: place.repository.clone(),
                    profile: profile.clone(),
                    queue,
                    places: vec![branch],
                }),
            }
        }
        side_by_side(&asks, |ask| {
            let branches: Vec<&str> = ask
                .places
                .iter()
                .map(|(_, branch)| branch.as_str())
                .collect();
            ask.places
                .iter()
                .map(|(at, _)| *at)
                .zip(ask.queue.read_submissions(&ask.repository, &branches))
                .collect::<Vec<_>>()
        })
        .into_iter()
        .flatten()
        .collect()
    }

    /// Every fact of one place, as it stands when asked.
    pub(crate) fn place_status(&self, held: &HeldState) -> PlaceStatus {
        let submission = self.submissions(std::slice::from_ref(held)).remove(&0);
        self.place_status_read(held, submission)
    }

    /// Every fact of one place, as it stands when asked, its submission
    /// as `submission` read it where its repository's merge queue was
    /// read for it.
    fn place_status_read(&self, held: &HeldState, submission: Option<ReadAcross>) -> PlaceStatus {
        let place = &held.place;
        let mut cannot_read = vec![];
        let mut refused_acts = vec![];
        let current = held.current;
        let read_facts = reads_facts(held);
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
        let base = base_place(&place.kind);
        let facts = bound
            .as_ref()
            .filter(|_| read_facts && !base)
            .map(|bound| self.facts(held, bound, with_start));
        // A base place's head is read where the merge queue says its
        // branch lives, and nothing at its location (80, 126, 203).
        let base_head = match bound.as_ref().and_then(|bound| bound.merge_queue.as_ref()) {
            Some(queue) if read_facts && base => {
                match self.base_place_head(place, queue.as_ref()) {
                    Ok(head) => Some(head),
                    Err(reason) => {
                        cannot_read.push(format!("the head of {}: {reason}", place.branch));
                        None
                    }
                }
            }
            _ => None,
        };
        let on_branch = base.then(|| self.on_branch(place));
        let on_branch_count = on_branch.as_ref().map(Vec::len);
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
            let mut read_back = false;
            let mut reserved = vec![];
            let mut effort_variable = None;
            if let Some(agent) = self.agent(&start.program) {
                if let Guarded::Refused { acts } = agent.guard(&folder, &start.identity) {
                    refused_acts.extend(acts);
                }
                last_words = agent.last_words(&folder, &start.identity);
                notes = agent.notes(&folder, &start.identity);
                read_back = agent.mode_read_back() == Offered::Offered;
                reserved = agent.reserved();
                effort_variable = agent.effort_variable();
            }
            let own = start.own.as_ref().map(|own| OwnStatus {
                passed: own.passed.clone(),
                read_back: false,
                files: own
                    .files
                    .iter()
                    .map(|file| OwnFileStatus {
                        layer: file.layer,
                        written: file.written.clone(),
                        read: file.read.clone(),
                        left_out: file
                            .left_out
                            .iter()
                            .map(|key| Reserved {
                                key: key.clone(),
                                owner: reserved
                                    .iter()
                                    .find(|reserved| &reserved.key == key)
                                    .map_or_else(
                                        || "a setting its profile no longer names".to_string(),
                                        |reserved| reserved.owner.clone(),
                                    ),
                            })
                            .collect(),
                    })
                    .collect(),
            });
            let mode = start.start_record.mode.clone().map(|passed| ModeStatus {
                passed,
                entry: matches!(start.settings.mode, Some(ModeSetting::Programs(_)))
                    .then(|| program_name(&start.program).to_string()),
                read_back,
            });
            let effort = effort_status(start, effort_variable);
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
                account: start.account.clone(),
                state: reading.state,
                started: reading.started,
                asking: reading.asking,
                own_work: reading.own_work,
                past_limit: start.past_limit,
                screen: starting.then(|| self.agent_on(start).screen()).flatten(),
                dialog: start.dialog.as_ref().map(|dialog| dialog.words.clone()),
                printed: start.printed.clone(),
                last_words,
                notes,
                mode,
                conversation: start.conversation.clone(),
                session: start.session.clone(),
                terminal: start.terminal.clone(),
                left_in_pane: held.stages[index]
                    .end
                    .as_ref()
                    .is_some_and(|end| end.left_in_pane),
                own,
                effort,
            }
        });
        let agent_state = agent.as_ref().map(|agent| agent.state);
        let asking = agent
            .as_ref()
            .is_some_and(|agent| agent.asking == Reading::Yes);
        let line = Engine::place_line(held, change, asking, on_branch_count, None);
        let waiting = waiting_now(held, on_branch_count);
        let waits_on = waits_on(held, &line.state, on_branch_count);
        let submission = match (&bound, place.kind.lands) {
            (Some(bound), true) if read_facts => bound.merge_queue.as_ref().map(|queue| {
                let across = submission
                    .unwrap_or_else(|| queue.read_submission(&place.repository, &place.branch));
                if let SubmissionRead::CouldNotRead { reason } = &across.read {
                    cannot_read.push(format!("the merge queue: {reason}"));
                }
                for over in &across.passed_over {
                    cannot_read.push(format!("the merge queue's {}: {}", over.branch, over.words));
                }
                if let Some(reason) = &held.submission.unread {
                    cannot_read.push(format!("the merge queue's inbox: {reason}"));
                }
                for over in &held.submission.passed_over {
                    cannot_read.push(format!(
                        "the merge queue's inbox, of {}: {}",
                        over.branch, over.words
                    ));
                }
                let read = crate::submit::own_read(held, across);
                SubmissionStatus {
                    read,
                    request: held.submission.request.clone(),
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
                answers: session_answers(held, index),
            })
            .collect();
        let group = current.and_then(|index| stages[index].group.clone());
        PlaceStatus {
            place: place.id.clone(),
            repository: place.repository.clone(),
            address: place.address.clone(),
            kind: place.kind.name.clone(),
            version: place.kind.version,
            declared: place.declared,
            identity: place
                .kind_from
                .as_ref()
                .map(|from| from.identity.spelled.clone()),
            kind_from: place.kind_from.clone(),
            name: place.name.clone(),
            branch: place.branch.clone(),
            base: place.base.clone(),
            base_from: place.base_from,
            mark: place.mark.clone(),
            lands: place.kind.lands,
            phase: held.phase.clone(),
            readiness: held.readiness.clone(),
            stages,
            current: current.map(|index| held.stages[index].name.clone()),
            agent,
            head: facts.as_ref().map(|facts| facts.head.clone()).or(base_head),
            delivered: with_start.and(facts.as_ref().map(|facts| facts.delivered)),
            tasks,
            submission,
            refused_acts,
            cannot_read,
            failures: held.failures.clone(),
            recipients: place.recipients.clone(),
            recipients_from: place.recipients_from.clone(),
            caller: place.caller.clone(),
            hold_words: place.hold_words.clone(),
            waiting_on: waiting.as_ref().map(|waiting| waiting.on.clone()),
            stalled: waiting.as_ref().is_some_and(|waiting| waiting.stalled),
            waits_on_session: session_wait(held),
            holds: held.holds.clone(),
            priority: held.priority,
            stream: place.stream.clone(),
            label: self.label_of(place),
            intent: place.intent.clone(),
            moves: held.moves.iter().map(MoveStatus::of).collect(),
            unsettled: held.unsettled.clone(),
            uncarried: held.uncarried.clone(),
            ends: held
                .ends
                .iter()
                .map(|end| EndStatus::of(end, &place.recipients))
                .collect(),
            role: held.role.clone(),
            clues: self.place_clues(held, agent_state, group.as_ref()),
            line,
            waits_on,
            worktree: base.then_some(false),
            on_branch,
        }
    }
}

impl Engine {
    /// Each source the host declares as it stands when asked, by name: its
    /// file, its packages' directory and every identity that directory
    /// holds, or why its file or its directory cannot be read; and its
    /// roles as `roles`, read once for the status, give them: its roles
    /// file, each address it declares, each entry that declares nothing,
    /// with why, or why the file cannot be read (56, 159, 191, 193;
    /// core.15).
    pub(crate) fn host_sources(&self, roles: &HostRoles) -> Vec<SourceStatus> {
        let kinds = self.resolver.kinds();
        self.host
            .sources()
            .into_iter()
            .map(|(name, source)| {
                let declared = roles
                    .declared
                    .iter()
                    .filter(|(_, entry)| entry.source() == name)
                    .map(|(address, _)| address.clone())
                    .collect();
                let roles_refused = roles
                    .refused
                    .iter()
                    .filter(|refused| refused.source == name)
                    .cloned()
                    .collect();
                let roles_cannot_read = roles.unread.get(&name).map(|unread| unread.reason.clone());
                match source.read {
                    HostSourceRead::Read {
                        kinds_dir,
                        roles: roles_file,
                    } => {
                        let (kinds, cannot_read) = match kinds.read_source(&name, &kinds_dir) {
                            SourceRead::Read { kinds } => {
                                (kinds.into_iter().map(|kind| kind.spelled).collect(), None)
                            }
                            SourceRead::CouldNotRead { reason } => (vec![], Some(reason)),
                        };
                        SourceStatus {
                            name,
                            source_file: source.file,
                            kinds_dir: Some(kinds_dir),
                            kinds,
                            cannot_read,
                            roles_file,
                            roles: declared,
                            roles_refused,
                            roles_cannot_read,
                        }
                    }
                    HostSourceRead::CouldNotRead { reason } => SourceStatus {
                        name,
                        source_file: source.file,
                        kinds_dir: None,
                        kinds: vec![],
                        cannot_read: Some(reason),
                        roles_file: None,
                        roles: declared,
                        roles_refused,
                        roles_cannot_read,
                    },
                }
            })
            .collect()
    }

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

/// Each answer the current start of the stage at `index` took in its
/// session, in the order it took them, each with the question it
/// answered, as the ends `held` records give them; none for a stage with
/// no start (42, 56, 198).
pub(crate) fn session_answers(held: &HeldState, index: usize) -> Vec<SessionAnswer> {
    let stage = &held.stages[index];
    let Some(start) = &stage.start else {
        return vec![];
    };
    held.ends
        .iter()
        .filter_map(|end| match &end.of {
            EndOf::Stage {
                stage: name,
                start: n,
                state: StageState::EndedShort,
                end: ended,
                ..
            } if *name == stage.name && *n == start.n => {
                let answer = end
                    .answer
                    .clone()
                    .filter(|answer| answer.session.is_some())?;
                Some(SessionAnswer {
                    question: ended.words.clone(),
                    answer,
                })
            }
            _ => None,
        })
        .collect()
}

/// The stage of a live place that waits for room while a process runs
/// the conversation its start would carry, or flume could not read
/// whether one does, with what the last settle's live read and since
/// when, from its held state alone; none where no stage waits so (49, 56,
/// 207).
pub(crate) fn session_wait(held: &HeldState) -> Option<SessionWait> {
    if held.phase != Phase::Live {
        return None;
    }
    held.stages.iter().find_map(|stage| {
        let room = stage
            .room
            .as_ref()
            .filter(|room| stage.state == StageState::WaitingForRoom && waits_on_session(room))?;
        Some(SessionWait {
            stage: stage.name.clone(),
            processes: room.live.clone(),
            unread: room.live_unread.clone(),
            since: stage.since,
        })
    })
}

/// The move a live place waits on its caller for, why, since when and
/// with what words, read from its held state alone, a person's answer to
/// the dialog its agent's program shows before it takes its prompt among
/// them, and for an answer whether its agent is kept open for it and
/// until when, or when and why it was let go and whether it is ended;
/// none where it waits on nobody, its next step being flume's or an
/// agent's, or the host's room (34, 35, 56, 70, 130, 198). A base place's
/// is read from its submission and `on_branch`, the count of the live
/// places on its branch (203).
pub(crate) fn waiting_on(held: &HeldState, on_branch: Option<usize>) -> Option<WaitingOn> {
    if held.phase != Phase::Live {
        return None;
    }
    let place = &held.place;
    if base_place(&place.kind) {
        return base_waiting_on(held, on_branch.unwrap_or(0));
    }
    let last = held.stages.len().checked_sub(1)?;
    let waits = |act, stage: Option<usize>, why, since, words| {
        Some(WaitingOn {
            act,
            stage: stage.map(|index| held.stages[index].name.clone()),
            why,
            since,
            words,
            open_until: None,
            closed: None,
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
            return waits(
                Act::Retry,
                crate::submit::fix_round_of(held),
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
        StageState::Starting | StageState::Started => {
            let dialog = stage.start.as_ref()?.dialog.as_ref()?;
            waits(
                Act::Answer,
                Some(index),
                Waits::Dialog,
                Some(dialog.since),
                Some(dialog.words.clone()),
            )
        }
        StageState::EndedShort => {
            if crate::submit::sent_back_by(held, index) {
                return None;
            }
            let end = stage.end.as_ref();
            let closed = end.and_then(|end| end.closed);
            retry(Waits::Answer, end.and_then(|end| end.words.clone())).map(|on| WaitingOn {
                open_until: end
                    .and_then(|end| end.open_until)
                    .filter(|_| closed.is_none()),
                closed,
                ..on
            })
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

/// The move a live base place waits on its caller for: none while a
/// place on its branch is live, since it waits on them and is never
/// stalled for it, or while its submission is open, since it waits on
/// the merge queue; else a retry of its submission, with the merge
/// queue's words where its last submission failed, conflicted, was
/// withdrawn or refused, with a hold's words where its submission is
/// held, and otherwise since the settle that read none live (47, 112,
/// 130, 203).
fn base_waiting_on(held: &HeldState, on_branch: usize) -> Option<WaitingOn> {
    let submission = &held.submission;
    let waits = |why, since, words| {
        Some(WaitingOn {
            act: Act::Retry,
            stage: None,
            why,
            since,
            words,
            open_until: None,
            closed: None,
        })
    };
    if on_branch > 0 {
        return None;
    }
    if let Some(words) = &submission.waiting {
        return waits(
            Waits::Submission,
            submission.waiting_since,
            Some(words.clone()),
        );
    }
    if let Some(hold) = held.submission_hold() {
        return waits(Waits::OnHold, Some(hold.since), hold.words.clone());
    }
    if submission_open(held) {
        return None;
    }
    waits(Waits::Unsubmitted, None, None)
}
