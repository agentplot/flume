use crate::{lock, Log};
use flume_seams::{
    ConfigLayer, ConfigValue, Host, HostConfiguration, HostSettings, HostSource, HostSourceRead,
    KindSettings, LabelFrom, LabelHeld, NamedBy, Pool, PoolFrom, PoolNamed, ProfileHeld, RoleRun,
    RoomGuard, Seconds, Secret, Share, Stream, StreamDisplay, StreamFrom, StreamHeld, Usage, View,
    ViewHeld, DEFAULT_STREAM,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Condvar, Mutex};

/// The host: a clock that moves only when a test advances it or a command
/// sleeps on it, numbered secrets, files' write times, the commands of
/// flume's that live, the bound on agents and the agents' settings its
/// configuration states, its streams with their sessions, labels'
/// shares, titles and displays, the host's own display, its pools of
/// accounts, each account's reading by its path, the labels it gives
/// kinds, its configuration's values with their layers, the sources of
/// its kinds with where their files and its lock lie, the recipient the
/// calling command's environment names, the host user's login
/// environment, and an exclusive room lock. It reads no configuration
/// file and no reading's file (106, 184).
pub struct FakeHost {
    state: Mutex<HostState>,
    room: Arc<Room>,
    pub log: Log,
    /// The lines written to the host's log (139).
    pub logged: Log,
}

/// A multiplexer, a view and a terminal app, each named or not.
#[derive(Clone, Default)]
struct Display {
    multiplexer: Option<String>,
    view: Option<View>,
    terminal_app: Option<String>,
}

/// The room lock: taken while held, waited on by every other taker.
#[derive(Default)]
struct Room {
    taken: Mutex<bool>,
    freed: Condvar,
}

/// Holding the room until dropped.
struct Held(Arc<Room>);

impl Drop for Held {
    fn drop(&mut self) {
        *lock(&self.0.taken) = false;
        self.0.freed.notify_one();
    }
}

struct HostState {
    now: Seconds,
    secrets: u64,
    files: BTreeMap<String, Seconds>,
    command: u64,
    alive: BTreeSet<String>,
    bound: Option<u32>,
    recipient: Option<String>,
    /// The agents' settings by kind, for every repository.
    kinds: BTreeMap<String, KindSettings>,
    /// The agents' settings by repository and kind.
    repos: BTreeMap<(String, String), KindSettings>,
    login: BTreeMap<String, String>,
    /// The role runs the configuration names, by address (132, C.7).
    roles: BTreeMap<String, RoleRun>,
    /// The streams the configuration declares, by name (143).
    streams: BTreeMap<String, Stream>,
    /// The stream named for a repository and kind, a repository, or a
    /// kind, by the repository and the kind each names (143).
    streams_named: BTreeMap<(Option<String>, Option<String>), String>,
    /// The host's own pool (144).
    pool: Option<String>,
    /// The host's own multiplexer, view and terminal app (186, 190).
    display: Display,
    /// The pools the configuration declares, by name (144).
    pools: BTreeMap<String, Pool>,
    /// The reading, or why there is none, by the path it is read at
    /// (184).
    usage: BTreeMap<String, Result<Usage, String>>,
    /// The label named for a repository and kind, or a kind, by the
    /// repository, none for every repository, and the kind (180).
    labels_named: BTreeMap<(Option<String>, String), String>,
    /// The configuration's values with their layers, whether its local
    /// layer was taken, and the keys passed over (C.7).
    configuration: HostConfiguration,
    /// The sources the configuration declares, by name (191).
    sources: BTreeMap<String, HostSource>,
}

/// Where the fake host's configuration keeps its sources' files (C.7).
pub const SOURCES: &str = "/home/user/.config/flume/sources";

/// The fake host's lock of kinds (155, C.7).
pub const HOST_LOCK: &str = "/home/user/.config/flume/flume.lock";

/// The file that declares the source `name` on the fake host (191, C.7).
pub fn source_file(name: &str) -> String {
    format!("{SOURCES}/{name}.toml")
}

