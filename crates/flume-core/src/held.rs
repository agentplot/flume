//! A place's held state, read with its version and written at it (73 to
//! 76, 102, 103): every act reads, decides and writes at the version it
//! read, and a write refused because the version moved is read again and
//! decided again, so two acts on one place leave it as one or the other
//! would have (75).

use crate::base::base_place;
use crate::status::waiting_on;
use crate::Engine;
use flume_seams::{
    Act, EndHeld, EndOf, HeldState, HoldOn, Over, Phase, PlaceId, Seconds, StageState,
    StopShortWaited, Verdict, Versioned, Waiting, WaitingOn, Waits, Written,
};

/// What an act decided of a place's held state.
pub enum Step<T> {
    /// Write the state as the act left it, then answer `T`.
    Write(T),
    /// Write nothing, and answer `T`.
    Keep(T),
}

/// How many times an act reads again after a refused write before it
/// gives up.
const TRIES: usize = 32;

impl Engine {
    /// A place's held state and the version it was read at (102).
    pub fn held(&self, place: &PlaceId) -> Option<Versioned> {
        self.store.read(place)
    }

    /// A new place's first held state, written only if the place has
    /// none: its version, or none when held state stands (75, 103).
    pub fn create(&self, state: &HeldState) -> Option<u64> {
        match self.store.write(&state.place.id, None, Some(state)) {
            Written::Written { version } => Some(version),
            Written::Moved => None,
        }
    }

    /// Reads the place's held state, lets `decide` change it, and writes
    /// it at the version read; a refused write reads again and decides
    /// again. None when the place has no held state.
    pub fn update<T>(
        &self,
        place: &PlaceId,
        mut decide: impl FnMut(&mut HeldState) -> Step<T>,
    ) -> Option<T> {
        for _ in 0..TRIES {
            let read = self.store.read(place)?;
            let mut next = read.state.clone();
            match decide(&mut next) {
                Step::Keep(answer) => return Some(answer),
                Step::Write(answer) => {
                    if next == read.state {
                        return Some(answer);
                    }
                    self.stamp(&read.state, &mut next);
                    match self.store.write(place, Some(read.version), Some(&next)) {
                        Written::Written { .. } => return Some(answer),
                        Written::Moved => continue,
                    }
                }
            }
        }
        None
    }

    /// Stamps since when each stage stands in its state and the place
    /// waits for its caller, wherever an act changed them, so status says
    /// since when a place waits (56, 130); a base place's wait read from
    /// the live places on its branch as they stand (203).
    fn stamp(&self, was: &HeldState, next: &mut HeldState) {
        let now = self.host.now();
        for (index, stage) in next.stages.iter_mut().enumerate() {
            if was.stages.get(index).map(|was| was.state) != Some(stage.state) {
                stage.since = Some(now);
            }
        }
        let submission = &mut next.submission;
        if submission.waiting.is_none() {
            submission.waiting_since = None;
        } else if submission.waiting != was.submission.waiting {
            submission.waiting_since = Some(now);
        }
        let on_branch = self.on_branch_count(next);
        refresh_waiting(next, now, on_branch);
        record_ends(next, now);
    }

    /// Removes the place's held state, and its folder with it, at the
    /// version read (62, 103).
    pub fn remove_held(&self, place: &PlaceId) -> bool {
        for _ in 0..TRIES {
            let Some(read) = self.store.read(place) else {
                return true;
            };
            if let Written::Written { .. } = self.store.write(place, Some(read.version), None) {
                return true;
            }
        }
        false
    }
}

