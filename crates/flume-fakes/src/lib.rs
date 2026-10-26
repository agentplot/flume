//! One scriptable fake for each of flume's seven seams, and for git's
//! history and the host: a test sets what each answers, its refusals and
//! unknowns among them, and reads back every call made of it, in order.
//! The fake clock moves only when a test advances it or a command sleeps
//! on it. Each fake passes its seam's scenarios in the conformance suite,
//! the same ones a profile runs (106).

mod agent;
mod change;
mod git;
mod host;
mod kinds;
mod log;
mod merge_queue;
mod place;
mod resolver;
mod store;
mod terminal;

pub use agent::FakeAgent;
pub use change::FakeChange;
pub use git::{FakeGit, FakeHistory};
pub use host::FakeHost;
pub use kinds::FakeKinds;
pub use log::Log;
pub use merge_queue::{FakeMergeQueue, Plan};
pub use place::FakePlace;
pub use resolver::FakeResolver;
pub use store::FakeStore;
pub use terminal::FakeTerminal;

use std::sync::{Mutex, MutexGuard};

/// The guard of a fake's state, read through a panic a scenario caught.
pub(crate) fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
