//! A stage's start (27 to 33): its settings resolved from their six
//! layers, or the stage refused naming the setting (32), a fresh identity
//! and secret, its prompt filled or the stage refused naming the part,
//! and one agent opened through the agent seam's command and the
//! terminal, never while another of the place's is live, and only under
//! the host's bound (70).

use crate::prompt::{asks, group_text, notes_path, render};
use crate::room::Admit;
use crate::{repository_id, Engine, Step};
use flume_seams::*;
use std::sync::Arc;

/// A start written as live, its agent still to open.
pub(crate) struct Claim {
    identity: Identity,
    secret: Secret,
    text: String,
    place: PlaceRef,
    /// The settings the agent starts with, the host's values read (29).
    settings: Settings,
    agent: Arc<dyn Agent>,
    /// The session name the agent carries: a role's run's address (132).
    session: Option<String>,
}

/// What a start came to.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Start {
    Opened(Identity),
    /// The terminal read the start held at a question its program asks
    /// before it starts, or its program not come up: never begun at once
    /// (35).
    NeverBegun(Identity),
    /// The host has no room, or stages waiting rank ahead of it (70).
    WaitingForRoom,
    /// The prompt asks for a part with nothing to fill it (27).
    Refused(PromptPart),
    /// Another agent of the place's is live (30).
    AgentLive,
    /// The terminal refused to open the agent.
    NotOpened(String),
    /// The stage's settings name a program no agent profile answers for,
    /// or a variable the host user's login environment lacks (27, 32,
    /// 105).
    Unresolved(String),
    NoPlace,
}

/// What readiness came to, as a prompt's `{{unready}}` gives it (12).
fn unready(readiness: &Readiness) -> String {
    match readiness.failed() {
        Some(step) => format!(
            "The place's readiness failed at {}: {}",
            step.step, step.words
        ),
        None if readiness.steps.is_empty() => "The repository declares no readiness.".into(),
        None => "The place's readiness passed.".into(),
    }
}

impl Engine {
    /// A report's path: under the place's own folder for `{{state}}`, or
    /// inside the place (24, 123).
    pub(crate) fn report_path(&self, place: &PlaceRecord, path: &str) -> String {
        match path.strip_prefix("{{state}}") {
            Some(rest) => format!("{}{rest}", self.store.folder(&place.id)),
            None => format!("{}/{path}", place.location),
        }
    }

    /// The identity of the stage's next start, its repository named by
    /// the id of its common git directory, or of its location where git
    /// cannot say (33, C.4).
    pub(crate) fn next_identity(&self, held: &HeldState, index: usize) -> Identity {
        let repository = &held.place.repository;
        let common = self
            .history
            .common_dir(repository)
            .unwrap_or_else(|| repository.clone());
        Identity {
            repository: repository_id(&common),
            place: held.place.short(),
            stage: held.place.kind.stages[index].name.clone(),
            start: held.stages[index].starts + 1,
        }
    }

    /// The stage's prompt filled for `identity`'s start, or the first
    /// part it asks for with nothing to fill it (27). The way to stop
    /// short is in every prompt (28), as `agent`, the profile of the
    /// stage's program, gives it; and a first stage started in a place
    /// that could not be made ready carries the step and its words (12).
    /// A repeating stage fills `{{group}}` with `group`, the start's,
    /// which none fills where the change could not be read, and
    /// `{{notes}}` with its notes' path; a stage that does not repeat
    /// fills neither (27, 141).
    pub(crate) fn fill(
        &self,
        held: &HeldState,
        index: usize,
        identity: &Identity,
        words: Option<&str>,
        agent: Option<&dyn Agent>,
        group: Option<Option<&GroupHeld>>,
    ) -> Result<String, PromptPart> {
        let place = &held.place;
        let stage = &place.kind.stages[index];
        let change = self.bound(place).ok().and_then(|bound| bound.change);
        let stop_short = agent.map(|agent| agent.stop_short(identity));
        let fill = |part: PromptPart| -> Option<String> {
            match part {
                PromptPart::Brief => place.brief.clone(),
                PromptPart::Name => Some(place.name.clone()),
                PromptPart::Kind => Some(place.kind.name.clone()),
                PromptPart::Stage => Some(stage.name.clone()),
                PromptPart::Branch => Some(place.branch.clone()),
                PromptPart::Base => Some(place.base.clone()),
                PromptPart::Place => Some(place.location.clone()),
                PromptPart::Change => change.as_ref().map(|change| change.path(&place.name)),
                PromptPart::Report => match stage.deliverables.first() {
                    Some(Deliverable::Report { path }) => Some(self.report_path(place, path)),
                    Some(Deliverable::CommittedReport { path }) => Some(path.clone()),
                    _ => None,
                },
                PromptPart::Words => words.map(str::to_string),
                PromptPart::Unready => Some(unready(&held.readiness)),
                PromptPart::Needs => stop_short.clone(),
                PromptPart::Made => (!stage.makes.is_empty()).then(|| self.made_path(&place.id)),
                PromptPart::Group => group.filter(|_| stage.repeats).map(group_text),
                PromptPart::Notes => stage
                    .repeats
                    .then(|| notes_path(&self.store.folder(&place.id), &stage.name)),
            }
        };
        let mut text = render(&stage.prompt, fill)?;
        if !asks(&stage.prompt, PromptPart::Needs) {
            text.push_str("\n\n");
            text.push_str(&stop_short.clone().ok_or(PromptPart::Needs)?);
        }
        if index == 0
            && held.readiness.failed().is_some()
            && !asks(&stage.prompt, PromptPart::Unready)
        {
            text.push_str("\n\n");
            text.push_str(&unready(&held.readiness));
        }
        Ok(text)
    }

