//! The host's configuration and directories (C.7; design 10): flume's
//! state under `$XDG_STATE_HOME/flume`, and its configuration in
//! `$XDG_CONFIG_HOME/flume/config.toml`, each beneath `$HOME` as the XDG
//! directories default where the variable is unset. The configuration's
//! `[host] agents` is the bound on agents running (70) and `[host]
//! terminal` the terminal profile bound for the host (105); a file or a
//! word left out is no bound and the printed command. Its
//! `[kinds.<kind>.agents]` and `[repos."<repository>".kinds.<kind>.agents]`
//! tables, each naming one stage's by a further `.stages.<stage>` table,
//! are the settings the host gives a kind's agents in every repository and
//! in one, each value kept as written (32, C.7; record 0017); a
//! repository's key has a leading `~` read as the home, so it names the
//! repository by its location as a launch does. Its
//! `[roles."<partition>/<role>"]` tables are the runs the host wakes for
//! each role's address, with `repository`, `kind`, `priority`, `base` and
//! `brief`, an address it names nothing for woken by nothing (132, C.7).

use flume_seams::{is_address, KindSettings, RoleRun, Settings};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use toml::{Table, Value};

/// The host's configuration as its file states it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
        let kinds = match table.get("kinds") {
            None => BTreeMap::new(),
            Some(kinds) => kinds_settings(kinds, "kinds")?,
        };
        let mut repos = BTreeMap::new();
        if let Some(given) = table.get("repos") {
            let given = given
                .as_table()
                .ok_or("[repos] is not a table of repositories".to_string())?;
            for (repository, table) in given {
                let at = format!("repos.{}", quoted(repository));
                let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
                let kinds = match table.get("kinds") {
                    None => BTreeMap::new(),
                    Some(kinds) => kinds_settings(kinds, &format!("{at}.kinds"))?,
                };
                repos.insert(home(repository), kinds);
            }
        }
        let roles = match table.get("roles") {
            None => BTreeMap::new(),
            Some(roles) => role_runs(roles)?,
        };
        let Some(host) = table.get("host") else {
            return Ok(HostConfig {
                kinds,
                repos,
                roles,
                ..HostConfig::default()
            });
        };
        let host = host.as_table().ok_or("[host] is not a table".to_string())?;
        let agents = match host.get("agents") {
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
        let terminal = match host.get("terminal") {
            None => None,
            Some(value) => Some(
                value
                    .as_str()
                    .ok_or("[host] terminal is not a name".to_string())?
                    .to_string(),
            ),
        };
        Ok(HostConfig {
            agents,
            terminal,
            kinds,
            repos,
            roles,
        })
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
                "repository" | "kind" | "priority" | "base" | "brief"
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
            },
        );
    }
    Ok(runs)
}

/// One agents table's `program`, `model`, `instructions` and `env`, each
/// as written; any other key, `stages` among them, is not a setting of
/// the host's (32, C.7).
fn agent_settings(table: &Table, at: &str) -> Result<Settings, String> {
    let text = |name: &str| match table.get(name) {
        None => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(format!("[{at}] {name} is not a string")),
    };
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
    Ok(Settings {
        program: text("program")?,
        model: text("model")?,
        instructions: text("instructions")?,
        env,
        beside: None,
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
            "[kinds.change.agents]\nmodel = \"opus\"\nenv = { AGENT_HOME = \"~/.agent-work\" }\n\
             [repos.\"/r\".kinds.change.agents.stages.verify]\nmodel = \"sonnet\"\n",
        )
        .unwrap();
        let kind = &config.kinds["change"];
        assert_eq!(kind.every_stage.model.as_deref(), Some("opus"));
        assert_eq!(
            kind.every_stage.env["AGENT_HOME"], "~/.agent-work",
            "kept as written"
        );
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
        assert!(HostConfig::parse("[kinds.change.agents]\nenv = { A = 1 }\n").is_err());
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
            },
            "no base, the repository's default branch, and no brief"
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

    #[test]
    fn a_bound_that_is_no_count_is_refused() {
        assert!(HostConfig::parse("[host]\nagents = -1\n").is_err());
        assert!(HostConfig::parse("[host]\nagents = \"two\"\n").is_err());
    }
}
