//! The agent profile for Claude Code (C.4; design 14; record 0016): the
//! command starts `claude` in the place with flume's own plugin loaded by
//! `--plugin-dir`, the plugin written once per version of flume under
//! `$XDG_DATA_HOME/flume/plugins/claude/`, its hooks naming the binary
//! that wrote it by its absolute path; every reading reads the start's
//! record in the place's own folder, which the plugin's hooks write
//! (`flume hook <event>`, [`hook`]), and the session's transcript the
//! started ack recorded (128). Every fact of Claude Code read here is
//! checked against the release `devenv.nix` pins, 2.1.296: its hook
//! events and their inputs, its transcript's records, its folder-trust
//! dialog and the keys that answer it.

pub mod guard;
pub mod hook;
pub mod record;
pub mod transcript;
pub mod wrapper;

use flume_seams::{
    Agent, AgentCommand, FolderTrust, Guarded, Identity, Offered, PlaceRef, Reading, Received,
    Secret, Settings,
};
use record::Record;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// The program the profile runs, and the name a repository's or a
/// host's settings bind it by (32, C.4).
pub const NAME: &str = "claude";

/// The program's kind, by which a terminal that starts a known agent
/// program names it (C.4).
pub const KIND: &str = "claude";

/// The version of flume, which the plugin it writes carries, so a hook
/// run by a binary of another version is reported (135).
pub const VERSION: &str = crate::VERSION;

/// A line of the folder-trust dialog Claude Code 2.1.296 shows in a
/// folder new to it: its confirming choice, short enough that no width of
/// pane wraps it. The dialog lists "No, exit" first and focused, so the
/// answer moves down to "Yes, I trust this folder" and confirms, in
/// herdr's key names (34; record 0018).
pub const TRUST_QUESTION: &str = "Yes, I trust this folder";

/// The keys that answer the folder-trust dialog yes (34).
pub const TRUST_ANSWER: &str = "down enter";

/// The variables the command gives the agent, which the plugin's hooks
/// read (33, 41, C.4).
pub const AGENT: &str = "FLUME_AGENT";
pub const SECRET: &str = "FLUME_SECRET";
pub const PLACE: &str = "FLUME_PLACE";
pub const STAGE: &str = "FLUME_STAGE";

/// The plugin's files, as `plugins/claude` holds them, by their path in
/// the plugin.
pub const PLUGIN: [(&str, &str); 4] = [
    (
        ".claude-plugin/plugin.json",
        include_str!("../../../../../../plugins/claude/.claude-plugin/plugin.json"),
    ),
    (
        "hooks/hooks.json",
        include_str!("../../../../../../plugins/claude/hooks/hooks.json"),
    ),
    (
        ".mcp.json",
        include_str!("../../../../../../plugins/claude/.mcp.json"),
    ),
    (
        "commands/launch.md",
        include_str!("../../../../../../plugins/claude/commands/launch.md"),
    ),
];

/// A file of the plugin as the binary writes it for the agents it
/// starts: each command naming `flume` names the binary by its absolute
/// path instead (C.4), and the manifest carries the binary's version, a
/// release's where the repository's says the workspace's (135).
pub fn written(text: &str, binary: &Path) -> String {
    let path = serde_json::to_string(&binary.to_string_lossy()).expect("a path as JSON");
    text.replace("\"command\": \"flume\"", &format!("\"command\": {path}"))
        .replace(
            concat!("\"version\": \"", env!("CARGO_PKG_VERSION"), "\""),
            &format!("\"version\": \"{VERSION}\""),
        )
}

