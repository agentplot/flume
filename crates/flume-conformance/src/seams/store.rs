//! The store seam's scenarios (B.7, 102 to 104).

use crate::scenario::sample_held;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;
use std::sync::Arc;

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
        pool: "work".into(),
        account: "work-1".into(),
        moves: vec![],
        gone: None,
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
