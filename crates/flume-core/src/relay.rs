//! A caller that carries a person's screen and keys relays a place's
//! live agent's terminal through flume (107, 221, 222): what the
//! terminal shows of the agent goes out as it changes and the person's
//! input goes in as the person typed it, a person's at the keyboard and
//! never a send of flume's (95). The relay reaches the agent only through
//! the terminal its start recorded (190; core.49), runs inside its
//! command and ends with it (120; core.13), and ends at its caller's
//! close or once the agent's terminal goes. It takes no lock, writes no
//! held state, document or file, records no act and settles nothing,
//! keeping of what it carried only the last screen given, to compare the
//! next with (221; core.16, core.17).

use crate::line::{stage_at, state_words};
use crate::start::Starts;
use crate::{Answer, Engine};
use flume_seams::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// One line a relay gives its caller while it runs: what the terminal
/// shows of the agent, a read the terminal did not answer in its words,
/// or an input not typed, in the words of why; written `{"screen": …}`,
/// `{"unread": …}` and `{"untyped": …}` (221).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelayLine {
    Screen(String),
    Unread(String),
    Untyped(String),
}

/// What a relay's caller gives it next: the person's input as it comes,
/// a tick once the relay's bound has passed since the last, or the
/// caller's close (221).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelayNext {
    Typed(Typed),
    Tick,
    Closed,
}

/// The caller's side of a relay: a command's standard input and output,
/// or a test's script (221; core.13).
pub trait RelayLink {
    /// `line` given to the caller; false once the caller takes no more,
    /// which ends the relay as the caller's.
    fn give(&mut self, line: &RelayLine) -> bool;

    /// The next of the person's input, a tick once `every` has passed
    /// since the last tick, or the caller's close.
    fn next(&mut self, every: Duration) -> RelayNext;
}

/// What ended a relay: its caller's close, or the agent's terminal gone
/// (221).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelayEnded {
    Caller,
    Gone,
}

/// What a relay holds while it runs: the last screen given, to compare
/// with, whether an unread read was given since a read last answered,
/// and the second of the host's clock it last read the agent's state in.
struct Relaying {
    last: Option<String>,
    unread_given: bool,
    state_read_in: Option<Seconds>,
}

/// What a read or a tick leaves the relay doing.
enum Goes {
    On,
    Ends(RelayEnded),
}

impl Engine {
    /// The relay of `place`'s live agent through `link`, until the link
    /// closes or takes no more, or the agent's terminal goes; refused,
    /// naming the place, that it has no live agent and its state in its
    /// line's words, where no live start of its current stage holds an
    /// agent, asking no terminal, and in the terminal's own words where
    /// its terminal is not carried or offers no relay (4, 107, 190, 221,
    /// 222). Its answer names the place, the agent and what ended it.
    pub fn relay(&self, place: &PlaceId, link: &mut dyn RelayLink) -> Answer {
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let held = read.state;
        let current = match held.current {
            Some(index) => held
                .stages
                .get(index)
                .and_then(|stage| stage.start.as_ref())
                .filter(|start| start.live),
            None => held.live_start().map(|(_, start)| start),
        };
        let Some(start) = current else {
            let state = state_words(&held, stage_at(&held), false, self.on_branch_count(&held));
            return Answer::refused(format!(
                "the place {place} has no live agent to relay: its line reads {state}"
            ));
        };
        let terminal = match self.terminal_of(start) {
            Ok(terminal) => terminal,
            Err(refused) => return Answer::refused(refused),
        };
        let identity = &start.identity;
        let mut relay = match terminal.relay(identity) {
            Ok(relay) => relay,
            Err(refused) => return Answer::refused(refused),
        };
        let every = relay.every();
        let mut relaying = Relaying {
            last: None,
            unread_given: false,
            state_read_in: None,
        };
        let ended = 'relay: {
            if let Goes::Ends(ended) =
                relaying.read(relay.as_mut(), terminal.as_ref(), identity, link)
            {
                break 'relay ended;
            }
            loop {
                match link.next(every) {
                    RelayNext::Typed(input) => {
                        if let Err(refused) = relay.typed(&input) {
                            if !link.give(&RelayLine::Untyped(refused)) {
                                break 'relay RelayEnded::Caller;
                            }
                        }
                    }
                    RelayNext::Tick => {
                        if let Goes::Ends(ended) =
                            relaying.read(relay.as_mut(), terminal.as_ref(), identity, link)
                        {
                            break 'relay ended;
                        }
                        let now = self.host.now();
                        if relaying.state_read_in != Some(now) {
                            relaying.state_read_in = Some(now);
                            if terminal.state(identity) == AgentState::Stopped {
                                break 'relay RelayEnded::Gone;
                            }
                        }
                    }
                    RelayNext::Closed => break 'relay RelayEnded::Caller,
                }
            }
        };
        Answer::Relayed {
            place: place.clone(),
            agent: identity.to_string(),
            ended,
        }
    }
}

