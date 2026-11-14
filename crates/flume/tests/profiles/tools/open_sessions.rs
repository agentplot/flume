//! The agent kept open for its answer, through the built binary under the
//! pinned herdr (design 10; S4, S104, S105; 42, 49, 50, 53 to 55, 70, 96,
//! 146, 198): each walkthrough a fix on a host under herdr, its code
//! agents the stand-in for Claude Code in panes of flume's own herdr
//! session, each start acting its own script, and the test settling as a
//! caller would. An answer within the hour, by a retry and then by a tell,
//! reaches the same session, each recorded, code started once and the
//! place landed (S4); an agent's time passing lets it go and ends it, its
//! room taken by the place waiting, a tell then refused naming the retry,
//! and the late retry's fresh agent carries the question and the answer
//! (S104); and an agent whose session ended, and one whose account was
//! spent, within the hour, are each let go and answered by a retry whose
//! fresh agent carries the question and the answer (S105).

use super::walk::*;
use flume::profiles::store::state_dir::StateDir;
use flume_seams::{AccountDocument, AccountSpent, HostDocument, HostKey, Store, Written};
use serde_json::{json, Value};
use std::time::Duration;

/// The question the code agent stops short on first.
const WHICH: &str = "Which file?";

/// The prompt a start's agent takes in its session for the question
/// `question` and the answer `answer`, as a retry's words give them.
fn asked(question: &str, answer: &str) -> String {
    format!("The question: {question}\n\nThe answer: {answer}")
}

/// The act of stopping short needing an answer, `question`.
fn needs(question: &str) -> Value {
    json!({ "bash": format!("flume needs '{question}'") })
}

/// The acts of a code stage that writes `notes.md` and commits it.
fn commits(text: &str) -> Value {
    json!([
        { "write": { "path": "notes.md", "text": text } },
        { "bash": "git add -A && git commit --quiet -m 'write notes.md'" },
    ])
}

/// Whether the place waits on an answer to code's question `question`.
fn waits_on(status: &Value, question: &str) -> bool {
    let waiting = &status["waiting_on"];
    waiting["act"] == "retry"
        && waiting["stage"] == "code"
        && waiting["why"] == "answer"
        && waiting["words"] == question
}

/// Settles the place until it is tidied, its work landed.
fn until_landed(host: &UnderHerdr, place: &str) {
    host.until_within(
        place,
        "landed and tidied",
        Duration::from_secs(180),
        Duration::from_millis(300),
        Value::is_null,
    );
}

/// Settles every place, as a caller keeping watch over the host does
/// (208), and reads the place's status until `holds` holds of it, for up
/// to three minutes; that status.
fn watching(host: &UnderHerdr, place: &str, what: &str, holds: impl Fn(&Value) -> bool) -> Value {
    let began = std::time::Instant::now();
    loop {
        host.flume(&["settle"]);
        let status = host.status(place);
        if holds(&status) {
            return status;
        }
        assert!(
            began.elapsed() < Duration::from_secs(180),
            "{place} never came to {what}: {status}"
        );
        std::thread::sleep(Duration::from_millis(300));
    }
}

/// flume's answer to `words`, refused, and its refusal's words.
fn refused(host: &UnderHerdr, words: &[&str]) -> String {
    let (answer, refused) = host.flume_ran(words);
    assert!(refused, "flume {} was taken: {answer}", words.join(" "));
    answer.to_string()
}

