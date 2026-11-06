//! Claude Code's live (207, C.4; core.10): whether a process of the host
//! runs the conversation of a start, the session its started ack names.
//! 2.1.296 names each session it runs in `<home>/sessions/<pid>.json`,
//! `<home>` its configuration home, `CLAUDE_CONFIG_DIR` or else
//! `~/.claude`: the process's `pid`, its `sessionId` and its `procStart`,
//! when the process started as `ps -o lstart=` gives it under `LC_ALL=C`
//! and `TZ=UTC`; it removes the record at its exit, and a record left by a
//! process that was killed names a process that is gone, or an id the host
//! gave another since. So a record counts only where the host runs a
//! process with its `pid` started at its `procStart`, as 2.1.296 reads its
//! own records. A session resumed under a home no account declares has no
//! record flume reads, so the host's processes are listed once besides,
//! and each `claude` whose arguments resume the session after `--resume`
//! or `-r` counts, the program by the last name of its first argument or,
//! where that is the program a `claude` nix wrapper runs, as
//! [`wrapper`](super::wrapper) reads one. flume reads only: it ends,
//! signals and sends nothing to a process it counts (core.10).

use super::carry;
use super::record::Record;
use super::wrapper;
use crate::host::process::output_bounded;
use flume_seams::{Live, LiveProcess, Seconds};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// The folder under a configuration home that holds a record per running
/// session.
pub const SESSIONS: &str = "sessions";

/// The short form of `--resume`.
pub const SHORT: &str = "-r";

/// Unknown, where a home's `sessions` folder stands and cannot be read.
pub const UNREADABLE: &str = "the running sessions of a configuration home (`CLAUDE_CONFIG_DIR`) cannot be read: its `sessions` folder stands and cannot be listed";

/// Unknown, where the host's processes cannot be listed.
pub const UNLISTED: &str = "the host's processes cannot be listed";

/// The most a listing of the host's processes waits on `ps`.
const BOUND: Duration = Duration::from_secs(10);

/// A session's record as 2.1.296 writes it, the fields live reads.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SessionRecord {
    #[serde(default)]
    pid: Option<u32>,
    #[serde(default)]
    session_id: Option<String>,
    #[serde(default)]
    proc_start: Option<String>,
}

/// A process of the host as `ps -axo pid=,lstart=,args=` lists it under
/// `LC_ALL=C` and `TZ=UTC`: its id, when it started in `lstart`'s words,
/// each run of spaces one, and its arguments joined by spaces.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Process {
    pub pid: u32,
    pub lstart: String,
    pub args: String,
}

impl Process {
    fn live(&self) -> LiveProcess {
        LiveProcess {
            pid: self.pid,
            started: started_at(&self.lstart).unwrap_or_default(),
        }
    }
}

/// The seconds since the epoch at which a process started, from `lstart`'s
/// words under `LC_ALL=C` and `TZ=UTC`, `Fri Nov  6 19:26:39 2026`; none
/// where they read otherwise.
pub fn started_at(lstart: &str) -> Option<Seconds> {
    chrono::NaiveDateTime::parse_from_str(&collapsed(lstart), "%a %b %d %H:%M:%S %Y")
        .ok()
        .and_then(|at| Seconds::try_from(at.and_utc().timestamp()).ok())
}

