//! The store profile over the host's state directory (C.7; design 10):
//! the store seam's scenarios against it over a scratch state directory,
//! two processes writing at one version, and S12 with the store the
//! engine's.

use crate::common::*;
use flume::profiles::store::state_dir::StateDir;
use flume_conformance::engine::world::{launch, launched, World};
use flume_conformance::scenario::{pass_all, sample_held};
use flume_conformance::seams::store::{self, StoreHarness};
use flume_core::Engine;
use flume_seams::*;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct Harness {
    scratch: Scratch,
    seam: StateDir,
}

impl Harness {
    fn new() -> Harness {
        let scratch = Scratch::new("store");
        let seam = StateDir::at(scratch.path("state/flume"));
        Harness { scratch, seam }
    }
}

impl StoreHarness for Harness {
    type Seam = StateDir;

    fn seam(&self) -> &StateDir {
        &self.seam
    }

    fn another(&self) -> Arc<dyn Store> {
        Arc::new(StateDir::at(self.scratch.path("state/flume")))
    }

    fn repository(&self) -> String {
        self.scratch
            .path("repository")
            .to_string_lossy()
            .into_owned()
    }

    fn record_made(&self, place: &PlaceId, line: &str) {
        use std::io::Write;
        let path = Path::new(&self.seam.folder(place)).join(MADE);
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("made.jsonl");
        writeln!(file, "{line}").expect("a line of made.jsonl");
    }
}

#[test]
fn the_store_profile_passes_the_store_scenarios() {
    pass_all("store", &store::scenarios(), Harness::new);
}

#[test]
fn a_removed_place_leaves_nothing_in_the_state_directory() {
    let harness = Harness::new();
    let id = PlaceId::new("/repository/places/fix--gone");
    let held = sample_held(&id, "/repository");
    let Written::Written { version } = harness.seam.write(&id, None, Some(&held)) else {
        panic!("the first write was refused");
    };
    let folder = harness.seam.folder(&id);
    assert!(
        Path::new(&folder).is_dir(),
        "the place's folder is not made"
    );
    std::fs::write(Path::new(&folder).join("mark"), "").unwrap();
    assert!(matches!(
        harness.seam.write(&id, Some(version), None),
        Written::Written { .. }
    ));
    let left: Vec<String> = std::fs::read_dir(harness.seam.places())
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    assert!(left.is_empty(), "the state directory keeps {left:?}");
    let Written::Written { version: again } = harness.seam.write(&id, None, Some(&held)) else {
        panic!("a place made again was refused");
    };
    assert!(
        again > version,
        "a place made again reused the version {version}"
    );
}

/// The variables a child process of this test binary is handed.
const CHILD_ROOT: &str = "FLUME_TEST_STORE_ROOT";
const CHILD_AT: &str = "FLUME_TEST_STORE_AT";
const CHILD_MARK: &str = "FLUME_TEST_STORE_MARK";
const CHILD_GO: &str = "FLUME_TEST_STORE_GO";
const CHILD: &str = "store_state_dir::a_child_process_writes_at_a_version";

/// The child's half of the two-process write: once the go file stands,
/// one write at the version it is handed, its answer printed. Run by the
/// test below as a process of its own; it does nothing otherwise.
#[test]
fn a_child_process_writes_at_a_version() {
    let Some(root) = std::env::var_os(CHILD_ROOT) else {
        return;
    };
    let at: u64 = std::env::var(CHILD_AT).unwrap().parse().unwrap();
    let mark = std::env::var(CHILD_MARK).unwrap();
    let go = std::env::var_os(CHILD_GO).unwrap();
    let store = StateDir::at(root);
    let id = PlaceId::new("/repository/places/fix--shared");
    let mut held = store.read(&id).expect("the place's held state").state;
    held.place.mark = Some(mark);
    let deadline = Instant::now() + Duration::from_secs(30);
    while !Path::new(&go).exists() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(5));
    }
    let answer = match store.write(&id, Some(at), Some(&held)) {
        Written::Written { .. } => "written",
        Written::Moved => "moved",
    };
    println!("\nanswer={answer}");
}

