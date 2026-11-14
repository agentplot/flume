//! A place's ends and their notices (43, 95, core.11, core.25): each end
//! the held state records is a delivery to each recipient the launch
//! named, its notice one line naming the place, the end and how to read
//! it, handed into the recipient's own session at its next wait with
//! every other delivery pending for it, as `batch.rs` hands it (214). A
//! recipient the host's configuration gives a standing agent is reached
//! in that agent alone, found by its name in its stream's terminal and
//! session at each delivery, none or more than one of the name leaving
//! the end pending with nothing claimed (132, 190, C.7; core.28). A
//! recipient no pane holds is sent no notice, its ends pending until it
//! collects them by the inbox or a wait; and no notice carries a drop's
//! or a done's end, which stands pending for the place's recipients that
//! are no role's address until each collects it (43, 52, 114; core.25).
//! A notice an earlier flume claimed on the place is read as before, an
//! answered one recorded once, and none is claimed there now (75). A
//! place whose work is over and whose worktree is gone keeps its document
//! alone, apart from a live place's, while an end of it stands pending,
//! and it goes once every end is delivered or its caller drops it (3,
//! 62).

use crate::answer::and_list;
use crate::prompt::group_name;
use crate::{Engine, Step};
use flume_seams::*;

/// How much of a brief's first line a notice gives (43; design 9).
pub const NOTICE_BRIEF: usize = 60;

/// The end's delivery's message, one line every recipient shares: the
/// place, its brief's first line cut at [`NOTICE_BRIEF`] characters, what
/// ended, until when a stop-short's agent is kept open for its answer,
/// the end's key, and the command that reads it whole: `flume inbox`
/// where the place, as `held` gives it when the end's delivery is
/// recorded, waits on a move of its caller's, which gives the question
/// and how to answer it, and `flume status <place>` otherwise. It is
/// fixed when the delivery is recorded, and a notice is it with the way
/// its recipient confirms it after it ([`notice_to`]). It names no fact
/// to act on blindly, and no question's words, which are many lines (43,
/// 198; weir 100; core.11, core.25; design 4, 9).
pub fn notice(held: &HeldState, of: &EndOf) -> String {
    let place = &held.place;
    let short = place.short();
    let brief = place
        .brief_line()
        .map(|line| format!(" ({})", cut(line, NOTICE_BRIEF)))
        .unwrap_or_default();
    let read = if held.waiting.is_some() {
        "flume inbox".to_string()
    } else {
        format!("flume status {short}")
    };
    format!(
        "flume: {short}{brief} {} [{}]; read: {read}",
        end_words(of),
        of.end_key()
    )
}

/// The one line of notice sent to `recipient`: the delivery's `message`
/// as recorded and then `; confirm: ` with the command that confirms it
/// as that recipient ([`confirm_command`]), since a caller's own name, its
/// session's in a pane, is often not the name the end is owed under. A
/// message an earlier flume recorded with the way to confirm already in
/// it, `; confirm: flume confirm <key>`, is sent with that taken off and
/// the recipient's put in its stead (43; weir 100, 102; core.11, core.25).
pub fn notice_to(message: &str, key: &str, recipient: &str) -> String {
    let message = message
        .strip_suffix(&format!("; confirm: flume confirm {key}"))
        .unwrap_or(message);
    format!(
        "{message}; confirm: {}",
        confirm_command(recipient, std::slice::from_ref(&key))
    )
}

/// The command by which `recipient` confirms each delivery of `keys`,
/// `flume confirm --recipient <recipient> <key>…`, the recipient's name
/// quoted for a shell where it holds anything but letters, digits and
/// `_@%+=:,./-`; the one way a notice, a wait, the inbox and status say
/// how a recipient confirms (43; weir 102).
pub fn confirm_command(recipient: &str, keys: &[&str]) -> String {
    let plain = !recipient.is_empty()
        && recipient
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "_@%+=:,./-".contains(c));
    let recipient = if plain {
        recipient.to_string()
    } else {
        format!("'{}'", recipient.replace('\'', r"'\''"))
    };
    let mut command = format!("flume confirm --recipient {recipient}");
    for key in keys {
        command.push(' ');
        command.push_str(key);
    }
    command
}

