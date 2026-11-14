//! The stand-in for Claude Code in the walkthroughs (design 18): the
//! program the printed command, or herdr's agent start, runs as `claude`,
//! first on the walkthrough's `PATH`, in the place. It acts as the pinned Claude Code, 2.1.296, acts
//! for flume's plugin: it writes a transcript in that release's shape, runs
//! the hooks the plugin's `hooks/hooks.json` names under `--plugin-dir`, as
//! that release runs them (the exec form with no shell, the hook's input on
//! standard input, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR` and
//! `CLAUDE_CODE_SESSION_ID` over the command's own environment less the
//! credentials that release keeps from every process it starts, a refusal
//! read from `permissionDecision` or exit 2, a prompt or a stop blocked by
//! `decision` or exit 2), and in place of a model acts the script `STAND_IN_SCRIPT`
//! names, one run's acts as a JSON array, or several runs as an array of
//! such arrays, each act a tool call:
//!
//! - `{"bash": "<line>"}`: Bash, the line run by `sh -c` in the place
//!   with the prompt in `STAND_IN_PROMPT`, unless PreToolUse refuses it;
//! - `{"write": {"path": "<file>", "text": "<text>"}}`: Write;
//! - `{"tick": {"path": "<tasks.md>", "count": <n>}}`: Edit, ticking the
//!   first `n` unticked tasks, every one where no count is given;
//! - `{"merge_named": {"<file>": "<text>"}}`: the base's revision the
//!   prompt's words name merged by Bash, each file contested written as
//!   given and the merge committed;
//! - `{"wait_for": "<file>"}`: nothing until the file exists, for up to
//!   two minutes, so a test reads status in the middle of a stage;
//! - `{"fail": {"error": "<error>", "words": "<words>"}}`: the turn ends
//!   there as that release ends a turn the API failed, the words written
//!   as its error message and StopFailure run with the error and the
//!   words in place of Stop, no later act made;
//! - `{"unset": "<name>"}`: the variable is gone from the session's
//!   environment before the session starts, wherever the act stands, as a
//!   pane brought back without it starts the program, so neither the
//!   session's hooks nor its transcript's folder see it;
//! - `{"begin_in": "<value>"}`: the session begins in the mode `value`,
//!   wherever the act stands, as a session whose account's configuration
//!   refuses the mode asked begins in another, so every hook input that
//!   names the mode names it as its `permission_mode`.
//!
//! Where the command passes `--append-system-prompt-file <file>` naming no
//! file, it prints `Error: Append system prompt file not found: <file>`
//! and exits 1 before any hook, as that release does.
//!
//! Where the command passes `--effort <word>`, it reads the word as that
//! release reads it, trimmed and in lower case, `med` read as `medium`
//! and `ultracode` kept as its own level, and prints `effort <level>`, the
//! level it began at; any other word outside that release's five, `low`,
//! `medium`, `high`, `xhigh` and `max`, it warns of on standard error in
//! that release's words and begins at its default, printing `effort
//! default` (197, C.4).
//!
//! Before its session starts it reads its settings as that release reads
//! them: the `settings.json` of its configuration home, then the place's
//! `.claude/settings.json` and `.claude/settings.local.json`, then the
//! settings `--settings <file-or-json>` passes, a JSON object as written
//! or the file it names, each source merged over those before it, objects
//! key by key, two lists joined with a repeated value kept once, and
//! anything else taken from the later source. It writes the settings so
//! merged to the file `STAND_IN_SETTINGS` names, where one is named, so a
//! walkthrough reads what the program read (195; design 14). Where the
//! settings `--settings` passes cannot be read, or are not JSON, it says so
//! and exits 1 before any hook.
//!
//! Where the command passes `--resume <session>`, it resumes the session
//! as that release does: it looks for `<home>/projects/<folder>/<session>.jsonl`
//! alone, `<home>` its configuration home, `CLAUDE_CONFIG_DIR` or else
//! `~/.claude`, and `<folder>` the working directory with every character
//! that is not a letter or a digit written `-`. Where it finds none it
//! prints `No conversation found with session ID: <session>` and exits 1
//! before any hook, no session begun. Otherwise its session is the one
//! resumed, its acts' records appended to that transcript and chained to
//! its last, and its SessionStart names `source` `resume`, where a
//! session not resumed names `startup`; every later hook's input names the
//! resumed session, UserPromptSubmit's among them.
//!
//! The inputs of UserPromptSubmit, PreToolUse, PostToolUse and Stop name
//! the mode the session runs in as that release names it: the
//! `--permission-mode` passed, `manual` read as its own `default`,
//! `default` where none was passed, or the `begin_in` act's. Those of
//! SessionStart, StopFailure and SessionEnd name none, as that release's
//! do not.
//!
//! While its session runs it names it in `<home>/sessions/<pid>.json`, as
//! that release does, the folder made readable by the host's user alone:
//! its `pid`, its `sessionId`, its `cwd`, when it started, and its
//! `procStart`, the words `ps -o lstart=` gives under `LC_ALL=C` and
//! `TZ=UTC`, here read from the host's process table, which a sandbox
//! lets it read where it refuses to run a setuid `ps`; it removes the
//! record as it exits, as that release does at its process's exit, and a
//! process killed leaves it.
//!
//! Where `STAND_IN_SCRIPTS` names a directory, the stand-in reads its
//! script from `<stage>-<n>.json` there in place of `STAND_IN_SCRIPT`'s,
//! `<stage>` its `FLUME_STAGE` and `<n>` the start's number, the last word
//! of its `FLUME_AGENT`, so each start acts its own (design 10); a start
//! the directory holds no script of acts none. It then keeps its log at
//! `<stage>-<n>.log` beside the script: each line it prints, each prompt
//! it took among them, as `took the prompt: <the prompt as JSON>`, so a
//! walkthrough reads what reached the session. A session read by
//! `STAND_IN_SCRIPT`, as one a person starts in a pane of their own, keeps
//! its log in the file `STAND_IN_LOG` names, where one is named.
//!
//! The context its SessionStart hooks give, each
//! `hookSpecificOutput.additionalContext`, it prints as `given at its
//! start: <the context as JSON>`, as that release hands it to the model
//! before the first prompt, so a walkthrough reads what a session was
//! given at its start (C.4; design 9).
//!
//! The first run's prompt is the command's argument, as the printed
//! command passes it; where the command passes none, as a terminal that
//! types the prompt into the agent starts it, it is the first prompt typed
//! into its terminal. A prompt is typed into its terminal as the pinned
//! herdr's `agent prompt` types it: its text, a newline within it typed as
//! one, then the enter that submits it, a ctrl+c clearing whatever was
//! typed before it, as that release clears its input; the terminal's input
//! is read so, a key at a time, unechoed, ctrl+c no signal, while the
//! session runs, and given back as it was at its end. Where another run
//! follows a turn, the session stays open once the turn ends, and the next
//! prompt typed into its terminal makes the next run's acts, as a person's
//! next prompt makes that release's next turn. Where no run follows, or
//! no prompt comes, its input ended, the session ends; under the printed
//! command no prompt is typed into it, so a script of one run acts as a
//! single prompt does (37, 38, 53, 54).
//!
//! Where it runs in a herdr pane, `HERDR_PANE_ID` and `HERDR_BIN_PATH` in
//! its environment, it reports its state to herdr as herdr's reading of
//! that release's screen gives it: `idle` once its session started and
//! waits for its prompt, `working` from the moment it takes a prompt until
//! the turn ends, and `idle` once it has, by `pane report-agent <pane>
//! --source flume-stand-in --agent claude --state <state>` run with the
//! pane's own `HERDR_*` left out and bounded at five seconds, in the
//! states the pinned herdr, 0.9.5, names (core.29).
//!
//! Where it runs in a Rex block, `REX_BLOCK` in its environment, it reports
//! its state to Rex in place of herdr, as Claude Code 2.1.322 reports its
//! own: OSC 7501 written to its terminal, `app=claude-code`, `idle` once its
//! session started and waits for its prompt, `working` from the moment it
//! takes a prompt, and `done` at its turn's end, kept until the next, in the
//! states the pinned Rex, 0.1.0-preview.14, names (C.5; core.29).
//!
//! The prompt is submitted as given, or, where `STAND_IN_PASTE_TAG` names
//! a four-hex-digit id, inside the paste tags that release wraps a pasted
//! prompt in where the account's features turn them on.
//!
//! A prompt UserPromptSubmit blocked is held back, as that release holds
//! it back: no act is made and no turn taken, and the session ends.
//! Otherwise the turn ends: Stop, once more with `stop_hook_active` where
//! the hook blocked it, or StopFailure where a `fail` act ended it, and,
//! where no run follows or a `fail` act ended it, the session ends, as a
//! person leaves the printed command's session, with SessionEnd. Each act
//! is printed on standard output as one line; the exit status is 1 where
//! an act failed that no hook refused. It never reaches a model, an
//! account or the network.

