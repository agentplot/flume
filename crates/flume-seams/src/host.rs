//! The host's own facts the core reads: its clock, a new secret, a
//! file's write time, whether a command of flume's still lives, the bound
//! on agents its configuration states, its streams with their sessions,
//! labels and displays, its pools of accounts, the labels it gives kinds,
//! its configuration's values with their layers, the sources of its
//! kinds and the roles they declare, each address's run, standing agent
//! or carrying, each stream's answerer and the roles that answer, and a
//! lock over the room (design 3, 6). No seam of Part B.

use crate::{KindSettings, Seconds, Secret, Settings};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The stream every place runs in that nothing names a stream for, which
/// the host always declares (143).
pub const DEFAULT_STREAM: &str = "default";

/// A stream the host's configuration declares: its bound on agents
/// running, none where it states none and the host's bound alone holds
/// it; its priority among streams, zero where it gives none; the pool
/// its agents' accounts come from, none where the host's own pool stands;
/// the terminal session its agents open in, none where it names none and
/// its name, or the terminal's own for the default stream, stands; the
/// sessions it names for kinds and their stages, by the kind's name, none
/// where it names none; its labels' shares of its agents, by label, none
/// where it states none; the terminal profile its agents run in, the view
/// a person is shown them in and the window's profile, each none where it
/// names none and the host's stands (`Host::display_for`); and the title
/// a window on its part of the terminal carries, none where it gives none
/// and its name stands (107, 143, 144, 181, 186, 190, 199, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stream {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agents: Option<u32>,
    #[serde(default)]
    pub priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub kinds: BTreeMap<String, KindSessions>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, Share>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiplexer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<View>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_app: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// A stream's table for a kind, `[streams.<stream>.kinds.<kind>]`: the
/// session the kind's agents open in, none where it names none; its
/// stages' tables, by the stage's name in the kind, none where it gives
/// none; and each other key it holds, which refuses every start that
/// reads it rather than the configuration (199, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindSessions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub stages: BTreeMap<String, StageSession>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown: Vec<String>,
}

/// A stream's table for one stage of a kind,
/// `[streams.<stream>.kinds.<kind>.stages.<stage>]`: the session that
/// stage's agents open in, none where it names none, and each other key
/// it holds, which refuses every start that reads it (199, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageSession {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown: Vec<String>,
}

/// A session a stream names for its kinds and their stages, by its name:
/// each kind it names it for, and each stage as `<kind>.<stage>`, with
/// the layer its `session` came from, none of either where it names it
/// for none (56, 199, C.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedSession {
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub kinds: Vec<NamedFor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<NamedFor>,
}

/// A kind, or a kind's stage as `<kind>.<stage>`, a session is named
/// for, and the layer of the configuration that names it (56, 199, C.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedFor {
    pub name: String,
    pub layer: ConfigLayer,
}

/// A fault of a stream's table for a kind or a stage, by the table's
/// dotted key, `streams.work.kinds.research`, and the refusal it causes
/// every start that reads it: another key it holds, or a session that is
/// another stream's (4, 56, 199).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableFault {
    pub table: String,
    pub refused: String,
}

/// How a person is shown a stream's agents: in a window a terminal app
/// opens on the terminal's command, or in the terminal's own app (186).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum View {
    Terminal,
    App,
}

impl View {
    /// The view as the host's configuration and status spell it.
    pub fn name(self) -> &'static str {
        match self {
            View::Terminal => "terminal",
            View::App => "app",
        }
    }
}

/// Where a stream's multiplexer, view or terminal app came from: its
/// stream's key, the host's, or neither, the default standing (186, 190,
/// C.7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NamedBy {
    Stream,
    Host,
    Neither,
}

/// A profile by its name and where it came from; the default the host's
/// reader names where neither the stream nor the host names one (105,
/// 190).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileHeld {
    pub profile: String,
    pub from: NamedBy,
}

/// A stream's view and where it came from (186).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewHeld {
    pub view: View,
    pub from: NamedBy,
}

/// A stream's terminal and how a person views it: the terminal profile
/// its agents run in, the view, and the window profile a terminal view
/// opens in (143, 186, 190).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamDisplay {
    pub multiplexer: ProfileHeld,
    pub view: ViewHeld,
    pub terminal_app: ProfileHeld,
}

