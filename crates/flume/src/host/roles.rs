//! The roles a host's sources declare (132, 143, 191, 193, C.7; design 1,
//! 2; core.28, core.30). A source's file in `sources/` may give `roles`,
//! an absolute path or one under `~`: the file in which the source
//! declares the roles it answers, its own tool's, written whole beside it
//! and renamed over it. flume reads it as it stands at each ask and writes
//! nothing in it. Each entry is a `[roles."<stream>/<role>"]` table, one
//! of three:
//!
//! - a run's: `kind`, one of the source's own kinds by its name, its
//!   version the locks', and `repository`, by its location on the host,
//!   a leading `~` read as the home; and `priority`, `base`, `brief`,
//!   `name`, `intent`, `answers` and `answerer`, each where it gives it,
//!   and under `agents` its agents' settings as a launch's `--agent`
//!   gives them, every stage's and each stage's under `stages`, a file a
//!   program's own settings name read beside the roles file;
//! - a standing agent's: `agent`, the name the host's terminal knows it
//!   by, and `session`, `answers` and `answerer` where it gives them;
//! - a carried address's: `carried = true`, and `answers` where it gives
//!   it, for an address whose run is on another host.
//!
//! A run's and a standing agent's address names a stream the host
//! declares, and its stream is that one, so no entry names a stream. A
//! fault stays its own (191): a roles file that cannot be read declares
//! no role; an entry naming a kind not its source's, an `env`, a pool,
//! an account or a `stream`, naming in its address a stream the host does
//! not declare and not carried, giving `agent` beside `kind` or a run's
//! other keys, or mixing `carried` with a run's keys, `agent` or
//! `answerer`, declares nothing for its address, and so does one writing
//! its kind with a version, which is the locks', or giving `points`,
//! which this flume takes for no place (165); an address two entries
//! declare, and two addresses whose runs would be one place, of one
//! repository, kind and name, are declared by neither. Each is named with
//! why, and none fails the configuration, another source or another
//! entry.

use super::config::{home, quoted};
use flume_seams::{
    is_address, role_of, stream_of, CarriedRole, HostRoles, HostSource, HostSourceRead, RoleAgents,
    RoleEntry, RoleRefused, RoleRun, RolesUnread, Settings, StandingAgent, Stream,
};
use std::collections::BTreeMap;
use std::path::Path;
use toml::{Table, Value};

/// The keys of a run's entry.
const RUN: &[&str] = &[
    "kind",
    "repository",
    "priority",
    "base",
    "brief",
    "name",
    "intent",
    "agents",
    "answers",
    "answerer",
];

/// The keys of a standing agent's entry.
const STANDING: &[&str] = &["agent", "session", "answers", "answerer"];

/// The settings an entry's `agents` table gives, as `--agent` gives them
/// (7, 32, 107, 162, 195, 197, 198).
const AGENTS: &[&str] = &[
    "program",
    "model",
    "effort",
    "instructions",
    "beside",
    "session",
    "mode",
    "keep_open",
    "programs",
];

/// What the host's sources declare of roles, each roles file read now:
/// `sources` the host declares and `streams` its streams (132, 143, 191,
/// 193, C.7).
pub fn read(
    sources: &BTreeMap<String, HostSource>,
    streams: &BTreeMap<String, Stream>,
) -> HostRoles {
    let mut roles = HostRoles::default();
    let mut entries: Vec<(String, RoleEntry)> = vec![];
    for (source, declared) in sources {
        let HostSourceRead::Read {
            roles: Some(file), ..
        } = &declared.read
        else {
            continue;
        };
        let read = match entries_of(source, Path::new(file)) {
            Ok(read) => read,
            Err(reason) => {
                roles.unread.insert(
                    source.clone(),
                    RolesUnread {
                        file: file.clone(),
                        reason,
                    },
                );
                continue;
            }
        };
        for (address, entry) in read {
            match entry.and_then(|entry| in_its_stream(&address, entry, streams)) {
                Ok(entry) => entries.push((address, entry)),
                Err(reason) => roles.refused.push(RoleRefused {
                    address,
                    source: source.clone(),
                    reason,
                }),
            }
        }
    }
    claimed(entries, &mut roles);
    roles
}

