//! A message to a role's address (53, 132, 193, 214, C.7; design 3, 4).
//! A message's target is a role's address where it is `<stream>/<role>`,
//! each part a word and neither `.` nor `..`, and otherwise a place, as
//! every command names one. A message to an address that one source of
//! the host declares, with a run, a standing agent or carried, is recorded
//! at once through weir's `record` in the host's deliveries, keyed
//! `message/<stream>/<role>/<token>`, its one reader the address, its
//! message a first line naming who sent it and then the words, expiring
//! after `[host] ends_expire`; the act then wakes the address's run where
//! its source gives it one and none is live or waiting for room, as a
//! settle's wake does, and answers at once, with no bound: the message is
//! pending until the address's session takes it and confirms it (43, 132,
//! 214; weir 100, 101, 102, 104). An address no source declares, two
//! declare, whose stream the host does not declare or whose entry is at
//! fault is refused naming why, and so is a bound or an answer's key with
//! an address, each recording nothing (42, 53, 193).

use crate::deliveries::{time, token, update};
use crate::{Answer, Answering, Engine, Found, Woke};
use flume_seams::*;
use weir_delivery::{Record, Recorded};

/// Whether `target` names a role's address, `<stream>/<role>` with each
/// part a word as an address's is and neither `.` nor `..`, and not a
/// place; `./east/planner` names a place's location (53, 132; design 3).
pub fn names_an_address(target: &str) -> bool {
    is_address(target) && target.split('/').all(|part| part != "." && part != "..")
}

/// A message's delivery key: `message`, the address and the message's
/// token joined by `/`, which never begins as an end's does, since a
/// place's name always holds `--` (C.7; weir 101).
pub fn message_key(address: &str, token: &str) -> String {
    format!("message/{address}/{token}")
}

/// Whether `key` is a message's delivery key (C.7).
pub fn is_message_key(key: &str) -> bool {
    key.starts_with("message/")
}

/// A message's words as its delivery holds them: a first line naming who
/// sent it, `<from> messages <address>:`, or `A message to <address>:`
/// where the sending command reads no one, then the words as given on the
/// lines after it (214, C.7).
pub fn message_words(address: &str, from: Option<&str>, words: &str) -> String {
    let first = match from {
        Some(from) => format!("{from} messages {address}:"),
        None => format!("A message to {address}:"),
    };
    format!("{first}\n{words}")
}

impl Engine {
    /// Messages the role's address `address` `words`, from the recipient
    /// `answering` names as the sending command's: refused, recording
    /// nothing, with a bound or a key, or for an address no source of the
    /// host can take; otherwise recorded as a delivery at the address,
    /// its run woken where its source gives it one and none stands, and
    /// answered at once with the delivery's key, the address and what the
    /// act found of it (53, 132, 193, 214, C.7; design 3, 4).
    pub fn message_address(
        &self,
        address: &str,
        words: &str,
        bound: Option<Seconds>,
        answering: &Answering,
    ) -> Answer {
        if bound.is_some() {
            return Answer::refused(format!(
                "{address} is a role's address, and a bound is how long a message waits for a \
                 place's agent: a message to an address waits at it, with no bound, until its \
                 session takes it; nothing was recorded"
            ));
        }
        if let Some(key) = &answering.key {
            return Answer::refused(format!(
                "{address} is a role's address, and an answer naming {key} goes to the place that \
                 asked it: message that place with --answers; nothing was recorded"
            ));
        }
        self.settle_attended();
        let roles = self.host.roles();
        let entry = match roles.declared(address) {
            Ok(entry) => entry,
            Err(why) => {
                return Answer::refused(format!(
                    "{address} takes no message: {why}; nothing was recorded"
                ))
            }
        };
        let key = match self.record_message(address, answering.by.as_deref(), words) {
            Ok(key) => key,
            Err(why) => return Answer::refused(why),
        };
        let (found, run, why) = match entry {
            RoleEntry::Run(_) => match self.wake_address(address, &roles) {
                Ok((woke, held)) => (
                    match woke {
                        Woke::Launched => Found::Woken,
                        Woke::Live => Found::Live,
                        Woke::Waiting => Found::Waiting,
                    },
                    Some(held.place.id),
                    None,
                ),
                Err(why) => (Found::Unwoken, None, Some(why)),
            },
            RoleEntry::Standing(_) => (Found::Standing, None, None),
            RoleEntry::Carried(_) => (Found::Carried, None, None),
        };
        Answer::Messaged {
            key,
            address: address.into(),
            found,
            run,
            why,
        }
    }

    /// Records, through weir's `record`, the message `words` to `address`
    /// from `from`: under a key of its own token, its one reader the
    /// address, expiring the host's span after now; its key, or why it
    /// was not recorded. flume's log names the delivery recorded, its
    /// reader and its expiry (43, 214, C.7; weir 100, 101, 104).
    fn record_message(
        &self,
        address: &str,
        from: Option<&str>,
        words: &str,
    ) -> Result<String, String> {
        let message = message_words(address, from, words);
        let at = time(self.host.now());
        let span = self.host.ends_expire();
        let mut key = String::new();
        let mut refused: Option<String> = None;
        let mut expires = String::new();
        let written = update(&*self.store, |records| {
            refused = None;
            // A token another message holds is drawn again, so two
            // messages never share a key (weir 101).
            loop {
                key = message_key(address, &token(&self.host.secret()));
                let held = records.iter().any(|record| {
                    matches!(record, Record::Delivery(delivery) if delivery.key().as_str() == key)
                });
                if !held {
                    break;
                }
            }
            match weir_delivery::record(records, &key, &message, &[address], &at, span) {
                Ok(Recorded::Add(added)) => {
                    expires = added
                        .iter()
                        .find_map(|record| match record {
                            Record::Delivery(delivery) => {
                                Some(delivery.expires_at().as_str().to_string())
                            }
                            _ => None,
                        })
                        .unwrap_or_default();
                    added
                }
                Ok(Recorded::Held(_)) => vec![],
                Err(refusal) => {
                    refused = Some(format!(
                        "the message to {address} was not recorded: {refusal}"
                    ));
                    vec![]
                }
            }
        });
        if let Err(why) = written {
            return Err(format!("the message to {address} was not recorded: {why}"));
        }
        if let Some(why) = refused {
            return Err(why);
        }
        self.host.log(&format!(
            "recorded the delivery {key}, a message owed to {address}, expiring at {expires}"
        ));
        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_address_and_a_place() {
        assert!(names_an_address("east/planner"));
        assert!(!names_an_address("./east/planner"));
        assert!(!names_an_address("../planner"));
        assert!(!names_an_address("east/.."));
        assert!(!names_an_address("fix--one"));
        assert!(!names_an_address("/r/places/fix--one"));
    }

    #[test]
    fn a_message_names_who_sent_it() {
        assert_eq!(
            message_words("east/planner", Some("ops"), "the nightly job failed"),
            "ops messages east/planner:\nthe nightly job failed"
        );
        assert_eq!(
            message_words("east/planner", None, "look"),
            "A message to east/planner:\nlook"
        );
        assert_eq!(
            message_key("east/planner", "0a1b2c3d"),
            "message/east/planner/0a1b2c3d"
        );
        assert!(is_message_key("message/east/planner/0a1b2c3d"));
        assert!(!is_message_key("fix--one/0a1b2c3d/stage/code/1/delivered"));
    }
}
