//! What a recipient takes at its next wait (43, 75, 214, 215, C.7;
//! core.11, core.25; design 5, 6). A recipient's batch is every delivery
//! whose readers name it, owed to it, handed to it by nothing and claimed
//! for it by nothing, but a drop's or a done's end, which no notice
//! carries, in the order the host's deliveries recorded them: each item
//! the delivery's message followed by the way that recipient confirms it,
//! the items joined by one blank line. It is handed only while the
//! recipient's session reads waiting, never working, asking or unknown:
//! claimed whole beside the host's deliveries in one write at the version
//! read, the document read again and each item another way handed to the
//! recipient meanwhile dropped, sent by the terminal's `send_to`, and, on
//! the send's answer, each item's hand-over added, by `notice` for an end
//! and by `message` for a message, at the claim's time, and the claims
//! removed, in one write; where that write fails, the claims are marked
//! answered, and the next settle records each at its time, once. A send
//! refused or not found removes the claims
//! and records nothing, so the next settle hands the batch again. A claim
//! a settle ended mid-send leaves sending is recorded by no other settle,
//! and holds its items out of every later batch, the inbox, a wait or the
//! session's start still handing them; a claim whose delivery another way
//! handed, or that was let go of, is removed by the next write. A place's
//! `noticing` claims an earlier flume wrote are read as before, an answered
//! one recorded once, and none is written. A settle of one place hands the
//! batch of each recipient of its ends pending and, for a role's run, of
//! its own address; a settle of every place hands the batch of every
//! recipient a delivery is pending for, once each in the pass.

use crate::deliveries::{owing, read_whole, rewrite_whole, time};
use crate::ends::notice_to;
use crate::message::is_message_key;
use crate::standing::{Reached, Standing};
use crate::start::Starts;
use crate::{Answer, Engine, Step};
use flume_seams::*;
use std::collections::BTreeSet;
use weir_delivery::Handed;

/// One delivery of a recipient's batch: its key, its message as recorded,
/// and the way its hand-over is recorded by, `notice` for an end's and
/// `message` for a message's (43, 214).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Item {
    pub key: String,
    pub message: String,
    pub by: DeliveredBy,
}

/// Each delivery a place's document holds back from a recipient's batch,
/// by its key and the recipient: a notice an earlier flume claimed on the
/// place, and a hand-over the old form recorded there (43, 75).
pub(crate) type HeldBack = BTreeSet<(String, String)>;

/// Whether `key` is the delivery of a drop's or a done's end, which no
/// notice carries (43, 52, 114).
fn of_an_over(key: &str) -> bool {
    !is_message_key(key)
        && key
            .splitn(3, '/')
            .nth(2)
            .is_some_and(|end| end.starts_with("over/"))
}

