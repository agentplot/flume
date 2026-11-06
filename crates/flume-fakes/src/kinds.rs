use crate::{lock, Log};
use flume_seams::{
    Binding, BindingRead, Kind, KindIdentity, KindNamed, KindSettings, Kinds, KindsRead, LockEntry,
    LockRead, PackageRead, Packaged, Revision, Settings, SourceRead,
};
use std::collections::BTreeMap;
use std::sync::Mutex;

/// The kinds seam: each repository's kinds, the settings it gives kinds
/// it does not declare and every kind, at a revision or at every
/// revision, the declarations launches carry, flume's own as packages
/// with their hashes, each repository's binding, the names it reads,
/// each source's directory with its packages by name and version, the
/// repository's lock at a revision or at every revision, each lock on
/// disk, which a lock written replaces, and the label a kind of a name
/// declares wherever it is read (180).
#[derive(Default)]
pub struct FakeKinds {
    state: Mutex<KindsState>,
    pub log: Log,
}

/// A source's directory as scripted: each package it holds by name and
/// version, a kind with its hash or why read-package refuses it; or why
/// the directory cannot be listed.
#[derive(Clone)]
enum Directory {
    Holds(BTreeMap<(String, u32), Result<(Kind, String), String>>),
    Unreadable(String),
}

#[derive(Default)]
struct KindsState {
    at: BTreeMap<(String, Revision), KindsRead>,
    every: BTreeMap<String, KindsRead>,
    declarations: BTreeMap<String, KindsRead>,
    /// Settings by repository, the revision they are set apart at or
    /// none for every other, and the kind's name.
    settings: BTreeMap<(String, Option<Revision>), BTreeMap<String, KindSettings>>,
    /// The settings a repository gives every kind run in it, its
    /// `[agents]`, by repository and the revision they are set apart at
    /// or none for every other (195).
    every_kind: BTreeMap<(String, Option<Revision>), Settings>,
    own: Vec<Packaged>,
    bindings: BTreeMap<String, BindingRead>,
    /// The texts read-named refuses, with why.
    unnamed: BTreeMap<String, String>,
    /// Each source's directory, by its path.
    directories: BTreeMap<String, Directory>,
    /// The repository's lock at a revision, and at every revision not
    /// set apart.
    locks_at: BTreeMap<(String, Revision), LockRead>,
    locks: BTreeMap<String, LockRead>,
    /// Each lock on disk, by its path.
    lock_files: BTreeMap<String, LockRead>,
    /// The paths a lock's write fails at, with why.
    unwritable: BTreeMap<String, String>,
    /// The label each kind of a name declares, by the kind's name.
    labels: BTreeMap<String, String>,
}

impl KindsState {
    /// `read` with each kind given the label its name is scripted with.
    fn labelled(&self, read: KindsRead) -> KindsRead {
        match read {
            KindsRead::Read {
                kinds,
                settings,
                agents,
            } => KindsRead::Read {
                kinds: kinds.into_iter().map(|kind| self.label(kind)).collect(),
                settings,
                agents,
            },
            refused => refused,
        }
    }

    /// `kind` with the label its name is scripted with, where it is.
    fn label(&self, mut kind: Kind) -> Kind {
        if let Some(label) = self.labels.get(&kind.name) {
            kind.label = Some(label.clone());
        }
        kind
    }
}

impl FakeKinds {
    /// The kinds `repository` declares at every revision not set apart.
    pub fn set_kinds(&self, repository: &str, kinds: Vec<Kind>) {
        lock(&self.state)
            .every
            .insert(repository.into(), read(kinds));
    }

    /// The kinds `repository` declares at `revision`.
    pub fn set_kinds_at(&self, repository: &str, revision: &Revision, kinds: Vec<Kind>) {
        lock(&self.state)
            .at
            .insert((repository.into(), revision.clone()), read(kinds));
    }

    pub fn unreadable_kinds(&self, repository: &str, reason: &str) {
        lock(&self.state).every.insert(
            repository.into(),
            KindsRead::CouldNotRead {
                reason: reason.into(),
            },
        );
    }

    pub fn set_declaration(&self, declaration: &str, kinds: Vec<Kind>) {
        lock(&self.state)
            .declarations
            .insert(declaration.into(), read(kinds));
    }

    /// The declaration at `declaration` cannot be read, for `reason`.
    pub fn unreadable_declaration(&self, declaration: &str, reason: &str) {
        lock(&self.state).declarations.insert(
            declaration.into(),
            KindsRead::CouldNotRead {
                reason: reason.into(),
            },
        );
    }

    /// The settings `repository` gives `kind`, which it does not declare,
    /// at every revision not set apart.
    pub fn set_settings(&self, repository: &str, kind: &str, settings: KindSettings) {
        lock(&self.state)
            .settings
            .entry((repository.into(), None))
            .or_default()
            .insert(kind.into(), settings);
    }