fn collapsed(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The host's processes, listed once; none where they cannot be.
pub fn processes() -> Option<Vec<Process>> {
    let mut command = Command::new("ps");
    command
        .args(["-axo", "pid=,lstart=,args="])
        .env("LC_ALL", "C")
        .env("TZ", "UTC");
    let listed = output_bounded(command, BOUND)?;
    Some(listed.lines().filter_map(listed_process).collect())
}

/// One line of the listing: the id, `lstart`'s five words, and the
/// arguments after them.
fn listed_process(line: &str) -> Option<Process> {
    let mut rest = line.trim_start();
    let mut word = || {
        let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let found = &rest[..end];
        rest = rest[end..].trim_start();
        (!found.is_empty()).then_some(found)
    };
    let pid = word()?.parse().ok()?;
    let lstart: Vec<&str> = (0..5).map(|_| word()).collect::<Option<_>>()?;
    Some(Process {
        pid,
        lstart: lstart.join(" "),
        args: rest.trim_end().to_string(),
    })
}

/// Whether the process whose arguments are `args` is Claude Code's
/// program: its first argument's last name `claude`, or the program a
/// `claude` nix wrapper beside it runs.
fn is_claude(args: &str) -> bool {
    let Some(first) = args.split_whitespace().next() else {
        return false;
    };
    let program = Path::new(first);
    if program.file_name().is_some_and(|name| name == super::NAME) {
        return true;
    }
    program.is_absolute()
        && program
            .parent()
            .map(|dir| dir.join(super::NAME))
            .and_then(|wrapper| wrapper::unwrapped(&wrapper))
            .is_some_and(|wrapped| wrapped.program == program)
}

/// Whether `args` resume `session`: `--resume <session>`,
/// `--resume=<session>` or `-r <session>`.
fn resumes(args: &str, session: &str) -> bool {
    let words: Vec<&str> = args.split_whitespace().collect();
    words.iter().enumerate().any(|(at, word)| {
        ((*word == carry::FLAG || *word == SHORT) && words.get(at + 1) == Some(&session))
            || word
                .strip_prefix(carry::FLAG)
                .and_then(|rest| rest.strip_prefix('='))
                == Some(session)
    })
}

/// The session the start's record names: its started ack's, else the
/// session of its word that it began otherwise than recorded; none where
/// it said neither.
fn session(record: &Record) -> Option<String> {
    record
        .started()
        .map(|started| started.session_id)
        .or_else(|| record.unsettled().map(|unsettled| unsettled.session_id))
        .filter(|session| !session.is_empty())
}

/// Each `(pid, procStart)` of a record under `home` that names `session`;
/// none where `home` keeps no `sessions` folder; an error where it stands
/// and cannot be listed, or a record in it cannot be read.
fn records(home: &Path, session: &str) -> Result<Vec<(u32, Option<String>)>, ()> {
    let folder = home.join(SESSIONS);
    let entries = match std::fs::read_dir(&folder) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(_) => return Err(()),
    };
    let mut found = vec![];
    for entry in entries {
        let path = entry.map_err(|_| ())?.path();
        if path.extension().is_none_or(|extension| extension != "json") {
            continue;
        }
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            // A process that ended between the listing and the reading
            // took its record with it.
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_) => return Err(()),
        };
        // A record half written, or not 2.1.296's, names no session.
        let Ok(record) = serde_json::from_str::<SessionRecord>(&text) else {
            continue;
        };
        if let (Some(pid), Some(named)) = (record.pid, record.session_id) {
            if named == session {
                found.push((pid, record.proc_start));
            }
        }
    }
    Ok(found)
}

/// Live of the start whose record is `record`, under each of `envs`'
/// homes, `login` the host user's home, against the host's processes as
/// `list` gives them: runs with each process, by its id, where a home's
/// record of the session names a process the host runs started at its
/// `procStart`, a record naming none counted where its process runs, as
/// 2.1.296 reads its own; or where a `claude` resumes the session in its
/// arguments, under any home; none where the start named no session or
/// none runs it; and unknown, naming `CLAUDE_CONFIG_DIR` or the host's
/// processes and no path, where a home's records or the processes cannot
/// be read (207; core.30).
pub fn live(
    record: &Record,
    envs: &[BTreeMap<String, String>],
    login: Option<&Path>,
    list: impl FnOnce() -> Option<Vec<Process>>,
) -> Live {
    let Some(session) = session(record) else {
        return Live::None;
    };
    let mut homes: Vec<PathBuf> = envs
        .iter()
        .filter_map(|env| carry::home(env, login))
        .collect();
    homes.sort();
    homes.dedup();
    let mut named = vec![];
    for home in &homes {
        match records(home, &session) {
            Ok(found) => named.extend(found),
            Err(()) => {
                return Live::Unknown {
                    what: UNREADABLE.into(),
                }
            }
        }
    }
    let Some(processes) = list() else {
        return Live::Unknown {
            what: UNLISTED.into(),
        };
    };
    let mut running: Vec<LiveProcess> = processes
        .iter()
        .filter(|process| {
            named.iter().any(|(pid, proc_start)| {
                *pid == process.pid
                    && proc_start
                        .as_deref()
                        .is_none_or(|at| collapsed(at) == process.lstart)
            }) || (is_claude(&process.args) && resumes(&process.args, &session))
        })
        .map(Process::live)
        .collect();
    running.sort_by_key(|process| process.pid);
    running.dedup_by_key(|process| process.pid);
    if running.is_empty() {
        Live::None
    } else {
        Live::Runs(running)
    }
}

