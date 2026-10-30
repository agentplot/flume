//! A launch (1 to 12, 19 to 23): the kind resolved over the three
//! declarations and the binding read, the base's revision where the
//! merge queue says it lives, every refusal made before anything is
//! made, then one place made, readied, and its first stage started, held
//! or left waiting on readiness. The repository's settings for the kind
//! are read with its kinds and kept, and every stage's agent is resolved
//! with the host's configuration as it stands, so a program no agent
//! profile runs refuses the launch (4, 32). Its recipients are the launch's, or the
//! one the caller's environment names, or the session the terminal says
//! launched it (7, 8), and a role's run has none, its agents' session
//! named by its address (132). The newest submission the merge queue
//! holds of the branch at the launch is an earlier place's of the same
//! name, kept so it is never read as this place's (3, 99). A refusal
//! after the place's held state
//! is created undoes what was made, so nothing is left (2, 76). The
//! place's stream is chosen once, the launch's, else the one the host's
//! configuration names for its repository and kind, its repository or
//! its kind, else `default`, and one the host does not declare refuses
//! the launch (4, 8, 143); its intent is the launch's name, else its
//! repository and base (147).

use crate::reading::reads_change;
use crate::settings::no_stage_lacking;
use crate::{Answer, Bound, Engine, Step};
use flume_seams::*;

impl Engine {
    /// Launches one place, or refuses naming why (2).
    pub fn launch(&self, launch: &Launch) -> Answer {
        self.settle_attended();
        match self.make_place(launch, None) {
            Ok(held) => Answer::Launched {
                place: self.place_status(&held),
            },
            Err(reason) => Answer::refused(reason),
        }
    }