/// The words a recipient is handed: each item's message and the way that
/// recipient confirms it, joined by one blank line, with nothing before or
/// after them (43, 214; design 5).
pub(crate) fn batch_words(items: &[Item], recipient: &str) -> String {
    items
        .iter()
        .map(|item| notice_to(&item.message, &item.key, recipient))
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Whether the delivery `delivery` is owed to `recipient` and handed to it
/// by nothing (43; weir 103).
fn unhanded(delivery: &EndDelivery, recipient: &str) -> bool {
    delivery.owed.iter().any(|owed| owed.recipient == recipient)
        && !delivery
            .handovers
            .iter()
            .any(|handover| handover.recipient == recipient)
}

/// `recipient`'s batch as `owing` and `claims` read it, `held_back` kept
/// out, in the order the deliveries were recorded (43, 214).
pub(crate) fn batch(
    owing: &Owing,
    claims: &[DeliveryClaim],
    held_back: &HeldBack,
    recipient: &str,
) -> Vec<Item> {
    owing
        .in_order()
        .filter(|delivery| !of_an_over(&delivery.key))
        .filter(|delivery| unhanded(delivery, recipient))
        .filter(|delivery| {
            !claims
                .iter()
                .any(|claim| claim.key == delivery.key && claim.recipient == recipient)
        })
        .filter(|delivery| !held_back.contains(&(delivery.key.clone(), recipient.to_string())))
        .map(|delivery| Item {
            key: delivery.key.clone(),
            message: delivery.message.clone(),
            by: if is_message_key(&delivery.key) {
                DeliveredBy::Message
            } else {
                DeliveredBy::Notice
            },
        })
        .collect()
}

/// Each recipient a delivery of `owing` is pending for and handed to by
/// nothing, but a drop's or a done's, once each, in the order the
/// deliveries were recorded (43, 214).
fn pending_recipients(owing: &Owing) -> Vec<String> {
    let mut recipients: Vec<String> = vec![];
    for delivery in owing
        .in_order()
        .filter(|delivery| !of_an_over(&delivery.key))
    {
        for owed in &delivery.owed {
            if unhanded(delivery, &owed.recipient) && !recipients.contains(&owed.recipient) {
                recipients.push(owed.recipient.clone());
            }
        }
    }
    recipients
}

/// Whether `claim` still stands: its delivery owed to its recipient and
/// handed to it by nothing (75, C.7).
fn stands(claim: &DeliveryClaim, owing: &Owing) -> bool {
    owing
        .deliveries
        .get(&claim.key)
        .is_some_and(|delivery| unhanded(delivery, &claim.recipient))
}

/// What `documents` hold back from a recipient's batch: each end's notice
/// an earlier flume claimed on its place, and each delivery its old form
/// recorded (43, 75).
fn held_back(documents: &[HeldState]) -> HeldBack {
    let mut held_back = HeldBack::new();
    for held in documents {
        for end in &held.ends {
            let Some(key) = held.place.delivery_key(&end.of) else {
                continue;
            };
            for claim in &end.noticing {
                held_back.insert((key.clone(), claim.recipient.clone()));
            }
            for delivery in &end.deliveries {
                held_back.insert((key.clone(), delivery.recipient.clone()));
            }
        }
    }
    held_back
}

/// A role's run's start's batch: the start, and the words its prompt
/// carries (214; design 9).
pub(crate) struct StartBatch {
    pub identity: Identity,
    pub words: String,
}

/// The stage's state and the start of `documents` whose identity is
/// `identity`, as a claim names it.
fn start_of<'a>(documents: &'a [HeldState], identity: &str) -> Option<(StageState, &'a StartHeld)> {
    documents.iter().find_map(|held| {
        held.stages.iter().find_map(|stage| {
            stage
                .start
                .as_ref()
                .filter(|start| start.identity.to_string() == identity)
                .map(|start| (stage.state, start))
        })
    })
}

impl Engine {
    /// Claims `address`'s batch for the start `identity` of its run, whose
    /// prompt carries it, every start's claims settled first; none where
    /// nothing is pending at the address (214; design 9).
    pub(crate) fn claim_for_start(&self, address: &str, identity: &Identity) -> Option<StartBatch> {
        let documents = self.documents();
        self.settle_start_claims(&documents);
        let items = self.claim_batch(
            address,
            &held_back(&documents),
            self.host.now(),
            Some(identity.to_string()),
        )?;
        Some(StartBatch {
            identity: identity.clone(),
            words: batch_words(&items, address),
        })
    }

    /// Lets go of every claim the start `identity` holds, recording nothing
    /// (214; design 9).
    pub(crate) fn release_start(&self, identity: &Identity) {
        let start = identity.to_string();
        let written = rewrite_whole(&*self.store, |records, claims| {
            let next: Vec<DeliveryClaim> = claims
                .iter()
                .filter(|claim| claim.start.as_deref() != Some(start.as_str()))
                .cloned()
                .collect();
            (next.len() != claims.len()).then(|| (records.to_vec(), next))
        });
        if let Err(why) = written {
            self.host.log(&why);
        }
    }

    /// Every place's document on the host, as one read gives them.
    fn documents(&self) -> Vec<HeldState> {
        self.store
            .read_every()
            .into_iter()
            .map(|read| read.state)
            .collect()
    }