    /// The settings `repository` gives `kind` at `revision`.
    pub fn set_settings_at(
        &self,
        repository: &str,
        revision: &Revision,
        kind: &str,
        settings: KindSettings,
    ) {
        lock(&self.state)
            .settings
            .entry((repository.into(), Some(revision.clone())))
            .or_default()
            .insert(kind.into(), settings);
    }

    /// The settings `repository` gives every kind run in it, at every
    /// revision not set apart (32, 195).
    pub fn set_every_kind(&self, repository: &str, settings: Settings) {
        lock(&self.state)
            .every_kind
            .insert((repository.into(), None), settings);
    }

    /// The settings `repository` gives every kind at `revision`.
    pub fn set_every_kind_at(&self, repository: &str, revision: &Revision, settings: Settings) {
        lock(&self.state)
            .every_kind
            .insert((repository.into(), Some(revision.clone())), settings);
    }

    /// The label every kind named `kind` declares, wherever it is read,
    /// or the one each was set with where none is given (180).
    pub fn set_label(&self, kind: &str, label: Option<&str>) {
        let mut state = lock(&self.state);
        match label {
            Some(label) => state.labels.insert(kind.into(), label.into()),
            None => state.labels.remove(kind),
        };
    }

    /// flume's own kinds, each a package at `flume/<name>@<version>` with
    /// a stand-in hash of its content, the same for the same kind.
    pub fn set_own(&self, kinds: Vec<Kind>) {
        lock(&self.state).own = kinds
            .into_iter()
            .map(|kind| packaged("flume", kind, None))
            .collect();
    }

    /// flume's own kinds as the packages given, their hashes among them.
    pub fn set_own_packages(&self, packages: Vec<Packaged>) {
        lock(&self.state).own = packages;
    }

    pub fn set_binding(&self, repository: &str, binding: Binding) {
        lock(&self.state)
            .bindings
            .insert(repository.into(), BindingRead::Read { binding });
    }

    pub fn unreadable_binding(&self, repository: &str, reason: &str) {
        lock(&self.state).bindings.insert(
            repository.into(),
            BindingRead::CouldNotRead {
                reason: reason.into(),
            },
        );
    }

    /// read-named refuses `text`, for `reason`; every other text is read
    /// as the fake reads it.
    pub fn unreadable_named(&self, text: &str, reason: &str) {
        lock(&self.state).unnamed.insert(text.into(), reason.into());
    }

    /// The directory `kinds_dir` holds `kind` as the package
    /// `<name>@<version>` at `hash`, in place of any it held there.
    pub fn set_package(&self, kinds_dir: &str, kind: Kind, hash: &str) {
        let key = (kind.name.clone(), kind.version);
        self.package_at(kinds_dir, key, Ok((kind, hash.into())));
    }

    /// The directory `kinds_dir` holds the package `<name>@<version>`,
    /// which read-package refuses for `reason`.
    pub fn refuse_package(&self, kinds_dir: &str, name: &str, version: u32, reason: &str) {
        self.package_at(kinds_dir, (name.into(), version), Err(reason.into()));
    }

    /// The directory `kinds_dir` holds no package `<name>@<version>`.
    pub fn remove_package(&self, kinds_dir: &str, name: &str, version: u32) {
        if let Some(Directory::Holds(packages)) = lock(&self.state).directories.get_mut(kinds_dir) {
            packages.remove(&(name.to_string(), version));
        }
    }

    /// The directory `kinds_dir` cannot be listed, for `reason`.
    pub fn unreadable_source(&self, kinds_dir: &str, reason: &str) {
        lock(&self.state)
            .directories
            .insert(kinds_dir.into(), Directory::Unreadable(reason.into()));
    }

    fn package_at(
        &self,
        kinds_dir: &str,
        key: (String, u32),
        package: Result<(Kind, String), String>,
    ) {
        let mut state = lock(&self.state);
        let directory = state
            .directories
            .entry(kinds_dir.into())
            .or_insert_with(|| Directory::Holds(BTreeMap::new()));
        if let Directory::Unreadable(_) = directory {
            *directory = Directory::Holds(BTreeMap::new());
        }
        if let Directory::Holds(packages) = directory {
            packages.insert(key, package);
        }
    }

    /// `repository`'s lock records `entries` at every revision not set
    /// apart.
    pub fn set_lock(&self, repository: &str, entries: Vec<LockEntry>) {
        lock(&self.state)
            .locks
            .insert(repository.into(), LockRead::Read { entries });
    }

    /// `repository`'s lock at `revision` records `entries`.
    pub fn set_lock_at(&self, repository: &str, revision: &Revision, entries: Vec<LockEntry>) {
        lock(&self.state).locks_at.insert(
            (repository.into(), revision.clone()),
            LockRead::Read { entries },
        );
    }

