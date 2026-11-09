//! A base place's fixes (23, 112, 203, S120; core.53): where a base
//! place's kind names the kind its fixes run as, a forecast in conflict
//! with its base itself, or a failed or conflicting submission of it,
//! launches a place of that kind on its branch inside the settle that
//! reads it, with no caller's word (1, I6). The fix is written onto the
//! base place by compare-and-set before it is launched, by its number,
//! name, place, cause and when, so a settle that finds one written with
//! no place standing for it launches it, and a launch refused because the
//! fix's place now stands takes that place as the fix (17, 74, 75). Each
//! forecast, by its head and base's revision, and each submission is acted
//! on once; while a fix is live none is forecast or launched; every fix,
//! a refused launch among them, counts toward its kind's bound, flume's
//! default for a stage's where it gives none, past which the base place
//! waits on its caller with what was found (130). A fix is its base
//! place's: on its branch, of the identity its launch resolved, named
//! `<base place's name>-<n>`, in its stream, at its priority, serving the
//! intent its launch named, delivering to its recipients, its brief one
//! line saying what it is for over the merge queue's words whole (43, 70,
//! 143, 147, 202; core.36). How it ended is written onto its base place
//! by its own settle before it is tidied, so a fix's end is never lost
//! and every fix written live whose place does not stand is one not yet
//! launched (63, 75). flume decides nothing of what a fix does: its kind
//! decides, and the merge queue's words (I6).

use crate::launch::and_list;
use crate::{Engine, Step};
use flume_seams::*;

/// What launches a base place's fix: its cause, what was found as the
/// base place's wait gives it, and the fix's brief (112, 203).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Found {
    pub cause: FixCause,
    /// What was found, with the merge queue's words.
    pub said: String,
    /// One line saying what the fix is for, a blank line, and the merge
    /// queue's words whole.
    pub brief: String,
}

impl Found {
    /// A forecast of `place`'s branch in conflict with its base on
    /// `files` at `onto`, the head `head` (203, 210).
    pub(crate) fn forecast(
        place: &PlaceRecord,
        head: &Revision,
        onto: &Revision,
        files: &[String],
        words: &str,
    ) -> Found {
        Found {
            cause: FixCause::Forecast {
                head: head.clone(),
                onto: onto.clone(),
            },
            said: format!("the merge queue forecast a conflict with its base: {words}"),
            brief: brief(&forecast_line(place, onto, files), words),
        }
    }

    /// The failed or conflicting submission `submission` of `place`, as
    /// its verdict gives it, a conflict at the base's revision `onto`
    /// where it is known; none for any other verdict (112, 203).
    pub(crate) fn submission(
        place: &PlaceRecord,
        submission: &str,
        verdict: &Verdict,
        onto: Option<&Revision>,
    ) -> Option<Found> {
        match verdict {
            Verdict::Failed { words } => Some(Found {
                cause: FixCause::Failed {
                    submission: submission.into(),
                },
                said: format!("the submission failed: {words}"),
                brief: brief(&failed_line(place), words),
            }),
            Verdict::Conflict { files, words } => Some(Found {
                cause: FixCause::Conflicted {
                    submission: submission.into(),
                },
                said: format!("the submission conflicted with its base: {words}"),
                brief: brief(&conflicted_line(place, onto, files), words),
            }),
            _ => None,
        }
    }

    /// What found the fix `cause` names, read again from the base place's
    /// held state for a fix written and never launched: the forecast it
    /// holds where it is that cause's, else the submission's verdict; the
    /// merge queue's words none where neither holds them now (17, 74).
    fn again(held: &HeldState, cause: &FixCause) -> Found {
        let place = &held.place;
        match cause {
            FixCause::Forecast { head, onto } => {
                match held.forecast.as_ref().map(|at| &at.forecast) {
                    Some(Forecast::Conflict {
                        head: at,
                        onto: on,
                        files,
                        words,
                        ..
                    }) if at == head && on == onto => {
                        Found::forecast(place, head, onto, files, words)
                    }
                    _ => Found::forecast(place, head, onto, &[], ""),
                }
            }
            FixCause::Failed { submission } | FixCause::Conflicted { submission } => {
                let verdict = match (&held.submission.verdict, cause) {
                    (Some(verdict @ Verdict::Failed { .. }), FixCause::Failed { .. })
                    | (Some(verdict @ Verdict::Conflict { .. }), FixCause::Conflicted { .. }) => {
                        verdict.clone()
                    }
                    (_, FixCause::Failed { .. }) => Verdict::Failed {
                        words: String::new(),
                    },
                    _ => Verdict::Conflict {
                        files: vec![],
                        words: String::new(),
                    },
                };
                Found::submission(place, submission, &verdict, None).unwrap_or_else(|| Found {
                    cause: cause.clone(),
                    said: String::new(),
                    brief: String::new(),
                })
            }
        }
    }
}

