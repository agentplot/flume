//! Base places (9, 10, 18, 202 to 206; core.53): a place whose kind lands
//! and has no stages, told by that shape alone and never by a kind's
//! name, a branch's prefix or a depth. It has no worktree: it is its
//! branch, which the merge queue opens at its launch and closes at its
//! tidy at flume's asking, and its held state. Other places name its
//! branch as their base, are cut there and land there, at any depth; it
//! is submitted by name on its caller's retry once none of them is live,
//! and its outcome is read from the merge queue's record of its branch.

use crate::{Engine, Step};
use flume_seams::*;

/// Whether a place of `kind` is a base place: its kind lands and has no
/// stages (18, 202; core.53).
pub(crate) fn base_place(kind: &Kind) -> bool {
    kind.lands && kind.stages.is_empty()
}

/// Whether a place's submission is made and not yet decided: submitted,
/// or a request opened for it, and no outcome read (111, 124, 204).
pub(crate) fn submission_open(held: &HeldState) -> bool {
    held.submission.submits > 0 && held.submission.verdict.is_none()
}

impl Engine {
    /// The places of `place`'s repository whose base is its branch that
    /// are being made or live, read from the store at each ask (130, 203,
    /// 204).
    pub(crate) fn on_branch(&self, place: &PlaceRecord) -> Vec<PlaceId> {
        self.store
            .read_every()
            .into_iter()
            .map(|read| read.state)
            .filter(|other| {
                matches!(other.phase, Phase::Making { .. } | Phase::Live)
                    && other.place.repository == place.repository
                    && other.place.base == place.branch
                    && other.place.id != place.id
            })
            .map(|other| other.place.id)
            .collect()
    }

    /// How many places are being made or live on a base place's branch;
    /// none for any other place, whose wait reads no other place (130,
    /// 203).
    pub(crate) fn on_branch_count(&self, held: &HeldState) -> Option<usize> {
        base_place(&held.place.kind).then(|| self.on_branch(&held.place).len())
    }

    /// Refuses a launch on `base` where it is the branch of a place of
    /// `repository` that stands: one that is no base place, whose
    /// worktree checks its branch out, so a landing there would move
    /// under its agent; or a base place being made, whose submission is
    /// open, or whose work is over, since a place landing there would
    /// land after what was submitted and the branch's close would keep
    /// it. A base no place holds is launched on as before (10, 63, 111,
    /// 204, 206).
    pub(crate) fn on_a_places_branch(&self, repository: &str, base: &str) -> Result<(), String> {
        for read in self.store.read_every() {
            let other = read.state;
            if matches!(other.phase, Phase::Tidied { .. })
                || other.place.repository != repository
                || other.place.branch != base
            {
                continue;
            }
            let id = &other.place.id;
            if !base_place(&other.place.kind) {
                return Err(format!(
                    "the base {base} is the branch of the place {id}, which is no base place: its worktree checks the branch out, and a landing there would move under its agent (10, 204)"
                ));
            }
            let why = match other.phase {
                Phase::Live if submission_open(&other) => {
                    "whose submission is open, so a place landing there would land after what was submitted"
                }
                Phase::Live => continue,
                Phase::Making { .. } => "which is being made",
                _ => "whose work is over",
            };
            return Err(format!(
                "the base {base} is the branch of the base place {id}, {why} (204)"
            ));
        }
        Ok(())
    }

    /// A base place's launch past its held state's writing: its branch
    /// opened by the merge queue at its base, the revision it was made
    /// at kept as its base's, and the place live with no stage, no
    /// worktree, no readiness and no start; or, where the merge queue
    /// refuses, the launch refused with its words and the held state
    /// removed, nothing else made (2, 9, 11, 202, 205; core.8, core.17).
    pub(crate) fn open_base_place(
        &self,
        held: &HeldState,
        queue: &dyn MergeQueue,
    ) -> Result<HeldState, String> {
        let place = &held.place;
        let id = &place.id;
        let revision = match queue.open(&place.repository, &place.branch, &place.base) {
            BranchOpened::Made { revision } => revision,
            BranchOpened::Refused { reason } => {
                self.remove_held(id);
                return Err(format!(
                    "the merge queue would not open the branch {} from {}: {reason}",
                    place.branch, place.base
                ));
            }
        };
        self.update(id, |held| {
            held.place.base_revision = revision.clone();
            held.phase = Phase::Live;
            Step::Write(())
        });
        self.store
            .read(id)
            .map(|read| read.state)
            .ok_or_else(|| format!("the place {id} was removed while it was launched"))
    }

    /// The head of a base place's branch where the merge queue says it
    /// lives, where work lands on it; as git gives the branch where the
    /// merge queue lands no such branch; nothing is read at its location
    /// (80, 126).
    pub(crate) fn base_place_head(
        &self,
        place: &PlaceRecord,
        queue: &dyn MergeQueue,
    ) -> Result<Revision, String> {
        match queue.base_revision(&place.repository, &place.branch) {
            BaseRevision::At { revision } => Ok(revision),
            BaseRevision::Refused { reason } => Err(reason),
            BaseRevision::Unlanded => self
                .history
                .head(&place.repository, &place.branch)
                .ok_or_else(|| format!("{} has no branch {}", place.repository, place.branch)),
        }
    }
}
