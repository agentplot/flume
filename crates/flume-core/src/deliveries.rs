//! flume's store of weir's deliveries (C.7; weir 112): every record of
//! C.8 for the ends of the host's places, in the order added, in the
//! host's one deliveries document, written whole by compare-and-set at the
//! version read (75; core.16, core.17). weir's crate spells and reads each
//! record and answers every call over them; flume spells no field of one
//! (core.4).
//!
//! A record's time is the host clock's seconds written RFC 3339 in UTC,
//! `2026-11-08T15:02:00Z`, and read back to seconds by its instant, so
//! status gives every time in seconds as it gives every other (56).

use crate::ends::notice;
use crate::{Answer, Engine, KeyConfirmed, Step};
use chrono::{DateTime, SecondsFormat, Utc};
use flume_seams::{
    ConfirmedBy, DeliveredBy, Deliveries, Delivery, DeliveryRecord, EndDelivery, EndOf, HeldState,
    HostDocument, HostKey, HostVersioned, OwedTo, Owing, PlaceId, Seconds, Secret, Store, Written,
};
use std::collections::BTreeMap;
use weir_delivery::{Confirmed, Handed, Record, Recorded};

/// How many times a write of the host's deliveries reads again on a
/// moved version before it says so (75).
const TRIES: usize = 32;

/// weir's store seam over flume's store: the host's deliveries document,
/// read whole and added to at the version read (weir 112, 113).
pub struct HostDeliveries<'a> {
    store: &'a dyn Store,
}

impl<'a> HostDeliveries<'a> {
    /// The host's deliveries in `store`.
    pub fn new(store: &'a dyn Store) -> Self {
        HostDeliveries { store }
    }
}

impl weir_delivery::Store for HostDeliveries<'_> {
    type Error = String;

    fn read(&self) -> Result<Vec<Record>, String> {
        read(self.store).map(|(_, records)| records)
    }

    fn add(&mut self, records: &[Record]) -> Result<(), String> {
        let (at, mut held) = read(self.store)?;
        held.extend(records.iter().cloned());
        match write(self.store, at, &held) {
            Written::Written { .. } => Ok(()),
            Written::Moved => Err(format!(
                "{} moved while they were written, and nothing was added",
                HostKey::Deliveries
            )),
        }
    }
}

/// The host's deliveries and the version they were read at; none and no
/// version where the host holds no deliveries document (C.7).
pub fn read(store: &dyn Store) -> Result<(Option<u64>, Vec<Record>), String> {
    match store.read_host(&HostKey::Deliveries) {
        None => Ok((None, Vec::new())),
        Some(HostVersioned {
            version,
            document: HostDocument::Deliveries(deliveries),
        }) => Ok((
            Some(version),
            deliveries
                .records
                .into_iter()
                .map(|record| record.0)
                .collect(),
        )),
        Some(_) => Err(format!("{} hold another document", HostKey::Deliveries)),
    }
}

/// The host's deliveries as `records`, whole, only if still at `at` (75,
/// 103).
fn write(store: &dyn Store, at: Option<u64>, records: &[Record]) -> Written {
    let document = HostDocument::Deliveries(Deliveries {
        records: records.iter().cloned().map(DeliveryRecord).collect(),
    });
    store.write_host(&HostKey::Deliveries, at, Some(&document))
}

/// Reads the host's deliveries and their version, asks `next` for the
/// records that stand after this write, the whole of them, and writes
/// them at that version, reading again where the version moved; `next`
/// giving none writes nothing. Gives the records as they stand after it,
/// or why they could not be written (75; core.17).
pub fn rewrite(
    store: &dyn Store,
    mut next: impl FnMut(&[Record]) -> Option<Vec<Record>>,
) -> Result<Vec<Record>, String> {
    for _ in 0..TRIES {
        let (at, records) = read(store)?;
        let Some(next) = next(&records) else {
            return Ok(records);
        };
        if let Written::Written { .. } = write(store, at, &next) {
            return Ok(next);
        }
    }
    Err(format!(
        "{} were not written: their version moved {TRIES} times",
        HostKey::Deliveries
    ))
}

