//! Git's history, which section 9 fixes and the core reads beside the
//! seams (design 3). No seam of Part B.

use crate::Revision;
use serde::{Deserialize, Serialize};

/// The newest commit at a head touching a path, and whether it was made
/// since a start head (24, 25).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Touch {
    pub commit: Revision,
    pub since: bool,
}
