//! The host's configuration and directories (C.7; design 10): flume's
//! state under `$XDG_STATE_HOME/flume`, and its configuration in
//! `$XDG_CONFIG_HOME/flume/config.toml`, each beneath `$HOME` as the XDG
//! directories default where the variable is unset. The configuration's
//! `[host] agents` is the bound on agents running (70) and `[host]
//! terminal` the terminal profile bound for the host (105); a file or a
//! word left out is no bound and the printed command.

use std::path::{Path, PathBuf};

/// The host's configuration as its file states it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HostConfig {
    /// The bound on agents running, every caller's counted together;
    /// none where the file states none (70).
    pub agents: Option<u32>,
    /// The terminal profile the host binds, none for the printed command
    /// (105).
    pub terminal: Option<String>,
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
        let Some(host) = table.get("host") else {
            return Ok(HostConfig::default());
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
        Ok(HostConfig { agents, terminal })
    }

    /// The host's configuration, `$XDG_CONFIG_HOME/flume/config.toml`.
    pub fn of_host() -> Result<HostConfig, String> {
        HostConfig::read(&config_file()?)
    }
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
                terminal: None
            }
        );
    }

    #[test]
    fn a_bound_that_is_no_count_is_refused() {
        assert!(HostConfig::parse("[host]\nagents = -1\n").is_err());
        assert!(HostConfig::parse("[host]\nagents = \"two\"\n").is_err());
    }
}
