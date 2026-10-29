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
        let queue = Arc::new(FakeMergeQueue::new(git.clone()));
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

/// A stage of a repository's or a launch's kind.
pub fn stage(name: &str, deliverable: Option<Deliverable>, prompt: &str) -> Stage {
    Stage {
        name: name.into(),
        prompt: prompt.into(),
        conversation: deliverable.is_none(),
        deliverables: deliverable.into_iter().collect(),
        agents: vec![Settings::default()],
        waits: false,
        quiet_limit: None,
        send_back_to: None,
        send_back_limit: None,
    }
}

/// A kind of a repository's or a launch's, at version 1.
pub fn kind(name: &str, lands: bool, stages: Vec<Stage>) -> Kind {
    Kind {
        name: name.into(),
        version: 1,
        branch_prefix: name.into(),
        starts_unready: false,
        lands,
        agents: KindSettings::default(),
        stages,
    }
}

/// A launch of `kind` named `name` on `main` with `brief`.
pub fn launch(kind: &str, name: &str, brief: Option<&str>) -> Launch {
    Launch {
        repository: REPOSITORY.into(),
        kind: kind.into(),
        name: name.into(),
        base: "main".into(),
        brief: brief.map(str::to_string),
        ..Launch::default()
    }
}

/// The place an answer gives, or what the answer was.
pub fn launched(answer: &flume_core::Answer) -> Result<flume_core::PlaceStatus, String> {
    match answer {
        flume_core::Answer::Launched { place } => Ok(place.clone()),
        other => Err(format!("the launch answered {other:?}")),
    }
}

/// The refusal's reason an answer gives, or what the answer was.
pub fn refused(answer: &flume_core::Answer) -> Result<String, String> {
    answer
        .refusal()
        .map(str::to_string)
        .ok_or_else(|| format!("the act was taken: {answer:?}"))
}

impl World {
    pub fn identity(kind: &str, name: &str, stage: &str, start: u32) -> Identity {
        Identity {
            repository: REPOSITORY.into(),
            place: format!("{kind}--{name}"),
            stage: stage.into(),
            start,
        }
    }

    pub fn stage_state(&self, place: &PlaceId, stage: &str) -> Option<StageState> {
        self.held(place)?
            .stages
            .iter()
            .find(|held| held.name == stage)
            .map(|held| held.state)
    }

    /// The prompt the start under `identity` was opened with.
    pub fn prompt(&self, place: &PlaceId, identity: &Identity) -> Option<String> {
        let held = self.held(place)?;
        held.stages
            .iter()
            .filter_map(|stage| stage.start.as_ref())
            .find(|start| &start.identity == identity)
            .map(|start| start.prompt.text.clone())
    }
}

impl World {
    /// A commit on the place's branch touching `paths`, as its agent
    /// makes one.
    pub fn commit(&self, branch: &str, paths: &[&str]) -> Revision {
        self.git.commit(REPOSITORY, branch, paths)
    }

    /// A commit on `branch` to the change `name`, its planning complete
    /// and `ticked` of `total` tasks ticked.
    pub fn tick(&self, branch: &str, name: &str, ticked: usize, total: usize) -> Revision {
        let path = format!("plans/{name}/tasks.md");
        let revision = self.commit(branch, &[&path]);
        self.change.set(
            REPOSITORY,
            &revision,
            name,
            ChangeFacts {
                exists: true,
                planning_complete: true,
                tasks: (0..total)
                    .map(|task| Task {
                        text: format!("1.{} task", task + 1),
                        ticked: task < ticked,
                    })
                    .collect(),
                archived: false,
            },
        );
        revision
    }

    /// Settles the place as an agent going quiet or the watch does.
    pub fn settle(&self, place: &PlaceId) {
        self.engine().settle(Some(place));
    }

    /// The place's status, as status gives it.
    pub fn status(&self, place: &PlaceId) -> Result<flume_core::PlaceStatus, String> {
        match self.engine().status(Some(place)) {
            flume_core::Answer::Status { mut places, .. } if places.len() == 1 => {
                Ok(places.remove(0))
            }
            other => Err(format!("status answered {other:?}")),
        }
    }

    /// A one-stage kind of the repository's, `name`, delivering
    /// `deliverable`, launched as `place` and begun.
    pub fn begun(
        &self,
        kind_name: &str,
        deliverable: Option<Deliverable>,
        place: &str,
    ) -> Result<PlaceId, String> {
        self.kinds.set_kinds(
            REPOSITORY,
            vec![kind(
                kind_name,
                false,
                vec![stage("work", deliverable, "{{brief}}")],
            )],
        );
        launched(
            &self
                .engine
                .launch(&launch(kind_name, place, Some("do the work"))),
        )?;
        let id = World::id(kind_name, place);
        self.settle(&id);
        match self.stage_state(&id, "work") {
            Some(StageState::Begun) => Ok(id),
            other => Err(format!("the stage did not begin: {other:?}")),
        }
    }
}