/// `line` cut at `at` characters, an ellipsis saying where it was cut.
fn cut(line: &str, at: usize) -> String {
    let mut chars = line.chars();
    let head: String = chars.by_ref().take(at).collect();
    if chars.next().is_none() {
        head
    } else {
        format!("{}…", head.trim_end())
    }
}

/// What ended, in the words a notice and the inbox give it: what ended,
/// and for a stop-short whose agent was kept open for its answer when it
/// was read, until when (43, 198).
pub fn end_words(of: &EndOf) -> String {
    let what = what_ended(of);
    match of {
        EndOf::Stage {
            state: StageState::EndedShort,
            end,
            ..
        } => match end.open_until {
            Some(until) => format!("{what}, its agent kept open for the answer until {until}"),
            None => what,
        },
        _ => what,
    }
}

/// What ended, in the words a notice gives it, a repetition's end naming
/// its task group (43, 142).
pub fn what_ended(of: &EndOf) -> String {
    match of {
        EndOf::Stage {
            stage, state, end, ..
        } => {
            let stage = match &end.group {
                Some(group) => format!(
                    "{stage} on {}",
                    group_name(group.number.as_deref(), group.title.as_deref())
                ),
                None => stage.clone(),
            };
            match state {
                StageState::Delivered => format!("{stage} delivered"),
                StageState::EndedShort if end.needs == Some(StopKind::FoundWork) => {
                    format!("{stage} found work")
                }
                StageState::EndedShort => format!("{stage} needs an answer"),
                StageState::Finished => format!("{stage} finished"),
                StageState::Lost if end.left_in_pane => format!(
                    "{stage} lost, and flume left the agent the terminal brought back in its pane"
                ),
                StageState::Lost => format!("{stage} lost"),
                StageState::NeverBegun => match &end.unsettled {
                    Some(unsettled) => {
                        format!("{stage} never begun, its agent began {}", unsettled.words())
                    }
                    None => format!("{stage} never begun"),
                },
                StageState::Stuck => format!("{stage} stuck"),
                other => format!("{stage} {other:?}"),
            }
        }
        EndOf::Submission { verdict, .. } => match verdict {
            Verdict::Landed { commit, .. } => format!("submission landed {commit}"),
            Verdict::Failed { .. } | Verdict::Conflict { .. } => "submission failed".into(),
            Verdict::Withdrawn => "submission withdrawn".into(),
            Verdict::Refused { .. } => "submission refused".into(),
        },
        EndOf::Stall { waiting, .. } => format!("stalled: {waiting}"),
        EndOf::Over { over } => over.word().into(),
    }
}

impl Engine {
    /// Records, at its time, the hand-over of each notice claimed on the
    /// place whose hand-over the host's deliveries lack, its send answered
    /// sent and its record never written, so it is sent no second time;
    /// a claim still sending it leaves to its sender, which records it
    /// once answered sent or takes it back; true where it recorded any
    /// (43, 75; weir 103; design 5).
    pub(crate) fn record_claims(&self, held: &HeldState, owing: &Owing) -> bool {
        let mut recorded = false;
        for end in &held.ends {
            let Some(delivery) = end.delivery(&held.place, owing) else {
                continue;
            };
            for claim in end.noticing.iter().filter(|claim| !claim.sending) {
                let handed = delivery
                    .handovers
                    .iter()
                    .any(|handover| handover.recipient == claim.recipient);
                let owed = delivery
                    .owed
                    .iter()
                    .any(|owed| owed.recipient == claim.recipient);
                if handed || !owed {
                    continue;
                }
                let answers = self.hand_over(
                    std::slice::from_ref(&delivery.key),
                    &claim.recipient,
                    DeliveredBy::Notice,
                    claim.at,
                    false,
                );
                for answer in answers {
                    match answer {
                        Ok(true) => recorded = true,
                        Ok(false) => {}
                        Err(why) => self.host.log(&format!(
                            "the notice claimed of {}'s end to {} was not recorded: {why}",
                            held.place.id, claim.recipient
                        )),
                    }
                }
            }
        }
        recorded
    }

