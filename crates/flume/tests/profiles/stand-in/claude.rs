//! The stand-in for Claude Code in the walkthroughs (design 18): the
//! program the printed command runs as `claude`, first on the walkthrough's
//! `PATH`, in the place. It acts as the pinned Claude Code, 2.1.296, acts
//! for flume's plugin: it writes a transcript in that release's shape, runs
//! the hooks the plugin's `hooks/hooks.json` names under `--plugin-dir`, as
//! that release runs them (the exec form with no shell, the hook's input on
//! standard input, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR` and
//! `CLAUDE_CODE_SESSION_ID` over the command's own environment, a refusal
//! read from `permissionDecision` or exit 2, a stop blocked by `decision`
//! or exit 2), and in place of a model acts the script `STAND_IN_SCRIPT`
//! names, one run's acts as a JSON array, each a tool call:
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
//!   words in place of Stop, no later act made.
//!
//! The prompt is submitted as given, or, where `STAND_IN_PASTE_TAG` names
//! a four-hex-digit id, inside the paste tags that release wraps a pasted
//! prompt in where the account's features turn them on.
//!
//! Then the turn ends: Stop, once more with `stop_hook_active` where the
//! hook blocked it, or StopFailure where a `fail` act ended it, and the
//! session ends, as a person leaves the printed
//! command's session, with SessionEnd. Each act is printed on standard
//! output as one line; the exit status is 1 where an act failed that no
//! hook refused. It never reaches a model, an account or the network.

use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// The release whose shape the transcript takes.
const VERSION: &str = "2.1.296";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut plugin = None;
    let mut prompt = None;
    let mut words = args.iter();
    while let Some(word) = words.next() {
        match word.as_str() {
            "--plugin-dir" => plugin = words.next().map(PathBuf::from),
            "--model" | "--append-system-prompt" | "--settings" | "--permission-mode" => {
                words.next();
            }
            flag if flag.starts_with("--") => {}
            given => prompt = Some(given.to_string()),
        }
    }
    let cwd = std::env::current_dir().expect("the place");
    let mut session = Session::new(cwd, plugin);
    let failed = session.run(prompt.as_deref());
    std::process::exit(i32::from(failed));
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
}

impl Session {
    fn new(cwd: PathBuf, plugin: Option<PathBuf>) -> Session {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_nanos());
        let id = format!(
            "{:08x}-{:04x}-4{:03x}-8{:03x}-{:012x}",
            (nanos >> 32) as u32,
            std::process::id() & 0xffff,
            (nanos >> 20) as u16 & 0xfff,
            (nanos >> 8) as u16 & 0xfff,
            nanos as u64 & 0xffff_ffff_ffff
        );
        let config = std::env::var_os("CLAUDE_CONFIG_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".claude")
            });
        let project: String = cwd
            .to_string_lossy()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect();
        let transcript = config
            .join("projects")
            .join(project)
            .join(format!("{id}.jsonl"));
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
            parent: None,
            calls: 0,
        }
    }

    /// The session: its start, the prompt, the script's acts, the stop
    /// and the end. Whether an act failed.
    fn run(&mut self, prompt: Option<&str>) -> bool {
        if let Some(dir) = self.transcript.parent() {
            std::fs::create_dir_all(dir).expect("the transcript's folder");
        }
        say(&format!("session {} in {}", self.id, self.cwd.display()));
        self.hook("SessionStart", json!({ "source": "startup" }), None);
        let mut failed = false;
        if let Some(prompt) = prompt {
            self.prompt = prompt.to_string();
            let submitted = submitted(prompt);
            self.queued(&submitted);
            self.hook("UserPromptSubmit", json!({ "prompt": submitted }), None);
            self.user(json!(submitted));
            let script = std::env::var("STAND_IN_SCRIPT")
                .ok()
                .and_then(|file| std::fs::read_to_string(file).ok())
                .and_then(|text| serde_json::from_str::<Value>(&text).ok())
                .unwrap_or(json!([]));
            for act in script.as_array().into_iter().flatten() {
                if let Some(failure) = act.get("fail") {
                    self.fail(failure);
                    return failed;
                }
                if !self.act(act) {
                    failed = true;
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
        self.hook("SessionEnd", json!({ "reason": "prompt_input_exit" }), None);
        say("session ended");
        failed
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
                "id": format!("msg_stand_in_{}", self.calls),
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
        let common = json!({
            "session_id": self.id,
            "transcript_path": self.transcript,
            "cwd": self.cwd,
            "permission_mode": "default",
            "hook_event_name": event,
        });
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
        if event != "Stop" {
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
            "id": format!("msg_stand_in_{}", self.calls),
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
        let uuid = format!("{}-{:04}-{}", &self.id[..8], self.calls, kind.len());
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

/// What the hooks of one event answered.
#[derive(Default)]
struct Answer {
    denied: Option<String>,
    blocked: Option<String>,
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

fn say(line: &str) {
    println!("stand-in: {line}");
}
