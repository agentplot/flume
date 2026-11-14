//! flume's telemetry through its built binary (216 to 219; the telemetry
//! spec): a scratch repository bound to `git-worktree`, its places' agents
//! printed for a person, and a collector of the test's own on its
//! loopback port that keeps each request's path, headers and bytes, so
//! the words a place holds are searched for in the bytes sent; one that
//! refuses each request, and one that accepts each connection and never
//! answers.

mod common;

use common::{Repository, Scratch};
use flume::profiles::store::state_dir::StateDir;
use flume_seams::{PlaceId, Store};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A phrase the place's brief holds, which no request carries.
const PHRASE: &str = "heron-blue lantern";

/// How the test's collector answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Answers {
    /// 200, empty, for every request.
    Takes,
    /// 503 for every request.
    Refuses,
    /// Nothing: the connection is held open and never read.
    Never,
}

/// One request the collector read: its path, its headers in lower case,
/// its bytes, and when it came.
#[derive(Clone, Debug)]
struct Request {
    path: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

/// A collector on the test's own loopback port.
struct Collector {
    port: u16,
    answers: Arc<Mutex<Answers>>,
    requests: Arc<Mutex<Vec<Request>>>,
    /// When the first connection came, if one did.
    first: Arc<Mutex<Option<Instant>>>,
}

impl Collector {
    fn new(answers: Answers) -> Collector {
        let listener = TcpListener::bind("127.0.0.1:0").expect("a loopback port");
        let port = listener.local_addr().expect("its address").port();
        let collector = Collector {
            port,
            answers: Arc::new(Mutex::new(answers)),
            requests: Arc::default(),
            first: Arc::default(),
        };
        let (answers, requests, first) = (
            collector.answers.clone(),
            collector.requests.clone(),
            collector.first.clone(),
        );
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                first.lock().unwrap().get_or_insert_with(Instant::now);
                let (answers, requests) = (answers.clone(), requests.clone());
                std::thread::spawn(move || serve(stream, &answers, &requests));
            }
        });
        collector
    }

    fn address(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    fn set(&self, answers: Answers) {
        *self.answers.lock().unwrap() = answers;
    }

    fn requests(&self) -> Vec<Request> {
        self.requests.lock().unwrap().clone()
    }

    fn paths(&self) -> Vec<String> {
        let mut paths: Vec<String> = self.requests().into_iter().map(|r| r.path).collect();
        paths.sort();
        paths.dedup();
        paths
    }

    fn clear(&self) {
        self.requests.lock().unwrap().clear();
        *self.first.lock().unwrap() = None;
    }

    /// Every byte every request carried.
    fn bytes(&self) -> Vec<u8> {
        self.requests().into_iter().flat_map(|r| r.body).collect()
    }
}

/// Serves one connection's requests as `answers` says.
fn serve(mut stream: TcpStream, answers: &Mutex<Answers>, requests: &Mutex<Vec<Request>>) {
    if *answers.lock().unwrap() == Answers::Never {
        std::thread::sleep(Duration::from_secs(30));
        return;
    }
    let mut reader = BufReader::new(stream.try_clone().expect("the stream"));
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap_or(0) == 0 {
            return;
        }
        let path = line
            .split_whitespace()
            .nth(1)
            .unwrap_or_default()
            .to_string();
        let mut headers = vec![];
        loop {
            let mut header = String::new();
            if reader.read_line(&mut header).unwrap_or(0) == 0 {
                return;
            }
            let header = header.trim_end();
            if header.is_empty() {
                break;
            }
            if let Some((name, value)) = header.split_once(':') {
                headers.push((name.trim().to_lowercase(), value.trim().to_string()));
            }
        }
        let length = headers
            .iter()
            .find(|(name, _)| name == "content-length")
            .and_then(|(_, value)| value.parse::<usize>().ok())
            .unwrap_or(0);
        let mut body = vec![0; length];
        if reader.read_exact(&mut body).is_err() {
            return;
        }
        requests.lock().unwrap().push(Request {
            path,
            headers,
            body,
        });
        let answer = match *answers.lock().unwrap() {
            Answers::Takes => "HTTP/1.1 200 OK\r\ncontent-type: application/x-protobuf\r\ncontent-length: 0\r\n\r\n",
            _ => "HTTP/1.1 503 Service Unavailable\r\ncontent-length: 0\r\n\r\n",
        };
        if stream.write_all(answer.as_bytes()).is_err() {
            return;
        }
    }
}

