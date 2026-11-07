//! The host's configuration and directories (C.7; design 10): flume's
//! state under `$XDG_STATE_HOME/flume`, and its configuration in
//! `$XDG_CONFIG_HOME/flume/config.toml`, each beneath `$HOME` as the XDG
//! directories default where the variable is unset. The configuration's
//! `[host] agents` is the bound on agents running (70), `[host]
//! multiplexer` the terminal profile bound for the host, `[host]
//! terminal` the word written before it and read as it (105, 190), and
//! `[host] view` and `terminal_app` how a person is shown a stream's
//! agents (186, 188); a file or a word left out is no bound, the printed
//! command, a terminal view and the printed window. Its
//! `[kinds.<kind>.agents]` and `[repos."<repository>".kinds.<kind>.agents]`
//! tables, each naming one stage's by a further `.stages.<stage>` table,
//! give a kind's agents in every repository and in one their `model`,
//! `instructions`, `mode` and `keep_open`, each kept as written, a mode
//! one of flume's four or a table of programs whose entries are strings,
//! and `keep_open`, how long an agent is kept open for an answer, one
//! time for every program, `<n>`, `<n>s`, `<n>m` or `<n>h` (32, 162,
//! 169, 198, C.7; records 0017, 0020, 0024, 0046); a
//! repository's key has a leading `~` read as the home, so it names the
//! repository by its location as a launch does. Its
//! `[repos."<repository>".agents]` gives the same for every kind in one
//! repository, its `[agents]` for every agent on the host and its
//! `[agents.streams.<stream>]` for every agent in one stream the host
//! declares, none of the three naming a program, an environment, a pane
//! beside or a stage, and the first no stream (32, 143, 195; record
//! 0043). Every agents table gives `programs`, each program's own
//! settings, read by flume's one reader of them; a file an entry names is
//! read on the host with the configuration, absolute, under `~/` read as
//! the home as a repository's key is, or relative to the configuration's
//! own directory, where a link leads, and never with a `$VAR` read (195,
//! C.7; core.30). Its
//! `[roles."<partition>/<role>"]` tables are the runs the host wakes for
//! each role's address, with `repository`, `kind`, `priority`, `base`,
//! `brief` and `stream`, or, where one gives `agent`, the standing agent
//! that holds the address, by the terminal's name for it, with an
//! optional `session` and `stream` and none of a run's other keys, a
//! table giving both `agent` and `kind` unreadable; an address it names
//! nothing for is woken by nothing (3, 132, C.7). Its `[streams.<stream>]` tables declare the host's streams,
//! each with `agents`, `priority`, `pool`, `session`, `multiplexer`,
//! `view`, `terminal_app`, `title`, its labels' shares in
//! `[streams.<stream>.labels.<label>]`, and the session it names for a
//! kind in `[streams.<stream>.kinds.<kind>]` and for one stage of a kind
//! in that table's `.stages.<stage>`, and `default` stands
//! whether or not a table declares it; its `[pools.<pool>]` tables its
//! pools, each with `strategy` and `accounts`, each account a `name`, a
//! `program`, an `env` kept as written and an optional `agents`; `[host]
//! pool` is the host's own pool; and `stream` in `[repos."<repository>"]`,
//! `[repos."<repository>".kinds.<kind>]` and `[kinds.<kind>]` names a
//! place's stream, and `label` in the last two its label (143, 144, 180,
//! 181, 199, C.7).
//!
//! The configuration has two layers (C.7; design 4): `config.toml` is the
//! managed one, and `config.local.toml` beside it the local one, read
//! over it key by key, each key of [`LOCAL_KEYS`] it sets replacing the
//! managed one's whole. A local file setting any other key, or at fault,
//! is not taken at all, the managed layer read alone; a key `[host] pin`
//! holds is passed over. The reading keeps each value with its layer, an
//! `env` table's and an own setting's shaped as a credential as `<set>`
//! (core.30, core.50). No command writes either file (core.16, core.45).
//!
//! The host's sources of kinds are files of the configuration's managed
//! layer, one each in `sources/` beside `config.toml` (159, 191, C.7;
//! design 1); the local layer declares none. Each regular file there, or
//! a link to one, whose name ends in `.toml` and begins with no `.`
//! declares the source its name less `.toml` names; every other entry is
//! ignored, so a source written beside as `.acme.toml.tmp` and renamed
//! over `acme.toml` is never read half written. Its one key, `kinds_dir`,
//! is the directory holding the source's packages, absolute or under `~`,
//! read as the home. A file that is not TOML, gives no `kinds_dir`, one
//! neither absolute nor under `~` or any other key, or takes a name the
//! kinds profile refuses for a source, is that source's fault alone, kept
//! with it and failing nothing else of the configuration; no `sources/`
//! is no source, and one that cannot be listed fails the configuration as
//! a `config.toml` that cannot be read does. flume writes nothing there.

use crate::profiles::kinds::weir::source_fault;
use flume_seams::{
    is_address, read_span, Account, ConfigLayer, ConfigValue, HostConfiguration, HostSource,
    HostSourceRead, KindSessions, KindSettings, LabelFrom, LabelHeld, LocalLayer, NamedBy,
    PinnedKey, Pool, PoolFrom, PoolNamed, ProfileHeld, RoleRun, Settings, Share, StageSession,
    StandingAgent, Strategy, Stream, StreamDisplay, StreamFrom, StreamHeld, View, ViewHeld,
    DEFAULT_STREAM, SET,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use toml::{Table, Value};

/// The profile a stream and the host naming none get, for its
/// multiplexer the printed command and for its terminal app the printed
/// window (105, 188, 190).
const PRINTED: &str = "printed";

/// The keys the local layer may set, each a path of segments, `*` any one
/// name: a stream's `agents`, `priority`, `points`, `pace`, `session` and
/// `multiplexer`, the `session` it names for a kind and for a kind's
/// stage, a label's `reserve` and `keep_free`, the `stream` a
/// repository, a repository's kind or a kind is given, the `label` a
/// repository's kind or a kind is given, and every agents table, a kind's
/// every stage's and one stage's, a repository's for every kind, a
/// stream's and every agent's, each of its settings and in its `programs`
/// each program's `file` and each of its own settings a key of its own
/// (32, 107, 143, 166, 167, 180, 181, 190, 195, 199, C.7; record 0043).
/// Each is one unit, its value taken whole. A change that gives the local
/// layer another key adds it here.
pub const LOCAL_KEYS: &[&[&str]] = &[
    &["streams", "*", "agents"],
    &["streams", "*", "priority"],
    &["streams", "*", "points"],
    &["streams", "*", "pace"],
    &["streams", "*", "session"],
    &["streams", "*", "multiplexer"],
    &["streams", "*", "kinds", "*", "session"],
    &["streams", "*", "kinds", "*", "stages", "*", "session"],
    &["streams", "*", "labels", "*", "reserve"],
    &["streams", "*", "labels", "*", "keep_free"],
    &["repos", "*", "stream"],
    &["repos", "*", "kinds", "*", "stream"],
    &["kinds", "*", "stream"],
    &["repos", "*", "kinds", "*", "label"],
    &["kinds", "*", "label"],
    &["kinds", "*", "agents", "*"],
    &["kinds", "*", "agents", "programs", "*", "file"],
    &["kinds", "*", "agents", "programs", "*", "settings", "*"],
    &["kinds", "*", "agents", "stages", "*", "*"],
    &[
        "kinds", "*", "agents", "stages", "*", "programs", "*", "file",
    ],
    &[
        "kinds", "*", "agents", "stages", "*", "programs", "*", "settings", "*",
    ],
    &["repos", "*", "kinds", "*", "agents", "*"],
    &[
        "repos", "*", "kinds", "*", "agents", "programs", "*", "file",
    ],
    &[
        "repos", "*", "kinds", "*", "agents", "programs", "*", "settings", "*",
    ],
    &["repos", "*", "kinds", "*", "agents", "stages", "*", "*"],
    &[
        "repos", "*", "kinds", "*", "agents", "stages", "*", "programs", "*", "file",
    ],
    &[
        "repos", "*", "kinds", "*", "agents", "stages", "*", "programs", "*", "settings", "*",
    ],
    &["repos", "*", "agents", "*"],
    &["repos", "*", "agents", "programs", "*", "file"],
    &["repos", "*", "agents", "programs", "*", "settings", "*"],
    &["agents", "*"],
    &["agents", "programs", "*", "file"],
    &["agents", "programs", "*", "settings", "*"],
    &["agents", "streams", "*", "*"],
    &["agents", "streams", "*", "programs", "*", "file"],
    &["agents", "streams", "*", "programs", "*", "settings", "*"],
];

/// The host's configuration as its layers state it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostConfig {
    /// The bound on agents running, every caller's counted together;
    /// none where the file states none (70).
    pub agents: Option<u32>,
    /// The terminal profile the host binds, `[host] multiplexer` or the
    /// word written before it, `[host] terminal`; none for the printed
    /// command (105, 190).
    pub terminal: Option<String>,
    /// How the host's streams are shown to a person, `[host] view`; none
    /// for a terminal view (186).
    pub view: Option<View>,
    /// The window profile the host binds, `[host] terminal_app`; none for
    /// the printed window (188).
    pub terminal_app: Option<String>,
    /// The settings given each kind's agents in every repository, by the
    /// kind's name (32).
    pub kinds: BTreeMap<String, KindSettings>,
    /// The settings given each kind's agents in one repository, by its
    /// location and the kind's name (32).
    pub repos: BTreeMap<String, BTreeMap<String, KindSettings>>,
    /// The settings given every kind's agents in one repository, by its
    /// location, its `[repos."<repository>".agents]` (32, 195).
    pub repos_agents: BTreeMap<String, Settings>,
    /// The settings given every agent in one stream, by the stream's
    /// name, its `[agents.streams.<stream>]` (32, 143).
    pub streams_agents: BTreeMap<String, Settings>,
    /// The settings given every agent on the host, its `[agents]` (32).
    pub every_agent: Settings,
    /// The run woken for each role's address, by the address (132, C.7).
    pub roles: BTreeMap<String, RoleRun>,
    /// The standing agent named for each role's address an entry gives
    /// `agent`, by the address, none of them in `roles` (132, C.7).
    pub standing: BTreeMap<String, StandingAgent>,
    /// The streams declared, by name, `default` among them always (143).
    pub streams: BTreeMap<String, Stream>,
    /// The streams named for places, by repository and kind, by
    /// repository and by kind (143).
    pub streams_named: StreamsNamed,
    /// The labels given places, by repository and kind and by kind (180).
    pub labels_named: LabelsNamed,
    /// The host's own pool, none where it names none (144).
    pub pool: Option<String>,
    /// The pools declared, by name (144).
    pub pools: BTreeMap<String, Pool>,
    /// Every value with its layer, whether the local layer was taken, and
    /// each local key passed over with its pin; none of it where only one
    /// table was read (C.7).
    pub configuration: HostConfiguration,
    /// The sources of kinds declared in `sources/` beside the file, by
    /// name, each with its fault where it has one (159, 191, C.7).
    pub sources: BTreeMap<String, HostSource>,
    /// The directory the file was read in, which holds `sources/` and the
    /// host's lock of kinds; none for a configuration read from text
    /// (155, 191, C.7).
    pub dir: Option<PathBuf>,
}

/// The streams the configuration names for places: `stream` in
/// `[repos."<repository>".kinds.<kind>]`, in `[repos."<repository>"]` and
/// in `[kinds.<kind>]` (143, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StreamsNamed {
    /// By the repository's location and the kind's name.
    pub repository_kind: BTreeMap<(String, String), String>,
    /// By the repository's location.
    pub repository: BTreeMap<String, String>,
    /// By the kind's name.
    pub kind: BTreeMap<String, String>,
}

/// The labels the configuration gives places: `label` in
/// `[repos."<repository>".kinds.<kind>]` and in `[kinds.<kind>]` (180,
/// C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LabelsNamed {
    /// By the repository's location and the kind's name.
    pub repository_kind: BTreeMap<(String, String), String>,
    /// By the kind's name.
    pub kind: BTreeMap<String, String>,
}

impl Default for HostConfig {
    fn default() -> Self {
        HostConfig {
            agents: None,
            terminal: None,
            view: None,
            terminal_app: None,
            kinds: BTreeMap::new(),
            repos: BTreeMap::new(),
            repos_agents: BTreeMap::new(),
            streams_agents: BTreeMap::new(),
            every_agent: Settings::default(),
            roles: BTreeMap::new(),
            standing: BTreeMap::new(),
            streams: BTreeMap::from([(DEFAULT_STREAM.to_string(), Stream::default())]),
            streams_named: StreamsNamed::default(),
            labels_named: LabelsNamed::default(),
            pool: None,
            pools: BTreeMap::new(),
            configuration: HostConfiguration::default(),
            sources: BTreeMap::new(),
            dir: None,
        }
    }
}

impl HostConfig {
    /// The configuration in `path`, the managed layer with no local one,
    /// and the sources in `sources/` beside it: none of the file where
    /// there is none, and the file's fault named where it is not one
    /// (159, 191, C.7).
    pub fn read(path: &Path) -> Result<HostConfig, String> {
        let managed = text_in(path).map_err(|error| format!("{}: {error}", path.display()))?;
        HostConfig::layers_in(&managed.unwrap_or_default(), None, &directory_of(path))
            .map_err(|fault| format!("{}: {fault}", path.display()))?
            .with_sources(path)
    }

    /// The configuration of the managed layer in `managed` and the local
    /// layer in `local` beside it, and the sources in `sources/` beside
    /// the managed one: a managed file's fault refuses it, naming the
    /// file, and a local file that cannot be read is not taken (159, 191,
    /// C.7; design 4).
    pub fn read_layers(managed: &Path, local: &Path) -> Result<HostConfig, String> {
        let text = text_in(managed).map_err(|error| format!("{}: {error}", managed.display()))?;
        let managed_text = text.unwrap_or_default();
        let directory = directory_of(managed);
        let config = match text_in(local) {
            Ok(local) => HostConfig::layers_in(&managed_text, local.as_deref(), &directory),
            Err(fault) => HostConfig::layers_in(&managed_text, None, &directory)
                .map(|config| config.local_not_taken(vec![], Some(fault))),
        };
        config
            .map_err(|fault| format!("{}: {fault}", managed.display()))?
            .with_sources(managed)
    }

    /// This configuration with the sources in `sources/` beside the
    /// managed layer's `path`, and the directory `path` lies in, which
    /// holds the host's lock of kinds: the sources are the managed
    /// layer's, the local layer declaring none (155, 159, 191, C.7).
    fn with_sources(mut self, path: &Path) -> Result<HostConfig, String> {
        self.sources = read_sources(&path.with_file_name(SOURCES))?;
        self.dir = path.parent().map(Path::to_path_buf);
        Ok(self)
    }

    /// The configuration the managed layer's `managed` and the local
    /// layer's `local` state, none where there is no local file (C.7;
    /// design 4). The local layer's units are each key of [`LOCAL_KEYS`]
    /// it sets; one that `[host] pin` holds is passed over, and the rest
    /// replace the managed layer's whole, a repository's table found by
    /// its location as a key with `~` reads it. A local layer that sets
    /// any other key, is no TOML, or whose units the reader refuses where
    /// the managed layer alone reads, is not taken at all. A fault of the
    /// managed layer, a pin of nothing the local layer may set among it,
    /// refuses the configuration. A settings file is read relative to the
    /// host's configuration directory.
    pub fn layers(managed: &str, local: Option<&str>) -> Result<HostConfig, String> {
        HostConfig::layers_in(managed, local, &host_directory())
    }

    /// The configuration `managed` and `local` state, as [`HostConfig::layers`]
    /// reads them, a settings file read relative to `directory`, the one
    /// both files share (195, C.7).
    pub fn layers_in(
        managed: &str,
        local: Option<&str>,
        directory: &Path,
    ) -> Result<HostConfig, String> {
        let managed: Table = managed.parse().map_err(|error| format!("{error}"))?;
        let alone = HostConfig::from_table(&managed, directory)?.with_layers(
            &managed,
            LocalLayer::NoFile,
            &[],
            vec![],
        );
        let Some(local) = local else {
            return Ok(alone);
        };
        let local_table: Table = match local.parse() {
            Ok(table) => table,
            Err(error) => return Ok(alone.local_not_taken(vec![], Some(toml_fault(local, &error)))),
        };
        let found = LocalUnits::of(&local_table);
        if !found.keys.is_empty() || found.fault.is_some() {
            return Ok(alone.local_not_taken(found.keys, found.fault));
        }
        let pins = pins_of(&managed)?;
        let mut merged = managed.clone();
        let mut taken = vec![];
        let mut passed_over = vec![];
        for (path, value) in found.units {
            let read = as_read(&path);
            match pins.iter().find(|(_, pin)| read.starts_with(pin)) {
                Some((pin, _)) => passed_over.push(PinnedKey {
                    key: dotted(&path),
                    pin: pin.clone(),
                }),
                None => {
                    set_at(&mut merged, &path, value);
                    taken.push(read);
                }
            }
        }
        match HostConfig::from_table(&merged, directory) {
            Ok(config) => Ok(config.with_layers(&merged, LocalLayer::Taken, &taken, passed_over)),
            Err(fault) => Ok(alone.local_not_taken(vec![], Some(fault))),
        }
    }

    /// This configuration, read from the managed layer alone, with its
    /// local layer not taken for `keys` or `fault` (C.7).
    fn local_not_taken(mut self, keys: Vec<String>, fault: Option<String>) -> HostConfig {
        self.configuration.local = LocalLayer::NotTaken { keys, fault };
        self
    }

    /// This configuration with every value of `table`, the one it was
    /// read from, each local where it lies at or under a unit of `taken`,
    /// the local layer's state and the keys passed over (C.7).
    fn with_layers(
        self,
        table: &Table,
        local: LocalLayer,
        taken: &[Vec<String>],
        passed_over: Vec<PinnedKey>,
    ) -> HostConfig {
        let mut values = vec![];
        for (name, value) in table {
            let at = Leaf {
                path: vec![name.clone()],
                key: quoted(name),
                listed: true,
                secret: false,
            };
            values_of(value, at, taken, &mut values);
        }
        HostConfig {
            configuration: HostConfiguration {
                local,
                values,
                passed_over,
            },
            ..self
        }
    }

    /// The configuration `text` states, read as one table, with no layer
    /// kept, a settings file read relative to the host's configuration
    /// directory.
    pub fn parse(text: &str) -> Result<HostConfig, String> {
        HostConfig::parse_in(text, &host_directory())
    }