    /// The kind a launch names, `<name>` or `<name>@<version>`, from the
    /// launch's declaration over the repository's at the base's revision
    /// over flume's own (19, 20), and the settings the repository gives a
    /// kind of that name it does not declare, which join whichever
    /// declaration the name resolves to (32, C.2). The launch's and the
    /// repository's declarations hold one version of a name, and a version
    /// named that the resolved one does not hold refuses the launch; among
    /// flume's own the version named is taken, else the newest, and a
    /// version flume does not hold refuses the launch naming the versions
    /// it holds (4, 22, 154).
    fn resolve_kind(
        &self,
        launch: &Launch,
        revision: &Revision,
    ) -> Result<(Kind, Declared, KindSettings), String> {
        let (name, version) = named_version(&launch.kind)?;
        let kinds = self.resolver.kinds();
        let named = |all: Vec<Kind>| all.into_iter().find(|kind| kind.name == name);
        let held = |kind: Kind, declared: Declared, by: &str| {
            match version {
            Some(version) if version != kind.version => Err(format!(
                "the kind {name}@{version} is not held: {by} declares {name} at version {}, and resolves the name (19, 154)",
                kind.version
            )),
            _ => Ok((kind, declared)),
        }
        };
        let (declared, mut given) = match kinds.read_kinds(&launch.repository, revision) {
            KindsRead::Read { kinds, settings } => (kinds, settings),
            KindsRead::CouldNotRead { reason } => {
                return Err(format!(
                    "the kinds {} declares at {revision} could not be read: {reason}",
                    launch.repository
                ))
            }
        };
        let given = given.remove(name).unwrap_or_default();
        if let Some(declaration) = &launch.declaration {
            match kinds.read_declaration(declaration) {
                KindsRead::Read { kinds, .. } => {
                    if let Some(kind) = named(kinds) {
                        let by = format!("the launch's declaration {declaration}");
                        let (kind, declared) = held(kind, Declared::Launch, &by)?;
                        return Ok((kind, declared, given));
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
            let by = format!("{} at {revision}", launch.repository);
            let (kind, declared) = held(kind, Declared::Repository, &by)?;
            return Ok((kind, declared, given));
        }
        let mut own: Vec<Kind> = kinds
            .flumes_own()
            .into_iter()
            .filter(|kind| kind.name == name)
            .collect();
        if own.is_empty() {
            return Err(format!(
                "the kind {} resolves to no declaration",
                launch.kind
            ));
        }
        own.sort_by_key(|kind| kind.version);
        let versions = format!(
            "version{} {}",
            if own.len() > 1 { "s" } else { "" },
            own.iter()
                .map(|kind| kind.version.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        let kind = match version {
            None => own.pop(),
            Some(version) => own.into_iter().find(|kind| kind.version == version),
        };
        kind.map(|kind| (kind, Declared::Flume, given))
            .ok_or_else(|| {
                format!(
                    "the kind {} is not held: flume holds {name} at {versions} (22, 154)",
                    launch.kind
                )
            })
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

    /// Makes the launch's place, a role's run where `role` names the
    /// address it serves: its agents' session named by the address and no
    /// recipient of its own (132).
    pub(crate) fn make_place(
        &self,
        launch: &Launch,
        role: Option<&str>,
    ) -> Result<HeldState, String> {
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
        no_stage_lacking(&kind, &launch.settings, &repository_settings)?;
        let intent = self.intent_of(launch);
        let stream = self.stream_of(launch, &kind.name, &intent)?;
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
        let before = match (&merge_queue, kind.lands) {
            (Some(queue), true) => match queue.read_submission(repository, &branch) {
                SubmissionRead::Found { submission } => Some(submission.id),
                SubmissionRead::None | SubmissionRead::CouldNotRead { .. } => None,
            },
            _ => None,
        };
        let recipients = if role.is_some() {
            vec![]
        } else if launch.recipients.is_empty() {
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
                stream,
                intent,
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
                    made_from: None,
                })
                .collect(),
            current: None,
            submission: SubmissionHeld {
                before,
                ..SubmissionHeld::default()
            },
            failures: vec![],
            priority: launch.priority,
            shown: None,
            ends: vec![],
            role: role.map(str::to_string),
            unwoken: Default::default(),
            moves: vec![],
            holds: launch_holds(launch, self.host.now()),
            waiting: None,
            stall_after: launch.stall_after.unwrap_or(STALL_AFTER),
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
        let holds = held.hold_on(&first.name).is_some() || first.waits;
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

impl Engine {
    /// The stream a launch's place runs in, chosen once (8, 143): the
    /// launch's, else the stream of its intent's affinity where one
    /// stands (147), else the one the host's configuration names for the
    /// repository and `kind`, for the repository or for `kind`, else
    /// `default`; a stream the host does not declare refuses the launch
    /// naming it (4).
    fn stream_of(
        &self,
        launch: &Launch,
        kind: &str,
        intent: &IntentKey,
    ) -> Result<StreamHeld, String> {
        let stream = match &launch.stream {
            Some(name) => StreamHeld {
                name: name.clone(),
                from: StreamFrom::Launch,
            },
            None => match self.affinity(intent) {
                Some((_, affinity)) => StreamHeld {
                    name: affinity.stream,
                    from: StreamFrom::Intent,
                },
                None => self
                    .host
                    .stream_for(&launch.repository, kind)
                    .unwrap_or_else(|| StreamHeld {
                        name: DEFAULT_STREAM.into(),
                        from: StreamFrom::Default,
                    }),
            },
        };
        if self.host.streams().contains_key(&stream.name) {
            return Ok(stream);
        }
        let repository = &launch.repository;
        let named = match stream.from {
            StreamFrom::Launch => "the launch names".to_string(),
            StreamFrom::HostRepositoryKind => {
                format!("the host's configuration names for {kind} in {repository}")
            }
            StreamFrom::HostRepository => {
                format!("the host's configuration names for {repository}")
            }
            StreamFrom::HostKind => format!("the host's configuration names for {kind}"),
            StreamFrom::Intent => "the intent's affinity stands in".to_string(),
            StreamFrom::Default => "the place takes".to_string(),
        };
        Err(format!(
            "the stream {} {named} is not declared by the host's configuration (143)",
            stream.name
        ))
    }

    /// The intent a launch's place serves: the launch's name, else the
    /// repository and base, each with its document's digest (147).
    fn intent_of(&self, launch: &Launch) -> IntentKey {
        match &launch.intent {
            Some(name) => IntentKey::Launch {
                name: name.clone(),
                digest: sha256(&format!("name:{name}")),
            },
            None => IntentKey::Base {
                repository: launch.repository.clone(),
                base: launch.base.clone(),
                digest: sha256(&format!(
                    "base:{}:{}",
                    self.repository_id_of(&launch.repository),
                    launch.base
                )),
            },
        }
    }
}

/// The SHA-256 of `text`, in hex.
fn sha256(text: &str) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// The holds a launch puts, each stage's with its words, and the
/// submission's, since the launch (7, 47, 130).
fn launch_holds(launch: &Launch, now: Seconds) -> Vec<Hold> {
    let mut holds: Vec<Hold> = launch
        .hold
        .iter()
        .map(|stage| Hold {
            on: HoldOn::Stage {
                stage: stage.clone(),
            },
            words: launch.hold_words.get(stage).cloned(),
            since: now,
        })
        .collect();
    if launch.hold_submission {
        holds.push(Hold {
            on: HoldOn::Submission,
            words: None,
            since: now,
        });
    }
    holds
}

/// A launch's kind as its name and the version it names, if any:
/// `<name>` or `<name>@<version>`, the version a whole number (154, C.2).
fn named_version(kind: &str) -> Result<(&str, Option<u32>), String> {
    match kind.split_once('@') {
        None => Ok((kind, None)),
        Some((name, version)) => version
            .parse()
            .map(|version| (name, Some(version)))
            .map_err(|_| {
                format!("the kind {kind} names no version flume reads: {version} is not a whole number (154)")
            }),
    }
}
