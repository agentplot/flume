use crate::{lock, Log};
use flume_seams::{Binding, BindingRead, Kind, KindSettings, Kinds, KindsRead, Revision, Settings};
use std::collections::BTreeMap;
use std::sync::Mutex;

/// The kinds seam: each repository's kinds, the settings it gives kinds
/// it does not declare and every kind, at a revision or at every revision, the
/// declarations launches carry, flume's own, each repository's binding,
/// and the label a kind of a name declares wherever it is read (180).
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
    /// The settings a repository gives every kind run in it, its
    /// `[agents]`, by repository and the revision they are set apart at
    /// or none for every other (191).
    every_kind: BTreeMap<(String, Option<Revision>), Settings>,
    own: Vec<Kind>,
    bindings: BTreeMap<String, BindingRead>,
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
    /// revision not set apart (32, 191).
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

    pub fn set_own(&self, kinds: Vec<Kind>) {
        lock(&self.state).own = kinds;
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

    fn flumes_own(&self) -> Vec<Kind> {
        self.log.push("flumes_own");
        let state = lock(&self.state);
        state
            .own
            .iter()
            .cloned()
            .map(|kind| state.label(kind))
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