    /// `repository`'s lock at `revision` cannot be read, for `reason`.
    pub fn unreadable_lock_at(&self, repository: &str, revision: &Revision, reason: &str) {
        lock(&self.state).locks_at.insert(
            (repository.into(), revision.clone()),
            LockRead::CouldNotRead {
                reason: reason.into(),
            },
        );
    }

    /// The lock on disk at `path` records `entries`.
    pub fn set_lock_file(&self, path: &str, entries: Vec<LockEntry>) {
        lock(&self.state)
            .lock_files
            .insert(path.into(), LockRead::Read { entries });
    }

    /// The lock on disk at `path` cannot be read, for `reason`.
    pub fn unreadable_lock_file(&self, path: &str, reason: &str) {
        lock(&self.state).lock_files.insert(
            path.into(),
            LockRead::CouldNotRead {
                reason: reason.into(),
            },
        );
    }

    /// A lock written at `path` fails, for `reason`, and leaves what lies
    /// there as it was.
    pub fn unwritable_lock(&self, path: &str, reason: &str) {
        lock(&self.state)
            .unwritable
            .insert(path.into(), reason.into());
    }
}

impl Kinds for FakeKinds {
    fn read_kinds(&self, repository: &str, revision: &Revision) -> KindsRead {
        self.log.push(format!("read_kinds {repository} {revision}"));
        let state = lock(&self.state);
        let read = state
            .at
            .get(&(repository.to_string(), revision.clone()))
            .or_else(|| state.every.get(repository))
            .cloned()
            .unwrap_or_else(|| read(vec![]));
        let given = state
            .settings
            .get(&(repository.to_string(), Some(revision.clone())))
            .or_else(|| state.settings.get(&(repository.to_string(), None)))
            .cloned()
            .unwrap_or_default();
        let every_kind = state
            .every_kind
            .get(&(repository.to_string(), Some(revision.clone())))
            .or_else(|| state.every_kind.get(&(repository.to_string(), None)))
            .cloned();
        state.labelled(match read {
            KindsRead::Read { kinds, agents, .. } => KindsRead::Read {
                kinds,
                settings: given,
                agents: every_kind.unwrap_or(agents),
            },
            refused => refused,
        })
    }

    fn read_declaration(&self, declaration: &str) -> KindsRead {
        self.log.push(format!("read_declaration {declaration}"));
        let state = lock(&self.state);
        state.labelled(state.declarations.get(declaration).cloned().unwrap_or(
            KindsRead::CouldNotRead {
                reason: format!("no declaration at {declaration}"),
            },
        ))
    }

    fn flumes_own(&self) -> Vec<Packaged> {
        self.log.push("flumes_own");
        let state = lock(&self.state);
        state
            .own
            .iter()
            .cloned()
            .map(|package| Packaged {
                kind: state.label(package.kind),
                ..package
            })
            .collect()
    }

    fn read_binding(&self, repository: &str) -> BindingRead {
        self.log.push(format!("read_binding {repository}"));
        lock(&self.state)
            .bindings
            .get(repository)
            .cloned()
            .unwrap_or(BindingRead::Read {
                binding: Binding::default(),
            })
    }

    fn read_named(&self, text: &str) -> Result<KindNamed, String> {
        self.log.push(format!("read_named {text}"));
        if let Some(reason) = lock(&self.state).unnamed.get(text) {
            return Err(reason.clone());
        }
        named(text).ok_or_else(|| format!("`{text}` names no kind"))
    }

    fn identity_of(&self, source: &str, name: &str, version: u32) -> Result<KindIdentity, String> {
        self.log
            .push(format!("identity_of {source} {name}@{version}"));
        Ok(kind_identity(source, name, version))
    }

    fn declaration_path(&self) -> Option<String> {
        Some(DECLARATION.into())
    }

    fn lock_path(&self) -> Option<String> {
        Some(LOCK.into())
    }

    fn read_source(&self, source: &str, kinds_dir: &str) -> SourceRead {
        self.log.push(format!("read_source {source} {kinds_dir}"));
        match lock(&self.state).directories.get(kinds_dir) {
            Some(Directory::Holds(packages)) => SourceRead::Read {
                kinds: packages
                    .keys()
                    .map(|(name, version)| kind_identity(source, name, *version))
                    .collect(),
            },
            Some(Directory::Unreadable(reason)) => SourceRead::CouldNotRead {
                reason: reason.clone(),
            },
            None => SourceRead::CouldNotRead {
                reason: format!("{kinds_dir} does not exist"),
            },
        }
    }