/// An entry of a roles file: the address it names, and the entry read or
/// its fault.
type EntryRead = (String, Result<RoleEntry, String>);

/// Each entry of the roles file at `path`, of `source`, by the address
/// it names, each read or its fault; or why the file declares no role
/// (191, 193, C.7).
fn entries_of(source: &str, path: &Path) -> Result<Vec<EntryRead>, String> {
    let text =
        std::fs::read_to_string(path).map_err(|error| format!("could not be read: {error}"))?;
    let table = text
        .parse::<Table>()
        .map_err(|error| format!("is not TOML: {}", error.message().trim()))?;
    if let Some(key) = table.keys().find(|key| *key != "roles") {
        return Err(format!(
            "`{key}` is no key of a roles file, which gives `roles` alone"
        ));
    }
    let Some(roles) = table.get("roles") else {
        return Ok(vec![]);
    };
    let roles = roles
        .as_table()
        .ok_or("`roles` is not a table of addresses".to_string())?;
    let directory = path.parent().unwrap_or(Path::new("/"));
    Ok(roles
        .iter()
        .map(|(address, value)| (address.clone(), entry_of(source, address, value, directory)))
        .collect())
}

/// The entry `value` of `source` for `address`, or why it declares
/// nothing (132, 193, C.7; core.30).
fn entry_of(
    source: &str,
    address: &str,
    value: &Value,
    directory: &Path,
) -> Result<RoleEntry, String> {
    let at = format!("roles.{}", quoted(address));
    if !is_address(address) {
        return Err(format!(
            "[{at}] names no role's address, which is <stream>/<role>"
        ));
    }
    let table = value.as_table().ok_or(format!("[{at}] is not a table"))?;
    for (key, why) in [
        ("env", "an agent's environment is its account's alone"),
        (
            "pool",
            "an agent's account is chosen from its stream's pool",
        ),
        (
            "account",
            "an agent's account is chosen from its stream's pool",
        ),
        ("stream", "its address names its stream"),
    ] {
        if table.contains_key(key) {
            return Err(format!(
                "[{at}] gives {key}, which no role's entry gives: {why}"
            ));
        }
    }
    let carried = match table.get("carried") {
        None => false,
        Some(Value::Boolean(carried)) => *carried,
        Some(_) => return Err(format!("[{at}] carried is not true or false")),
    };
    let keys: Vec<&String> = table.keys().filter(|key| *key != "carried").collect();
    let entry = Entry { table, at: &at };
    if carried {
        if let Some(key) = keys.iter().find(|key| key.as_str() != "answers") {
            return Err(format!(
                "[{at}] gives carried beside {key}: a carried address gives answers alone"
            ));
        }
        return Ok(RoleEntry::Carried(CarriedRole {
            source: source.into(),
            answers: entry.text("answers")?,
        }));
    }
    if table.contains_key("agent") {
        if table.contains_key("kind") {
            return Err(format!(
                "[{at}] gives both a standing agent and a kind, two agents of one role"
            ));
        }
        if let Some(key) = keys.iter().find(|key| !STANDING.contains(&key.as_str())) {
            return Err(format!(
                "[{at}] gives {key} beside agent, which is no word of a standing agent's entry"
            ));
        }
        return Ok(RoleEntry::Standing(StandingAgent {
            source: source.into(),
            agent: entry
                .name("agent")?
                .ok_or(format!("[{at}] names no agent"))?,
            session: entry.name("session")?,
            answers: entry.text("answers")?,
            answerer: entry.flag("answerer")?,
        }));
    }
    if let Some(key) = keys.iter().find(|key| !RUN.contains(&key.as_str())) {
        let why = match key.as_str() {
            "points" => {
                ": this flume takes a place's points from nothing, as it takes no stream's (165)"
            }
            _ => "",
        };
        return Err(format!("[{at}] {key} is no word of a role's run{why}"));
    }
    let named = |key: &str| entry.text(key)?.ok_or(format!("[{at}] names no {key}"));
    let priority = match table.get("priority") {
        None => 0,
        Some(value) => value
            .as_integer()
            .ok_or(format!("[{at}] priority is not a whole number"))?,
    };
    Ok(RoleEntry::Run(Box::new(RoleRun {
        source: source.into(),
        repository: home(&named("repository")?),
        kind: own_kind(source, &named("kind")?, &at)?,
        priority,
        base: entry.text("base")?,
        brief: entry.text("brief")?,
        name: entry.name("name")?,
        intent: entry.name("intent")?,
        agents: match table.get("agents") {
            None => RoleAgents::default(),
            Some(agents) => role_agents(agents, &format!("{at}.agents"), directory)?,
        },
        answers: entry.text("answers")?,
        answerer: entry.flag("answerer")?,
    })))
}

