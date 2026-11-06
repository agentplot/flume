//! flume's list (200; core.52): a line for each place on the host under
//! its repository, a place that landed or was dropped only where every
//! place is asked, a tidied one whose document stays among them then, and
//! a line for each stream the host declares. A place's line is read
//! from its held state and its live agent's asking mark alone, in the
//! words its own line gives it (56, 139; core.14); a stream's from the
//! host's configuration and its terminal's runs, asked once (107, 186,
//! 190, 201). No worktree, head, deliverable, change or submission is
//! read, no terminal asked of a place's agent and no repository's binding
//! resolved; it takes no lock, writes nothing, settles nothing and
//! refuses nothing (57; core.13, core.16, core.17).

use crate::line::{stage_at, state_words, SEPARATOR};
use crate::room::StreamRoom;
use crate::terminals::{uncarried, Setting};
use crate::{label, Answer, Engine, Listed, StreamListed, WaitsOn};
use flume_seams::*;

impl Engine {
    /// Each place's line, by repository, then by `<kind>--<name>`, then
    /// in the order the store gives their documents, a place that landed
    /// or was dropped left out unless `every` place is asked (63, 200);
    /// and every declared stream's line, `default` among them, by
    /// priority and then by name, as status gives the streams (70, 143,
    /// 200).
    pub fn list(&self, every: bool) -> Answer {
        let mut places: Vec<Listed> = self
            .store
            .read_every()
            .iter()
            .filter(|read| every || !landed_or_dropped(&read.state.phase))
            .map(|read| self.listed(&read.state))
            .collect();
        order(&mut places);
        let mut streams: Vec<(i64, StreamListed)> = self
            .host
            .streams()
            .into_iter()
            .map(|(name, declared)| (declared.priority, self.stream_listed(&name, &declared)))
            .collect();
        streams.sort_by(|(a, one), (b, other)| b.cmp(a).then(one.stream.cmp(&other.stream)));
        Answer::List {
            places,
            streams: streams.into_iter().map(|(_, stream)| stream).collect(),
        }
    }

    /// The place's line from its held state and its live agent's asking
    /// mark (90): the stage a live place's line names with no group, its
    /// state in its line's words, whom it waits on and what keeps it.
    fn listed(&self, held: &HeldState) -> Listed {
        let place = &held.place;
        let at = (held.phase == Phase::Live)
            .then(|| stage_at(held))
            .flatten();
        let stage = at.map(|index| label(&held.stages[index].name, None));
        let state = state_words(held, at, self.asking(held));
        let waits_on = waits_on(held, &state);
        let kept = kept_words(&held.phase);
        let name = place.short();
        let line = joined(&[
            Some(name.as_str()),
            stage.as_deref(),
            Some(state.as_str()),
            waits_on.map(WaitsOn::words),
            kept.as_deref(),
        ]);
        Listed {
            place: place.id.clone(),
            repository: place.repository.clone(),
            name,
            kind: format!("{}@{}", place.kind.name, place.kind.version),
            stage,
            state,
            waits_on,
            kept,
            line,
        }
    }

    /// The stream's line: its session as an open resolves it, and its
    /// terminal's runs of that session asked once, or no terminal asked
    /// where its multiplexer names no profile the resolver carries (107,
    /// 186, 190, 201).
    fn stream_listed(&self, name: &str, declared: &Stream) -> StreamListed {
        let session = StreamRoom {
            session: declared.session.clone(),
            ..StreamRoom::default()
        }
        .session(name);
        let multiplexer = self.host.display_for(name).multiplexer;
        let (runs, uncarried) = match self.resolver.terminal(&multiplexer.profile) {
            Some(terminal) => (Some(terminal.runs(session.name.as_deref())), None),
            None => (
                None,
                Some(uncarried(
                    name,
                    Setting::Multiplexer,
                    &multiplexer,
                    &self.resolver.terminal_profiles(),
                )),
            ),
        };
        let line = stream_line(name, &session, runs.as_ref(), &multiplexer.profile);
        StreamListed {
            stream: name.to_string(),
            session,
            multiplexer,
            runs,
            uncarried,
            line,
        }
    }
}