    /// The configuration `text` states, as [`HostConfig::parse`] reads it,
    /// a settings file read relative to `directory`.
    pub fn parse_in(text: &str, directory: &Path) -> Result<HostConfig, String> {
        let table: toml::Table = text.parse().map_err(|error| format!("{error}"))?;
        HostConfig::from_table(&table, directory)
    }

    /// The configuration `table` states, its pins checked and no layer
    /// kept, a settings file read relative to `directory`.
    fn from_table(table: &Table, directory: &Path) -> Result<HostConfig, String> {
        pins_of(table)?;
        let mut config = HostConfig::default();
        if let Some(kinds) = table.get("kinds") {
            config.kinds = kinds_settings(kinds, "kinds", directory)?;
            config.streams_named.kind = kinds_streams(kinds, "kinds")?;
            config.labels_named.kind = kinds_labels(kinds, "kinds")?;
        }
        if let Some(given) = table.get("repos") {
            let given = given
                .as_table()
                .ok_or("[repos] is not a table of repositories".to_string())?;
            for (repository, table) in given {
                let at = format!("repos.{}", quoted(repository));
                let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
                let location = home(repository);
                if let Some(agents) = table.get("agents") {
                    let settings = every_kind(agents, &format!("{at}.agents"), directory)?;
                    config.repos_agents.insert(location.clone(), settings);
                }
                if let Some(stream) = text_of(table, "stream", &at)? {
                    config
                        .streams_named
                        .repository
                        .insert(location.clone(), stream);
                }
                let Some(kinds) = table.get("kinds") else {
                    config.repos.insert(location, BTreeMap::new());
                    continue;
                };
                let at = format!("{at}.kinds");
                for (kind, stream) in kinds_streams(kinds, &at)? {
                    config
                        .streams_named
                        .repository_kind
                        .insert((location.clone(), kind), stream);
                }
                for (kind, label) in kinds_labels(kinds, &at)? {
                    config
                        .labels_named
                        .repository_kind
                        .insert((location.clone(), kind), label);
                }
                config
                    .repos
                    .insert(location, kinds_settings(kinds, &at, directory)?);
            }
        }
        if let Some(roles) = table.get("roles") {
            (config.roles, config.standing) = role_runs(roles)?;
        }
        if let Some(pools) = table.get("pools") {
            config.pools = pools_declared(pools)?;
        }
        if let Some(streams) = table.get("streams") {
            config.streams.extend(streams_declared(streams)?);
        }
        if let Some(agents) = table.get("agents") {
            (config.every_agent, config.streams_agents) =
                every_agent(agents, &config.streams, directory)?;
        }
        if let Some(host) = table.get("host") {
            let host = host.as_table().ok_or("[host] is not a table".to_string())?;
            config.agents = match host.get("agents") {
                None => None,
                Some(value) => {
                    let agents = value
                        .as_integer()
                        .ok_or("[host] agents is not a whole number".to_string())?;
                    Some(u32::try_from(agents).map_err(|_| {
                        format!("[host] agents is {agents}, where a bound is a count from zero")
                    })?)
                }
            };
            config.terminal = host_multiplexer(host)?;
            config.view = view_of(host, "host")?;
            config.terminal_app = text_of(host, "terminal_app", "host")
                .map_err(|_| "[host] terminal_app is not a name".to_string())?;
            config.pool = text_of(host, "pool", "host")?;
        }
        let declared = |pool: &str| config.pools.contains_key(pool);
        if let Some(pool) = config.pool.as_deref().filter(|pool| !declared(pool)) {
            return Err(format!(
                "[host] pool names the pool {pool}, which no [pools] table declares"
            ));
        }
        for (name, stream) in &config.streams {
            if let Some(pool) = stream.pool.as_deref().filter(|pool| !declared(pool)) {
                return Err(format!(
                    "[streams.{}] pool names the pool {pool}, which no [pools] table declares",
                    quoted(name)
                ));
            }
        }
        Ok(config)
    }

    /// The stream the configuration names for `kind` in `repository`:
    /// for the repository and kind, else the repository, else the kind
    /// (143).
    pub fn stream_for(&self, repository: &str, kind: &str) -> Option<StreamHeld> {
        let named = &self.streams_named;
        let held = |name: &String, from| StreamHeld {
            name: name.clone(),
            from,
        };
        named
            .repository_kind
            .get(&(repository.to_string(), kind.to_string()))
            .map(|name| held(name, StreamFrom::HostRepositoryKind))
            .or_else(|| {
                named
                    .repository
                    .get(repository)
                    .map(|name| held(name, StreamFrom::HostRepository))
            })
            .or_else(|| {
                named
                    .kind
                    .get(kind)
                    .map(|name| held(name, StreamFrom::HostKind))
            })
    }

    /// The label the configuration gives `kind` in `repository`: for the
    /// repository and kind, else the kind (180).
    pub fn label_for(&self, repository: &str, kind: &str) -> Option<LabelHeld> {
        let named = &self.labels_named;
        let held = |name: &String, from| LabelHeld {
            name: name.clone(),
            from,
        };
        named
            .repository_kind
            .get(&(repository.to_string(), kind.to_string()))
            .map(|name| held(name, LabelFrom::HostRepositoryKind))
            .or_else(|| {
                named
                    .kind
                    .get(kind)
                    .map(|name| held(name, LabelFrom::HostKind))
            })
    }

    /// The multiplexer, view and terminal app of a place in `stream`:
    /// each the stream's own, else the host's, else the printed command,
    /// a terminal view and the printed window (186, 188, 190, C.7).
    pub fn display_for(&self, stream: &str) -> StreamDisplay {
        let stream = self.streams.get(stream);
        let named = |own: Option<&String>, host: Option<&String>| match (own, host) {
            (Some(name), _) => ProfileHeld {
                profile: name.clone(),
                from: NamedBy::Stream,
            },
            (None, Some(name)) => ProfileHeld {
                profile: name.clone(),
                from: NamedBy::Host,
            },
            (None, None) => ProfileHeld {
                profile: PRINTED.into(),
                from: NamedBy::Neither,
            },
        };
        StreamDisplay {
            multiplexer: named(
                stream.and_then(|stream| stream.multiplexer.as_ref()),
                self.terminal.as_ref(),
            ),
            view: match (stream.and_then(|stream| stream.view), self.view) {
                (Some(view), _) => ViewHeld {
                    view,
                    from: NamedBy::Stream,
                },
                (None, Some(view)) => ViewHeld {
                    view,
                    from: NamedBy::Host,
                },
                (None, None) => ViewHeld {
                    view: View::Terminal,
                    from: NamedBy::Neither,
                },
            },
            terminal_app: named(
                stream.and_then(|stream| stream.terminal_app.as_ref()),
                self.terminal_app.as_ref(),
            ),
        }
    }

    /// The pool of a place in `stream`: the stream's own, else the
    /// host's (144).
    pub fn pool_for(&self, stream: &str) -> Option<PoolNamed> {
        let named = |name: &String, from| PoolNamed {
            name: name.clone(),
            from,
        };
        self.streams
            .get(stream)
            .and_then(|stream| stream.pool.as_ref())
            .map(|name| named(name, PoolFrom::Stream))
            .or_else(|| self.pool.as_ref().map(|name| named(name, PoolFrom::Host)))
    }

    /// The host's configuration, `$XDG_CONFIG_HOME/flume/config.toml` and
    /// `config.local.toml` over it (C.7).
    pub fn of_host() -> Result<HostConfig, String> {
        HostConfig::read_layers(&config_file()?, &local_config_file()?)
    }
}

/// A file's text, none where there is no file.
fn text_in(path: &Path) -> Result<Option<String>, String> {
    match std::fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("{error}")),
    }
}

/// A file's fault as TOML gives it, on one line, with the line of the
/// file it lies at.
fn toml_fault(text: &str, error: &toml::de::Error) -> String {
    let message = error.message().trim_end();
    match error.span() {
        Some(span) => {
            let line = text[..span.start.min(text.len())].matches('\n').count() + 1;
            format!("{message} at line {line}")
        }
        None => message.to_string(),
    }
}

/// Whether `path` matches the first segments of `key`, `*` matching any
/// one name.
fn begins(key: &[&str], path: &[String]) -> bool {
    path.len() <= key.len()
        && path
            .iter()
            .zip(key)
            .all(|(segment, word)| *word == "*" || segment == word)
}

/// Whether `path` is a key of [`LOCAL_KEYS`].
fn is_local_key(path: &[String]) -> bool {
    LOCAL_KEYS
        .iter()
        .any(|key| key.len() == path.len() && begins(key, path))
}

/// Whether `path` is a table holding some key of [`LOCAL_KEYS`].
fn leads_to_local_key(path: &[String]) -> bool {
    LOCAL_KEYS
        .iter()
        .any(|key| key.len() > path.len() && begins(key, path))
}

/// Whether the value `value` at `path` is one unit of the local layer's:
/// a key of [`LOCAL_KEYS`], unless it is a table holding a longer one,
/// as a kind's `agents.stages` does.
fn is_unit(path: &[String], value: &Value) -> bool {
    is_local_key(path) && !(value.is_table() && leads_to_local_key(path))
}

/// `path` with the repository segment after `repos` read as a
/// repository's key is, its leading `~` the home, so two spellings of
/// one repository compare equal.
fn as_read(path: &[String]) -> Vec<String> {
    let mut read = path.to_vec();
    if read.first().is_some_and(|first| first == "repos") {
        if let Some(repository) = read.get_mut(1) {
            *repository = home(repository);
        }
    }
    read
}

/// `path` as a dotted key in TOML's spelling.
fn dotted(path: &[String]) -> String {
    path.iter()
        .map(|segment| quoted(segment))
        .collect::<Vec<_>>()
        .join(".")
}

/// What a local layer's table holds: each unit it sets, by its path, in
/// order; each key it may not set, named once; and its fault, a table of
/// the list's that is no table (C.7).
#[derive(Default)]
struct LocalUnits {
    units: Vec<(Vec<String>, Value)>,
    keys: Vec<String>,
    fault: Option<String>,
}

impl LocalUnits {
    /// The units, keys and fault of the local layer's `table`.
    fn of(table: &Table) -> LocalUnits {
        let mut found = LocalUnits::default();
        found.walk(table, &mut vec![]);
        found
    }

    /// Walks `table` at `path`, each table holding a key of the list
    /// walked further, and each top table, so a key it may not set is
    /// named by two segments.
    fn walk(&mut self, table: &Table, path: &mut Vec<String>) {
        for (name, value) in table {
            path.push(name.clone());
            match value {
                Value::Table(inner) if path.len() < 2 || leads_to_local_key(path) => {
                    self.walk(inner, path)
                }
                _ if is_local_key(path) => self.units.push((path.clone(), value.clone())),
                _ if leads_to_local_key(path) => {
                    self.fault
                        .get_or_insert_with(|| format!("[{}] is not a table", dotted(path)));
                }
                _ => {
                    let key = dotted(named_by(path));
                    if !self.keys.contains(&key) {
                        self.keys.push(key);
                    }
                }
            }
            path.pop();
        }
    }
}

/// The part of `path`, a key the local layer may not set, that names it:
/// the shortest that leads to no key of [`LOCAL_KEYS`], and at least two
/// segments where it has two, so everything in `[pools.work]` is named
/// `pools.work` and a stream's `pool` `streams.work.pool` (C.7).
fn named_by(path: &[String]) -> &[String] {
    let shortest = (1..=path.len())
        .find(|n| {
            let part = &path[..*n];
            !is_local_key(part) && !leads_to_local_key(part)
        })
        .unwrap_or(path.len());
    &path[..shortest.max(2).min(path.len())]
}

/// The managed layer's pins, `[host] pin`, each by its words as written
/// and its path as read, a repository's `~` the home: each a dotted key
/// in TOML's spelling, read by parsing `<pin> = 0` as a document, naming
/// a key of [`LOCAL_KEYS`] or a table holding some; any other refused
/// naming it (C.7).
fn pins_of(table: &Table) -> Result<Vec<(String, Vec<String>)>, String> {
    let Some(given) = table
        .get("host")
        .and_then(Value::as_table)
        .and_then(|host| host.get("pin"))
    else {
        return Ok(vec![]);
    };
    let given = given
        .as_array()
        .ok_or("[host] pin is not a list of dotted keys".to_string())?;
    let mut pins = vec![];
    for pin in given {
        let pin = pin
            .as_str()
            .ok_or(format!("[host] pin holds {pin}, which is not a dotted key"))?;
        let path = key_path(pin).ok_or(format!(
            "[host] pin holds {pin:?}, which is not a dotted key"
        ))?;
        if !is_local_key(&path) && !leads_to_local_key(&path) {
            return Err(format!(
                "[host] pin holds {pin:?}, which names nothing the local layer may set"
            ));
        }
        pins.push((pin.to_string(), as_read(&path)));
    }
    Ok(pins)
}

/// The path the dotted key `key` names in TOML's spelling, read by parsing
/// `<key> = 0` as a document; none where that is not one key.
fn key_path(key: &str) -> Option<Vec<String>> {
    let mut table: Table = format!("{key} = 0").parse().ok()?;
    let mut path = vec![];
    loop {
        if table.len() != 1 {
            return None;
        }
        let (name, value) = table.into_iter().next()?;
        path.push(name);
        match value {
            Value::Table(inner) => table = inner,
            Value::Integer(0) => return Some(path),
            _ => return None,
        }
    }
}

/// Sets `value` at `path` in `table`, replacing what stands there whole,
/// each table on the way made where there is none; a repository's table
/// is the one whose key reads as the same location, `~` the home, where
/// the table has one.
fn set_at(table: &mut Table, path: &[String], value: Value) {
    let mut table = table;
    for (n, segment) in path.iter().enumerate() {
        let segment = if n == 1 && path[0] == "repos" {
            let location = home(segment);
            table
                .keys()
                .find(|key| home(key) == location)
                .cloned()
                .unwrap_or_else(|| segment.clone())
        } else {
            segment.clone()
        };
        if n + 1 == path.len() {
            table.insert(segment, value);
            return;
        }
        let entry = table
            .entry(segment)
            .or_insert_with(|| Value::Table(Table::new()));
        if !entry.is_table() {
            *entry = Value::Table(Table::new());
        }
        table = entry.as_table_mut().expect("a table was made");
    }
}

/// Where a value of the configuration lies: its path, its dotted key as
/// TOML writes it, whether it lies in no array, so its path may be a key
/// of [`LOCAL_KEYS`], and whether it lies in an `env` table.
struct Leaf {
    path: Vec<String>,
    key: String,
    listed: bool,
    secret: bool,
}

/// Every value `value` at `at` holds, pushed onto `values` by its dotted
/// key: a unit of [`LOCAL_KEYS`] whole, an array of tables by index
/// (`pools.work.accounts[0].name`), an `env` table's values and an
/// account's `usage` as `<set>` and never as written, and every other
/// leaf as TOML writes it; each local where it lies at or under a unit of
/// `taken`, else managed (184, C.7; core.30, core.32).
fn values_of(value: &Value, at: Leaf, taken: &[Vec<String>], values: &mut Vec<ConfigValue>) {
    let layer = if at.listed && taken.iter().any(|unit| as_read(&at.path).starts_with(unit)) {
        ConfigLayer::Local
    } else {
        ConfigLayer::Managed
    };
    let mut push = |value: String| {
        values.push(ConfigValue {
            key: at.key.clone(),
            value,
            layer,
        })
    };
    let env = at.path.last().is_some_and(|name| name == "env");
    let reading = at.path.first().is_some_and(|name| name == "pools")
        && at.path.last().is_some_and(|name| name == "usage");
    match value {
        Value::Table(table) if at.secret || env || !(at.listed && is_unit(&at.path, value)) => {
            for (name, inner) in table {
                let mut path = at.path.clone();
                path.push(name.clone());
                let inner_at = Leaf {
                    path,
                    key: format!("{}.{}", at.key, quoted(name)),
                    listed: at.listed,
                    secret: at.secret || env,
                };
                values_of(inner, inner_at, taken, values);
            }
        }
        _ if at.secret || env || reading || own_credential(&at.path, value) => {
            push(SET.to_string())
        }
        Value::Array(items)
            if !items.is_empty()
                && items.iter().all(Value::is_table)
                && !(at.listed && is_unit(&at.path, value)) =>
        {
            for (n, item) in items.iter().enumerate() {
                let item_at = Leaf {
                    path: at.path.clone(),
                    key: format!("{}[{n}]", at.key),
                    listed: false,
                    secret: at.secret,
                };
                values_of(item, item_at, taken, values);
            }
        }
        _ => push(value.to_string()),
    }
}

/// Whether `value`, at `path`, is a program's own setting given inline,
/// `…programs.<program>.settings.<key>…`, holding a string shaped as a
/// credential, which status never gives as written: the launch and the
/// start refuse it, and the configuration's values give it as `<set>`
/// (195; design 7; core.50).
fn own_credential(path: &[String], value: &Value) -> bool {
    let own = path
        .windows(3)
        .any(|run| run[0] == "programs" && run[2] == "settings")
        && path.len() > 3;
    fn shaped(name: &str, value: &Value) -> bool {
        match value {
            Value::String(text) => flume_core::shaped_as_credential(name, text),
            Value::Array(items) => items.iter().any(|item| shaped(name, item)),
            Value::Table(table) => table.iter().any(|(key, inner)| shaped(key, inner)),
            _ => false,
        }
    }
    own && shaped(path.last().map_or("", String::as_str), value)
}

/// The directory beside `config.toml` holding a file for each source of
/// kinds the host declares (191, C.7).
pub const SOURCES: &str = "sources";

/// The host's lock of kinds, beside `config.toml` (155, C.7).
pub const KINDS_LOCK: &str = "flume.lock";

/// What ends the name of a source's file.
const SOURCE_FILE: &str = ".toml";

/// Every source the sources directory `dir` declares, by name: none where
/// there is no directory, and why it cannot be listed where it cannot.
/// Each file's fault is its own source's (191, C.7; design 1).
pub fn read_sources(dir: &Path) -> Result<BTreeMap<String, HostSource>, String> {
    let listed = |error: std::io::Error| format!("{}: {error}", dir.display());
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeMap::new()),
        Err(error) => return Err(listed(error)),
    };
    let mut sources = BTreeMap::new();
    for entry in entries {
        let entry = entry.map_err(listed)?;
        let file_name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        let Some(name) = file_name.strip_suffix(SOURCE_FILE) else {
            continue;
        };
        if file_name.starts_with('.') || !path.is_file() {
            continue;
        }
        let read = match source_of(name, &path) {
            Ok(kinds_dir) => HostSourceRead::Read { kinds_dir },
            Err(reason) => HostSourceRead::CouldNotRead { reason },
        };
        sources.insert(
            name.to_string(),
            HostSource {
                file: path.to_string_lossy().into_owned(),
                read,
            },
        );
    }
    Ok(sources)
}