#[test]
fn s4_an_answer_within_the_hour_by_a_retry_and_a_tell_reaches_the_same_session() {
    // S4; 42, 49, 53 to 55, 198: code's first start stops short needing
    // an answer; status gives the move as a retry of code, its agent kept
    // open until an hour after the stop-short, and `keep_open` from flume's
    // own default. A retry with the words `notes.md` reaches the same
    // session, which takes the question and the answer as its next prompt
    // and stops short again, an end of its own; a tell reaches the same
    // session too, and its last run commits. Code started once, status
    // gave both answers with their questions and acts, and the place lands.
    let host = Walk::under_herdr("open-s4");
    host.script(
        "code",
        1,
        json!([
            [needs(WHICH)],
            [needs("And its text?")],
            commits("Hello.\n"),
        ]),
    );
    let place = host.launch("fix", "one", &["--brief", "write the notes"]);
    let status = host.until(&place, "code waiting on its answer", |status| {
        waits_on(status, WHICH)
    });
    let waiting = &status["waiting_on"];
    let since = waiting["since"].as_u64().expect("since when");
    let open_until = waiting["open_until"]
        .as_u64()
        .unwrap_or_else(|| panic!("its agent not kept open: {status}"));
    assert!(
        (3595..=3600).contains(&(open_until - since)),
        "kept open until {open_until}, the stop-short at {since}: {status}"
    );
    assert!(waiting["closed"].is_null(), "{status}");
    assert_eq!(
        status["agent"]["settings"]["from"]["keep_open"],
        json!({ "layer": "default", "value": "1h" }),
        "{status}"
    );
    assert_eq!(stage(&status, "code")["state"], "ended_short", "{status}");

    host.flume(&["retry", "fix--one", "--words", "notes.md"]);
    let status = host.until(&place, "code waiting on its second answer", |status| {
        waits_on(status, "And its text?")
    });
    assert_eq!(stage(&status, "code")["starts"], 1, "{status}");
    let questions: Vec<&Value> = status["ends"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|end| &end["of"])
        .collect();
    assert_eq!(questions.len(), 2, "two ends of their own: {status}");

    host.flume(&["tell", "fix--one", "Hello."]);
    let status = host.status(&place);
    let answers = &stage(&status, "code")["answers"];
    assert_eq!(
        answers
            .as_array()
            .into_iter()
            .flatten()
            .map(|answer| (&answer["question"], &answer["words"], &answer["act"]))
            .collect::<Vec<_>>(),
        [
            (&json!(WHICH), &json!("notes.md"), &json!("retry")),
            (&json!("And its text?"), &json!("Hello."), &json!("tell")),
        ],
        "{status}"
    );
    for answer in answers.as_array().into_iter().flatten() {
        assert!(
            answer["session"]["received_at"].is_u64(),
            "an answer not received in its session: {status}"
        );
    }
    until_landed(&host, &place);
    assert_eq!(host.on_remote("notes.md").as_deref(), Some("Hello.\n"));
    let prompts = host.prompts("code", 1);
    assert_eq!(prompts.len(), 3, "{prompts:?}");
    assert!(prompts[0].contains("write the notes"), "{prompts:?}");
    assert_eq!(
        prompts[1..],
        [asked(WHICH, "notes.md"), asked("And its text?", "Hello.")],
        "the answers the same session took"
    );
    assert_eq!(host.log("code", 2), "", "a second start of code");
}

/// The place's agent's pane, as status gives where it shows the agent.
fn agent_pane(status: &Value) -> String {
    status["clues"]["shown_at"]["agent"]
        .as_str()
        .and_then(|shown| shown.strip_prefix("herdr:"))
        .and_then(|shown| shown.split_once('@'))
        .map(|(pane, _)| pane.to_string())
        .unwrap_or_else(|| panic!("no pane shows the agent: {status}"))
}

/// Whether code's agent kept open was let go `by`, and ended.
fn let_go(status: &Value, by: &str) -> bool {
    let closed = &status["waiting_on"]["closed"];
    closed["by"] == by && closed["ended_at"].is_u64()
}

#[test]
fn s104_the_time_passing_lets_the_agent_go_and_a_late_answer_starts_a_fresh_one() {
    // S104; 42, 49, 53, 70, 198: the host keeps an asking agent open five
    // seconds and runs one agent at once. Code stops short, and a second
    // fix waits for room. Once five seconds have passed, a settle records
    // the agent let go by its time and ends it, its pane closed; status
    // says the place waits on an answer, its agent ended, and the second
    // fix's code starts in the room it freed. A tell is refused, naming
    // the retry. A retry with the answer starts code's second start, whose
    // prompt carries the question then the answer; it commits and the
    // place lands.
    let host = Walk::under_herdr("open-s104");
    host.configure("agents = 1\n", "[agents]\nkeep_open = \"5s\"\n");
    host.script("code", 1, json!([[needs(WHICH)], []]));
    host.script("code", 2, commits("notes.md\n"));
    let place = host.launch("fix", "one", &["--brief", "write the notes"]);
    let status = host.until(&place, "code waiting on its answer", |status| {
        waits_on(status, WHICH) && status["waiting_on"]["open_until"].is_u64()
    });
    assert_eq!(
        status["agent"]["settings"]["from"]["keep_open"],
        json!({ "layer": "host", "value": "5s" }),
        "{status}"
    );
    let pane = agent_pane(&status);
    let second = host.launch("fix", "two", &["--brief", "write the readme"]);
    assert_eq!(
        stage(&host.status(&second), "code")["starts"],
        0,
        "the second fix's code started with no room"
    );

    let status = watching(&host, &place, "its agent let go by its time", |status| {
        let_go(status, "time")
    });
    assert!(waits_on(&status, WHICH), "{status}");
    assert!(status["waiting_on"]["open_until"].is_null(), "{status}");
    assert!(
        status["agent"].is_null(),
        "an agent of one's stands: {status}"
    );
    assert!(
        !host
            .panes()
            .iter()
            .any(|listed| listed["pane_id"] == pane.as_str()),
        "the asking agent's pane {pane} stands: {:?}",
        host.panes()
    );
    watching(&host, &second, "the second fix's code started", |status| {
        stage(status, "code")["starts"] == 1
    });

    let refusal = refused(&host, &["tell", "fix--one", "notes.md"]);
    assert!(
        refusal.contains("no longer kept open")
            && refusal.contains(&format!("flume retry {place} --words")),
        "{refusal}"
    );
    // The second fix's code stops short too, its agent holding the room
    // until its own five seconds pass and a settle lets it go.
    host.flume(&["retry", "fix--one", "--words", "notes.md"]);
    watching(&host, &place, "landed and tidied", Value::is_null);
    assert_eq!(host.on_remote("notes.md").as_deref(), Some("notes.md\n"));
    let prompts = host.prompts("code", 2);
    assert_eq!(prompts.len(), 1, "{prompts:?}");
    assert!(
        prompts[0].contains(&asked(WHICH, "notes.md")),
        "the question then the answer: {prompts:?}"
    );
}