/// The engine's one way to add a record: reads the host's deliveries and
/// their version, asks `add` over them for the records one of weir's calls
/// gives, and writes them after the others at that version, reading again
/// where it moved, so no call of weir's answers over records another
/// process has added to since; `add` giving none writes nothing. Gives the
/// records as they stand after it, or why they could not be written (75;
/// core.17).
pub fn update(
    store: &dyn Store,
    mut add: impl FnMut(&[Record]) -> Vec<Record>,
) -> Result<Vec<Record>, String> {
    rewrite(store, |records| {
        let added = add(records);
        (!added.is_empty()).then(|| [records, &added].concat())
    })
}

/// The host's `seconds` as a record's time: RFC 3339 in UTC, to the second
/// (C.8).
pub fn time(seconds: Seconds) -> String {
    i64::try_from(seconds)
        .ok()
        .and_then(|seconds| DateTime::<Utc>::from_timestamp(seconds, 0))
        .unwrap_or_default()
        .to_rfc3339_opts(SecondsFormat::Secs, true)
}

/// A record's time read back to the host's seconds by its instant,
/// whatever its offset; none where it is no RFC 3339 time at or after the
/// host clock's start (C.8).
pub fn seconds(time: &str) -> Option<Seconds> {
    DateTime::parse_from_rfc3339(time)
        .ok()
        .and_then(|at| Seconds::try_from(at.timestamp()).ok())
}

/// What `records` hold of each end's delivery at `now`: each delivery's
/// readers, message and expiry, each hand-over and each confirmation in
/// the order added, and, through weir's `owed`, each reader that has not
/// confirmed it, expired or not, its name known or not (43; weir 104 to
/// 106, 111). A hand-over by a way flume records none by is passed over.
pub fn owing(records: &[Record], now: Seconds) -> Owing {
    let at = |time: &weir_delivery::Time| seconds(time.as_str()).unwrap_or_default();
    let mut deliveries: BTreeMap<String, EndDelivery> = BTreeMap::new();
    for record in records {
        match record {
            Record::Delivery(delivery) => {
                let key = delivery.key().to_string();
                deliveries
                    .entry(key.clone())
                    .or_insert_with(|| EndDelivery {
                        key,
                        readers: delivery.readers().iter().map(ToString::to_string).collect(),
                        message: delivery.message().to_string(),
                        expires_at: at(delivery.expires_at()),
                        owed: vec![],
                        confirmed: vec![],
                        handovers: vec![],
                    });
            }
            Record::Handover(handover) => {
                let (Some(delivery), Some(by)) = (
                    deliveries.get_mut(handover.key.as_str()),
                    DeliveredBy::of_way(handover.by.as_str()),
                ) else {
                    continue;
                };
                delivery.handovers.push(Delivery {
                    recipient: handover.reader.to_string(),
                    at: at(&handover.at),
                    by,
                });
            }
            Record::Confirmation(confirmation) => {
                let Some(delivery) = deliveries.get_mut(confirmation.key.as_str()) else {
                    continue;
                };
                let recipient = confirmation.reader.to_string();
                if !delivery
                    .confirmed
                    .iter()
                    .any(|confirmed| confirmed.recipient == recipient)
                {
                    delivery.confirmed.push(ConfirmedBy {
                        recipient,
                        at: at(&confirmation.at),
                    });
                }
            }
            Record::Known(_) => {}
        }
    }
    if let Ok(owed) = weir_delivery::owed(records, &time(now)) {
        for reader in owed.readers {
            for owed in reader.deliveries {
                if let Some(delivery) = deliveries.get_mut(owed.delivery.key().as_str()) {
                    delivery.owed.push(OwedTo {
                        recipient: reader.reader.to_string(),
                        expired: owed.expired,
                        known: reader.known,
                    });
                }
            }
        }
    }
    for delivery in deliveries.values_mut() {
        let readers = delivery.readers.clone();
        delivery.owed.sort_by_key(|owed| {
            readers
                .iter()
                .position(|reader| *reader == owed.recipient)
                .unwrap_or(usize::MAX)
        });
    }
    Owing { deliveries }
}

