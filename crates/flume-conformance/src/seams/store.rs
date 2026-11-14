//! The store seam's scenarios (B.7, 102 to 104).

use crate::scenario::sample_held;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::deliveries::{self, HostDeliveries};
use flume_seams::*;
use std::collections::BTreeMap;
use std::sync::Arc;
use weir_delivery::Store as _;

pub trait StoreHarness {
    type Seam: Store + ?Sized;

    fn seam(&self) -> &Self::Seam;

    /// The same store as another flume process on the host reaches it.
    fn another(&self) -> Arc<dyn Store>;

    /// A repository on the host whose content the store must not touch.
    fn repository(&self) -> String;

    /// A line appended to `made.jsonl` in the place's own folder, as a
    /// stage's run records what it made (129).
    fn record_made(&self, place: &PlaceId, line: &str);
}

fn place<H: StoreHarness>(harness: &H, name: &str) -> (PlaceId, HeldState) {
    let repository = harness.repository();
    let id = PlaceId::new(format!("{repository}/places/fix--{name}"));
    let held = sample_held(&id, &repository);
    (id, held)
}

fn written(answer: Written) -> Result<u64, String> {
    match answer {
        Written::Written { version } => Ok(version),
        Written::Moved => Err("a write was refused".into()),
    }
}

// ---- the store seam: a profile unit appends its own scenarios at the list's end ----

pub fn scenarios<H: StoreHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name: "store: a first write, then reads of one place and of every place",
            cites: &["102", "103"],
            run: write_then_read::<H>,
        },
        Scenario {
            name: "store: a write at a version that has moved is refused",
            cites: &["103", "75"],
            run: a_moved_version_is_refused::<H>,
        },
        Scenario {
            name: "store: a first write for a place with held state is refused",
            cites: &["103", "3", "75"],
            run: a_second_first_write_is_refused::<H>,
        },
        Scenario {
            name: "store: writing nothing removes the held state",
            cites: &["103", "62"],
            run: removal::<H>,
        },
        Scenario {
            name: "store: the lines of made.jsonl in a place's folder are read in order as written, and go with the place",
            cites: &["128", "129", "62"],
            run: made_lines::<H>,
        },
        Scenario {
            name: "store: every flume process reaches the same store, outside every repository",
            cites: &["104", "73", "123"],
            run: one_store_outside_the_repository::<H>,
        },
        Scenario {
            name: "store: the host's own documents are read and written by key, a write at a moved version refused, and every flume process reaches them",
            cites: &["102", "103", "104", "153"],
            run: the_hosts_own_documents::<H>,
        },
        Scenario {
            name: "store: two settles with a bound writing the host's watch from the same version, one write takes, the other is refused, and the one that read again writes over it",
            cites: &["73", "103", "153", "208"],
            run: two_watches_at_once::<H>,
        },
        Scenario {
            name: "store: a delivery, a hand-over, a confirmation and a known name written to the host's deliveries read back as written, in order, each spelled as weir's JSON spells it (C.8; weir 100, 112)",
            cites: &["43", "102", "103", "104"],
            run: deliveries_round_trip::<H>,
        },
        Scenario {
            name: "store: two engines confirming one end at the version both read, one write lands and the other reads again, so the host's deliveries hold one confirmation and one known name (weir 101, 102)",
            cites: &["43", "75", "103"],
            run: two_confirmations_at_once::<H>,
        },
        Scenario {
            name: "store: flume's store of deliveries passes every delivery case of weir's pack, its deliveries document holding a claim of flume's alone before each case, which each case keeps as it found it (weir 87, 112, 113)",
            cites: &["43", "75", "102", "103", "104"],
            run: the_pack_over_the_store::<H>,
        },
    ]
}

fn write_then_read<H: StoreHarness>(harness: &H) -> Outcome {
    let (id, held) = place(harness, "read");
    let version = written(harness.seam().write(&id, None, Some(&held)))?;
    let read = harness.seam().read(&id).ok_or("no held state read")?;
    ensure_eq!(read.version, version, "the version read");
    ensure_eq!(read.state, held, "the held state read");
    ensure!(
        harness
            .seam()
            .read_every()
            .iter()
            .any(|every| every.state.place.id == id),
        "every place's read lacks the place"
    );
    Ok(())
}

fn a_moved_version_is_refused<H: StoreHarness>(harness: &H) -> Outcome {
    let (id, mut held) = place(harness, "moved");
    let first = written(harness.seam().write(&id, None, Some(&held)))?;
    held.place.mark = Some("one".into());
    written(harness.seam().write(&id, Some(first), Some(&held)))?;
    held.place.mark = Some("two".into());
    ensure_eq!(
        harness.seam().write(&id, Some(first), Some(&held)),
        Written::Moved,
        "a write at the old version"
    );
    let read = harness.seam().read(&id).ok_or("no held state read")?;
    ensure_eq!(
        read.state.place.mark,
        Some("one".to_string()),
        "the held mark"
    );
    Ok(())
}