    /// Settles each claim a start of a role's run holds for its prompt:
    /// recorded handed, at its receipt, once the start's prompt is read
    /// received, by `notice` for an end and by `message` for a message;
    /// kept while the start is live and its prompt not yet received; and
    /// otherwise let go of, recording nothing, as for a start read never
    /// begun or ended before its prompt was received, so the next start
    /// or the next wait takes them; each in one write (37, 38, 43, 214;
    /// record 0058; design 9).
    fn settle_start_claims(&self, documents: &[HeldState]) {
        let mut refused: Vec<String> = vec![];
        let written = rewrite_whole(&*self.store, |records, claims| {
            refused.clear();
            if claims.iter().all(|claim| claim.start.is_none()) {
                return None;
            }
            let mut all = records.to_vec();
            let mut next: Vec<DeliveryClaim> = vec![];
            for claim in claims {
                let Some(start) = claim.start.as_deref() else {
                    next.push(claim.clone());
                    continue;
                };
                match start_of(documents, start) {
                    Some((_, start)) if start.prompt.received_at.is_some() => {
                        let by = if is_message_key(&claim.key) {
                            DeliveredBy::Message
                        } else {
                            DeliveredBy::Notice
                        };
                        let at = time(start.prompt.received_at.unwrap_or(claim.at));
                        match weir_delivery::hand(
                            &all,
                            &claim.key,
                            &claim.recipient,
                            by.way(),
                            &at,
                            false,
                        ) {
                            Ok(Handed::Add(added)) => all.extend(added),
                            Ok(Handed::Confirmed(_)) => {}
                            Err(refusal) => refused.push(format!("{}: {refusal}", claim.key)),
                        }
                    }
                    Some((StageState::Starting | StageState::Started, start)) if start.live => {
                        next.push(claim.clone());
                    }
                    _ => {}
                }
            }
            (next.len() != claims.len()).then_some((all, next))
        });
        if let Err(why) = written {
            self.host.log(&why);
        }
        for why in refused {
            self.host.log(&format!(
                "a delivery a run's prompt carried was not recorded: {why}"
            ));
        }
    }

    /// Settles the claims of the starts of roles' runs, where any stands
    /// (214; design 9).
    fn settle_any_start_claims(&self) {
        let any = read_whole(&*self.store)
            .map(|whole| whole.claims.iter().any(|claim| claim.start.is_some()))
            .unwrap_or(false);
        if any {
            self.settle_start_claims(&self.documents());
        }
    }

    /// Hands each batch a settle reads: of one place, each recipient of
    /// its ends pending and, for a role's run, its own address; of every
    /// place, every recipient a delivery is pending for, once each (43,
    /// 121, 132, 214; design 5).
    pub(crate) fn hand_batches(&self, place: Option<&PlaceId>) {
        let recipients: Vec<String> = match place {
            Some(id) => {
                let Some(held) = self.with_token(id) else {
                    return;
                };
                self.record_deliveries(std::slice::from_ref(&held));
                let owing = self.owing();
                let mut recipients: Vec<String> = vec![];
                for end in held.ends.iter().filter(|end| end.noticed()) {
                    for recipient in end.pending(&held.place, &owing) {
                        if !recipients.contains(&recipient) {
                            recipients.push(recipient);
                        }
                    }
                }
                if let Some(address) = held.role.clone() {
                    if !recipients.contains(&address) {
                        recipients.push(address);
                    }
                }
                recipients
            }
            None => pending_recipients(&self.owing()),
        };
        self.record_answered();
        self.settle_any_start_claims();
        if recipients.is_empty() {
            return;
        }
        let documents: Vec<HeldState> = self
            .store
            .read_every()
            .into_iter()
            .map(|read| read.state)
            .collect();
        for held in documents
            .iter()
            .filter(|held| held.ends.iter().any(|end| !end.noticing.is_empty()))
        {
            self.record_claims(held, &self.owing());
        }
        let held_back = held_back(&documents);
        let mut standing = None;
        for recipient in recipients {
            self.hand_batch(&recipient, &mut standing, &held_back);
        }
    }

    /// Hands `recipient` its batch alone, as a settle would, where its
    /// session reads waiting (43, 214).
    pub(crate) fn hand_batch_to(&self, recipient: &str) {
        let documents: Vec<HeldState> = self
            .store
            .read_every()
            .into_iter()
            .map(|read| read.state)
            .collect();
        self.hand_batch(recipient, &mut None, &held_back(&documents));
    }