/// A place's token, eight hex characters of `secret`, by which its ends'
/// deliveries are keyed, so two launches of one place never share a key
/// (43, C.7; weir 101).
pub(crate) fn token(secret: &Secret) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(secret.0.as_bytes())
        .iter()
        .take(4)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

impl Engine {
    /// The place's document holding a token, given one by this write
    /// where it held none, as a document written before; none where the
    /// place has no document (43, C.7; weir 101).
    pub(crate) fn with_token(&self, id: &PlaceId) -> Option<HeldState> {
        let read = self.store.read(id)?;
        if read.state.place.token.is_some() {
            return Some(read.state);
        }
        let token = token(&self.host.secret());
        self.update(id, |held| {
            if held.place.token.is_some() {
                return Step::Keep(());
            }
            held.place.token = Some(token.clone());
            Step::Write(())
        });
        self.store.read(id).map(|read| read.state)
    }

    /// Records, through weir's `record`, the delivery of each end of
    /// `documents` that has a recipient and no record: under its key, its
    /// readers the end's recipients, its message the end's notice as the
    /// document stands, at the time the end was read, expiring the host's
    /// span after it. An end with no recipient records none, and an end
    /// whose key is held records nothing again, so recording again is the
    /// same act (43; weir 100, 101, 104; core.17; design 3, 4).
    pub(crate) fn record_deliveries(&self, documents: &[HeldState]) {
        let wanted: Vec<(String, String, Vec<String>, String)> = documents
            .iter()
            .flat_map(|held| {
                held.ends.iter().filter_map(move |end| {
                    // A stall an earlier flume recorded is recorded once a
                    // settle has read whom it reaches (130, 193).
                    if matches!(
                        end.of,
                        EndOf::Stall {
                            answerer_read: false,
                            ..
                        }
                    ) {
                        return None;
                    }
                    let readers = end.recipients(&held.place.recipients);
                    if readers.is_empty() {
                        return None;
                    }
                    let key = held.place.delivery_key(&end.of)?;
                    Some((key, notice(held, &end.of), readers, time(end.read_at)))
                })
            })
            .collect();
        if wanted.is_empty() {
            return;
        }
        let span = self.host.ends_expire();
        let mut refused: Vec<String> = vec![];
        let written = update(&*self.store, |records| {
            refused.clear();
            let mut added = vec![];
            for (key, message, readers, at) in &wanted {
                let readers: Vec<&str> = readers.iter().map(String::as_str).collect();
                match weir_delivery::record(records, key, message, &readers, at, span) {
                    Ok(Recorded::Add(records)) => added.extend(records),
                    Ok(Recorded::Held(_)) => {}
                    Err(refusal) => refused.push(format!("{key}: {refusal}")),
                }
            }
            added
        });
        for why in refused {
            self.host
                .log(&format!("an end's delivery was not recorded: {why}"));
        }
        if let Err(why) = written {
            self.host.log(&why);
        }
    }

    /// The documents, each holding a token, and every end of them with a
    /// recipient recorded as a delivery, as a settle, a wait or an inbox
    /// reads them before it reads what is owed (43; design 3).
    pub(crate) fn recorded(&self, documents: Vec<HeldState>) -> Vec<HeldState> {
        let documents: Vec<HeldState> = documents
            .into_iter()
            .filter_map(|held| match held.place.token {
                Some(_) => Some(held),
                None => self.with_token(&held.place.id),
            })
            .collect();
        self.record_deliveries(&documents);
        documents
    }

    /// What the host's deliveries hold of each end's delivery now, read
    /// once (43; weir 111; core.15).
    pub(crate) fn owing(&self) -> Owing {
        match read(&*self.store) {
            Ok((_, records)) => owing(&records, self.host.now()),
            Err(why) => {
                self.host.log(&why);
                Owing::default()
            }
        }
    }

