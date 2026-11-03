//! The host's configuration and directories (C.7; design 10): flume's
//! state under `$XDG_STATE_HOME/flume`, and its configuration in
//! `$XDG_CONFIG_HOME/flume/config.toml`, each beneath `$HOME` as the XDG
//! directories default where the variable is unset. The configuration's
//! `[host] agents` is the bound on agents running (70) and `[host]
//! terminal` the terminal profile bound for the host (105); a file or a
//! word left out is no bound and the printed command. Its
//! `[kinds.<kind>.agents]` and `[repos."<repository>".kinds.<kind>.agents]`
//! tables, each naming one stage's by a further `.stages.<stage>` table,
//! give a kind's agents in every repository and in one their `model`,
//! `instructions` and `mode`, each kept as written, a mode one of
//! flume's four or a table of programs whose entries are strings (32,
//! 162, 169, C.7; records 0017, 0020, 0024); a
//! repository's key has a leading `~` read as the home, so it names the
//! repository by its location as a launch does. Its
//! `[roles."<partition>/<role>"]` tables are the runs the host wakes for
//! each role's address, with `repository`, `kind`, `priority`, `base`,
//! `brief` and `stream`, an address it names nothing for woken by nothing
//! (132, C.7). Its `[streams.<stream>]` tables declare the host's streams,
//! each with `agents`, `priority` and `pool`, and `default` stands
//! whether or not a table declares it; its `[pools.<pool>]` tables its
//! pools, each with `strategy` and `accounts`, each account a `name`, a
//! `program`, an `env` kept as written and an optional `agents`; `[host]
//! pool` is the host's own pool; and `stream` in `[repos."<repository>"]`,
//! `[repos."<repository>".kinds.<kind>]` and `[kinds.<kind>]` names a
//! place's stream (143, 144, C.7).
//!
//! The configuration has two layers (C.7; design 4): `config.toml` is the
//! managed one, and `config.local.toml` beside it the local one, read
//! over it key by key, each key of [`LOCAL_KEYS`] it sets replacing the
//! managed one's whole. A local file setting any other key, or at fault,
//! is not taken at all, the managed layer read alone; a key `[host] pin`
//! holds is passed over. The reading keeps each value with its layer. No
//! command writes either file (core.16, core.45).