/// A label's share of its stream's agents: how many of the stream's
/// agents its places are owed, `owed`, and how many of those are held for
/// it alone, `reserved`, each a whole number of agents from zero, zero
/// where its table gives none; and each other key its table holds, which
/// refuses every start of the stream rather than the configuration (181,
/// 182, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Share {
    #[serde(default)]
    pub owed: u32,
    #[serde(default)]
    pub reserved: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown: Vec<String>,
}

/// Where a place's label came from: the host's configuration for its
/// repository and kind, the host's for its kind, or the kind's own
/// (180).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LabelFrom {
    HostRepositoryKind,
    HostKind,
    Kind,
}

/// A label by its name and where it came from (56, 180).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelHeld {
    pub name: String,
    pub from: LabelFrom,
}

/// The layer of the host's configuration a value came from: the managed
/// `config.toml`, or the local `config.local.toml` read over it (C.7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConfigLayer {
    Managed,
    Local,
}

/// One value of the host's configuration by its dotted key, as TOML
/// writes it, an account's variable's as [`crate::SET`], and the layer
/// it came from (56, C.7; core.30).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigValue {
    pub key: String,
    pub value: String,
    pub layer: ConfigLayer,
}

/// Whether the host's local layer was taken: no local file, taken, or
/// not taken at all, naming each key it may not set and, where it could
/// not be read, why (C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum LocalLayer {
    #[default]
    NoFile,
    Taken,
    NotTaken {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        keys: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fault: Option<String>,
    },
}

/// A key the local layer set that the managed layer pins, passed over,
/// with the pin that holds it (C.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinnedKey {
    pub key: String,
    pub pin: String,
}

/// The host's configuration as it stands when asked: whether its local
/// layer was taken, every value with its layer, and each local key
/// passed over with its pin (56, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostConfiguration {
    pub local: LocalLayer,
    #[serde(default)]
    pub values: Vec<ConfigValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passed_over: Vec<PinnedKey>,
}

/// How a pool chooses among its usable accounts: the first in order, the
/// one running the fewest of flume's agents, or the one with the most
/// left by a fresh reading, equals in order (145, 184).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Strategy {
    FillFirst,
    LeastBusy,
    MostLeft,
}

impl Strategy {
    /// The strategy as the host's configuration and status spell it.
    pub fn name(self) -> &'static str {
        match self {
            Strategy::FillFirst => "fill-first",
            Strategy::LeastBusy => "least-busy",
            Strategy::MostLeft => "most-left",
        }
    }
}

/// An account of a pool: its name, the agent program it runs, the
/// environment that signs that program in, each value as the
/// configuration wrote it, and how many agents run on it at once, none
/// where it states none (144, C.7). Its environment is an agent's whole
/// environment beyond the login environment (32). It names no reading:
/// its reading is what its program reports in a session flume started,
/// kept in the account's own document (184, 211; record 0062).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub program: String,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agents: Option<u32>,
}

/// A pool of accounts, in order, the strategy by which one is chosen at
/// each start, and the whole percentage of a window past which an account
/// takes no new start, none where the pool states none and 90 holds (144,
/// 145, 185).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pool {
    pub strategy: Strategy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<u32>,
    pub accounts: Vec<Account>,
}

/// An account's reading, kept in its own document by flume alone: when
/// flume took it, by the host's clock, the agent whose start's program
/// reported it, by its identity, the session it came from, none where the
/// agent's record names none, and each of its usage windows (153, 184,
/// 211, C.7). It is written in C.7's form: `taken_at` RFC 3339's with its
/// offset, UTC written `Z`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountReading {
    #[serde(with = "rfc3339")]
    pub taken_at: Seconds,
    pub agent: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    pub windows: Vec<UsageWindow>,
}

/// One usage window of an account's allowance: its name as the program
/// gives it, how much of it is used in hundredths of a percent, 0 to
/// 10000, and when it resets, none where the report gives none (184, 211,
/// C.7). It is written in C.7's form: `used` a share from 0 to 100,
/// written from its hundredths, 42 or 23.5, and `resets_at` RFC 3339's
/// with its offset, UTC written `Z`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageWindow {
    pub name: String,
    #[serde(with = "share")]
    pub used: u32,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "rfc3339_or_none"
    )]
    pub resets_at: Option<Seconds>,
}

