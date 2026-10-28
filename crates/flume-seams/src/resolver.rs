//! From a binding's profile names to the seams that answer for them
//! (design 2, 105). The core is built over one; a profile is chosen from
//! the binding alone (core.3).

use crate::{Agent, Binding, Change, Kinds, MergeQueue, Place};
use std::sync::Arc;

pub trait Resolver: Send + Sync {
    /// The kinds seam, which reads every declaration in the one form
    /// (105).
    fn kinds(&self) -> Arc<dyn Kinds>;

    /// The names of the place profiles it holds, so a binding naming
    /// another is refused naming them (C.1, core.3).
    fn place_profiles(&self) -> Vec<String>;

    /// The place profile `binding` names for `repository`, or where it
    /// names none the one that needs nothing beyond git, bound with the
    /// data the binding's merge queue profile declares for the place: its
    /// readiness, its teardown and where the base lives (105, design 9,
    /// 13). Neither profile calls the other (core.2).
    fn place(&self, repository: &str, binding: &Binding) -> Option<Arc<dyn Place>>;

    /// The change profile a binding names.
    fn change(&self, profile: &str) -> Option<Arc<dyn Change>>;

    /// The merge-queue profile a binding names.
    fn merge_queue(&self, profile: &str) -> Option<Arc<dyn MergeQueue>>;

    /// The agent profile a stage's settings name by its program (32,
    /// 105).
    fn agent(&self, program: &str) -> Option<Arc<dyn Agent>>;
}
