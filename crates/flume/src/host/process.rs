//! The host's process table and a program found on a `PATH`, read by any
//! profile that asks what a process runs: the terminal's, for the program
//! in a pane's foreground (32; record 0018), and the agent program's, for
//! the program its hook runs under (161); and a program run to its end
//! under a bound, as a window's profile runs the program that hands the
//! display its window (189); and every process of the host's user, as
//! the agent program's profile reads which of them runs a session (207). No tool is named here (core.1), and no
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
pub fn run_bounded(mut command: Command, bound: Duration) -> Result<Bounded, String> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    std::os::unix::process::CommandExt::process_group(&mut command, 0);
    let mut child = command.spawn().map_err(|error| format!("{error}"))?;
    let errors = child.stderr.take().map(|mut pipe| {
        std::thread::spawn(move || {
            let mut text = String::new();
            let _ = pipe.read_to_string(&mut text);
            text
        })
    });
    let deadline = Instant::now() + bound;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let errors = errors
                    .map(|read| read.join().unwrap_or_default())
                    .unwrap_or_default();
                return Ok(Bounded::Exited {
                    success: status.success(),
                    errors: errors.trim().to_string(),
                });
            }
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(10)),
            Ok(None) => {
                if let Ok(group) = libc::pid_t::try_from(child.id()) {
                    // SAFETY: a signal sent to the process group the child
                    // leads, which this command made for it.
                    unsafe { libc::kill(-group, libc::SIGKILL) };
                }
                let _ = child.wait();
                return Ok(Bounded::Unanswered);
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

/// A process of the host as its process table gives it: its id, when it
/// started by the host's clock in seconds since the epoch, and its
/// arguments, as the process holds them now (207).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Listed {
    pub pid: u32,
    pub started: u64,
    pub args: Vec<String>,
}

/// Each process of the host this process may read, the host user's own
/// among them, from the process table itself, which a sandbox that
/// refuses to run a setuid `ps`, as a merge queue's checks run under,
/// lets it read; a process that ended while it was read left out, and
/// one whose arguments cannot be read listed with none; none where the
/// table cannot be listed (207).
#[cfg(target_os = "macos")]
pub fn listed() -> Option<Vec<Listed>> {
    // SAFETY: a null buffer of no length asks only for the count.
    let count = unsafe { libc::proc_listallpids(std::ptr::null_mut(), 0) };
    let count = usize::try_from(count).ok().filter(|count| *count > 0)?;
    // Room for the processes started since the count.
    let mut pids: Vec<libc::pid_t> = vec![0; count + count / 2 + 64];
    let size = libc::c_int::try_from(std::mem::size_of_val(pids.as_slice())).ok()?;
    // SAFETY: the buffer holds `size` bytes, as the call is told.
    let found = unsafe { libc::proc_listallpids(pids.as_mut_ptr().cast(), size) };
    let found = usize::try_from(found).ok().filter(|found| *found > 0)?;
    pids.truncate(found.min(pids.len()));
    Some(
        pids.into_iter()
            .filter_map(|pid| {
                let started = macos::started(pid)?;
                Some(Listed {
                    pid: u32::try_from(pid).ok().filter(|pid| *pid > 0)?,
                    started,
                    args: macos::args(pid).unwrap_or_default(),
                })
            })
            .collect(),
    )
}

#[cfg(target_os = "macos")]
mod macos {
    /// When the process `pid` started, from its BSD information.
    pub fn started(pid: libc::pid_t) -> Option<u64> {
        // SAFETY: proc_bsdinfo is plain data, for which all zeroes is a value.
        let mut info: libc::proc_bsdinfo = unsafe { std::mem::zeroed() };
        let size = libc::c_int::try_from(std::mem::size_of::<libc::proc_bsdinfo>()).ok()?;
        // SAFETY: the buffer is one proc_bsdinfo, `size` bytes, as the call
        // is told.
        let read = unsafe {
            libc::proc_pidinfo(pid, libc::PROC_PIDTBSDINFO, 0, (&raw mut info).cast(), size)
        };
        (read == size).then_some(info.pbi_start_tvsec)
    }