use flume::host::process::listed;
use flume::profiles::agent::claude::live::{proc_start, SESSIONS};
use flume::profiles::agent::claude::withheld::WITHHELD;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// The release whose shape the transcript takes.
const VERSION: &str = "2.1.296";

/// The events whose input that release builds with the session's mode.
const MODED: [&str; 4] = ["UserPromptSubmit", "PreToolUse", "PostToolUse", "Stop"];

/// The efforts that release's `--effort` names, in its order.
const EFFORTS: [&str; 5] = ["low", "medium", "high", "xhigh", "max"];

/// The level that release begins at for `--effort <word>`, as its `Wit`
/// reads the word: trimmed and in lower case, `med` read as `medium` and
/// `ultracode` kept as its own level; none for any other word outside its
/// five, of which it warns on standard error, in its words, and begins at
/// its default.
fn began_at(word: &str) -> Option<String> {
    let read = word.trim().to_lowercase();
    let read = if read == "med" { "medium".into() } else { read };
    if EFFORTS.contains(&read.as_str()) || read == "ultracode" {
        return Some(read);
    }
    eprintln!(
        "Warning: Unknown --effort value '{word}' \u{2014} ignoring it and using the default effort. Valid values: {}.",
        EFFORTS.join(", ")
    );
    None
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut plugin = None;
    let mut prompt = None;
    let mut mode = None;
    let mut resume = None;
    let mut flag = None;
    let mut words = args.iter();
    while let Some(word) = words.next() {
        match word.as_str() {
            "--plugin-dir" => plugin = words.next().map(PathBuf::from),
            "--permission-mode" => mode = words.next().cloned(),
            "--resume" => resume = words.next().cloned(),
            "--settings" => flag = words.next().cloned(),
            "--effort" => {
                let level = words.next().and_then(|word| began_at(word));
                say(&format!("effort {}", level.as_deref().unwrap_or("default")));
            }
            "--append-system-prompt-file" => {
                let file = words.next().cloned().unwrap_or_default();
                if !Path::new(&file).is_file() {
                    eprintln!("Error: Append system prompt file not found: {file}");
                    std::process::exit(1);
                }
            }
            "--model" | "--append-system-prompt" => {
                words.next();
            }
            flag if flag.starts_with("--") => {}
            given => prompt = Some(given.to_string()),
        }
    }
    let runs = runs(&script());
    let mut mode = match mode.as_deref() {
        None | Some("manual") => "default".to_string(),
        Some(passed) => passed.to_string(),
    };
    for act in runs.iter().flatten() {
        if let Some(name) = act["unset"].as_str() {
            std::env::remove_var(name);
        }
        if let Some(began) = act["begin_in"].as_str() {
            mode = began.to_string();
        }
    }
    let cwd = std::env::current_dir().expect("the place");
    match settings(&cwd, flag.as_deref()) {
        Ok(read) => {
            if let Some(file) = std::env::var_os("STAND_IN_SETTINGS").filter(|f| !f.is_empty()) {
                std::fs::write(&file, format!("{read}\n")).expect("the settings read");
            }
        }
        Err(words) => {
            eprintln!("{words}");
            std::process::exit(1);
        }
    }
    let resumed = resume.map(|session| {
        let transcript = transcript_of(&cwd, &session);
        if !transcript.is_file() {
            eprintln!("No conversation found with session ID: {session}");
            std::process::exit(1);
        }
        (session, transcript)
    });
    let mut session = Session::new(cwd, plugin, mode, resumed);
    let record = session.name_running();
    let typed = Typed::taken();
    let failed = session.run(prompt, &runs, typed.as_ref());
    drop(typed);
    if let Some(record) = record {
        let _ = std::fs::remove_file(record);
    }
    std::process::exit(i32::from(failed));
}

