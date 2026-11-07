//! The settle, flume's one mover (46, 119 to 122): it reads every fact of
//! a place again and takes each step Part A then allows, collecting its
//! submission's outcomes first, safe to repeat (74, 133). It runs inside
//! a command of flume's, started by an agent going quiet, by a caller
//! keeping watch over the host, or by any other caller, never by a
//! terminal or a plugin of one, and waits on no merge queue (120, 122,
//! 208, core.13). Each end it or an act reads is recorded once as it is
//! written, and the settle delivers each end pending for a recipient,
//! then wakes each role's address one stands pending for with no run
//! (43, 132, core.25).

use crate::held::read_stall;
use crate::{Answer, Engine, Step, Tidied};
use flume_seams::*;

/// How many steps one settle of a place takes at most.
const STEPS: usize = 64;

/// How many times a write of the host's watch reads again where its
/// version moved.
const TRIES: usize = 8;

impl Engine {
    /// Settles the place, or every place on the host.
    pub fn settle(&self, place: Option<&PlaceId>) -> Answer {
        let places: Vec<PlaceId> = match place {
            Some(id) => vec![id.clone()],
            None => self
                .store
                .read_every()
                .into_iter()
                .map(|read| read.state.place.id)
                .collect(),
        };
        let mut tidy = Tidied::default();
        for id in &places {
            tidy.add(self.settle_place(id));
        }
        Answer::Settled { places, tidy }
    }

    /// Settles every place on the host as a caller's watch over it,
    /// whose bound is `bound`: the host's watch written first, when this
    /// settle began and its bound, by compare-and-set, then every place
    /// settled as [`Engine::settle`] naming none settles them. A caller
    /// keeps watch while its newest such settle began less than twice its
    /// bound ago; the watch decides nothing of what the settle does (46,
    /// 119, 120, 208; core.13).
    pub fn settle_every(&self, bound: Seconds) -> Answer {
        if bound == 0 {
            return Answer::refused(
                "a watch's bound is a positive duration, and 0 seconds is none",
            );
        }
        self.keep_watch(Watched {
            began: self.host.now(),
            bound,
        });
        self.settle(None)
    }

    /// Writes the host's watch as `watched`, by compare-and-set, reading
    /// it again where its version moved; a watch that began later and
    /// stands is kept (73, 153, 208).
    fn keep_watch(&self, watched: Watched) {
        for _ in 0..TRIES {
            let read = self.store.read_host(&HostKey::Watch);
            if let Some(HostVersioned {
                document: HostDocument::Watch(standing),
                ..
            }) = &read
            {
                if standing.began > watched.began {
                    return;
                }
            }
            let at = read.map(|read| read.version);
            if let Written::Written { .. } =
                self.store
                    .write_host(&HostKey::Watch, at, Some(&HostDocument::Watch(watched)))
            {
                return;
            }
        }
        self.host.log(&format!(
            "the host's watch was not written: its version moved {TRIES} times"
        ));
    }

    /// The host's watch as it is held, none where none is held, a host's
    /// state written before among it (153, 208).
    pub(crate) fn watched(&self) -> Option<Watched> {
        match self.store.read_host(&HostKey::Watch)?.document {
            HostDocument::Watch(watched) => Some(watched),
            _ => None,
        }
    }

    /// The host's watch as status gives it: the bound a caller keeping
    /// watch stated, or none; when its newest settle began; and, while
    /// flume runs attended after a watch lapsed, when it lapsed, read
    /// from the watch held (56, 119, 208).
    pub(crate) fn watch_status(&self) -> (Watch, Option<Seconds>, Option<Seconds>) {
        let Some(watched) = self.watched() else {
            return (Watch::None, None, None);
        };
        if watched.kept(self.host.now()) {
            let watch = Watch::Bound {
                seconds: watched.bound,
            };
            (watch, Some(watched.began), None)
        } else {
            (Watch::None, Some(watched.began), Some(watched.lapses()))
        }
    }

    /// Whether flume runs attended: no caller keeps watch over the host,
    /// none being held or the newest lapsed, twice its bound having
    /// passed since it began. No terminal is read (119, 208).
    pub fn attended(&self) -> bool {
        !self
            .watched()
            .is_some_and(|watched| watched.kept(self.host.now()))
    }

    /// Settles every place on the host.
    pub(crate) fn settle_all(&self) -> Tidied {
        let mut tidy = Tidied::default();
        for read in self.store.read_every() {
            tidy.add(self.settle_place(&read.state.place.id));
        }
        tidy
    }

    /// Where no caller keeps watch over the host, every command of
    /// flume's that acts on places settles every place first, whatever
    /// terminals it binds (119, 190, 208, I8); where one keeps watch its
    /// settles settle them.
    pub(crate) fn settle_attended(&self) -> Tidied {
        if self.attended() {
            return self.settle_all();
        }
        Tidied::default()
    }

    /// Settles one place, then starts what waits for room on the host
    /// (46, 70), and gives what its tidy did.
    pub(crate) fn settle_place(&self, id: &PlaceId) -> Tidied {
        let mut tidied = Tidied::default();
        self.deliver_ends(id);
        for _ in 0..STEPS {
            if !self.step(id, &mut tidied) {
                break;
            }
        }
        self.stall(id);
        self.deliver_ends(id);
        self.wake_roles(id);
        self.show(id);
        self.start_waiting();
        tidied
    }

