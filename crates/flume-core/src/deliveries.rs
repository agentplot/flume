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

use chrono::{DateTime, SecondsFormat, Utc};
use flume_seams::{
    Deliveries, DeliveryRecord, HostDocument, HostKey, HostVersioned, Seconds, Store, Written,
};
use weir_delivery::Record;

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