/// One host's scratch: flume's state, its configuration, a home, a
/// repository bound to `git-worktree`, and a kind `fix` of one stage whose
/// agent a person runs, its brief holding [`PHRASE`].
struct Host {
    scratch: Scratch,
    repository: Repository,
    kinds: String,
}

/// What a command printed, its exit status, when its answer came and
/// when it exited.
struct Ran {
    text: String,
    code: i32,
    answered: Instant,
    exited: Instant,
}

impl Host {
    fn new(label: &str) -> Host {
        let scratch = Scratch::new(label);
        for dir in ["state", "config/flume", "home", "kinds/prompts"] {
            std::fs::create_dir_all(scratch.path(dir)).expect("the host's directory");
        }
        std::fs::write(
            scratch.path("kinds/prompts/work.md"),
            "Do the work of {{name}}: {{brief}}. To stop short: {{needs}}\n",
        )
        .expect("the prompt");
        let kinds = scratch.path("kinds/kinds.toml");
        std::fs::write(
            &kinds,
            "[kinds.fix]\nversion = 1\nbranch = \"fix\"\nstarts_unready = true\nlands = false\n\n\
             [[kinds.fix.stages]]\nname = \"code\"\nprompt = \"prompts/work.md\"\n\
             deliverables = [\"commits\"]\nagents = [{ program = \"claude\" }]\n",
        )
        .expect("the declaration");
        let repository = Repository::new(label);
        repository.write(
            ".config/flume.toml",
            "[profiles]\nplace = \"git-worktree\"\n",
        );
        repository.commit("the binding");
        Host {
            scratch,
            repository,
            kinds: kinds.to_string_lossy().into_owned(),
        }
    }

    fn configure(&self, text: &str) {
        std::fs::write(self.scratch.path("config/flume/config.toml"), text)
            .expect("the configuration");
    }

    /// Sends every signal to `collector`, with `more` after.
    fn sending_to(&self, collector: &Collector, more: &str) {
        self.configure(&format!(
            "[telemetry]\nsend = true\ncollector = \"{}\"\n{more}",
            collector.address()
        ));
    }

    /// flume's binary run in the repository with `words` and `env` over
    /// nothing of the caller's flume, terminal or OpenTelemetry, read as
    /// it runs: when its answer came and when it exited.
    fn flume(&self, words: &[&str], env: &[(&str, &str)]) -> Ran {
        let mut command = Command::new(env!("CARGO_BIN_EXE_flume"));
        command.current_dir(&self.repository.dir);
        for (name, _) in std::env::vars() {
            if ["FLUME_", "HERDR_", "REX_", "OTEL_", "TRACEPARENT"]
                .iter()
                .any(|prefix| name.starts_with(prefix))
            {
                command.env_remove(name);
            }
        }
        command
            .env("XDG_STATE_HOME", self.scratch.path("state"))
            .env("XDG_CONFIG_HOME", self.scratch.path("config"))
            .env("XDG_DATA_HOME", self.scratch.path("home/.local/share"))
            .env("HOME", self.scratch.path("home"))
            .env("FLUME_RECIPIENT", "caller");
        for (name, value) in env {
            command.env(name, value);
        }
        let mut child = command
            .args(words)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("flume runs");
        let mut out = BufReader::new(child.stdout.take().expect("its output"));
        let mut text = String::new();
        out.read_line(&mut text).expect("its answer");
        let answered = Instant::now();
        let mut rest = String::new();
        let _ = out.read_to_string(&mut rest);
        text.push_str(&rest);
        let status = child.wait().expect("flume exits");
        Ran {
            text,
            code: status.code().unwrap_or(-1),
            answered,
            exited: Instant::now(),
        }
    }