/// The hex digest of `text`'s SHA-256.
pub fn digest(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// `text` with every run of whitespace one space and none at its ends, as
/// a prompt is compared with what the agent received (89).
pub fn collapsed(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Claude Code with flume's plugin.
#[derive(Clone, Debug)]
pub struct Claude {
    /// Where the plugin's versions are written.
    plugins: PathBuf,
    /// The binary the plugin's hooks call.
    binary: PathBuf,
}

impl Claude {
    /// The plugin under `$XDG_DATA_HOME/flume/plugins/claude`, its hooks
    /// calling the binary that runs (design 14).
    pub fn of_host() -> Claude {
        let data = std::env::var_os("XDG_DATA_HOME")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/share")
            });
        let binary = std::env::current_exe()
            .and_then(fs::canonicalize)
            .unwrap_or_else(|_| PathBuf::from("flume"));
        Claude::new(&data, &binary)
    }

    /// The plugin under `<data>/flume/plugins/claude`, its hooks calling
    /// `binary`.
    pub fn new(data: &Path, binary: &Path) -> Claude {
        Claude {
            plugins: data.join("flume/plugins/claude"),
            binary: binary.to_path_buf(),
        }
    }

    /// The plugin's directory for this version of flume and this binary:
    /// the version, then a digest of the binary's path and the files, so
    /// two binaries of one version never share a plugin whose hooks name
    /// the other.
    pub fn plugin_dir(&self) -> PathBuf {
        let mut text = self.binary.to_string_lossy().into_owned();
        for (path, file) in PLUGIN {
            text.push('\0');
            text.push_str(path);
            text.push('\0');
            text.push_str(file);
        }
        self.plugins
            .join(format!("{VERSION}-{}", &digest(&text)[..12]))
    }

    /// The plugin, written once for this version and binary: a directory
    /// already written is kept as it is, and one is written whole beside
    /// it and moved into place, so no agent loads half a plugin.
    pub fn plugin(&self) -> Result<PathBuf, String> {
        let dir = self.plugin_dir();
        if dir.join(PLUGIN[0].0).is_file() {
            return Ok(dir);
        }
        let partial = dir.with_extension(format!("partial-{}", std::process::id()));
        let write = || -> std::io::Result<()> {
            for (path, text) in PLUGIN {
                let file = partial.join(path);
                if let Some(parent) = file.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(file, written(text, &self.binary))?;
            }
            Ok(())
        };
        let outcome = write().and_then(|()| fs::rename(&partial, &dir));
        let _ = fs::remove_dir_all(&partial);
        match outcome {
            Ok(()) => Ok(dir),
            Err(_) if dir.join(PLUGIN[0].0).is_file() => Ok(dir),
            Err(error) => Err(format!("the plugin at {}: {error}", dir.display())),
        }
    }
}

impl Agent for Claude {
    /// `claude --plugin-dir <plugin> [--model <m>] [--append-system-prompt
    /// <instructions>]` in the place, the program the settings name by
    /// name or path, or the program it runs where it is a nix wrapper,
    /// with the variables the wrapper sets ([`wrapper`]), the resolved
    /// environment and the four `FLUME_*` words, the prompt as its argument where the terminal cannot send
    /// it; nothing of the caller's session (29, 32, 33, 88, C.4). The
    /// host user's login environment is the terminal's to give, set on
    /// the pane under herdr and the person's shell's under the printed
    /// command.
    fn command(
        &self,
        settings: &Settings,
        place: &PlaceRef,
        identity: &Identity,
        secret: &Secret,
        prompt: &str,
    ) -> AgentCommand {
        let plugin = self.plugin().unwrap_or_else(|_| self.plugin_dir());
        let mut args = vec![
            "--plugin-dir".to_string(),
            plugin.to_string_lossy().into_owned(),
        ];
        if let Some(model) = &settings.model {
            args.extend(["--model".to_string(), model.clone()]);
        }
        if let Some(instructions) = &settings.instructions {
            args.extend(["--append-system-prompt".to_string(), instructions.clone()]);
        }
        let mut env: BTreeMap<String, String> = settings.env.clone();
        let mut program = settings.program.clone().unwrap_or_else(|| NAME.into());
        let path = env
            .get("PATH")
            .cloned()
            .or_else(|| std::env::var("PATH").ok())
            .unwrap_or_default();
        if let Some(wrapped) =
            wrapper::located(&program, &path).and_then(|file| wrapper::unwrapped(&file))
        {
            program = wrapped.program.to_string_lossy().into_owned();
            wrapped.environment(&mut env);
        }
        env.insert(AGENT.into(), identity.to_string());
        env.insert(SECRET.into(), secret.0.clone());
        env.insert(PLACE.into(), place.location.clone());
        env.insert(STAGE.into(), identity.stage.clone());
        AgentCommand {
            program,
            args,
            env,
            prompt_args: vec![prompt.to_string()],
            folder_trust: Some(FolderTrust {
                question: TRUST_QUESTION.into(),
                answer: TRUST_ANSWER.into(),
            }),
            kind: Some(KIND.into()),
        }
    }

    /// The start's started ack (135).
    fn started(&self, folder: &str, agent: &Identity) -> Reading {
        let record = Record::at(folder, agent);
        if !record.readable() {
            return Reading::Unknown;
        }
        match record.started() {
            Some(_) => Reading::Yes,
            None => Reading::No,
        }
    }

