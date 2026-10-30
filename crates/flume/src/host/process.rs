//! The host's process table and a program found on a `PATH`, read by any
//! profile that asks what a process runs: the terminal's, for the program
//! in a pane's foreground (32; record 0018), and the agent program's, for
//! the program its hook runs under (161). No tool is named here (core.1),
//! and no profile calls another for it (core.2).

use std::path::{Path, PathBuf};

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