#[test]
fn s105_the_agents_session_ended_within_the_hour_and_a_retry_answers() {
    // S105; 42, 96, 198: code stops short and its script holds no later
    // run, so its session ends. The next settle records the agent let go,
    // ended. A retry with the answer starts a fresh agent with the
    // question and the answer, and the place lands.
    let host = Walk::under_herdr("open-s105-ended");
    host.script("code", 1, json!([[needs(WHICH)]]));
    host.script("code", 2, commits("notes.md\n"));
    let place = host.launch("fix", "one", &["--brief", "write the notes"]);
    let status = host.until(&place, "its agent let go, ended", |status| {
        waits_on(status, WHICH) && let_go(status, "agent_ended")
    });
    assert!(status["agent"].is_null(), "{status}");
    assert!(
        host.log("code", 1).contains("session ended"),
        "{}",
        host.log("code", 1)
    );
    host.flume(&["retry", "fix--one", "--words", "notes.md"]);
    until_landed(&host, &place);
    assert_eq!(host.on_remote("notes.md").as_deref(), Some("notes.md\n"));
    let prompts = host.prompts("code", 2);
    assert_eq!(prompts.len(), 1, "{prompts:?}");
    assert!(
        prompts[0].contains(&asked(WHICH, "notes.md")),
        "the question then the answer: {prompts:?}"
    );
}

#[test]
fn s105_the_agents_account_spent_within_the_hour_and_a_retry_answers() {
    // S105; 42, 146, 198: on a pool of two accounts, code's first start
    // stops short under the first, and the walkthrough writes that account
    // spent into the host's documents, as another agent reaching its limit
    // leaves it. The settle records the agent let go by its account and
    // ends it. A retry with the answer starts code on the pool's second
    // account with the question and the answer, and the place lands. Code's
    // second start waits on a signal before it commits, so the test reads
    // its agent's account while the agent is live: run straight through,
    // it delivers and the place lands between two of the test's reads.
    let host = Walk::under_herdr("open-s105-spent");
    let second = host.account_folder("account-2");
    host.configure_whole(&format!(
        "[host]\nmultiplexer = \"herdr\"\n\n[streams.default]\npool = \"stand-in\"\n\n\
         [pools.stand-in]\nstrategy = \"fill-first\"\naccounts = [{}, {}]\n",
        host.stand_in_account("stand-in-1", host.account()),
        host.stand_in_account("stand-in-2", &second),
    ));
    host.script("code", 1, json!([[needs(WHICH)], []]));
    let go = host.signal("go");
    let mut acts = vec![json!({ "wait_for": go })];
    acts.extend(commits("notes.md\n").as_array().cloned().expect("the acts"));
    host.script("code", 2, Value::Array(acts));
    let place = host.launch("fix", "one", &["--brief", "write the notes"]);
    let status = host.until(&place, "code waiting on its answer", |status| {
        waits_on(status, WHICH) && status["waiting_on"]["open_until"].is_u64()
    });
    assert_eq!(status["agent"]["account"]["name"], "stand-in-1", "{status}");

    let now = now();
    let key = HostKey::Account {
        pool: "stand-in".into(),
        account: "stand-in-1".into(),
    };
    let spent = HostDocument::Account(AccountDocument {
        spent: Some(AccountSpent {
            until: now + 3600,
            words: Some("You've hit your session limit".into()),
            recorded_at: now,
            unread: false,
        }),
        reading: None,
    });
    let store = StateDir::at(host.home().join("state/flume"));
    let at = store.read_host(&key).map(|read| read.version);
    assert!(
        matches!(
            store.write_host(&key, at, Some(&spent)),
            Written::Written { .. }
        ),
        "the account written spent"
    );
    let status = host.until(&place, "its agent let go by its account", |status| {
        let_go(status, "account_spent")
    });
    assert!(waits_on(&status, WHICH), "{status}");

    host.flume(&["retry", "fix--one", "--words", "notes.md"]);
    let status = host.until(&place, "code's second start", |status| {
        stage(status, "code")["starts"] == 2 && !status["agent"].is_null()
    });
    assert_eq!(status["agent"]["account"]["name"], "stand-in-2", "{status}");
    std::fs::write(&go, "").expect("the signal");
    until_landed(&host, &place);
    assert_eq!(host.on_remote("notes.md").as_deref(), Some("notes.md\n"));
    let prompts = host.prompts("code", 2);
    assert_eq!(prompts.len(), 1, "{prompts:?}");
    assert!(
        prompts[0].contains(&asked(WHICH, "notes.md")),
        "the question then the answer: {prompts:?}"
    );
}
