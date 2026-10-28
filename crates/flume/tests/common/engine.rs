//! An engine over the conformance suite's fakes with one seam's real
//! profile put in their place, so a profile is driven through the core
//! as a launch drives it.

use flume_conformance::engine::world::World;
use flume_core::Engine;
use flume_fakes::FakeResolver;
use flume_seams::*;
use std::sync::Arc;

/// The place profile name the fakes' no-tool place answers to here, as
/// the binary's resolver names its one place profile (C.1).
pub const PLACE_PROFILE: &str = "git-worktree";

/// The fakes' resolver with `kinds` in place of the fake kinds, holding
/// one place profile, `git-worktree`, which is the fakes' no-tool place.
pub struct WithKinds {
    pub fakes: Arc<FakeResolver>,
    pub kinds: Arc<dyn Kinds>,
}

impl Resolver for WithKinds {
    fn kinds(&self) -> Arc<dyn Kinds> {
        self.kinds.clone()
    }

    fn place_profiles(&self) -> Vec<String> {
        vec![PLACE_PROFILE.into()]
    }

    fn place(&self, repository: &str, binding: &Binding) -> Option<Arc<dyn Place>> {
        match binding.place.as_deref() {
            None | Some(PLACE_PROFILE) => self.fakes.place(repository, &Binding::default()),
            Some(_) => None,
        }
    }

    fn change(&self, profile: &str) -> Option<Arc<dyn Change>> {
        self.fakes.change(profile)
    }

    fn merge_queue(&self, profile: &str) -> Option<Arc<dyn MergeQueue>> {
        self.fakes.merge_queue(profile)
    }

    fn agent(&self, program: &str) -> Option<Arc<dyn Agent>> {
        self.fakes.agent(program)
    }
}

/// An engine over `world`'s fakes and the kinds profile `kinds`, as the
/// next command builds one.
pub fn engine(world: &World, kinds: Arc<dyn Kinds>) -> Engine {
    Engine::new(
        Arc::new(WithKinds {
            fakes: world.resolver.clone(),
            kinds,
        }),
        world.terminal.clone(),
        world.store.clone(),
        world.history.clone(),
        world.host.clone(),
        world.defaults.clone(),
    )
}

/// The base `main` of the scratch repository `repository` as the fakes'
/// history reads it: at the scratch repository's own commit.
pub fn base_at(world: &World, repository: &str, commit: &str) {
    world
        .git
        .set_branch(repository, "main", &Revision::new(commit));
}