    /// The process `pid`'s arguments, from `KERN_PROCARGS2`: the count of
    /// arguments, the program's path, then each argument, every one ended
    /// by a NUL.
    pub fn args(pid: libc::pid_t) -> Option<Vec<String>> {
        let mut mib = [libc::CTL_KERN, libc::KERN_PROCARGS2, pid];
        let mut size: libc::size_t = 0;
        // SAFETY: a null buffer asks only for the size.
        let asked = unsafe {
            libc::sysctl(
                mib.as_mut_ptr(),
                3,
                std::ptr::null_mut(),
                &mut size,
                std::ptr::null_mut(),
                0,
            )
        };
        if asked != 0 || size < 4 {
            return None;
        }
        let mut buffer = vec![0u8; size];
        // SAFETY: the buffer holds `size` bytes, as the call is told.
        let read = unsafe {
            libc::sysctl(
                mib.as_mut_ptr(),
                3,
                buffer.as_mut_ptr().cast(),
                &mut size,
                std::ptr::null_mut(),
                0,
            )
        };
        if read != 0 || size < 4 {
            return None;
        }
        buffer.truncate(size);
        let count = usize::try_from(i32::from_ne_bytes(buffer[..4].try_into().ok()?)).ok()?;
        let rest = &buffer[4..];
        let path_end = rest.iter().position(|byte| *byte == 0)?;
        let rest = &rest[path_end..];
        let start = rest.iter().position(|byte| *byte != 0)?;
        Some(
            rest[start..]
                .split(|byte| *byte == 0)
                .take(count)
                .map(|arg| String::from_utf8_lossy(arg).into_owned())
                .collect(),
        )
    }
}

/// Each process of the host this process may read, from `/proc`: its
/// start from its `stat`'s clock ticks after the boot `/proc/stat` names,
/// and its arguments from its `cmdline`; a process that ended while it was
/// read left out; none where `/proc` cannot be listed (207).
#[cfg(target_os = "linux")]
pub fn listed() -> Option<Vec<Listed>> {
    let boot: u64 = std::fs::read_to_string("/proc/stat")
        .ok()?
        .lines()
        .find_map(|line| line.strip_prefix("btime ")?.trim().parse().ok())?;
    // SAFETY: sysconf reads a value of the system's and changes nothing.
    let ticks = u64::try_from(unsafe { libc::sysconf(libc::_SC_CLK_TCK) })
        .ok()
        .filter(|ticks| *ticks > 0)?;
    let entries = std::fs::read_dir("/proc").ok()?;
    Some(
        entries
            .flatten()
            .filter_map(|entry| {
                let pid: u32 = entry.file_name().to_str()?.parse().ok()?;
                let stat = std::fs::read_to_string(entry.path().join("stat")).ok()?;
                // The fields after the program's name, which may hold any
                // character, `)` among them: the state is the third field,
                // and the start the twenty-second.
                let after = &stat[stat.rfind(')')? + 1..];
                let since_boot: u64 = after.split_whitespace().nth(19)?.parse().ok()?;
                let args = std::fs::read(entry.path().join("cmdline"))
                    .map(|bytes| {
                        bytes
                            .split(|byte| *byte == 0)
                            .filter(|arg| !arg.is_empty())
                            .map(|arg| String::from_utf8_lossy(arg).into_owned())
                            .collect()
                    })
                    .unwrap_or_default();
                Some(Listed {
                    pid,
                    started: boot + since_boot / ticks,
                    args,
                })
            })
            .collect(),
    )
}

/// The host's processes, on a system whose process table flume does not
/// read: none.
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn listed() -> Option<Vec<Listed>> {
    None
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
    fn this_process_and_a_child_are_listed() {
        // Only arguments every sleep takes: GNU's refuses a word that is no
        // interval and exits at once, its arguments gone from the table.
        let mut child = Command::new("sleep")
            .arg("600")
            .stdin(Stdio::null())
            .spawn()
            .expect("sleep");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("after the epoch")
            .as_secs();
        let processes = listed().expect("the process table");
        let _ = child.kill();
        let _ = child.wait();
        let this = processes
            .iter()
            .find(|process| process.pid == std::process::id())
            .expect("this process listed");
        assert!(this.started <= now && this.started + 3600 > now, "{this:?}");
        let sleeping = processes
            .iter()
            .find(|process| process.pid == child.id())
            .expect("the child listed");
        assert_eq!(sleeping.args, ["sleep", "600"]);
        assert!(sleeping.started + 60 > now && sleeping.started <= now + 1);
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