/// The places by repository, then by `<kind>--<name>`, a place's
/// documents in the order the store gave them, since the sort is stable
/// (62, 200).
fn order(places: &mut [Listed]) {
    places.sort_by(|a, b| {
        a.repository
            .cmp(&b.repository)
            .then_with(|| a.name.cmp(&b.name))
    });
}

/// Whether the place landed or was dropped, kept, being tidied or tidied
/// alike: it has nothing left to open, and the list gives it only where
/// every place is asked. A place its caller declared done is not one
/// (63, 200).
fn landed_or_dropped(phase: &Phase) -> bool {
    match phase {
        Phase::Over { over, .. } | Phase::Gone { over, .. } | Phase::Tidied { over, .. } => {
            matches!(over, Over::Landed { .. } | Over::Dropped)
        }
        Phase::Making { .. } | Phase::Live => false,
    }
}

/// Whom a live place waits on, from its held state alone: its caller
/// where it waits on its caller's move, stalled or not (130); the host
/// where a stage waits for room and nothing waits on its caller (70); the
/// merge queue where it is `submitted` (111); and nobody for an agent's
/// own question in its pane (90).
fn waits_on(held: &HeldState, state: &str) -> Option<WaitsOn> {
    if held.phase != Phase::Live {
        None
    } else if held.waiting.is_some() {
        Some(WaitsOn::Caller)
    } else if held
        .stages
        .iter()
        .any(|stage| stage.state == StageState::WaitingForRoom)
    {
        Some(WaitsOn::Host)
    } else if state == "submitted" {
        Some(WaitsOn::MergeQueue)
    } else {
        None
    }
}

/// What keeps a place whose work is over, in a word, from its phase:
/// the step its teardown failed at by the teardown's own name for it,
/// what it holds, its live agent or its unread state; `being tidied` once
/// removed with its branch and document still to go, and `tidied` for a
/// document kept for its ends; nothing for a place whose tidy has not run
/// or whose work is not over. The teardown's words and what the holds
/// name are status's (62, 64 to 66, 68).
fn kept_words(phase: &Phase) -> Option<String> {
    let words = match phase {
        Phase::Over {
            kept: Some(kept), ..
        } => match kept {
            Kept::Teardown { step, .. } => return Some(format!("kept by {step}")),
            Kept::Holds { holds } => {
                let work = !holds.uncommitted.is_empty() || !holds.untracked.is_empty();
                match (work, holds.lock.is_some()) {
                    (true, true) => "kept holding work and a lock",
                    (false, true) => "kept holding a lock",
                    // A tidy keeps a place by its holds only where it holds
                    // something (64), so holding no lock it holds work.
                    (_, false) => "kept holding work",
                }
            }
            Kept::AgentLive { .. } => "kept while its agent runs",
            Kept::Unreadable { .. } => "kept unread",
        },
        Phase::Gone { .. } => "being tidied",
        Phase::Tidied { .. } => "tidied",
        Phase::Making { .. } | Phase::Live | Phase::Over { kept: None, .. } => return None,
    };
    Some(words.to_string())
}

/// The parts given, joined by ` · `, a part the place has none of left
/// out.
fn joined(parts: &[Option<&str>]) -> String {
    parts
        .iter()
        .flatten()
        .copied()
        .collect::<Vec<_>>()
        .join(SEPARATOR)
}

