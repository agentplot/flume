//! The terminal profile over Rex (C.5) against the pinned Rex, on servers
//! each test starts under a state folder of its own, short enough that
//! every socket under it fits the 104 bytes a socket's path holds, and
//! stops at its end, so no Rex server of the host's, the Rex app's among
//! them, is touched and no `REX_*` of the caller's reaches one (core.29,
//! core.51).

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

/// How many state folders this run has made, which names the next.
static FOLDERS: AtomicU32 = AtomicU32::new(0);

/// The pinned Rex, by its own path: the first `rex` on the test's `PATH`,
/// which the devenv shell leads with the release devenv.lock pins. A run
/// with none fails naming it, and never passes without it (core.22,
/// core.51).
fn rex_program() -> PathBuf {
    let path = std::env::var("PATH").unwrap_or_default();
    path.split(':')
        .filter(|dir| !dir.is_empty())
        .map(|dir| Path::new(dir).join("rex"))
        .find(|candidate| candidate.is_file())
        .and_then(|found| found.canonicalize().ok())
        .expect(
            "the pinned rex on PATH: the devenv shell gives it where Rex's flake packages Rex, \
             and these tests run nowhere else",
        )
}

/// A folder short enough for a state folder whose sockets fit: the
/// temporary folder, which a sandboxed run may write and which sluice
/// keeps short, or `/tmp` where the temporary folder is long.
fn short_folder() -> PathBuf {
    let temporary = std::env::temp_dir();
    if temporary.as_os_str().len() <= 40 {
        temporary
    } else {
        PathBuf::from("/tmp")
    }
}

/// A state folder of the test's own, as flume's is, its Rex profile's
/// records and sockets under `terminal/rex`; every Rex server listening
/// under it stopped, and the folder removed, when it is dropped.
struct RexState {
    root: PathBuf,
}

impl RexState {
    fn new() -> RexState {
        let root = short_folder().join(format!(
            "x{}-{}",
            std::process::id(),
            FOLDERS.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("terminal/rex")).unwrap_or_else(|error| {
            panic!(
                "the Rex tests' state folder {} cannot be made: {error}",
                root.display()
            )
        });
        let root = root.canonicalize().expect("the state folder's path");
        RexState { root }
    }

    /// Where the profile keeps its records and its sessions' sockets.
    fn records(&self) -> PathBuf {
        self.root.join("terminal/rex")
    }

    /// The socket of the session `name`, as the profile names it.
    fn socket(&self, name: &str) -> PathBuf {
        self.records().join(format!("{name}.sock"))
    }

    /// The state folder of the session `name`'s server, beside its socket.
    fn server_state(&self, name: &str) -> PathBuf {
        self.records().join(format!("{name}.state"))
    }

    /// Starts the session `name`'s server as the profile does, restoring
    /// nothing, and gives what Rex answered.
    fn start_server(&self, name: &str) -> Value {
        let state = self.server_state(name);
        rex(
            &self.socket(name),
            &[
                "server",
                "start",
                "--state-dir",
                &state.to_string_lossy(),
                "--restore=false",
                "--json",
            ],
        )
    }

    /// Every socket a Rex server may listen on under the folder.
    fn sockets(&self) -> Vec<PathBuf> {
        let mut sockets = vec![];
        let mut folders = vec![self.root.clone()];
        while let Some(folder) = folders.pop() {
            for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
                let path = entry.path();
                let Ok(kind) = entry.file_type() else {
                    continue;
                };
                if kind.is_dir() {
                    folders.push(path);
                } else if path.extension().is_some_and(|ext| ext == "sock") {
                    sockets.push(path);
                }
            }
        }
        sockets
    }
}

impl Drop for RexState {
    fn drop(&mut self) {
        for socket in self.sockets() {
            stop_server(&socket);
        }
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// The server at `socket` stopped through its socket, and its process
/// ended where it does not stop, a frozen one woken first.
fn stop_server(socket: &Path) {
    let pid = try_rex(socket, &["api", "call", "server.status"])
        .ok()
        .and_then(|status| status["pid"].as_i64());
    let pid = pid.or_else(|| server_pid(socket));
    if let Some(pid) = pid {
        // SAFETY: a signal to a server this test's folder holds.
        unsafe { libc::kill(pid as libc::pid_t, libc::SIGCONT) };
    }
    let _ = try_rex(socket, &["server", "stop"]);
    if let Some(pid) = pid {
        let deadline = Instant::now() + Duration::from_secs(10);
        // SAFETY: signal 0 asks only whether the process stands.
        while unsafe { libc::kill(pid as libc::pid_t, 0) } == 0 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(100));
        }
        // SAFETY: a signal to a server this test's folder holds.
        unsafe { libc::kill(pid as libc::pid_t, libc::SIGKILL) };
    }
}

/// The process of the server listening at `socket`, by the pid a start
/// of the profile's or the test's answered, kept beside it.
fn server_pid(socket: &Path) -> Option<i64> {
    fs::read_to_string(socket.with_extension("pid"))
        .ok()?
        .trim()
        .parse()
        .ok()
}

/// Rex's answer for `args` on the server at `socket`, the caller's
/// `REX_*` stripped and no server started of Rex's choosing; or its
/// words where it refuses or does not answer within ten seconds.
fn try_rex(socket: &Path, args: &[&str]) -> Result<Value, String> {
    let mut command = Command::new(rex_program());
    command
        .arg("--autostart=false")
        .arg("-S")
        .arg(format!("unix://{}", socket.display()))
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("REX_") {
            command.env_remove(name);
        }
    }
    let child = command.spawn().map_err(|error| error.to_string())?;
    let pid = child.id();
    let (sent, got) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = sent.send(child.wait_with_output());
    });
    let output = match got.recv_timeout(Duration::from_secs(10)) {
        Ok(output) => output.map_err(|error| error.to_string())?,
        Err(_) => {
            // SAFETY: a signal to the call this test started.
            unsafe { libc::kill(pid as libc::pid_t, libc::SIGKILL) };
            return Err(format!("rex {} did not answer", args.join(" ")));
        }
    };
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    if !output.status.success() {
        return Err(format!(
            "{text}{}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    if text.trim().is_empty() {
        return Ok(Value::Null);
    }
    serde_json::from_str(text.trim()).or(Ok(Value::String(text)))
}

/// Rex's answer for `args` on the server at `socket`.
fn rex(socket: &Path, args: &[&str]) -> Value {
    try_rex(socket, args).unwrap_or_else(|words| panic!("rex {}: {words}", args.join(" ")))
}

#[test]
fn the_pinned_rex_runs_at_api_revision_twelve_or_later() {
    // core.22, C.5: the release the tree pins starts a server of the
    // test's own on a socket under its state folder, restoring nothing,
    // and answers its status at API revision 12 or later.
    let state = RexState::new();
    let socket = state.socket("flume");
    assert!(
        socket.as_os_str().len() <= 104,
        "the state folder leaves its socket {} bytes",
        socket.as_os_str().len()
    );
    let started = state.start_server("flume");
    fs::write(socket.with_extension("pid"), started["pid"].to_string())
        .expect("the server's pid kept");
    assert!(started["pid"].as_i64().is_some(), "start gave {started}");
    let status = rex(&socket, &["api", "call", "server.status"]);
    let revision = status["api_revision"].as_u64().unwrap_or_default();
    assert!(revision >= 12, "server.status gave {status}");
    assert_eq!(status["pid"], started["pid"], "another server answered");
    assert_eq!(status["session_count"], 0, "the server restored sessions");
}
