//! From a binding's profile names to the seams that answer for them
//! (design 2, 105). The core is built over one; a profile is chosen from
//! the binding alone (core.3).

use crate::{Agent, Change, Kinds, MergeQueue, Place};
use std::sync::Arc;

pub trait Resolver: Send + Sync {
    /// The kinds seam, which reads every declaration in the one form
    /// (105).
    fn kinds(&self) -> Arc<dyn Kinds>;

    /// The place profile a binding names, or with `None` the one that
    /// needs nothing beyond git (105).
    fn place(&self, profile: Option<&str>) -> Option<Arc<dyn Place>>;

    /// The change profile a binding names.
    fn change(&self, profile: &str) -> Option<Arc<dyn Change>>;

    /// The merge-queue profile a binding names.
    fn merge_queue(&self, profile: &str) -> Option<Arc<dyn MergeQueue>>;

    /// The agent profile a stage's settings name by its program (32,
    /// 105).
    fn agent(&self, program: &str) -> Option<Arc<dyn Agent>>;
}