impl Relaying {
    /// One read of the relay: a screen given where it differs from the
    /// last given; gone ending the relay; an unread given once until a
    /// read answers, and the agent's state read at once after it, a
    /// stopped agent ending the relay gone (96, 98, 136, 221).
    fn read(
        &mut self,
        relay: &mut dyn Relay,
        terminal: &dyn Terminal,
        identity: &Identity,
        link: &mut dyn RelayLink,
    ) -> Goes {
        match relay.screen() {
            RelayScreen::Shows { screen } => {
                self.unread_given = false;
                if self.last.as_ref() == Some(&screen) {
                    return Goes::On;
                }
                let given = link.give(&RelayLine::Screen(screen.clone()));
                self.last = Some(screen);
                if given {
                    Goes::On
                } else {
                    Goes::Ends(RelayEnded::Caller)
                }
            }
            RelayScreen::Gone => Goes::Ends(RelayEnded::Gone),
            RelayScreen::Unread { reason } => {
                if !self.unread_given {
                    self.unread_given = true;
                    if !link.give(&RelayLine::Unread(reason)) {
                        return Goes::Ends(RelayEnded::Caller);
                    }
                }
                if terminal.state(identity) == AgentState::Stopped {
                    Goes::Ends(RelayEnded::Gone)
                } else {
                    Goes::On
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A relay's lines and its answer, each one JSON object as the
    /// command writes it (221).
    #[test]
    fn a_relays_lines_and_its_answer_are_one_json_object_each() {
        for (line, written) in [
            (
                RelayLine::Screen("\u{1b}[1mready\u{1b}[0m".into()),
                r#"{"screen":"\u001b[1mready\u001b[0m"}"#,
            ),
            (RelayLine::Unread("slow".into()), r#"{"unread":"slow"}"#),
            (
                RelayLine::Untyped("hyper+x is no key".into()),
                r#"{"untyped":"hyper+x is no key"}"#,
            ),
        ] {
            assert_eq!(serde_json::to_string(&line).unwrap(), written);
            assert_eq!(serde_json::from_str::<RelayLine>(written).unwrap(), line);
        }
        for (ended, word) in [(RelayEnded::Caller, "caller"), (RelayEnded::Gone, "gone")] {
            let answer = Answer::Relayed {
                place: PlaceId::new("/host/places/fix--one"),
                agent: "flume:0123456789ab:fix--one:code:1".into(),
                ended,
            };
            let written = answer.to_json();
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&written).unwrap(),
                serde_json::json!({"relayed": {
                    "place": "/host/places/fix--one",
                    "agent": "flume:0123456789ab:fix--one:code:1",
                    "ended": word,
                }}),
                "{written}"
            );
            assert!(!written.contains('\n'), "one line: {written}");
            assert_eq!(serde_json::from_str::<Answer>(&written).unwrap(), answer);
            assert_eq!(answer.exit(), 0);
        }
    }
}
