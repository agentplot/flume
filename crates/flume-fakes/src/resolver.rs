use crate::{lock, FakeKinds, Log};
use flume_seams::{Agent, Binding, Change, Kinds, MergeQueue, Place, Resolver};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// From profile names to fakes: the place profile with no name is the
/// one that needs nothing beyond git, and a name it does not hold
/// resolves to nothing. Every place it binds is logged with the
/// repository and the merge queue whose data it is bound with.
pub struct FakeResolver {
    kinds: Arc<FakeKinds>,
    state: Mutex<Profiles>,
    pub log: Log,
}

#[derive(Default)]
struct Profiles {
    no_tool_place: Option<Arc<dyn Place>>,
    places: BTreeMap<String, Arc<dyn Place>>,
    changes: BTreeMap<String, Arc<dyn Change>>,
    merge_queues: BTreeMap<String, Arc<dyn MergeQueue>>,
    agents: BTreeMap<String, Arc<dyn Agent>>,
}

impl FakeResolver {
    pub fn new(kinds: Arc<FakeKinds>) -> FakeResolver {
        FakeResolver {
            kinds,
            state: Mutex::default(),
            log: Log::default(),
        }
    }

    pub fn bind_no_tool_place(&self, place: Arc<dyn Place>) {
        lock(&self.state).no_tool_place = Some(place);
    }

    pub fn bind_place(&self, name: &str, place: Arc<dyn Place>) {
        lock(&self.state).places.insert(name.into(), place);
    }

    pub fn bind_change(&self, name: &str, change: Arc<dyn Change>) {
        lock(&self.state).changes.insert(name.into(), change);
    }

    pub fn bind_merge_queue(&self, name: &str, merge_queue: Arc<dyn MergeQueue>) {
        lock(&self.state)
            .merge_queues
            .insert(name.into(), merge_queue);
    }

    pub fn bind_agent(&self, program: &str, agent: Arc<dyn Agent>) {
        lock(&self.state).agents.insert(program.into(), agent);
    }
}

impl Resolver for FakeResolver {
    fn kinds(&self) -> Arc<dyn Kinds> {
        self.kinds.clone()
    }

    fn place_profiles(&self) -> Vec<String> {
        lock(&self.state).places.keys().cloned().collect()
    }

    fn place(&self, repository: &str, binding: &Binding) -> Option<Arc<dyn Place>> {
        self.log.push(format!(
            "place {repository} {} {}",
            binding.place.as_deref().unwrap_or("-"),
            binding.merge_queue.as_deref().unwrap_or("-")
        ));
        let state = lock(&self.state);
        match &binding.place {
            None => state.no_tool_place.clone(),
            Some(name) => state.places.get(name).cloned(),
        }
    }

    fn change(&self, profile: &str) -> Option<Arc<dyn Change>> {
        lock(&self.state).changes.get(profile).cloned()
    }

    fn merge_queue(&self, profile: &str) -> Option<Arc<dyn MergeQueue>> {
        lock(&self.state).merge_queues.get(profile).cloned()
    }

    fn agent(&self, program: &str) -> Option<Arc<dyn Agent>> {
        lock(&self.state).agents.get(program).cloned()
    }
}