    /// Hands `recipient` its batch where it has one and its session reads
    /// waiting: claimed, read again, sent, and recorded handed on the
    /// send's answer, or let go of where the send was not made (43, 75,
    /// 214; core.11).
    fn hand_batch(&self, recipient: &str, standing: &mut Option<Standing>, held_back: &HeldBack) {
        let whole = match read_whole(&*self.store) {
            Ok(whole) => whole,
            Err(why) => {
                self.host.log(&why);
                return;
            }
        };
        let now = self.host.now();
        if batch(
            &owing(&whole.records, now),
            &whole.claims,
            held_back,
            recipient,
        )
        .is_empty()
        {
            return;
        }
        if self.takes_itself(recipient) {
            return;
        }
        let standing = standing.get_or_insert_with(|| self.standing());
        let Some(Reached {
            terminal,
            state: AgentState::Waiting,
            at,
        }) = self.reached(standing, recipient)
        else {
            return;
        };
        let Some(items) = self.claim_batch(recipient, held_back, now, None) else {
            return;
        };
        // An inbox, a wait or a session's start that handed an item over
        // between the read and the claim is read now, after the claim, and
        // that item not sent (43; core.25).
        let read = self.owing();
        let (items, gone): (Vec<Item>, Vec<Item>) = items.into_iter().partition(|item| {
            read.deliveries
                .get(&item.key)
                .is_some_and(|delivery| unhanded(delivery, recipient))
        });
        if !gone.is_empty() {
            self.release(recipient, now, &gone);
        }
        if items.is_empty() {
            return;
        }
        let why = match terminal.send_to(&at, &batch_words(&items, recipient)) {
            ToSession::Sent => {
                self.handed(recipient, now, &items);
                return;
            }
            ToSession::NotFound => format!("no session is named {at}"),
            ToSession::Refused { reason } => reason,
        };
        self.host.log(&format!(
            "the deliveries {} were not handed to {recipient}: {why}",
            items
                .iter()
                .map(|item| item.key.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        self.release(recipient, now, &items);
    }

    /// Claims `recipient`'s batch whole beside the host's deliveries in one
    /// write at the version read, `start` the start of a role's run whose
    /// prompt carries it, and removes every claim that no longer stands;
    /// the items claimed, none where there were none or the write failed
    /// (75, 214, C.7).
    pub(crate) fn claim_batch(
        &self,
        recipient: &str,
        held_back: &HeldBack,
        now: Seconds,
        start: Option<String>,
    ) -> Option<Vec<Item>> {
        let mut claimed: Vec<Item> = vec![];
        let written = rewrite_whole(&*self.store, |records, claims| {
            claimed.clear();
            let owing = owing(records, now);
            let kept: Vec<DeliveryClaim> = claims
                .iter()
                .filter(|claim| stands(claim, &owing))
                .cloned()
                .collect();
            let items = batch(&owing, &kept, held_back, recipient);
            if items.is_empty() && kept.len() == claims.len() {
                return None;
            }
            let mut next = kept;
            next.extend(items.iter().map(|item| DeliveryClaim {
                key: item.key.clone(),
                recipient: recipient.to_string(),
                at: now,
                sending: true,
                start: start.clone(),
            }));
            claimed = items;
            Some((records.to_vec(), next))
        });
        if let Err(why) = written {
            self.host.log(&why);
            return None;
        }
        (!claimed.is_empty()).then_some(claimed)
    }

    /// Removes the claims of `items` for `recipient` made at `at`,
    /// recording nothing, so the next wait takes them (43, 75).
    pub(crate) fn release(&self, recipient: &str, at: Seconds, items: &[Item]) {
        let written = rewrite_whole(&*self.store, |records, claims| {
            let next: Vec<DeliveryClaim> = claims
                .iter()
                .filter(|claim| !claimed_as(claim, recipient, at, items))
                .cloned()
                .collect();
            (next.len() != claims.len()).then(|| (records.to_vec(), next))
        });
        if let Err(why) = written {
            self.host.log(&why);
        }
    }

    /// Records each of `items` handed to `recipient` by its way at `at`,
    /// the claim's time, through weir's `hand`, and removes its claim, in
    /// one write; an item its recipient confirmed meanwhile adds nothing,
    /// and flume's log names each hand-over weir refused (43, 75, 214;
    /// weir 103).
    pub(crate) fn handed(&self, recipient: &str, at: Seconds, items: &[Item]) {
        let when = time(at);
        let mut refused: Vec<String> = vec![];
        let written = rewrite_whole(&*self.store, |records, claims| {
            refused.clear();
            let mut all = records.to_vec();
            for item in items {
                match weir_delivery::hand(&all, &item.key, recipient, item.by.way(), &when, false) {
                    Ok(Handed::Add(added)) => all.extend(added),
                    Ok(Handed::Confirmed(_)) => {}
                    Err(refusal) => refused.push(format!("{}: {refusal}", item.key)),
                }
            }
            let next: Vec<DeliveryClaim> = claims
                .iter()
                .filter(|claim| !claimed_as(claim, recipient, at, items))
                .cloned()
                .collect();
            Some((all, next))
        });
        for why in refused {
            self.host.log(&format!(
                "a delivery handed to {recipient} was not recorded: {why}"
            ));
        }
        let Err(why) = written else {
            return;
        };
        self.host.log(&format!(
            "the deliveries handed to {recipient} were not recorded: {why}"
        ));
        // The send was answered: its claims are marked so, and the next
        // settle records their hand-overs at the claim's time, once (43,
        // 75; weir 103).
        let marked = rewrite_whole(&*self.store, |records, claims| {
            let mut next = claims.to_vec();
            let mut changed = false;
            for claim in next
                .iter_mut()
                .filter(|claim| claimed_as(claim, recipient, at, items) && claim.sending)
            {
                claim.sending = false;
                changed = true;
            }
            changed.then(|| (records.to_vec(), next))
        });
        if let Err(why) = marked {
            self.host.log(&format!(
                "the deliveries handed to {recipient} were not marked answered: {why}"
            ));
        }
    }

    /// Records, at the claim's time, the hand-over of each claim answered
    /// sent whose hand-over the host's deliveries lack, by `notice` for an
    /// end and by `message` for a message, and removes it, in one write;
    /// a claim still sending is recorded by none, since its send may never
    /// have typed (43, 75; weir 103).
    fn record_answered(&self) {
        let mut refused: Vec<String> = vec![];
        let written = rewrite_whole(&*self.store, |records, claims| {
            refused.clear();
            if claims.iter().all(|claim| claim.sending) {
                return None;
            }
            let mut all = records.to_vec();
            for claim in claims.iter().filter(|claim| !claim.sending) {
                let by = if is_message_key(&claim.key) {
                    DeliveredBy::Message
                } else {
                    DeliveredBy::Notice
                };
                match weir_delivery::hand(
                    &all,
                    &claim.key,
                    &claim.recipient,
                    by.way(),
                    &time(claim.at),
                    false,
                ) {
                    Ok(Handed::Add(added)) => all.extend(added),
                    Ok(Handed::Confirmed(_)) => {}
                    Err(refusal) => refused.push(format!("{}: {refusal}", claim.key)),
                }
            }
            let next: Vec<DeliveryClaim> = claims
                .iter()
                .filter(|claim| claim.sending)
                .cloned()
                .collect();
            Some((all, next))
        });
        if let Err(why) = written {
            self.host.log(&why);
        }
        for why in refused {
            self.host
                .log(&format!("a claim answered sent was not recorded: {why}"));
        }
    }
}

impl Engine {
    /// The deliver part's take: refused unless `identity` names the live
    /// agent of the place's current stage, `secret` is its start's and
    /// its profile offers deliver; nothing while its asking mark stands,
    /// its prompt is not received or its stage waits on an answer;
    /// otherwise the oldest message waiting on its start, alone, marked
    /// taken, and else, for a role's run, its address's batch, recorded
    /// handed in one write with no send between; the words, or nothing
    /// (41, 42, 53, 54, 214, 215; design 7).
    pub fn take(&self, place: &PlaceId, identity: &str, secret: &str) -> Answer {
        let none = Answer::Taken { words: None };
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let held = read.state;
        let Some((index, start)) = held
            .current
            .and_then(|index| {
                held.stages[index]
                    .start
                    .as_ref()
                    .map(|start| (index, start))
            })
            .filter(|(_, start)| start.live && start.identity.to_string() == identity)
        else {
            return Answer::refused(format!(
                "no live agent of {place}'s current stage is {identity}"
            ));
        };
        if start.secret.0 != secret {
            return Answer::refused(format!("the secret is not {identity}'s start's"));
        }
        let Some(agent) = self.agent(&start.program) else {
            return Answer::refused(format!(
                "no agent profile runs the program {}",
                start.program
            ));
        };
        if agent.deliver() != Offered::Offered {
            return Answer::refused(format!(
                "the profile of {}, which {identity} runs, offers no deliver part",
                start.program
            ));
        }
        let folder = self.store.folder(place);
        if start.prompt.received_at.is_none()
            || crate::open_sessions::waits_on_answer(&held).is_some()
            || agent.asking(&folder, &start.identity) == Reading::Yes
        {
            return none;
        }
        let at = self.host.now();
        let start_identity = start.identity.clone();
        let taken = self
            .update(place, |held| {
                let Some(start) = held.stages[index]
                    .start
                    .as_mut()
                    .filter(|start| start.live && start.identity == start_identity)
                else {
                    return Step::Keep(None);
                };
                match start
                    .messages
                    .iter_mut()
                    .find(|message| message.taken_at.is_none())
                {
                    Some(message) => {
                        message.taken_at = Some(at);
                        Step::Write(Some(message.words.clone()))
                    }
                    None => Step::Keep(None),
                }
            })
            .flatten();
        if let Some(words) = taken {
            return Answer::Taken { words: Some(words) };
        }
        let Some(address) = held.role.as_deref() else {
            return none;
        };
        let items = self.take_batch(address, &held_back(&self.documents()), at);
        if items.is_empty() {
            return none;
        }
        Answer::Taken {
            words: Some(batch_words(&items, address)),
        }
    }

    /// `recipient`'s batch taken by its agent's deliver part: computed and
    /// recorded handed, each by its way at `at`, in one write at the
    /// version read, every claim that no longer stands removed; the items
    /// taken (43, 75, 214, 215).
    fn take_batch(&self, recipient: &str, held_back: &HeldBack, at: Seconds) -> Vec<Item> {
        let when = time(at);
        let mut taken: Vec<Item> = vec![];
        let written = rewrite_whole(&*self.store, |records, claims| {
            taken.clear();
            let owing = owing(records, at);
            let kept: Vec<DeliveryClaim> = claims
                .iter()
                .filter(|claim| stands(claim, &owing))
                .cloned()
                .collect();
            let items = batch(&owing, &kept, held_back, recipient);
            if items.is_empty() && kept.len() == claims.len() {
                return None;
            }
            let mut all = records.to_vec();
            for item in &items {
                if let Ok(Handed::Add(added)) =
                    weir_delivery::hand(&all, &item.key, recipient, item.by.way(), &when, false)
                {
                    all.extend(added);
                    taken.push(item.clone());
                }
            }
            Some((all, kept))
        });
        if let Err(why) = written {
            self.host.log(&why);
            return vec![];
        }
        taken
    }

    /// Whether `recipient` is a role's address whose run's live agent's
    /// profile offers deliver, so its agent takes its batch itself and a
    /// settle hands it nothing by the terminal (214, 215; design 6).
    fn takes_itself(&self, recipient: &str) -> bool {
        is_address(recipient)
            && self.documents().iter().any(|held| {
                held.role.as_deref() == Some(recipient)
                    && !matches!(held.phase, Phase::Tidied { .. })
                    && held.live_start().is_some_and(|(_, start)| {
                        self.agent(&start.program)
                            .is_some_and(|agent| agent.deliver() == Offered::Offered)
                    })
            })
    }
}

/// Whether `claim` is the claim of one of `items` for `recipient` made at
/// `at` (75).
fn claimed_as(claim: &DeliveryClaim, recipient: &str, at: Seconds, items: &[Item]) -> bool {
    claim.recipient == recipient && claim.at == at && items.iter().any(|item| item.key == claim.key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_drops_or_a_dones_end_is_carried_by_no_notice() {
        assert!(of_an_over("fix--one/0a1b2c3d/over/dropped"));
        assert!(of_an_over("fix--one/0a1b2c3d/over/done"));
        assert!(!of_an_over("fix--one/0a1b2c3d/stage/code/1/delivered"));
        assert!(!of_an_over("message/east/over/0a1b2c3d"));
    }

    #[test]
    fn items_are_joined_by_one_blank_line() {
        let items = vec![
            Item {
                key: "message/east/ops/0a1b2c3d".into(),
                message: "ops messages east/ops:\nlook".into(),
                by: DeliveredBy::Message,
            },
            Item {
                key: "fix--one/0a1b2c3d/stage/code/1/delivered".into(),
                message: "flume: fix--one code delivered".into(),
                by: DeliveredBy::Notice,
            },
        ];
        assert_eq!(
            batch_words(&items, "east/ops"),
            "ops messages east/ops:\nlook; confirm: flume confirm --recipient east/ops message/east/ops/0a1b2c3d\n\nflume: fix--one code delivered; confirm: flume confirm --recipient east/ops fix--one/0a1b2c3d/stage/code/1/delivered"
        );
    }
}