impl Default for FakeHost {
    fn default() -> Self {
        FakeHost {
            state: Mutex::new(HostState {
                now: 1_000_000,
                secrets: 0,
                files: BTreeMap::new(),
                command: 1,
                alive: BTreeSet::from(["command-1".to_string()]),
                bound: None,
                recipient: None,
                kinds: BTreeMap::new(),
                repos: BTreeMap::new(),
                login: BTreeMap::from([("HOME".to_string(), "/home/user".to_string())]),
                roles: BTreeMap::new(),
                streams: BTreeMap::new(),
                streams_named: BTreeMap::new(),
                pool: None,
                display: Display::default(),
                pools: BTreeMap::new(),
                sources: BTreeMap::new(),
                usage: BTreeMap::new(),
                labels_named: BTreeMap::new(),
                configuration: HostConfiguration::default(),
            }),
            room: Arc::default(),
            log: Log::default(),
            logged: Log::default(),
        }
    }
}

impl FakeHost {
    pub fn advance(&self, seconds: Seconds) {
        lock(&self.state).now += seconds;
    }

    /// A file written at the clock's now.
    pub fn write_file(&self, path: &str) {
        let mut state = lock(&self.state);
        let now = state.now;
        state.files.insert(path.to_string(), now);
    }

    pub fn remove_file(&self, path: &str) {
        lock(&self.state).files.remove(path);
    }

    /// The host restarts: every command of flume's that ran is gone, and
    /// the next runs under a new name.
    pub fn restart(&self) {
        let mut state = lock(&self.state);
        state.alive.clear();
        state.command += 1;
        let next = format!("command-{}", state.command);
        state.alive.insert(next);
    }

    /// Another command of flume's than this one, live.
    pub fn another_command(&self) -> String {
        let mut state = lock(&self.state);
        state.command += 1;
        let name = format!("command-{}", state.command);
        state.alive.insert(name.clone());
        name
    }

    pub fn end_command(&self, command: &str) {
        lock(&self.state).alive.remove(command);
    }

    /// The bound on agents the host's configuration states, or none.
    pub fn set_bound(&self, bound: Option<u32>) {
        lock(&self.state).bound = bound;
    }

    /// The recipient the calling command's environment names, or none.
    pub fn set_recipient(&self, recipient: Option<&str>) {
        lock(&self.state).recipient = recipient.map(str::to_string);
    }

    /// The settings the configuration gives `kind`'s agents, in
    /// `repository` or in every repository where none is named; empty
    /// settings remove them.
    pub fn set_settings(&self, repository: Option<&str>, kind: &str, settings: KindSettings) {
        let mut state = lock(&self.state);
        let empty = settings.is_empty();
        match repository {
            Some(repository) => {
                let key = (repository.to_string(), kind.to_string());
                if empty {
                    state.repos.remove(&key);
                } else {
                    state.repos.insert(key, settings);
                }
            }
            None if empty => {
                state.kinds.remove(kind);
            }
            None => {
                state.kinds.insert(kind.to_string(), settings);
            }
        }
    }

    /// A variable of the host user's login environment, or none; `HOME`
    /// is `/home/user` until set.
    pub fn set_login(&self, name: &str, value: Option<&str>) {
        let mut state = lock(&self.state);
        match value {
            Some(value) => state.login.insert(name.to_string(), value.to_string()),
            None => state.login.remove(name),
        };
    }

    /// The run the configuration names for a role's address, or none.
    pub fn set_role(&self, address: &str, run: Option<RoleRun>) {
        let mut state = lock(&self.state);
        match run {
            Some(run) => state.roles.insert(address.to_string(), run),
            None => state.roles.remove(address),
        };
    }

    /// The stream the configuration declares under `name`, or none.
    pub fn set_stream(&self, name: &str, stream: Option<Stream>) {
        let mut state = lock(&self.state);
        match stream {
            Some(stream) => state.streams.insert(name.to_string(), stream),
            None => state.streams.remove(name),
        };
    }

