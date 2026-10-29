//! A place's held state, read with its version and written at it (73 to
//! 76, 102, 103): every act reads, decides and writes at the version it
//! read, and a write refused because the version moved is read again and
//! decided again, so two acts on one place leave it as one or the other
//! would have (75).

use crate::status::waiting_on;
use crate::Engine;
use flume_seams::{
    EndHeld, EndOf, HeldState, Over, Phase, PlaceId, Seconds, StageEnd, StageState, Verdict,
    Versioned, Waiting, Waits, Written,
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
    /// since when a place waits (56, 130).
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
        refresh_waiting(next, now);
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
/// record. Written in the same write as the end itself, so whatever act
/// or settle reads an end records it, and a repeat records nothing
/// (43, 74, 133).
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
            end: stage.end.clone().unwrap_or_else(StageEnd::default),
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
    for of in read {
        if !next.ends.iter().any(|end| end.same(&of)) {
            next.ends.push(EndHeld {
                of,
                read_at: now,
                deliveries: vec![],
            });
        }
    }
}

/// Reads, from the held state alone, the move the place waits on its
/// caller for: a move read for the first time is kept with since when
/// it stood so, the same move keeps its since and whether it stalled,
/// and none clears it (56, 130; design 5).
pub(crate) fn refresh_waiting(next: &mut HeldState, now: Seconds) {
    next.waiting = waiting_on(next).map(|mut on| match &next.waiting {
        Some(was) if was.on.same_move(&on) => {
            on.since = was.on.since;
            Waiting {
                on,
                stalled: was.stalled,
            }
        }
        _ => {
            on.since = Some(on.since.unwrap_or(now));
            Waiting { on, stalled: false }
        }
    });
}

/// Reads the place stalled where it has waited on its caller under no
/// hold's words for its bound, recording the stall as an end once, to
/// be delivered to each recipient (43, 130; core.24). A hold with words
/// never stalls, and a stage waiting for room is no move of its
/// caller's (70).
pub(crate) fn read_stall(next: &mut HeldState, now: Seconds) {
    refresh_waiting(next, now);
    let Some(waiting) = &mut next.waiting else {
        return;
    };
    let held = waiting.on.why == Waits::OnHold && waiting.on.words.is_some();
    let since = waiting.on.since.unwrap_or(now);
    if waiting.stalled || held || now.saturating_sub(since) < next.stall_after {
        return;
    }
    waiting.stalled = true;
    let of = EndOf::Stall {
        since,
        waiting: waiting.on.move_words(),
    };
    if !next.ends.iter().any(|end| end.same(&of)) {
        next.ends.push(EndHeld {
            of,
            read_at: now,
            deliveries: vec![],
        });
    }
}
