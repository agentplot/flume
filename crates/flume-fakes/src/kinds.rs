use crate::{lock, Log};
use flume_seams::{
    Binding, BindingRead, Kind, KindIdentity, KindSettings, Kinds, KindsRead, Packaged, Revision,
};
use std::collections::BTreeMap;
use std::sync::Mutex;

/// The kinds seam: each repository's kinds and the settings it gives
/// kinds it does not declare, at a revision or at every revision, the
/// declarations launches carry, flume's own, and each repository's
/// binding.
#[derive(Default)]
pub struct FakeKinds {
    state: Mutex<KindsState>,
    pub log: Log,
}

#[derive(Default)]
struct KindsState {
    at: BTreeMap<(String, Revision), KindsRead>,
    every: BTreeMap<String, KindsRead>,
    declarations: BTreeMap<String, KindsRead>,
    /// Settings by repository, the revision they are set apart at or
    /// none for every other, and the kind's name.
    settings: BTreeMap<(String, Option<Revision>), BTreeMap<String, KindSettings>>,
    own: Vec<Packaged>,
    bindings: BTreeMap<String, BindingRead>,
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
        match read {
            KindsRead::Read { kinds, .. } => KindsRead::Read {
                kinds,
                settings: given,
            },
            refused => refused,
        }
    }

    fn read_declaration(&self, declaration: &str) -> KindsRead {
        self.log.push(format!("read_declaration {declaration}"));
        lock(&self.state)
            .declarations
            .get(declaration)
            .cloned()
            .unwrap_or(KindsRead::CouldNotRead {
                reason: format!("no declaration at {declaration}"),
            })
    }

    fn flumes_own(&self) -> Vec<Packaged> {
        self.log.push("flumes_own");
        lock(&self.state).own.clone()
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
}

/// The identity `<source>/<name>@<version>`, spelled as the fake spells
/// it, standing in for the profile's reader.
pub fn identity(source: &str, name: &str, version: u32) -> KindIdentity {
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
        identity: identity(source, &kind.name, kind.version),
        kind,
        hash,
    }
}

fn stand_in_hash(kind: &Kind) -> String {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    format!("{kind:?}").hash(&mut hasher);
    format!("sha256:{:064x}", hasher.finish())
}

/// A declaration read holding `kinds` and no settings for others.
fn read(kinds: Vec<Kind>) -> KindsRead {
    KindsRead::Read {
        kinds,
        settings: BTreeMap::new(),
    }
}