#[test]
fn two_processes_writing_at_one_version_leave_one_write() {
    let harness = Harness::new();
    let root = harness.scratch.path("state/flume");
    let id = PlaceId::new("/repository/places/fix--shared");
    let held = sample_held(&id, "/repository");
    let Written::Written { mut version } = harness.seam.write(&id, None, Some(&held)) else {
        panic!("the first write was refused");
    };
    for round in 0..5 {
        let go = harness.scratch.path(&format!("go-{round}"));
        let children: Vec<_> = ["one", "two"]
            .iter()
            .map(|mark| {
                Command::new(std::env::current_exe().unwrap())
                    .args([CHILD, "--exact", "--nocapture", "--test-threads=1"])
                    .env(CHILD_ROOT, &root)
                    .env(CHILD_AT, version.to_string())
                    .env(CHILD_MARK, format!("{mark}-{round}"))
                    .env(CHILD_GO, &go)
                    .stdout(std::process::Stdio::piped())
                    .spawn()
                    .expect("a child process")
            })
            .collect();
        std::fs::write(&go, "").unwrap();
        let answers: Vec<String> = children
            .into_iter()
            .map(|child| {
                let output = child.wait_with_output().expect("the child's answer");
                assert!(output.status.success(), "a child failed");
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .find_map(|line| line.split("answer=").nth(1).map(str::to_string))
                    .unwrap_or_else(|| panic!("the child gave no answer: {stdout}"))
            })
            .collect();
        let written = answers.iter().filter(|answer| *answer == "written").count();
        assert_eq!(written, 1, "round {round}: {answers:?}");
        let read = harness.seam.read(&id).expect("the held state");
        assert!(read.version > version, "the version did not move");
        let mark = read.state.place.mark.clone().unwrap_or_default();
        assert!(mark.ends_with(&format!("-{round}")), "the mark {mark}");
        version = read.version;
    }
}

#[test]
fn s12_a_restart_over_the_state_directory_loses_nothing() {
    let scratch = Scratch::new("restart");
    let root = scratch.path("state/flume");
    let world = World::new();
    let over = |store: Arc<dyn Store>| {
        Engine::new(
            world.resolver.clone(),
            world.terminal.clone(),
            store,
            world.history.clone(),
            world.host.clone(),
            world.defaults.clone(),
        )
    };
    let engine = over(Arc::new(StateDir::at(&root)));
    let id = World::id("fix", "one");
    launched(&engine.launch(&launch("fix", "one", Some("x")))).expect("the launch");
    engine.settle(Some(&id));
    let code = World::identity("fix", "one", "code", 1);
    world.terminal.set_state(&code, AgentState::Working);
    over(Arc::new(StateDir::at(&root))).settle(Some(&id));
    let before = StateDir::at(&root).read(&id).expect("the place").state;
    assert!(
        before
            .stages
            .iter()
            .any(|stage| stage.name == "code" && stage.state == StageState::Begun),
        "the code stage works: {:?}",
        before.stages
    );

    world.host.restart();
    world.terminal.forget_all();
    let rebuilt = over(Arc::new(StateDir::at(&root)));
    let after = StateDir::at(&root)
        .read(&id)
        .expect("the place's held state was lost")
        .state;
    assert_eq!(after, before, "the held state after a restart");
    rebuilt.settle(None);
    let state = StateDir::at(&root)
        .read(&id)
        .and_then(|read| {
            read.state
                .stages
                .into_iter()
                .find(|stage| stage.name == "code")
        })
        .map(|stage| stage.state);
    assert_eq!(
        state,
        Some(StageState::Lost),
        "the code stage after a restart"
    );
}

#[test]
fn the_hosts_deliveries_are_seen_by_another_process() {
    // C.7; weir 112: the store-profile spec's "Seen by another process":
    // one engine over a scratch state directory records an end's
    // delivery, a second engine over the same directory reads it as
    // written, and `deliveries.json` is readable by the host's user alone.
    use flume_core::deliveries::{owing, read};
    use std::os::unix::fs::PermissionsExt;
    let scratch = Scratch::new("deliveries");
    let root = scratch.path("state/flume");
    let world = World::new();
    let over = |store: Arc<dyn Store>| {
        Engine::new(
            world.resolver.clone(),
            world.terminal.clone(),
            store,
            world.history.clone(),
            world.host.clone(),
            world.defaults.clone(),
        )
    };
    let one = StateDir::at(&root);
    let engine = over(Arc::new(StateDir::at(&root)));
    let id = World::id("fix", "one");
    launched(&engine.launch(&launch("fix", "one", Some("x")))).expect("the launch");
    engine.settle(Some(&id));
    let dropped = engine.drop_place(&id, true);
    assert!(dropped.refusal().is_none(), "the drop: {dropped:?}");
    let (version, written) = read(&one).expect("the deliveries read");
    let recorded = owing(&written, world.host.now());
    assert!(
        recorded
            .deliveries
            .keys()
            .any(|key| key.starts_with("fix--one/") && key.ends_with("/over/dropped")),
        "no delivery of the drop recorded: {recorded:?}"
    );
    let two = StateDir::at(&root);
    assert_eq!(
        read(&two).expect("the deliveries read again"),
        (version, written),
        "the deliveries another process reads"
    );
    let flume_core::Answer::Status { places, .. } =
        over(Arc::new(StateDir::at(&root))).status(Some(&id))
    else {
        panic!("status was refused");
    };
    let end = places[0]
        .ends
        .iter()
        .find(|end| end.key.to_string() == "over/dropped")
        .expect("the drop's end");
    assert!(
        end.delivery.is_some() && end.pending_for == vec![world_caller()],
        "the drop's end as another engine gives it: {end:?}"
    );
    let file = root.join("deliveries.json");
    let mode = std::fs::metadata(&file)
        .unwrap_or_else(|error| panic!("{}: {error}", file.display()))
        .permissions()
        .mode();
    assert_eq!(mode & 0o777, 0o600, "the deliveries' mode");
}

