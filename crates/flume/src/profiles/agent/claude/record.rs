//! A start's record in the place's own folder, which the plugin's hooks
//! write and the profile's readings read (128): under
//! `<folder>/claude/<identity>/`, the started ack, the digests of the
//! prompts received, the asking mark, the acts the guard refused, the
//! session's end, a failed turn's reason, its error and when it ran, and
//! the word that the agent began otherwise than its start recorded.

use flume_seams::Identity;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// The started ack: the session the start's agent runs and what it said
/// of itself (135).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Started {
    pub session_id: String,
    pub transcript_path: String,
    /// Claude Code's word for how the session began: startup, resume,
    /// clear, compact or fork.
    pub source: String,
    /// The version of the plugin whose hook acked.
    pub plugin_version: String,
}

/// The asking mark: the call a person is asked to answer, by its id where
/// Claude Code gives one, else by its tool and input (90).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asking {
    #[serde(default)]
    pub tool_use_id: Option<String>,
    pub tool_name: String,
    #[serde(default)]
    pub tool_input: serde_json::Value,
}

/// A failed turn as the StopFailure hook recorded it: the failure's
/// `error`, the words it gave and when the hook ran by the host's clock
/// (43, 151).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Failure {
    pub error: String,
    pub words: String,
    pub at: u64,
}

/// The SessionStart hook's word that the agent began otherwise than its
/// start recorded, in place of the started ack: each variable by its
/// name alone and whether the program differed, with the session it runs
/// and its transcript (161).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unsettled {
    #[serde(flatten)]
    pub unsettled: flume_seams::Unsettled,
    pub session_id: String,
    pub transcript_path: String,
}

/// One start's record in one folder.
#[derive(Clone, Debug)]
pub struct Record {
    folder: PathBuf,
    dir: PathBuf,
}

impl Record {
    /// The record of the start `identity` names in `folder`.
    pub fn at(folder: &str, identity: &Identity) -> Record {
        let folder = PathBuf::from(folder);
        let dir = folder
            .join("claude")
            .join(identity.to_string().replace('/', "%"));
        Record { folder, dir }
    }

    /// The record of the start whose identity is written `identity`.
    pub fn named(folder: &str, identity: &str) -> Record {
        let folder = PathBuf::from(folder);
        let dir = folder.join("claude").join(identity.replace('/', "%"));
        Record { folder, dir }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Whether the folder handed to the reading is one there is: a reading
    /// of a folder that does not stand is unknown (128, 93).
    pub fn readable(&self) -> bool {
        self.folder.is_dir()
    }

    fn file(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    fn write(&self, name: &str, text: &str) -> std::io::Result<()> {
        fs::create_dir_all(&self.dir)?;
        let partial = self.file(&format!(".{name}.{}", std::process::id()));
        fs::write(&partial, text)?;
        fs::rename(partial, self.file(name))
    }

    fn append(&self, name: &str, line: &str) -> std::io::Result<()> {
        fs::create_dir_all(&self.dir)?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.file(name))?;
        writeln!(file, "{}", line.replace('\n', " "))
    }

    fn lines(&self, name: &str) -> Vec<String> {
        fs::read_to_string(self.file(name))
            .map(|text| text.lines().map(str::to_string).collect())
            .unwrap_or_default()
    }

    pub fn started(&self) -> Option<Started> {
        let text = fs::read_to_string(self.file("started.json")).ok()?;
        serde_json::from_str(&text).ok()
    }

    pub fn ack_started(&self, started: &Started) -> std::io::Result<()> {
        let text = serde_json::to_string_pretty(started).expect("the ack as JSON");
        self.write("started.json", &text)
    }

    /// How the agent began otherwise than its start recorded; none where
    /// it began as recorded, or the word cannot be read.
    pub fn unsettled(&self) -> Option<Unsettled> {
        let text = fs::read_to_string(self.file("unsettled.json")).ok()?;
        serde_json::from_str(&text).ok()
    }

    pub fn set_unsettled(&self, unsettled: &Unsettled) -> std::io::Result<()> {
        let text = serde_json::to_string_pretty(unsettled).expect("the word as JSON");
        self.write("unsettled.json", &text)
    }

    /// The session's transcript, as the started ack named it, else the
    /// word that the agent began otherwise.
    pub fn transcript_path(&self) -> Option<String> {
        self.started()
            .map(|started| started.transcript_path)
            .or_else(|| self.unsettled().map(|unsettled| unsettled.transcript_path))
    }

    pub fn received_path(&self) -> PathBuf {
        self.file("received")
    }

    /// Whether a prompt of `digest` was acked received; none where the
    /// acks cannot be read.
    pub fn received(&self, digest: &str) -> Option<bool> {
        match fs::read_to_string(self.file("received")) {
            Ok(text) => Some(text.lines().any(|line| line == digest)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Some(false),
            Err(_) => None,
        }
    }

    pub fn ack_received(&self, digest: &str) -> std::io::Result<()> {
        self.append("received", digest)
    }

    pub fn asking(&self) -> Option<Asking> {
        let text = fs::read_to_string(self.file("asking.json")).ok()?;
        serde_json::from_str(&text).ok()
    }

    pub fn mark_asking(&self, asking: &Asking) -> std::io::Result<()> {
        let text = serde_json::to_string(asking).expect("the mark as JSON");
        self.write("asking.json", &text)
    }

    pub fn clear_asking(&self) -> std::io::Result<()> {
        match fs::remove_file(self.file("asking.json")) {
            Err(error) if error.kind() != std::io::ErrorKind::NotFound => Err(error),
            _ => Ok(()),
        }
    }

    /// Each act the guard refused, in the order refused (116, 108).
    pub fn refused(&self) -> Vec<String> {
        self.lines("refused")
    }

    pub fn refuse(&self, act: &str) -> std::io::Result<()> {
        self.append("refused", act)
    }

    pub fn ended(&self) -> bool {
        self.file("ended").exists()
    }

    pub fn mark_ended(&self, reason: &str) -> std::io::Result<()> {
        self.write("ended", reason)
    }

    pub fn last_words(&self) -> Option<String> {
        fs::read_to_string(self.file("last-words"))
            .ok()
            .filter(|words| !words.trim().is_empty())
    }

    pub fn set_last_words(&self, words: &str) -> std::io::Result<()> {
        self.write("last-words", words)
    }

    /// The last failed turn, none where none was recorded or it cannot
    /// be read.
    pub fn failure(&self) -> Option<Failure> {
        let text = fs::read_to_string(self.file("failure.json")).ok()?;
        serde_json::from_str(&text).ok()
    }

    pub fn set_failure(&self, failure: &Failure) -> std::io::Result<()> {
        let text = serde_json::to_string(failure).expect("the failure as JSON");
        self.write("failure.json", &text)
    }
}