    /// Confirms each of `keys` as `recipient`, through weir's `confirm`, in
    /// one write: each key with when it was confirmed and whether it had
    /// been before, its first time then; or, at the first key weir refuses,
    /// that refusal naming the key and its field, the keys before it
    /// confirmed and none after it (43; weir 102, 106, 110; core.25).
    pub fn confirm(&self, recipient: &str, keys: &[String]) -> Answer {
        let now = self.host.now();
        let at = time(now);
        let mut confirmed: Vec<KeyConfirmed> = vec![];
        let mut refused: Option<String> = None;
        let written = update(&*self.store, |records| {
            confirmed.clear();
            refused = None;
            let mut all = records.to_vec();
            let mut added = vec![];
            for key in keys {
                match weir_delivery::confirm(&all, key, recipient, &at) {
                    Ok(Confirmed::Add(records)) => {
                        all.extend(records.iter().cloned());
                        added.extend(records);
                        confirmed.push(KeyConfirmed {
                            key: key.clone(),
                            at: now,
                            before: false,
                        });
                    }
                    Ok(Confirmed::Already(first)) => confirmed.push(KeyConfirmed {
                        key: key.clone(),
                        at: seconds(first.as_str()).unwrap_or_default(),
                        before: true,
                    }),
                    Err(refusal) => {
                        refused = Some(format!(
                            "the delivery key {key} was not confirmed as {recipient}: {refusal}"
                        ));
                        break;
                    }
                }
            }
            added
        });
        if let Err(why) = written {
            return Answer::refused(why);
        }
        match refused {
            Some(why) if confirmed.is_empty() => Answer::refused(why),
            Some(why) => Answer::refused(format!(
                "{why}; the keys before it were confirmed: {}",
                confirmed
                    .iter()
                    .map(|key| key.key.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            None => Answer::Confirmed {
                recipient: recipient.into(),
                confirmed,
            },
        }
    }

    /// Hands the delivery under each of `keys` to `recipient` `by` a way at
    /// `at`, through weir's `hand`, in one write, confirming each on
    /// receipt where `confirms`: for each key, whether this call handed it
    /// over, false where the recipient had confirmed it, or the refusal
    /// weir gave. A hand-over confirms nothing by itself (43; weir 103,
    /// 109; core.25).
    pub(crate) fn hand_over(
        &self,
        keys: &[String],
        recipient: &str,
        by: DeliveredBy,
        at: Seconds,
        confirms: bool,
    ) -> Vec<Result<bool, String>> {
        let at = time(at);
        let mut answers: Vec<Result<bool, String>> = vec![];
        let written = update(&*self.store, |records| {
            answers.clear();
            let mut all = records.to_vec();
            let mut added = vec![];
            for key in keys {
                match weir_delivery::hand(&all, key, recipient, by.way(), &at, confirms) {
                    Ok(Handed::Add(records)) => {
                        all.extend(records.iter().cloned());
                        added.extend(records);
                        answers.push(Ok(true));
                    }
                    Ok(Handed::Confirmed(_)) => answers.push(Ok(false)),
                    Err(refusal) => answers.push(Err(format!("{key}: {refusal}"))),
                }
            }
            added
        });
        if let Err(why) = written {
            return keys.iter().map(|_| Err(why.clone())).collect();
        }
        answers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hosts_second_is_written_in_utc_and_read_back() {
        assert_eq!(time(1_794_150_120), "2026-11-08T15:02:00Z");
        assert_eq!(seconds("2026-11-08T15:02:00Z"), Some(1_794_150_120));
        assert_eq!(seconds(&time(1_000_000)), Some(1_000_000));
    }

    #[test]
    fn an_offset_time_reads_back_to_its_instant() {
        assert_eq!(seconds("2026-11-08T10:02:00-05:00"), Some(1_794_150_120));
        assert_eq!(seconds("2026-11-09T00:32:00+09:30"), Some(1_794_150_120));
        assert_eq!(seconds("2026-11-08 15:02"), None);
    }
}
