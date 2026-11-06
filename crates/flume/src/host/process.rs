//! The host's process table and a program found on a `PATH`, read by any
//! profile that asks what a process runs: the terminal's, for the program
//! in a pane's foreground (32; record 0018), and the agent program's, for
//! the program its hook runs under (161); and a program run to its end
//! under a bound, as a window's profile runs the program that hands the
//! display its window (189) and the agent program's lists the host's
//! processes (207). No tool is named here (core.1), and no
//! profile calls another for it (core.2).

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// What a program run under a bound answered: whether it exited
/// succeeding, with its errors' words, or no exit within the bound.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Bounded {
    Exited { success: bool, errors: String },
    Unanswered,
}

/// `command` run to its end within `bound`, in a process group of its
/// own, its input and output closed and its errors read: its exit, or,
/// once the bound has passed, its group ended and nothing left waiting
/// on it; the words it could not be run with where it could not (189;
/// core.13).
pub fn run_bounded(command: Command, bound: Duration) -> Result<Bounded, String> {
    Ok(match bounded(command, bound, false)? {
        Some((success, errors, _)) => Bounded::Exited { success, errors },
        None => Bounded::Unanswered,
    })
}

/// What a program run under a bound printed: its output where it exited
/// succeeding within `bound`, run as [`run_bounded`] runs one; none where
/// it failed, gave no answer within the bound or could not be run (207).
pub fn output_bounded(command: Command, bound: Duration) -> Option<String> {
    match bounded(command, bound, true) {
        Ok(Some((true, _, output))) => Some(output),
        _ => None,
    }
}

/// Whether `command` exited succeeding within `bound`, with its errors'
/// words and, where `output` is asked, what it printed; none once the
/// bound has passed and its group was ended.
fn bounded(
    mut command: Command,
    bound: Duration,
    output: bool,
) -> Result<Option<(bool, String, String)>, String> {
    command
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(if output {
            Stdio::piped()
        } else {
            Stdio::null()
        });
    std::os::unix::process::CommandExt::process_group(&mut command, 0);
    let mut child = command.spawn().map_err(|error| format!("{error}"))?;
    let read = |pipe: Option<Box<dyn Read + Send>>| {
        pipe.map(|mut pipe| {
            std::thread::spawn(move || {
                let mut text = String::new();
                let _ = pipe.read_to_string(&mut text);
                text
            })
        })
    };
    let errors = read(child.stderr.take().map(|pipe| Box::new(pipe) as _));
    let printed = read(child.stdout.take().map(|pipe| Box::new(pipe) as _));
    let joined = |reading: Option<std::thread::JoinHandle<String>>| {
        reading
            .map(|read| read.join().unwrap_or_default())
            .unwrap_or_default()
    };
    let deadline = Instant::now() + bound;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let errors = joined(errors);
                return Ok(Some((
                    status.success(),
                    errors.trim().to_string(),
                    joined(printed),
                )));
            }
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(10)),
            Ok(None) => {
                if let Ok(group) = libc::pid_t::try_from(child.id()) {
                    // SAFETY: a signal sent to the process group the child
                    // leads, which this command made for it.
                    unsafe { libc::kill(-group, libc::SIGKILL) };
                }
                let _ = child.wait();
                return Ok(None);
            }
            Err(error) => return Err(format!("{error}")),
        }
    }
}

/// The program `program` names, by its path: as named where it holds a
/// `/`, else the first file of that name on `path`'s folders (32).
pub fn program_path(program: &str, path: &str) -> Option<PathBuf> {
    if program.contains('/') {
        return Some(PathBuf::from(program));
    }
    path.split(':')
        .filter(|dir| !dir.is_empty())
        .map(|dir| Path::new(dir).join(program))
        .find(|candidate| candidate.is_file())
}

/// The file a process runs, from the host's process table; none where it
/// cannot be read.
#[cfg(target_os = "macos")]
pub fn executable(pid: u32) -> Option<PathBuf> {
    let mut buffer = vec![0u8; libc::PROC_PIDPATHINFO_MAXSIZE as usize];
    // SAFETY: the buffer is as long as `PROC_PIDPATHINFO_MAXSIZE`, as
    // `proc_pidpath` asks.
    let length = unsafe {
        libc::proc_pidpath(
            pid as libc::c_int,
            buffer.as_mut_ptr().cast(),
            buffer.len() as u32,
        )
    };
    (length > 0).then(|| {
        buffer.truncate(length as usize);
        PathBuf::from(String::from_utf8_lossy(&buffer).into_owned())
    })
}

/// The file a process runs, from the host's process table; none where it
/// cannot be read.
#[cfg(not(target_os = "macos"))]
pub fn executable(pid: u32) -> Option<PathBuf> {
    std::fs::read_link(format!("/proc/{pid}/exe")).ok()
}

/// The file this process's parent runs; none where it cannot be read.
pub fn parent_executable() -> Option<PathBuf> {
    executable(std::os::unix::process::parent_id())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_program_by_its_path_or_on_path() {
        assert_eq!(
            program_path("/opt/agent", ""),
            Some(PathBuf::from("/opt/agent"))
        );
        let scratch = std::env::temp_dir().join(format!("flume-process-{}", std::process::id()));
        let (first, second) = (scratch.join("a"), scratch.join("b"));
        for dir in [&first, &second] {
            std::fs::create_dir_all(dir).expect("a folder");
        }
        std::fs::write(second.join("agent"), "").expect("the program");
        let path = format!("{}::{}", first.display(), second.display());
        assert_eq!(program_path("agent", &path), Some(second.join("agent")));
        assert_eq!(program_path("other", &path), None);
        let _ = std::fs::remove_dir_all(&scratch);
    }

    #[test]
    fn a_program_run_under_a_bound() {
        let run = |script: &str, bound: u64| {
            let mut command = Command::new("/bin/sh");
            command.args(["-c", script]);
            run_bounded(command, Duration::from_secs(bound)).expect("sh runs")
        };
        assert_eq!(
            run("echo 'no such app' >&2; exit 1", 5),
            Bounded::Exited {
                success: false,
                errors: "no such app".into(),
            }
        );
        assert_eq!(
            run("exit 0", 5),
            Bounded::Exited {
                success: true,
                errors: String::new(),
            }
        );
        let asked = Instant::now();
        assert_eq!(run("sleep 30; exit 0", 1), Bounded::Unanswered);
        assert!(asked.elapsed() < Duration::from_secs(10), "the bound held");
        assert!(run_bounded(Command::new("/no/such/program"), Duration::from_secs(1)).is_err());
    }

    #[test]
    fn a_programs_output_read_under_a_bound() {
        let read = |script: &str, bound: u64| {
            let mut command = Command::new("/bin/sh");
            command.args(["-c", script]);
            output_bounded(command, Duration::from_secs(bound))
        };
        assert_eq!(read("echo one; echo two", 5), Some("one\ntwo\n".into()));
        assert_eq!(read("echo printed; exit 1", 5), None);
        let asked = Instant::now();
        assert_eq!(read("echo early; sleep 30", 1), None);
        assert!(asked.elapsed() < Duration::from_secs(10), "the bound held");
        assert_eq!(
            output_bounded(Command::new("/no/such/program"), Duration::from_secs(1)),
            None
        );
    }

    #[test]
    fn this_process_and_its_parent_are_read() {
        let this = executable(std::process::id()).expect("this process's program");
        assert_eq!(
            this.canonicalize().ok(),
            std::env::current_exe()
                .and_then(|exe| exe.canonicalize())
                .ok()
        );
        assert!(parent_executable().is_some(), "the parent's program");
    }
}