/// A fix's brief: the line saying what it is for, a blank line, and the
/// merge queue's words whole, unrewritten (112; core.36).
fn brief(line: &str, words: &str) -> String {
    if words.trim().is_empty() {
        return line.to_string();
    }
    format!("{line}\n\n{words}")
}

/// The files contested as a sentence names them.
fn files_named(files: &[String]) -> String {
    if files.is_empty() {
        return "the files the merge queue names".into();
    }
    and_list(files.to_vec())
}

/// The first line of a forecast conflict's fix: take the base's revision
/// in by a merge, before anything else (112, 203).
pub(crate) fn forecast_line(place: &PlaceRecord, onto: &Revision, files: &[String]) -> String {
    format!(
        "Take {} at {onto} into {}, which the merge queue forecast to conflict with it on {}: merge that revision in and commit it, resolving the files contested, before anything else.",
        place.base,
        place.branch,
        files_named(files)
    )
}

/// The first line of a conflicting submission's fix, at the base's
/// revision the merge queue tested against where it is known (112, 203).
pub(crate) fn conflicted_line(
    place: &PlaceRecord,
    onto: Option<&Revision>,
    files: &[String],
) -> String {
    let at = onto.map(|onto| format!(" at {onto}")).unwrap_or_default();
    format!(
        "Take {}{at} into {}, whose submission conflicted with it on {}: merge that revision in and commit it, resolving the files contested, before anything else.",
        place.base,
        place.branch,
        files_named(files)
    )
}

/// The first line of a failed submission's fix (112, 203).
pub(crate) fn failed_line(place: &PlaceRecord) -> String {
    format!(
        "Make {} pass the checks of {} that failed on its submission.",
        place.branch, place.base
    )
}

/// A fix's cause as a few words name it, in a base place's wait and the
/// person's form of status (56, 203).
pub fn cause_words(cause: &FixCause) -> &'static str {
    match cause {
        FixCause::Forecast { .. } => "a forecast conflict",
        FixCause::Failed { .. } => "a failed submission",
        FixCause::Conflicted { .. } => "a conflicting submission",
    }
}

/// The fix `found` launches decided in the base place's held state, in the
/// write that holds what found it: nothing where a fix of the same cause
/// was launched, or one is live, which leaves the base place waiting on
/// it; past the kind's bound, `limit` where it gives none, the base place
/// waits on its caller with what was found and the bound spent; and
/// otherwise the fix written live, its number the next from 1, named
/// `<name>-<n>`, at the place `place_of` gives that name, with its cause
/// and `now`, the base place's wait cleared. The fix's number to launch
/// (23, 130, 203; core.17).
pub(crate) fn decide(
    held: &mut HeldState,
    found: &Found,
    limit: u32,
    now: Seconds,
    place_of: &dyn Fn(&str) -> PlaceId,
) -> Option<u32> {
    if held.fixes.iter().any(|fix| fix.cause == found.cause) {
        return None;
    }
    if held.fixes.iter().any(|fix| fix.ended == FixEnded::Live) {
        return None;
    }
    let allowed = held.place.kind.fix_limit.unwrap_or(limit);
    let count = u32::try_from(held.fixes.len()).unwrap_or(u32::MAX);
    if count >= allowed {
        let spent = match count {
            1 => "Its kind's 1 fix was launched, as many as it allows".to_string(),
            n => format!("Its kind's {n} fixes were launched, as many as it allows"),
        };
        held.submission.waiting = Some(format!("{}. {spent}", found.said));
        return None;
    }
    let number = count + 1;
    let name = format!("{}-{number}", held.place.name);
    held.fixes.push(BaseFix {
        number,
        place: place_of(&name),
        name,
        cause: found.cause.clone(),
        launched: now,
        ended: FixEnded::Live,
    });
    held.submission.waiting = None;
    Some(number)
}