/// The directory a source's file at `path` declares the source `name`'s
/// packages in, or every fault of the file (154, 191, C.7).
fn source_of(name: &str, path: &Path) -> Result<String, String> {
    let mut faults: Vec<String> = source_fault(name).into_iter().collect();
    let read = std::fs::read_to_string(path)
        .map_err(|error| format!("could not be read: {error}"))
        .and_then(|text| {
            text.parse::<Table>()
                .map_err(|error| format!("is not TOML: {}", error.message().trim()))
        });
    let table = match read {
        Ok(table) => table,
        Err(fault) => {
            faults.push(fault);
            return Err(faults.join("; "));
        }
    };
    for key in table.keys().filter(|key| *key != "kinds_dir") {
        faults.push(format!(
            "`{key}` is no key of a source's file, which gives `kinds_dir` alone"
        ));
    }
    let kinds_dir = match table.get("kinds_dir") {
        None => {
            faults.push("gives no `kinds_dir`, the directory holding the source's packages".into());
            None
        }
        Some(Value::String(given)) => {
            let read = home(given);
            if Path::new(&read).is_absolute() {
                Some(read)
            } else {
                faults.push(format!(
                    "`kinds_dir` is `{given}`, which is neither absolute nor under `~`"
                ));
                None
            }
        }
        Some(_) => {
            faults.push("`kinds_dir` is not a path".into());
            None
        }
    };
    match kinds_dir {
        Some(kinds_dir) if faults.is_empty() => Ok(kinds_dir),
        _ => Err(faults.join("; ")),
    }
}

/// A key as TOML writes it in a table's name.
fn quoted(key: &str) -> String {
    if !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        key.to_string()
    } else {
        format!("{key:?}")
    }
}

/// A repository's location with a leading `~` read as `$HOME`.
fn home(location: &str) -> String {
    match (location.strip_prefix('~'), std::env::var("HOME")) {
        (Some(rest), Ok(home)) if rest.is_empty() || rest.starts_with('/') => {
            format!("{home}{rest}")
        }
        _ => location.to_string(),
    }
}

/// A `kinds` table of the configuration: each kind's `agents` table, by
/// the kind's name; a kind's table holding no `agents` gives nothing.
fn kinds_settings(
    kinds: &Value,
    at: &str,
    directory: &Path,
) -> Result<BTreeMap<String, KindSettings>, String> {
    let kinds = kinds
        .as_table()
        .ok_or(format!("[{at}] is not a table of kinds"))?;
    let mut settings = BTreeMap::new();
    for (kind, table) in kinds {
        let at = format!("{at}.{}", quoted(kind));
        let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
        let Some(agents) = table.get("agents") else {
            continue;
        };
        let at = format!("{at}.agents");
        let agents = agents
            .as_table()
            .ok_or(format!("[{at}] is not a table of settings"))?;
        let mut stages = BTreeMap::new();
        if let Some(named) = agents.get("stages") {
            let named = named
                .as_table()
                .ok_or(format!("[{at}.stages] is not a table of stages"))?;
            for (stage, own) in named {
                let at = format!("{at}.stages.{}", quoted(stage));
                let own = own.as_table().ok_or(format!("[{at}] is not a table"))?;
                stages.insert(stage.clone(), agent_settings(own, &at, directory)?);
            }
        }
        settings.insert(
            kind.clone(),
            KindSettings {
                every_stage: agent_settings(agents, &at, directory)?,
                stages,
            },
        );
    }
    Ok(settings)
}

/// A text word of `table`, none where it is left out, refused naming it
/// where it is no string.
fn text_of(table: &Table, name: &str, at: &str) -> Result<Option<String>, String> {
    match table.get(name) {
        None => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(format!("[{at}] {name} is not a string")),
    }
}

/// The host's multiplexer: `[host] multiplexer`, or `[host] terminal`,
/// the word written before it and read as it; a `[host]` giving both
/// refused naming both, and a value that is no name refused naming its
/// key (105, 190, C.7).
fn host_multiplexer(host: &Table) -> Result<Option<String>, String> {
    let named =
        |key: &str| text_of(host, key, "host").map_err(|_| format!("[host] {key} is not a name"));
    match (named("multiplexer")?, named("terminal")?) {
        (Some(_), Some(_)) => Err(
            "[host] gives both terminal and multiplexer; terminal is the \
             word written before multiplexer, so give one"
                .to_string(),
        ),
        (multiplexer, terminal) => Ok(multiplexer.or(terminal)),
    }
}

/// The `view` of `table`, `terminal` or `app`, none where it is left out,
/// any other value refused naming its key and the two values (186, C.7).
fn view_of(table: &Table, at: &str) -> Result<Option<View>, String> {
    match table.get("view") {
        None => Ok(None),
        Some(Value::String(word)) if word == View::Terminal.name() => Ok(Some(View::Terminal)),
        Some(Value::String(word)) if word == View::App.name() => Ok(Some(View::App)),
        Some(value) => Err(format!(
            "[{at}] view is {value}, where a view is terminal or app"
        )),
    }
}

/// A bound of `table`, a whole number above zero, none where it is left
/// out (143, 144).
fn bound_of(table: &Table, name: &str, at: &str) -> Result<Option<u32>, String> {
    let Some(value) = table.get(name) else {
        return Ok(None);
    };
    value
        .as_integer()
        .and_then(|bound| u32::try_from(bound).ok())
        .filter(|bound| *bound > 0)
        .map(Some)
        .ok_or(format!("[{at}] {name} is not a whole number above zero"))
}

/// Refuses a key of `table` that is none of `words`, naming it.
fn only(table: &Table, words: &[&str], at: &str, what: &str) -> Result<(), String> {
    match table.keys().find(|key| !words.contains(&key.as_str())) {
        Some(unknown) => Err(format!("[{at}] {unknown} is no word of {what}")),
        None => Ok(()),
    }
}

/// The `stream` each kind's table of a `kinds` table names, by the
/// kind's name (143).
fn kinds_streams(kinds: &Value, at: &str) -> Result<BTreeMap<String, String>, String> {
    let kinds = kinds
        .as_table()
        .ok_or(format!("[{at}] is not a table of kinds"))?;
    let mut streams = BTreeMap::new();
    for (kind, table) in kinds {
        let at = format!("{at}.{}", quoted(kind));
        let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
        if let Some(stream) = text_of(table, "stream", &at)? {
            streams.insert(kind.clone(), stream);
        }
    }
    Ok(streams)
}

/// The `label` each kind's table of a `kinds` table gives, by the kind's
/// name: one word of letters, digits, `-` and `_`, any other value
/// refused naming its key (180, C.7).
fn kinds_labels(kinds: &Value, at: &str) -> Result<BTreeMap<String, String>, String> {
    let kinds = kinds
        .as_table()
        .ok_or(format!("[{at}] is not a table of kinds"))?;
    let mut labels = BTreeMap::new();
    for (kind, table) in kinds {
        let at = format!("{at}.{}", quoted(kind));
        let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
        let Some(value) = table.get("label") else {
            continue;
        };
        let label = value
            .as_str()
            .filter(|label| is_label(label))
            .ok_or(format!(
                "{at}.label is {value}, where a label is one word of letters, digits, - and _"
            ))?;
        labels.insert(kind.clone(), label.to_string());
    }
    Ok(labels)
}

