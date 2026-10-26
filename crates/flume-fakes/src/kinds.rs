use crate::{lock, Log};
use flume_seams::{Binding, BindingRead, Kind, Kinds, KindsRead, Revision};
use std::collections::BTreeMap;
use std::sync::Mutex;

/// The kinds seam: each repository's kinds, at a revision or at every
/// revision, the declarations launches carry, flume's own, and each
/// repository's binding.
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
    own: Vec<Kind>,
    bindings: BTreeMap<String, BindingRead>,
}

impl FakeKinds {
    /// The kinds `repository` declares at every revision not set apart.
    pub fn set_kinds(&self, repository: &str, kinds: Vec<Kind>) {
        lock(&self.state)
            .every
            .insert(repository.into(), KindsRead::Read { kinds });
    }

    /// The kinds `repository` declares at `revision`.
    pub fn set_kinds_at(&self, repository: &str, revision: &Revision, kinds: Vec<Kind>) {
        lock(&self.state).at.insert(
            (repository.into(), revision.clone()),
            KindsRead::Read { kinds },
        );
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
            .insert(declaration.into(), KindsRead::Read { kinds });
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
        state
            .at
            .get(&(repository.to_string(), revision.clone()))
            .or_else(|| state.every.get(repository))
            .cloned()
            .unwrap_or(KindsRead::Read { kinds: vec![] })
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

    fn flumes_own(&self) -> Vec<Kind> {
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