    /// The title the configuration declares for the stream `name`, or
    /// none, the stream declared with nothing else where it was not (143,
    /// 186).
    pub fn set_title(&self, name: &str, title: Option<&str>) {
        lock(&self.state)
            .streams
            .entry(name.to_string())
            .or_default()
            .title = title.map(str::to_string);
    }

    /// The multiplexer, view and terminal app the configuration names for
    /// the stream `name`, each none where it names none, the stream
    /// declared where it was not (186, 190).
    pub fn set_display(
        &self,
        name: &str,
        multiplexer: Option<&str>,
        view: Option<View>,
        terminal_app: Option<&str>,
    ) {
        let mut state = lock(&self.state);
        let stream = state.streams.entry(name.to_string()).or_default();
        stream.multiplexer = multiplexer.map(str::to_string);
        stream.view = view;
        stream.terminal_app = terminal_app.map(str::to_string);
    }

    /// The host's own multiplexer, view and terminal app, each none where
    /// the configuration names none (186, 190).
    pub fn set_host_display(
        &self,
        multiplexer: Option<&str>,
        view: Option<View>,
        terminal_app: Option<&str>,
    ) {
        lock(&self.state).display = Display {
            multiplexer: multiplexer.map(str::to_string),
            view,
            terminal_app: terminal_app.map(str::to_string),
        };
    }

    /// The stream the configuration names for `kind` in `repository`,
    /// for every kind in `repository` where no kind is given, or for
    /// `kind` in every repository where no repository is; none removes
    /// it (143).
    pub fn name_stream(&self, repository: Option<&str>, kind: Option<&str>, stream: Option<&str>) {
        let mut state = lock(&self.state);
        let key = (repository.map(str::to_string), kind.map(str::to_string));
        match stream {
            Some(stream) => state.streams_named.insert(key, stream.to_string()),
            None => state.streams_named.remove(&key),
        };
    }

    /// The session the configuration names for the stream `name`'s
    /// agents, or none, the stream declared where it was not (107, 143).
    pub fn set_stream_session(&self, name: &str, session: Option<&str>) {
        lock(&self.state)
            .streams
            .entry(name.to_string())
            .or_default()
            .session = session.map(str::to_string);
    }

    /// The share the configuration gives `label` of the stream `name`'s
    /// agents, or none, the stream declared where it was not (181).
    pub fn set_share(&self, name: &str, label: &str, share: Option<Share>) {
        let mut state = lock(&self.state);
        let labels = &mut state.streams.entry(name.to_string()).or_default().labels;
        match share {
            Some(share) => labels.insert(label.to_string(), share),
            None => labels.remove(label),
        };
    }

    /// The label the configuration gives `kind` in `repository`, or in
    /// every repository where none is named; none removes it (180).
    pub fn name_label(&self, repository: Option<&str>, kind: &str, label: Option<&str>) {
        let mut state = lock(&self.state);
        let key = (repository.map(str::to_string), kind.to_string());
        match label {
            Some(label) => state.labels_named.insert(key, label.to_string()),
            None => state.labels_named.remove(&key),
        };
    }

    /// The configuration as it stands: its values with their layers,
    /// whether its local layer was taken, and the keys passed over (C.7).
    pub fn set_configuration(&self, configuration: HostConfiguration) {
        lock(&self.state).configuration = configuration;
    }

    /// The configuration's value at `key`, as TOML writes it, from
    /// `layer`, in place of any it gave before (C.7).
    pub fn set_value(&self, key: &str, value: &str, layer: ConfigLayer) {
        let mut state = lock(&self.state);
        let values = &mut state.configuration.values;
        values.retain(|given| given.key != key);
        values.push(ConfigValue {
            key: key.to_string(),
            value: value.to_string(),
            layer,
        });
    }

    /// The host's own pool, or none (144).
    pub fn set_host_pool(&self, pool: Option<&str>) {
        lock(&self.state).pool = pool.map(str::to_string);
    }