/// Records in `ends`, once each by its key, every end the held state
/// shows at `now` that it has not recorded: each stage's start
/// delivered, ended short, finished, lost or never begun, or stuck once
/// what was read of it is written, with what was read; each outcome of
/// a submission the merge queue gave, and a landing read from its
/// record; and a drop or a done, as the end of the place's work. A
/// stage's end carries the answers its start had taken in its session,
/// so a stop-short after an answer is an end of its own (42). Written in
/// the same write as the end itself, so whatever act or settle reads an
/// end records it, and a repeat records nothing (43, 52, 74, 114, 133).
fn record_ends(next: &mut HeldState, now: Seconds) {
    if matches!(next.phase, Phase::Tidied { .. }) {
        return;
    }
    let mut read: Vec<EndOf> = vec![];
    for stage in &next.stages {
        let Some(start) = &stage.start else {
            continue;
        };
        let ended = match stage.state {
            StageState::Delivered
            | StageState::EndedShort
            | StageState::Finished
            | StageState::Lost
            | StageState::NeverBegun => true,
            StageState::Stuck => stage.end.is_some(),
            _ => false,
        };
        if !ended {
            continue;
        }
        read.push(EndOf::Stage {
            stage: stage.name.clone(),
            start: start.n,
            state: stage.state,
            quiet_from: if stage.state == StageState::Stuck {
                start.first_quiet
            } else {
                None
            },
            answered: start.answered,
            end: Box::new(stage.end.clone().unwrap_or_default()),
        });
    }
    let submit = next.submission.submits;
    match &next.submission.verdict {
        Some(Verdict::Refused { .. }) | None => {}
        Some(verdict) => read.push(EndOf::Submission {
            submit,
            verdict: verdict.clone(),
        }),
    }
    if let Phase::Over {
        over: Over::Landed { commit },
        ..
    }
    | Phase::Gone {
        over: Over::Landed { commit },
        ..
    } = &next.phase
    {
        if !matches!(next.submission.verdict, Some(Verdict::Landed { .. })) {
            read.push(EndOf::Submission {
                submit,
                verdict: Verdict::Landed {
                    commit: commit.clone(),
                    place_removed: false,
                    branch_removed: false,
                },
            });
        }
    }
    if let Phase::Over {
        over: over @ (Over::Dropped | Over::Done),
        ..
    }
    | Phase::Gone {
        over: over @ (Over::Dropped | Over::Done),
        ..
    } = &next.phase
    {
        read.push(EndOf::Over { over: over.clone() });
    }
    for of in read {
        if !next.ends.iter().any(|end| end.same(&of)) {
            next.ends.push(EndHeld {
                of,
                read_at: now,
                deliveries: vec![],
                answer: None,
                claim: None,
                noticing: vec![],
            });
        }
    }
}

/// Reads, from the held state alone, and for a base place `on_branch`,
/// the count of the live places on its branch, the move the place waits
/// on its caller for: a move read for the first time is kept with since
/// when it stood so, the same move keeps its since and whether it
/// stalled, and none clears it; a hold on the move ends once the place
/// waits on another move or none, the move made (56, 130, 203; design 5,
/// 7).
pub(crate) fn refresh_waiting(next: &mut HeldState, now: Seconds, on_branch: Option<usize>) {
    let was = next.waiting.clone();
    next.waiting = waiting_read(next, on_branch, Some(now));
    let moved = match (&was, &next.waiting) {
        (Some(was), Some(now)) => !was.on.same_move(&now.on),
        (Some(_), None) => true,
        (None, _) => false,
    };
    if moved {
        next.holds.retain(|hold| hold.on != HoldOn::Waiting);
    }
}

/// The move the place waits on its caller for as status, its line and
/// the list give it when asked: as the last settle or act wrote it, but
/// a base place's read again from `on_branch`, the live places on its
/// branch as they stand, keeping since when and whether it stalled where
/// the move is the one written (56, 58, 130, 203; core.15).
pub(crate) fn waiting_now(held: &HeldState, on_branch: Option<usize>) -> Option<Waiting> {
    if base_place(&held.place.kind) {
        waiting_read(held, on_branch, None)
    } else {
        held.waiting.clone()
    }
}

/// The move `held` waits on its caller for, read again, the same move
/// keeping its since and whether it stalled, and a move read for the
/// first time since `now` where one is given.
fn waiting_read(
    held: &HeldState,
    on_branch: Option<usize>,
    now: Option<Seconds>,
) -> Option<Waiting> {
    waiting_on(held, on_branch).map(|mut on| match &held.waiting {
        Some(was) if was.on.same_move(&on) => {
            on.since = was.on.since;
            Waiting {
                on,
                stalled: was.stalled,
            }
        }
        _ => {
            on.since = on.since.or(now);
            Waiting { on, stalled: false }
        }
    })
}

