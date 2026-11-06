//! The host's own facts the core reads: its clock, a new secret, a
//! file's write time, whether a command of flume's still lives, the bound
//! on agents its configuration states, its streams with their sessions,
//! labels and displays, its pools of accounts, the labels it gives kinds,
//! its configuration's values with their layers, and a lock over the room
//! (design 3, 6). No seam of Part B.

use crate::{KindSettings, Seconds, Secret};
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
/// its name, or the terminal's own for the default stream, stands; its
/// labels' shares of its agents, by label, none where it states none; the
/// terminal profile its agents run in, the view a person is shown them in
/// and the window's profile, each none where it names none and the host's
/// stands (`Host::display_for`); and the title a window on its part of
/// the terminal carries, none where it gives none and its name stands
/// (107, 143, 144, 181, 186, 190, C.7).
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

/// A label's share of its stream's agents: the reserve its places start
/// within before the stream's others, and how many of its stream's slots
/// it keeps free while it runs fewer, each zero where its table gives
/// none; and each other key its table holds, which refuses every start of
/// the stream rather than the configuration (181, 182, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Share {
    #[serde(default)]
    pub reserve: u32,
    #[serde(default)]
    pub keep_free: u32,
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
/// configuration wrote it, how many agents run on it at once, none where
/// it states none, and the path of its reading as the configuration wrote
/// it, none where it names none (144, 184, C.7). Its environment is an
/// agent's whole environment beyond the login environment (32).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub program: String,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agents: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
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

/// An account's reading as the host last wrote it: when the host read the
/// account's allowance, and each of its usage windows (184, C.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usage {
    pub read_at: Seconds,
    pub windows: Vec<UsageWindow>,
}

/// One usage window of an account's allowance: its name as the host gives
/// it, how much of it is used in hundredths of a percent, 0 to 10000, and
/// when it resets, none where the reading gives none (184, C.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageWindow {
    pub name: String,
    pub used: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resets_at: Option<Seconds>,
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

/// The settings the host's configuration gives a kind's agents, each value
/// as the configuration wrote it (32, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostSettings {
    /// For the kind in one repository.
    pub repository: KindSettings,
    /// For the kind in every repository.
    pub kind: KindSettings,
}

/// The run a host wakes for a role's address, as its configuration's
/// `[roles."<partition>/<role>"]` table gives it: the repository and kind
/// it is launched as, at the priority given, on the base named or the
/// repository's default branch where it names none, with the brief where
/// one is given, in the stream named where one is (132, 143, C.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleRun {
    pub repository: String,
    pub kind: String,
    #[serde(default)]
    pub priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brief: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}

/// Whether a recipient's name is a role's address, `<partition>/<role>`,
/// each part a word of letters, digits, `.`, `_` and `-`; any other name
/// is a session's (132; design 1).
pub fn is_address(name: &str) -> bool {
    let word = |part: &str| {
        !part.is_empty()
            && part
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
    };
    name.split_once('/')
        .is_some_and(|(partition, role)| word(partition) && word(role))
}

/// The role an address names, its part after the partition: the name a
/// run of the role is launched under (132).
pub fn role_of(address: &str) -> &str {
    address.split_once('/').map_or(address, |(_, role)| role)
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
    /// `repository`, as it stands now: read at every start, never kept
    /// in held state (32, C.7).
    fn settings(&self, repository: &str, kind: &str) -> HostSettings;

    /// The run the host's configuration names for a role's address, as
    /// it stands now; none where it names nothing for it (132, C.7).
    fn role(&self, address: &str) -> Option<RoleRun>;

    /// The streams the host's configuration declares, by name, as it
    /// stands now: `default` among them always, held by the host's bound
    /// alone where no `[streams.default]` table states its own (143,
    /// C.7).
    fn streams(&self) -> BTreeMap<String, Stream>;

    /// The stream the host's configuration names for `kind` in
    /// `repository`, as it stands now: its word for the repository and
    /// kind, else for the repository, else for the kind, with where it
    /// came from; none where it names none (143, C.7).
    fn stream_for(&self, repository: &str, kind: &str) -> Option<StreamHeld>;

    /// The label the host's configuration gives `kind` in `repository`,
    /// as it stands now: its word for the repository and kind, else for
    /// the kind, with where it came from; none where it gives none (180,
    /// C.7).
    fn label_for(&self, repository: &str, kind: &str) -> Option<LabelHeld>;

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

    /// The reading in the file at `path`, an absolute path, as it stands
    /// now; or why there is none, in words that name no path (184, C.7).
    fn usage(&self, path: &str) -> Result<Usage, String>;

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
}