/// The source of the pinned release's record of a running session, each
/// fragment by its name as the release's program holds it: the folder
/// under the configuration home, the record's fields as it writes them,
/// the words of `procStart` it reads from `ps`, and `--resume`'s short
/// form (207, C.4; core.22).
pub fn fragments() -> Vec<(&'static str, String)> {
    vec![
        (
            "the running sessions' folder under the configuration home",
            format!("function oV(){{return A(be(),\"{SESSIONS}\")}}"),
        ),
        (
            "the record of a running session, by the process's id",
            "w=Wt(b,`${process.pid}.json`)".into(),
        ),
        (
            "the record's process, session and start",
            "P=_({pid:process.pid,sessionId:K(),cwd:Ee(),startedAt:Date.now(),procStart:await Ul(process.pid),".into(),
        ),
        (
            "when the process started, as ps gives it",
            "[\"-o\",\"lstart=\",\"-p\",String(e)],{timeout:1000,...r,env:{...n??process.env,LC_ALL:\"C\",TZ:\"UTC\"}}".into(),
        ),
        (
            "the record removed at the process's exit",
            "process.on(\"exit\",()=>{try{wU(w)}catch{}})".into(),
        ),
        (
            "the short form of the option that resumes a session",
            format!("\"{SHORT}, {} [value]\"", carry::FLAG),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_listing_line_is_read() {
        assert_eq!(
            listed_process("  4242 Fri Nov  6 19:26:39 2026     claude --resume s-1  "),
            Some(Process {
                pid: 4242,
                lstart: "Fri Nov 6 19:26:39 2026".into(),
                args: "claude --resume s-1".into(),
            })
        );
        assert_eq!(listed_process("4242 Fri Nov 6"), None);
        assert_eq!(listed_process("pid Fri Nov 6 19:26:39 2026 claude"), None);
        assert_eq!(started_at("Fri Nov  6 19:26:39 2026"), Some(1_793_993_199));
        assert_eq!(started_at("yesterday"), None);
    }

    #[test]
    fn a_resume_is_read_in_its_three_forms() {
        for args in [
            "claude --resume s-1",
            "claude --plugin-dir /p -r s-1 --model opus",
            "/nix/store/x/bin/claude --resume=s-1",
        ] {
            assert!(is_claude(args) && resumes(args, "s-1"), "{args}");
        }
        assert!(!resumes("claude --resume s-10", "s-1"));
        assert!(!resumes("claude s-1", "s-1"));
        assert!(!resumes("claude --resume", "s-1"));
        assert!(!is_claude("vim --resume s-1"));
        assert!(!is_claude("/opt/claude-tools/bin/other --resume s-1"));
        assert!(!is_claude(""));
    }

    #[test]
    fn a_wrapped_program_is_read_as_claude() {
        let dir = std::env::temp_dir().join(format!("flume-live-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch folder");
        let program = dir.join(".claude-wrapped");
        std::fs::write(&program, "the program").expect("the program");
        std::fs::write(
            dir.join("claude"),
            format!(
                "#! /bin/bash -e\nexport DISABLE_AUTOUPDATER='1'\nexec -a \"claude\" \"{}\"  \"$@\" \n",
                program.display()
            ),
        )
        .expect("the wrapper");
        assert!(is_claude(&format!("{} --resume s-1", program.display())));
        assert!(!is_claude(&format!(
            "{} --resume s-1",
            dir.join(".other").display()
        )));
        assert!(!is_claude(".claude-wrapped --resume s-1"));
        let _ = std::fs::remove_dir_all(dir);
    }
}