    /// The task group a start of the stage at `index` takes at `head`,
    /// for a repeating stage: the first of the change's groups holding a
    /// task unticked there, or none where every task is ticked or the
    /// change holds none (140). None for a stage that does not repeat,
    /// and for a change that cannot be read at `head`, which gives a
    /// repetition nothing to work from.
    pub(crate) fn group_at(
        &self,
        held: &HeldState,
        index: usize,
        head: &Revision,
    ) -> Option<Option<GroupHeld>> {
        let place = &held.place;
        if !place.kind.stages[index].repeats {
            return None;
        }
        let change = self.bound(place).ok()?.change?;
        let ChangeRead::Read { facts } = change.read_change(&place.repository, head, &place.name)
        else {
            return None;
        };
        Some(facts.first_unticked_group().map(|at| {
            let group = &facts.groups[at];
            GroupHeld {
                index: at,
                number: group.number.clone(),
                title: group.title.clone(),
                tasks: facts.tasks_of(at).map(|task| task.text.clone()).collect(),
            }
        }))
    }

    /// Starts the stage at `index` with one fresh agent and `words` in
    /// its prompt, admitted under the host's room lock: the start is
    /// written to held state as live while there is room and no stage
    /// waiting ranks ahead of it, so two acts at once open one agent and
    /// two settles at once never start past the bound (30, 70, 75), and
    /// the agent is opened once the lock is released; otherwise the stage
    /// waits for room with its words.
    pub(crate) fn start_stage(&self, id: &PlaceId, index: usize, words: Option<String>) -> Start {
        let claimed = {
            let _room = self.host.room();
            let queue = self.queue();
            self.claim(id, index, words, Admit::Queue(&queue))
        };
        match claimed {
            Ok(claim) => self.open(id, claim),
            Err(start) => start,
        }
    }

    /// Claims the stage's start in held state: refused naming the part
    /// its prompt cannot fill, left waiting for room where `admit` says
    /// so, or written as live; called under the room lock.
    pub(crate) fn claim(
        &self,
        id: &PlaceId,
        index: usize,
        words: Option<String>,
        admit: Admit<'_>,
    ) -> Result<Claim, Start> {
        let Some(read) = self.store.read(id) else {
            return Err(Start::NoPlace);
        };
        let place = &read.state.place;
        let resolution = self.resolution(place, &place.kind.stages[index]);
        let now = self.host.now();
        let made_from = self.store.made(id).len();
        let claimed = self.update(id, |held| {
            if let Admit::Waiting = admit {
                if held.phase != Phase::Live
                    || held.stages[index].state != StageState::WaitingForRoom
                {
                    return Step::Keep(Err(Start::NoPlace));
                }
            }
            if held
                .stages
                .iter()
                .any(|stage| stage.start.as_ref().is_some_and(|start| start.live))
            {
                return Step::Keep(Err(Start::AgentLive));
            }
            let resolution = match &resolution {
                Ok(resolution) => resolution,
                Err(reason) => {
                    let stage = &mut held.stages[index];
                    stage.state = StageState::Refused;
                    stage.refused_part = None;
                    stage.refused_setting = Some(reason.clone());
                    stage.room = None;
                    held.current = Some(index);
                    return Step::Write(Err(Start::Unresolved(reason.clone())));
                }
            };
            let head = self
                .history
                .head(&held.place.repository, &held.place.branch)
                .unwrap_or_else(|| held.place.base_revision.clone());
            let group = self.group_at(held, index, &head);
            let identity = self.next_identity(held, index);
            let agent = resolution.agent.as_ref();
            let text = match self.fill(
                held,
                index,
                &identity,
                words.as_deref(),
                Some(agent),
                group.as_ref().map(Option::as_ref),
            ) {
                Err(part) => {
                    let stage = &mut held.stages[index];
                    stage.state = StageState::Refused;
                    stage.refused_part = Some(part.name().into());
                    stage.refused_setting = None;
                    stage.room = None;
                    return Step::Write(Err(Start::Refused(part)));
                }
                Ok(text) => text,
            };
            if let Admit::Queue(queue) = admit {
                if queue.must_wait(id, held.priority) {
                    let stage = &mut held.stages[index];
                    stage.state = StageState::WaitingForRoom;
                    stage.room = Some(RoomWait {
                        order: queue.next_order(),
                        words: words.clone(),
                    });
                    held.current = Some(index);
                    return Step::Write(Err(Start::WaitingForRoom));
                }
            }
            let secret = self.host.secret();
            let stage = &mut held.stages[index];
            stage.state = StageState::Starting;
            stage.starts = identity.start;
            stage.refused_part = None;
            stage.refused_setting = None;
            stage.room = None;
            stage.end = None;
            stage.start = Some(StartHeld {
                n: identity.start,
                started_at: now,
                start_head: head.clone(),
                made_from,
                words: words.clone(),
                identity: identity.clone(),
                secret: secret.clone(),
                program: resolution
                    .resolved
                    .settings
                    .program
                    .clone()
                    .unwrap_or_default(),
                settings: resolution.resolved.settings.clone(),
                from: resolution.resolved.from.clone(),
                record: None,
                prompt: Prompt {
                    text: text.clone(),
                    ..Prompt::default()
                },
                live: true,
                begun_at: None,
                first_quiet: None,
                last_head: head,
                last_head_move: now,
                past_limit: false,
                printed: None,
                group: group.clone().flatten(),
            });
            held.current = Some(index);
            Step::Write(Ok(Claim {
                identity,
                secret,
                text,
                place: held.place.place_ref(),
                settings: resolution.settings.clone(),
                agent: resolution.agent.clone(),
                session: held.role.clone(),
            }))
        });
        claimed.unwrap_or(Err(Start::NoPlace))
    }

