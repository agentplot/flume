//! What the terminal profiles read alike, kept once so no two of them
//! drift apart: the folder-trust question read off a screen (34), the
//! program a terminal runs checked over the host's process table (32),
//! the label of the part that holds a repository's places on one base,
//! the digest an identity's records are named by, a word quoted for a
//! POSIX shell, and the JSON records a profile keeps in the host's
//! state (core.16). It is no profile and names no tool (core.1); each
//! profile calls it, and no profile calls another for it (core.2).

use crate::host::process::executable;
use flume_seams::{FolderTrust, PlaceRef};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

/// Whether `screen` shows the folder-trust question: its yes choice
/// (34).
pub fn asks(screen: &str, trust: &FolderTrust) -> bool {
    screen.lines().any(|line| line.contains(&trust.question))
}

/// Whether the yes choice has the focus on `screen`, or none where the
/// question is not shown (34).
pub fn focused(screen: &str, trust: &FolderTrust) -> Option<bool> {
    screen
        .lines()
        .find(|line| line.contains(&trust.question))
        .map(|line| !trust.focus.is_empty() && line.contains(&trust.focus))
}

/// A process a terminal says may run the agent's program: its id, its
/// arguments, its folder and its command line where the terminal gives
/// them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Running {
    pub pid: u32,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub cmdline: Option<String>,
}

/// Whether one of `processes` runs `program`, by the file the host's
/// process table says it runs or, for a program a script starts, the
/// script's path among its arguments, a relative one read in its folder;
/// otherwise what each ran, in words a refusal carries (32; record 0018).
pub fn runs_program(processes: &[Running], program: &Path) -> Result<(), Vec<String>> {
    let wanted = program.canonicalize().unwrap_or_else(|_| program.into());
    let mut ran = vec![];
    for process in processes {
        let executable = executable(process.pid);
        let mut paths: Vec<PathBuf> = executable.iter().cloned().collect();
        for arg in &process.args {
            let path = Path::new(arg);
            if path.is_absolute() {
                paths.push(path.into());
            } else if arg.contains('/') {
                if let Some(cwd) = &process.cwd {
                    paths.push(cwd.join(path));
                }
            }
        }
        if paths
            .iter()
            .filter_map(|path| path.canonicalize().ok())
            .any(|path| path == wanted)
        {
            return Ok(());
        }
        let cmdline = process
            .cmdline
            .clone()
            .or_else(|| (!process.args.is_empty()).then(|| process.args.join(" ")));
        ran.push(match (executable, cmdline) {
            (Some(path), Some(cmdline)) => format!("{} ({cmdline})", path.display()),
            (Some(path), None) => path.display().to_string(),
            (None, Some(cmdline)) => cmdline,
            (None, None) => format!("the process {}", process.pid),
        });
    }
    Err(ran)
}

/// The label of the part that holds the places of a repository on one
/// base, `<repository> <base>`, the repository by its directory's name
/// (107, C.5).
pub fn base_label(place: &PlaceRef) -> String {
    let repository = Path::new(&place.repository).file_name().map_or_else(
        || place.repository.clone(),
        |name| name.to_string_lossy().into_owned(),
    );
    format!("{repository} {}", place.base)
}