/// A stream's line: `<stream> · session <name> · <runs>`, the name the
/// session's, or for the terminal's own the name its runs gives, else
/// `its terminal's own session`; `<stream> · no session` where the
/// terminal holds no part; and `no profile for <multiplexer>` where no
/// terminal was asked (107, 186, 190, 201).
fn stream_line(
    stream: &str,
    session: &SessionHeld,
    runs: Option<&SessionRuns>,
    multiplexer: &str,
) -> String {
    let named = |name: &str| format!("session {name}");
    let session = match (runs, &session.name) {
        (Some(SessionRuns::NoPart { .. }), _) => None,
        (_, Some(name)) => Some(named(name)),
        (Some(SessionRuns::Runs { name } | SessionRuns::NotRunning { name }), None) => {
            Some(named(name))
        }
        (_, None) => Some("its terminal's own session".to_string()),
    };
    let runs = match runs {
        Some(SessionRuns::Runs { .. }) => "runs".to_string(),
        Some(SessionRuns::NotRunning { .. }) => "not running".to_string(),
        Some(SessionRuns::NoPart { .. }) => "no session".to_string(),
        Some(SessionRuns::Unread { .. }) => "not read".to_string(),
        None => format!("no profile for {multiplexer}"),
    };
    joined(&[Some(stream), session.as_deref(), Some(runs.as_str())])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn listed(repository: &str, name: &str, state: &str) -> Listed {
        Listed {
            place: PlaceId::new(format!("{repository}/places/{name}")),
            repository: repository.into(),
            name: name.into(),
            kind: "fix@1".into(),
            stage: None,
            state: state.into(),
            waits_on: None,
            kept: None,
            line: format!("{name} · {state}"),
        }
    }

    fn over(kept: Option<Kept>) -> Phase {
        Phase::Over {
            over: Over::Landed {
                commit: Revision::new("c1"),
            },
            kept,
        }
    }

    fn holds(work: bool, lock: bool) -> Phase {
        over(Some(Kept::Holds {
            holds: Holds {
                uncommitted: if work { vec!["a.rs".into()] } else { vec![] },
                untracked: vec![],
                lock: lock.then(|| "index.lock".into()),
            },
        }))
    }

    #[test]
    fn the_kept_words_follow_the_phase_in_the_designs_order() {
        let teardown = over(Some(Kept::Teardown {
            step: "release".into(),
            words: "line\n".repeat(40),
        }));
        let untracked = over(Some(Kept::Holds {
            holds: Holds {
                untracked: vec!["scratch.txt".into()],
                ..Holds::default()
            },
        }));
        let rows = [
            (teardown, Some("kept by release")),
            (holds(true, false), Some("kept holding work")),
            (untracked, Some("kept holding work")),
            (holds(false, true), Some("kept holding a lock")),
            (holds(true, true), Some("kept holding work and a lock")),
            (
                over(Some(Kept::AgentLive {
                    identity: "r:fix--one:code:1".into(),
                })),
                Some("kept while its agent runs"),
            ),
            (
                over(Some(Kept::Unreadable {
                    reason: "the worktree could not be read".into(),
                })),
                Some("kept unread"),
            ),
            (
                Phase::Gone {
                    over: Over::Done,
                    at: None,
                },
                Some("being tidied"),
            ),
            (
                Phase::Tidied {
                    over: Over::Dropped,
                    at: 1,
                    pending_for: vec!["operator".into()],
                },
                Some("tidied"),
            ),
            (over(None), None),
            (Phase::Live, None),
            (
                Phase::Making {
                    by: "launch".into(),
                },
                None,
            ),
        ];
        for (phase, words) in rows {
            assert_eq!(kept_words(&phase).as_deref(), words, "{phase:?}");
        }
    }

    #[test]
    fn a_place_that_landed_or_was_dropped_is_left_out_whatever_keeps_it() {
        let landed = || Over::Landed {
            commit: Revision::new("c1"),
        };
        let rows = [
            (over(None), true),
            (holds(true, false), true),
            (
                Phase::Over {
                    over: Over::Dropped,
                    kept: None,
                },
                true,
            ),
            (
                Phase::Gone {
                    over: Over::Dropped,
                    at: None,
                },
                true,
            ),
            (
                Phase::Tidied {
                    over: landed(),
                    at: 1,
                    pending_for: vec!["operator".into()],
                },
                true,
            ),
            (
                Phase::Over {
                    over: Over::Done,
                    kept: None,
                },
                false,
            ),
            (
                Phase::Tidied {
                    over: Over::Done,
                    at: 1,
                    pending_for: vec!["operator".into()],
                },
                false,
            ),
            (Phase::Live, false),
            (
                Phase::Making {
                    by: "launch".into(),
                },
                false,
            ),
        ];
        for (phase, left_out) in rows {
            assert_eq!(landed_or_dropped(&phase), left_out, "{phase:?}");
        }
    }

    #[test]
    fn the_places_come_by_repository_then_name_then_the_stores_order() {
        let mut places = vec![
            listed("/b", "fix--one", "working"),
            listed("/a", "fix--two", "working"),
            listed("/a", "chore--one", "landed"),
            listed("/a", "fix--two", "tidied"),
            listed("/a", "change--one", "held"),
        ];
        order(&mut places);
        let got: Vec<(&str, &str, &str)> = places
            .iter()
            .map(|place| {
                (
                    place.repository.as_str(),
                    place.name.as_str(),
                    place.state.as_str(),
                )
            })
            .collect();
        assert_eq!(
            got,
            vec![
                ("/a", "change--one", "held"),
                ("/a", "chore--one", "landed"),
                ("/a", "fix--two", "working"),
                ("/a", "fix--two", "tidied"),
                ("/b", "fix--one", "working"),
            ]
        );
    }

    #[test]
    fn a_line_joins_its_parts_in_order_leaving_out_what_it_has_none_of() {
        assert_eq!(
            joined(&[
                Some("fix--one"),
                Some("code"),
                Some("waiting for room"),
                Some(WaitsOn::Host.words()),
                None,
            ]),
            "fix--one · code · waiting for room · waits on the host"
        );
        assert_eq!(
            joined(&[
                Some("chore--one"),
                None,
                Some("landed"),
                None,
                Some("kept by release"),
            ]),
            "chore--one · landed · kept by release"
        );
    }

    #[test]
    fn a_streams_line_names_its_session_and_whether_it_runs() {
        let session = |name: Option<&str>, from| SessionHeld {
            name: name.map(str::to_string),
            from,
        };
        let work = session(Some("work"), SessionFrom::StreamName);
        let own = session(None, SessionFrom::Own);
        let rows = [
            (
                stream_line(
                    "work",
                    &work,
                    Some(&SessionRuns::Runs {
                        name: "work".into(),
                    }),
                    "fake",
                ),
                "work · session work · runs",
            ),
            (
                stream_line(
                    "office",
                    &session(Some("desk"), SessionFrom::Stream),
                    Some(&SessionRuns::NotRunning {
                        name: "desk".into(),
                    }),
                    "fake",
                ),
                "office · session desk · not running",
            ),
            (
                stream_line(
                    "default",
                    &own,
                    Some(&SessionRuns::Runs {
                        name: "flume".into(),
                    }),
                    "fake",
                ),
                "default · session flume · runs",
            ),
            (
                stream_line(
                    "default",
                    &own,
                    Some(&SessionRuns::Unread {
                        reason: "no answer".into(),
                    }),
                    "fake",
                ),
                "default · its terminal's own session · not read",
            ),
            (
                stream_line(
                    "work",
                    &work,
                    Some(&SessionRuns::NoPart {
                        reason: "it holds no session".into(),
                    }),
                    "printed",
                ),
                "work · no session",
            ),
            (
                stream_line("trial", &work, None, "rex"),
                "trial · session work · no profile for rex",
            ),
        ];
        for (got, want) in rows {
            assert_eq!(got, want);
        }
    }
}