/// A time by the host's clock as RFC 3339 writes it, UTC written `Z`, the
/// seconds whole (C.7).
pub fn rfc3339_of(seconds: Seconds) -> String {
    i64::try_from(seconds)
        .ok()
        .and_then(|seconds| chrono::DateTime::<chrono::Utc>::from_timestamp(seconds, 0))
        .map(|at| at.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
        .unwrap_or_else(|| seconds.to_string())
}

/// A time RFC 3339 writes with its offset, read as seconds by the host's
/// clock, a fraction of a second dropped; none where RFC 3339 cannot read
/// it or it is before 1970 (C.7).
pub fn rfc3339_read(written: &str) -> Option<Seconds> {
    chrono::DateTime::parse_from_rfc3339(written)
        .ok()
        .and_then(|at| Seconds::try_from(at.timestamp()).ok())
}

/// A time written as RFC 3339's (C.7).
mod rfc3339 {
    use crate::Seconds;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(seconds: &Seconds, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&super::rfc3339_of(*seconds))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Seconds, D::Error> {
        let written = String::deserialize(deserializer)?;
        super::rfc3339_read(&written).ok_or_else(|| {
            serde::de::Error::custom(format!("`{written}` is no time RFC 3339 reads"))
        })
    }
}

/// A time written as RFC 3339's where there is one (C.7).
mod rfc3339_or_none {
    use crate::Seconds;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(
        seconds: &Option<Seconds>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match seconds {
            Some(seconds) => super::rfc3339::serialize(seconds, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Seconds>, D::Error> {
        match Option::<String>::deserialize(deserializer)? {
            Some(written) => super::rfc3339_read(&written).map(Some).ok_or_else(|| {
                serde::de::Error::custom(format!("`{written}` is no time RFC 3339 reads"))
            }),
            None => Ok(None),
        }
    }
}

/// Hundredths of a percent written as a share from 0 to 100, a whole
/// number where it is one (C.7).
mod share {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(hundredths: &u32, serializer: S) -> Result<S::Ok, S::Error> {
        match hundredths % 100 {
            0 => serializer.serialize_u32(hundredths / 100),
            _ => serializer.serialize_f64(f64::from(*hundredths) / 100.0),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
        let share = f64::deserialize(deserializer)?;
        if !(0.0..=100.0).contains(&share) {
            return Err(serde::de::Error::custom(format!(
                "{share} is not a share from 0 to 100"
            )));
        }
        Ok((share * 100.0).round() as u32)
    }
}

/// A repository as the host's configuration names it: by its address,
/// `<owner>/<name>` as its clone's `origin` gives it on its forge, else by
/// its folder (143, C.7; record 0055). The host's tables for a repository
/// are the address's where the configuration has a table under it, whole,
/// and the folder's otherwise.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepositoryName {
    /// `<owner>/<name>` as its clone's origin gives it on its forge; none
    /// where it has no forge origin, or a place held none.
    pub address: Option<String>,
    /// The folder that holds it, as a launch names it.
    pub folder: String,
}

impl RepositoryName {
    /// A repository named by its folder alone, with no address.
    pub fn of_folder(folder: impl Into<String>) -> RepositoryName {
        RepositoryName {
            address: None,
            folder: folder.into(),
        }
    }
}

/// Where a place's stream came from (143).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StreamFrom {
    /// The launch named it.
    Launch,
    /// The intent's affinity stands in it (147).
    Intent,
    /// The host's configuration for the repository and kind.
    HostRepositoryKind,
    /// The host's configuration for the repository.
    HostRepository,
    /// The host's configuration for the kind.
    HostKind,
    /// Nothing named one.
    Default,
}

/// A stream by its name and where it came from (143, 56).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamHeld {
    pub name: String,
    pub from: StreamFrom,
}

/// Where a stage's pool came from: its stream's `pool`, or the host's
/// own (144).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PoolFrom {
    Stream,
    Host,
}

/// A pool by its name and where it came from (144, 56).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoolNamed {
    pub name: String,
    pub from: PoolFrom,
}