impl Engine {
    /// Where a fix of the base place `held` named `name` is made, as its
    /// place profile gives the location of `<kind>--<name>`, the kind its
    /// fixes run as; the place's short name where none can be found, whose
    /// launch is then refused naming why (3, 9, C.1).
    pub(crate) fn fix_place_of(&self, held: &HeldState) -> impl Fn(&str) -> PlaceId {
        let repository = held.place.repository.clone();
        let kind = held
            .place
            .fixes_as
            .as_ref()
            .map(|fixes| fixes.identity.name.clone())
            .unwrap_or_default();
        let place = self.bound(&held.place).ok().map(|bound| bound.place);
        move |name: &str| {
            let short = format!("{kind}--{name}");
            place
                .as_ref()
                .and_then(|place| place.location(&repository, &short).ok())
                .map_or_else(|| PlaceId::new(short), PlaceId::new)
        }
    }

    /// The place standing as the base place `base`'s fix `number`, by the
    /// base place and number it holds, a tidied document aside (3, 203).
    pub(crate) fn fix_standing(&self, base: &PlaceId, number: u32) -> Option<PlaceId> {
        self.store
            .read_every()
            .into_iter()
            .map(|read| read.state)
            .find(|held| {
                !matches!(held.phase, Phase::Tidied { .. })
                    && held
                        .place
                        .fix_of
                        .as_ref()
                        .is_some_and(|of| of.place == *base && of.number == number)
            })
            .map(|held| held.place.id)
    }

    /// Launches the base place `base`'s fix `number`, written live onto
    /// it, inside the settle, as `wake.rs` launches a role's run: of the
    /// identity its launch resolved, on its branch, named as written, in
    /// its stream at its priority, serving the intent its launch named,
    /// delivering to its recipients, with its stall bound and nothing else
    /// of its own, its brief what `found` found, or what the base place
    /// holds of its cause where none is given. A launch refused because
    /// the fix's place now stands takes that place as the fix; any other
    /// refusal is held on the fix with its words, counting toward the
    /// bound, and the base place waits on its caller with them (3, 4, 43,
    /// 70, 143, 147, 203; core.28).
    pub(crate) fn launch_fix(&self, base: &PlaceId, number: u32, found: Option<&Found>) {
        let Some(read) = self.store.read(base) else {
            return;
        };
        let held = read.state;
        let place = &held.place;
        let (Some(fixes_as), Some(fix)) = (
            &place.fixes_as,
            held.fixes.iter().find(|fix| fix.number == number),
        ) else {
            return;
        };
        if fix.ended != FixEnded::Live {
            return;
        }
        let found = found
            .cloned()
            .unwrap_or_else(|| Found::again(&held, &fix.cause));
        let launch = Launch {
            repository: place.repository.clone(),
            kind: fixes_as.identity.spelled.clone(),
            name: fix.name.clone(),
            base: place.branch.clone(),
            brief: Some(found.brief.clone()),
            declaration: (fixes_as.from.source == LAUNCH_SOURCE)
                .then(|| fixes_as.from.declaration_file.clone())
                .flatten(),
            priority: held.priority,
            recipients: place.recipients.clone(),
            stall_after: Some(held.stall_after),
            stream: Some(place.stream.name.clone()),
            intent: match &place.intent {
                IntentKey::Launch { name, .. } => Some(name.clone()),
                IntentKey::Base { .. } => None,
            },
            ..Launch::default()
        };
        let fix_of = FixOf {
            place: base.clone(),
            number,
            cause: fix.cause.clone(),
        };
        let Err(reason) = self.make_place(&launch, None, Some(&fix_of)) else {
            return;
        };
        if self.fix_standing(base, number).is_some() {
            return;
        }
        let identity = fixes_as.identity.spelled.clone();
        self.update(base, |held| {
            let Some(fix) = held.fixes.iter_mut().find(|fix| fix.number == number) else {
                return Step::Keep(());
            };
            if fix.ended != FixEnded::Live {
                return Step::Keep(());
            }
            fix.ended = FixEnded::Refused {
                reason: reason.clone(),
            };
            held.submission.waiting = Some(format!(
                "The fix {number}, {identity}, was not launched: {reason}. {}",
                found.said
            ));
            Step::Write(())
        });
    }