    /// Launches `fix--one`, its stage held with words holding [`PHRASE`],
    /// so the place waits on its caller from its launch.
    fn launch(&self, env: &[(&str, &str)]) -> Ran {
        let brief = format!("mend {PHRASE} under /home/someone/notes");
        let hold = format!("code=until {PHRASE} is read");
        let ran = self.flume(
            &[
                "launch",
                "--kind",
                "fix",
                "--name",
                "one",
                "--kinds",
                &self.kinds,
                "--brief",
                &brief,
                "--hold",
                &hold,
                "--json",
            ],
            env,
        );
        assert_eq!(ran.code, 0, "{}", ran.text);
        ran
    }

    /// Retries the held stage with words holding [`PHRASE`] and a path:
    /// the hold and the place's wait on its caller end, and its stage
    /// starts.
    fn retry(&self, place: &str, env: &[(&str, &str)]) -> Ran {
        let words = format!("{PHRASE}, as /home/someone/notes says");
        let ran = self.flume(
            &[
                "retry", place, "--stage", "code", "--words", &words, "--json",
            ],
            env,
        );
        assert_eq!(ran.code, 0, "{}", ran.text);
        ran
    }

    /// The place `fix--one`'s location.
    fn place(&self) -> String {
        let ran = self.flume(&["status", "fix--one", "--json"], &[]);
        let json: serde_json::Value = serde_json::from_str(ran.text.trim()).expect("status");
        json["status"]["places"][0]["place"]
            .as_str()
            .expect("the place")
            .to_string()
    }

    /// The printed command's environment file of the place's agent.
    fn printed_env(&self, place: &str) -> String {
        let folder = StateDir::at(self.scratch.path("state/flume")).folder(&PlaceId::new(place));
        let dir = Path::new(&folder).join("printed");
        let file = std::fs::read_dir(&dir)
            .expect("the printed command's folder")
            .flatten()
            .map(|entry| entry.path())
            .find(|path: &PathBuf| path.extension().is_some_and(|ext| ext == "env"))
            .expect("the printed command's environment");
        std::fs::read_to_string(file).expect("the environment file")
    }
}

/// Whether `haystack` holds `needle`.
fn holds(haystack: &[u8], needle: &str) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle.as_bytes())
}

#[test]
fn a_collector_that_never_answers_holds_no_answer() {
    // The telemetry spec's "A collector that never answers": the command
    // writes its answer before any signal is sent, and exits within four
    // seconds of its answer, sending nothing more (216; core.13).
    let host = Host::new("telemetry-never");
    let collector = Collector::new(Answers::Never);
    host.sending_to(&collector, "");
    let ran = host.launch(&[]);
    assert!(ran.text.contains("\"launched\""), "{}", ran.text);
    let first = collector.first.lock().unwrap().expect("a signal was sent");
    assert!(first >= ran.answered, "a signal went before the answer");
    assert!(
        ran.exited.duration_since(ran.answered) < Duration::from_secs(4),
        "exited {:?} after its answer",
        ran.exited.duration_since(ran.answered)
    );
    let place = host.place();
    let retried = host.retry(&place, &[]);
    assert!(
        retried.exited.duration_since(retried.answered) < Duration::from_secs(4),
        "exited {:?} after its answer",
        retried.exited.duration_since(retried.answered)
    );
}