/// The host's room lock, held while this guard lives and released when
/// it is dropped (70, 75).
pub struct RoomGuard(#[allow(dead_code)] Box<dyn Send>);

impl RoomGuard {
    /// A guard that keeps the lock as long as `held` lives.
    pub fn new(held: impl Send + 'static) -> RoomGuard {
        RoomGuard(Box::new(held))
    }
}

impl std::fmt::Debug for RoomGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RoomGuard")
    }
}

/// The settings the host's configuration gives a kind's agents in one
/// repository and one stream, each value as the configuration wrote it,
/// each table none where the configuration gives none, and in settings
/// written before (32, 143, 195, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostSettings {
    /// For the kind in one repository.
    pub repository: KindSettings,
    /// For every kind in the repository, its
    /// `[repos."<repository>".agents]`, beneath its table for the kind.
    #[serde(default)]
    pub every_kind: Settings,
    /// For the kind in every repository.
    pub kind: KindSettings,
    /// For every agent in the stream, its `[agents.streams.<stream>]`.
    #[serde(default)]
    pub stream: Settings,
    /// For every agent on the host, its `[agents]`.
    #[serde(default)]
    pub every_agent: Settings,
}

/// The run a source declares for a role's address, as the
/// `[roles."<stream>/<role>"]` entry of the roles file its declaration
/// names gives it: the source that declares it; the repository it is
/// launched in, by its location on the host, and the kind it is launched
/// as, one of its source's own named by its name, its version the
/// locks'; at the priority given, zero where it gives none; on the base
/// named, the repository's default branch where it names none; with the
/// brief, the place's name (else `<stream>-<role>`), the intent and the
/// agents' settings it gives, each none where it gives none; the words of
/// what the role answers, which a stage's prompt gives; and whether it
/// stands behind its address's stream. Its stream is its address's, and
/// it names none of its own (7, 28, 132, 143, 154, 155, 193, C.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleRun {
    #[serde(default)]
    pub source: String,
    pub repository: String,
    pub kind: String,
    #[serde(default)]
    pub priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brief: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    #[serde(default, skip_serializing_if = "RoleAgents::is_empty")]
    pub agents: RoleAgents,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answers: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub answerer: bool,
}

/// The settings a role's entry gives its run's agents under its `agents`
/// table, as a launch's `--agent` gives them: every stage's, and each
/// stage's by the stage's name under `stages`, which come before every
/// stage's; none where it gives none (7, 32, 193, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleAgents {
    #[serde(default, skip_serializing_if = "is_unset")]
    pub every_stage: Settings,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub stages: BTreeMap<String, Settings>,
}

impl RoleAgents {
    /// Whether the entry gives its run's agents no setting.
    pub fn is_empty(&self) -> bool {
        is_unset(&self.every_stage) && self.stages.is_empty()
    }
}

/// Whether settings give nothing.
fn is_unset(settings: &Settings) -> bool {
    *settings == Settings::default()
}

/// The standing agent a source declares for a role's address, as its
/// roles file's entry gives it with `agent`: the source that declares
/// it; the name the terminal knows the agent by, which a caller keeps
/// standing; the session it lives in where the entry names one, else its
/// address's stream's own; the words of what the role answers; and
/// whether it stands behind its address's stream. It is found in its
/// address's stream's terminal, the address's ends go into it, and flume
/// starts, wakes and keeps nothing of it (43, 132, 143, 190, 193, C.5,
/// C.7; core.28; record 0039's amendment).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StandingAgent {
    #[serde(default)]
    pub source: String,
    pub agent: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answers: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub answerer: bool,
}

/// An address a source declares carried, `carried = true`, its run on
/// another host and its stream that host's: the source, and the words of
/// what the role answers where it gives them. flume launches nothing for
/// it and keeps its ends pending (132, 193, C.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CarriedRole {
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answers: Option<String>,
}

/// What one entry of a source's roles file declares for its address: a
/// run, a standing agent, or the address carried to another host (132,
/// 193, C.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoleEntry {
    Run(Box<RoleRun>),
    Standing(StandingAgent),
    Carried(CarriedRole),
}

impl RoleEntry {
    /// The source that declares it.
    pub fn source(&self) -> &str {
        match self {
            RoleEntry::Run(run) => &run.source,
            RoleEntry::Standing(agent) => &agent.source,
            RoleEntry::Carried(carried) => &carried.source,
        }
    }