    fn read_package(&self, source: &str, kinds_dir: &str, name: &str, version: u32) -> PackageRead {
        self.log.push(format!(
            "read_package {source} {kinds_dir} {name}@{version}"
        ));
        let refused = |reason: String| PackageRead::CouldNotRead { reason };
        let state = lock(&self.state);
        match state.directories.get(kinds_dir) {
            Some(Directory::Holds(packages)) => match packages.get(&(name.to_string(), version)) {
                Some(Ok((kind, hash))) => PackageRead::Read {
                    package: Box::new(Packaged {
                        dir: Some(package_dir(kinds_dir, name, version)),
                        ..packaged(source, state.label(kind.clone()), Some(hash))
                    }),
                },
                Some(Err(reason)) => refused(reason.clone()),
                None => refused(format!("{kinds_dir} holds no {name}@{version}")),
            },
            Some(Directory::Unreadable(reason)) => refused(reason.clone()),
            None => refused(format!("{kinds_dir} does not exist")),
        }
    }

    fn read_lock_at(&self, repository: &str, revision: &Revision) -> LockRead {
        self.log
            .push(format!("read_lock_at {repository} {revision}"));
        let state = lock(&self.state);
        state
            .locks_at
            .get(&(repository.to_string(), revision.clone()))
            .or_else(|| state.locks.get(repository))
            .cloned()
            .unwrap_or(LockRead::None)
    }

    fn read_lock_file(&self, path: &str) -> LockRead {
        self.log.push(format!("read_lock_file {path}"));
        lock(&self.state)
            .lock_files
            .get(path)
            .cloned()
            .unwrap_or(LockRead::None)
    }

    fn write_lock(&self, path: &str, entries: &[LockEntry]) -> Result<(), String> {
        let spelled: Vec<String> = entries
            .iter()
            .map(|entry| format!("{}={}", entry.kind, entry.hash))
            .collect();
        self.log
            .push(format!("write_lock {path} {}", spelled.join(",")));
        let mut state = lock(&self.state);
        if let Some(reason) = state.unwritable.get(path) {
            return Err(reason.clone());
        }
        let mut entries = entries.to_vec();
        entries.sort_by(|a, b| a.kind.cmp(&b.kind));
        state
            .lock_files
            .insert(path.into(), LockRead::Read { entries });
        Ok(())
    }
}

/// `text`'s parts as the fake reads a kind's name, standing in for the
/// profile's reader: a source before a `/`, a version after an `@`, each
/// part not empty and a version a whole number of at least 1.
fn named(text: &str) -> Option<KindNamed> {
    let (source, rest) = match text.split_once('/') {
        Some((source, rest)) => (Some(source), rest),
        None => (None, text),
    };
    let (name, version) = match rest.split_once('@') {
        Some((name, version)) => (name, Some(version.parse::<u32>().ok().filter(|v| *v > 0)?)),
        None => (rest, None),
    };
    let word = |part: &str| !part.is_empty() && !part.contains(['/', '@']);
    if !word(name) || !source.is_none_or(word) {
        return None;
    }
    Some(KindNamed {
        source: source.map(str::to_string),
        name: name.into(),
        version,
        spelled: text.into(),
    })
}

/// The identity `<source>/<name>@<version>`, spelled as the fake spells
/// it, standing in for the profile's reader.
pub fn kind_identity(source: &str, name: &str, version: u32) -> KindIdentity {
    KindIdentity {
        source: source.into(),
        name: name.into(),
        version,
        spelled: format!("{source}/{name}@{version}"),
    }
}

/// `kind` as a package of `source`, at `hash` where one is given, else at
/// a stand-in hash of its content: `sha256:` and 64 hex digits that are
/// no sha256, the same for the same kind.
pub fn packaged(source: &str, kind: Kind, hash: Option<&str>) -> Packaged {
    let hash = hash.map_or_else(|| stand_in_hash(&kind), str::to_string);
    Packaged {
        identity: kind_identity(source, &kind.name, kind.version),
        kind,
        hash,
        dir: None,
    }
}

/// The directory of the package `<name>@<version>` in `kinds_dir`, as the
/// fake reads it (159).
pub fn package_dir(kinds_dir: &str, name: &str, version: u32) -> String {
    format!("{kinds_dir}/{name}@{version}")
}

/// Where the fake reads a repository's declaration and its lock in its
/// tree (20, 155).
pub const DECLARATION: &str = ".config/flume.toml";
pub const LOCK: &str = ".config/flume.lock";

fn stand_in_hash(kind: &Kind) -> String {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    format!("{kind:?}").hash(&mut hasher);
    format!("sha256:{:064x}", hasher.finish())
}

/// A declaration read holding `kinds` and no settings for others or for
/// every kind.
fn read(kinds: Vec<Kind>) -> KindsRead {
    KindsRead::Read {
        kinds,
        settings: BTreeMap::new(),
        agents: Settings::default(),
    }
}
