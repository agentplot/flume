//! The host's configuration and directories (C.7; design 10): flume's
//! state under `$XDG_STATE_HOME/flume`, and its configuration in
//! `$XDG_CONFIG_HOME/flume/config.toml`, each beneath `$HOME` as the XDG
//! directories default where the variable is unset. The configuration's
//! `[host] agents` is the bound on agents running (70) and `[host]
//! terminal` the terminal profile bound for the host (105); a file or a
//! word left out is no bound and the printed command. Its
//! `[kinds.<kind>.agents]` and `[repos."<repository>".kinds.<kind>.agents]`
//! tables, each naming one stage's by a further `.stages.<stage>` table,
//! give a kind's agents in every repository and in one their `model` and
//! `instructions`, each kept as written (32, C.7; records 0017, 0020); a
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

use flume_seams::{
    is_address, Account, KindSettings, Pool, PoolFrom, PoolNamed, RoleRun, Settings, Strategy,
    Stream, StreamFrom, StreamHeld, DEFAULT_STREAM,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use toml::{Table, Value};

/// The host's configuration as its file states it.
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
        }
    }
}

impl HostConfig {
    /// The configuration in `path`: none of it where there is no file,
    /// and the file's fault named where it is not one.
    pub fn read(path: &Path) -> Result<HostConfig, String> {
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(HostConfig::default())
            }
            Err(error) => return Err(format!("{}: {error}", path.display())),
        };
        HostConfig::parse(&text).map_err(|fault| format!("{}: {fault}", path.display()))
    }

    /// The configuration `text` states.
    pub fn parse(text: &str) -> Result<HostConfig, String> {
        let table: toml::Table = text.parse().map_err(|error| format!("{error}"))?;
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

    /// The host's configuration, `$XDG_CONFIG_HOME/flume/config.toml`.
    pub fn of_host() -> Result<HostConfig, String> {
        HostConfig::read(&config_file()?)
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

/// One agents table's `model` and `instructions`, each as written; any
/// other key, `stages` among them, is not a setting of the host's (32,
/// C.7; record 0020).
fn agent_settings(table: &Table, at: &str) -> Result<Settings, String> {
    let text = |name: &str| match table.get(name) {
        None => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(format!("[{at}] {name} is not a string")),
    };
    Ok(Settings {
        model: text("model")?,
        instructions: text("instructions")?,
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

/// The host's configuration file, `$XDG_CONFIG_HOME/flume/config.toml`.
pub fn config_file() -> Result<PathBuf, String> {
    xdg("XDG_CONFIG_HOME", ".config").map(|dir| dir.join("flume").join("config.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

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
                    }
                ),
                (
                    "work".to_string(),
                    Stream {
                        agents: Some(5),
                        priority: 10,
                        pool: Some("work".into()),
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
}