    /// Reads the move a live place waits on its caller for, and records
    /// it stalled once its bound has passed under no hold's words (130);
    /// a base place's from the live places on its branch, read now (203).
    fn stall(&self, id: &PlaceId) {
        let now = self.host.now();
        let Some(read) = self.store.read(id) else {
            return;
        };
        let on_branch = self.on_branch_count(&read.state);
        self.update(id, |held| {
            if held.phase != Phase::Live {
                return Step::Keep(());
            }
            read_stall(held, now, on_branch);
            Step::Write(())
        });
    }

    /// Shows the place's clues where they changed since last shown: its
    /// current stage, a repeating one with its group's place, and that
    /// stage's state, the short form of the place's line, read from its
    /// held state, the change at its head and its agent's asking mark as
    /// `flume status --line` reads them (57, 142), and the count of the
    /// places of its repository on its base, in the place's terminal
    /// (190). A clue is display only: one that fails is logged and fails
    /// nothing, and nothing reads one back (139).
    fn show(&self, id: &PlaceId) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        let group = self.current_group(&held);
        let Some(mut clues) = self.clues(&held, None, group.as_ref()) else {
            return;
        };
        clues.base = self.base_count(&held.place);
        clues.line = self.line_of(&held, None).short;
        let place = &held.place;
        let key = format!(
            "{} · {}; {}; {}",
            clues.stage, clues.state, clues.line, clues.base
        );
        if held.shown.as_ref() == Some(&key) {
            return;
        }
        let shown = match self.place_terminal(&held) {
            Ok(terminal) => terminal.show(&place.place_ref(), &clues),
            Err(reason) => Shown::Failed { reason },
        };
        match shown {
            Shown::Failed { reason } => {
                self.host
                    .log(&format!("the clues of {id} did not show: {reason}"));
            }
            Shown::Shown | Shown::ShowsNone => {
                self.update(id, |held| {
                    held.shown = Some(key.clone());
                    Step::Write(())
                });
            }
        }
    }

    /// One step the place allows; true when it changed something.
    fn step(&self, id: &PlaceId, tidied: &mut Tidied) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let held = read.state;
        match &held.phase {
            Phase::Making { by } => return !self.host.alive(by) && self.undo(&held),
            Phase::Over { .. } | Phase::Gone { .. } => return self.tidy_place(id, tidied),
            Phase::Tidied { .. } => return self.settle_tidied(id),
            Phase::Live => {}
        }
        if self.submission(&held) {
            return true;
        }
        let Some(index) = held.current else {
            return false;
        };
        match held.stages[index].state {
            StageState::Starting | StageState::Started => self.deliver(id),
            StageState::NeverBegun => self.begun_after(id),
            StageState::Begun | StageState::Stuck | StageState::Lost => self.read_end(id),
            StageState::Delivered | StageState::Finished | StageState::Repeating => {
                self.advance(id)
            }
            StageState::EndedShort => self.found_work(id),
            _ => false,
        }
    }

    /// After a stage delivered and its agent ended, starts the next stage
    /// unless it waits on its caller's word (40, 47); after a repetition
    /// delivered its group with a task still unticked, starts the same
    /// stage again, waiting for room like any start, unless a hold names
    /// it (70, 140; core.26). The current stage is read again in the
    /// update, and nothing starts unless it still stands delivered,
    /// finished or repeating there: a settle beside this one may have
    /// moved the place on since this one read it, its next stage refused
    /// at its start and current, and the stage after that is not this
    /// settle's to start (75).
    fn advance(&self, id: &PlaceId) -> bool {
        let next = self.update(id, |held| {
            let Some(index) = held.current else {
                return Step::Keep(None);
            };
            let ended = &held.stages[index];
            let ends = matches!(
                ended.state,
                StageState::Delivered | StageState::Finished | StageState::Repeating
            );
            if !ends || ended.start.as_ref().is_some_and(|start| start.live) {
                return Step::Keep(None);
            }
            if ended.state == StageState::Repeating {
                if held.hold_on(&ended.name).is_some() {
                    held.stages[index].state = StageState::OnHold;
                    return Step::Write(None);
                }
                return Step::Keep(Some(index));
            }
            let next = index + 1;
            if next >= held.stages.len() || held.stages[next].state != StageState::NotReached {
                return Step::Keep(None);
            }
            let declared = &held.place.kind.stages[next];
            if held.hold_on(&declared.name).is_some() || declared.waits {
                held.stages[next].state = StageState::OnHold;
                return Step::Write(None);
            }
            Step::Keep(Some(next))
        });
        match next.flatten() {
            Some(next) => {
                self.start_stage(id, next, None);
                true
            }
            None => false,
        }
    }

    /// A place whose launching command is gone while it was made: what
    /// was made is removed, so nothing of it is left; a base place's
    /// branch closed by the merge queue into its base, where its open
    /// made it, and nothing asked of the place seam (2, 76, 206).
    fn undo(&self, held: &HeldState) -> bool {
        let place = &held.place;
        let Ok(bound) = self.bound(place) else {
            return false;
        };
        if crate::base::base_place(&place.kind) {
            if let Some(queue) = &bound.merge_queue {
                if let BranchClosed::Kept { reason } =
                    queue.close(&place.repository, &place.branch, &place.base)
                {
                    self.host.log(&format!(
                        "the branch {} of {}, whose launch died, was kept: {reason}",
                        place.branch, place.id
                    ));
                }
            }
            return self.remove_held(&place.id);
        }
        let at = place.place_ref();
        if let PlaceRead::Live { .. } = bound.place.read(&at) {
            if let Removed::Refused { .. } = bound.place.remove(&at) {
                return false;
            }
        }
        bound
            .place
            .remove_branch(&place.repository, &place.branch, &place.base);
        self.remove_held(&place.id)
    }
}