    /// The pool the configuration declares under `name`, or none (144).
    pub fn set_pool(&self, name: &str, pool: Option<Pool>) {
        let mut state = lock(&self.state);
        match pool {
            Some(pool) => state.pools.insert(name.to_string(), pool),
            None => state.pools.remove(name),
        };
    }

    /// The configuration declares the source `name` by its file, its
    /// packages in `kinds_dir`, in place of any it declared under that
    /// name (191).
    pub fn set_source(&self, name: &str, kinds_dir: &str) {
        self.source(
            name,
            HostSourceRead::Read {
                kinds_dir: kinds_dir.into(),
            },
        );
    }

    /// The file of the source `name` cannot be read, for `reason` (191).
    pub fn faulty_source(&self, name: &str, reason: &str) {
        self.source(
            name,
            HostSourceRead::CouldNotRead {
                reason: reason.into(),
            },
        );
    }

    /// The configuration declares no source `name`: its file is gone.
    pub fn remove_source(&self, name: &str) {
        lock(&self.state).sources.remove(name);
    }

    fn source(&self, name: &str, read: HostSourceRead) {
        lock(&self.state).sources.insert(
            name.into(),
            HostSource {
                file: source_file(name),
                read,
            },
        );
    }

    /// The reading at `path`, or why there is none, in place of any set
    /// before; every path no test set answers that the file is missing
    /// (184).
    pub fn set_usage(&self, path: &str, usage: Result<Usage, String>) {
        lock(&self.state).usage.insert(path.to_string(), usage);
    }

    /// Whether the room lock is held now.
    pub fn room_taken(&self) -> bool {
        *lock(&self.room.taken)
    }
}

impl Host for FakeHost {
    fn now(&self) -> Seconds {
        lock(&self.state).now
    }

    fn sleep(&self, seconds: Seconds) {
        self.log.push(format!("sleep {seconds}"));
        self.advance(seconds);
    }

    fn secret(&self) -> Secret {
        let mut state = lock(&self.state);
        state.secrets += 1;
        Secret(format!("secret-{}", state.secrets))
    }

    fn write_time(&self, path: &str) -> Option<Seconds> {
        self.log.push(format!("write_time {path}"));
        lock(&self.state).files.get(path).copied()
    }

    fn command(&self) -> String {
        format!("command-{}", lock(&self.state).command)
    }

    fn alive(&self, command: &str) -> bool {
        lock(&self.state).alive.contains(command)
    }

    fn bound(&self) -> Option<u32> {
        self.log.push("bound");
        lock(&self.state).bound
    }

    fn recipient(&self) -> Option<String> {
        self.log.push("recipient");
        lock(&self.state).recipient.clone()
    }

    fn role(&self, address: &str) -> Option<RoleRun> {
        self.log.push(format!("role {address}"));
        lock(&self.state).roles.get(address).cloned()
    }

    fn streams(&self) -> BTreeMap<String, Stream> {
        self.log.push("streams");
        let mut streams = lock(&self.state).streams.clone();
        streams.entry(DEFAULT_STREAM.to_string()).or_default();
        streams
    }

    fn stream_for(&self, repository: &str, kind: &str) -> Option<StreamHeld> {
        self.log.push(format!("stream_for {repository} {kind}"));
        let state = lock(&self.state);
        let (repository, kind) = (Some(repository.to_string()), Some(kind.to_string()));
        [
            (
                (repository.clone(), kind.clone()),
                StreamFrom::HostRepositoryKind,
            ),
            ((repository, None), StreamFrom::HostRepository),
            ((None, kind), StreamFrom::HostKind),
        ]
        .into_iter()
        .find_map(|(key, from)| {
            state.streams_named.get(&key).map(|name| StreamHeld {
                name: name.clone(),
                from,
            })
        })
    }

    fn label_for(&self, repository: &str, kind: &str) -> Option<LabelHeld> {
        self.log.push(format!("label_for {repository} {kind}"));
        let state = lock(&self.state);
        [
            (Some(repository.to_string()), LabelFrom::HostRepositoryKind),
            (None, LabelFrom::HostKind),
        ]
        .into_iter()
        .find_map(|(repository, from)| {
            state
                .labels_named
                .get(&(repository, kind.to_string()))
                .map(|name| LabelHeld {
                    name: name.clone(),
                    from,
                })
        })
    }

