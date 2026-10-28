//! The host's facts and git's history, read for real (design 11;
//! history-and-host spec): the system clock, a sleep, a secret of 32
//! random bytes, a file's write time, a command of flume's named by its
//! process and that process's start, the bound the host's configuration
//! states, the recipient the calling command's environment names, and
//! the room lock at `$XDG_STATE_HOME/flume/room.lock` (41, 70, 75; design
//! 6, 10). No tool is named here (core.1).

pub mod config;
pub mod history;

use config::HostConfig;
use flume_seams::{Host, RoomGuard, Seconds, Secret};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// The host this command runs on.
#[derive(Clone, Debug)]
pub struct RealHost {
    state: PathBuf,
    config: HostConfig,
    recipient: Option<String>,
}

/// The variable a caller names a launch's default recipient in (7).
pub const RECIPIENT: &str = "FLUME_RECIPIENT";

impl RealHost {
    /// The host over flume's state directory `state`, its configuration
    /// `config`, and the recipient the command's environment names.
    pub fn new(
        state: impl Into<PathBuf>,
        config: HostConfig,
        recipient: Option<String>,
    ) -> RealHost {
        RealHost {
            state: state.into(),
            config,
            recipient,
        }
    }

    /// The host as this command finds it: `$XDG_STATE_HOME/flume`, the
    /// configuration in `$XDG_CONFIG_HOME/flume/config.toml`, and
    /// `$FLUME_RECIPIENT`.
    pub fn of_host() -> Result<RealHost, String> {
        Ok(RealHost::new(
            config::flume_state()?,
            HostConfig::of_host()?,
            std::env::var(RECIPIENT)
                .ok()
                .filter(|recipient| !recipient.is_empty()),
        ))
    }

    /// The room lock's file.
    pub fn room_lock(&self) -> PathBuf {
        self.state.join("room.lock")
    }
}

/// A process's start, as the host gives it, which no other process with
/// its id has had while it lives; none where it is not running.
#[cfg(target_os = "macos")]
fn started(pid: u32) -> Option<String> {
    let mut info: libc::proc_bsdinfo = unsafe { std::mem::zeroed() };
    let size = std::mem::size_of::<libc::proc_bsdinfo>() as libc::c_int;
    // SAFETY: the buffer is a zeroed `proc_bsdinfo` of the size given, as
    // `PROC_PIDTBSDINFO` asks.
    let read = unsafe {
        libc::proc_pidinfo(
            pid as libc::c_int,
            libc::PROC_PIDTBSDINFO,
            0,
            (&mut info as *mut libc::proc_bsdinfo).cast(),
            size,
        )
    };
    (read == size).then(|| format!("{}.{:06}", info.pbi_start_tvsec, info.pbi_start_tvusec))
}

/// A process's start, as the host gives it: the clock ticks since boot
/// at which it started, from `/proc`; none where it is not running.
#[cfg(not(target_os = "macos"))]
fn started(pid: u32) -> Option<String> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    // The command's name, in parentheses, may hold spaces: the fields
    // count from its close.
    let after = &stat[stat.rfind(')')? + 1..];
    after.split_whitespace().nth(19).map(str::to_string)
}

/// The name of the command of flume's running as process `pid`: its id
/// and its process's start, none where no process runs as `pid`.
pub fn command_of(pid: u32) -> Option<String> {
    started(pid).map(|start| format!("{pid}@{start}"))
}

/// This command's name: its process id and its process's start.
fn this_command() -> &'static str {
    static COMMAND: OnceLock<String> = OnceLock::new();
    COMMAND.get_or_init(|| {
        let pid = std::process::id();
        command_of(pid).unwrap_or_else(|| format!("{pid}@"))
    })
}

/// A file's modification time by the host's clock.
fn modified(path: &Path) -> Option<Seconds> {
    let time = std::fs::metadata(path).ok()?.modified().ok()?;
    time.duration_since(UNIX_EPOCH)
        .ok()
        .map(|since| since.as_secs())
}

impl Host for RealHost {
    fn now(&self) -> Seconds {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_secs())
    }

    fn sleep(&self, seconds: Seconds) {
        std::thread::sleep(Duration::from_secs(seconds));
    }

    fn secret(&self) -> Secret {
        let mut bytes = [0u8; 32];
        File::open("/dev/urandom")
            .and_then(|mut random| random.read_exact(&mut bytes))
            .expect("the host gives random bytes from /dev/urandom");
        Secret(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
    }

    fn write_time(&self, path: &str) -> Option<Seconds> {
        modified(Path::new(path))
    }

    fn command(&self) -> String {
        this_command().to_string()
    }

    fn alive(&self, command: &str) -> bool {
        let Some((pid, start)) = command.split_once('@') else {
            return false;
        };
        let Ok(pid) = pid.parse::<u32>() else {
            return false;
        };
        started(pid).is_some_and(|now| now == start)
    }

    fn bound(&self) -> Option<u32> {
        self.config.agents
    }

    fn recipient(&self) -> Option<String> {
        self.recipient.clone()
    }

    fn room(&self) -> RoomGuard {
        let path = self.room_lock();
        let file = path
            .parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .and_then(|()| {
                OpenOptions::new()
                    .create(true)
                    .truncate(false)
                    .write(true)
                    .open(&path)
            })
            .and_then(|file| file.lock().map(|()| file))
            .unwrap_or_else(|error| {
                panic!("the room lock {} cannot be taken: {error}", path.display())
            });
        RoomGuard::new(file)
    }
}