    /// Opens the agent of a claimed start, out of the room lock; an open
    /// the terminal refuses ends the start, freeing its room (70).
    pub(crate) fn open(&self, id: &PlaceId, claim: Claim) -> Start {
        let Claim {
            identity,
            secret,
            text,
            place,
            settings,
            agent,
            session,
        } = claim;
        let command = agent.command(&settings, &place, &identity, &secret, &text);
        let folder = self.store.folder(id);
        match self.terminal.open(
            &place,
            &folder,
            &command,
            &identity,
            settings.beside.as_deref(),
            session.as_deref(),
        ) {
            Opened::Opened { carried, printed } => {
                if carried || printed.is_some() {
                    self.with_start(id, &identity, |start, _| {
                        start.prompt.carried = carried;
                        start.printed = printed.clone();
                    });
                }
                Start::Opened(identity)
            }
            Opened::Held { screen } | Opened::NotComeUp { screen } => {
                self.never_begun_with(id, &identity, screen);
                Start::NeverBegun(identity)
            }
            Opened::Refused { reason } => {
                let failure = format!("the terminal did not open {identity}: {reason}");
                self.update(id, |held| {
                    if let Some(index) = held.index_of(&identity) {
                        let stage = &mut held.stages[index];
                        stage.state = StageState::NeverBegun;
                        if let Some(start) = stage.start.as_mut() {
                            start.live = false;
                        }
                        held.failures.push(failure.clone());
                    }
                    Step::Write(())
                });
                Start::NotOpened(reason)
            }
        }
    }

    /// Changes the start under `identity`, if it is still the stage's,
    /// and writes `end` as its stage's end in the same write, so the end
    /// is recorded whole with what was read of it (43).
    pub(crate) fn end_start(
        &self,
        id: &PlaceId,
        identity: &Identity,
        mut change: impl FnMut(&mut StartHeld, &mut StageState),
        end: Option<StageEnd>,
    ) {
        self.update(id, |held| {
            if let Some(index) = held.index_of(identity) {
                let stage = &mut held.stages[index];
                if let Some(start) = stage.start.as_mut() {
                    change(start, &mut stage.state);
                }
                if let Some(end) = &end {
                    stage.end = Some(end.clone());
                }
            }
            Step::Write(())
        });
    }

    /// Changes the start under `identity`, if it is still the stage's.
    pub(crate) fn with_start(
        &self,
        id: &PlaceId,
        identity: &Identity,
        mut change: impl FnMut(&mut StartHeld, &mut StageState),
    ) {
        self.update(id, |held| {
            if let Some(index) = held.index_of(identity) {
                let stage = &mut held.stages[index];
                if let Some(start) = stage.start.as_mut() {
                    change(start, &mut stage.state);
                }
            }
            Step::Write(())
        });
    }
}

/// Finding a start in held state by its identity.
pub(crate) trait Starts {
    fn index_of(&self, identity: &Identity) -> Option<usize>;
    fn live_start(&self) -> Option<(usize, &StartHeld)>;
}

impl Starts for HeldState {
    fn index_of(&self, identity: &Identity) -> Option<usize> {
        self.stages.iter().position(|stage| {
            stage
                .start
                .as_ref()
                .is_some_and(|start| &start.identity == identity)
        })
    }

    fn live_start(&self) -> Option<(usize, &StartHeld)> {
        self.stages.iter().enumerate().find_map(|(index, stage)| {
            stage
                .start
                .as_ref()
                .filter(|start| start.live)
                .map(|start| (index, start))
        })
    }
}