fn a_second_first_write_is_refused<H: StoreHarness>(harness: &H) -> Outcome {
    let (id, held) = place(harness, "twice");
    written(harness.seam().write(&id, None, Some(&held)))?;
    ensure_eq!(
        harness.seam().write(&id, None, Some(&held)),
        Written::Moved,
        "a second first write"
    );
    Ok(())
}

fn removal<H: StoreHarness>(harness: &H) -> Outcome {
    let (id, held) = place(harness, "removed");
    let version = written(harness.seam().write(&id, None, Some(&held)))?;
    written(harness.seam().write(&id, Some(version), None))?;
    ensure!(
        harness.seam().read(&id).is_none(),
        "removed held state is read"
    );
    Ok(())
}

fn made_lines<H: StoreHarness>(harness: &H) -> Outcome {
    let (id, held) = place(harness, "made");
    let version = written(harness.seam().write(&id, None, Some(&held)))?;
    ensure_eq!(
        harness.seam().made(&id),
        Vec::<String>::new(),
        "lines before any"
    );
    let lines = [
        r#"{"name":"spike","what":"spike","address":"https://forge.example/o/r/tree/spike/x?ref=a%2Fb"}"#,
        r#"{"name":"preview","what":"preview","branch":"fix/made","revision":"c3"}"#,
        "not one object",
    ];
    for line in lines {
        harness.record_made(&id, line);
    }
    ensure_eq!(harness.seam().made(&id), lines.to_vec(), "the lines read");
    ensure_eq!(
        harness.another().made(&id),
        lines.to_vec(),
        "another process's read"
    );
    written(harness.seam().write(&id, Some(version), None))?;
    ensure_eq!(
        harness.seam().made(&id),
        Vec::<String>::new(),
        "the lines after the place is removed"
    );
    Ok(())
}

fn one_store_outside_the_repository<H: StoreHarness>(harness: &H) -> Outcome {
    let (id, held) = place(harness, "shared");
    written(harness.seam().write(&id, None, Some(&held)))?;
    ensure_eq!(
        harness.another().read(&id).map(|read| read.state),
        Some(held),
        "another process's read"
    );
    let folder = harness.seam().folder(&id);
    ensure!(
        !folder.starts_with(&harness.repository()),
        "the place's folder {folder} is in the repository"
    );
    Ok(())
}

fn the_hosts_own_documents<H: StoreHarness>(harness: &H) -> Outcome {
    let account = HostKey::Account {
        pool: "work".into(),
        account: "work-1".into(),
    };
    let intent = HostKey::Intent {
        digest: "0f1e2d".into(),
    };
    ensure!(
        harness.seam().read_host(&account).is_none(),
        "a document read before any was written"
    );
    let spent = |until| {
        HostDocument::Account(AccountSpent {
            until,
            words: Some("You've hit your session limit".into()),
            recorded_at: 100,
            unread: false,
        })
    };
    let first = written(harness.seam().write_host(&account, None, Some(&spent(500))))?;
    ensure_eq!(
        harness.seam().write_host(&account, None, Some(&spent(600))),
        Written::Moved,
        "a second first write"
    );
    let read = harness
        .seam()
        .read_host(&account)
        .ok_or("no document read")?;
    ensure_eq!(read.version, first, "the version read");
    ensure_eq!(read.document, spent(500), "the document read");
    let another = harness.another();
    let landed = written(another.write_host(&account, Some(first), Some(&spent(700))))?;
    ensure_eq!(
        harness
            .seam()
            .write_host(&account, Some(first), Some(&spent(800))),
        Written::Moved,
        "the second writer at the version both read"
    );
    ensure_eq!(
        harness.seam().read_host(&account).map(|read| read.document),
        Some(spent(700)),
        "the document after the first writer's write"
    );
    written(
        harness
            .seam()
            .write_host(&account, Some(landed), Some(&spent(800))),
    )?;
    ensure_eq!(
        another.read_host(&account).map(|read| read.document),
        Some(spent(800)),
        "another process's read after the second writer wrote again"
    );
    let affinity = HostDocument::Intent(Affinity {
        stream: "work".into(),
        accounts: BTreeMap::from([
            (
                "agent".into(),
                AffinityAccount {
                    pool: "work".into(),
                    account: "agent-1".into(),
                },
            ),
            (
                "other".into(),
                AffinityAccount {
                    pool: "work".into(),
                    account: "other-1".into(),
                },
            ),
        ]),
        moves: vec![],
        gone: None,
        pool: None,
        account: None,
    });
    let version = written(harness.seam().write_host(&intent, None, Some(&affinity)))?;
    ensure_eq!(
        another.read_host(&intent).map(|read| read.document),
        Some(affinity),
        "another process's read of the intent"
    );
    written(harness.seam().write_host(&intent, Some(version), None))?;
    ensure!(
        harness.seam().read_host(&intent).is_none(),
        "a removed document is read"
    );
    ensure!(
        harness.seam().read_host(&account).is_some(),
        "the removal took another document"
    );
    Ok(())
}

