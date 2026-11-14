//! The host's facts and git's history, read for real (design 11;
//! history-and-host spec): the system clock, a sleep, a secret of 32
//! random bytes, a file's write time, a command of flume's named by its
//! process and that process's start, the bound and the agents' settings
//! the host's configuration states, the role runs, streams, pools and
//! labels it names, its values with their layers, an account's reading
//! from the file its `usage` names, the sources of kinds it declares and
//! its lock of kinds beside them, the recipient the calling command's
//! environment names, the host user's login environment, the room lock
//! at `$XDG_STATE_HOME/flume/room.lock`, and the lock `flume kinds lock`
//! holds while it writes a lock of kinds, at
//! `$XDG_STATE_HOME/flume/kinds-lock.lock` (41, 70, 75, 155, 184, 191;
//! design 6, 10). No tool is named here (core.1).

pub mod config;
pub mod history;
pub mod process;
pub mod roles;
pub mod usage;

use config::HostConfig;
use flume_seams::{
    Host, HostConfiguration, HostRoles, HostSettings, HostSource, LabelHeld, Pool, PoolNamed,
    RepositoryName, RoomGuard, Seconds, Secret, Stream, StreamDisplay, StreamHeld, TelemetryTable,
    Usage,
};
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// The system a profile is built for: macOS, or any other, so a profile
/// whose program differs between them holds both paths on any host (C.8).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum System {
    MacOs,
    Other,
}

impl System {
    /// The system this host runs.
    pub fn of_host() -> System {
        if cfg!(target_os = "macos") {
            System::MacOs
        } else {
            System::Other
        }
    }
}

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
    /// configuration in `$XDG_CONFIG_HOME/flume/config.toml` and
    /// `config.local.toml` over it, and `$FLUME_RECIPIENT`.
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

/// The file of the advisory lock `flume kinds lock` holds, under flume's
/// state directory (155; design 6).
pub const WRITING_KINDS_LOCK: &str = "kinds-lock.lock";

/// The advisory lock `flume kinds lock` holds while it reads and writes a
/// lock of kinds, the host's or a working copy's, so two writers on the
/// host never lose each other's entry; it lies under flume's state
/// directory, so nothing but the lock is written in a working copy, and
/// is held while the file lives (74, 155; design 6; core.16, core.17).
pub fn writing_kinds_lock() -> Result<File, String> {
    let path = config::flume_state()?.join(WRITING_KINDS_LOCK);
    path.parent()
        .map_or(Ok(()), fs::create_dir_all)
        .and_then(|()| {
            OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .open(&path)
        })
        .and_then(|file| file.lock().map(|()| file))
        .map_err(|error| format!("the lock {} cannot be taken: {error}", path.display()))
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

    fn ends_expire(&self) -> Seconds {
        self.config.ends_expire
    }

    fn telemetry(&self) -> Option<TelemetryTable> {
        self.config.telemetry.clone()
    }

    fn recipient(&self) -> Option<String> {
        self.recipient.clone()
    }

    /// The configuration's settings as this command read it: the engine
    /// is built for each command, so each start reads the host's
    /// configuration as it stands (32, C.7; record 0017).
    /// Every repository-keyed table read under the one key
    /// [`HostConfig::table_for`] gives, the address's or the folder's,
    /// never the two merged (32, 143, C.7).
    fn settings(&self, repository: &RepositoryName, kind: &str, stream: &str) -> HostSettings {
        let config = &self.config;
        let repository = config.table_for(repository);
        HostSettings {
            repository: config
                .repos
                .get(repository)
                .and_then(|kinds| kinds.get(kind))
                .cloned()
                .unwrap_or_default(),
            every_kind: config
                .repos_agents
                .get(repository)
                .cloned()
                .unwrap_or_default(),
            kind: config.kinds.get(kind).cloned().unwrap_or_default(),
            stream: config
                .streams_agents
                .get(stream)
                .cloned()
                .unwrap_or_default(),
            every_agent: config.every_agent.clone(),
        }
    }

    /// What the sources declare of roles, each source's roles file read
    /// at each call, so a file its tool renamed over the last is read as
    /// it stands (132, 191, 193, C.7).
    fn roles(&self) -> HostRoles {
        roles::read(&self.config.sources, &self.config.streams)
    }

    fn streams(&self) -> BTreeMap<String, Stream> {
        self.config.streams.clone()
    }

    fn stream_for(&self, repository: &RepositoryName, kind: &str) -> Option<StreamHeld> {
        self.config.stream_for(repository, kind)
    }

    fn label_for(&self, repository: &RepositoryName, kind: &str) -> Option<LabelHeld> {
        self.config.label_for(repository, kind)
    }

    /// The configuration as this command read it, its managed layer and
    /// its local one over it, every value with its layer (56, C.7).
    fn configuration(&self) -> HostConfiguration {
        self.config.configuration.clone()
    }

    fn pool_for(&self, stream: &str) -> Option<PoolNamed> {
        self.config.pool_for(stream)
    }

    fn display_for(&self, stream: &str) -> StreamDisplay {
        self.config.display_for(stream)
    }

    fn pool(&self, name: &str) -> Option<Pool> {
        self.config.pools.get(name).cloned()
    }

    /// The reading in the file at `path` as it stands now, read and never
    /// written (184, C.7).
    fn usage(&self, path: &str) -> Result<Usage, String> {
        usage::read(Path::new(path))
    }

    fn login(&self, name: &str) -> Option<String> {
        self.login_environment().get(name).cloned()
    }

    /// The sources as this command read the configuration: each command
    /// reads them as they stand (159, 191, C.7).
    fn sources(&self) -> BTreeMap<String, HostSource> {
        self.config.sources.clone()
    }

    /// `sources/` beside `config.toml` (191, C.7).
    fn sources_dir(&self) -> Option<String> {
        let dir = self.config.dir.as_ref()?;
        Some(dir.join(config::SOURCES).to_string_lossy().into_owned())
    }

    /// `flume.lock` beside `config.toml` (155, C.7).
    fn kinds_lock(&self) -> Option<String> {
        let dir = self.config.dir.as_ref()?;
        Some(dir.join(config::KINDS_LOCK).to_string_lossy().into_owned())
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