/// The script: `<stage>-<n>.json` in the directory `STAND_IN_SCRIPTS`
/// names, by the start's `FLUME_STAGE` and the number its `FLUME_AGENT`
/// ends in, its log kept beside it as `<stage>-<n>.log`; else the file
/// `STAND_IN_SCRIPT` names. No acts where neither gives a script.
fn script() -> Value {
    let file = match std::env::var_os("STAND_IN_SCRIPTS")
        .map(PathBuf::from)
        .filter(|dir| dir.is_dir())
    {
        Some(dir) => {
            let stage = std::env::var("FLUME_STAGE").unwrap_or_default();
            let agent = std::env::var("FLUME_AGENT").unwrap_or_default();
            let start = agent.rsplit(':').next().unwrap_or_default();
            let _ = LOG.set(dir.join(format!("{stage}-{start}.log")));
            Some(dir.join(format!("{stage}-{start}.json")))
        }
        None => {
            if let Some(log) = std::env::var_os("STAND_IN_LOG").filter(|log| !log.is_empty()) {
                let _ = LOG.set(PathBuf::from(log));
            }
            std::env::var_os("STAND_IN_SCRIPT").map(PathBuf::from)
        }
    };
    file.and_then(|file| std::fs::read_to_string(file).ok())
        .and_then(|text| serde_json::from_str::<Value>(&text).ok())
        .unwrap_or(json!([]))
}

/// The script's runs, each a list of acts: each array of an array of
/// arrays, or the array of acts whole as the one run.
fn runs(script: &Value) -> Vec<Vec<Value>> {
    let items = script.as_array().cloned().unwrap_or_default();
    if items.first().is_some_and(Value::is_array) {
        items
            .into_iter()
            .map(|run| run.as_array().cloned().unwrap_or_default())
            .collect()
    } else {
        vec![items]
    }
}

/// The file the stand-in keeps its log in, where its script was read by
/// its start.
static LOG: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

/// The terminal the session reads its typed prompts from: its standard
/// input, where that is a terminal, read a key at a time, unechoed, ctrl+c
/// a key rather than a signal and enter its own key, its settings given
/// back when the session ends.
struct Typed {
    saved: libc::termios,
}

impl Typed {
    /// The standard input taken to read typed prompts from, none where it
    /// is no terminal.
    fn taken() -> Option<Typed> {
        // SAFETY: an all-zero `termios` is a valid value for `tcgetattr`
        // to fill.
        let mut saved: libc::termios = unsafe { std::mem::zeroed() };
        // SAFETY: standard input's descriptor, and a `termios` to fill.
        if unsafe { libc::isatty(0) } != 1 || unsafe { libc::tcgetattr(0, &mut saved) } != 0 {
            return None;
        }
        let mut keys = saved;
        keys.c_lflag &= !(libc::ICANON | libc::ECHO | libc::ISIG | libc::IEXTEN);
        keys.c_iflag &= !(libc::ICRNL | libc::INLCR | libc::IGNCR | libc::IXON);
        keys.c_cc[libc::VMIN] = 1;
        keys.c_cc[libc::VTIME] = 0;
        // SAFETY: standard input's descriptor, and the settings to take.
        unsafe { libc::tcsetattr(0, libc::TCSANOW, &keys) };
        Some(Typed { saved })
    }

    /// The next prompt typed: the keys up to the enter that submits it,
    /// those before a ctrl+c cleared, an enter with nothing typed taking
    /// nothing; none once the input ends.
    fn next(&self) -> Option<String> {
        let mut line: Vec<u8> = vec![];
        let mut stdin = std::io::stdin().lock();
        let mut key = [0u8; 1];
        loop {
            match stdin.read(&mut key) {
                Ok(0) => return None,
                Ok(_) => match key[0] {
                    b'\r' if line.is_empty() => {}
                    b'\r' => return Some(String::from_utf8_lossy(&line).into_owned()),
                    0x03 => line.clear(),
                    byte => line.push(byte),
                },
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                Err(_) => return None,
            }
        }
    }
}