    /// The words of what the role answers, none where it gives none.
    pub fn answers(&self) -> Option<&str> {
        match self {
            RoleEntry::Run(run) => run.answers.as_deref(),
            RoleEntry::Standing(agent) => agent.answers.as_deref(),
            RoleEntry::Carried(carried) => carried.answers.as_deref(),
        }
    }

    /// Whether it stands behind its address's stream; a carried address
    /// never does.
    pub fn answerer(&self) -> bool {
        match self {
            RoleEntry::Run(run) => run.answerer,
            RoleEntry::Standing(agent) => agent.answerer,
            RoleEntry::Carried(_) => false,
        }
    }
}

/// An entry of a source's roles file that declares nothing for its
/// address: the address as the entry names it, the source, and why: the
/// entry at fault, its address's stream one the host does not declare,
/// the address another entry declares too, or another address whose run
/// would be the same place (3, 4, 132, 143, 193, C.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleRefused {
    pub address: String,
    pub source: String,
    pub reason: String,
}

/// A source's roles file that cannot be read, so declares no role: the
/// file as the source's declaration names it, read, and why (191, 193).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolesUnread {
    pub file: String,
    pub reason: String,
}

/// A role's address and the source that declares it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DeclaredBy {
    pub address: String,
    pub source: String,
}

/// Who stands behind a stream: the one role its sources declare to, an
/// address of the stream, with its source; or the roles that each
/// declare to, two or more, so none stands behind it (143, 193).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamAnswerer {
    Stands(DeclaredBy),
    Contested { roles: Vec<DeclaredBy> },
}

/// A role a source declares with the words of what it answers: its
/// address, its stream and the words (28, 41, 132, 193).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnsweringRole {
    pub address: String,
    pub stream: String,
    pub answers: String,
}

/// What the host's sources declare of roles, read from the roles file
/// each source's declaration names as they stand when asked, and never
/// kept: each address one entry declares, with that entry; each entry
/// that declares nothing for its address, with why; and each source whose
/// roles file cannot be read, by the source, with the file and why (132,
/// 191, 193, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostRoles {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub declared: BTreeMap<String, RoleEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub refused: Vec<RoleRefused>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub unread: BTreeMap<String, RolesUnread>,
}

impl HostRoles {
    /// What the sources declare for `address`, or why nothing is: each
    /// entry's reason that names it and declares nothing, else that no
    /// source declares it (132, 193).
    pub fn declared(&self, address: &str) -> Result<RoleEntry, String> {
        if let Some(entry) = self.declared.get(address) {
            return Ok(entry.clone());
        }
        let mut reasons: Vec<&str> = vec![];
        for refused in self
            .refused
            .iter()
            .filter(|refused| refused.address == address)
        {
            if !reasons.contains(&refused.reason.as_str()) {
                reasons.push(&refused.reason);
            }
        }
        if reasons.is_empty() {
            Err(format!("no source of the host declares {address}"))
        } else {
            Err(reasons.join("; "))
        }
    }

    /// The run a source declares for `address`; none where it declares a
    /// standing agent, a carried address or nothing (132).
    pub fn run(&self, address: &str) -> Option<RoleRun> {
        match self.declared.get(address) {
            Some(RoleEntry::Run(run)) => Some(RoleRun::clone(run)),
            _ => None,
        }
    }

    /// Each address a source declares a standing agent for, by the
    /// address (132; record 0039's amendment).
    pub fn standing(&self) -> BTreeMap<String, StandingAgent> {
        self.declared
            .iter()
            .filter_map(|(address, entry)| match entry {
                RoleEntry::Standing(agent) => Some((address.clone(), agent.clone())),
                _ => None,
            })
            .collect()
    }

    /// Who stands behind `stream`: none where no declared role stands
    /// behind it (143, 193).
    pub fn answerer(&self, stream: &str) -> Option<StreamAnswerer> {
        let mut roles: Vec<DeclaredBy> = self
            .declared
            .iter()
            .filter(|(address, entry)| entry.answerer() && stream_of(address) == stream)
            .map(|(address, entry)| DeclaredBy {
                address: address.clone(),
                source: entry.source().to_string(),
            })
            .collect();
        match roles.len() {
            0 => None,
            1 => roles.pop().map(StreamAnswerer::Stands),
            _ => Some(StreamAnswerer::Contested { roles }),
        }
    }

