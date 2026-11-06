//! The store seam's answers (102 to 104), and the host's own documents
//! it keeps beside the places' held state (153).

use crate::{AccountMove, HeldState, MoveOn, Seconds};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A place's held state with the version it was read at (102).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Versioned {
    pub version: u64,
    pub state: HeldState,
}

/// write: done at a new version, or refused because the place's version
/// moved, or because a first write found held state already (103).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Written {
    Written { version: u64 },
    Moved,
}

/// The store seam (B.7), kept once for the host. The guarantee: it holds
/// nothing in a repository's content, and every flume process on the
/// host reaches the same store (104).
pub trait Store: Send + Sync {
    /// The place's held state and the version it was read at (102).
    fn read(&self, place: &crate::PlaceId) -> Option<Versioned>;

    /// Every place's held state (102).
    fn read_every(&self) -> Vec<Versioned>;

    /// The place's next held state, all or nothing, only if its held
    /// state is still at `at`; `at` none is a new place's first write,
    /// which succeeds only if it has none. `next` none removes the held
    /// state and the place's folder (103, 62).
    fn write(&self, place: &crate::PlaceId, at: Option<u64>, next: Option<&HeldState>) -> Written;

    /// The place's own folder beside its held state, for what its stages
    /// write for flume (123, design 2).
    fn folder(&self, place: &crate::PlaceId) -> String;

    /// Every line of `made.jsonl` in the place's own folder, in order and
    /// as written, where a stage's run records what it made; none where
    /// the file is absent. The lines go with the folder (128, 129, 62).
    fn made(&self, place: &crate::PlaceId) -> Vec<String>;

    /// The host's own document under `key` and the version it was read
    /// at; none where there is none (153).
    fn read_host(&self, key: &HostKey) -> Option<HostVersioned>;

    /// The host's next document under `key`, whole, only if it is still
    /// at `at`; `at` none is a first write, which succeeds only if there
    /// is none. `next` none removes it (103, 153).
    fn write_host(&self, key: &HostKey, at: Option<u64>, next: Option<&HostDocument>) -> Written;
}

/// A document of the host's own, by what it is of: an account of a pool,
/// an intent by its digest, or the host's watch, one for the host (146,
/// 147, 153, 208).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "of", rename_all = "snake_case")]
pub enum HostKey {
    Account { pool: String, account: String },
    Intent { digest: String },
    Watch,
}

impl std::fmt::Display for HostKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostKey::Account { pool, account } => write!(f, "account {pool}/{account}"),
            HostKey::Intent { digest } => write!(f, "intent {digest}"),
            HostKey::Watch => write!(f, "the host's watch"),
        }
    }
}

/// A host's own document with the version it was read at (153).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostVersioned {
    pub version: u64,
    pub document: HostDocument,
}

/// A host's own document: an account's spent time, an intent's
/// affinity, or the host's watch (146, 147, 153, 208).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "of", rename_all = "snake_case")]
pub enum HostDocument {
    Account(AccountSpent),
    Intent(Affinity),
    Watch(Watched),
}

/// The newest settle of every place a caller asked as its watch: when it
/// began by the host's clock, and the bound it stated. A caller keeps
/// watch while it began less than twice its bound ago; a host that holds
/// none has no watch (119, 208).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Watched {
    pub began: Seconds,
    pub bound: Seconds,
}

impl Watched {
    /// Whether the watch is kept at `now`: its settle began less than
    /// twice its bound ago (119, 208).
    pub fn kept(&self, now: Seconds) -> bool {
        now.saturating_sub(self.began) < self.bound.saturating_mul(2)
    }

    /// When the watch lapses: twice its bound after its settle began
    /// (119, 208).
    pub fn lapses(&self) -> Seconds {
        self.began.saturating_add(self.bound.saturating_mul(2))
    }
}

/// An account spent until a time by the host's clock, with the words its
/// agent's record gave, when flume recorded it, and whether flume read no
/// reset in the words, so the time is the hour 146 gives, false in a
/// document written before (146, 151).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountSpent {
    pub until: Seconds,
    pub words: Option<String>,
    pub recorded_at: Seconds,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub unread: bool,
}

/// An intent's affinity: the stream its agents keep to, the pool and
/// account they keep to for each program they run, each move it made
/// between accounts, and when its last place went, none while a place of
/// it stands (147).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Affinity {
    pub stream: String,
    /// For each program the intent's agents run, as its accounts name
    /// it, the account the first of them on that program started on, as
    /// moved since (147).
    #[serde(default)]
    pub accounts: BTreeMap<String, AffinityAccount>,
    pub moves: Vec<AccountMove>,
    pub gone: Option<Seconds>,
    /// The one account a document written before an intent kept one
    /// per program holds, and its pool; read and never written (147).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

/// The account an intent's agents on one program keep to, and its pool
/// (147).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffinityAccount {
    pub pool: String,
    pub account: String,
}

impl Affinity {
    /// Whether the affinity for `program` once left `account` (147).
    pub fn left(&self, program: &str, account: &str) -> bool {
        self.moves.iter().any(|moved| {
            moved.from == account
                && matches!(&moved.on, MoveOn::Intent { program: Some(moved) } if moved == program)
        })
    }
}

/// The name of the file in a place's own folder that a stage's run
/// records what it made in, one line per thing (129).
pub const MADE: &str = "made.jsonl";
