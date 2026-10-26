//! A stage's start (27 to 33): its settings resolved, a fresh identity
//! and secret, its prompt filled or the stage refused naming the part,
//! and one agent opened through the agent seam's command and the
//! terminal, never while another of the place's is live.

use crate::prompt::{asks, render};
use crate::{Engine, Step};
use flume_seams::*;

/// What a start came to.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Start {
    Opened(Identity),
    /// The prompt asks for a part with nothing to fill it (27).
    Refused(PromptPart),
    /// Another agent of the place's is live (30).
    AgentLive,
    /// The terminal refused to open the agent.
    NotOpened(String),
    /// No agent profile answers for the stage's program (32, 105).
    NoAgent(String),
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
    /// A stage's agent settings: the launch's for its stage over the
    /// stage's declaration over flume's own default (32).
    pub(crate) fn settings(&self, place: &PlaceRecord, stage: &Stage) -> Settings {
        let declared = stage.agents.first().cloned().unwrap_or_default();
        let launched = place.settings.get(&stage.name).cloned().unwrap_or_default();
        self.defaults.settings.under(&declared).under(&launched)
    }

    /// A report's path: under the place's own folder for `{{state}}`, or
    /// inside the place (24, 123).
    pub(crate) fn report_path(&self, place: &PlaceRecord, path: &str) -> String {
        match path.strip_prefix("{{state}}") {
            Some(rest) => format!("{}{rest}", self.store.folder(&place.id)),
            None => format!("{}/{path}", place.location),
        }
    }

    /// The identity of the stage's next start (33).
    pub(crate) fn next_identity(&self, held: &HeldState, index: usize) -> Identity {
        Identity {
            repository: held.place.repository.clone(),
            place: held.place.short(),
            stage: held.place.kind.stages[index].name.clone(),
            start: held.stages[index].starts + 1,
        }
    }

    /// The stage's prompt filled for `identity`'s start, or the first
    /// part it asks for with nothing to fill it (27). The way to stop
    /// short is in every prompt (28), and a first stage started in a
    /// place that could not be made ready carries the step and its words
    /// (12).
    pub(crate) fn fill(
        &self,
        held: &HeldState,
        index: usize,
        identity: &Identity,
        words: Option<&str>,
    ) -> Result<String, PromptPart> {
        let place = &held.place;
        let stage = &place.kind.stages[index];
        let change = self.bound(place).ok().and_then(|bound| bound.change);
        let program = self.settings(place, stage).program.unwrap_or_default();
        let stop_short = self.agent(&program).map(|agent| agent.stop_short(identity));
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

    /// Starts the stage at `index` with one fresh agent and `words` in
    /// its prompt: the start is written to held state first, so two acts
    /// at once open one agent (30, 75), then the agent is opened.
    pub(crate) fn start_stage(&self, id: &PlaceId, index: usize, words: Option<String>) -> Start {
        let Some(read) = self.store.read(id) else {
            return Start::NoPlace;
        };
        let place = &read.state.place;
        let settings = self.settings(place, &place.kind.stages[index]);
        let program = settings.program.clone().unwrap_or_default();
        let Some(agent) = self.agent(&program) else {
            return Start::NoAgent(program);
        };
        let now = self.host.now();
        let claimed = self.update(id, |held| {
            if held
                .stages
                .iter()
                .any(|stage| stage.start.as_ref().is_some_and(|start| start.live))
            {
                return Step::Keep(Err(Start::AgentLive));
            }
            let identity = self.next_identity(held, index);
            match self.fill(held, index, &identity, words.as_deref()) {
                Err(part) => {
                    let stage = &mut held.stages[index];
                    stage.state = StageState::Refused;
                    stage.refused_part = Some(part.name().into());
                    Step::Write(Err(Start::Refused(part)))
                }
                Ok(text) => {
                    let head = self
                        .history
                        .head(&held.place.repository, &held.place.branch)
                        .unwrap_or_else(|| held.place.base_revision.clone());
                    let secret = self.host.secret();
                    let stage = &mut held.stages[index];
                    stage.state = StageState::Started;
                    stage.starts = identity.start;
                    stage.refused_part = None;
                    stage.end = None;
                    stage.start = Some(StartHeld {
                        n: identity.start,
                        started_at: now,
                        start_head: head.clone(),
                        words: words.clone(),
                        identity: identity.clone(),
                        secret: secret.clone(),
                        program: program.clone(),
                        settings: settings.clone(),
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
                    });
                    held.current = Some(index);
                    Step::Write(Ok((identity, secret, text, held.place.place_ref())))
                }
            }
        });
        let (identity, secret, text, place_ref) = match claimed {
            None => return Start::NoPlace,
            Some(Err(refused)) => return refused,
            Some(Ok(claim)) => claim,
        };
        let command = agent.command(&settings, &place_ref, &identity, &secret, &text);
        match self
            .terminal
            .open(&place_ref, &command, &identity, settings.beside.as_deref())
        {
            Opened::Opened { carried } => {
                if carried {
                    self.with_start(id, &identity, |start, _| start.prompt.carried = true);
                }
                Start::Opened(identity)
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
