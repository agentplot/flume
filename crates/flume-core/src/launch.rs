//! A launch (1 to 12, 19 to 23, 154 to 160): the kind its text names,
//! read by the kinds seam, resolved over the three declarations or from
//! the source it names, flume's own, the repository's, the launch's or one
//! the host declares, and checked against the locks where it is flume's
//! own or a host's source's, with where it came from kept on the place;
//! the binding read, the base's revision where the merge queue says it
//! lives, else at origin after a fetch, and where it was read kept for
//! status (9, 56, C.1), every refusal made before anything is made, then
//! one place made, readied, and its first stage started, held or left
//! waiting on readiness. The repository's settings for the kind
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
//! its kind, else `default`, and one the host does not declare, or whose
//! multiplexer names a terminal profile flume does not carry, refuses
//! the launch (4, 8, 143, 190); its intent is the launch's name, else its
//! repository and base (147). A base place, whose kind lands and has
//! no stages, is launched as any place until its held state is written,
//! and then has its branch opened by the merge queue and nothing else
//! made: no worktree, no readiness and no start (202, 205); and a launch
//! on the branch of a place that stands is refused unless that place is
//! a base place whose submission is not open and whose work is not over
//! (10, 204).

use crate::own::Table;
use crate::reading::reads_change;
use crate::settings::no_stage_lacking;
use crate::{Answer, Bound, Engine, Step};
use flume_seams::*;
use std::collections::BTreeMap;

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

    /// The kind a launch names, as the kinds seam reads its text (154;
    /// core.4), with the settings the repository gives a kind of its name
    /// it does not declare, which join whichever source the name resolves
    /// to (32, 158, C.2), those it gives every kind (195), and where it
    /// came from (19, 21). A source named
    /// is the only one searched: `flume` flume's own, `repo` the
    /// repository's declaration at the base's revision, `launch` the
    /// launch's declaration, and any other a source the host declares,
    /// whose fault refuses the launch before its packages are read (191).
    /// A name with no source resolves over the launch's declaration, the
    /// repository's and flume's own alone, never a host's source (154).
    /// The launch's and the repository's declarations hold one version of
    /// a name, and a version named that the one resolved does not hold
    /// refuses the launch; among flume's own and a host's source, the
    /// version named is taken, else the newest the repository's lock
    /// records, else the newest the host's lock records, else the newest
    /// held, and one not held refuses the launch naming those held (4, 22,
    /// 154). A kind of flume's own or of a host's source reads both locks,
    /// and is checked against each that records it; one of a host's
    /// source runs only where one does (155, 156; core.34). A host's
    /// source's package is read before its locks are checked, so a package
    /// weir refuses is named with its field whether or not a lock records
    /// it, and the hash checked is the one just read (156, 157).
    fn resolve_kind(&self, launch: &Launch, revision: &Revision) -> Result<Resolved, String> {
        let kinds = self.resolver.kinds();
        let named = kinds.read_named(&launch.kind).map_err(|reason| {
            format!(
                "the kind {} names no kind flume reads: {reason} (154)",
                launch.kind
            )
        })?;
        let source = match named.source.as_deref() {
            None => Search::Bare,
            Some(OWN_SOURCE) => Search::Own,
            Some(REPO_SOURCE) => Search::Repository,
            Some(LAUNCH_SOURCE) => Search::Launch,
            Some(source) => Search::Host(self.host_source(source, &named)?),
        };
        let (declared, mut given, every_kind) = match kinds.read_kinds(&launch.repository, revision)
        {
            KindsRead::Read {
                kinds,
                settings,
                agents,
            } => (kinds, settings, *agents),
            KindsRead::CouldNotRead { reason } => {
                return Err(format!(
                    "the kinds {} declares at {revision} could not be read: {reason}",
                    launch.repository
                ))
            }
        };
        let settings = given.remove(&named.name).unwrap_or_default();
        let resolved = |(kind, declared, from)| Resolved {
            kind,
            declared,
            settings: settings.clone(),
            every_kind: every_kind.clone(),
            from,
        };
        let repository = || {
            let by = format!("{} at {revision}", launch.repository);
            let from = self
                .declaration_from(REPO_SOURCE, &named, &declared)?
                .map(|from| KindFrom {
                    declaration_file: kinds.declaration_path(),
                    revision: Some(revision.clone()),
                    ..from
                });
            self.declared_kind(&named, &declared, &by, Declared::Repository, from)
        };
        let declaration = || -> Result<Option<_>, String> {
            let Some(file) = &launch.declaration else {
                return Ok(None);
            };
            let held = match kinds.read_declaration(file) {
                KindsRead::Read { kinds, .. } => kinds,
                KindsRead::CouldNotRead { reason } => {
                    return Err(format!(
                        "the launch's declaration {file} could not be read: {reason}"
                    ))
                }
            };
            let by = format!("the launch's declaration {file}");
            let from = self
                .declaration_from(LAUNCH_SOURCE, &named, &held)?
                .map(|from| KindFrom {
                    declaration_file: Some(file.clone()),
                    ..from
                });
            self.declared_kind(&named, &held, &by, Declared::Launch, from)
        };
        let found = match source {
            Search::Host(at) => {
                let (kind, from) = self.host_kind(&named, &at, &launch.repository, revision)?;
                (kind, Declared::Host, from)
            }
            Search::Own => self
                .own_kind(&named, &launch.repository, revision)?
                .ok_or_else(|| {
                    format!(
                        "the kind {} is not held: flume holds no {} (22, 154)",
                        named.spelled, named.name
                    )
                })?,
            Search::Repository => repository()?.ok_or_else(|| {
                format!(
                    "the kind {} is not held: {} declares no {} at {revision} (19, 154)",
                    named.spelled, launch.repository, named.name
                )
            })?,
            Search::Launch => match &launch.declaration {
                None => {
                    return Err(format!(
                        "the kind {} names the launch's declaration, and the launch carries none (19, 154)",
                        named.spelled
                    ))
                }
                Some(file) => declaration()?.ok_or_else(|| {
                    format!(
                        "the kind {} is not held: the launch's declaration {file} holds no {} (19, 154)",
                        named.spelled, named.name
                    )
                })?,
            },
            Search::Bare => match declaration()? {
                Some(found) => found,
                None => match repository()? {
                    Some(found) => found,
                    None => self
                        .own_kind(&named, &launch.repository, revision)?
                        .ok_or_else(|| self.resolves_to_none(launch, &named, revision))?,
                },
            },
        };
        Ok(resolved(found))
    }

    /// The kind of `named`'s name a declaration holds, at the version
    /// named where one is, with where it came from; none where it holds
    /// none of that name (19, 154).
    fn declared_kind(
        &self,
        named: &KindNamed,
        kinds: &[Kind],
        by: &str,
        declared: Declared,
        from: Option<KindFrom>,
    ) -> Result<Option<(Kind, Declared, KindFrom)>, String> {
        let (Some(kind), Some(from)) = (kinds.iter().find(|kind| kind.name == named.name), from)
        else {
            return Ok(None);
        };
        let resolves = if named.source.is_none() {
            ", and resolves the name"
        } else {
            ""
        };
        match named.version {
            Some(version) if version != kind.version => Err(format!(
                "the kind {} is not held: {by} declares {} at version {}{resolves} (19, 154)",
                named.spelled, kind.name, kind.version
            )),
            _ => Ok(Some((kind.clone(), declared, from))),
        }
    }

    /// Where a kind of a declaration of `source`'s came from, its identity
    /// spelled by the kinds seam (19; core.4); none where the declaration
    /// holds no kind of `named`'s name.
    fn declaration_from(
        &self,
        source: &str,
        named: &KindNamed,
        kinds: &[Kind],
    ) -> Result<Option<KindFrom>, String> {
        let Some(kind) = kinds.iter().find(|kind| kind.name == named.name) else {
            return Ok(None);
        };
        let identity = self
            .resolver
            .kinds()
            .identity_of(source, &kind.name, kind.version)
            .map_err(|reason| {
                format!(
                    "the kind {} has no identity flume reads: {reason} (154)",
                    named.spelled
                )
            })?;
        Ok(Some(KindFrom {
            identity,
            source: source.into(),
            source_file: None,
            kind_dir: None,
            declaration_file: None,
            revision: None,
            hash: None,
            lock_file: None,
        }))
    }

    /// flume's own kind of `named`'s name, at the version named, else the
    /// newest a lock records, else the newest flume holds, checked against
    /// each lock that records it (22, 154 to 156); none where flume holds
    /// none of that name.
    fn own_kind(
        &self,
        named: &KindNamed,
        repository: &str,
        revision: &Revision,
    ) -> Result<Option<(Kind, Declared, KindFrom)>, String> {
        let mut own: Vec<Packaged> = self
            .resolver
            .kinds()
            .flumes_own()
            .into_iter()
            .filter(|own| own.kind.name == named.name)
            .collect();
        own.sort_by_key(|own| own.identity.version);
        let Some(newest) = own.last() else {
            return Ok(None);
        };
        let source = newest.identity.source.clone();
        let locks = self.locks(repository, revision)?;
        let (version, wanted) = locks.version(named, &source, newest.identity.version);
        let versions = format!(
            "version{} {}",
            if own.len() > 1 { "s" } else { "" },
            own.iter()
                .map(|own| own.identity.version.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        let Some(package) = own.into_iter().find(|own| own.identity.version == version) else {
            return Err(format!(
                "the kind {wanted} is not held: flume holds {} at {versions} (22, 154)",
                named.name
            ));
        };
        let lock_file = locks.check(&package.identity, &package.hash, false)?;
        Ok(Some((
            package.kind,
            Declared::Flume,
            KindFrom {
                identity: package.identity,
                source,
                source_file: None,
                kind_dir: package.dir,
                declaration_file: None,
                revision: None,
                hash: Some(package.hash),
                lock_file,
            },
        )))
    }

    /// The source the host declares under `source`, with its file and its
    /// packages' directory; the launch refused where the host declares
    /// none of that name, or its file cannot be read, naming the file and
    /// why (4, 191).
    pub(crate) fn host_source(&self, source: &str, named: &KindNamed) -> Result<SourceAt, String> {
        let sources = self.host.sources();
        match sources.get(source).map(|declared| &declared.read) {
            None => {
                let file = match self.host.sources_dir() {
                    Some(dir) => format!("no file {dir}/{source}.toml"),
                    None => format!("no file {source}.toml among its sources"),
                };
                let declares = if sources.is_empty() {
                    "the host declares no source".to_string()
                } else {
                    format!(
                        "the host declares {}",
                        and_list(sources.keys().cloned().collect())
                    )
                };
                Err(format!(
                    "the kind {} names the source {source}, which the host does not declare: {file}; {declares} (4, 154, 191)",
                    named.spelled
                ))
            }
            Some(HostSourceRead::CouldNotRead { reason }) => Err(format!(
                "the kind {} names the source {source}, whose file {} cannot be read: {reason} (4, 191)",
                named.spelled, sources[source].file
            )),
            Some(HostSourceRead::Read { kinds_dir }) => Ok(SourceAt {
                name: source.into(),
                file: sources[source].file.clone(),
                kinds_dir: kinds_dir.clone(),
            }),
        }
    }

    /// The kind of `named`'s name the host's source `at` holds as a
    /// package, at the version named, else the newest a lock records, else
    /// the newest the source holds; refused naming what the source holds
    /// where it holds no such version, naming the package and its fields
    /// where its package cannot be read, and where no lock records it or
    /// its content is not the lock's (4, 154 to 159).
    pub(crate) fn host_kind(
        &self,
        named: &KindNamed,
        at: &SourceAt,
        repository: &str,
        revision: &Revision,
    ) -> Result<(Kind, KindFrom), String> {
        let (package, locks) = self.package_of(named, at, || self.locks(repository, revision))?;
        let lock_file = locks.check(&package.identity, &package.hash, true)?;
        Ok((
            package.kind,
            KindFrom {
                identity: package.identity,
                source: at.name.clone(),
                source_file: Some(at.file.clone()),
                kind_dir: package.dir,
                declaration_file: None,
                revision: None,
                hash: Some(package.hash),
                lock_file,
            },
        ))
    }

    /// The package of `named`'s name the host's source `at` holds, at the
    /// version named or the one `locks` picks, with the locks it read
    /// (154, 157, 159).
    pub(crate) fn package_of(
        &self,
        named: &KindNamed,
        at: &SourceAt,
        locks: impl FnOnce() -> Result<Locks, String>,
    ) -> Result<(Packaged, Locks), String> {
        let kinds = self.resolver.kinds();
        let every = match kinds.read_source(&at.name, &at.kinds_dir) {
            SourceRead::Read { kinds } => kinds,
            SourceRead::CouldNotRead { reason } => {
                return Err(format!(
                "the kind {} names the source {}, whose packages cannot be read: {reason} (4, 159)",
                named.spelled, at.name
            ))
            }
        };
        let holds = |what: &str| {
            let every: Vec<String> = every.iter().map(|kind| kind.spelled.clone()).collect();
            let holds = if every.is_empty() {
                "nothing".to_string()
            } else {
                and_list(every)
            };
            format!(
                "the source {} holds no {what}: it holds {holds} in {} (4, 154, 159)",
                at.name, at.kinds_dir
            )
        };
        let mut held: Vec<&KindIdentity> = every
            .iter()
            .filter(|kind| kind.name == named.name)
            .collect();
        held.sort_by_key(|kind| kind.version);
        let Some(newest) = held.last() else {
            return Err(holds(&named.name));
        };
        let locks = locks()?;
        let (version, wanted) = locks.version(named, &at.name, newest.version);
        if !held.iter().any(|kind| kind.version == version) {
            return Err(holds(&wanted));
        }
        match kinds.read_package(&at.name, &at.kinds_dir, &named.name, version) {
            PackageRead::Read { package } => Ok((*package, locks)),
            PackageRead::CouldNotRead { reason } => Err(format!(
                "the kind {wanted} could not be read: {reason} (4, 157, 158)"
            )),
        }
    }

    /// The two locks a launch whose kind is flume's own or a host's source
    /// reads: the repository's at the base's revision and the host's as it
    /// stands; a lock that cannot be read refuses the launch naming it and
    /// why (155).
    pub(crate) fn locks(&self, repository: &str, revision: &Revision) -> Result<Locks, String> {
        let kinds = self.resolver.kinds();
        let repository_file = kinds
            .lock_path()
            .unwrap_or_else(|| "the repository's lock".into());
        let at = match kinds.read_lock_at(repository, revision) {
            LockRead::None => vec![],
            LockRead::Read { entries } => entries,
            LockRead::CouldNotRead { reason } => {
                return Err(format!(
                    "the repository's lock {repository_file} at {revision} could not be read: {reason} (155)"
                ))
            }
        };
        let host_file = self.host.kinds_lock();
        let host = match &host_file {
            None => vec![],
            Some(file) => entries_of(kinds.read_lock_file(file), file)?,
        };
        Ok(Locks {
            repository: at,
            repository_file,
            revision: revision.clone(),
            host,
            host_file,
        })
    }

    /// Why a name no declaration holds resolves to nothing: what each of
    /// the three holds, and each identity of that name the host's sources
    /// hold, which a launch names by its source (19, 154).
    fn resolves_to_none(&self, launch: &Launch, named: &KindNamed, revision: &Revision) -> String {
        let carried = match &launch.declaration {
            Some(file) => format!("the launch's declaration {file} holds none"),
            None => "the launch carries none".into(),
        };
        let kinds = self.resolver.kinds();
        let mut hosted = vec![];
        for (source, declared) in self.host.sources() {
            if let HostSourceRead::Read { kinds_dir } = declared.read {
                if let SourceRead::Read { kinds } = kinds.read_source(&source, &kinds_dir) {
                    hosted.extend(
                        kinds
                            .into_iter()
                            .filter(|kind| kind.name == named.name)
                            .map(|kind| kind.spelled),
                    );
                }
            }
        }
        let hosted = if hosted.is_empty() {
            String::new()
        } else {
            format!(
                "; the host's sources hold {}, which a launch names by its source",
                and_list(hosted)
            )
        };
        format!(
            "the kind {} resolves to no declaration: {carried}, {} declares no {} at {revision}, flume holds none{hosted} (19, 154)",
            launch.kind, launch.repository, named.name
        )
    }

    /// The base's current revision and where it was read (9, 126, C.1):
    /// where the merge queue's profile says the base lives, where the
    /// merge queue lands it; else `origin/<base>` after git's fetch, so no
    /// copy the host holds is read while origin holds the base; else the
    /// host's own branch, where the repository has no `origin` or `origin`
    /// holds no such branch. A fetch that fails refuses the launch.
    fn base_now(
        &self,
        repository: &str,
        base: &str,
        queue: Option<&dyn MergeQueue>,
    ) -> Result<(Revision, BaseFrom), String> {
        let unread = |reason: String| {
            format!("the base {base}'s current revision could not be read: {reason}")
        };
        if let Some(queue) = queue {
            match queue.base_revision(repository, base) {
                BaseRevision::At { revision } => return Ok((revision, BaseFrom::MergeQueue)),
                BaseRevision::Refused { reason } => return Err(unread(reason)),
                BaseRevision::Unlanded => {}
            }
        }
        let from = match self.history.origin_branch(repository, base) {
            OriginBranch::At { revision } => return Ok((revision, BaseFrom::Origin)),
            OriginBranch::Refused { reason } => return Err(unread(reason)),
            OriginBranch::NoOrigin => BaseFrom::NoOrigin,
            OriginBranch::Lacks => BaseFrom::OriginLacks,
        };
        self.history
            .head(repository, base)
            .map(|revision| (revision, from))
            .ok_or_else(|| format!("{repository} has no base {base}"))
    }

    /// Refuses a kind that asks for more than flume runs (4, 23): a kind
    /// with no stages runs only where it lands, as a base place's (202).
    fn runs(kind: &Kind, identity: &KindIdentity) -> Result<(), String> {
        let named = format!("the kind {identity}");
        if kind.stages.is_empty() && !kind.lands {
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
        self.on_a_places_branch(repository, &launch.base)?;
        let (base_revision, base_from) =
            self.base_now(repository, &launch.base, merge_queue.as_deref())?;
        let Resolved {
            kind,
            declared,
            settings: repository_settings,
            every_kind: repository_agents,
            from,
        } = self.resolve_kind(launch, &base_revision)?;
        Self::runs(&kind, &from.identity)?;
        no_stage_lacking(&kind, &launch.settings, &repository_settings)?;
        // Every layer's own settings the launch read are checked before
        // anything is made or held, and kept as checked; the host's are
        // checked as each stage's agent is resolved below (195, core.8).
        let settings = launch
            .settings
            .iter()
            .map(|(stage, own)| {
                self.own_checked(own, &Table::stage(Layer::Launch, stage))
                    .map(|own| (stage.clone(), own))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        let every_stage =
            self.own_checked(&launch.every_stage, &Table::every_stage(Layer::Launch))?;
        let repository_settings = self.own_checked_kind(&repository_settings, Layer::Repository)?;
        let repository_agents = self.own_checked(
            &repository_agents,
            &Table::new(Layer::Repository, "for every kind"),
        )?;
        let kind = self.own_checked_declaration(&kind)?;
        let intent = self.intent_of(launch);
        let stream = self.stream_of(launch, &kind.name, &intent)?;
        // A stream whose multiplexer names a terminal profile the resolver
        // does not carry runs no agent: the launch is refused before
        // anything is made (4, 190; core.8).
        self.stream_terminal(&stream.name)?;
        let named = format!("the kind {}", from.identity);
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
        let location = place_seam
            .location(repository, &format!("{}--{}", kind.name, launch.name))
            .map_err(|reason| format!("the place's location could not be found: {reason}"))?;
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
            (Some(queue), true) => match crate::submit::on_base(
                &launch.base,
                queue.read_submission(repository, &branch),
            ) {
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
                .or_else(|| self.caller_session())
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
                kind_from: Some(from),
                name: launch.name.clone(),
                branch: branch.clone(),
                base: launch.base.clone(),
                base_revision: base_revision.clone(),
                base_from: Some(base_from),
                location: location.clone(),
                binding,
                brief: launch.brief.clone(),
                mark: launch.mark.clone(),
                settings,
                every_stage,
                repository_settings,
                repository_agents,
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
            unsettled: vec![],
            uncarried: vec![],
            terminals: vec![],
            holds: launch_holds(launch, self.host.now()),
            waiting: None,
            stall_after: launch.stall_after.unwrap_or(STALL_AFTER),
        };
        let programs = kind
            .stages
            .iter()
            .map(|stage| {
                self.resolution(&held.place, stage)
                    .map(|resolution| resolution.resolved.settings.program)
            })
            .collect::<Result<Vec<_>, _>>()?;
        for (stage, program) in kind.stages.iter().zip(&programs) {
            self.pool_runs(&held.place.stream.name, &stage.name, program.as_deref())?;
        }
        if self.create(&held).is_none() {
            return Err(format!("the place {id} already stands"));
        }
        if let (true, Some(queue)) = (crate::base::base_place(&kind), &merge_queue) {
            return self.open_base_place(&held, queue.as_ref());
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

/// A launch's kind resolved: the kind, the declaration it came from, the
/// settings the repository gives a kind of its name it does not declare,
/// and where it came from (19, 21, 32).
struct Resolved {
    kind: Kind,
    declared: Declared,
    settings: KindSettings,
    every_kind: Settings,
    from: KindFrom,
}

/// Where a kind named with its source, or with none, is searched (154).
enum Search {
    Bare,
    Own,
    Repository,
    Launch,
    Host(SourceAt),
}

/// A source the host declares, its file read: its name, its file and its
/// packages' directory (191, C.7).
pub(crate) struct SourceAt {
    pub name: String,
    pub file: String,
    pub kinds_dir: String,
}

/// The repository's lock at a revision and the host's, each with its file
/// and every entry it records, none where no lock lies (155).
pub(crate) struct Locks {
    repository: Vec<LockEntry>,
    repository_file: String,
    revision: Revision,
    host: Vec<LockEntry>,
    host_file: Option<String>,
}

impl Locks {
    /// No lock read, for an act that names its version whole and checks
    /// no lock (155).
    pub(crate) fn none() -> Locks {
        Locks {
            repository: vec![],
            repository_file: String::new(),
            revision: Revision::new(""),
            host: vec![],
            host_file: None,
        }
    }

    /// The version a launch of `named`, of `source`, takes, with the
    /// identity it is named by: the version named, else the newest the
    /// repository's lock records, else the newest the host's records, else
    /// `newest`, the newest held (154).
    fn version(&self, named: &KindNamed, source: &str, newest: u32) -> (u32, String) {
        if let Some(version) = named.version {
            return (version, named.spelled.clone());
        }
        let recorded = |entries: &[LockEntry], file: &str| {
            entries
                .iter()
                .filter(|entry| entry.kind.source == source && entry.kind.name == named.name)
                .max_by_key(|entry| entry.kind.version)
                .map(|entry| {
                    (
                        entry.kind.version,
                        format!("{}, the newest {file} records", entry.kind),
                    )
                })
        };
        let host_file = self.host_file.clone().unwrap_or_default();
        recorded(&self.repository, &self.repository_named())
            .or_else(|| recorded(&self.host, &format!("the host's lock {host_file}")))
            .unwrap_or_else(|| (newest, named.spelled.clone()))
    }

    /// The lock that records `identity` at `hash`, the repository's where
    /// it does, else the host's, none where neither does; refused where
    /// the two give it two hashes, where the one that records it gives
    /// another hash, and, for a kind from outside flume, where neither
    /// records it (155, 156; core.34).
    fn check(
        &self,
        identity: &KindIdentity,
        hash: &str,
        outside: bool,
    ) -> Result<Option<String>, String> {
        let at = |entries: &[LockEntry]| {
            entries
                .iter()
                .find(|entry| same(&entry.kind, identity))
                .map(|entry| entry.hash.clone())
        };
        let host_file = self.host_file.clone().unwrap_or_default();
        let repository = at(&self.repository);
        let host = at(&self.host);
        if let (Some(theirs), Some(hosts)) = (&repository, &host) {
            if theirs != hosts {
                return Err(format!(
                    "the locks give the kind {identity} two hashes: {} records {theirs}, and the host's lock {host_file} records {hosts} (155)",
                    self.repository_named()
                ));
            }
        }
        let recorded = repository
            .map(|recorded| {
                (
                    recorded,
                    self.repository_named(),
                    self.repository_file.clone(),
                )
            })
            .or_else(|| {
                host.map(|recorded| {
                    (
                        recorded,
                        format!("the host's lock {host_file}"),
                        host_file.clone(),
                    )
                })
            });
        match recorded {
            Some((recorded, named, _)) if recorded != hash => Err(format!(
                "the kind {identity}'s content is not what {named} records: it hashes to {hash}, and the lock records {recorded}; a change to a kind's content is a new version (156)"
            )),
            Some((_, _, file)) => Ok(Some(file)),
            None if outside => Err(format!(
                "the kind {identity} is recorded in no lock: record it by `flume kinds lock {identity}` in the repository, committed on its base, or by `flume kinds lock --host {identity}` (155; core.34)"
            )),
            None => Ok(None),
        }
    }

    /// The repository's lock as a refusal names it, at its revision.
    fn repository_named(&self) -> String {
        format!("{} at {}", self.repository_file, self.revision)
    }
}

/// Whether two identities name one kind at one version (154).
pub(crate) fn same(a: &KindIdentity, b: &KindIdentity) -> bool {
    a.source == b.source && a.name == b.name && a.version == b.version
}

/// The entries of the host's lock at `file`, none where none lies; why it
/// cannot be read where it cannot (155).
pub(crate) fn entries_of(read: LockRead, file: &str) -> Result<Vec<LockEntry>, String> {
    match read {
        LockRead::None => Ok(vec![]),
        LockRead::Read { entries } => Ok(entries),
        LockRead::CouldNotRead { reason } => Err(format!(
            "the host's lock {file} could not be read: {reason} (155)"
        )),
    }
}

/// `items` as a sentence lists them: `a`, `a and b`, `a, b and c`.
pub(crate) fn and_list(mut items: Vec<String>) -> String {
    match items.len() {
        0 => String::new(),
        1 => items.remove(0),
        _ => {
            let last = items.pop().unwrap_or_default();
            format!("{} and {last}", items.join(", "))
        }
    }
}
