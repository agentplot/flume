//! A caller's launch (2, 7, 8).

use crate::{Seconds, Settings};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// One ask to start a piece of work: a repository, a kind, a name, a
/// base, and the caller's brief, settings, stages on hold with their
/// words, submission's hold, mark, priority, recipients and stall
/// bound (2, 7, 8, 70, 130).
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
    /// Settings for every stage's agent, which a stage's own come before
    /// (7, 32).
    #[serde(default)]
    pub every_stage: Settings,
    /// The stages it puts on hold (47).
    pub hold: Vec<String>,
    pub hold_submission: bool,
    pub mark: Option<String>,
    /// The words of each hold, by stage name: why the stage waits on its
    /// caller (7, 130).
    #[serde(default)]
    pub hold_words: BTreeMap<String, String>,
    /// The place's priority, higher first, zero where the caller gives
    /// none (7, 70).
    #[serde(default)]
    pub priority: i64,
    /// The recipients of the place's ends (7, 43).
    #[serde(default)]
    pub recipients: Vec<String>,
    /// How long the place may wait on its caller under no hold's words
    /// before it is stalled; five minutes where none is given (130).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stall_after: Option<Seconds>,
}