    /// A fix written live onto the base place `id` whose place does not
    /// stand, launched now, so a command that died between the two loses
    /// nothing; true where a fix of the base place's is written live,
    /// standing or not, so nothing is forecast or launched beside it (17,
    /// 74, 203).
    pub(crate) fn live_fix(&self, id: &PlaceId, held: &HeldState) -> bool {
        let Some(fix) = held.fixes.iter().find(|fix| fix.ended == FixEnded::Live) else {
            return false;
        };
        if self.fix_standing(id, fix.number).is_none() {
            self.launch_fix(id, fix.number, None);
        }
        true
    }

    /// Writes how the fix `held` ended onto its base place's fixes, by
    /// compare-and-set, once its work is over: landed with its commit, or
    /// dropped, the base place then waiting on its caller with words
    /// naming the fix; a base place whose document is gone takes nothing.
    /// True once written or nothing is to be written, so the fix is tidied
    /// only after (63, 75, 130, 203).
    pub(crate) fn fix_ended(&self, held: &HeldState, over: &Over) -> bool {
        let Some(of) = &held.place.fix_of else {
            return true;
        };
        let ended = match over {
            Over::Landed { commit } => FixEnded::Landed {
                commit: commit.clone(),
            },
            Over::Dropped | Over::Done => FixEnded::Dropped,
        };
        let short = held.place.short();
        let id = &held.place.id;
        let written = self.update(&of.place, |base| {
            let Some(fix) = base
                .fixes
                .iter_mut()
                .find(|fix| fix.number == of.number && fix.place == *id)
            else {
                return Step::Keep(());
            };
            if fix.ended != FixEnded::Live {
                return Step::Keep(());
            }
            fix.ended = ended.clone();
            if ended == FixEnded::Dropped && base.phase == Phase::Live {
                base.submission.waiting = Some(format!(
                    "its fix {short}, launched for {}, was dropped",
                    cause_words(&of.cause)
                ));
            }
            Step::Write(())
        });
        written.is_some() || self.store.read(&of.place).is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spring() -> PlaceRecord {
        let mut place = crate::tests::record(Binding::default());
        place.name = "spring".into();
        place.branch = "train/spring".into();
        place.base = "main".into();
        place
    }

    #[test]
    fn a_forecast_conflicts_fix_takes_the_base_in() {
        let onto = Revision::new("c0ffee");
        let found = Found::forecast(
            &spring(),
            &Revision::new("aaa111"),
            &onto,
            &["README.md".into()],
            "would conflict: README.md with main",
        );
        let mut lines = found.brief.lines();
        assert_eq!(
            lines.next(),
            Some(
                "Take main at c0ffee into train/spring, which the merge queue forecast to conflict with it on README.md: merge that revision in and commit it, resolving the files contested, before anything else."
            )
        );
        assert_eq!(lines.next(), Some(""));
        assert_eq!(
            lines.collect::<Vec<_>>(),
            vec!["would conflict: README.md with main"]
        );
    }

    #[test]
    fn a_conflicting_submissions_fix_takes_the_base_in() {
        let verdict = Verdict::Conflict {
            files: vec!["README.md".into(), "src/lib.rs".into()],
            words: "README.md conflicts with main".into(),
        };
        let found = Found::submission(&spring(), "7", &verdict, Some(&Revision::new("c0ffee")))
            .expect("a conflict's fix");
        assert_eq!(
            found.brief.lines().next(),
            Some(
                "Take main at c0ffee into train/spring, whose submission conflicted with it on README.md and src/lib.rs: merge that revision in and commit it, resolving the files contested, before anything else."
            )
        );
        assert!(found.brief.ends_with("\n\nREADME.md conflicts with main"));
        assert_eq!(
            found.cause,
            FixCause::Conflicted {
                submission: "7".into()
            }
        );
    }

    #[test]
    fn a_failed_submissions_fix_makes_the_checks_pass() {
        let verdict = Verdict::Failed {
            words: "the check test failed".into(),
        };
        let found = Found::submission(&spring(), "8", &verdict, None).expect("a failure's fix");
        assert_eq!(
            found.brief,
            "Make train/spring pass the checks of main that failed on its submission.\n\nthe check test failed"
        );
        assert!(Found::submission(&spring(), "9", &Verdict::Withdrawn, None).is_none());
    }
}