/// The seam-contracts spec's "Two watches at once": the host's watch is
/// one document for the host, written by compare-and-set as every host
/// document is, none read on a host that holds none (73, 153, 208).
fn two_watches_at_once<H: StoreHarness>(harness: &H) -> Outcome {
    ensure!(
        harness.seam().read_host(&HostKey::Watch).is_none(),
        "a watch read on a host that holds none"
    );
    let watch = |began| HostDocument::Watch(Watched { began, bound: 120 });
    let at = written(
        harness
            .seam()
            .write_host(&HostKey::Watch, None, Some(&watch(100))),
    )?;
    let another = harness.another();
    let taken = written(another.write_host(&HostKey::Watch, Some(at), Some(&watch(160))))?;
    ensure_eq!(
        harness
            .seam()
            .write_host(&HostKey::Watch, Some(at), Some(&watch(170))),
        Written::Moved,
        "the second settle's write at the version both read"
    );
    let read = harness
        .seam()
        .read_host(&HostKey::Watch)
        .ok_or("no watch read")?;
    ensure_eq!(
        (read.version, read.document),
        (taken, watch(160)),
        "the watch after the first write took"
    );
    written(
        harness
            .seam()
            .write_host(&HostKey::Watch, Some(taken), Some(&watch(170))),
    )?;
    ensure_eq!(
        another.read_host(&HostKey::Watch).map(|read| read.document),
        Some(watch(170)),
        "the watch once the second read again and wrote"
    );
    Ok(())
}

/// The host's deliveries document removed, so a scenario starts from
/// none (C.7).
fn no_deliveries(store: &dyn Store) -> Outcome {
    if let Some(read) = store.read_host(&HostKey::Deliveries) {
        written(store.write_host(&HostKey::Deliveries, Some(read.version), None))?;
    }
    Ok(())
}

const DELIVERED: &str = "fix--deliveries/0a1b2c3d/stage/code/1/delivered";

/// The delivery of a fix's code end to `ops` and `boomdrive`, as weir's
/// `record` gives it (weir 100, 108).
fn a_delivery() -> Result<Vec<weir_delivery::Record>, String> {
    match weir_delivery::record(
        &[],
        DELIVERED,
        "fix--deliveries: code delivered",
        &["ops", "boomdrive"],
        "2026-11-08T15:02:00Z",
        604_800,
    )
    .map_err(|refused| refused.to_string())?
    {
        weir_delivery::Recorded::Add(records) => Ok(records),
        weir_delivery::Recorded::Held(_) => Err("a first record held its key".into()),
    }
}

fn deliveries_round_trip<H: StoreHarness>(harness: &H) -> Outcome {
    let store = harness.another();
    no_deliveries(&*store)?;
    let mut records = a_delivery()?;
    match weir_delivery::hand(
        &records,
        DELIVERED,
        "ops",
        "inbox",
        "2026-11-08T15:03:00Z",
        true,
    )
    .map_err(|refused| refused.to_string())?
    {
        weir_delivery::Handed::Add(added) => records.extend(added),
        weir_delivery::Handed::Confirmed(_) => {
            return Err("handed to a reader that confirmed".into())
        }
    }
    let formats: Vec<&str> = records.iter().map(weir_delivery::Record::format).collect();
    ensure_eq!(
        formats,
        vec![
            weir_delivery::DELIVERY,
            weir_delivery::HANDOVER,
            weir_delivery::CONFIRMATION,
            weir_delivery::KNOWN,
        ],
        "a record of each of weir's four formats"
    );
    HostDeliveries::new(&*store).add(&records)?;
    ensure_eq!(
        HostDeliveries::new(&*harness.another()).read()?,
        records,
        "the records another process reads back, in order"
    );
    let read = store
        .read_host(&HostKey::Deliveries)
        .ok_or("no deliveries document read")?;
    let json = serde_json::to_string(&read).map_err(|error| error.to_string())?;
    for record in &records {
        let spelled = weir_delivery::json::write(record);
        ensure!(
            json.contains(&spelled),
            "the document's JSON lacks {spelled}: {json}"
        );
    }
    Ok(())
}