impl Drop for Typed {
    fn drop(&mut self) {
        // SAFETY: standard input's descriptor, and the settings it had.
        unsafe { libc::tcsetattr(0, libc::TCSANOW, &self.saved) };
    }
}

/// Where the stand-in reports its state: a herdr pane, or a Rex block, by
/// OSC 7501 to its terminal.
enum Reporter {
    Herdr(Pane),
    Rex,
}

impl Reporter {
    /// The Rex block it runs in, where `REX_BLOCK` names one, else the
    /// herdr pane; none outside both.
    fn here() -> Option<Reporter> {
        if std::env::var("REX_BLOCK").is_ok_and(|block| !block.is_empty()) {
            return Some(Reporter::Rex);
        }
        Pane::here().map(Reporter::Herdr)
    }

    /// `state` reported, in herdr's words or Rex's.
    fn report(&self, state: &str) {
        match self {
            Reporter::Herdr(pane) => pane.report(state),
            Reporter::Rex => {
                let mut out = std::io::stdout().lock();
                let _ = write!(out, "\x1b]7501;state={state}:app=claude-code\x1b\\");
                let _ = out.flush();
            }
        }
    }

    /// The turn's end reported: herdr reads the release's screen idle, and
    /// the release reports `done` to Rex, kept until the next turn.
    fn ended(&self) {
        match self {
            Reporter::Herdr(pane) => pane.report("idle"),
            Reporter::Rex => self.report("done"),
        }
    }
}

/// The herdr pane the stand-in runs in, by `HERDR_PANE_ID`, and the herdr
/// that runs it, by `HERDR_BIN_PATH`; none outside a herdr pane.
struct Pane {
    id: String,
    herdr: PathBuf,
}

impl Pane {
    fn here() -> Option<Pane> {
        let id = std::env::var("HERDR_PANE_ID")
            .ok()
            .filter(|id| !id.is_empty())?;
        let herdr = std::env::var_os("HERDR_BIN_PATH").filter(|bin| !bin.is_empty())?;
        Some(Pane {
            id,
            herdr: herdr.into(),
        })
    }

    /// The state reported to herdr as its reading of the pinned release's
    /// screen gives it, with the pane's own `HERDR_*` left out, bounded at
    /// five seconds.
    fn report(&self, state: &str) {
        let spawned = Command::new(&self.herdr)
            .args(["pane", "report-agent", &self.id])
            .args(["--source", "flume-stand-in", "--agent", "claude"])
            .args(["--state", state])
            .env_remove("HERDR_PANE_ID")
            .env_remove("HERDR_TAB_ID")
            .env_remove("HERDR_WORKSPACE_ID")
            .env_remove("HERDR_ENV")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        let Ok(mut child) = spawned else {
            say(&format!("herdr could not be told {state}"));
            return;
        };
        let began = Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(_)) => return,
                Ok(None) if began.elapsed() < Duration::from_secs(5) => {
                    std::thread::sleep(Duration::from_millis(20));
                }
                _ => {
                    let _ = child.kill();
                    let _ = child.wait();
                    say(&format!("herdr was not told {state} within five seconds"));
                    return;
                }
            }
        }
    }
}

/// The prompt as the session submits it: as given, or where
/// `STAND_IN_PASTE_TAG` names a tag's id, inside the tags Claude Code
/// 2.1.296 wraps a pasted prompt in where the account's features turn them
/// on, newlines around them.
fn submitted(prompt: &str) -> String {
    match std::env::var("STAND_IN_PASTE_TAG") {
        Ok(id) if !id.is_empty() => {
            format!("\n\n<pasted_content id=\"{id}\">\n{prompt}\n</pasted_content id=\"{id}\">\n")
        }
        _ => prompt.to_string(),
    }
}

/// Where that release keeps the transcript of the session `session` run
/// in `cwd`: `<home>/projects/<folder>/<session>.jsonl`, `<home>` its
/// configuration home, `CLAUDE_CONFIG_DIR` or else `~/.claude`, `<folder>`
/// the working directory with every character that is not a letter or a
/// digit written `-`.
fn transcript_of(cwd: &Path, session: &str) -> PathBuf {
    let project: String = cwd
        .to_string_lossy()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    config_home()
        .join("projects")
        .join(project)
        .join(format!("{session}.jsonl"))
}

/// That release's configuration home: `CLAUDE_CONFIG_DIR`, or else
/// `~/.claude`.
fn config_home() -> PathBuf {
    std::env::var_os("CLAUDE_CONFIG_DIR")
        .filter(|dir| !dir.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".claude")
        })
}

/// The settings that release reads in `cwd`, `flag` the value of its
/// `--settings`: the user's `settings.json` in its configuration home, the
/// project's `.claude/settings.json` and its `.claude/settings.local.json`,
/// then the flag's, a JSON object as written or the file it names, each
/// merged over those before it (195; design 14). A source that is not there
/// gives nothing; the flag's that cannot be read or is not JSON gives the
/// words the session ends with.
fn settings(cwd: &Path, flag: Option<&str>) -> Result<Value, String> {
    let mut read = json!({});
    for file in [
        config_home().join("settings.json"),
        cwd.join(".claude/settings.json"),
        cwd.join(".claude/settings.local.json"),
    ] {
        if let Some(source) = std::fs::read_to_string(&file)
            .ok()
            .and_then(|text| serde_json::from_str::<Value>(&text).ok())
        {
            merge(&mut read, source);
        }
    }
    if let Some(given) = flag {
        let text = if given.trim_start().starts_with('{') {
            given.to_string()
        } else {
            std::fs::read_to_string(given)
                .map_err(|error| format!("Error: --settings {given} cannot be read: {error}"))?
        };
        let source: Value = serde_json::from_str(&text)
            .map_err(|error| format!("Error: --settings {given} is not JSON: {error}"))?;
        merge(&mut read, source);
    }
    Ok(read)
}

