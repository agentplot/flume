//! The seven seams of Part B of flume's statement, one trait each, the
//! types they pass, and git's history and the host's facts beside them
//! (section 9). A profile depends on this crate alone, so it implements
//! its interface and nothing of the core (core.2), and no type or trait
//! here names a tool (core.1).

mod agent;
mod change;
mod held;
mod history;
mod host;
mod kinds;
mod launch;
mod merge_queue;
mod place;
mod prompt;
mod resolver;
mod store;
mod terminal;

pub use agent::*;
pub use change::*;
pub use held::*;
pub use history::*;
pub use host::*;
pub use kinds::*;
pub use launch::*;
pub use merge_queue::*;
pub use place::*;
pub use prompt::*;
pub use resolver::*;
pub use store::*;
pub use terminal::*;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A commit of a repository, by its id.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Revision(pub String);

impl Revision {
    pub fn new(id: impl Into<String>) -> Self {
        Revision(id.into())
    }
}

impl fmt::Display for Revision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A place, by its location `<repository>/places/<kind>--<name>` (C.1),
/// which names one place on the host.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlaceId(pub String);

impl PlaceId {
    pub fn new(location: impl Into<String>) -> Self {
        PlaceId(location.into())
    }
}

impl fmt::Display for PlaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A moment by the host's clock, in seconds.
pub type Seconds = u64;

#[cfg(test)]
mod round_trip;