fn two_confirmations_at_once<H: StoreHarness>(harness: &H) -> Outcome {
    let (one, other) = (harness.another(), harness.another());
    no_deliveries(&*one)?;
    HostDeliveries::new(&*one).add(&a_delivery()?)?;
    let confirm = |records: &[weir_delivery::Record], at: &str| match weir_delivery::confirm(
        records, DELIVERED, "ops", at,
    ) {
        Ok(weir_delivery::Confirmed::Add(added)) => added,
        Ok(weir_delivery::Confirmed::Already(_)) | Err(_) => Vec::new(),
    };
    let mut asked = 0;
    let mut inner: Option<Result<Vec<weir_delivery::Record>, String>> = None;
    let held = deliveries::update(&*one, |records| {
        asked += 1;
        if inner.is_none() {
            // The other engine confirms at the version this one read, and
            // its write lands first.
            inner = Some(deliveries::update(&*other, |records| {
                confirm(records, "2026-11-08T15:04:00Z")
            }));
        }
        confirm(records, "2026-11-08T15:05:00Z")
    })?;
    inner.ok_or("the other engine never wrote")??;
    ensure_eq!(asked, 2, "the times the first engine read the records");
    let confirmations: Vec<_> = held
        .iter()
        .filter(|record| matches!(record, weir_delivery::Record::Confirmation(_)))
        .collect();
    let known = held
        .iter()
        .filter(|record| matches!(record, weir_delivery::Record::Known(_)))
        .count();
    ensure_eq!(
        confirmations.len(),
        1,
        format!("the confirmations held: {held:?}")
    );
    ensure_eq!(known, 1, format!("the known names held: {held:?}"));
    ensure!(
        matches!(confirmations[0], weir_delivery::Record::Confirmation(confirmation)
            if confirmation.at.as_str() == "2026-11-08T15:04:00Z"),
        "the confirmation held is the other engine's: {held:?}"
    );
    ensure_eq!(
        HostDeliveries::new(&*harness.another()).read()?,
        held,
        "the records as the store holds them"
    );
    Ok(())
}

/// The claim of flume's each case of weir's pack begins over, as the
/// deliveries spec's "weir's pack over a document with claims" gives it
/// (75, C.7).
fn a_claim() -> DeliveryClaim {
    DeliveryClaim {
        key: "message/east/planner/0a1b2c3d".into(),
        recipient: "east/planner".into(),
        at: 1_794_150_180,
        sending: true,
        start: None,
    }
}

/// The host's deliveries holding [`a_claim`] and no record, as each case
/// of weir's pack begins (C.7; weir 113).
fn only_a_claim(store: &dyn Store) -> Outcome {
    no_deliveries(store)?;
    let document = HostDocument::Deliveries(Deliveries {
        records: vec![],
        claims: vec![a_claim()],
    });
    written(store.write_host(&HostKey::Deliveries, None, Some(&document)))?;
    Ok(())
}

/// Whether the host's deliveries still hold [`a_claim`] alone, as a case
/// of weir's pack found it (C.7; core.4).
fn the_claim_kept(store: &dyn Store) -> Outcome {
    let claims = deliveries::read_whole(store)?.claims;
    ensure_eq!(
        claims,
        vec![a_claim()],
        "the claims a case of weir's pack left"
    );
    Ok(())
}

fn the_pack_over_the_store<H: StoreHarness>(harness: &H) -> Outcome {
    let store = harness.another();
    let mut fault: Outcome = Ok(());
    let mut begun = false;
    let verdicts = weir_pack::conform_deliveries(&weir_pack::Pack::embedded(), || {
        if begun {
            if let Err(why) = the_claim_kept(&*store) {
                fault = fault.clone().and(Err(why));
            }
        }
        begun = true;
        if let Err(why) = only_a_claim(&*store) {
            fault = fault.clone().and(Err(why));
        }
        HostDeliveries::new(&*store)
    });
    fault?;
    ensure!(!verdicts.is_empty(), "weir's pack ran no delivery case");
    the_claim_kept(&*store)?;
    let failed: Vec<String> = verdicts
        .into_iter()
        .filter_map(|(case, verdict)| {
            verdict
                .err()
                .map(|why| format!("{case}: {}", why.join("; ")))
        })
        .collect();
    ensure!(
        failed.is_empty(),
        "cases of weir's pack failed: {}",
        failed.join(" | ")
    );
    no_deliveries(&*store)
}