/// One entry's table at `at`, its values read.
struct Entry<'a> {
    table: &'a Table,
    at: &'a str,
}

impl Entry<'_> {
    /// The string `key` gives, none where it gives none.
    fn text(&self, key: &str) -> Result<Option<String>, String> {
        match self.table.get(key) {
            None => Ok(None),
            Some(Value::String(value)) => Ok(Some(value.clone())),
            Some(_) => Err(format!("[{}] {key} is not a string", self.at)),
        }
    }

    /// The string `key` gives, which may not be empty.
    fn name(&self, key: &str) -> Result<Option<String>, String> {
        match self.text(key)? {
            Some(value) if value.is_empty() => Err(format!("[{}] {key} is empty", self.at)),
            given => Ok(given),
        }
    }

    /// Whether `key` gives true, false where it gives nothing.
    fn flag(&self, key: &str) -> Result<bool, String> {
        match self.table.get(key) {
            None => Ok(false),
            Some(Value::Boolean(value)) => Ok(*value),
            Some(_) => Err(format!("[{}] {key} is not true or false", self.at)),
        }
    }
}

/// The kind `written` names, one of `source`'s own by its name: written
/// bare or under the source's own name, never another source's and never
/// with a version, which is the locks' (154, 155, 193; core.34).
fn own_kind(source: &str, written: &str, at: &str) -> Result<String, String> {
    let name = match written.split_once('/') {
        Some((of, name)) if of == source => name,
        Some(_) => {
            return Err(format!(
                "[{at}] names the kind {written}, which is not {source}'s own: a role runs as one of its source's own kinds"
            ))
        }
        None => written,
    };
    if name.contains('@') {
        return Err(format!(
            "[{at}] names the kind {written} with a version: a role's kind is named by its name, its version the locks'"
        ));
    }
    if name.is_empty() {
        return Err(format!("[{at}] names no kind"));
    }
    Ok(name.to_string())
}

/// An entry's `agents` table at `at`: every stage's settings as a launch's
/// `--agent` gives them, and each stage's under `stages`; an `env`, or a
/// key no `--agent` sets, refused naming it (7, 32, 195; core.30).
fn role_agents(agents: &Value, at: &str, directory: &Path) -> Result<RoleAgents, String> {
    let agents = agents
        .as_table()
        .ok_or(format!("[{at}] is not a table of settings"))?;
    let mut stages = BTreeMap::new();
    if let Some(named) = agents.get("stages") {
        let named = named
            .as_table()
            .ok_or(format!("[{at}.stages] is not a table of stages"))?;
        for (stage, table) in named {
            let at = format!("{at}.stages.{}", quoted(stage));
            let table = table.as_table().ok_or(format!("[{at}] is not a table"))?;
            stages.insert(stage.clone(), settings_of(table, &at, directory, false)?);
        }
    }
    Ok(RoleAgents {
        every_stage: settings_of(agents, at, directory, true)?,
        stages,
    })
}

