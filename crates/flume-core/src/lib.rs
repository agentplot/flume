//! flume's one core (60): a launch makes one place and starts, holds or
//! leaves waiting its first stage; each stage starts one fresh agent with
//! its prompt filled; a stage ends only on its deliverable read at one
//! head, its agent's stop-short or a conversation's finish; a settle
//! moves a place on and submits a landing kind's place; the caller's acts
//! and status answer as Part A says; and a tidy removes what is over.
//! The engine reaches every tool through the seams alone (core.1, core.2)
//! and keeps its state in the store (core.16).

mod acts;
mod answer;
mod held;
mod launch;
mod prompt;
mod reading;
mod start;
mod status;

pub use acts::Retry;
pub use answer::*;
pub use held::Step;

use flume_seams::{
    Agent, Binding, Change, History, Host, MergeQueue, Place, PlaceRecord, Resolver, Seconds,
    Settings, Store, Terminal,
};
use std::sync::Arc;

/// A stage's quiet limit where its declaration sets none (44).
pub const QUIET_LIMIT: Seconds = 30 * 60;

/// How long after each sending flume reads whether a prompt was
/// received (38).
pub const RECEIVED_WITHIN: Seconds = 60;

/// How long a tell waits for its agent where its caller gives no bound
/// (54).
pub const TELL_BOUND: Seconds = 10 * 60;

/// How often a command that waits on an agent reads it again.
pub const POLL: Seconds = 5;

/// What the statement leaves to flume's own default and the engine is
/// built with, holding no value of its own (design 10): the agent's
/// settings where neither launch nor stage gives them (32), and the
/// send-backs a kind allows where it sets none (23, 112).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Defaults {
    pub settings: Settings,
    pub send_backs: u32,
}

/// The engine, built for one command over a resolver from profile names
/// to seams, the host's terminal and store, git's history and the host
/// (design 2, 3, 10).
pub struct Engine {
    resolver: Arc<dyn Resolver>,
    terminal: Arc<dyn Terminal>,
    store: Arc<dyn Store>,
    history: Arc<dyn History>,
    host: Arc<dyn Host>,
    defaults: Defaults,
}

/// The seams a place's binding resolved to (105).
pub(crate) struct Bound {
    pub place: Arc<dyn Place>,
    pub change: Option<Arc<dyn Change>>,
    pub merge_queue: Option<Arc<dyn MergeQueue>>,
}

impl Engine {
    pub fn new(
        resolver: Arc<dyn Resolver>,
        terminal: Arc<dyn Terminal>,
        store: Arc<dyn Store>,
        history: Arc<dyn History>,
        host: Arc<dyn Host>,
        defaults: Defaults,
    ) -> Engine {
        Engine {
            resolver,
            terminal,
            store,
            history,
            host,
            defaults,
        }
    }

    pub fn defaults(&self) -> &Defaults {
        &self.defaults
    }

    /// The seams a binding names (105, core.3): an unbound place seam is
    /// the one that needs nothing beyond git, and an unbound change or
    /// merge queue stays unbound.
    pub(crate) fn bind(&self, binding: &Binding) -> Result<Bound, String> {
        let place = self
            .resolver
            .place(binding.place.as_deref())
            .ok_or_else(|| match &binding.place {
                Some(name) => format!("no place profile is named {name}"),
                None => "no place profile needs nothing beyond git".to_string(),
            })?;
        let change = match &binding.change {
            Some(name) => Some(
                self.resolver
                    .change(name)
                    .ok_or_else(|| format!("no change profile is named {name}"))?,
            ),
            None => None,
        };
        let merge_queue = match &binding.merge_queue {
            Some(name) => Some(
                self.resolver
                    .merge_queue(name)
                    .ok_or_else(|| format!("no merge queue profile is named {name}"))?,
            ),
            None => None,
        };
        Ok(Bound {
            place,
            change,
            merge_queue,
        })
    }

    /// The seams of a place's binding as its launch resolved it (21).
    pub(crate) fn bound(&self, place: &PlaceRecord) -> Result<Bound, String> {
        self.bind(&place.binding)
    }

    /// The agent profile a stage's settings name by its program (32,
    /// 105).
    pub(crate) fn agent(&self, program: &str) -> Option<Arc<dyn Agent>> {
        self.resolver.agent(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flume_fakes::*;
    use flume_seams::{Binding, Declared, Kind, PlaceId, Readiness, Revision};
    use std::collections::BTreeMap;

    fn record(binding: Binding) -> PlaceRecord {
        PlaceRecord {
            id: PlaceId::new("/r/places/fix--one"),
            repository: "/r".into(),
            kind: Kind {
                name: "fix".into(),
                version: 1,
                branch_prefix: "fix".into(),
                starts_unready: true,
                lands: true,
                stages: vec![],
            },
            declared: Declared::Flume,
            name: "one".into(),
            branch: "fix/one".into(),
            base: "main".into(),
            base_revision: Revision::new("c1"),
            location: "/r/places/fix--one".into(),
            binding,
            brief: None,
            mark: None,
            settings: BTreeMap::new(),
            hold: vec![],
            hold_submission: false,
        }
    }

    #[test]
    fn the_engine_is_built_over_the_resolver_and_the_hosts_seams() {
        let git = FakeGit::new();
        let host = Arc::new(FakeHost::default());
        let kinds = Arc::new(FakeKinds::default());
        let resolver = Arc::new(FakeResolver::new(kinds));
        let place = Arc::new(FakePlace::new(git.clone()));
        resolver.bind_no_tool_place(place.clone());
        resolver.bind_place("trees", place);
        resolver.bind_merge_queue("queue", Arc::new(FakeMergeQueue::new(git.clone(), None)));
        let defaults = Defaults {
            settings: Settings {
                program: Some("agent".into()),
                ..Settings::default()
            },
            send_backs: 2,
        };
        let engine = Engine::new(
            resolver,
            Arc::new(FakeTerminal::new(host.clone())),
            Arc::new(FakeStore::default()),
            Arc::new(FakeHistory::new(git)),
            host,
            defaults.clone(),
        );
        assert_eq!(engine.defaults(), &defaults);
        let unbound = engine.bound(&record(Binding::default())).ok().unwrap();
        assert!(unbound.change.is_none() && unbound.merge_queue.is_none());
        let bound = engine
            .bound(&record(Binding {
                place: Some("trees".into()),
                change: None,
                merge_queue: Some("queue".into()),
            }))
            .ok()
            .unwrap();
        assert!(bound.merge_queue.is_some());
        let unknown = engine.bound(&record(Binding {
            place: Some("other".into()),
            ..Binding::default()
        }));
        assert!(unknown.is_err());
        assert!(engine.agent("agent").is_none());
        let _ = Readiness::default();
    }
}