/// Reads the place stalled where it has waited on its caller under no
/// hold's words for its bound, recording the stall as an end once, with
/// the stop-short the place waits on, to be delivered to each recipient,
/// to the role that stop-short named and to the answerer `answerer` reads
/// of the place as the stall is recorded (43, 130, 193; core.24). A hold
/// on the waiting move holds it from stalling until its bound passes:
/// the settle that reads it passed lapses it, recorded with when, and
/// stalls the place again at once, its stall naming the lapsed hold's
/// words. A stall an earlier flume recorded, which read neither the
/// answerer nor the stop-short, is read for them at the first settle that
/// reads it while the place still waits so, and delivered as a new stall
/// would be. A hold with words never stalls, a stage waiting for room is
/// no move of its caller's (70), and nor is a base place's wait on the
/// places on its branch (203).
pub(crate) fn read_stall(
    next: &mut HeldState,
    now: Seconds,
    on_branch: Option<usize>,
    answerer: &dyn Fn(&HeldState) -> Option<String>,
) {
    refresh_waiting(next, now, on_branch);
    let Some(waiting) = &next.waiting else {
        return;
    };
    if waiting.on.why == Waits::OnHold && waiting.on.words.is_some() {
        return;
    }
    let mut since = waiting.on.since.unwrap_or(now);
    let mut lapsed = None;
    if let Some(hold) = next.waiting_hold() {
        match hold.bound {
            Some(bound) if now.saturating_sub(hold.since) >= bound => {
                lapsed = Some(hold.words.clone().unwrap_or_default());
                since = now;
            }
            _ => return,
        }
    } else if waiting.stalled {
        read_earlier_stall(next, since, answerer);
        return;
    } else if now.saturating_sub(since) < next.stall_after {
        return;
    }
    let words = waiting.on.move_words();
    let stop_short = stop_short_waited(next, &waiting.on);
    let answerer = answerer(next);
    if lapsed.is_some() {
        for hold in &mut next.holds {
            if hold.on == HoldOn::Waiting && hold.lapsed.is_none() {
                hold.lapsed = Some(now);
            }
        }
    }
    if let Some(waiting) = &mut next.waiting {
        waiting.stalled = true;
    }
    let of = EndOf::Stall {
        since,
        waiting: words,
        answerer,
        stop_short: stop_short.map(Box::new),
        answerer_read: true,
        lapsed,
    };
    if !next.ends.iter().any(|end| end.same(&of)) {
        next.ends.push(EndHeld {
            of,
            read_at: now,
            deliveries: vec![],
            answer: None,
            claim: None,
            noticing: vec![],
        });
    }
}

/// Reads, for the stall of the move the place still waits on since
/// `since` that an earlier flume recorded, the answerer behind its stream
/// and the stop-short it waits on, so the stall stands pending for them
/// (130, 193).
fn read_earlier_stall(
    next: &mut HeldState,
    since: Seconds,
    answerer: &dyn Fn(&HeldState) -> Option<String>,
) {
    let earlier = |end: &EndHeld| {
        matches!(
            end.of,
            EndOf::Stall { since: at, answerer_read: false, .. } if at == since
        )
    };
    if !next.ends.iter().any(earlier) {
        return;
    }
    let Some(waiting) = &next.waiting else {
        return;
    };
    let read = stop_short_waited(next, &waiting.on);
    let found = answerer(next);
    let Some(EndOf::Stall {
        answerer,
        stop_short,
        answerer_read,
        ..
    }) = next
        .ends
        .iter_mut()
        .rev()
        .find(|end| earlier(end))
        .map(|end| &mut end.of)
    else {
        return;
    };
    if answerer.is_none() {
        *answerer = found;
    }
    if stop_short.is_none() {
        *stop_short = read.map(Box::new);
    }
    *answerer_read = true;
}

/// The stop-short the move `waiting` answers: the end of the place's
/// current stage's start that ended short, where the move is a retry
/// that answers it, with its words, whether it needs an answer or found
/// work, its key and the role it named; none for any other move (41, 42,
/// 130).
pub(crate) fn stop_short_waited(held: &HeldState, waiting: &WaitingOn) -> Option<StopShortWaited> {
    if waiting.act != Act::Retry || !matches!(waiting.why, Waits::Answer | Waits::PastBound) {
        return None;
    }
    let stage = &held.stages[held.current?];
    if stage.state != StageState::EndedShort {
        return None;
    }
    held.ends.iter().rev().find_map(|end| match &end.of {
        EndOf::Stage {
            stage: name,
            start,
            state: StageState::EndedShort,
            end: read,
            ..
        } if *name == stage.name && *start == stage.starts => Some(StopShortWaited {
            words: read.words.clone(),
            needs: read.needs,
            key: end.of.end_key(),
            addressee: read.addressee.clone(),
        }),
        _ => None,
    })
}
