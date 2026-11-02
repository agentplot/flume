//! Claude Code's carry (175, C.4; core.32): whether the account chosen for
//! a stage's next start reaches the conversation of the start whose agent
//! stopped at its account's usage limit. 2.1.296 keeps a session's
//! transcript at `<home>/projects/<folder>/<session>.jsonl`, `<home>` its
//! configuration home, `CLAUDE_CONFIG_DIR` or else `~/.claude`, and
//! `--resume <session>` looks for the session under that home's
//! `projects` alone and goes on in it under the same session. So the
//! conversation is carried where the chosen account's
//! `<home>/projects/<folder>/<file>` is the very file the stopped start's
//! started ack named, by its device and inode with links followed:
//! `<folder>` and `<file>` are the last two names of that path, so flume
//! computes no name of Claude Code's. A host makes a conversation
//! reachable by sharing one `projects` folder between its accounts; flume
//! writes, links and copies nothing in either home, and only reads
//! whether the host did.

use super::record::Record;
use flume_seams::Carry;
use std::collections::BTreeMap;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

/// The option that resumes a session by its id.
pub const FLAG: &str = "--resume";

/// The variable naming Claude Code's configuration home.
pub const CONFIG_DIR: &str = "CLAUDE_CONFIG_DIR";

/// Why not, where the stopped start never acked.
pub const UNKNOWN: &str = "its conversation is not known: the agent that stopped never said it started, so no configuration home (`CLAUDE_CONFIG_DIR`) is known to hold it";

/// Why not, where the stopped start's transcript cannot be read.
pub const UNREADABLE: &str = "its conversation's transcript, in the stopped account's configuration home (`CLAUDE_CONFIG_DIR`), cannot be read";

/// Why not, where the chosen account's home holds another file or none.
pub const UNREACHED: &str = "the chosen account's configuration home (`CLAUDE_CONFIG_DIR`) does not reach its conversation: its `projects` folder is not the stopped account's";

/// The configuration home of an account whose environment is `env`: its
/// `CLAUDE_CONFIG_DIR`, else `.claude` under `login`, the host user's
/// home; none where neither is given. An empty value is none, as 2.1.296
/// reads it.
pub fn home(env: &BTreeMap<String, String>, login: Option<&Path>) -> Option<PathBuf> {
    env.get(CONFIG_DIR)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .or_else(|| login.map(|login| login.join(".claude")))
}

/// The carry of the stopped start whose record is `record` to the account
/// whose environment is `env`, `login` the host user's home: yes with the
/// session its started ack named where the account's home reaches that
/// ack's transcript, and no otherwise, the words naming `CLAUDE_CONFIG_DIR`
/// and no path (175; core.30).
pub fn carry(record: &Record, env: &BTreeMap<String, String>, login: Option<&Path>) -> Carry {
    let no = |why: &str| Carry::No { why: why.into() };
    let Some(started) = record
        .started()
        .filter(|started| !started.session_id.is_empty())
    else {
        return no(UNKNOWN);
    };
    let transcript = Path::new(&started.transcript_path);
    let (Ok(stopped), Some(file), Some(folder)) = (
        std::fs::metadata(transcript),
        transcript.file_name(),
        transcript.parent().and_then(Path::file_name),
    ) else {
        return no(UNREADABLE);
    };
    let Some(home) = home(env, login) else {
        return no(UNREACHED);
    };
    match std::fs::metadata(home.join("projects").join(folder).join(file)) {
        Ok(found) if found.dev() == stopped.dev() && found.ino() == stopped.ino() => Carry::Yes {
            session: started.session_id,
        },
        _ => no(UNREACHED),
    }
}

/// The source of the pinned release's resume, each fragment by its name
/// as the release's program holds it: `--resume`'s option and
/// `--fork-session`'s, which a resumed session keeps its id without, the
/// transcripts' folder under the configuration home, and the words it
/// ends with where it finds no such session (C.4; core.22).
pub fn fragments() -> Vec<(&'static str, String)> {
    vec![
        (
            "the option that resumes a session",
            format!("\"-r, {FLAG} [value]\",\"Resume a conversation by session ID, or open interactive picker with optional search term\""),
        ),
        (
            "the option that forks a resumed session",
            "\"--fork-session\",\"When resuming, create a new session ID instead of reusing the original (use with --resume or --continue)\"".into(),
        ),
        (
            "the transcripts' folder under the configuration home",
            "function Df(){return S(be(),\"projects\")}".into(),
        ),
        (
            "the words where no such session is found",
            "No conversation found with session ID: ".into(),
        ),
    ]
}