    /// The document of a place whose worktree and branch are gone, kept
    /// apart from a live place's under its id and the tidy's time while an
    /// end of it stands pending, naming the recipients it waits on, and
    /// holding no line of what its stages made; none kept where nothing
    /// is pending; or why it could not be written, so the tidy keeps the
    /// place's own document and no pending end is left with none (43, 62,
    /// 129; weir I9).
    pub(crate) fn keep_for_ends(
        &self,
        held: &HeldState,
        over: &Over,
    ) -> Result<Option<PlaceId>, String> {
        let pending = held.pending_for(&self.owing());
        if pending.is_empty() {
            return Ok(None);
        }
        let at = match &held.phase {
            Phase::Gone { at: Some(at), .. } => *at,
            _ => self.host.now(),
        };
        let id = PlaceId::new(format!("{}@{at}", held.place.location));
        let mut kept = held.clone();
        kept.place.id = id.clone();
        kept.phase = Phase::Tidied {
            over: over.clone(),
            at,
            pending_for: pending.clone(),
        };
        // The made lines go with the place's folder, so the document kept
        // for its ends holds none of them (62, 129; core.23).
        for stage in &mut kept.stages {
            if let Some(start) = stage.start.as_mut() {
                start.live = false;
            }
            if let Some(end) = stage.end.as_mut() {
                end.made.clear();
            }
        }
        match self.store.write(&id, None, Some(&kept)) {
            Written::Written { .. } => Ok(Some(id)),
            Written::Moved => match self.store.read(&id) {
                Some(_) => Ok(Some(id)),
                None => Err(format!(
                    "the document {id} kept for its ends pending for {} could not be written",
                    and_list(&pending)
                )),
            },
        }
    }

    /// A tidied place's document: removed once no end of it stands
    /// pending, every record of its ends' deliveries let go of then, and
    /// otherwise kept naming the recipients it waits on; true when it
    /// changed something (43, 62; weir 107).
    pub(crate) fn settle_tidied(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let pending = read.state.pending_for(&self.owing());
        if pending.is_empty() {
            let removed = self.remove_held(id);
            if removed {
                self.let_go_of(&read.state);
            }
            return removed;
        }
        self.update(id, |held| match &mut held.phase {
            Phase::Tidied { pending_for, .. } if *pending_for != pending => {
                *pending_for = pending.clone();
                Step::Write(true)
            }
            _ => Step::Keep(false),
        })
        .unwrap_or(false)
    }

    /// The documents a place's name reads, as status and the inbox read
    /// them: its live document, else those kept for it once it was
    /// tidied, `<location>@<at>` naming one; none where no document
    /// stands under the name (43, 62; core.14).
    pub(crate) fn documents_of(&self, id: &PlaceId) -> Vec<HeldState> {
        if let Some(read) = self.store.read(id) {
            return vec![read.state];
        }
        self.tidied_of(id)
            .iter()
            .filter_map(|id| self.store.read(id))
            .map(|read| read.state)
            .collect()
    }

    /// The tidied documents of the place `id` names: the one under that
    /// id, and those kept for the place at that location (43, 62).
    pub(crate) fn tidied_of(&self, id: &PlaceId) -> Vec<PlaceId> {
        self.store
            .read_every()
            .into_iter()
            .filter(|read| matches!(read.state.phase, Phase::Tidied { .. }))
            .filter(|read| read.state.place.id == *id || read.state.place.location == id.0)
            .map(|read| read.state.place.id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "fix--one/0a1b2c3d/stage/code/1/delivered";

    #[test]
    fn a_notice_confirms_as_its_recipient() {
        let message = "flume: fix--one (x) code delivered [stage/code/1/delivered]; read: flume status fix--one";
        assert_eq!(
            notice_to(message, KEY, "agentplot/ops"),
            format!("{message}; confirm: flume confirm --recipient agentplot/ops {KEY}")
        );
    }

    #[test]
    fn an_earlier_messages_confirm_gives_way_to_the_recipients() {
        let read = "flume: fix--one (x) code delivered [stage/code/1/delivered]; read: flume status fix--one";
        assert_eq!(
            notice_to(
                &format!("{read}; confirm: flume confirm {KEY}"),
                KEY,
                "pane:w3:p3@/run/term.sock"
            ),
            format!("{read}; confirm: flume confirm --recipient pane:w3:p3@/run/term.sock {KEY}")
        );
    }

    #[test]
    fn a_recipient_a_shell_would_split_is_quoted() {
        assert_eq!(
            confirm_command("the ops' desk", &[KEY, "b"]),
            format!(r"flume confirm --recipient 'the ops'\'' desk' {KEY} b")
        );
        assert_eq!(
            confirm_command("", &[KEY]),
            format!("flume confirm --recipient '' {KEY}")
        );
    }
}
