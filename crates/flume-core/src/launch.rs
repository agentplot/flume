//! A launch (1 to 12, 19 to 23): the kind resolved over the three
//! declarations and the binding read, the base's revision where the
//! merge queue says it lives, every refusal made before anything is
//! made, then one place made, readied, and its first stage started, held
//! or left waiting on readiness. The repository's settings for the kind
//! are read with its kinds and kept, and every stage's agent is resolved
//! with the host's configuration as it stands, so a program no agent
//! profile runs refuses the launch (4, 32). Its recipients are the launch's, or the
//! one the caller's environment names, or the session the terminal says
//! launched it (7, 8). A refusal after the place's held state
//! is created undoes what was made, so nothing is left (2, 76).

use crate::reading::reads_change;
use crate::settings::{no_repository_env, no_stage_lacking};
use crate::{Answer, Bound, Engine, Step};
use flume_seams::*;

impl Engine {
    /// Launches one place, or refuses naming why (2).
    pub fn launch(&self, launch: &Launch) -> Answer {
        self.settle_attended();
        match self.make_place(launch) {
            Ok(held) => Answer::Launched {
                place: self.place_status(&held),
            },
            Err(reason) => Answer::refused(reason),
        }
    }

    /// The kind a launch names, from the launch's declaration over the
    /// repository's at the base's revision over flume's own (19, 20), and
    /// the settings the repository gives a kind of that name it does not
    /// declare, which join whichever declaration the name resolves to
    /// (32, C.2).
    fn resolve_kind(
        &self,
        launch: &Launch,
        revision: &Revision,
    ) -> Result<(Kind, Declared, KindSettings), String> {
        let kinds = self.resolver.kinds();
        let named = |all: Vec<Kind>| all.into_iter().find(|kind| kind.name == launch.kind);
        let (declared, mut given) = match kinds.read_kinds(&launch.repository, revision) {
            KindsRead::Read { kinds, settings } => (kinds, settings),
            KindsRead::CouldNotRead { reason } => {
                return Err(format!(
                    "the kinds {} declares at {revision} could not be read: {reason}",
                    launch.repository
                ))
            }
        };
        let given = given.remove(&launch.kind).unwrap_or_default();
        if let Some(declaration) = &launch.declaration {
            match kinds.read_declaration(declaration) {
                KindsRead::Read { kinds, .. } => {
                    if let Some(kind) = named(kinds) {
                        return Ok((kind, Declared::Launch, given));
                    }
                }
                KindsRead::CouldNotRead { reason } => {
                    return Err(format!(
                        "the launch's declaration {declaration} could not be read: {reason}"
                    ))
                }
            }
        }
        if let Some(kind) = named(declared) {
            return Ok((kind, Declared::Repository, given));
        }
        named(kinds.flumes_own())
            .map(|kind| (kind, Declared::Flume, given))
            .ok_or_else(|| format!("the kind {} resolves to no declaration", launch.kind))
    }

    /// Refuses a kind that asks for more than flume runs (4, 23).
    fn runs(kind: &Kind) -> Result<(), String> {
        let named = format!("the kind {}@{}", kind.name, kind.version);
        if kind.stages.is_empty() {
            return Err(format!("{named} has no stage"));
        }
        for stage in &kind.stages {
            if stage.agents.len() != 1 {
                return Err(format!(
                    "{named} asks for more than flume runs: its stage {} names {} agents, and flume runs one",
                    stage.name,
                    stage.agents.len()
                ));
            }
            let due = usize::from(!stage.conversation);
            if stage.deliverables.len() != due {
                return Err(format!(
                    "{named} asks for more than flume runs: its stage {} names {} deliverables",
                    stage.name,
                    stage.deliverables.len()
                ));
            }
            if let Some(Deliverable::Unreadable { spelled }) = stage.deliverables.first() {
                return Err(format!(
                    "{named} names a deliverable flume cannot read: {spelled}, of its stage {}",
                    stage.name
                ));
            }
        }
        Ok(())
    }