use flume_seams::{
    is_address, Account, ConfigLayer, ConfigValue, HostConfiguration, KindSettings, LocalLayer,
    PassedOver, Pool, PoolFrom, PoolNamed, RoleRun, Settings, Strategy, Stream, StreamFrom,
    StreamHeld, DEFAULT_STREAM, SET,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use toml::{Table, Value};

/// The keys the local layer may set, each a path of segments, `*` any one
/// name: a stream's `agents`, `priority`, `points`, `pace` and `session`,
/// a label's `reserve` and `keep_free`, the `stream` a repository, a
/// repository's kind or a kind is given, the `label` a repository's kind
/// or a kind is given, and a kind's agents table, every stage's and one
/// stage's (32, 107, 143, 166, 167, 180, 181, C.7). Each is one unit, its
/// value taken whole. A change that gives the local layer another key
/// adds it here.
pub const LOCAL_KEYS: &[&[&str]] = &[
    &["streams", "*", "agents"],
    &["streams", "*", "priority"],
    &["streams", "*", "points"],
    &["streams", "*", "pace"],
    &["streams", "*", "session"],
    &["streams", "*", "labels", "*", "reserve"],
    &["streams", "*", "labels", "*", "keep_free"],
    &["repos", "*", "stream"],
    &["repos", "*", "kinds", "*", "stream"],
    &["kinds", "*", "stream"],
    &["repos", "*", "kinds", "*", "label"],
    &["kinds", "*", "label"],
    &["kinds", "*", "agents", "*"],
    &["kinds", "*", "agents", "stages", "*", "*"],
    &["repos", "*", "kinds", "*", "agents", "*"],
    &["repos", "*", "kinds", "*", "agents", "stages", "*", "*"],
];

/// The host's configuration as its layers state it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostConfig {
    /// The bound on agents running, every caller's counted together;
    /// none where the file states none (70).
    pub agents: Option<u32>,
    /// The terminal profile the host binds, none for the printed command
    /// (105).
    pub terminal: Option<String>,
    /// The settings given each kind's agents in every repository, by the
    /// kind's name (32).
    pub kinds: BTreeMap<String, KindSettings>,
    /// The settings given each kind's agents in one repository, by its
    /// location and the kind's name (32).
    pub repos: BTreeMap<String, BTreeMap<String, KindSettings>>,
    /// The run woken for each role's address, by the address (132, C.7).
    pub roles: BTreeMap<String, RoleRun>,
    /// The streams declared, by name, `default` among them always (143).
    pub streams: BTreeMap<String, Stream>,
    /// The streams named for places, by repository and kind, by
    /// repository and by kind (143).
    pub streams_named: StreamsNamed,
    /// The host's own pool, none where it names none (144).
    pub pool: Option<String>,
    /// The pools declared, by name (144).
    pub pools: BTreeMap<String, Pool>,
    /// Every value with its layer, whether the local layer was taken, and
    /// each local key passed over with its pin; none of it where only one
    /// table was read (C.7).
    pub configuration: HostConfiguration,
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

impl Default for HostConfig {
    fn default() -> Self {
        HostConfig {
            agents: None,
            terminal: None,
            kinds: BTreeMap::new(),
            repos: BTreeMap::new(),
            roles: BTreeMap::new(),
            streams: BTreeMap::from([(DEFAULT_STREAM.to_string(), Stream::default())]),
            streams_named: StreamsNamed::default(),
            pool: None,
            pools: BTreeMap::new(),
            configuration: HostConfiguration::default(),
        }
    }
}

impl HostConfig {
    /// The configuration in `path`, the managed layer with no local one:
    /// none of it where there is no file, and the file's fault named where
    /// it is not one.
    pub fn read(path: &Path) -> Result<HostConfig, String> {
        let managed = text_in(path).map_err(|error| format!("{}: {error}", path.display()))?;
        HostConfig::layers(&managed.unwrap_or_default(), None)
            .map_err(|fault| format!("{}: {fault}", path.display()))
    }

    /// The configuration of the managed layer in `managed` and the local
    /// layer in `local` beside it: a managed file's fault refuses it,
    /// naming the file, and a local file that cannot be read is not taken
    /// (C.7; design 4).
    pub fn read_layers(managed: &Path, local: &Path) -> Result<HostConfig, String> {
        let text = text_in(managed).map_err(|error| format!("{}: {error}", managed.display()))?;
        let managed_text = text.unwrap_or_default();
        let config = match text_in(local) {
            Ok(local) => HostConfig::layers(&managed_text, local.as_deref()),
            Err(fault) => HostConfig::layers(&managed_text, None)
                .map(|config| config.local_not_taken(vec![], Some(fault))),
        };
        config.map_err(|fault| format!("{}: {fault}", managed.display()))
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
    /// refuses the configuration.
    pub fn layers(managed: &str, local: Option<&str>) -> Result<HostConfig, String> {
        let managed: Table = managed.parse().map_err(|error| format!("{error}"))?;
        let alone = HostConfig::from_table(&managed)?.with_layers(
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
                Some((pin, _)) => passed_over.push(PassedOver {
                    key: dotted(&path),
                    pin: pin.clone(),
                }),
                None => {
                    set_at(&mut merged, &path, value);
                    taken.push(read);
                }
            }
        }
        match HostConfig::from_table(&merged) {
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
        passed_over: Vec<PassedOver>,
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
    /// kept.
    pub fn parse(text: &str) -> Result<HostConfig, String> {
        let table: toml::Table = text.parse().map_err(|error| format!("{error}"))?;
        HostConfig::from_table(&table)
    }

    /// The configuration `table` states, its pins checked and no layer
    /// kept.
    fn from_table(table: &Table) -> Result<HostConfig, String> {
        pins_of(table)?;
        let mut config = HostConfig::default();
        if let Some(kinds) = table.get("kinds") {
            config.kinds = kinds_settings(kinds, "kinds")?;
            config.streams_named.kind = kinds_streams(kinds, "kinds")?;
        }
        if let Some(given) = table.get("repos") {
            let given = given
                .as_table()
                .ok_or("[repos] is not a table of repositories".to_string())?;
            for (repository, table) in given {
                let at = format!("repos.{}", quoted(repository));
                let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
                let location = home(repository);
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
                config.repos.insert(location, kinds_settings(kinds, &at)?);
            }
        }
        if let Some(roles) = table.get("roles") {
            config.roles = role_runs(roles)?;
        }
        if let Some(pools) = table.get("pools") {
            config.pools = pools_declared(pools)?;
        }
        if let Some(streams) = table.get("streams") {
            config.streams.extend(streams_declared(streams)?);
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
            config.terminal = text_of(host, "terminal", "host")
                .map_err(|_| "[host] terminal is not a name".to_string())?;
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
/// (`pools.work.accounts[0].name`), an `env` table's values as `<set>`
/// and never as written, and every other leaf as TOML writes it; each
/// local where it lies at or under a unit of `taken`, else managed (C.7;
/// core.30).
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
        _ if at.secret || env => push(SET.to_string()),
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
fn kinds_settings(kinds: &Value, at: &str) -> Result<BTreeMap<String, KindSettings>, String> {
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
                stages.insert(stage.clone(), agent_settings(own, &at)?);
            }
        }
        settings.insert(
            kind.clone(),
            KindSettings {
                every_stage: agent_settings(agents, &at)?,
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

/// The `streams` table: each stream with its bound, its priority, zero
/// where it gives none, and its pool; a key a stream does not know
/// refused naming it (143, C.7).
fn streams_declared(streams: &Value) -> Result<BTreeMap<String, Stream>, String> {
    let streams = streams
        .as_table()
        .ok_or("[streams] is not a table of streams".to_string())?;
    let mut declared = BTreeMap::new();
    for (name, table) in streams {
        let at = format!("streams.{}", quoted(name));
        let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
        only(table, &["agents", "priority", "pool"], &at, "a stream")?;
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
                ..Stream::default()
            },
        );
    }
    Ok(declared)
}

/// The `pools` table: each pool's strategy and its accounts in order,
/// each with a name, a program, an `env` kept as written and an optional
/// bound; a pool with no accounts, two accounts of one name, an account
/// with no name or program, or a key either does not know refused naming
/// it (144, C.7).
fn pools_declared(pools: &Value) -> Result<BTreeMap<String, Pool>, String> {
    let pools = pools
        .as_table()
        .ok_or("[pools] is not a table of pools".to_string())?;
    let mut declared = BTreeMap::new();
    for (name, table) in pools {
        let at = format!("pools.{}", quoted(name));
        let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
        only(table, &["strategy", "accounts"], &at, "a pool")?;
        let strategy = match text_of(table, "strategy", &at)?.as_deref() {
            Some("fill-first") => Strategy::FillFirst,
            Some("least-busy") => Strategy::LeastBusy,
            Some(other) => {
                return Err(format!(
                "[{at}] strategy is {other}, where a pool's strategy is fill-first or least-busy"
            ))
            }
            None => return Err(format!("[{at}] names no strategy")),
        };
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
                &["name", "program", "env", "agents"],
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
                name,
            });
        }
        declared.insert(name.clone(), Pool { strategy, accounts });
    }
    Ok(declared)
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

/// The `roles` table: each address's run, its repository and kind named,
/// its priority zero, its base the repository's default branch and its
/// brief none where the table gives none; a key that is no address, or a
/// key the run does not know, refused naming it (132, C.7).
fn role_runs(roles: &Value) -> Result<BTreeMap<String, RoleRun>, String> {
    let roles = roles
        .as_table()
        .ok_or("[roles] is not a table of addresses".to_string())?;
    let mut runs = BTreeMap::new();
    for (address, table) in roles {
        let at = format!("roles.{}", quoted(address));
        if !is_address(address) {
            return Err(format!(
                "[{at}] names no role's address, which is <partition>/<role>"
            ));
        }
        let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
        if let Some(unknown) = table.keys().find(|key| {
            !matches!(
                key.as_str(),
                "repository" | "kind" | "priority" | "base" | "brief" | "stream"
            )
        }) {
            return Err(format!("[{at}] {unknown} is no word of a role's run"));
        }
        let text = |name: &str| match table.get(name) {
            None => Ok(None),
            Some(Value::String(value)) => Ok(Some(value.clone())),
            Some(_) => Err(format!("[{at}] {name} is not a string")),
        };
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
    Ok(runs)
}

/// One agents table's `model`, `instructions` and `mode`, each as
/// written, the mode read as a declaration's is (162, 169); any other
/// key, `stages` among them, is not a setting of the host's (32, C.7;
/// record 0020).
fn agent_settings(table: &Table, at: &str) -> Result<Settings, String> {
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
    Ok(Settings {
        model: text("model")?,
        instructions: text("instructions")?,
        mode,
        ..Settings::default()
    })
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

    /// C.7's example, as the statement writes it.
    const C7: &str = r#"
[host]
agents = 8

[streams.work]
agents = 5
priority = 10
pool = "work"

[streams.personal]
agents = 3
pool = "personal"

[pools.work]
strategy = "fill-first"
accounts = [
  { name = "work-1", program = "agent", env = { AGENT_HOME = "~/.agent-work-1" }, agents = 4 },
  { name = "work-2", program = "agent", env = { AGENT_HOME = "~/.agent-work-2" } },
]

[pools.personal]
strategy = "least-busy"
accounts = [{ name = "personal", program = "agent", env = { AGENT_HOME = "~/.agent-personal" } }]

[repos."/home/user/Code/acme"]
stream = "work"
"#;

    #[test]
    fn c7s_example_read_whole() {
        let config = HostConfig::parse(C7).unwrap();
        assert_eq!(config.agents, Some(8));
        assert_eq!(
            config.streams.keys().collect::<Vec<_>>(),
            vec!["default", "personal", "work"]
        );
        let acme = "/home/user/Code/acme";
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
        assert_eq!(work.strategy, Strategy::FillFirst);
        assert_eq!(
            work.accounts,
            vec![
                Account {
                    name: "work-1".into(),
                    program: "agent".into(),
                    env: BTreeMap::from([("AGENT_HOME".into(), "~/.agent-work-1".into())]),
                    agents: Some(4),
                },
                Account {
                    name: "work-2".into(),
                    program: "agent".into(),
                    env: BTreeMap::from([("AGENT_HOME".into(), "~/.agent-work-2".into())]),
                    agents: None,
                },
            ],
            "in order, each env kept as written"
        );
        assert_eq!(config.pools["personal"].strategy, Strategy::LeastBusy);
        assert_eq!(config.stream_for("/elsewhere", "fix"), None);
        assert_eq!(config.pool_for("default"), None, "no pool for default");
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
            vec![PassedOver {
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
                vec![PassedOver {
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
}