/// One agents table's settings at `at`, each as written, the mode read as
/// a declaration's is and the programs' own settings by flume's one
/// reader of them, a file they name read beside the roles file in
/// `directory`; `stages` taken where `staged`, and any other key refused
/// (32, 162, 169, 195; core.30).
fn settings_of(
    table: &Table,
    at: &str,
    directory: &Path,
    staged: bool,
) -> Result<Settings, String> {
    if let Some(key) = table
        .keys()
        .find(|key| !(AGENTS.contains(&key.as_str()) || staged && *key == "stages"))
    {
        let why = match key.as_str() {
            "env" => "an agent's environment is its account's alone",
            _ => "a role's run's agents take the settings a launch's --agent gives",
        };
        return Err(format!("[{at}] {key} is no setting of a role's run: {why}"));
    }
    let entry = Entry { table, at };
    let mode = table
        .get("mode")
        .map(crate::mode::read)
        .transpose()
        .map_err(|reason| format!("[{at}] mode: {reason}"))?;
    let programs = table
        .get("programs")
        .map(|programs| {
            crate::own::read(programs, at, &|written| {
                let path = directory.join(home(written));
                let read = path.display().to_string();
                std::fs::read_to_string(&path)
                    .map(|text| (read.clone(), text))
                    .map_err(|error| format!("{read} could not be read: {error}"))
            })
        })
        .transpose()?
        .unwrap_or_default();
    Ok(Settings {
        program: entry.name("program")?,
        model: entry.text("model")?,
        effort: entry.text("effort")?,
        instructions: entry.text("instructions")?,
        beside: entry.name("beside")?,
        session: entry.name("session")?,
        mode,
        keep_open: super::config::keep_open_of(table, at)?,
        programs,
    })
}

/// `entry` for `address` where its address's stream is one the host
/// declares, or it is carried; otherwise why it declares nothing (4, 143,
/// 193).
fn in_its_stream(
    address: &str,
    entry: RoleEntry,
    streams: &BTreeMap<String, Stream>,
) -> Result<RoleEntry, String> {
    let stream = stream_of(address);
    if matches!(entry, RoleEntry::Carried(_)) || streams.contains_key(stream) {
        Ok(entry)
    } else {
        Err(format!(
            "{address} names the stream {stream}, which the host does not declare, and is not carried"
        ))
    }
}

/// `entries`, each read and in its stream, declared in `roles`, but an
/// address two entries declare and two addresses whose runs would be one
/// place, each refused naming both (3, 4, 132, 193).
fn claimed(entries: Vec<(String, RoleEntry)>, roles: &mut HostRoles) {
    let mut by_address: BTreeMap<String, Vec<RoleEntry>> = BTreeMap::new();
    for (address, entry) in entries {
        by_address.entry(address).or_default().push(entry);
    }
    let mut single: BTreeMap<String, RoleEntry> = BTreeMap::new();
    for (address, mut claims) in by_address {
        if claims.len() == 1 {
            single.extend(claims.pop().map(|entry| (address, entry)));
            continue;
        }
        let sources: Vec<&str> = claims.iter().map(RoleEntry::source).collect();
        let reason = format!(
            "{address} is declared by {}, so by neither",
            sources.join(" and by ")
        );
        for entry in &claims {
            roles.refused.push(RoleRefused {
                address: address.clone(),
                source: entry.source().into(),
                reason: reason.clone(),
            });
        }
    }
    let mut by_place: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for (address, entry) in &single {
        if let RoleEntry::Run(run) = entry {
            by_place
                .entry((run.repository.clone(), place_of(address, run)))
                .or_default()
                .push(address.clone());
        }
    }
    for ((repository, place), addresses) in by_place {
        if addresses.len() < 2 {
            continue;
        }
        let reason = format!(
            "{} would each run as {place} in {repository}, one place, so neither is declared",
            addresses.join(" and ")
        );
        for address in addresses {
            if let Some(entry) = single.remove(&address) {
                roles.refused.push(RoleRefused {
                    address,
                    source: entry.source().into(),
                    reason: reason.clone(),
                });
            }
        }
    }
    roles.declared = single;
}

/// The place a role's run at `address` is made as, `<kind>--<name>`, its
/// name its entry's, else `<stream>-<role>` (3, 132, 193).
fn place_of(address: &str, run: &RoleRun) -> String {
    let name = run
        .name
        .clone()
        .unwrap_or_else(|| format!("{}-{}", stream_of(address), role_of(address)));
    format!("{}--{name}", run.kind)
}