/// Whether `word` is a label: one word of letters, digits, `-` and `_`
/// (180).
fn is_label(word: &str) -> bool {
    !word.is_empty()
        && word
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// A stream's `labels` table: each label's share, its `reserve` and
/// `keep_free` whole numbers from zero, zero where left out, and each
/// other key of its table kept unknown, which refuses the stream's starts
/// rather than the configuration (181, 182, C.7).
fn shares_of(table: &Table, at: &str) -> Result<BTreeMap<String, Share>, String> {
    let Some(labels) = table.get("labels") else {
        return Ok(BTreeMap::new());
    };
    let labels = labels
        .as_table()
        .ok_or(format!("[{at}.labels] is not a table of labels"))?;
    let mut shares = BTreeMap::new();
    for (label, share) in labels {
        let at = format!("{at}.labels.{}", quoted(label));
        let share = share.as_table().ok_or(format!("[{at}] is not a table"))?;
        let count = |name: &str| match share.get(name) {
            None => Ok(0),
            Some(value) => value
                .as_integer()
                .and_then(|count| u32::try_from(count).ok())
                .ok_or(format!("[{at}] {name} is not a whole number from zero")),
        };
        shares.insert(
            label.clone(),
            Share {
                reserve: count("reserve")?,
                keep_free: count("keep_free")?,
                unknown: share
                    .keys()
                    .filter(|key| !matches!(key.as_str(), "reserve" | "keep_free"))
                    .cloned()
                    .collect(),
            },
        );
    }
    Ok(shares)
}

/// A stream's `kinds` table: the session each kind's table names, and each
/// of its stages' tables, by the kind's name and the stage's; each other
/// key of either table kept unknown, which refuses the starts that read
/// it rather than the configuration; and a `session` that is no string,
/// or a `stages` that is no table of tables, refused naming its table and
/// the key (199, C.7).
fn kinds_sessions(table: &Table, at: &str) -> Result<BTreeMap<String, KindSessions>, String> {
    let Some(kinds) = table.get("kinds") else {
        return Ok(BTreeMap::new());
    };
    let kinds = kinds
        .as_table()
        .ok_or(format!("[{at}.kinds] is not a table of kinds"))?;
    let unknown = |table: &Table, words: &[&str]| -> Vec<String> {
        table
            .keys()
            .filter(|key| !words.contains(&key.as_str()))
            .cloned()
            .collect()
    };
    let mut sessions = BTreeMap::new();
    for (kind, own) in kinds {
        let at = format!("{at}.kinds.{}", quoted(kind));
        let own = own.as_table().ok_or(format!("[{at}] is not a table"))?;
        let mut stages = BTreeMap::new();
        if let Some(named) = own.get("stages") {
            let named = named
                .as_table()
                .ok_or(format!("[{at}.stages] is not a table of stages"))?;
            for (stage, table) in named {
                let at = format!("{at}.stages.{}", quoted(stage));
                let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
                stages.insert(
                    stage.clone(),
                    StageSession {
                        session: text_of(table, "session", &at)?,
                        unknown: unknown(table, &["session"]),
                    },
                );
            }
        }
        sessions.insert(
            kind.clone(),
            KindSessions {
                session: text_of(own, "session", &at)?,
                stages,
                unknown: unknown(own, &["session", "stages"]),
            },
        );
    }
    Ok(sessions)
}

/// The `streams` table: each stream with its bound, its priority, zero
/// where it gives none, its pool, its session, the sessions it names for
/// its kinds and their stages, its multiplexer, its view, its terminal
/// app, its title and its labels' shares; a key a stream does not know
/// refused naming it (107, 143, 181, 186, 190, 199, C.7).
fn streams_declared(streams: &Value) -> Result<BTreeMap<String, Stream>, String> {
    let streams = streams
        .as_table()
        .ok_or("[streams] is not a table of streams".to_string())?;
    let mut declared = BTreeMap::new();
    for (name, table) in streams {
        let at = format!("streams.{}", quoted(name));
        let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
        only(
            table,
            &[
                "agents",
                "priority",
                "pool",
                "session",
                "multiplexer",
                "view",
                "terminal_app",
                "title",
                "labels",
                "kinds",
            ],
            &at,
            "a stream",
        )?;
        let priority = match table.get("priority") {
            None => 0,
            Some(value) => value
                .as_integer()
                .ok_or(format!("[{at}] priority is not a whole number"))?,
        };
        declared.insert(
            name.clone(),
            Stream {
                agents: bound_of(table, "agents", &at)?,
                priority,
                pool: text_of(table, "pool", &at)?,
                session: text_of(table, "session", &at)?,
                kinds: kinds_sessions(table, &at)?,
                labels: shares_of(table, &at)?,
                multiplexer: text_of(table, "multiplexer", &at)?,
                view: view_of(table, &at)?,
                terminal_app: text_of(table, "terminal_app", &at)?,
                title: text_of(table, "title", &at)?,
            },
        );
    }
    Ok(declared)
}

/// The `pools` table: each pool's strategy, its optional threshold and
/// its accounts in order, each with a name, a program, an `env` kept as
/// written, an optional bound and an optional `usage`, the path of its
/// reading kept as written; a pool with no accounts, two accounts of one
/// name, an account with no name or program, a value either cannot hold,
/// or a key either does not know refused naming it (144, 184, 185, C.7).
fn pools_declared(pools: &Value) -> Result<BTreeMap<String, Pool>, String> {
    let pools = pools
        .as_table()
        .ok_or("[pools] is not a table of pools".to_string())?;
    let mut declared = BTreeMap::new();
    for (name, table) in pools {
        let at = format!("pools.{}", quoted(name));
        let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
        only(table, &["strategy", "threshold", "accounts"], &at, "a pool")?;
        let strategy = match text_of(table, "strategy", &at)?.as_deref() {
            Some("fill-first") => Strategy::FillFirst,
            Some("least-busy") => Strategy::LeastBusy,
            Some("most-left") => Strategy::MostLeft,
            Some(other) => {
                return Err(format!(
                "[{at}] strategy is {other}, where a pool's strategy is fill-first, least-busy or most-left"
            ))
            }
            None => return Err(format!("[{at}] names no strategy")),
        };
        let threshold = threshold_of(table, &at)?;
        let given = match table.get("accounts") {
            None => vec![],
            Some(Value::Array(given)) => given.clone(),
            Some(_) => return Err(format!("[{at}] accounts is not a list of accounts")),
        };
        if given.is_empty() {
            return Err(format!("[{at}] accounts names no account"));
        }
        let mut accounts: Vec<Account> = vec![];
        for (n, account) in given.iter().enumerate() {
            let at = format!("{at}.accounts[{n}]");
            let account = account.as_table().ok_or(format!("[{at}] is not a table"))?;
            only(
                account,
                &["name", "program", "env", "agents", "usage"],
                &at,
                "an account",
            )?;
            let name = text_of(account, "name", &at)?.ok_or(format!("[{at}] names no name"))?;
            if accounts.iter().any(|held| held.name == name) {
                return Err(format!(
                    "[{at}] name {name} names a second account of the pool"
                ));
            }
            accounts.push(Account {
                program: text_of(account, "program", &at)?
                    .ok_or(format!("[{at}] names no program"))?,
                env: env_of(account, &at)?,
                agents: bound_of(account, "agents", &at)?,
                usage: usage_of(account, &at)?,
                name,
            });
        }
        declared.insert(
            name.clone(),
            Pool {
                strategy,
                threshold,
                accounts,
            },
        );
    }
    Ok(declared)
}

/// A pool's `threshold`, a whole percentage from 1 to 100, none where it
/// is left out and 90 holds (185, C.7).
fn threshold_of(table: &Table, at: &str) -> Result<Option<u32>, String> {
    let Some(value) = table.get("threshold") else {
        return Ok(None);
    };
    value
        .as_integer()
        .and_then(|threshold| u32::try_from(threshold).ok())
        .filter(|threshold| (1..=100).contains(threshold))
        .map(Some)
        .ok_or(format!(
            "[{at}] threshold is not a whole percentage from 1 to 100"
        ))
}

/// An account's `usage`, the path of its reading kept as written, its `~`
/// and `$VAR` read only when the reading is asked; none where it is left
/// out (184, C.7).
fn usage_of(account: &Table, at: &str) -> Result<Option<String>, String> {
    match text_of(account, "usage", at)? {
        Some(usage) if usage.is_empty() => Err(format!("[{at}] usage is empty")),
        usage => Ok(usage),
    }
}

/// An `env` table of variables, each value kept as written (29).
fn env_of(table: &Table, at: &str) -> Result<BTreeMap<String, String>, String> {
    let mut env = BTreeMap::new();
    if let Some(given) = table.get("env") {
        let given = given
            .as_table()
            .ok_or(format!("[{at}] env is not a table of variables"))?;
        for (name, value) in given {
            let value = value
                .as_str()
                .ok_or(format!("[{at}] env.{name} is not a string"))?;
            env.insert(name.clone(), value.to_string());
        }
    }
    Ok(env)
}

/// The runs the `roles` table names, by the address (132, C.7).
type Roles = BTreeMap<String, RoleRun>;

/// The standing agents the `roles` table names, by the address (132, C.7).
type StandingAgents = BTreeMap<String, StandingAgent>;

/// The `roles` table: each address's run, its repository and kind named,
/// its priority zero, its base the repository's default branch and its
/// brief none where the table gives none; and each address an entry
/// giving `agent` names a standing agent for, with its `session` and
/// `stream` where it gives them. A key that is no address, a key the run
/// does not know, `session` among them, an `agent` beside `kind`, which
/// would be two agents of one role, a key of a run's beside `agent`, or an
/// `agent` or a `session` that is empty or no string, refused naming the
/// address and the key (3, 132, C.7).
fn role_runs(roles: &Value) -> Result<(Roles, StandingAgents), String> {
    let roles = roles
        .as_table()
        .ok_or("[roles] is not a table of addresses".to_string())?;
    let mut runs = BTreeMap::new();
    let mut standing = BTreeMap::new();
    for (address, table) in roles {
        let at = format!("roles.{}", quoted(address));
        if !is_address(address) {
            return Err(format!(
                "[{at}] names no role's address, which is <partition>/<role>"
            ));
        }
        let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
        let text = |name: &str| match table.get(name) {
            None => Ok(None),
            Some(Value::String(value)) => Ok(Some(value.clone())),
            Some(_) => Err(format!("[{at}] {name} is not a string")),
        };
        if table.contains_key("agent") {
            standing.insert(address.clone(), standing_agent(table, &at)?);
            continue;
        }
        if let Some(unknown) = table.keys().find(|key| {
            !matches!(
                key.as_str(),
                "repository" | "kind" | "priority" | "base" | "brief" | "stream"
            )
        }) {
            return Err(format!("[{at}] {unknown} is no word of a role's run"));
        }
        let named = |name: &str| text(name)?.ok_or(format!("[{at}] names no {name}"));
        let priority = match table.get("priority") {
            None => 0,
            Some(value) => value
                .as_integer()
                .ok_or(format!("[{at}] priority is not a whole number"))?,
        };
        runs.insert(
            address.clone(),
            RoleRun {
                repository: home(&named("repository")?),
                kind: named("kind")?,
                priority,
                base: text("base")?,
                brief: text("brief")?,
                stream: text("stream")?,
            },
        );
    }
    Ok((runs, standing))
}

/// The standing agent an entry of the `roles` table at `at` names by its
/// `agent`, with its `session` and `stream` where it gives them; a `kind`
/// beside it, a run's other key or any key a standing agent's entry does
/// not know, or an `agent` or a `session` that is empty or no string,
/// refused naming the address and the key. The name is the terminal's to
/// hold or never list, and flume checks no rule of its own on it (3, 132,
/// C.7; core.44).
fn standing_agent(table: &Table, at: &str) -> Result<StandingAgent, String> {
    if table.contains_key("kind") {
        return Err(format!(
            "[{at}] gives both a standing agent and a kind, two agents of one role"
        ));
    }
    if let Some(unknown) = table
        .keys()
        .find(|key| !matches!(key.as_str(), "agent" | "session" | "stream"))
    {
        return Err(format!(
            "[{at}] {unknown} is no word of a standing agent's entry"
        ));
    }
    let name = |key: &str| match table.get(key) {
        None => Ok(None),
        Some(Value::String(value)) if value.is_empty() => Err(format!("[{at}] {key} is empty")),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(format!("[{at}] {key} is not a string")),
    };
    Ok(StandingAgent {
        agent: name("agent")?.ok_or(format!("[{at}] names no agent"))?,
        session: name("session")?,
        stream: match table.get("stream") {
            None => None,
            Some(Value::String(stream)) => Some(stream.clone()),
            Some(_) => return Err(format!("[{at}] stream is not a string")),
        },
    })
}

/// The host's `[agents]`, its settings for every agent, and each of its
/// `[agents.streams.<stream>]`, every agent's in a stream `streams`
/// declares, by the stream's name; a `program`, an `env`, a `beside` or a
/// `stages` in either, or a stream no `[streams]` table declares, refused
/// naming the table and the key (32, 143, 195, C.7; record 0043).
fn every_agent(
    agents: &Value,
    streams: &BTreeMap<String, Stream>,
    directory: &Path,
) -> Result<(Settings, BTreeMap<String, Settings>), String> {
    const NAMES: &[&str] = &["program", "env", "beside", "stages"];
    let agents = agents
        .as_table()
        .ok_or("[agents] is not a table of settings".to_string())?;
    refuse(agents, "agents", "for every agent", NAMES)?;
    let mut by_stream = BTreeMap::new();
    if let Some(named) = agents.get("streams") {
        let named = named
            .as_table()
            .ok_or("[agents.streams] is not a table of streams".to_string())?;
        for (stream, table) in named {
            let at = format!("agents.streams.{}", quoted(stream));
            if !streams.contains_key(stream) {
                return Err(format!(
                    "[{at}] names the stream {stream}, which no [streams] table declares"
                ));
            }
            let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
            refuse(table, &at, "for every agent in a stream", NAMES)?;
            by_stream.insert(stream.clone(), agent_settings(table, &at, directory)?);
        }
    }
    Ok((agent_settings(agents, "agents", directory)?, by_stream))
}

/// A repository's `[repos."<repository>".agents]` at `at`, the host's
/// settings for every kind in it; a `program`, an `env`, a `beside`, a
/// `stages` or a `streams` refused naming the table and the key (32, 195,
/// C.7; record 0043).
fn every_kind(agents: &Value, at: &str, directory: &Path) -> Result<Settings, String> {
    let agents = agents
        .as_table()
        .ok_or(format!("[{at}] is not a table of settings"))?;
    refuse(
        agents,
        at,
        "for every kind of a repository",
        &["program", "env", "beside", "stages", "streams"],
    )?;
    agent_settings(agents, at, directory)
}

/// Refuses the first of `names` that the agents table `table` at `at`
/// holds, naming the table, the key and where its word goes instead (32,
/// 107, 143, 144, C.7).
fn refuse(table: &Table, at: &str, whose: &str, names: &[&str]) -> Result<(), String> {
    let Some(name) = names.iter().find(|name| table.contains_key(**name)) else {
        return Ok(());
    };
    let instead = match *name {
        "program" => "the host chooses a program by the accounts it pools",
        "env" => "an agent's environment is its account's alone",
        "beside" => "the pane an agent opens beside is the launch's to name",
        "stages" => {
            "a stage's name is its kind's, so a stage's settings go in [kinds.<kind>.agents.stages.<stage>]"
        }
        _ => "a stream's settings go in [agents.streams.<stream>]",
    };
    Err(format!(
        "[{at}] {name} is no setting of the host's {whose}: {instead}"
    ))
}

/// One agents table's `model`, `instructions`, `mode`, `keep_open` and
/// `programs`, each as written, the mode read as a declaration's is (162,
/// 169), `keep_open` by [`keep_open_of`] (198), and `programs` by flume's
/// one reader of them, a file read by [`host_file`] (195); any other key,
/// `stages` among them, is not a setting of the host's (32, C.7; record
/// 0020).
fn agent_settings(table: &Table, at: &str, directory: &Path) -> Result<Settings, String> {
    let text = |name: &str| match table.get(name) {
        None => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(format!("[{at}] {name} is not a string")),
    };
    let mode = table
        .get("mode")
        .map(crate::mode::read)
        .transpose()
        .map_err(|reason| format!("[{at}] mode: {reason}"))?;
    let programs = table
        .get("programs")
        .map(|programs| crate::own::read(programs, at, &|written| host_file(directory, written)))
        .transpose()?
        .unwrap_or_default();
    Ok(Settings {
        model: text("model")?,
        instructions: text("instructions")?,
        mode,
        programs,
        keep_open: keep_open_of(table, at)?,
        ..Settings::default()
    })
}

/// An agents table's `keep_open` at `at`, how long its agents are kept
/// open for an answer: a string [`read_span`] takes, kept as written,
/// none where the table gives none; any other value, a table of programs
/// among them, refused naming the table and the key, since it is flume's
/// one time for every program (32, 198, C.7).
fn keep_open_of(table: &Table, at: &str) -> Result<Option<String>, String> {
    match table.get("keep_open") {
        None => Ok(None),
        Some(Value::String(written)) => read_span(written)
            .map(|_| Some(written.clone()))
            .map_err(|reason| format!("{at}.keep_open: {reason}")),
        Some(Value::Table(_)) => Err(format!(
            "{at}.keep_open is one time for every program, never a table of programs: \
             it is <n>, <n>s, <n>m or <n>h"
        )),
        Some(_) => Err(format!(
            "{at}.keep_open is not a string: it is <n>, <n>s, <n>m or <n>h"
        )),
    }
}

/// A settings file a host's agents table names at `written`, read on the
/// host now: absolute, beginning `~/` read against the home as a
/// repository's key is, or relative to `directory`, the configuration's,
/// read where it leads when it is a link, and with no `$VAR` read (195,
/// C.7; core.30). Its path read and its text, or why it cannot be read,
/// naming it.
fn host_file(directory: &Path, written: &str) -> Result<(String, String), String> {
    let path = directory.join(home(written));
    let read = path.display().to_string();
    std::fs::read_to_string(&path)
        .map(|text| (read.clone(), text))
        .map_err(|error| format!("{read} could not be read: {error}"))
}

/// The directory a configuration file lies in, where its settings files
/// are read relative to.
fn directory_of(file: &Path) -> PathBuf {
    file.parent().map(Path::to_path_buf).unwrap_or_default()
}

/// The host's configuration directory, `$XDG_CONFIG_HOME/flume`; none
/// where neither it nor `$HOME` is set, so a relative file is read
/// against the working directory.
fn host_directory() -> PathBuf {
    config_file()
        .map(|file| directory_of(&file))
        .unwrap_or_default()
}

/// An XDG directory: the variable `name` where it names an absolute
/// path, otherwise `$HOME/<under>`.
fn xdg(name: &str, under: &str) -> Result<PathBuf, String> {
    if let Some(dir) = std::env::var_os(name)
        .map(PathBuf::from)
        .filter(|dir| dir.is_absolute())
    {
        return Ok(dir);
    }
    std::env::var_os("HOME")
        .map(|home| PathBuf::from(home).join(under))
        .ok_or_else(|| format!("neither ${name} nor $HOME is set"))
}

/// flume's state on the host, `$XDG_STATE_HOME/flume` (C.7).
pub fn flume_state() -> Result<PathBuf, String> {
    xdg("XDG_STATE_HOME", ".local/state").map(|dir| dir.join("flume"))
}

/// flume's own data on the host, `$XDG_DATA_HOME/flume` (C.7).
pub fn flume_data() -> Result<PathBuf, String> {
    xdg("XDG_DATA_HOME", ".local/share").map(|dir| dir.join("flume"))
}

/// The host's configuration file, `$XDG_CONFIG_HOME/flume/config.toml`.
pub fn config_file() -> Result<PathBuf, String> {
    xdg("XDG_CONFIG_HOME", ".config").map(|dir| dir.join("flume").join("config.toml"))
}

/// The host's local layer, `$XDG_CONFIG_HOME/flume/config.local.toml`
/// (C.7).
pub fn local_config_file() -> Result<PathBuf, String> {
    config_file().map(|file| file.with_file_name("config.local.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use flume_seams::{Mode, ModeSetting};

    #[test]
    fn a_bound_and_a_terminal_stated() {
        let config = HostConfig::parse("[host]\nagents = 2\nterminal = \"pane\"\n").unwrap();
        assert_eq!(config.agents, Some(2));
        assert_eq!(config.terminal.as_deref(), Some("pane"));
    }

    #[test]
    fn a_word_left_out_is_none() {
        assert_eq!(HostConfig::parse("").unwrap(), HostConfig::default());
        assert_eq!(
            HostConfig::parse("[host]\nagents = 3\n").unwrap(),
            HostConfig {
                agents: Some(3),
                ..HostConfig::default()
            }
        );
    }

    #[test]
    fn settings_for_a_kind_and_for_a_repositorys_kind() {
        let config = HostConfig::parse(
            "[kinds.change.agents]\nmodel = \"opus\"\ninstructions = \"be brief\"\n\
             [repos.\"/r\".kinds.change.agents.stages.verify]\nmodel = \"sonnet\"\n",
        )
        .unwrap();
        let kind = &config.kinds["change"];
        assert_eq!(kind.every_stage.model.as_deref(), Some("opus"));
        assert_eq!(kind.every_stage.instructions.as_deref(), Some("be brief"));
        let repository = &config.repos["/r"]["change"];
        assert_eq!(repository.every_stage, Settings::default());
        assert_eq!(repository.stages["verify"].model.as_deref(), Some("sonnet"));
    }

    #[test]
    fn a_mode_for_a_kind_and_a_table_for_a_repositorys_stage() {
        // 32, 162, C.7: one of flume's four at the host-kind layer, a
        // table of programs for one stage at the host-repository layer.
        let config = HostConfig::parse(
            "[kinds.fix.agents]\nmode = \"edits\"\n\
             [repos.\"/r\".kinds.fix.agents.stages.code]\nmode = { agent = \"careful\" }\n",
        )
        .unwrap();
        assert_eq!(
            config.kinds["fix"].every_stage.mode,
            Some(ModeSetting::Flume(Mode::Edits))
        );
        let repository = &config.repos["/r"]["fix"];
        assert_eq!(repository.every_stage, Settings::default());
        assert_eq!(
            repository.stages["code"].mode,
            Some(ModeSetting::Programs(BTreeMap::from([(
                "agent".to_string(),
                "careful".to_string()
            )])))
        );
    }

    #[test]
    fn a_mode_that_is_neither_is_refused_naming_the_table_and_mode() {
        // 162, 169, C.7: a program's own value given bare, a number and a
        // table whose entry is no string.
        for mode in ["\"careful\"", "3", "{ agent = 3 }"] {
            let text = format!("[kinds.fix.agents]\nmode = {mode}\n");
            let refused = HostConfig::parse(&text).unwrap_err();
            assert!(
                refused.contains("kinds.fix.agents") && refused.contains("mode"),
                "{text}: {refused}"
            );
        }
        let refused = HostConfig::parse("[repos.\"/r\".kinds.fix.agents.stages.code]\nmode = {}\n")
            .unwrap_err();
        assert!(
            refused.contains("kinds.fix.agents.stages.code") && refused.contains("mode"),
            "{refused}"
        );
    }

    #[test]
    fn a_setting_that_is_no_string_is_refused() {
        let refused = HostConfig::parse("[kinds.change.agents]\nmodel = 4\n").unwrap_err();
        assert!(
            refused.contains("kinds.change.agents") && refused.contains("model"),
            "{refused}"
        );
    }

    #[test]
    fn a_keep_open_for_a_kind_and_for_one_stage() {
        // 32, 198, C.7: the store-profile spec's "A time for a kind and
        // for one stage", each kept as written, in the managed layer and
        // the local one over it.
        let text = "[kinds.change.agents]\nkeep_open = \"30m\"\n\
             [kinds.change.agents.stages.verify]\nkeep_open = \"0\"\n";
        let config = HostConfig::parse(text).unwrap();
        let change = &config.kinds["change"];
        assert_eq!(change.every_stage.keep_open.as_deref(), Some("30m"));
        assert_eq!(change.stages["verify"].keep_open.as_deref(), Some("0"));
        let config = HostConfig::layers(
            text,
            Some("[kinds.change.agents.stages.verify]\nkeep_open = \"90s\"\n"),
        )
        .unwrap();
        assert_eq!(config.configuration.local, LocalLayer::Taken);
        let change = &config.kinds["change"];
        assert_eq!(change.every_stage.keep_open.as_deref(), Some("30m"));
        assert_eq!(change.stages["verify"].keep_open.as_deref(), Some("90s"));
        assert_eq!(
            value_at(&config, "kinds.change.agents.stages.verify.keep_open"),
            Some(("\"90s\"".into(), ConfigLayer::Local))
        );
    }

    #[test]
    fn a_keep_open_for_a_stream_and_for_every_agent() {
        // 32, 143, 198, C.7: the store-profile spec's "A time for a stream
        // and for every agent", and a repository's for every kind and for
        // one kind, each as written.
        let config = HostConfig::parse(
            "[streams.work]\n\
             [agents.streams.work]\nkeep_open = \"10m\"\n\
             [agents]\nkeep_open = \"45m\"\n\
             [repos.\"/r\".agents]\nkeep_open = \"2h\"\n\
             [repos.\"/r\".kinds.fix.agents.stages.code]\nkeep_open = \"5m\"\n",
        )
        .unwrap();
        assert_eq!(
            config.streams_agents["work"].keep_open.as_deref(),
            Some("10m")
        );
        assert_eq!(config.every_agent.keep_open.as_deref(), Some("45m"));
        assert_eq!(config.repos_agents["/r"].keep_open.as_deref(), Some("2h"));
        assert_eq!(
            config.repos["/r"]["fix"].stages["code"]
                .keep_open
                .as_deref(),
            Some("5m")
        );
        let config = HostConfig::layers(
            "[streams.work]\n[agents]\nkeep_open = \"45m\"\n",
            Some("[agents]\nkeep_open = \"15m\"\n[agents.streams.work]\nkeep_open = \"1h\"\n"),
        )
        .unwrap();
        assert_eq!(config.configuration.local, LocalLayer::Taken);
        assert_eq!(config.every_agent.keep_open.as_deref(), Some("15m"));
        assert_eq!(
            config.streams_agents["work"].keep_open.as_deref(),
            Some("1h")
        );
    }

    #[test]
    fn a_keep_open_in_another_form_is_refused() {
        // C.7: the store-profile spec's "A time in another form", named by
        // its table and key with the forms it takes; a local one so
        // refused leaves the local layer untaken, naming why.
        let forms = "<n>, <n>s, <n>m or <n>h";
        for (text, named) in [
            (
                "[kinds.fix.agents]\nkeep_open = \"an hour\"\n",
                "kinds.fix.agents.keep_open",
            ),
            (
                "[kinds.fix.agents.stages.code]\nkeep_open = \"+5m\"\n",
                "kinds.fix.agents.stages.code.keep_open",
            ),
            ("[agents]\nkeep_open = 3600\n", "agents.keep_open"),
        ] {
            let refused = HostConfig::parse(text).unwrap_err();
            assert!(
                refused.contains(named) && refused.contains(forms),
                "{text}: {refused}"
            );
        }
        let config = HostConfig::layers(
            "[agents]\nkeep_open = \"1h\"\n",
            Some("[kinds.fix.agents]\nkeep_open = \"an hour\"\n"),
        )
        .unwrap();
        let LocalLayer::NotTaken { fault, .. } = &config.configuration.local else {
            panic!("the local layer taken: {:?}", config.configuration.local);
        };
        let fault = fault.as_deref().unwrap_or_default();
        assert!(fault.contains("kinds.fix.agents.keep_open"), "{fault}");
        assert_eq!(config.every_agent.keep_open.as_deref(), Some("1h"));
        assert!(!config.kinds.contains_key("fix"), "the managed layer alone");
    }

    #[test]
    fn a_keep_open_table_of_programs_is_refused() {
        // 198, C.7: the store-profile spec's "A table of programs": one
        // time for every program, never a table.
        for (text, named) in [
            (
                "[agents]\nkeep_open = { agent = \"1h\" }\n",
                "agents.keep_open",
            ),
            (
                "[repos.\"/r\".kinds.fix.agents.stages.code.keep_open]\nagent = \"1h\"\n",
                "repos.\"/r\".kinds.fix.agents.stages.code.keep_open",
            ),
        ] {
            let refused = HostConfig::parse(text).unwrap_err();
            assert!(
                refused.contains(named) && refused.contains("one time for every program"),
                "{text}: {refused}"
            );
        }
    }

    #[test]
    fn a_role_named() {
        let config = HostConfig::parse(
            "[roles.\"madswan/planner\"]\nrepository = \"/r\"\nkind = \"planner\"\npriority = 5\n",
        )
        .unwrap();
        assert_eq!(
            config.roles["madswan/planner"],
            RoleRun {
                repository: "/r".into(),
                kind: "planner".into(),
                priority: 5,
                base: None,
                brief: None,
                stream: None,
            },
            "no base, the repository's default branch, no brief and no stream"
        );
    }

    #[test]
    fn a_role_with_a_base_and_a_brief() {
        let config = HostConfig::parse(
            "[roles.\"madswan/planner\"]\nrepository = \"/r\"\nkind = \"planner\"\npriority = 5\n\
             base = \"next\"\nbrief = \"Read the records and plan the next unit.\"\n",
        )
        .unwrap();
        let run = &config.roles["madswan/planner"];
        assert_eq!(run.base.as_deref(), Some("next"));
        assert_eq!(
            run.brief.as_deref(),
            Some("Read the records and plan the next unit.")
        );
    }

    #[test]
    fn two_roles() {
        let config = HostConfig::parse(
            "[host]\nagents = 2\n\
             [roles.\"madswan/planner\"]\nrepository = \"/r\"\nkind = \"planner\"\npriority = 5\n\
             [roles.\"madswan/ops\"]\nrepository = \"/s\"\nkind = \"ops\"\n",
        )
        .unwrap();
        assert_eq!(config.agents, Some(2));
        assert_eq!(
            config.roles.keys().collect::<Vec<_>>(),
            vec!["madswan/ops", "madswan/planner"]
        );
        assert_eq!(config.roles["madswan/ops"].repository, "/s");
        assert_eq!(config.roles["madswan/ops"].priority, 0, "none given");
    }

    #[test]
    fn no_role_named() {
        let config = HostConfig::parse("[host]\nagents = 2\n").unwrap();
        assert!(config.roles.is_empty());
        assert_eq!(config.roles.get("madswan/planner"), None);
    }

    #[test]
    fn a_role_the_table_cannot_run_is_refused() {
        for (text, named) in [
            (
                "[roles.planner]\nrepository = \"/r\"\nkind = \"planner\"\n",
                "address",
            ),
            ("[roles.\"a/b\"]\nkind = \"planner\"\n", "repository"),
            ("[roles.\"a/b\"]\nrepository = \"/r\"\n", "kind"),
            (
                "[roles.\"a/b\"]\nrepository = \"/r\"\nkind = \"k\"\npriority = \"high\"\n",
                "priority",
            ),
            (
                "[roles.\"a/b\"]\nrepository = \"/r\"\nkind = \"k\"\nbranch = \"main\"\n",
                "branch",
            ),
        ] {
            let refused = HostConfig::parse(text).unwrap_err();
            assert!(refused.contains(named), "{refused}");
        }
    }

    /// role-waking, "A standing agent's entry read" (132, C.7).
    #[test]
    fn a_standing_agents_entry_read() {
        let config = HostConfig::parse(
            "[roles.\"madswan/planner\"]\nagent = \"madswan-planner\"\nsession = \"madswan\"\n\
             [roles.\"madswan/ops\"]\nrepository = \"/r\"\nkind = \"ops\"\n",
        )
        .unwrap();
        assert_eq!(
            config.standing,
            BTreeMap::from([(
                "madswan/planner".to_string(),
                StandingAgent {
                    agent: "madswan-planner".into(),
                    session: Some("madswan".into()),
                    stream: None,
                }
            )]),
            "the planner's agent and session, and nothing standing for ops"
        );
        assert_eq!(
            config.roles.keys().collect::<Vec<_>>(),
            vec!["madswan/ops"],
            "ops's run, and no run for the planner"
        );
        assert_eq!(config.roles["madswan/ops"].kind, "ops");
    }

    #[test]
    fn a_standing_agent_in_its_stream() {
        let config = HostConfig::parse(
            "[roles.\"madswan/planner\"]\nagent = \"madswan-planner\"\nstream = \"madswan\"\n",
        )
        .unwrap();
        assert_eq!(
            config.standing["madswan/planner"],
            StandingAgent {
                agent: "madswan-planner".into(),
                session: None,
                stream: Some("madswan".into()),
            },
            "no session, the stream's own"
        );
        assert!(config.roles.is_empty());
    }

    #[test]
    fn the_real_host_gives_the_standing_agents() {
        use flume_seams::Host;
        let config = HostConfig::parse(
            "[roles.\"madswan/planner\"]\nagent = \"madswan-planner\"\n\
             [roles.\"madswan/ops\"]\nrepository = \"/r\"\nkind = \"ops\"\n",
        )
        .unwrap();
        let scratch = Scratch::new("standing");
        let host = super::super::RealHost::new(scratch.0.join("state"), config, None);
        assert_eq!(
            host.standing().keys().collect::<Vec<_>>(),
            vec!["madswan/planner"]
        );
        assert_eq!(host.role("madswan/planner"), None);
        assert!(host.role("madswan/ops").is_some());
    }

    /// role-waking, "A standing agent and a kind" (3, 132).
    #[test]
    fn a_standing_agent_and_a_kind_are_refused() {
        for text in [
            "[roles.\"madswan/planner\"]\nagent = \"madswan-planner\"\nkind = \"planner\"\n",
            "[roles.\"madswan/planner\"]\nagent = \"madswan-planner\"\nrepository = \"/r\"\n\
             kind = \"planner\"\npriority = 5\n",
        ] {
            let refused = HostConfig::parse(text).unwrap_err();
            assert!(
                refused.contains("roles.\"madswan/planner\"")
                    && refused.contains("both a standing agent and a kind"),
                "{refused}"
            );
        }
    }

    /// role-waking, "A run's key beside a standing agent" (C.7).
    #[test]
    fn a_runs_key_beside_a_standing_agent_is_refused() {
        for (key, value) in [
            ("repository", "\"/r\""),
            ("priority", "5"),
            ("base", "\"main\""),
            ("brief", "\"Plan.\""),
            ("pane", "\"w-1\""),
        ] {
            let refused = HostConfig::parse(&format!(
                "[roles.\"madswan/planner\"]\nagent = \"madswan-planner\"\n{key} = {value}\n"
            ))
            .unwrap_err();
            assert!(
                refused.contains("roles.\"madswan/planner\"")
                    && refused.contains(&format!("{key} is no word of a standing agent's entry")),
                "{refused}"
            );
        }
    }

    /// role-waking, "A session with no agent" (C.7).
    #[test]
    fn a_session_with_no_agent_is_refused() {
        let refused = HostConfig::parse(
            "[roles.\"madswan/planner\"]\nrepository = \"/r\"\nkind = \"planner\"\n\
             session = \"madswan\"\n",
        )
        .unwrap_err();
        assert!(
            refused.contains("roles.\"madswan/planner\"")
                && refused.contains("session is no word of a role's run"),
            "{refused}"
        );
    }

    #[test]
    fn an_agent_or_a_session_empty_or_no_string_is_refused() {
        for (text, named) in [
            ("agent = \"\"\n", "agent is empty"),
            ("agent = 4\n", "agent is not a string"),
            ("agent = \"w\"\nsession = \"\"\n", "session is empty"),
            (
                "agent = \"w\"\nsession = [\"w\"]\n",
                "session is not a string",
            ),
            ("agent = \"w\"\nstream = 2\n", "stream is not a string"),
        ] {
            let refused =
                HostConfig::parse(&format!("[roles.\"madswan/planner\"]\n{text}")).unwrap_err();
            assert!(
                refused.contains("roles.\"madswan/planner\"") && refused.contains(named),
                "{refused}"
            );
        }
    }

    #[test]
    fn a_standing_agent_from_the_local_layer_is_not_taken() {
        // C.7, core.45: `[roles]` is the managed layer's alone.
        let config = HostConfig::layers(
            "",
            Some("[roles.\"madswan/planner\"]\nagent = \"madswan-planner\"\n"),
        )
        .unwrap();
        assert!(config.standing.is_empty());
        let LocalLayer::NotTaken { keys, .. } = config.configuration.local else {
            panic!("the local layer taken");
        };
        assert_eq!(keys, vec!["roles.\"madswan/planner\""]);
    }

    /// C.7's example, as the statement writes it at fa40c94, its program
    /// `agent` (core.1), and `personal`'s `points` and `pace` left out,
    /// which record 0025's change reads.
    const C7: &str = r#"
[host]
agents = 8

[streams.work]
agents = 5
priority = 10
pool = "work"

[streams.work.labels.operation]
reserve = 2
keep_free = 1

[streams.work.labels.construction]
reserve = 2

[streams.work.kinds.research]
session = "work-design"

[streams.personal]
agents = 3
pool = "personal"

[pools.work]
strategy = "fill-first"
accounts = [
  { name = "work-1", program = "agent", env = { AGENT_HOME = "~/.agent-work-1" }, agents = 4, usage = "~/.local/state/usage/work-1.json" },
  { name = "work-2", program = "agent", env = { AGENT_HOME = "~/.agent-work-2" }, usage = "~/.local/state/usage/work-2.json" },
]

[pools.personal]
strategy = "least-busy"
accounts = [{ name = "personal", program = "agent", env = { AGENT_HOME = "~/.agent-personal" } }]

[repos."~/Code/acme"]
stream = "work"

[kinds.research]
label = "inception"
"#;

    #[test]
    fn c7s_example_read_whole() {
        let config = HostConfig::parse(C7).unwrap();
        assert_eq!(config.agents, Some(8));
        assert_eq!(
            config.streams.keys().collect::<Vec<_>>(),
            vec!["default", "personal", "work"]
        );
        let acme = &format!("{}/Code/acme", std::env::var("HOME").expect("the home"));
        let stream = config.stream_for(acme, "fix").unwrap();
        assert_eq!(
            stream,
            StreamHeld {
                name: "work".into(),
                from: StreamFrom::HostRepository,
            }
        );
        let pool = config.pool_for(&stream.name).unwrap();
        assert_eq!(
            pool,
            PoolNamed {
                name: "work".into(),
                from: PoolFrom::Stream,
            }
        );
        let work = &config.pools["work"];
        // The store-profile spec's C.7's example with readings: work
        // fill-first, stating no threshold, so 90 holds (185).
        assert_eq!(work.strategy, Strategy::FillFirst);
        assert_eq!(work.threshold, None);
        assert_eq!(
            work.accounts,
            vec![
                Account {
                    name: "work-1".into(),
                    program: "agent".into(),
                    env: BTreeMap::from([("AGENT_HOME".into(), "~/.agent-work-1".into())]),
                    agents: Some(4),
                    usage: Some("~/.local/state/usage/work-1.json".into()),
                },
                Account {
                    name: "work-2".into(),
                    program: "agent".into(),
                    env: BTreeMap::from([("AGENT_HOME".into(), "~/.agent-work-2".into())]),
                    agents: None,
                    usage: Some("~/.local/state/usage/work-2.json".into()),
                },
            ],
            "in order, each env and usage kept as written"
        );
        assert_eq!(config.pools["personal"].accounts[0].usage, None);
        assert_eq!(config.pools["personal"].strategy, Strategy::LeastBusy);
        assert_eq!(config.stream_for("/elsewhere", "fix"), None);
        assert_eq!(config.pool_for("default"), None, "no pool for default");
        // 180, 181: two of work's five owed to operation, one of them kept
        // free for it, and two to construction; research's places
        // inception's.
        assert_eq!(
            config.streams["work"].labels,
            BTreeMap::from([
                (
                    "construction".to_string(),
                    Share {
                        reserve: 2,
                        keep_free: 0,
                        unknown: vec![],
                    }
                ),
                (
                    "operation".to_string(),
                    Share {
                        reserve: 2,
                        keep_free: 1,
                        unknown: vec![],
                    }
                ),
            ])
        );
        assert_eq!(config.streams["personal"].labels, BTreeMap::new());
        assert_eq!(
            config.label_for(acme, "research"),
            Some(LabelHeld {
                name: "inception".into(),
                from: LabelFrom::HostKind,
            })
        );
        assert_eq!(config.label_for(acme, "fix"), None);
        assert_eq!(
            config.streams["work"].session, None,
            "work's agents in the session of its name"
        );
        // 199: a research place's agents opening in `work-design`, every
        // other agent of work's in `work`.
        assert_eq!(
            config.streams["work"].kinds,
            BTreeMap::from([(
                "research".to_string(),
                KindSessions {
                    session: Some("work-design".into()),
                    ..KindSessions::default()
                }
            )])
        );
        assert_eq!(config.streams["personal"].kinds, BTreeMap::new());
        let with_points = C7.replace(
            "pool = \"personal\"\n",
            "pool = \"personal\"\npoints = 8\npace = { points = 13, per = \"day\" }\n",
        );
        let refused = HostConfig::parse(&with_points).unwrap_err();
        assert!(
            refused.contains("streams.personal")
                && (refused.contains("points") || refused.contains("pace")),
            "{refused}"
        );
    }

    #[test]
    fn a_label_by_repository_and_kind_then_kind() {
        // 180, S85: the repository's kind's before the kind's.
        let config = HostConfig::parse(
            "[kinds.research]\nlabel = \"inception\"\n\
             [kinds.deploy]\nlabel = \"operation\"\n\
             [repos.\"/r\".kinds.deploy]\nlabel = \"construction\"\nstream = \"work\"\n",
        )
        .unwrap();
        assert_eq!(
            config.label_for("/r", "deploy"),
            Some(LabelHeld {
                name: "construction".into(),
                from: LabelFrom::HostRepositoryKind,
            })
        );
        assert_eq!(
            config.label_for("/s", "deploy"),
            Some(LabelHeld {
                name: "operation".into(),
                from: LabelFrom::HostKind,
            })
        );
        assert_eq!(
            config.label_for("/r", "research").map(|held| held.from),
            Some(LabelFrom::HostKind)
        );
        assert_eq!(config.label_for("/r", "fix"), None);
        assert_eq!(
            config.stream_for("/r", "deploy").map(|held| held.name),
            Some("work".into()),
            "a kind's table holds its label beside its stream"
        );
    }

    #[test]
    fn a_label_that_is_no_word_is_refused_naming_its_key() {
        // 180: one word of letters, digits, `-` and `_`.
        for (text, named) in [
            ("[kinds.fix]\nlabel = \"two words\"\n", "kinds.fix.label"),
            ("[kinds.fix]\nlabel = \"\"\n", "kinds.fix.label"),
            ("[kinds.fix]\nlabel = \"a.b\"\n", "kinds.fix.label"),
            ("[kinds.fix]\nlabel = 3\n", "kinds.fix.label"),
            (
                "[repos.\"/r\".kinds.fix]\nlabel = \"two words\"\n",
                "repos.\"/r\".kinds.fix.label",
            ),
        ] {
            let refused = HostConfig::parse(text).unwrap_err();
            assert!(refused.contains(named), "{text}: {refused}");
        }
        for word in ["operation", "night-ops", "ops_2", "A1"] {
            let text = format!("[kinds.fix]\nlabel = \"{word}\"\n");
            assert_eq!(
                HostConfig::parse(&text)
                    .unwrap()
                    .label_for("/r", "fix")
                    .map(|held| held.name),
                Some(word.to_string())
            );
        }
    }

    #[test]
    fn a_labels_table_holding_a_pool_is_read_with_the_pool_unknown() {
        // 181, 182: the configuration read, the key kept for the start
        // that reads it to refuse.
        let config = HostConfig::parse(
            "[streams.work]\nagents = 5\n\
             [streams.work.labels.operation]\nreserve = 1\npool = \"ops\"\npoints = 3\n",
        )
        .unwrap();
        assert_eq!(
            config.streams["work"].labels["operation"],
            Share {
                reserve: 1,
                keep_free: 0,
                unknown: vec!["points".into(), "pool".into()],
            }
        );
    }

    #[test]
    fn a_labels_share_that_is_no_count_is_refused() {
        // 181: whole numbers from zero.
        for (text, named) in [
            ("[streams.work.labels.operation]\nreserve = -1\n", "reserve"),
            (
                "[streams.work.labels.operation]\nkeep_free = \"one\"\n",
                "keep_free",
            ),
            (
                "[streams.work.labels]\noperation = 2\n",
                "streams.work.labels.operation",
            ),
            ("[streams.work]\nlabels = 2\n", "streams.work.labels"),
        ] {
            let refused = HostConfig::parse(text).unwrap_err();
            assert!(refused.contains(named), "{text}: {refused}");
        }
        let config = HostConfig::parse("[streams.work.labels.operation]\nreserve = 0\n").unwrap();
        assert_eq!(config.streams["work"].labels["operation"], Share::default());
    }

    #[test]
    fn a_session_in_a_streams_table_and_none_in_an_agents_table() {
        // 32, 107, 158: a stream's session read; a kind's agents table,
        // every stage's or one stage's, gives none as a setting.
        let config = HostConfig::parse(
            "[streams.work]\nsession = \"office\"\n\
             [kinds.fix.agents]\nsession = \"y\"\nmodel = \"opus\"\n\
             [kinds.fix.agents.stages.code]\nsession = \"z\"\n\
             [repos.\"/r\".kinds.fix.agents]\nsession = \"y\"\n",
        )
        .unwrap();
        assert_eq!(config.streams["work"].session.as_deref(), Some("office"));
        let fix = &config.kinds["fix"];
        assert_eq!(fix.every_stage.session, None);
        assert_eq!(fix.every_stage.model.as_deref(), Some("opus"));
        assert_eq!(fix.stages["code"], Settings::default());
        assert_eq!(config.repos["/r"]["fix"].every_stage, Settings::default());
        let refused = HostConfig::parse("[streams.work]\nsession = 3\n").unwrap_err();
        assert!(refused.contains("session"), "{refused}");
    }

    #[test]
    fn a_kinds_session_and_a_stages_read_with_each_other_key_unknown() {
        // 182, 199: the sessions read, each other key kept for the start
        // that reads its table to refuse.
        let config = HostConfig::parse(
            "[streams.work.kinds.research]\nsession = \"work-design\"\nlabel = \"inception\"\n\
             [streams.work.kinds.release.stages.review]\nsession = \"work-review\"\npool = \"work\"\n",
        )
        .unwrap();
        assert_eq!(
            config.streams["work"].kinds,
            BTreeMap::from([
                (
                    "release".to_string(),
                    KindSessions {
                        session: None,
                        stages: BTreeMap::from([(
                            "review".to_string(),
                            StageSession {
                                session: Some("work-review".into()),
                                unknown: vec!["pool".into()],
                            }
                        )]),
                        unknown: vec![],
                    }
                ),
                (
                    "research".to_string(),
                    KindSessions {
                        session: Some("work-design".into()),
                        stages: BTreeMap::new(),
                        unknown: vec!["label".into()],
                    }
                ),
            ])
        );
        assert_eq!(config.streams["default"].kinds, BTreeMap::new());
    }

    #[test]
    fn a_session_that_is_no_word() {
        // C.7: a kind's or a stage's `session` no string, or its tables no
        // tables, refused naming the table and the key.
        let refused =
            HostConfig::parse("[streams.work.kinds.research]\nsession = 3\n").unwrap_err();
        assert_eq!(
            refused,
            "[streams.work.kinds.research] session is not a string"
        );
        for (text, table, key) in [
            (
                "[streams.work.kinds.release.stages.review]\nsession = true\n",
                "streams.work.kinds.release.stages.review",
                "session",
            ),
            (
                "[streams.work.kinds.release]\nstages = 3\n",
                "streams.work.kinds.release",
                "stages",
            ),
            (
                "[streams.work.kinds.release.stages]\nreview = \"work-review\"\n",
                "streams.work.kinds.release.stages",
                "review",
            ),
            (
                "[streams.work.kinds]\nresearch = \"work-design\"\n",
                "streams.work.kinds",
                "research",
            ),
            ("[streams.work]\nkinds = 3\n", "streams.work", "kinds"),
        ] {
            let refused = HostConfig::parse(text).unwrap_err();
            assert!(
                refused.contains(table) && refused.contains(key),
                "{text}: {refused}"
            );
        }
    }

    #[test]
    fn labels_sessions_and_a_kinds_label_set_locally() {
        // 107, 180, 181, C.7, S85: each a key the local layer may set, and
        // a label's pool one it may not.
        let config = HostConfig::layers(
            &format!("{WORK}[streams.work.labels.operation]\nreserve = 1\n[kinds.research]\nlabel = \"inception\"\n"),
            Some(
                "[streams.work]\nsession = \"office\"\n\
                 [streams.work.labels.operation]\nreserve = 3\n\
                 [kinds.research]\nlabel = \"operation\"\n",
            ),
        )
        .unwrap();
        assert_eq!(config.configuration.local, LocalLayer::Taken);
        let work = &config.streams["work"];
        assert_eq!(work.session.as_deref(), Some("office"));
        assert_eq!(work.labels["operation"].reserve, 3);
        assert_eq!(work.agents, Some(6));
        assert_eq!(
            config.label_for("/r", "research").map(|held| held.name),
            Some("operation".into())
        );
        assert_eq!(
            value_at(&config, "streams.work.labels.operation.reserve"),
            Some(("3".into(), ConfigLayer::Local))
        );
        assert_eq!(
            value_at(&config, "kinds.research.label"),
            Some(("\"operation\"".into(), ConfigLayer::Local))
        );
        let config = HostConfig::layers(
            WORK,
            Some("[streams.work.labels.operation]\nreserve = 1\npool = \"ops\"\n"),
        )
        .unwrap();
        assert_eq!(
            config.configuration.local,
            LocalLayer::NotTaken {
                keys: vec!["streams.work.labels.operation.pool".into()],
                fault: None,
            }
        );
        let config =
            HostConfig::layers(WORK, Some("[kinds.fix]\nlabel = \"two words\"\n")).unwrap();
        let LocalLayer::NotTaken { keys, fault } = &config.configuration.local else {
            panic!("a label that is no word taken");
        };
        assert!(keys.is_empty());
        assert!(
            fault
                .as_deref()
                .is_some_and(|fault| fault.contains("kinds.fix.label")),
            "{fault:?}"
        );
    }

    #[test]
    fn a_kinds_session_and_a_stages_set_locally() {
        // 199, C.7, S107: each a key the local layer may set, kept with
        // its layer.
        let config = HostConfig::layers(
            &format!("{WORK}[streams.work.kinds.research]\nsession = \"work-design\"\n"),
            Some(
                "[streams.work.kinds.research]\nsession = \"work-research\"\n\
                 [streams.work.kinds.release.stages.review]\nsession = \"work-review\"\n",
            ),
        )
        .unwrap();
        assert_eq!(config.configuration.local, LocalLayer::Taken);
        let work = &config.streams["work"];
        assert_eq!(
            work.kinds["research"].session.as_deref(),
            Some("work-research")
        );
        assert_eq!(
            work.kinds["release"].stages["review"].session.as_deref(),
            Some("work-review")
        );
        assert_eq!(work.agents, Some(6));
        assert_eq!(
            value_at(&config, "streams.work.kinds.research.session"),
            Some(("\"work-research\"".into(), ConfigLayer::Local))
        );
        assert_eq!(
            value_at(&config, "streams.work.kinds.release.stages.review.session"),
            Some(("\"work-review\"".into(), ConfigLayer::Local))
        );
    }

    #[test]
    fn a_kinds_table_key_the_local_layer_cannot_set() {
        // C.7: a kind's or a stage's table holding any key but its
        // session, set locally, is a key the local layer may not set.
        for (local, key) in [
            (
                "[streams.work.kinds.research]\nlabel = \"inception\"\n",
                "streams.work.kinds.research.label",
            ),
            (
                "[streams.work.kinds.release.stages.review]\npool = \"work\"\n",
                "streams.work.kinds.release.stages.review.pool",
            ),
        ] {
            let config = HostConfig::layers(WORK, Some(local)).unwrap();
            assert_eq!(
                config.configuration.local,
                LocalLayer::NotTaken {
                    keys: vec![key.into()],
                    fault: None,
                },
                "{local}"
            );
            assert_eq!(config.streams["work"].kinds, BTreeMap::new(), "{local}");
        }
    }

    #[test]
    fn two_streams_declared() {
        let config = HostConfig::parse(
            "[host]\nagents = 8\n\
             [streams.work]\nagents = 5\npriority = 10\npool = \"work\"\n\
             [streams.personal]\nagents = 3\n\
             [pools.work]\nstrategy = \"fill-first\"\naccounts = [{ name = \"w\", program = \"agent\" }]\n",
        )
        .unwrap();
        assert_eq!(
            config.streams,
            BTreeMap::from([
                ("default".to_string(), Stream::default()),
                (
                    "personal".to_string(),
                    Stream {
                        agents: Some(3),
                        priority: 0,
                        pool: None,
                        ..Stream::default()
                    }
                ),
                (
                    "work".to_string(),
                    Stream {
                        agents: Some(5),
                        priority: 10,
                        pool: Some("work".into()),
                        ..Stream::default()
                    }
                ),
            ]),
            "personal at priority zero, and default held by the host's eight alone"
        );
    }

    #[test]
    fn no_streams() {
        let config = HostConfig::parse("[host]\nagents = 2\n").unwrap();
        assert_eq!(
            config.streams,
            BTreeMap::from([("default".to_string(), Stream::default())])
        );
        assert_eq!(config.stream_for("/r", "fix"), None);
        assert_eq!(config.pool_for("default"), None);
    }

    #[test]
    fn the_default_stream_states_its_own() {
        let config = HostConfig::parse("[streams.default]\nagents = 2\npriority = -1\n").unwrap();
        assert_eq!(
            config.streams,
            BTreeMap::from([(
                "default".to_string(),
                Stream {
                    agents: Some(2),
                    priority: -1,
                    pool: None,
                    ..Stream::default()
                }
            )])
        );
    }

    #[test]
    fn a_stream_named_by_repository_and_kind_then_repository_then_kind() {
        let config = HostConfig::parse(
            "[streams.work]\n[streams.personal]\n[streams.nightly]\n\
             [kinds.chore]\nstream = \"personal\"\n\
             [repos.\"/r\"]\nstream = \"work\"\n\
             [repos.\"/r\".kinds.fix]\nstream = \"nightly\"\n\
             [repos.\"/r\".kinds.fix.agents]\nmodel = \"sonnet\"\n",
        )
        .unwrap();
        let named = |repository, kind| config.stream_for(repository, kind).unwrap();
        assert_eq!(
            named("/r", "fix"),
            StreamHeld {
                name: "nightly".into(),
                from: StreamFrom::HostRepositoryKind,
            }
        );
        assert_eq!(
            named("/r", "chore"),
            StreamHeld {
                name: "work".into(),
                from: StreamFrom::HostRepository,
            },
            "the repository's word before the kind's"
        );
        assert_eq!(
            named("/s", "chore"),
            StreamHeld {
                name: "personal".into(),
                from: StreamFrom::HostKind,
            }
        );
        assert_eq!(
            config.repos["/r"]["fix"].every_stage.model.as_deref(),
            Some("sonnet"),
            "a kind's table holds its stream beside its agents"
        );
    }

    #[test]
    fn the_streams_pool_before_the_hosts() {
        let config = HostConfig::parse(
            "[host]\npool = \"personal\"\n\
             [streams.work]\npool = \"work\"\n[streams.personal]\n\
             [pools.work]\nstrategy = \"fill-first\"\naccounts = [{ name = \"w\", program = \"agent\" }]\n\
             [pools.personal]\nstrategy = \"least-busy\"\naccounts = [{ name = \"p\", program = \"agent\" }]\n",
        )
        .unwrap();
        let pool = |stream| config.pool_for(stream).unwrap();
        assert_eq!(
            pool("work"),
            PoolNamed {
                name: "work".into(),
                from: PoolFrom::Stream,
            }
        );
        for stream in ["personal", "default"] {
            assert_eq!(
                pool(stream),
                PoolNamed {
                    name: "personal".into(),
                    from: PoolFrom::Host,
                }
            );
        }
    }

    #[test]
    fn a_role_in_its_stream() {
        let config = HostConfig::parse(
            "[roles.\"madswan/planner\"]\nrepository = \"/r\"\nkind = \"planner\"\nstream = \"work\"\n",
        )
        .unwrap();
        assert_eq!(
            config.roles["madswan/planner"].stream.as_deref(),
            Some("work")
        );
    }

    #[test]
    fn a_configuration_that_refuses_names_the_key() {
        let account = "accounts = [{ name = \"w\", program = \"agent\" }]";
        for (text, named) in [
            ("[streams.work]\npool = \"missing\"\n".to_string(), "missing"),
            ("[host]\npool = \"missing\"\n".to_string(), "missing"),
            (
                format!("[pools.work]\nstrategy = \"round-robin\"\n{account}\n"),
                "round-robin",
            ),
            (format!("[pools.work]\n{account}\n"), "strategy"),
            ("[pools.work]\nstrategy = \"fill-first\"\n".to_string(), "accounts"),
            (
                "[pools.work]\nstrategy = \"fill-first\"\naccounts = []\n".to_string(),
                "accounts",
            ),
            (
                "[pools.work]\nstrategy = \"fill-first\"\naccounts = [{ name = \"w\", program = \"agent\" }, { name = \"w\", program = \"agent\" }]\n".to_string(),
                "second account",
            ),
            (
                "[pools.work]\nstrategy = \"fill-first\"\naccounts = [{ program = \"agent\" }]\n".to_string(),
                "name",
            ),
            (
                "[pools.work]\nstrategy = \"fill-first\"\naccounts = [{ name = \"w\" }]\n".to_string(),
                "program",
            ),
            (
                "[pools.work]\nstrategy = \"fill-first\"\naccounts = [{ name = \"w\", program = \"agent\", agents = 0 }]\n".to_string(),
                "agents",
            ),
            (
                "[pools.work]\nstrategy = \"fill-first\"\naccounts = [{ name = \"w\", program = \"agent\", env = { A = 1 } }]\n".to_string(),
                "env.A",
            ),
            ("[streams.work]\nagents = -2\n".to_string(), "agents"),
            ("[streams.work]\nagents = \"five\"\n".to_string(), "agents"),
            ("[streams.work]\npriority = \"high\"\n".to_string(), "priority"),
            ("[streams.work]\nshare = 3\n".to_string(), "share"),
            ("[repos.\"/r\"]\nstream = 3\n".to_string(), "stream"),
            ("[kinds.fix]\nstream = 3\n".to_string(), "stream"),
        ] {
            let refused = HostConfig::parse(&text).unwrap_err();
            assert!(refused.contains(named), "{text}: {refused}");
        }
    }

    /// The store-profile spec's most-left pool with readings: its
    /// strategy, its threshold, and an account's `usage` kept as written,
    /// its `~` read only when the reading is asked (184, 185, C.7).
    #[test]
    fn a_most_left_pool_with_readings() {
        let config = HostConfig::parse(
            "[pools.work]\nstrategy = \"most-left\"\nthreshold = 85\n\
             accounts = [{ name = \"work-1\", program = \"agent\", usage = \"~/.local/state/usage/work-1.json\" }]\n",
        )
        .unwrap();
        let work = &config.pools["work"];
        assert_eq!(work.strategy, Strategy::MostLeft);
        assert_eq!(work.threshold, Some(85));
        assert_eq!(
            work.accounts[0].usage.as_deref(),
            Some("~/.local/state/usage/work-1.json")
        );
        for threshold in [1, 100] {
            let config = HostConfig::parse(&format!(
                "[pools.work]\nstrategy = \"least-busy\"\nthreshold = {threshold}\n\
                 accounts = [{{ name = \"w\", program = \"agent\" }}]\n"
            ))
            .unwrap();
            assert_eq!(config.pools["work"].threshold, Some(threshold));
        }
    }

    /// The store-profile spec's values that refuse: each names its key,
    /// and the strategy's words name all three (4, C.7).
    #[test]
    fn a_threshold_a_usage_and_a_strategy_that_refuse() {
        let account = "accounts = [{ name = \"w\", program = \"agent\" }]";
        for (text, words) in [
            (
                format!("[pools.work]\nstrategy = \"fill-first\"\nthreshold = 0\n{account}\n"),
                "[pools.work] threshold is not a whole percentage from 1 to 100",
            ),
            (
                format!("[pools.work]\nstrategy = \"fill-first\"\nthreshold = 101\n{account}\n"),
                "[pools.work] threshold is not a whole percentage from 1 to 100",
            ),
            (
                format!("[pools.work]\nstrategy = \"fill-first\"\nthreshold = \"90%\"\n{account}\n"),
                "[pools.work] threshold is not a whole percentage from 1 to 100",
            ),
            (
                format!("[pools.work]\nstrategy = \"fill-first\"\nthreshold = 90.5\n{account}\n"),
                "[pools.work] threshold is not a whole percentage from 1 to 100",
            ),
            (
                "[pools.work]\nstrategy = \"fill-first\"\naccounts = [{ name = \"w\", program = \"agent\", usage = 7 }]\n".to_string(),
                "[pools.work.accounts[0]] usage is not a string",
            ),
            (
                "[pools.work]\nstrategy = \"fill-first\"\naccounts = [{ name = \"w\", program = \"agent\", usage = \"\" }]\n".to_string(),
                "[pools.work.accounts[0]] usage is empty",
            ),
            (
                format!("[pools.work]\nstrategy = \"round-robin\"\n{account}\n"),
                "[pools.work] strategy is round-robin, where a pool's strategy is fill-first, least-busy or most-left",
            ),
        ] {
            assert_eq!(HostConfig::parse(&text).unwrap_err(), words, "{text}");
        }
    }

    /// A reading's path is the managed layer's alone and given as `<set>`:
    /// a local layer setting a pool's threshold or an account's usage is
    /// not taken, and the configuration's values give no path (184, C.7;
    /// core.30, core.32, core.45).
    #[test]
    fn a_threshold_and_a_usage_are_the_managed_layers() {
        let managed = "[pools.work]\nstrategy = \"most-left\"\nthreshold = 85\n\
             accounts = [{ name = \"w\", program = \"agent\", usage = \"~/.local/state/usage/w.json\" }]\n";
        let config = HostConfig::layers(managed, None).unwrap();
        let values: Vec<(&str, &str)> = config
            .configuration
            .values
            .iter()
            .map(|value| (value.key.as_str(), value.value.as_str()))
            .collect();
        for value in [
            ("pools.work.strategy", "\"most-left\""),
            ("pools.work.threshold", "85"),
            ("pools.work.accounts[0].usage", SET),
        ] {
            assert!(values.contains(&value), "{value:?} in {values:?}");
        }
        assert!(
            !values
                .iter()
                .any(|(_, value)| value.contains("usage/w.json")),
            "a reading's path in {values:?}"
        );
        for local in [
            "[pools.work]\nthreshold = 50\n",
            "[pools.work]\nstrategy = \"most-left\"\nthreshold = 50\n\
             accounts = [{ name = \"w\", program = \"agent\", usage = \"/elsewhere.json\" }]\n",
        ] {
            let config = HostConfig::layers(managed, Some(local)).unwrap();
            assert_eq!(
                config.configuration.local,
                LocalLayer::NotTaken {
                    keys: vec!["pools.work".into()],
                    fault: None,
                },
                "{local}"
            );
            assert_eq!(config.pools["work"].threshold, Some(85));
            assert_eq!(
                config.pools["work"].accounts[0].usage.as_deref(),
                Some("~/.local/state/usage/w.json")
            );
        }
    }

    #[test]
    fn a_bound_that_is_no_count_is_refused() {
        assert!(HostConfig::parse("[host]\nagents = -1\n").is_err());
        assert!(HostConfig::parse("[host]\nagents = \"two\"\n").is_err());
    }

    /// A managed layer giving `work` six agents and the pool `work`.
    const WORK: &str = "[streams.work]\nagents = 6\npool = \"work\"\n\
         [pools.work]\nstrategy = \"fill-first\"\n\
         accounts = [{ name = \"w\", program = \"agent\", env = { AGENT_HOME = \"~/.agent-w\" } }]\n";

    /// The configuration's value at `key`, with its layer.
    fn value_at(config: &HostConfig, key: &str) -> Option<(String, ConfigLayer)> {
        config
            .configuration
            .values
            .iter()
            .find(|value| value.key == key)
            .map(|value| (value.value.clone(), value.layer))
    }

    #[test]
    fn a_streams_bound_and_priority_set_locally() {
        // C.7, S83: the local layer's three agents and priority 30 over
        // the managed layer's six, the pool the managed layer's.
        let config =
            HostConfig::layers(WORK, Some("[streams.work]\nagents = 3\npriority = 30\n")).unwrap();
        assert_eq!(config.configuration.local, LocalLayer::Taken);
        let work = &config.streams["work"];
        assert_eq!(
            (work.agents, work.priority, work.pool.as_deref()),
            (Some(3), 30, Some("work"))
        );
        assert_eq!(
            value_at(&config, "streams.work.agents"),
            Some(("3".into(), ConfigLayer::Local))
        );
        assert_eq!(
            value_at(&config, "streams.work.priority"),
            Some(("30".into(), ConfigLayer::Local))
        );
        assert_eq!(
            value_at(&config, "streams.work.pool"),
            Some(("\"work\"".into(), ConfigLayer::Managed))
        );
        assert_eq!(config.configuration.passed_over, vec![]);
    }

    #[test]
    fn a_kinds_settings_key_by_key() {
        // 32, C.7: the local model, the managed instructions.
        let config = HostConfig::layers(
            "[kinds.change.agents]\nmodel = \"opus\"\ninstructions = \"be brief\"\n",
            Some("[kinds.change.agents]\nmodel = \"sonnet\"\n"),
        )
        .unwrap();
        let change = &config.kinds["change"].every_stage;
        assert_eq!(change.model.as_deref(), Some("sonnet"));
        assert_eq!(change.instructions.as_deref(), Some("be brief"));
        assert_eq!(
            value_at(&config, "kinds.change.agents.model"),
            Some(("\"sonnet\"".into(), ConfigLayer::Local))
        );
        assert_eq!(
            value_at(&config, "kinds.change.agents.instructions"),
            Some(("\"be brief\"".into(), ConfigLayer::Managed))
        );
    }

    #[test]
    fn one_stages_settings_and_a_mode_each_taken_whole() {
        // 32, C.7; core.30: a stage's model under `stages`, and a mode's
        // table of programs one unit, replacing the managed one whole.
        let config = HostConfig::layers(
            "[kinds.fix.agents]\nmode = { agent = \"careful\", other = \"wide\" }\n\
             [kinds.fix.agents.stages.code]\nmodel = \"opus\"\ninstructions = \"test first\"\n",
            Some(
                "[kinds.fix.agents]\nmode = { agent = \"edits\" }\n\
                 [kinds.fix.agents.stages.code]\nmodel = \"sonnet\"\n",
            ),
        )
        .unwrap();
        let fix = &config.kinds["fix"];
        assert_eq!(
            fix.every_stage.mode,
            Some(ModeSetting::Programs(BTreeMap::from([(
                "agent".to_string(),
                "edits".to_string()
            )]))),
            "the managed table's other program goes with it"
        );
        assert_eq!(fix.stages["code"].model.as_deref(), Some("sonnet"));
        assert_eq!(
            fix.stages["code"].instructions.as_deref(),
            Some("test first")
        );
        assert_eq!(
            value_at(&config, "kinds.fix.agents.mode"),
            Some(("{ agent = \"edits\" }".into(), ConfigLayer::Local)),
            "a unit given whole"
        );
        assert_eq!(
            value_at(&config, "kinds.fix.agents.stages.code.instructions"),
            Some(("\"test first\"".into(), ConfigLayer::Managed))
        );
    }

    #[test]
    fn no_local_file() {
        // C.7: the managed layer alone, every value its own.
        let config = HostConfig::layers(WORK, None).unwrap();
        assert_eq!(config.configuration.local, LocalLayer::NoFile);
        assert_eq!(config.streams["work"].agents, Some(6));
        assert!(!config.configuration.values.is_empty());
        assert!(config
            .configuration
            .values
            .iter()
            .all(|value| value.layer == ConfigLayer::Managed));
    }

    #[test]
    fn every_value_as_toml_writes_it_and_no_variables() {
        // 56, C.7; core.30: an array of tables by index, an account's
        // variables `<set>`, a pin's list as written.
        let config = HostConfig::layers(
            &format!("[host]\nagents = 8\npin = [\"streams.work.agents\"]\n{WORK}"),
            None,
        )
        .unwrap();
        let values: Vec<(&str, &str)> = config
            .configuration
            .values
            .iter()
            .map(|value| (value.key.as_str(), value.value.as_str()))
            .collect();
        for value in [
            ("host.agents", "8"),
            ("host.pin", "[\"streams.work.agents\"]"),
            ("streams.work.agents", "6"),
            ("pools.work.strategy", "\"fill-first\""),
            ("pools.work.accounts[0].name", "\"w\""),
            ("pools.work.accounts[0].program", "\"agent\""),
            ("pools.work.accounts[0].env.AGENT_HOME", SET),
        ] {
            assert!(values.contains(&value), "{value:?} in {values:?}");
        }
        assert!(
            !values.iter().any(|(_, value)| value.contains(".agent-w")),
            "a variable's value in {values:?}"
        );
    }

    #[test]
    fn an_own_setting_shaped_as_a_credential_is_given_as_set() {
        // 195, core.50: an own setting's value is given as written where
        // it passes design 7's check, and as `<set>` where it does not,
        // at any depth and under any agents table.
        let config = HostConfig::layers(
            "[agents.programs.agent.settings]
             line = \"~/bin/line $HOME\"
             status = { command = \"run --key sk-a1B2c3D4e5F6g7H8i9J0\" }
             [kinds.fix.agents.programs.agent.settings]
             headers = { Authorization = \"Bearer abc\" }
",
            None,
        )
        .unwrap();
        assert_eq!(
            value_at(&config, "agents.programs.agent.settings.line"),
            Some(("\"~/bin/line $HOME\"".into(), ConfigLayer::Managed))
        );
        for key in [
            "agents.programs.agent.settings.status",
            "kinds.fix.agents.programs.agent.settings.headers",
        ] {
            assert_eq!(
                value_at(&config, key).map(|(value, _)| value),
                Some(SET.to_string()),
                "{key}"
            );
        }
        assert!(
            !config
                .configuration
                .values
                .iter()
                .any(|value| value.value.contains("a1B2") || value.value.contains("abc")),
            "a credential among {:?}",
            config.configuration.values
        );
    }

    #[test]
    fn a_local_file_that_sets_a_pool() {
        // C.7, S83: no key of it taken, its pool named.
        let config = HostConfig::layers(
            WORK,
            Some(
                "[streams.work]\nagents = 3\n\
                 [pools.work]\nstrategy = \"least-busy\"\n\
                 accounts = [{ name = \"local\", program = \"agent\" }]\n",
            ),
        )
        .unwrap();
        assert_eq!(
            config.configuration.local,
            LocalLayer::NotTaken {
                keys: vec!["pools.work".into()],
                fault: None,
            }
        );
        assert_eq!(config.streams["work"].agents, Some(6));
        assert_eq!(config.pools["work"].strategy, Strategy::FillFirst);
        assert!(config
            .configuration
            .values
            .iter()
            .all(|value| value.layer == ConfigLayer::Managed));
    }

    #[test]
    fn a_streams_pool_set_locally() {
        // C.7: a stream's pool is the managed layer's alone.
        let config =
            HostConfig::layers(WORK, Some("[streams.work]\npool = \"personal\"\n")).unwrap();
        assert_eq!(
            config.configuration.local,
            LocalLayer::NotTaken {
                keys: vec!["streams.work.pool".into()],
                fault: None,
            }
        );
        assert_eq!(config.streams["work"].pool.as_deref(), Some("work"));
    }

    #[test]
    fn the_keys_and_none() {
        // 143, 186, 190: a stream's multiplexer, view and terminal app its
        // own, else the host's, else the default; its title its own, the
        // core naming it by its name where none, which status gives
        // through the binary in commands.rs's the_keys_and_none_in_status.
        let config = HostConfig::parse(
            "[host]\nmultiplexer = \"mux\"\nterminal_app = \"app-window\"\n\
             [streams.office]\nterminal_app = \"terminal\"\ntitle = \"Office Agents\"\n\
             [streams.personal]\n",
        )
        .unwrap();
        let held = |profile: &str, from| ProfileHeld {
            profile: profile.into(),
            from,
        };
        let neither = ViewHeld {
            view: View::Terminal,
            from: NamedBy::Neither,
        };
        assert_eq!(
            config.display_for("office"),
            StreamDisplay {
                multiplexer: held("mux", NamedBy::Host),
                view: neither.clone(),
                terminal_app: held("terminal", NamedBy::Stream),
            }
        );
        assert_eq!(
            config.streams["office"].title.as_deref(),
            Some("Office Agents")
        );
        assert_eq!(
            config.display_for("personal"),
            StreamDisplay {
                multiplexer: held("mux", NamedBy::Host),
                view: neither,
                terminal_app: held("app-window", NamedBy::Host),
            }
        );
        assert_eq!(config.streams["personal"].title, None);
        let config = HostConfig::parse(
            "[host]\nview = \"app\"\n[streams.work]\nmultiplexer = \"mux\"\nview = \"terminal\"\n",
        )
        .unwrap();
        let work = config.display_for("work");
        assert_eq!(work.multiplexer, held("mux", NamedBy::Stream));
        assert_eq!(
            work.view,
            ViewHeld {
                view: View::Terminal,
                from: NamedBy::Stream,
            }
        );
        assert_eq!(
            config.display_for("default"),
            StreamDisplay {
                multiplexer: held(PRINTED, NamedBy::Neither),
                view: ViewHeld {
                    view: View::App,
                    from: NamedBy::Host,
                },
                terminal_app: held(PRINTED, NamedBy::Neither),
            }
        );
    }

    #[test]
    fn the_word_written_before() {
        // C.7: `[host] terminal` read as `[host] multiplexer`, and both
        // refused naming both.
        let config = HostConfig::parse("[host]\nterminal = \"mux\"\n").unwrap();
        assert_eq!(config.display_for("default").multiplexer.profile, "mux");
        assert_eq!(
            config.display_for("default").multiplexer.from,
            NamedBy::Host
        );
        let config = HostConfig::parse("[host]\nmultiplexer = \"mux\"\n").unwrap();
        assert_eq!(config.terminal.as_deref(), Some("mux"));
        let refused = HostConfig::parse("[host]\nterminal = \"mux\"\nmultiplexer = \"printed\"\n")
            .unwrap_err();
        assert!(
            refused.contains("terminal") && refused.contains("multiplexer"),
            "{refused}"
        );
        let refused = HostConfig::parse("[host]\nmultiplexer = 3\n").unwrap_err();
        assert!(refused.contains("[host] multiplexer"), "{refused}");
    }

    #[test]
    fn a_view_no_word_names() {
        // C.7: a view is terminal or app, the key and both values named.
        let refused = HostConfig::parse("[streams.work]\nview = \"tab\"\n").unwrap_err();
        assert!(
            refused.contains("[streams.work] view")
                && refused.contains("\"tab\"")
                && refused.contains("terminal or app"),
            "{refused}"
        );
        let refused = HostConfig::parse("[host]\nview = 1\n").unwrap_err();
        assert!(refused.contains("[host] view"), "{refused}");
    }

    #[test]
    fn a_title_that_is_no_text() {
        // C.7: a stream's title is a text, and its terminal app and
        // multiplexer names.
        for text in [
            "[streams.work]\ntitle = 3\n",
            "[streams.work]\nterminal_app = 3\n",
            "[streams.work]\nmultiplexer = true\n",
            "[host]\nterminal_app = []\n",
        ] {
            let key = text.lines().nth(1).unwrap().split(' ').next().unwrap();
            let refused = HostConfig::parse(text).unwrap_err();
            assert!(refused.contains(key), "{text}: {refused}");
        }
        let refused = HostConfig::parse("[streams.work]\ntitle = 3\n").unwrap_err();
        assert!(refused.contains("[streams.work] title"), "{refused}");
    }

    #[test]
    fn a_streams_multiplexer_set_locally() {
        // 190, C.7, S91: a stream's multiplexer a key the local layer may
        // set, kept with its layer.
        let config = HostConfig::layers(
            "[host]\nmultiplexer = \"mux\"\n[streams.trial]\nmultiplexer = \"printed\"\n",
            Some("[streams.trial]\nmultiplexer = \"mux\"\n"),
        )
        .unwrap();
        assert_eq!(config.configuration.local, LocalLayer::Taken);
        assert_eq!(
            config.display_for("trial").multiplexer,
            ProfileHeld {
                profile: "mux".into(),
                from: NamedBy::Stream,
            }
        );
        assert_eq!(
            value_at(&config, "streams.trial.multiplexer"),
            Some(("\"mux\"".into(), ConfigLayer::Local))
        );
        assert_eq!(
            value_at(&config, "host.multiplexer"),
            Some(("\"mux\"".into(), ConfigLayer::Managed))
        );
    }

    #[test]
    fn a_streams_view_set_locally() {
        // 186, C.7: a stream's view the managed layer's alone.
        let config = HostConfig::layers(WORK, Some("[streams.work]\nview = \"app\"\n")).unwrap();
        assert_eq!(
            config.configuration.local,
            LocalLayer::NotTaken {
                keys: vec!["streams.work.view".into()],
                fault: None,
            }
        );
        assert_eq!(config.display_for("work").view.view, View::Terminal);
    }

    #[test]
    fn a_terminal_app_set_by_the_local_layer() {
        // C.7; record 0032: a stream's terminal app and title, and every
        // key of `[host]`, the managed layer's alone; open uses the
        // managed layer's terminal app.
        let managed = format!("{WORK}[host]\nterminal_app = \"app-window\"\n");
        let config = HostConfig::layers(
            &managed,
            Some("[streams.work]\nterminal_app = \"terminal\"\n"),
        )
        .unwrap();
        assert_eq!(
            config.configuration.local,
            LocalLayer::NotTaken {
                keys: vec!["streams.work.terminal_app".into()],
                fault: None,
            }
        );
        assert_eq!(
            config.display_for("work").terminal_app,
            ProfileHeld {
                profile: "app-window".into(),
                from: NamedBy::Host,
            }
        );
        for (local, key) in [
            ("[streams.work]\ntitle = \"Mine\"\n", "streams.work.title"),
            ("[host]\nmultiplexer = \"mux\"\n", "host.multiplexer"),
            ("[host]\nview = \"app\"\n", "host.view"),
            ("[host]\nterminal_app = \"terminal\"\n", "host.terminal_app"),
        ] {
            let config = HostConfig::layers(&managed, Some(local)).unwrap();
            assert_eq!(
                config.configuration.local,
                LocalLayer::NotTaken {
                    keys: vec![key.into()],
                    fault: None,
                },
                "{local}"
            );
        }
    }

    #[test]
    fn each_key_the_local_layer_may_not_set_named_once() {
        // C.7: by the shortest part of its path that leads to no key the
        // local layer may set, and at least two segments.
        let config = HostConfig::layers(
            "",
            Some(
                "[pools.work]\nstrategy = \"fill-first\"\naccounts = [{ name = \"w\", program = \"agent\" }]\n\
                 [host]\nagents = 2\n\
                 [roles.\"a/b\"]\nrepository = \"/r\"\nkind = \"planner\"\n\
                 [streams.work]\nagents = 3\npool = \"x\"\n",
            ),
        )
        .unwrap();
        let LocalLayer::NotTaken { mut keys, fault } = config.configuration.local else {
            panic!("the local layer taken");
        };
        keys.sort();
        assert_eq!(
            keys,
            vec![
                "host.agents",
                "pools.work",
                "roles.\"a/b\"",
                "streams.work.pool"
            ]
        );
        assert_eq!(fault, None);
        assert_eq!(config.streams["default"], Stream::default());
        assert!(!config.streams.contains_key("work"), "no key of it taken");
    }

    #[test]
    fn a_local_file_at_fault() {
        // C.7: not TOML, or a value its key refuses, and the managed
        // layer read alone.
        for (local, named) in [
            ("[streams.work\nagents = 3\n", "line 1"),
            ("[streams.work]\nagents = \"three\"\n", "agents"),
            ("streams = 3\n", "[streams] is not a table"),
        ] {
            let config = HostConfig::layers(WORK, Some(local)).unwrap();
            let LocalLayer::NotTaken { keys, fault } = &config.configuration.local else {
                panic!("{local}: {:?}", config.configuration.local);
            };
            assert!(keys.is_empty(), "{local}: {keys:?}");
            let fault = fault.as_deref().unwrap_or_default();
            assert!(fault.contains(named), "{local}: {fault}");
            assert_eq!(config.streams["work"].agents, Some(6), "{local}");
            assert!(
                !fault.contains('\n'),
                "{local}: the fault on one line: {fault}"
            );
        }
    }

    #[test]
    fn a_stream_only_the_local_layer_names() {
        // 143, 144, C.7: declared by the local layer with no pool, so its
        // places take the host's.
        let config = HostConfig::layers(
            "[host]\npool = \"personal\"\n\
             [pools.personal]\nstrategy = \"least-busy\"\naccounts = [{ name = \"p\", program = \"agent\" }]\n",
            Some("[streams.nightly]\nagents = 1\n[kinds.chore]\nstream = \"nightly\"\n"),
        )
        .unwrap();
        assert_eq!(config.configuration.local, LocalLayer::Taken);
        assert_eq!(config.streams["nightly"].agents, Some(1));
        assert_eq!(
            config.stream_for("/r", "chore"),
            Some(StreamHeld {
                name: "nightly".into(),
                from: StreamFrom::HostKind,
            })
        );
        assert_eq!(
            config.pool_for("nightly"),
            Some(PoolNamed {
                name: "personal".into(),
                from: PoolFrom::Host,
            })
        );
        assert_eq!(
            value_at(&config, "streams.nightly.agents"),
            Some(("1".into(), ConfigLayer::Local))
        );
        assert_eq!(
            value_at(&config, "host.pool"),
            Some(("\"personal\"".into(), ConfigLayer::Managed))
        );
    }

    #[test]
    fn one_key_pinned() {
        // C.7, S83: the priority taken, the agents passed over and named
        // with their pin.
        let config = HostConfig::layers(
            &format!("[host]\npin = [\"streams.work.agents\"]\n{WORK}"),
            Some("[streams.work]\nagents = 3\npriority = 30\n"),
        )
        .unwrap();
        assert_eq!(config.configuration.local, LocalLayer::Taken);
        let work = &config.streams["work"];
        assert_eq!((work.agents, work.priority), (Some(6), 30));
        assert_eq!(
            config.configuration.passed_over,
            vec![PinnedKey {
                key: "streams.work.agents".into(),
                pin: "streams.work.agents".into(),
            }]
        );
        assert_eq!(
            value_at(&config, "streams.work.agents"),
            Some(("6".into(), ConfigLayer::Managed))
        );
        assert_eq!(
            value_at(&config, "streams.work.priority"),
            Some(("30".into(), ConfigLayer::Local))
        );
    }

    #[test]
    fn a_table_pinned() {
        // C.7: both of work's keys passed over, the fix's stream taken.
        let config = HostConfig::layers(
            &format!("[host]\npin = [\"streams.work\"]\n{WORK}"),
            Some("[streams.work]\nagents = 3\nsession = \"office\"\n[kinds.fix]\nstream = \"work\"\n"),
        )
        .unwrap();
        assert_eq!(config.configuration.local, LocalLayer::Taken);
        let mut passed: Vec<(String, String)> = config
            .configuration
            .passed_over
            .iter()
            .map(|over| (over.key.clone(), over.pin.clone()))
            .collect();
        passed.sort();
        assert_eq!(
            passed,
            vec![
                ("streams.work.agents".into(), "streams.work".into()),
                ("streams.work.session".into(), "streams.work".into()),
            ]
        );
        assert_eq!(config.streams["work"].agents, Some(6));
        assert_eq!(config.streams["work"].session, None);
        assert_eq!(
            config.stream_for("/r", "fix").map(|held| held.name),
            Some("work".into())
        );
    }

    #[test]
    fn a_kinds_session_pinned() {
        // 199, C.7: work's agents taken, research's session passed over
        // and named with its pin, by the table holding it or by the key.
        let managed = format!(
            "{WORK}[streams.work.kinds.research]\nsession = \"work-design\"\n\
             [streams.work.kinds.release.stages.review]\nsession = \"work-review\"\n"
        );
        let local = "[streams.work]\nagents = 3\n\
             [streams.work.kinds.research]\nsession = \"work-research\"\n\
             [streams.work.kinds.release.stages.review]\nsession = \"work-local\"\n";
        let config = HostConfig::layers(
            &format!("[host]\npin = [\"streams.work.kinds\"]\n{managed}"),
            Some(local),
        )
        .unwrap();
        assert_eq!(config.configuration.local, LocalLayer::Taken);
        let work = &config.streams["work"];
        assert_eq!(work.agents, Some(3));
        assert_eq!(
            work.kinds["research"].session.as_deref(),
            Some("work-design")
        );
        assert_eq!(
            work.kinds["release"].stages["review"].session.as_deref(),
            Some("work-review")
        );
        assert_eq!(
            config.configuration.passed_over,
            vec![
                PinnedKey {
                    key: "streams.work.kinds.release.stages.review.session".into(),
                    pin: "streams.work.kinds".into(),
                },
                PinnedKey {
                    key: "streams.work.kinds.research.session".into(),
                    pin: "streams.work.kinds".into(),
                },
            ]
        );
        assert_eq!(
            value_at(&config, "streams.work.kinds.research.session"),
            Some(("\"work-design\"".into(), ConfigLayer::Managed))
        );
        assert_eq!(
            value_at(&config, "streams.work.agents"),
            Some(("3".into(), ConfigLayer::Local))
        );
        let config = HostConfig::layers(
            &format!("[host]\npin = [\"streams.work.kinds.research.session\"]\n{managed}"),
            Some(local),
        )
        .unwrap();
        let work = &config.streams["work"];
        assert_eq!(
            work.kinds["research"].session.as_deref(),
            Some("work-design")
        );
        assert_eq!(
            work.kinds["release"].stages["review"].session.as_deref(),
            Some("work-local"),
            "the stage's session no pin holds"
        );
        let refused = HostConfig::layers(
            &format!("[host]\npin = [\"streams.work.kinds.research.label\"]\n{managed}"),
            None,
        )
        .unwrap_err();
        assert!(
            refused.contains("[host] pin") && refused.contains("streams.work.kinds.research.label"),
            "{refused}"
        );
    }

    #[test]
    fn a_pin_of_nothing_the_local_layer_sets() {
        // C.7: refused naming the pin, as any fault of the managed layer
        // is, whether or not there is a local file.
        for pin in [
            "\"pools.work\"",
            "\"host.agents\"",
            "\"streams.work.pool\"",
            "\"streams.work.agents.more\"",
            "\"no key\"",
            "3",
        ] {
            let managed = format!("[host]\npin = [{pin}]\n{WORK}");
            for local in [None, Some("[streams.work]\nagents = 3\n")] {
                let refused = HostConfig::layers(&managed, local).unwrap_err();
                assert!(
                    refused.contains("[host] pin") && refused.contains(pin.trim_matches('"')),
                    "{pin}: {refused}"
                );
            }
        }
        for pin in [
            "\"streams\"",
            "\"kinds.fix.agents\"",
            "'repos.\"/r\".stream'",
        ] {
            let managed = format!("[host]\npin = [{pin}]\n");
            assert!(HostConfig::layers(&managed, None).is_ok(), "{pin}");
        }
    }

    #[test]
    fn a_pin_of_a_repositorys_key_with_the_home_or_tilde() {
        // C.7: a repository's key in a pin, in the local layer and in the
        // managed one each read with its `~` the home.
        let home = std::env::var("HOME").expect("the home");
        let at_home = format!("{home}/Code/acme");
        for (pinned, local) in [
            ("~/Code/acme", at_home.as_str()),
            (at_home.as_str(), "~/Code/acme"),
        ] {
            let config = HostConfig::layers(
                &format!(
                    "[host]\npin = ['repos.\"{pinned}\".stream']\n\
                     [repos.\"~/Code/acme\"]\nstream = \"work\"\n"
                ),
                Some(&format!(
                    "[repos.\"{local}\"]\nstream = \"personal\"\n\
                     [repos.\"{local}\".kinds.fix]\nstream = \"nightly\"\n"
                )),
            )
            .unwrap();
            assert_eq!(
                config.configuration.passed_over,
                vec![PinnedKey {
                    key: format!("repos.\"{local}\".stream"),
                    pin: format!("repos.\"{pinned}\".stream"),
                }],
                "{pinned} pinning {local}"
            );
            assert_eq!(
                config.stream_for(&at_home, "chore").map(|held| held.name),
                Some("work".into())
            );
            assert_eq!(
                config.stream_for(&at_home, "fix"),
                Some(StreamHeld {
                    name: "nightly".into(),
                    from: StreamFrom::HostRepositoryKind,
                }),
                "the local kind's stream set into the managed repository's table"
            );
            assert_eq!(
                value_at(&config, "repos.\"~/Code/acme\".kinds.fix.stream"),
                Some(("\"nightly\"".into(), ConfigLayer::Local))
            );
        }
    }

    /// A scratch directory of the test's own, removed when dropped.
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(label: &str) -> Scratch {
            let dir =
                std::env::temp_dir().join(format!("flume-config-{label}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("the scratch directory");
            Scratch(dir)
        }

        /// Writes `text` at `name` in the directory, and gives its path.
        fn write(&self, name: &str, text: &str) -> PathBuf {
            let path = self.0.join(name);
            std::fs::write(&path, text).expect("the file");
            path
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// `path` as a key beginning `~/` reads it: up from the home to the
    /// root and down to `path`, so a scratch file is named under the home
    /// with nothing written there.
    fn under_home(path: &Path) -> String {
        let home = std::env::var("HOME").expect("$HOME is set");
        let up = Path::new(&home).components().count() - 1;
        format!(
            "~/{}{}",
            "../".repeat(up),
            path.to_string_lossy().trim_start_matches('/')
        )
    }

    #[test]
    fn settings_for_every_agent_and_for_a_stream() {
        // 32, 143, 195, C.7: `[agents]` and `[agents.streams.<stream>]`
        // give `model`, `mode`, `instructions` and `programs`, a file
        // under the home read on the host; the host's settings for a
        // place's repository, stream and every agent as the caller names
        // the stream.
        let scratch = Scratch::new("every-agent");
        let people = scratch.write(
            "agent-settings.json",
            r#"{ "tui": "fullscreen", "outputStyle": "plain" }"#,
        );
        let written = under_home(&people);
        let config = scratch.write(
            "config.toml",
            &format!(
                "[agents]\nmodel = \"sonnet\"\nmode = \"edits\"\ninstructions = \"Be brief.\"\n\
                 [agents.programs.agent]\nfile = {written:?}\n\
                 settings = {{ statusLine = {{ type = \"command\", command = \"line\" }} }}\n\
                 [streams.work]\n\
                 [agents.streams.work]\nmodel = \"opus\"\n\
                 [agents.streams.work.programs.agent.settings]\ntui = \"default\"\n\
                 [repos.\"/r\".agents]\ninstructions = \"Mind the tagline.\"\n"
            ),
        );
        let config = HostConfig::read(&config).unwrap();
        let every = &config.every_agent;
        assert_eq!(every.model.as_deref(), Some("sonnet"));
        assert_eq!(every.mode, Some(ModeSetting::Flume(Mode::Edits)));
        assert_eq!(every.instructions.as_deref(), Some("Be brief."));
        let agent = &every.programs["agent"];
        assert_eq!(
            agent.settings["statusLine"],
            serde_json::json!({ "type": "command", "command": "line" })
        );
        let file = agent.file.as_ref().expect("the file");
        assert_eq!(file.written, written);
        assert_eq!(file.read, home(&written), "the home read for ~/");
        assert_eq!(file.keys["tui"], serde_json::json!("fullscreen"));
        let work = &config.streams_agents["work"];
        assert_eq!(work.model.as_deref(), Some("opus"));
        assert_eq!(
            work.programs["agent"].settings["tui"],
            serde_json::json!("default")
        );
        let every = every.clone();
        let host = super::super::RealHost::new(scratch.0.join("state"), config, None);
        let in_work = flume_seams::Host::settings(&host, "/r", "fix", "work");
        assert_eq!(in_work.every_agent, every);
        assert_eq!(in_work.stream.model.as_deref(), Some("opus"));
        assert_eq!(
            in_work.every_kind.instructions.as_deref(),
            Some("Mind the tagline.")
        );
        let in_default = flume_seams::Host::settings(&host, "/s", "fix", "default");
        assert_eq!(in_default.stream, Settings::default());
        assert_eq!(in_default.every_kind, Settings::default());
        assert_eq!(in_default.every_agent.model.as_deref(), Some("sonnet"));
    }

    #[test]
    fn a_table_for_every_agent_or_a_stream_cannot_hold() {
        // C.7: a program, an environment, a pane beside or a stage in
        // `[agents]` or a stream's, and a stream the host does not
        // declare, refused naming the table and the key.
        for (text, named) in [
            ("[agents]\nprogram = \"agent\"\n", "[agents] program"),
            ("[agents]\nenv = { X = \"1\" }\n", "[agents] env"),
            ("[agents]\nbeside = \"pane\"\n", "[agents] beside"),
            (
                "[agents.stages.code]\nmodel = \"opus\"\n",
                "[agents] stages is no setting of the host's for every agent: a stage's name is its kind's, so a stage's settings go in [kinds.<kind>.agents.stages.<stage>]",
            ),
            (
                "[streams.work]\n[agents.streams.work]\nprogram = \"agent\"\n",
                "[agents.streams.work] program",
            ),
            (
                "[streams.work]\n[agents.streams.work.stages.code]\nmodel = \"opus\"\n",
                "[agents.streams.work] stages",
            ),
            (
                "[agents.streams.missing]\nmodel = \"opus\"\n",
                "[agents.streams.missing] names the stream missing, which no [streams] table declares",
            ),
            ("[agents]\nstreams = 3\n", "[agents.streams] is not a table"),
            ("agents = 3\n", "[agents] is not a table"),
        ] {
            let refused = HostConfig::parse(text).unwrap_err();
            assert!(refused.contains(named), "{text}: {refused}");
        }
        let config = HostConfig::parse("[agents.streams.default]\nmodel = \"opus\"\n").unwrap();
        assert_eq!(
            config.streams_agents["default"].model.as_deref(),
            Some("opus"),
            "default always stands"
        );
    }

    #[test]
    fn own_settings_for_a_kind_and_for_a_repositorys_stage() {
        // 32, 195, C.7: every agents table of the host's gives
        // `programs`, a file relative to the configuration's directory.
        let scratch = Scratch::new("kind-own");
        scratch.write(
            "fix-code.json",
            r#"{ "statusLine": { "type": "command", "command": "fix-line" } }"#,
        );
        let config = scratch.write(
            "config.toml",
            "[kinds.fix.agents.programs.agent.settings]\ntui = \"fullscreen\"\n\
             [repos.\"/r\".kinds.fix.agents.stages.code.programs.agent]\nfile = \"fix-code.json\"\n",
        );
        let config = HostConfig::read(&config).unwrap();
        assert_eq!(
            config.kinds["fix"].every_stage.programs["agent"].settings["tui"],
            serde_json::json!("fullscreen")
        );
        let file = config.repos["/r"]["fix"].stages["code"].programs["agent"]
            .file
            .clone()
            .expect("the file");
        assert_eq!(file.written, "fix-code.json");
        assert_eq!(
            file.read,
            scratch.0.join("fix-code.json").display().to_string()
        );
        assert_eq!(
            file.keys["statusLine"],
            serde_json::json!({ "type": "command", "command": "fix-line" })
        );
    }

    #[test]
    fn a_programs_that_is_no_table_of_entries() {
        // 195, C.7: refused naming the table and `programs`; a program's
        // key beside its file, and a date, refused naming the key.
        for (text, named) in [
            ("[agents]\nprograms = \"agent\"\n", "agents.programs:"),
            ("[agents.programs]\nagent = 3\n", "agents.programs.agent:"),
            (
                "[agents.programs.agent]\nline = \"a\"\n",
                "agents.programs.agent.line: is no word of a program's entry, which gives file and settings: a program's own settings go under settings",
            ),
            (
                "[agents.programs.agent.settings]\nsince = 2026-11-01\n",
                "agents.programs.agent.settings.since: is a date or time, which flume cannot pass on: give it as a string",
            ),
            (
                "[kinds.fix.agents.stages.code.programs]\nagent = \"x\"\n",
                "kinds.fix.agents.stages.code.programs.agent:",
            ),
        ] {
            let refused = HostConfig::parse(text).unwrap_err();
            assert!(refused.contains(named), "{text}: {refused}");
        }
    }

    #[test]
    fn a_hosts_file_flume_cannot_read() {
        // 195, C.7: a name neither `.json` nor `.toml`, a file that does
        // not exist or does not parse, refused naming the field and the
        // file; no `$VAR` read in a path (core.30).
        let scratch = Scratch::new("bad-file");
        scratch.write("broken.json", "{ not json");
        for (file, words) in [
            (
                "settings.yaml",
                "settings.yaml is named neither .json nor .toml",
            ),
            ("missing.json", "missing.json could not be read"),
            ("broken.json", "broken.json does not parse as JSON"),
            ("$HOME/x.json", "$HOME/x.json could not be read"),
        ] {
            let config = scratch.write(
                "config.toml",
                &format!("[agents.programs.agent]\nfile = {file:?}\n"),
            );
            let refused = HostConfig::read(&config).unwrap_err();
            assert!(
                refused.contains("agents.programs.agent.file: ") && refused.contains(words),
                "{file}: {refused}"
            );
        }
    }

    #[test]
    fn a_hosts_file_read_where_its_link_leads() {
        // 195, C.7: a link its configurer keeps, read where it leads, at
        // each reading of the configuration.
        let scratch = Scratch::new("linked");
        let first = scratch.write("first.json", r#"{ "tui": "fullscreen" }"#);
        let second = scratch.write("second.json", r#"{ "tui": "default" }"#);
        let link = scratch.0.join("agent-settings.json");
        std::os::unix::fs::symlink(&first, &link).unwrap();
        let config = scratch.write(
            "config.toml",
            "[agents.programs.agent]\nfile = \"agent-settings.json\"\n",
        );
        let keys = || {
            HostConfig::read(&config).unwrap().every_agent.programs["agent"]
                .file
                .clone()
                .unwrap()
                .keys
        };
        assert_eq!(keys()["tui"], serde_json::json!("fullscreen"));
        std::fs::remove_file(&link).unwrap();
        std::os::unix::fs::symlink(&second, &link).unwrap();
        assert_eq!(keys()["tui"], serde_json::json!("default"), "read again");
    }

    #[test]
    fn a_forks_table_for_every_kind() {
        // S102, 32, 195, C.7: `[repos."<repository>".agents]` by the
        // repository's location, `~` the home.
        let config = HostConfig::parse(
            "[repos.\"~/Code/upstream-fork\".agents.programs.agent.settings]\n\
             attribution = { commit = \"\", pr = \"\", sessionUrl = false }\n\
             [repos.\"~/Code/upstream-fork\".kinds.fix.agents]\nmodel = \"opus\"\n",
        )
        .unwrap();
        let fork = home("~/Code/upstream-fork");
        assert_eq!(
            config.repos_agents[&fork].programs["agent"].settings["attribution"],
            serde_json::json!({ "commit": "", "pr": "", "sessionUrl": false })
        );
        assert_eq!(
            config.repos[&fork]["fix"].every_stage.model.as_deref(),
            Some("opus"),
            "beside its kinds' tables"
        );
    }

    #[test]
    fn a_repositorys_table_that_names_a_stage() {
        // C.7: a stage, a program, an environment, a pane beside or a
        // stream in `[repos."<repository>".agents]`, refused naming the
        // table and the key.
        for key in ["stages.code", "program", "env", "beside", "streams.work"] {
            let (table, value) = match key.split_once('.') {
                Some((table, name)) => (format!(".{table}.{name}"), "model = \"opus\""),
                None => (String::new(), ""),
            };
            let text = match value {
                "" => format!("[repos.\"/r\".agents]\n{key} = \"agent\"\n"),
                value => format!("[repos.\"/r\".agents{table}]\n{value}\n"),
            };
            let refused = HostConfig::parse(&text).unwrap_err();
            let name = key.split('.').next().unwrap();
            assert!(
                refused.contains(&format!("[repos.\"/r\".agents] {name} is no setting")),
                "{text}: {refused}"
            );
        }
    }

    #[test]
    fn own_settings_set_locally_key_by_key() {
        // C.7; record 0032: each own setting and each file of every
        // agents table one key of the local layer's, a file read beside
        // the configuration, and a pin holding one.
        let scratch = Scratch::new("own-local");
        scratch.write("local.json", r#"{ "outputStyle": "plain" }"#);
        let managed = scratch.write(
            "config.toml",
            "[host]\npin = [\"agents.programs.agent.settings.line\"]\n\
             [streams.work]\n\
             [agents.programs.agent.settings]\ntui = \"fullscreen\"\nline = \"managed\"\n\
             [repos.\"/r\".agents]\nmodel = \"opus\"\n",
        );
        let local = scratch.write(
            "config.local.toml",
            "[agents.programs.agent]\nfile = \"local.json\"\n\
             [agents.programs.agent.settings]\ntui = \"default\"\nline = \"local\"\n\
             [agents.streams.work]\nmodel = \"haiku\"\n\
             [repos.\"/r\".agents.programs.agent.settings]\nattribution = { commit = \"\" }\n\
             [kinds.fix.agents.stages.code.programs.agent.settings]\ntui = \"code\"\n",
        );
        let config = HostConfig::read_layers(&managed, &local).unwrap();
        assert_eq!(config.configuration.local, LocalLayer::Taken);
        let agent = &config.every_agent.programs["agent"];
        assert_eq!(agent.settings["tui"], serde_json::json!("default"));
        assert_eq!(
            agent.settings["line"],
            serde_json::json!("managed"),
            "pinned"
        );
        assert_eq!(
            agent.file.as_ref().unwrap().keys["outputStyle"],
            serde_json::json!("plain")
        );
        assert_eq!(
            config.streams_agents["work"].model.as_deref(),
            Some("haiku")
        );
        let r = &config.repos_agents["/r"];
        assert_eq!(r.model.as_deref(), Some("opus"), "the managed key kept");
        assert_eq!(
            r.programs["agent"].settings["attribution"],
            serde_json::json!({ "commit": "" })
        );
        assert_eq!(
            config.kinds["fix"].stages["code"].programs["agent"].settings["tui"],
            serde_json::json!("code")
        );
        assert_eq!(
            value_at(&config, "agents.programs.agent.settings.tui"),
            Some(("\"default\"".into(), ConfigLayer::Local))
        );
        assert_eq!(
            config.configuration.passed_over,
            vec![PinnedKey {
                key: "agents.programs.agent.settings.line".into(),
                pin: "agents.programs.agent.settings.line".into(),
            }]
        );
        let bad = scratch.write(
            "config.local.toml",
            "[agents.programs.agent]\nfile = \"gone.json\"\n",
        );
        let config = HostConfig::read_layers(&managed, &bad).unwrap();
        let LocalLayer::NotTaken { keys, fault } = &config.configuration.local else {
            panic!("a local file naming no file taken");
        };
        assert!(keys.is_empty(), "{keys:?}");
        assert!(
            fault
                .as_deref()
                .is_some_and(|fault| fault.contains("gone.json")),
            "{fault:?}"
        );
        assert!(config.every_agent.programs["agent"].file.is_none());
    }
}
