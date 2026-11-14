//! A live start's report of its account's usage windows, taken as that
//! account's reading (153, 184, 211; core.32): asked with the start's
//! secret, by its agent program's own part of flume once the program
//! reported its windows, the windows read through the agent seam and
//! written into the account's own document by compare-and-set, its spent
//! time carried. flume asks no program or its maker, reads no sign-in
//! and fetches nothing (146), and no settle takes a reading: the report
//! is the one way in, and a report not recorded is followed by the next.

use crate::spent::{account_document, TRIES};
use crate::Engine;
use flume_seams::*;

impl Engine {
    /// Takes the windows the program of the live current start
    /// `identity` names last reported, asked with that start's `secret`,
    /// as the reading of the account it runs under, with when they were
    /// taken, the start's identity as its agent and the session they came
    /// from, by compare-and-set on the account's document, tried again on
    /// a moved version, its spent time carried; refused naming the start
    /// where `identity` names no live current start or `secret` is not its
    /// start's. Nothing is taken for a start under no account or whose
    /// program's profile gives no report, and nothing is written where
    /// the reading kept was taken at the same second or later, so taking
    /// one report twice changes nothing (153, 184, 211; core.17).
    pub fn take_reading(&self, identity: &str, secret: &str) -> Result<(), String> {
        let Some((held, index)) = self.current_start(identity) else {
            return Err(format!(
                "no live agent of a place's current stage is {identity}"
            ));
        };
        let start = held.stages[index].start.as_ref().expect("a live start");
        if start.secret.0 != secret {
            return Err(format!("the secret is not {identity}'s start's"));
        }
        let Some(account) = &start.account else {
            return Ok(());
        };
        let Some(agent) = self.agent(&start.program) else {
            return Ok(());
        };
        let folder = self.store.folder(&held.place.id);
        let Windows::Reported {
            taken_at,
            windows,
            session,
        } = agent.windows(&folder, &start.identity)
        else {
            return Ok(());
        };
        let reading = AccountReading {
            taken_at,
            agent: start.identity.to_string(),
            session,
            windows,
        };
        let key = HostKey::Account {
            pool: account.pool.clone(),
            account: account.name.clone(),
        };
        for _ in 0..TRIES {
            let read = self.store.read_host(&key);
            let mut document = account_document(read.as_ref());
            if document
                .reading
                .as_ref()
                .is_some_and(|kept| kept.taken_at >= reading.taken_at)
            {
                return Ok(());
            }
            document.reading = Some(reading.clone());
            let at = read.map(|read| read.version);
            let next = HostDocument::Account(document);
            if let Written::Written { .. } = self.store.write_host(&key, at, Some(&next)) {
                return Ok(());
            }
        }
        Ok(())
    }
}