/// The recipient the fakes' world names in its calling command's
/// environment, which a launch that names none takes (7).
fn world_caller() -> String {
    flume_conformance::engine::world::CALLER.to_string()
}

/// The account `work-1` of the pool `work` spent until `until`.
fn spent_until(until: Seconds) -> HostDocument {
    HostDocument::Account(AccountDocument {
        spent: Some(AccountSpent {
            until,
            words: Some("You've hit your session limit · resets 3pm (UTC)".into()),
            recorded_at: until - 3600,
            unread: false,
        }),
        reading: None,
    })
}

fn work_1() -> HostKey {
    HostKey::Account {
        pool: "work".into(),
        account: "work-1".into(),
    }
}

#[test]
fn two_processes_record_one_account_spent() {
    let harness = Harness::new();
    let one = StateDir::at(harness.scratch.path("state/flume"));
    let two = StateDir::at(harness.scratch.path("state/flume"));
    let Written::Written { version } = one.write_host(&work_1(), None, Some(&spent_until(54_000)))
    else {
        panic!("the first write was refused");
    };
    let read = two.read_host(&work_1()).expect("the account's document");
    assert_eq!(read.version, version);
    assert!(matches!(
        one.write_host(&work_1(), Some(version), Some(&spent_until(55_000))),
        Written::Written { .. }
    ));
    assert_eq!(
        two.write_host(&work_1(), Some(read.version), Some(&spent_until(56_000))),
        Written::Moved,
        "the second process's write at the version both read"
    );
    let again = two.read_host(&work_1()).expect("the account's document");
    assert_eq!(again.document, spent_until(55_000));
    assert!(matches!(
        two.write_host(&work_1(), Some(again.version), Some(&spent_until(56_000))),
        Written::Written { .. }
    ));
    assert_eq!(
        one.read_host(&work_1()).map(|read| read.document),
        Some(spent_until(56_000))
    );
}

#[test]
fn a_spent_time_is_seen_by_another_process_and_kept_from_other_users() {
    use std::os::unix::fs::PermissionsExt;
    let harness = Harness::new();
    let root = harness.scratch.path("state/flume");
    assert!(matches!(
        StateDir::at(&root).write_host(&work_1(), None, Some(&spent_until(54_000))),
        Written::Written { .. }
    ));
    assert_eq!(
        StateDir::at(&root)
            .read_host(&work_1())
            .map(|read| read.document),
        Some(spent_until(54_000))
    );
    let file = root.join("accounts/work/work-1.json");
    let mode = std::fs::metadata(&file)
        .unwrap_or_else(|error| panic!("{}: {error}", file.display()))
        .permissions()
        .mode();
    assert_eq!(mode & 0o777, 0o600, "the document's mode");
    let intent = HostKey::Intent {
        digest: "0f1e2d".into(),
    };
    assert!(matches!(
        StateDir::at(&root).write_host(&intent, None, Some(&two_programs())),
        Written::Written { .. }
    ));
    assert!(root.join("intents/0f1e2d.json").is_file());
}

/// A reading of work-1 taken at 14:02 from a fix's code agent: its
/// 5-hour window 42% used resetting at 17:00, and its 7-day window 23.5%
/// with no reset (184, C.7).
fn a_reading() -> AccountReading {
    AccountReading {
        taken_at: 1_793_887_320,
        agent: "flume:r:fix--one:code:1".into(),
        session: Some("s-1".into()),
        windows: vec![
            UsageWindow {
                name: "five_hour".into(),
                used: 4_200,
                resets_at: Some(1_793_898_000),
            },
            UsageWindow {
                name: "seven_day".into(),
                used: 2_350,
                resets_at: None,
            },
        ],
    }
}

