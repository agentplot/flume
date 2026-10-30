//! The host's facts and git's history, read for real (design 11;
//! history-and-host spec): the system clock, a sleep, a secret of 32
//! random bytes, a file's write time, a command of flume's named by its
//! process and that process's start, the bound and the agents' settings
//! the host's configuration states, the role runs, streams and pools it
//! names, the recipient the calling command's
//! environment names, the host user's login environment, and the room
//! lock at `$XDG_STATE_HOME/flume/room.lock` (41, 70, 75; design
//! 6, 10). No tool is named here (core.1).

pub mod config;
pub mod history;
pub mod process;

use config::HostConfig;
use flume_seams::{
    Host, HostSettings, Pool, PoolNamed, RoleRun, RoomGuard, Seconds, Secret, Stream, StreamHeld,
};
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// The host this command runs on.
#[derive(Clone, Debug)]
pub struct RealHost {
    state: PathBuf,
    config: HostConfig,
    recipient: Option<String>,
    login: Arc<OnceLock<BTreeMap<String, String>>>,
}

/// The variables of the host user's own a login shell starts from, all
/// else cleared, so nothing of the calling command's session reaches the
/// login environment (29).
const LOGIN_NAMES: [&str; 6] = ["HOME", "USER", "LOGNAME", "SHELL", "TMPDIR", "LANG"];

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
            login: Arc::default(),
        }
    }

    /// The host with exactly `variables` for the host user's login
    /// environment.
    pub fn with_login(self, variables: &[(String, String)]) -> RealHost {
        let login = OnceLock::new();
        let _ = login.set(variables.iter().cloned().collect());
        RealHost {
            login: Arc::new(login),
            ..self
        }
    }

    /// The host user's login environment, read once (29).
    fn login_environment(&self) -> &BTreeMap<String, String> {
        self.login.get_or_init(login_environment)
    }

    /// The host as this command finds it: `$XDG_STATE_HOME/flume`, the
    /// configuration in `$XDG_CONFIG_HOME/flume/config.toml`, and
    /// `$FLUME_RECIPIENT`.
    pub fn of_host() -> Result<RealHost, String> {
        Ok(RealHost::new(
            config::flume_state()?,
            HostConfig::of_host()?,
            recipient(),
        ))
    }

    /// The room lock's file.
    pub fn room_lock(&self) -> PathBuf {
        self.state.join("room.lock")
    }
}

/// The host user's login environment: what their login shell sets,
/// started from their own variables alone; those variables alone where
/// the shell cannot be run (29). Each read runs the shell again, so a
/// caller keeps what it read.
pub fn login_environment() -> BTreeMap<String, String> {
    let own: BTreeMap<String, String> = LOGIN_NAMES
        .iter()
        .filter_map(|name| {
            std::env::var(name)
                .ok()
                .map(|value| (name.to_string(), value))
        })
        .collect();
    let shell = own
        .get("SHELL")
        .cloned()
        .unwrap_or_else(|| "/bin/sh".into());
    let read = Command::new(&shell)
        .args(["-l", "-c", "env -0"])
        .env_clear()
        .envs(&own)
        .output()
        .ok()
        .filter(|output| output.status.success());
    match read {
        Some(output) => output
            .stdout
            .split(|byte| *byte == 0)
            .filter_map(|pair| {
                let pair = String::from_utf8_lossy(pair);
                pair.split_once('=')
                    .map(|(name, value)| (name.to_string(), value.to_string()))
            })
            .collect(),
        None => own,
    }
}

/// The recipient the command's environment names in `$FLUME_RECIPIENT`,
/// none where it is unset or empty (7).
pub fn recipient() -> Option<String> {
    std::env::var(RECIPIENT)
        .ok()
        .filter(|recipient| !recipient.is_empty())
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

    /// The configuration's settings as this command read it: the engine
    /// is built for each command, so each start reads the host's
    /// configuration as it stands (32, C.7; record 0017).
    fn settings(&self, repository: &str, kind: &str) -> HostSettings {
        HostSettings {
            repository: self
                .config
                .repos
                .get(repository)
                .and_then(|kinds| kinds.get(kind))
                .cloned()
                .unwrap_or_default(),
            kind: self.config.kinds.get(kind).cloned().unwrap_or_default(),
        }
    }

    fn role(&self, address: &str) -> Option<RoleRun> {
        self.config.roles.get(address).cloned()
    }

    fn streams(&self) -> BTreeMap<String, Stream> {
        self.config.streams.clone()
    }

    fn stream_for(&self, repository: &str, kind: &str) -> Option<StreamHeld> {
        self.config.stream_for(repository, kind)
    }

    fn pool_for(&self, stream: &str) -> Option<PoolNamed> {
        self.config.pool_for(stream)
    }

    fn pool(&self, name: &str) -> Option<Pool> {
        self.config.pools.get(name).cloned()
    }

    fn login(&self, name: &str) -> Option<String> {
        self.login_environment().get(name).cloned()
    }

    /// A line appended to `$XDG_STATE_HOME/flume/flume.log` with the
    /// host's clock; a log that cannot be written loses the line and
    /// fails nothing (139).
    fn log(&self, line: &str) {
        let path = self.state.join("flume.log");
        let now = self.now();
        let _ = fs::create_dir_all(&self.state).and_then(|()| {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .and_then(|mut file| writeln!(file, "{now} {line}"))
        });
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
