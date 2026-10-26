//! The seam scenarios of Part B, one file per seam. Each scenario is a
//! function generic over its seam's harness, which hands it the seam and
//! arranges what it needs, so a profile unit runs the same scenarios
//! against its real tool as the fakes pass (106). Each file's list sits
//! below its seam's marker, and a profile unit appends scenarios of its
//! own at the list's end.

pub mod agent;
pub mod change;
pub mod history;
pub mod kinds;
pub mod merge_queue;
pub mod place;
pub mod store;
pub mod terminal;