/// `over` merged into `under` as that release merges one source of its
/// settings over those before it: two objects key by key, two lists joined,
/// a string, a number or a boolean the two give kept once, as its `Set`
/// keeps one; anything else taken from `over`.
fn merge(under: &mut Value, over: Value) {
    match (under, over) {
        (Value::Object(under), Value::Object(over)) => {
            for (key, value) in over {
                match under.get_mut(&key) {
                    Some(beneath) => merge(beneath, value),
                    None => {
                        under.insert(key, value);
                    }
                }
            }
        }
        (Value::Array(under), Value::Array(over)) => {
            let joined: Vec<Value> = under.drain(..).chain(over).collect();
            for item in joined {
                let kept_once = !matches!(item, Value::Object(_) | Value::Array(_));
                if !(kept_once && under.contains(&item)) {
                    under.push(item);
                }
            }
        }
        (under, over) => *under = over,
    }
}

/// One session of the stand-in.
struct Session {
    cwd: PathBuf,
    plugin: Option<PathBuf>,
    id: String,
    transcript: PathBuf,
    hooks: Value,
    prompt: String,
    parent: Option<String>,
    calls: u32,
    /// The mode the session runs in, as every hook's input names it.
    mode: String,
    /// Whether the session is one resumed.
    resumed: bool,
    /// The records written, which number each record's id and its
    /// message's.
    records: u32,
    /// The herdr pane or the Rex block it runs in, if any, to which it
    /// reports its state.
    reporter: Option<Reporter>,
}

