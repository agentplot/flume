//! Base places (9, 10, 18, 202 to 206; core.53): a place whose kind lands
//! and has no stages, told by that shape alone and never by a kind's
//! name, a branch's prefix or a depth. It has no worktree: it is its
//! branch, which the merge queue opens at its launch and closes at its
//! tidy at flume's asking, and its held state. Other places name its
//! branch as their base, are cut there and land there, at any depth; it
//! is submitted by name on its caller's retry once none of them is live,
//! and its outcome is read from the merge queue's record of its branch.

use crate::fixes::{decide, Found};
use crate::{Engine, Step};
use flume_seams::*;

/// Whether a place of `kind` is a base place: its kind lands and has no
/// stages (18, 202; core.53).
pub(crate) fn base_place(kind: &Kind) -> bool {
    kind.lands && kind.stages.is_empty()
}

/// Whether two forecasts are one for holding: the same outcome, clear,
/// in conflict with the base, in conflict only with entries queued ahead,
/// or unread, at the same head and base's revision (203, 210).
fn same_forecast(was: &Forecast, now: &Forecast) -> bool {
    let outcome = |forecast: &Forecast| match forecast {
        Forecast::Clear { .. } => 0,
        Forecast::Conflict { with, .. } if with.is_empty() => 1,
        Forecast::Conflict { .. } => 2,
        Forecast::CouldNotRead { .. } => 3,
    };
    outcome(was) == outcome(now) && was.formed() == now.formed()
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

    /// The settle's forecast of a base place whose kind names a fix, once
    /// at each settle of it: asked of the merge queue while it is live, no
    /// fix of its is written live, no place on its branch is being made or
    /// live and its submission is not open, and held with when it was read
    /// only where its outcome, its head or its base's revision changed. A
    /// forecast in conflict with the base itself launches its fix, decided
    /// in the write that holds it; one in conflict only with entries queued
    /// ahead launches nothing. A forecast that could not be read fails
    /// nothing, and the next settle asks again; a settle of another place
    /// never asks it; and a fix written live whose place does not stand is
    /// launched in its stead (46, 203, 208, 210; core.13, core.17).
    pub(crate) fn forecast_base(&self, id: &PlaceId) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        let place = &held.place;
        if held.phase != Phase::Live || !base_place(&place.kind) || place.fixes_as.is_none() {
            return;
        }
        if self.live_fix(id, &held) || !self.on_branch(place).is_empty() || submission_open(&held) {
            return;
        }
        let Some(queue) = self.bound(place).ok().and_then(|bound| bound.merge_queue) else {
            return;
        };
        let forecast = queue.forecast(&place.repository, &place.branch, &place.base);
        let found = match &forecast {
            Forecast::Conflict {
                head,
                onto,
                files,
                with,
                words,
            } if with.is_empty() => Some(Found::forecast(place, head, onto, files, words)),
            _ => None,
        };
        let now = self.host.now();
        let limit = self.defaults.send_backs;
        let place_of = self.fix_place_of(&held);
        let number = self.update(id, |held| {
            if held.phase != Phase::Live {
                return Step::Keep(None);
            }
            if !held
                .forecast
                .as_ref()
                .is_some_and(|was| same_forecast(&was.forecast, &forecast))
            {
                held.forecast = Some(ForecastHeld {
                    at: now,
                    forecast: forecast.clone(),
                });
            }
            let number = found
                .as_ref()
                .and_then(|found| decide(held, found, limit, now, &place_of));
            Step::Write(number)
        });
        if let (Some(Some(number)), Some(found)) = (number, &found) {
            self.launch_fix(id, number, Some(found));
        }
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