    fn make_place(&self, launch: &Launch) -> Result<HeldState, String> {
        let repository = &launch.repository;
        let kinds = self.resolver.kinds();
        let binding = match kinds.read_binding(repository) {
            BindingRead::Read { binding } => binding,
            BindingRead::CouldNotRead { reason } => {
                return Err(format!(
                    "the binding of {repository} could not be read: {reason}"
                ))
            }
        };
        let Bound {
            place: place_seam,
            change,
            merge_queue,
        } = self
            .bind(repository, &binding)
            .map_err(|reason| format!("{repository}'s binding: {reason}"))?;
        let base_revision = match &merge_queue {
            Some(queue) => match queue.base_revision(repository, &launch.base) {
                BaseRevision::At { revision } => revision,
                BaseRevision::Refused { reason } => {
                    return Err(format!(
                        "the base {}'s current revision could not be read: {reason}",
                        launch.base
                    ))
                }
            },
            None => self
                .history
                .head(repository, &launch.base)
                .ok_or_else(|| format!("{repository} has no base {}", launch.base))?,
        };
        let (kind, declared, repository_settings) = self.resolve_kind(launch, &base_revision)?;
        Self::runs(&kind)?;
        no_repository_env(
            &kind,
            declared == Declared::Repository,
            &repository_settings,
        )?;
        no_stage_lacking(&kind, &launch.settings, &repository_settings)?;
        let named = format!("the kind {}@{}", kind.name, kind.version);
        if kind.lands && merge_queue.is_none() {
            return Err(format!(
                "{named} lands, and {repository} binds no merge queue for it to land through"
            ));
        }
        let reads = reads_change(&kind);
        if reads && change.is_none() {
            return Err(format!(
                "{named} reads a change, and {repository} binds no change profile"
            ));
        }
        for stage in &kind.stages {
            if launch.brief.is_none() && crate::prompt::asks(&stage.prompt, PromptPart::Brief) {
                return Err(format!(
                    "the stage {} asks for the brief, and the launch gives none",
                    stage.name
                ));
            }
        }
        let branch = format!("{}/{}", kind.branch_prefix, launch.name);
        if !self.history.valid_branch(&branch) {
            return Err(format!(
                "the name {} gives no valid branch: {branch}",
                launch.name
            ));
        }
        let location = format!("{repository}/places/{}--{}", kind.name, launch.name);
        let id = PlaceId::new(location.clone());
        if reads {
            let change = change.as_ref().expect("bound");
            match change.read_change(repository, &base_revision, &launch.name) {
                ChangeRead::Read { facts } if facts.exists || facts.archived => {
                    return Err(format!(
                        "the base {} already holds a change named {}",
                        launch.base, launch.name
                    ))
                }
                ChangeRead::Read { .. } => {}
                ChangeRead::CouldNotRead { reason } => {
                    return Err(format!(
                        "the change {} could not be read: {reason}",
                        launch.name
                    ))
                }
            }
            if let Some(other) = self.store.read_every().into_iter().find(|other| {
                let tidied = matches!(other.state.phase, Phase::Tidied { .. });
                let other = &other.state.place;
                !tidied
                    && other.repository == *repository
                    && other.name == launch.name
                    && reads_change(&other.kind)
            }) {
                return Err(format!(
                    "the live place {} has the change name {}",
                    other.state.place.id, launch.name
                ));
            }
        }
        let recipients = if launch.recipients.is_empty() {
            self.host
                .recipient()
                .or_else(|| self.terminal.session())
                .into_iter()
                .collect()
        } else {
            launch.recipients.clone()
        };
        let held = HeldState {
            place: PlaceRecord {
                id: id.clone(),
                repository: repository.clone(),
                kind: kind.clone(),
                declared,
                name: launch.name.clone(),
                branch: branch.clone(),
                base: launch.base.clone(),
                base_revision: base_revision.clone(),
                location: location.clone(),
                binding,
                brief: launch.brief.clone(),
                mark: launch.mark.clone(),
                settings: launch.settings.clone(),
                every_stage: launch.every_stage.clone(),
                repository_settings,
                hold: launch.hold.clone(),
                hold_submission: launch.hold_submission,
                hold_words: launch.hold_words.clone(),
                recipients,
            },
            phase: Phase::Making {
                by: self.host.command(),
            },
            readiness: Readiness::default(),
            stages: kind
                .stages
                .iter()
                .map(|stage| StageHeld {
                    name: stage.name.clone(),
                    state: StageState::NotReached,
                    starts: 0,
                    refused_part: None,
                    refused_setting: None,
                    end: None,
                    start: None,
                    since: None,
                    room: None,
                })
                .collect(),
            current: None,
            submission: SubmissionHeld::default(),
            failures: vec![],
            priority: launch.priority,
            shown: None,
            ends: vec![],
        };
        for stage in &kind.stages {
            let resolved = self.resolve(&held.place, stage);
            self.agent_for(&resolved, &stage.name)?;
        }
        if self.create(&held).is_none() {
            return Err(format!("the place {id} already stands"));
        }
        let refused = match place_seam.make(repository, &base_revision, &branch, &location) {
            MakeOutcome::Made => None,
            MakeOutcome::BranchExists => Some(format!("the branch {branch} already exists")),
            MakeOutcome::LocationExists => Some(format!("something already stands at {location}")),
            MakeOutcome::Refused { reason } => {
                Some(format!("the place could not be made: {reason}"))
            }
        };
        if let Some(reason) = refused {
            self.remove_held(&id);
            return Err(reason);
        }
        let place_ref = held.place.place_ref();
        let readiness = place_seam.ready(&place_ref, &base_revision);
        let failed = readiness.failed().is_some();
        let first = &kind.stages[0];
        let holds = launch.hold.contains(&first.name) || first.waits;
        self.update(&id, |held| {
            held.readiness = readiness.clone();
            if failed && !kind.starts_unready {
                held.stages[0].state = StageState::WaitingOnReadiness;
            } else if holds {
                held.stages[0].state = StageState::OnHold;
            }
            Step::Write(())
        });
        if !(failed && !kind.starts_unready) && !holds {
            self.start_stage(&id, 0, None);
        }
        self.update(&id, |held| {
            held.phase = Phase::Live;
            Step::Write(())
        });
        self.store
            .read(&id)
            .map(|read| read.state)
            .ok_or_else(|| format!("the place {id} was removed while it was launched"))
    }
}