    fn configuration(&self) -> HostConfiguration {
        self.log.push("configuration");
        lock(&self.state).configuration.clone()
    }

    fn pool_for(&self, stream: &str) -> Option<PoolNamed> {
        self.log.push(format!("pool_for {stream}"));
        let state = lock(&self.state);
        let named = |name: &String, from| PoolNamed {
            name: name.clone(),
            from,
        };
        state
            .streams
            .get(stream)
            .and_then(|stream| stream.pool.as_ref())
            .map(|name| named(name, PoolFrom::Stream))
            .or_else(|| state.pool.as_ref().map(|name| named(name, PoolFrom::Host)))
    }

    /// The stream's multiplexer, view and terminal app, each the
    /// stream's key where it names one, else the host's, else `fake`, a
    /// terminal view and `fake-window`, the fakes' own (186, 190).
    fn display_for(&self, stream: &str) -> StreamDisplay {
        self.log.push(format!("display_for {stream}"));
        let state = lock(&self.state);
        let stream = state.streams.get(stream);
        let host = &state.display;
        let from = |stream: bool, host: bool| match (stream, host) {
            (true, _) => NamedBy::Stream,
            (false, true) => NamedBy::Host,
            (false, false) => NamedBy::Neither,
        };
        let named = |stream: Option<&String>, host: Option<&String>, default: &str| ProfileHeld {
            profile: stream.or(host).map_or(default, String::as_str).to_string(),
            from: from(stream.is_some(), host.is_some()),
        };
        let view = stream.and_then(|stream| stream.view);
        StreamDisplay {
            multiplexer: named(
                stream.and_then(|stream| stream.multiplexer.as_ref()),
                host.multiplexer.as_ref(),
                "fake",
            ),
            view: ViewHeld {
                view: view.or(host.view).unwrap_or(View::Terminal),
                from: from(view.is_some(), host.view.is_some()),
            },
            terminal_app: named(
                stream.and_then(|stream| stream.terminal_app.as_ref()),
                host.terminal_app.as_ref(),
                "fake-window",
            ),
        }
    }

    fn pool(&self, name: &str) -> Option<Pool> {
        self.log.push(format!("pool {name}"));
        lock(&self.state).pools.get(name).cloned()
    }

    fn usage(&self, path: &str) -> Result<Usage, String> {
        self.log.push(format!("usage {path}"));
        lock(&self.state)
            .usage
            .get(path)
            .cloned()
            .unwrap_or_else(|| Err("the file is missing".to_string()))
    }

    fn settings(&self, repository: &str, kind: &str) -> HostSettings {
        self.log.push(format!("settings {repository} {kind}"));
        let state = lock(&self.state);
        HostSettings {
            repository: state
                .repos
                .get(&(repository.to_string(), kind.to_string()))
                .cloned()
                .unwrap_or_default(),
            kind: state.kinds.get(kind).cloned().unwrap_or_default(),
        }
    }

    fn login(&self, name: &str) -> Option<String> {
        lock(&self.state).login.get(name).cloned()
    }

    fn log(&self, line: &str) {
        self.logged.push(line);
    }

    fn sources(&self) -> BTreeMap<String, HostSource> {
        self.log.push("sources");
        lock(&self.state).sources.clone()
    }

    fn sources_dir(&self) -> Option<String> {
        Some(SOURCES.into())
    }

    fn kinds_lock(&self) -> Option<String> {
        Some(HOST_LOCK.into())
    }

    fn room(&self) -> RoomGuard {
        self.log.push("room");
        let mut taken = lock(&self.room.taken);
        while *taken {
            taken = self
                .room
                .freed
                .wait(taken)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
        *taken = true;
        RoomGuard::new(Held(self.room.clone()))
    }
}