    /// The start's received ack, the prompt's digest matched; where none
    /// came, the transcript's user turn or queued record holding the
    /// prompt, a slash command by its name and arguments, whitespace
    /// collapsed; a transcript not yet written reads not received (38,
    /// 89).
    fn received(&self, folder: &str, agent: &Identity, text: &str) -> Received {
        let record = Record::at(folder, agent);
        if !record.readable() {
            return Received::Unknown;
        }
        match record.received(&digest(&collapsed(text))) {
            Some(true) => {
                return Received::Received {
                    record: record.received_path().to_string_lossy().into_owned(),
                }
            }
            Some(false) => {}
            None => return Received::Unknown,
        }
        let Some(started) = record.started() else {
            return Received::NotReceived {
                record: record.dir().to_string_lossy().into_owned(),
            };
        };
        // With the plugin's hooks in force every prompt Claude Code submits
        // is acked, a queued one when it is submitted; a slash command
        // Claude Code runs itself fires no UserPromptSubmit, and only its
        // transcript can say it was received (38, 89).
        if !collapsed(text).starts_with('/') {
            return Received::NotReceived {
                record: record.received_path().to_string_lossy().into_owned(),
            };
        }
        let path = started.transcript_path.clone();
        match transcript::Transcript::read(Path::new(&path)) {
            transcript::Read::Read(transcript) if transcript.holds_prompt(text) => {
                Received::Received { record: path }
            }
            transcript::Read::Read(_) | transcript::Read::NotWritten => {
                Received::NotReceived { record: path }
            }
            transcript::Read::Unreadable => Received::Unknown,
        }
    }

    /// The mark the plugin leaves on a question, a plan or a permission
    /// prompt for a person, until the transcript holds its answer (90).
    fn asking(&self, folder: &str, agent: &Identity) -> Reading {
        let record = Record::at(folder, agent);
        if !record.readable() {
            return Reading::Unknown;
        }
        let Some(mark) = record.asking() else {
            return Reading::No;
        };
        let answered = record.started().is_some_and(|started| {
            matches!(
                transcript::Transcript::read(Path::new(&started.transcript_path)),
                transcript::Read::Read(transcript) if transcript.answered(&mark)
            )
        });
        if answered {
            Reading::No
        } else {
            Reading::Yes
        }
    }

    /// Background shells and tasks the transcript shows started and not
    /// finished (91).
    fn own_work(&self, folder: &str, agent: &Identity) -> Reading {
        let record = Record::at(folder, agent);
        let Some(started) = record.readable().then(|| record.started()).flatten() else {
            return Reading::Unknown;
        };
        match transcript::Transcript::read(Path::new(&started.transcript_path)) {
            transcript::Read::Read(transcript) => {
                if transcript.background_running().is_empty() {
                    Reading::No
                } else {
                    Reading::Yes
                }
            }
            transcript::Read::NotWritten | transcript::Read::Unreadable => Reading::Unknown,
        }
    }

    /// The SessionEnd hook's mark (134).
    fn ended(&self, folder: &str, agent: &Identity) -> Reading {
        let record = Record::at(folder, agent);
        if !record.readable() {
            Reading::Unknown
        } else if record.ended() {
            Reading::Yes
        } else {
            Reading::No
        }
    }

    /// The reason the StopFailure hook recorded, else the transcript's
    /// last message (43).
    fn last_words(&self, folder: &str, agent: &Identity) -> Option<String> {
        let record = Record::at(folder, agent);
        if !record.readable() {
            return None;
        }
        if let Some(words) = record.last_words() {
            return Some(words);
        }
        let started = record.started()?;
        match transcript::Transcript::read(Path::new(&started.transcript_path)) {
            transcript::Read::Read(transcript) => transcript.last_message(),
            _ => None,
        }
    }

    /// A plugin of another version than this flume's, as the started ack
    /// named it (135).
    fn notes(&self, folder: &str, agent: &Identity) -> Vec<String> {
        let record = Record::at(folder, agent);
        match record.started() {
            Some(started) if started.plugin_version != VERSION => vec![format!(
                "the agent runs flume's plugin version {}, and this flume is version {VERSION}",
                if started.plugin_version.is_empty() {
                    "unknown"
                } else {
                    &started.plugin_version
                }
            )],
            _ => vec![],
        }
    }

    /// `flume needs "<words>"` and `flume needs --found "<words>"` (41,
    /// 92, 131).
    fn stop_short(&self, _agent: &Identity) -> String {
        "run `flume needs \"<words>\"` when you cannot go on without an answer, saying what you need, or `flume needs --found \"<words>\"` when you found work this stage cannot deliver past, saying what you found; then stop.".into()
    }

    fn send_back(&self) -> Offered {
        Offered::Offered
    }

    /// Each act the PreToolUse hook refused (116, 108).
    fn guard(&self, folder: &str, agent: &Identity) -> Guarded {
        Guarded::Refused {
            acts: Record::at(folder, agent).refused(),
        }
    }

    fn on_quiet(&self) -> Offered {
        Offered::Offered
    }
}