    /// Every address a source declares with the words of what it
    /// answers, a run's, a standing agent's or a carried one's, each with
    /// its stream and its words, in the addresses' order (28, 41, 132,
    /// 193).
    pub fn answering(&self) -> Vec<AnsweringRole> {
        self.declared
            .iter()
            .filter_map(|(address, entry)| {
                entry.answers().map(|answers| AnsweringRole {
                    address: address.clone(),
                    stream: stream_of(address).to_string(),
                    answers: answers.to_string(),
                })
            })
            .collect()
    }
}

/// A source the host's configuration declares by a file of its own in its
/// sources directory, the file's name less `.toml` being the source's
/// (191, C.7): the file, and the directory its packages lie in or why
/// the file cannot be read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostSource {
    pub file: String,
    pub read: HostSourceRead,
}

/// A source's file read: the directory holding its packages, and the file
/// in which it declares its roles where it names one, none where it
/// names none, each leading `~` read as the host user's home; or why the
/// file cannot be read, a name that is flume's own source's, the
/// repository's or a launch's among the reasons (154, 191, 193, C.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostSourceRead {
    Read {
        kinds_dir: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        roles: Option<String>,
    },
    CouldNotRead {
        reason: String,
    },
}

/// Whether a recipient's name is a role's address, `<stream>/<role>`,
/// each part a word of letters, digits, `.`, `_` and `-`: the role a
/// source declares for a stream; any other name is a session's (132,
/// 143, 193; design 1).
pub fn is_address(name: &str) -> bool {
    let word = |part: &str| {
        !part.is_empty()
            && part
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
    };
    name.split_once('/')
        .is_some_and(|(stream, role)| word(stream) && word(role))
}

/// The role an address names, its part after its stream (132, 193).
pub fn role_of(address: &str) -> &str {
    address.split_once('/').map_or(address, |(_, role)| role)
}

/// The stream an address names, its part before its role: the stream a
/// role's run runs in and its standing agent is found in (143, 193).
pub fn stream_of(address: &str) -> &str {
    address.split_once('/').map_or("", |(stream, _)| stream)
}

/// The host's facts (design 3).
pub trait Host: Send + Sync {
    /// The host's clock.
    fn now(&self) -> Seconds;

    /// A wait inside a command of flume's (38, 54).
    fn sleep(&self, seconds: Seconds);

    /// A secret no other start has (41).
    fn secret(&self) -> Secret;

    /// A file's write time by the host's clock, if it exists (25).
    fn write_time(&self, path: &str) -> Option<Seconds>;

    /// This command of flume's, as held state names the launch making a
    /// place (2, 76).
    fn command(&self) -> String;

    /// Whether a command of flume's named in held state still lives (76).
    fn alive(&self, command: &str) -> bool;

    /// The bound on agents running that the host's configuration states,
    /// every caller's counted together; none where it states none, and
    /// then the host keeps no queue (70).
    fn bound(&self) -> Option<u32>;

    /// The recipient the calling command's environment names in
    /// `FLUME_RECIPIENT`, a launch's default recipient; none where it
    /// names none (7).
    fn recipient(&self) -> Option<String>;

    /// The settings the host's configuration gives `kind`'s agents in
    /// `repository` and in `stream`, the place's, and every agent's, as
    /// it stands now: read at every start, never kept in held state; the
    /// repository's tables its address's where the configuration has a
    /// table under it, whole, else its folder's (32, 143, C.7).
    fn settings(&self, repository: &RepositoryName, kind: &str, stream: &str) -> HostSettings;

    /// What the host's sources declare of roles, read from their roles
    /// files as they stand now, and never kept (132, 191, 193, C.7).
    fn roles(&self) -> HostRoles;

    /// What the sources declare for a role's address, or why nothing is,
    /// as they stand now (132, 193).
    fn declared(&self, address: &str) -> Result<RoleEntry, String> {
        self.roles().declared(address)
    }

    /// The run a source declares for a role's address, as it stands now;
    /// none where it declares no run for it (132, 193).
    fn role(&self, address: &str) -> Option<RoleRun> {
        self.roles().run(address)
    }

    /// Each address a source declares a standing agent for, by the
    /// address, as the sources stand now: read at each delivery and each
    /// status, never kept, and none of them a run's, which `role` gives
    /// (132, 193; record 0039's amendment).
    fn standing(&self) -> BTreeMap<String, StandingAgent> {
        self.roles().standing()
    }

