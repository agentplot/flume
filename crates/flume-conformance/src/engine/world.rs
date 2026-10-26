//! The fakes an engine scenario runs over, bound for one repository, and
//! the engine built over them.

use crate::fake_harness::flumes_own;
use flume_core::{Defaults, Engine};
use flume_fakes::*;
use flume_seams::*;
use std::sync::Arc;

pub const REPOSITORY: &str = "/repository";

/// Every fake, the resolver over them, and an engine for one command.
pub struct World {
    pub git: Arc<FakeGit>,
    pub host: Arc<FakeHost>,
    pub place: Arc<FakePlace>,
    pub kinds: Arc<FakeKinds>,
    pub change: Arc<FakeChange>,
    pub terminal: Arc<FakeTerminal>,
    pub agent: Arc<FakeAgent>,
    pub queue: Arc<FakeMergeQueue>,
    pub store: Arc<FakeStore>,
    pub history: Arc<FakeHistory>,
    pub resolver: Arc<FakeResolver>,
    pub defaults: Defaults,
    pub engine: Engine,
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}

impl World {
    /// A repository on `main` binding the fake place, change and merge
    /// queue, flume's own kinds, and agents run by the fake agent.
    pub fn new() -> World {
        let git = FakeGit::new();
        git.init(REPOSITORY, "main");
        let host = Arc::new(FakeHost::default());
        let place = Arc::new(FakePlace::new(git.clone()));
        let kinds = Arc::new(FakeKinds::default());
        kinds.set_own(flumes_own());
        kinds.set_binding(
            REPOSITORY,
            Binding {
                place: Some("trees".into()),
                change: Some("plans".into()),
                merge_queue: Some("queue".into()),
            },
        );
        let change = Arc::new(FakeChange::new(git.clone()));
        let terminal = Arc::new(FakeTerminal::new(host.clone()));
        let agent = Arc::new(FakeAgent::new(terminal.clone()));
        let queue = Arc::new(FakeMergeQueue::new(git.clone(), Some(place.clone())));
        let store = Arc::new(FakeStore::default());
        let history = Arc::new(FakeHistory::new(git.clone()));
        let resolver = Arc::new(FakeResolver::new(kinds.clone()));
        resolver.bind_no_tool_place(place.clone());
        resolver.bind_place("trees", place.clone());
        resolver.bind_change("plans", change.clone());
        resolver.bind_merge_queue("queue", queue.clone());
        resolver.bind_agent("agent", agent.clone());
        let defaults = Defaults {
            settings: Settings {
                program: Some("agent".into()),
                ..Settings::default()
            },
            send_backs: 2,
        };
        let engine = Engine::new(
            resolver.clone(),
            terminal.clone(),
            store.clone(),
            history.clone(),
            host.clone(),
            defaults.clone(),
        );
        World {
            git,
            host,
            place,
            kinds,
            change,
            terminal,
            agent,
            queue,
            store,
            history,
            resolver,
            defaults,
            engine,
        }
    }

    /// A new engine over the same fakes, as the next command builds one.
    pub fn engine(&self) -> Engine {
        Engine::new(
            self.resolver.clone(),
            self.terminal.clone(),
            self.store.clone(),
            self.history.clone(),
            self.host.clone(),
            self.defaults.clone(),
        )
    }

    pub fn id(kind: &str, name: &str) -> PlaceId {
        PlaceId::new(format!("{REPOSITORY}/places/{kind}--{name}"))
    }

    pub fn held(&self, place: &PlaceId) -> Option<HeldState> {
        self.store.held(place)
    }
}