#[test]
fn nothing_is_sent_without_a_table() {
    // The telemetry spec's "No table, nothing sent": a host with no
    // [telemetry] sends nothing, whatever OpenTelemetry's variables name,
    // and its agents are given no OTEL_RESOURCE_ATTRIBUTES or TRACEPARENT
    // of flume's (216, 219).
    let host = Host::new("telemetry-none");
    let collector = Collector::new(Answers::Takes);
    let endpoint = collector.address();
    let env = [
        ("OTEL_EXPORTER_OTLP_ENDPOINT", endpoint.as_str()),
        ("OTEL_TRACES_EXPORTER", "otlp"),
    ];
    host.launch(&env);
    let place = host.place();
    host.retry(&place, &env);
    let printed = host.printed_env(&place);
    assert!(!printed.contains("OTEL_RESOURCE_ATTRIBUTES"), "{printed}");
    assert!(!printed.contains("TRACEPARENT"), "{printed}");
    assert!(collector.requests().is_empty(), "{:?}", collector.paths());
}

#[test]
fn a_callers_collector_is_never_flumes() {
    // The telemetry spec's "A caller's collector is never flume's": every
    // signal reaches the table's collector with no header of the caller's,
    // and the caller's receives nothing; the agent is given the place's
    // facts and its start's context; and no request holds the brief's,
    // the hold's or the retry's words, or a path (216, 218, 219; core.56).
    let host = Host::new("telemetry-caller");
    let ours = Collector::new(Answers::Takes);
    let theirs = Collector::new(Answers::Takes);
    host.sending_to(&ours, "");
    let endpoint = theirs.address();
    let env = [
        ("OTEL_EXPORTER_OTLP_ENDPOINT", endpoint.as_str()),
        ("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT", endpoint.as_str()),
        ("OTEL_EXPORTER_OTLP_HEADERS", "x-callers-secret=yes"),
    ];
    host.launch(&env);
    let place = host.place();
    host.retry(&place, &env);
    let printed = host.printed_env(&place);
    assert!(
        printed.contains("OTEL_RESOURCE_ATTRIBUTES") && printed.contains("flume.place=one"),
        "{printed}"
    );
    assert!(printed.contains("TRACEPARENT"), "{printed}");
    assert_eq!(
        ours.paths(),
        ["/v1/logs", "/v1/metrics", "/v1/traces"],
        "every signal at the table's collector"
    );
    for request in ours.requests() {
        assert!(
            !request
                .headers
                .iter()
                .any(|(name, _)| name == "x-callers-secret"),
            "{:?}",
            request.headers
        );
        assert!(
            request
                .headers
                .iter()
                .any(|(name, value)| name == "content-type" && value == "application/x-protobuf"),
            "OTLP over HTTP in protobuf: {:?}",
            request.headers
        );
    }
    assert!(theirs.requests().is_empty(), "{:?}", theirs.paths());
    let bytes = ours.bytes();
    assert!(!holds(&bytes, PHRASE), "the place's words were sent");
    assert!(!holds(&bytes, "/home/someone"), "a path was sent");
    assert!(
        !holds(&bytes, &host.scratch.root.to_string_lossy()),
        "the host's path was sent"
    );
    assert!(holds(&bytes, "flume.repository"), "the place's facts");
}

#[test]
fn otels_variables_turn_signals_off_and_never_on() {
    // The telemetry spec's "OpenTelemetry's variables turn signals off"
    // and "A variable never turns a signal on" (216).
    let host = Host::new("telemetry-off");
    let collector = Collector::new(Answers::Takes);
    host.sending_to(&collector, "");
    host.launch(&[("OTEL_TRACES_EXPORTER", "none")]);
    let place = host.place();
    collector.clear();
    host.retry(&place, &[("OTEL_TRACES_EXPORTER", "none")]);
    assert_eq!(collector.paths(), ["/v1/logs", "/v1/metrics"]);

    let host = Host::new("telemetry-disabled");
    host.sending_to(&collector, "");
    collector.clear();
    host.launch(&[("OTEL_SDK_DISABLED", "true")]);
    let place = host.place();
    host.retry(&place, &[("OTEL_SDK_DISABLED", "true")]);
    assert!(collector.requests().is_empty(), "{:?}", collector.paths());

    let host = Host::new("telemetry-never-on");
    host.sending_to(&collector, "traces = false\n");
    collector.clear();
    host.launch(&[("OTEL_TRACES_EXPORTER", "otlp")]);
    let place = host.place();
    host.retry(&place, &[("OTEL_TRACES_EXPORTER", "otlp")]);
    assert!(!collector.paths().contains(&"/v1/traces".to_string()));
    assert!(!collector.paths().is_empty());
}