impl Session {
    /// A new session in `cwd`, or the session `resumed` names and the
    /// transcript it was found in.
    fn new(
        cwd: PathBuf,
        plugin: Option<PathBuf>,
        mode: String,
        resumed: Option<(String, PathBuf)>,
    ) -> Session {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_nanos());
        let new = format!(
            "{:08x}-{:04x}-4{:03x}-8{:03x}-{:012x}",
            (nanos >> 32) as u32,
            std::process::id() & 0xffff,
            (nanos >> 20) as u16 & 0xfff,
            (nanos >> 8) as u16 & 0xfff,
            nanos as u64 & 0xffff_ffff_ffff
        );
        let is_resumed = resumed.is_some();
        let (id, transcript, parent, calls) = match resumed {
            Some((id, transcript)) => {
                // Chained to the transcript's last record, its ids past
                // every one the transcript holds.
                let text = std::fs::read_to_string(&transcript).unwrap_or_default();
                let parent = text.lines().rev().find_map(|line| {
                    serde_json::from_str::<Value>(line)
                        .ok()
                        .and_then(|record| record["uuid"].as_str().map(str::to_string))
                });
                let calls = u32::try_from(text.lines().count()).unwrap_or(u32::MAX);
                (id, transcript, parent, calls)
            }
            None => {
                let transcript = transcript_of(&cwd, &new);
                (new, transcript, None, 0)
            }
        };
        let hooks = plugin
            .as_ref()
            .and_then(|dir| std::fs::read_to_string(dir.join("hooks/hooks.json")).ok())
            .and_then(|text| serde_json::from_str::<Value>(&text).ok())
            .map_or(Value::Null, |file| file["hooks"].clone());
        Session {
            cwd,
            plugin,
            id,
            transcript,
            hooks,
            prompt: String::new(),
            parent,
            calls,
            mode,
            resumed: is_resumed,
            records: calls,
            reporter: Reporter::here(),
        }
    }

    /// The session named in `<home>/sessions/<pid>.json` while it runs, as
    /// that release names it, the folder made readable by the host's user
    /// alone; the record's path, none where it could not be written.
    fn name_running(&self) -> Option<PathBuf> {
        use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
        let folder = config_home().join(SESSIONS);
        std::fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(&folder)
            .ok()?;
        std::fs::set_permissions(&folder, std::fs::Permissions::from_mode(0o700)).ok()?;
        let pid = std::process::id();
        let started = listed()
            .into_iter()
            .flatten()
            .find(|process| process.pid == pid)
            .and_then(|process| proc_start(process.started));
        let mut record = json!({
            "pid": pid,
            "sessionId": self.id,
            "cwd": self.cwd,
            "startedAt": SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |since| since.as_millis()),
            "version": VERSION,
            "kind": "interactive",
            "entrypoint": "cli",
        });
        if let Some(started) = started {
            record["procStart"] = json!(started);
        }
        let path = folder.join(format!("{pid}.json"));
        std::fs::write(&path, record.to_string()).ok()?;
        Some(path)
    }

    /// The session: its start, then each run's turn, its prompt the
    /// command's `prompt` for the first where it passes one and otherwise
    /// the next typed into `typed`, then the end. Whether an act failed.
    fn run(&mut self, prompt: Option<String>, runs: &[Vec<Value>], typed: Option<&Typed>) -> bool {
        if let Some(dir) = self.transcript.parent() {
            std::fs::create_dir_all(dir).expect("the transcript's folder");
        }
        let source = if self.resumed { "resume" } else { "startup" };
        say(&format!(
            "session {} in {}{}",
            self.id,
            self.cwd.display(),
            if self.resumed { ", resumed" } else { "" }
        ));
        let started = self.hook("SessionStart", json!({ "source": source }), None);
        for context in &started.context {
            say(&format!("given at its start: {}", json!(context)));
        }
        let mut failed = false;
        let mut given = prompt;
        for (at, acts) in runs.iter().enumerate() {
            let prompt = match given.take() {
                Some(prompt) => Some(prompt),
                None => {
                    if at == 0 {
                        self.report("idle");
                    }
                    typed.and_then(Typed::next)
                }
            };
            if prompt.is_none() && at > 0 {
                break;
            }
            match self.turn(prompt.as_deref(), acts) {
                Turn::Ended(acted) => failed |= !acted,
                Turn::Over => return failed,
            }
        }
        self.hook("SessionEnd", json!({ "reason": "prompt_input_exit" }), None);
        say("session ended");
        failed
    }

    /// The state reported to herdr or Rex where the stand-in runs in a
    /// herdr pane or a Rex block.
    fn report(&self, state: &str) {
        if let Some(reporter) = &self.reporter {
            reporter.report(state);
        }
    }

    /// One turn: `prompt` submitted, the run's `acts` made and the turn's
    /// stop; with no prompt, the stop alone, as a session given none.
    fn turn(&mut self, prompt: Option<&str>, acts: &[Value]) -> Turn {
        let mut acted = true;
        if let Some(prompt) = prompt {
            self.report("working");
            say(&format!("took the prompt: {}", json!(prompt)));
            self.prompt = prompt.to_string();
            let submitted = submitted(prompt);
            self.queued(&submitted);
            let answer = self.hook("UserPromptSubmit", json!({ "prompt": submitted }), None);
            if let Some(reason) = answer.blocked {
                say(&format!("prompt blocked: {reason}"));
                self.hook("SessionEnd", json!({ "reason": "prompt_input_exit" }), None);
                say("session ended");
                return Turn::Over;
            }
            self.user(json!(submitted));
            for act in acts {
                if let Some(failure) = act.get("fail") {
                    self.fail(failure);
                    return Turn::Over;
                }
                if !self.act(act) {
                    acted = false;
                }
            }
        }
        self.assistant(json!([{ "type": "text", "text": "Done." }]), "end_turn");
        let mut active = false;
        loop {
            let answer = self.hook(
                "Stop",
                json!({
                    "stop_hook_active": active,
                    "last_assistant_message": "Done.",
                    "background_tasks": [],
                }),
                None,
            );
            match answer.blocked {
                Some(reason) if !active => {
                    say(&format!("stop blocked: {reason}"));
                    self.user(json!(format!("Stop hook feedback:\n{reason}")));
                    active = true;
                }
                _ => break,
            }
        }
        say("the turn ended");
        if let Some(reporter) = &self.reporter {
            reporter.ended();
        }
        Turn::Ended(acted)
    }

    /// The turn ended by the API's failure, as 2.1.296 ends one: its words
    /// written as the synthetic error message, StopFailure run with the
    /// error and the words in place of Stop, and the session ended.
    fn fail(&mut self, failure: &Value) {
        let error = failure["error"].as_str().unwrap_or_default();
        let words = failure["words"].as_str().unwrap_or_default();
        let mut record = self.record(
            "assistant",
            json!({
                "id": format!("msg_stand_in_{}", self.records + 1),
                "model": "<synthetic>",
                "role": "assistant",
                "type": "message",
                "stop_reason": "stop_sequence",
                "content": [{ "type": "text", "text": words }],
            }),
        );
        record["isApiErrorMessage"] = json!(true);
        record["error"] = json!(error);
        self.line(record);
        say(&format!("the turn failed: {words} ({error})"));
        self.hook(
            "StopFailure",
            json!({ "error": error, "last_assistant_message": words }),
            None,
        );
        self.hook("SessionEnd", json!({ "reason": "prompt_input_exit" }), None);
        say("session ended");
    }

    /// One act of the script, as a tool call. Whether it did not fail.
    fn act(&mut self, act: &Value) -> bool {
        if let Some(line) = act["bash"].as_str() {
            return self.bash(line);
        }
        if let Some(write) = act.get("write") {
            let path = write["path"].as_str().unwrap_or_default();
            let text = write["text"].as_str().unwrap_or_default();
            let input = json!({ "file_path": self.cwd.join(path), "content": text });
            return self.tool("Write", input, |session| {
                let file = session.cwd.join(path);
                if let Some(dir) = file.parent() {
                    std::fs::create_dir_all(dir).map_err(|error| error.to_string())?;
                }
                std::fs::write(&file, text).map_err(|error| error.to_string())?;
                Ok(format!("wrote {path}"))
            });
        }
        if let Some(tick) = act.get("tick") {
            let path = tick["path"].as_str().unwrap_or_default().to_string();
            let count = tick["count"].as_u64().map_or(usize::MAX, |n| n as usize);
            let input = json!({ "file_path": self.cwd.join(&path), "old_string": "- [ ]", "new_string": "- [x]" });
            return self.tool("Edit", input, |session| {
                let file = session.cwd.join(&path);
                let text = std::fs::read_to_string(&file).map_err(|error| error.to_string())?;
                let ticked = text.replacen("- [ ]", "- [x]", count);
                std::fs::write(&file, ticked).map_err(|error| error.to_string())?;
                Ok(format!("ticked {path}"))
            });
        }
        if let Some(files) = act.get("merge_named") {
            return self.merge_named(files);
        }
        if act.get("unset").is_some() || act.get("begin_in").is_some() {
            // Made before the session started.
            return true;
        }
        if let Some(file) = act["wait_for"].as_str() {
            let began = Instant::now();
            while !Path::new(file).exists() {
                if began.elapsed() > Duration::from_secs(120) {
                    say(&format!("{file} never came"));
                    return false;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            return true;
        }
        say(&format!("an act the stand-in does not know: {act}"));
        false
    }

    /// The base's revision the prompt's words name merged, its contested
    /// files written as `files` gives them, and the merge committed.
    fn merge_named(&mut self, files: &Value) -> bool {
        let named = self
            .prompt
            .split(|c: char| !c.is_ascii_hexdigit())
            .filter(|word| word.len() == 40)
            .find(|word| {
                Command::new("git")
                    .args(["cat-file", "-e", &format!("{word}^{{commit}}")])
                    .current_dir(&self.cwd)
                    .status()
                    .is_ok_and(|status| status.success())
            })
            .map(str::to_string);
        let Some(revision) = named else {
            say("the prompt names no revision to merge");
            return false;
        };
        // The merge stops on its conflicts, which are what the act resolves.
        self.bash(&format!("git merge --no-edit {revision} || true"));
        let mut done = true;
        for (path, text) in files.as_object().into_iter().flatten() {
            done &= self.act(&json!({ "write": { "path": path, "text": text } }));
        }
        done && self.bash("git add -A && git commit --quiet --no-edit")
    }

    /// The Bash tool: the line run in the place unless PreToolUse refuses
    /// it.
    fn bash(&mut self, line: &str) -> bool {
        let prompt = self.prompt.clone();
        self.tool("Bash", json!({ "command": line }), move |session| {
            let output = Command::new("sh")
                .args(["-c", line])
                .current_dir(&session.cwd)
                .env("STAND_IN_PROMPT", &prompt)
                .stdin(Stdio::null())
                .output()
                .map_err(|error| error.to_string())?;
            let text = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            if output.status.success() {
                Ok(text)
            } else {
                Err(format!("{line}: {text}"))
            }
        })
    }

    /// One tool call: written as the assistant's, PreToolUse asked, the
    /// call made unless refused, its result written and PostToolUse run.
    fn tool(
        &mut self,
        name: &str,
        input: Value,
        call: impl FnOnce(&Session) -> Result<String, String>,
    ) -> bool {
        self.calls += 1;
        let id = format!("toolu_stand_in_{:04}", self.calls);
        self.assistant(
            json!([{ "type": "tool_use", "id": id, "name": name, "input": input }]),
            "tool_use",
        );
        let ask = json!({ "tool_name": name, "tool_input": input, "tool_use_id": id });
        let answer = self.hook("PreToolUse", ask, Some(name));
        if let Some(reason) = answer.denied {
            say(&format!("{name} refused: {reason}"));
            self.result(&id, &reason, true);
            return true;
        }
        let made = call(self);
        let (text, error) = match &made {
            Ok(text) => (text.clone(), false),
            Err(text) => (text.clone(), true),
        };
        say(&format!(
            "{name} {}: {}",
            if error { "failed" } else { "done" },
            input["command"]
                .as_str()
                .or(input["file_path"].as_str())
                .unwrap_or_default()
        ));
        if error {
            say(&text);
        }
        self.result(&id, &text, error);
        self.hook(
            "PostToolUse",
            json!({
                "tool_name": name,
                "tool_input": input,
                "tool_response": { "output": text },
                "tool_use_id": id,
            }),
            Some(name),
        );
        made.is_ok()
    }

    /// Runs the plugin's hooks for `event`, those whose matcher takes
    /// `tool`, with `input` over the fields every hook's input carries.
    fn hook(&self, event: &str, mut input: Value, tool: Option<&str>) -> Answer {
        let mut common = json!({
            "session_id": self.id,
            "transcript_path": self.transcript,
            "cwd": self.cwd,
            "hook_event_name": event,
        });
        if MODED.contains(&event) {
            common["permission_mode"] = json!(self.mode);
        }
        for (key, value) in common.as_object().into_iter().flatten() {
            input
                .as_object_mut()
                .expect("an object")
                .entry(key)
                .or_insert(value.clone());
        }
        let mut answer = Answer::default();
        for group in self.hooks[event].as_array().into_iter().flatten() {
            if !matches(group["matcher"].as_str(), tool) {
                continue;
            }
            for hook in group["hooks"].as_array().into_iter().flatten() {
                if hook["type"] != "command" {
                    continue;
                }
                let ran = self.command(hook, &input);
                let output: Value = serde_json::from_str(ran.out.trim()).unwrap_or(Value::Null);
                let specific = &output["hookSpecificOutput"];
                if ran.code == 2 {
                    answer.denied.get_or_insert(ran.err.clone());
                    answer.blocked.get_or_insert(ran.err.clone());
                }
                if specific["permissionDecision"] == "deny" {
                    answer.denied.get_or_insert(
                        specific["permissionDecisionReason"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string(),
                    );
                }
                if let Some(context) = specific["additionalContext"].as_str() {
                    answer.context.push(context.to_string());
                }
                if output["decision"] == "block" {
                    answer
                        .blocked
                        .get_or_insert(output["reason"].as_str().unwrap_or_default().to_string());
                }
            }
        }
        if event != "PreToolUse" {
            answer.denied = None;
        }
        if !matches!(event, "Stop" | "UserPromptSubmit") {
            answer.blocked = None;
        }
        answer
    }

    /// One command hook, its exec form with no shell, or its line by
    /// `sh -c` where it names no arguments, bounded by its timeout.
    fn command(&self, hook: &Value, input: &Value) -> Ran {
        let program = hook["command"].as_str().unwrap_or_default();
        let mut command = match hook["args"].as_array() {
            Some(args) => {
                let mut command = Command::new(program);
                command.args(args.iter().filter_map(Value::as_str));
                command
            }
            None => {
                let mut command = Command::new("sh");
                command.args(["-c", program]);
                command
            }
        };
        for name in WITHHELD {
            command.env_remove(name);
        }
        if let Some(root) = &self.plugin {
            command.env("CLAUDE_PLUGIN_ROOT", root);
        }
        let spawned = command
            .current_dir(&self.cwd)
            .env("CLAUDE_PROJECT_DIR", &self.cwd)
            .env("CLAUDE_CODE_SESSION_ID", &self.id)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        let mut child = match spawned {
            Ok(child) => child,
            Err(error) => {
                say(&format!("the hook {program} could not run: {error}"));
                return Ran::default();
            }
        };
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(input.to_string().as_bytes());
        }
        let limit = Duration::from_secs(hook["timeout"].as_u64().unwrap_or(600));
        let began = Instant::now();
        let code = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status.code().unwrap_or(-1),
                Ok(None) if began.elapsed() < limit => {
                    std::thread::sleep(Duration::from_millis(20));
                }
                _ => {
                    let _ = child.kill();
                    let _ = child.wait();
                    say(&format!("the hook {program} timed out"));
                    return Ran::default();
                }
            }
        };
        let mut ran = Ran {
            code,
            ..Ran::default()
        };
        if let Some(mut out) = child.stdout.take() {
            let _ = out.read_to_string(&mut ran.out);
        }
        if let Some(mut err) = child.stderr.take() {
            let _ = err.read_to_string(&mut ran.err);
        }
        ran
    }

    /// The prompt queued and taken, as every prompt is.
    fn queued(&self, prompt: &str) {
        self.line(json!({
            "type": "queue-operation", "operation": "enqueue",
            "timestamp": now(), "sessionId": self.id, "content": prompt,
        }));
        self.line(json!({
            "type": "queue-operation", "operation": "dequeue",
            "timestamp": now(), "sessionId": self.id,
        }));
    }

    fn user(&mut self, content: Value) {
        let record = self.record("user", json!({ "role": "user", "content": content }));
        self.line(record);
    }

    fn assistant(&mut self, content: Value, stop: &str) {
        let message = json!({
            "id": format!("msg_stand_in_{}", self.records + 1),
            "model": "stand-in",
            "role": "assistant",
            "type": "message",
            "stop_reason": stop,
            "content": content,
        });
        let record = self.record("assistant", message);
        self.line(record);
    }

    fn result(&mut self, id: &str, text: &str, error: bool) {
        let content = json!([{
            "type": "tool_result", "tool_use_id": id, "content": text, "is_error": error,
        }]);
        let mut record = self.record("user", json!({ "role": "user", "content": content }));
        record["toolUseResult"] = json!({ "output": text });
        self.line(record);
    }

    /// A conversation record in the release's shape, chained to the last.
    fn record(&mut self, kind: &str, message: Value) -> Value {
        self.records += 1;
        let uuid = format!("{}-{:04}-{}", &self.id[..8], self.records, kind.len());
        let mut fields = BTreeMap::new();
        fields.insert("parentUuid", json!(self.parent));
        fields.insert("isSidechain", json!(false));
        fields.insert("type", json!(kind));
        fields.insert("message", message);
        fields.insert("uuid", json!(uuid.clone()));
        fields.insert("timestamp", json!(now()));
        fields.insert("userType", json!("external"));
        fields.insert("cwd", json!(self.cwd));
        fields.insert("sessionId", json!(self.id));
        fields.insert("version", json!(VERSION));
        self.parent = Some(uuid);
        json!(fields)
    }

    fn line(&self, record: Value) {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.transcript)
            .expect("the transcript");
        writeln!(file, "{record}").expect("a transcript line");
    }
}