/// The first twelve hex digits of `text`'s SHA-256, which names an
/// identity's records and a start's folder, since an identity's `:`
/// would split a `PATH`.
pub fn digest(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .take(6)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// `text` quoted for a POSIX shell, whole, whatever it holds.
pub fn quoted(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}

/// The JSON record at `path`; none where it is missing or not JSON.
pub fn read_json(path: &Path) -> Option<Value> {
    serde_json::from_str(&fs::read_to_string(path).ok()?).ok()
}

/// `value` written as the record at `path`, its folder made first; the
/// words naming the path where it cannot be.
pub fn write_json(path: &Path, value: &Value) -> Result<(), String> {
    path.parent()
        .map_or(Ok(()), fs::create_dir_all)
        .and_then(|()| fs::write(path, value.to_string()))
        .map_err(|error| format!("{}: {error}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::process::{Command, Stdio};

    fn trust() -> FolderTrust {
        FolderTrust {
            question: "Yes, I trust this folder".into(),
            focus: "\u{276f}".into(),
            next: "down".into(),
            confirm: "enter".into(),
        }
    }

    #[test]
    fn the_folder_trust_question_and_its_focus_are_read_off_a_screen() {
        // 34: the question by its yes choice, and the focus on it only
        // where its line carries the focus mark.
        let asked = " Quick safety check\n \u{276f} No, exit\n   Yes, I trust this folder\n";
        let focused_yes = " Quick safety check\n   No, exit\n \u{276f} Yes, I trust this folder\n";
        assert!(asks(asked, &trust()));
        assert_eq!(focused(asked, &trust()), Some(false));
        assert_eq!(focused(focused_yes, &trust()), Some(true));
        assert!(!asks("$ claude\n", &trust()));
        assert_eq!(focused("$ claude\n", &trust()), None);
        let unmarked = FolderTrust {
            focus: String::new(),
            ..trust()
        };
        assert_eq!(focused(focused_yes, &unmarked), Some(false));
    }

    #[test]
    fn the_program_is_found_by_its_file_or_its_script_and_else_named() {
        // 32: this test's own process runs its own binary; a `sleep` runs
        // neither it nor a script of the test's, and is named with what it
        // ran; a script is found among its interpreter's arguments, a
        // relative one read in its folder.
        let this = std::env::current_exe().expect("this test's binary");
        let mine = Running {
            pid: std::process::id(),
            ..Running::default()
        };
        assert_eq!(runs_program(std::slice::from_ref(&mine), &this), Ok(()));
        let mut child = Command::new("sleep")
            .arg("600")
            .stdin(Stdio::null())
            .spawn()
            .expect("sleep");
        let sleeping = Running {
            pid: child.id(),
            args: vec!["sleep".into(), "600".into()],
            ..Running::default()
        };
        let ran = runs_program(std::slice::from_ref(&sleeping), &this);
        let scratch = std::env::temp_dir().join(format!("flume-common-{}", std::process::id()));
        fs::create_dir_all(&scratch).expect("a scratch folder");
        let script = scratch.join("agent");
        fs::write(&script, "#!/bin/sh\n").expect("a script");
        let by_script = Running {
            pid: child.id(),
            args: vec!["/bin/sh".into(), "./agent".into()],
            cwd: Some(scratch.clone()),
            ..Running::default()
        };
        let found = runs_program(&[by_script], &script);
        let _ = child.kill();
        let _ = child.wait();
        let _ = fs::remove_dir_all(&scratch);
        let ran = ran.expect_err("sleep is not this binary");
        assert_eq!(ran.len(), 1);
        assert!(ran[0].contains("sleep 600"), "{ran:?}");
        assert_eq!(found, Ok(()));
        assert_eq!(runs_program(&[], &this), Err(vec![]));
    }

    #[test]
    fn a_bases_label_is_its_repositorys_folder_and_its_base() {
        let place = PlaceRef {
            repository: "/home/me/acme".into(),
            location: "/state/places/r/fix--one".into(),
            branch: "fix/one".into(),
            base: "bolt/the-surfaces".into(),
        };
        assert_eq!(base_label(&place), "acme bolt/the-surfaces");
    }

    #[test]
    fn a_digest_is_twelve_hex_digits_of_the_texts_sha256() {
        assert_eq!(digest("abc"), "ba7816bf8f01");
        assert_eq!(digest("r1/fix--one/code/1").len(), 12);
        assert_ne!(digest("a"), digest("b"));
    }

    #[test]
    fn a_word_is_quoted_whole_for_a_shell() {
        assert_eq!(quoted("plain"), "'plain'");
        assert_eq!(quoted("it's $HOME"), "'it'\\''s $HOME'");
        let output = Command::new("/bin/sh")
            .arg("-c")
            .arg(format!("printf %s {}", quoted("a 'b'\n$c \u{e9}")))
            .output()
            .expect("sh runs");
        assert_eq!(String::from_utf8_lossy(&output.stdout), "a 'b'\n$c \u{e9}");
    }

    #[test]
    fn a_record_is_written_and_read_back() {
        let scratch =
            std::env::temp_dir().join(format!("flume-common-json-{}", std::process::id()));
        let path = scratch.join("agents/one.json");
        assert_eq!(read_json(&path), None);
        write_json(&path, &json!({"block": "b1", "socket": "/s"})).expect("written");
        assert_eq!(
            read_json(&path),
            Some(json!({"block": "b1", "socket": "/s"}))
        );
        fs::write(&path, "not json").expect("overwritten");
        assert_eq!(read_json(&path), None);
        let blocked = scratch.join("file");
        fs::write(&blocked, "").expect("a file");
        let refused = write_json(&blocked.join("under"), &json!({}));
        assert!(
            refused.is_err_and(|words| words.contains(&blocked.display().to_string())),
            "a record under a file was written"
        );
        let _ = fs::remove_dir_all(&scratch);
    }
}