/// The store-profile spec's scenario "A reading beside a spent time"
/// (153, 184, C.7).
#[test]
fn an_accounts_reading_beside_its_spent_time() {
    use std::os::unix::fs::PermissionsExt;
    let harness = Harness::new();
    let root = harness.scratch.path("state/flume");
    let HostDocument::Account(spent) = spent_until(1_793_890_800) else {
        unreachable!("an account's document");
    };
    let document = HostDocument::Account(AccountDocument {
        reading: Some(a_reading()),
        ..spent
    });
    assert!(matches!(
        StateDir::at(&root).write_host(&work_1(), None, Some(&document)),
        Written::Written { .. }
    ));
    let file = root.join("accounts/work/work-1.json");
    let text = std::fs::read_to_string(&file)
        .unwrap_or_else(|error| panic!("{}: {error}", file.display()));
    let written: serde_json::Value = serde_json::from_str(&text).expect("JSON");
    let held = &written["document"];
    assert_eq!(held["until"], serde_json::json!(1_793_890_800), "{text}");
    assert_eq!(
        held["reading"],
        serde_json::json!({
            "taken_at": "2026-11-05T14:02:00Z",
            "agent": "flume:r:fix--one:code:1",
            "session": "s-1",
            "windows": [
                {"name": "five_hour", "used": 42, "resets_at": "2026-11-05T17:00:00Z"},
                {"name": "seven_day", "used": 23.5},
            ],
        }),
        "{text}"
    );
    let mode = std::fs::metadata(&file)
        .expect("the document")
        .permissions()
        .mode();
    assert_eq!(mode & 0o777, 0o600, "the document's mode");
    assert_eq!(
        StateDir::at(&root)
            .read_host(&work_1())
            .map(|read| read.document),
        Some(document),
        "a second store's read"
    );
}

/// The store-profile spec's scenario "An account's document an earlier
/// release wrote" (153).
#[test]
fn an_accounts_document_an_earlier_release_wrote() {
    let harness = Harness::new();
    let root = harness.scratch.path("state/flume");
    let file = root.join("accounts/work/work-1.json");
    std::fs::create_dir_all(file.parent().expect("the accounts directory")).expect("made");
    let before = serde_json::json!({
        "version": 3,
        "document": {
            "of": "account",
            "until": 54_000,
            "words": "You've hit your session limit",
            "recorded_at": 47_400,
        },
    });
    std::fs::write(&file, before.to_string()).expect("written");
    let read = StateDir::at(&root)
        .read_host(&work_1())
        .expect("the document");
    assert_eq!(read.version, 3);
    let HostDocument::Account(document) = read.document else {
        panic!("an account's document read as another's");
    };
    assert_eq!(
        document,
        AccountDocument {
            spent: Some(AccountSpent {
                until: 54_000,
                words: Some("You've hit your session limit".into()),
                recorded_at: 47_400,
                unread: false,
            }),
            reading: None,
        }
    );
    let next = HostDocument::Account(AccountDocument {
        reading: Some(a_reading()),
        ..document
    });
    assert!(matches!(
        StateDir::at(&root).write_host(&work_1(), Some(3), Some(&next)),
        Written::Written { .. }
    ));
    let text = std::fs::read_to_string(&file).expect("read");
    let written: serde_json::Value = serde_json::from_str(&text).expect("JSON");
    let held = &written["document"];
    for (key, value) in [
        ("until", serde_json::json!(54_000)),
        ("words", serde_json::json!("You've hit your session limit")),
        ("recorded_at", serde_json::json!(47_400)),
    ] {
        assert_eq!(held[key], value, "{key} in {text}");
    }
    assert_eq!(
        held["reading"]["taken_at"],
        serde_json::json!("2026-11-05T14:02:00Z"),
        "{text}"
    );
}

/// An intent's affinity holding an account for `agent` and one for
/// `other` (147).
fn two_programs() -> HostDocument {
    let kept = |account: &str| AffinityAccount {
        pool: "work".into(),
        account: account.into(),
    };
    HostDocument::Intent(Affinity {
        stream: "work".into(),
        accounts: [("agent", "agent-1"), ("other", "other-1")]
            .into_iter()
            .map(|(program, account)| (program.to_string(), kept(account)))
            .collect(),
        moves: vec![],
        gone: None,
        pool: None,
        account: None,
    })
}