    /// Who stands behind `stream`, as the sources stand now; none where
    /// no role does (143, 193).
    fn answerer(&self, stream: &str) -> Option<StreamAnswerer> {
        self.roles().answerer(stream)
    }

    /// Every address a source declares with the words of what it
    /// answers, as the sources stand now (28, 41, 132, 193).
    fn answering(&self) -> Vec<AnsweringRole> {
        self.roles().answering()
    }

    /// The streams the host's configuration declares, by name, as it
    /// stands now: `default` among them always, held by the host's bound
    /// alone where no `[streams.default]` table states its own (143,
    /// C.7).
    fn streams(&self) -> BTreeMap<String, Stream>;

    /// The stream the host's configuration names for `kind` in
    /// `repository`, as it stands now: its word for the repository and
    /// kind, else for the repository, else for the kind, with where it
    /// came from; none where it names none. The repository's words are
    /// its address's table's where the configuration has a table under
    /// it, whole, else its folder's (143, C.7).
    fn stream_for(&self, repository: &RepositoryName, kind: &str) -> Option<StreamHeld>;

    /// The label the host's configuration gives `kind` in `repository`,
    /// as it stands now: its word for the repository and kind, else for
    /// the kind, with where it came from; none where it gives none. The
    /// repository's word is its address's table's where the configuration
    /// has a table under it, whole, else its folder's (180, C.7).
    fn label_for(&self, repository: &RepositoryName, kind: &str) -> Option<LabelHeld>;

    /// The host's configuration as it stands now: every value with its
    /// layer, whether the local layer was taken, and each key passed over
    /// with its pin (56, C.7).
    fn configuration(&self) -> HostConfiguration;

    /// The pool of a place in `stream`, as the host's configuration
    /// stands now: the stream's own, else the host's, with where it came
    /// from; none where neither names one, and the place's agents run
    /// under no account (144, C.7).
    fn pool_for(&self, stream: &str) -> Option<PoolNamed>;

    /// The multiplexer, view and terminal app of a place in `stream`, as
    /// the host's configuration stands now: each the stream's own, else
    /// the host's, else the default the host's reader names, with where
    /// it came from (143, 186, 190, C.7).
    fn display_for(&self, stream: &str) -> StreamDisplay;

    /// The pool the host's configuration declares under `name`, as it
    /// stands now (144, C.7).
    fn pool(&self, name: &str) -> Option<Pool>;

    /// A variable of the host user's login environment, `HOME` among
    /// them, never of the calling command's session; none where it lacks
    /// it (29).
    fn login(&self, name: &str) -> Option<String>;

    /// A line written to the host's log, for what fails nothing, as a
    /// clue that failed to show (139).
    fn log(&self, line: &str);

    /// The host-wide lock over the room, held until the guard is dropped,
    /// so two starts at once never pass the bound (70, 75).
    fn room(&self) -> RoomGuard;

    /// The sources the host's configuration declares, by name, as they
    /// stand now, each source's fault its own (159, 191, C.7); none by
    /// default, a host with no sources directory.
    fn sources(&self) -> BTreeMap<String, HostSource> {
        BTreeMap::new()
    }

    /// The directory the host's configuration keeps its sources' files in,
    /// a source named by `<source>.toml` there (191, C.7); none by
    /// default.
    fn sources_dir(&self) -> Option<String> {
        None
    }

    /// The host's lock of kinds, `$XDG_CONFIG_HOME/flume/flume.lock`, read
    /// as it stands (155, C.7); none by default, a host that keeps no lock.
    fn kinds_lock(&self) -> Option<String> {
        None
    }

    /// The span after which an end pending for a recipient is expired,
    /// `[host] ends_expire` as the host's configuration stands now, read
    /// at each delivery's record and fixed there; `ENDS_EXPIRE` by default
    /// (43, C.7; weir 104).
    fn ends_expire(&self) -> Seconds {
        ENDS_EXPIRE
    }
}

/// flume's own span after which an end pending for a recipient is
/// expired, where the host's configuration gives none: a week (43, C.7).
pub const ENDS_EXPIRE: Seconds = 7 * 24 * 60 * 60;
