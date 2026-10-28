//! A caller's launch (2, 7, 8).

use crate::Settings;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// One ask to start a piece of work: a repository, a kind, a name, a
/// base, and the caller's brief, settings, stages on hold, submission's
/// hold and mark (2, 7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Launch {
    pub repository: String,
    pub kind: String,
    pub name: String,
    pub base: String,
    pub brief: Option<String>,
    /// A declaration the launch carries, as the kinds seam reads it (19).
    pub declaration: Option<String>,
    /// Settings for each stage's agent, by stage name (32).
    pub settings: BTreeMap<String, Settings>,
    /// The stages it puts on hold (47).
    pub hold: Vec<String>,
    pub hold_submission: bool,
    pub mark: Option<String>,
}