#[test]
fn an_intents_document_an_earlier_release_wrote() {
    let harness = Harness::new();
    let root = harness.scratch.path("state/flume");
    let file = root.join("intents/0f1e2d.json");
    std::fs::create_dir_all(file.parent().expect("the intents directory")).expect("made");
    let before = serde_json::json!({
        "version": 3,
        "document": {
            "of": "intent",
            "stream": "work",
            "pool": "work",
            "account": "work-2",
            "moves": [{
                "on": "intent",
                "from": "work-1",
                "to": "work-2",
                "why": "spent",
                "at": 47_400,
            }],
            "gone": null,
        },
    });
    std::fs::write(&file, before.to_string()).expect("written");
    let intent = HostKey::Intent {
        digest: "0f1e2d".into(),
    };
    let read = StateDir::at(&root)
        .read_host(&intent)
        .expect("the document");
    assert_eq!(read.version, 3);
    let HostDocument::Intent(affinity) = read.document else {
        panic!("an intent's document read as an account's");
    };
    assert_eq!(affinity.pool.as_deref(), Some("work"));
    assert_eq!(affinity.account.as_deref(), Some("work-2"));
    assert!(affinity.accounts.is_empty());
    assert_eq!(affinity.moves[0].on, MoveOn::Intent { program: None });
    assert!(matches!(
        StateDir::at(&root).write_host(&intent, Some(3), Some(&two_programs())),
        Written::Written { .. }
    ));
    let text = std::fs::read_to_string(&file).expect("read");
    let written: serde_json::Value = serde_json::from_str(&text).expect("JSON");
    let document = &written["document"];
    assert!(document["accounts"]["agent"].is_object(), "{text}");
    assert!(document["accounts"]["other"].is_object(), "{text}");
    assert!(
        document.get("pool").is_none() && document.get("account").is_none(),
        "{text}"
    );
}

#[test]
fn a_name_reaches_nothing_outside_its_directory() {
    let harness = Harness::new();
    let root = harness.scratch.path("state/flume");
    let key = HostKey::Account {
        pool: "../out".into(),
        account: ".hidden".into(),
    };
    assert!(matches!(
        StateDir::at(&root).write_host(&key, None, Some(&spent_until(54_000))),
        Written::Written { .. }
    ));
    assert!(root.join("accounts/%2E.%2Fout/%2Ehidden.json").is_file());
    assert_eq!(
        StateDir::at(&root)
            .read_host(&key)
            .map(|read| read.document),
        Some(spent_until(54_000))
    );
}

#[test]
fn a_file_of_a_places_folder_is_written_whole_for_the_hosts_user_alone() {
    use std::os::unix::fs::PermissionsExt;
    let harness = Harness::new();
    let id = PlaceId::new("/repository/places/fix--own");
    let held = sample_held(&id, "/repository");
    assert!(matches!(
        harness.seam.write(&id, None, Some(&held)),
        Written::Written { .. }
    ));
    let folder = harness.seam.folder(&id);
    let path = format!("{folder}/agent/flume%r%fix--own%code%1/own.json");
    assert_eq!(harness.seam.write_file(&id, &path, "{\"a\":1}"), Ok(()));
    assert_eq!(harness.seam.write_file(&id, &path, "{\"a\":2}"), Ok(()));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "{\"a\":2}");
    let mode = std::fs::metadata(&path).unwrap().permissions().mode();
    assert_eq!(mode & 0o777, 0o600, "the file's mode");
    let beside: Vec<_> = std::fs::read_dir(Path::new(&path).parent().unwrap())
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(beside, vec!["own.json"], "nothing left beside it");
    for outside in [
        harness
            .scratch
            .path("elsewhere.json")
            .to_string_lossy()
            .into_owned(),
        format!("{folder}/../escape.json"),
        folder.clone(),
    ] {
        let refused = harness.seam.write_file(&id, &outside, "x").unwrap_err();
        assert!(refused.contains("outside the place's folder"), "{refused}");
    }
    assert!(!harness.scratch.path("elsewhere.json").exists());
    assert!(matches!(
        harness
            .seam
            .write(&id, Some(read_version(&harness, &id)), None),
        Written::Written { .. }
    ));
    assert!(!Path::new(&path).exists(), "the file went with the folder");
}

fn read_version(harness: &Harness, id: &PlaceId) -> u64 {
    harness.seam.read(id).expect("held state").version
}