/// The answers of a launch, a list, a retry and a list of every place on
/// `host`, each with its exit status, every path of the host's scratch and
/// every number taken out.
fn run_through(host: &Host) -> Vec<(i32, String)> {
    let root = host.scratch.root.to_string_lossy().into_owned();
    let repository = host.repository.dir.to_string_lossy().into_owned();
    let folded = |path: &str| path.trim_start_matches('/').replace('/', "%");
    let plain = |ran: Ran| {
        let text = ran
            .text
            .replace(&root, "<scratch>")
            .replace(&folded(&root), "<scratch>")
            .replace(&repository, "<repository>");
        (ran.code, without_numbers(&text))
    };
    let mut answers = vec![plain(host.launch(&[]))];
    let place = host.place();
    answers.push(plain(host.flume(&["list"], &[])));
    answers.push(plain(host.retry(&place, &[])));
    answers.push(plain(host.flume(&["list", "--all"], &[])));
    answers
}

/// `text` with each run of hex digits or digits replaced by `#`, so two
/// hosts' answers compare whatever their times, ids and labels.
fn without_numbers(text: &str) -> String {
    let mut out = String::new();
    let mut run = String::new();
    let flush = |run: &mut String, out: &mut String| {
        if run.chars().any(|c| c.is_ascii_digit()) {
            out.push('#');
        } else {
            out.push_str(run);
        }
        run.clear();
    };
    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            run.push(c);
        } else {
            flush(&mut run, &mut out);
            out.push(c);
        }
    }
    flush(&mut run, &mut out);
    out
}

#[test]
fn a_refusing_collector_changes_nothing() {
    // The telemetry spec's "A collector that refuses": every answer and
    // exit status is the same as with no [telemetry], and the next command
    // sends none of what was refused (216).
    let quiet = Host::new("telemetry-quiet");
    let refused = Host::new("telemetry-refused");
    let collector = Collector::new(Answers::Refuses);
    refused.sending_to(&collector, "");
    assert_eq!(run_through(&refused), run_through(&quiet));
    assert!(!collector.requests().is_empty(), "the collector was asked");
    collector.set(Answers::Takes);
    collector.clear();
    let listed = refused.flume(&["list", "--all"], &[]);
    assert_eq!(listed.code, 0);
    assert!(collector.requests().is_empty(), "nothing sent again");
}

#[test]
fn status_and_the_list_send_nothing() {
    // The telemetry spec's "Status sends nothing": status and the list,
    // asked of a place on a host that sends, send nothing; the next
    // command that ends its wait sends the wait's span (216; core.15,
    // core.52).
    let host = Host::new("telemetry-status");
    let collector = Collector::new(Answers::Takes);
    host.sending_to(&collector, "");
    host.launch(&[]);
    let place = host.place();
    collector.clear();
    for words in [
        &["status", "--json"][..],
        &["status", &place][..],
        &["list"][..],
        &["list", "--all", "--json"][..],
    ] {
        let ran = host.flume(words, &[]);
        assert_eq!(ran.code, 0, "{words:?}: {}", ran.text);
    }
    assert!(collector.requests().is_empty(), "{:?}", collector.paths());
    host.retry(&place, &[]);
    assert!(collector.paths().contains(&"/v1/traces".to_string()));
}