/// How a turn came out: ended, whether its acts were made, and the
/// session open for another; or the session over with it.
enum Turn {
    Ended(bool),
    Over,
}

/// What the hooks of one event answered.
#[derive(Default)]
struct Answer {
    denied: Option<String>,
    blocked: Option<String>,
    /// Each hook's added context, in order.
    context: Vec<String>,
}

#[derive(Default)]
struct Ran {
    code: i32,
    out: String,
    err: String,
}

/// Whether a group's matcher takes the tool: every tool where it names
/// none or `*`, else one of the names it joins by `|`, as the plugin's
/// matchers are written.
fn matches(matcher: Option<&str>, tool: Option<&str>) -> bool {
    match (matcher, tool) {
        (None | Some("" | "*"), _) => true,
        (Some(_), None) => true,
        (Some(names), Some(tool)) => names.split('|').any(|name| name == tool),
    }
}

/// The time now in UTC, as the transcript writes it,
/// `2026-10-29T00:57:45.327Z`.
fn now() -> String {
    let since = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let seconds = since.as_secs();
    let days = (seconds / 86_400) as i64;
    let of_day = seconds % 86_400;
    // Days to a civil date (Howard Hinnant's days_from_civil, inverted).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:03}Z",
        of_day / 3600,
        of_day % 3600 / 60,
        of_day % 60,
        since.subsec_millis()
    )
}

/// The line printed, and kept in the log where the script was read by its
/// start.
fn say(line: &str) {
    println!("stand-in: {line}");
    if let Some(log) = LOG.get() {
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log)
        {
            let _ = writeln!(file, "stand-in: {line}");
        }
    }
}
