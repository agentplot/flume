//! An agent's settings, resolved at its start from the eight layers
//! clause 32 orders and from nothing else (design 3; records 0017, 0043;
//! core.30): the launch's, the host's for the repository, for its kind
//! and then for every kind, the host's for the kind, the host's for the
//! place's stream, the host's for every agent, the repository's, for a
//! kind it does not declare and then for every kind, the kind's
//! declaration, and flume's default. Within a layer a stage's own come
//! before those for every stage, and those before a repository's for
//! every kind, and every setting is taken whole. A program's own
//! settings are taken key by key, each top-level key whole from the
//! first table that gives it, a table's `settings` before its file, and
//! only the agent's program's (195). The host's layers are read as they
//! stand at the start, their own settings checked then, every other
//! layer as the launch read it. An agent's environment is its account's
//! alone, each value's `~` and variables read against the host user's
//! login environment (29, 144). Its mode, where a layer gives one, is
//! mapped by its agent profile to its program's setting, or the stage
//! refused naming the mode, the program, the stage and the layer (162,
//! 169); its effort, where a layer gives one, taken whole as the model
//! is, is asked of its profile, or the stage refused naming the effort,
//! the program, the stage and the layer, and nothing is asked where no
//! layer gives one (197); and its own settings are taken by its profile,
//! or the stage refused naming each key with its layer and its file (195,
//! 196).

use crate::own::{fold, Table};
use crate::Engine;
use flume_seams::{
    Account, Agent, Deliverable, Given, Kind, KindSettings, Layer, Mode, ModeEntry, OwnFile,
    PlaceRecord, ProgramSettings, Settings, Stage,
};
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::Arc;

/// One of the eight layers of a stage's settings: its tables, most
/// specific first, a stage's own, then the table for every stage, then,
/// in a repository's two layers, the table for every kind (32).
type Tables = (Layer, Vec<Settings>);

/// A stage's settings with each value as its layer wrote it, and the
/// layer of each, by `program`, `model`, `effort`, `mode`,
/// `instructions` or `programs.<program>.settings.<key>`, an own
/// setting's with its file (32, 56, 195, 197); the agent's program's own
/// settings; and the layers they were folded from.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Resolved {
    pub settings: Settings,
    pub from: BTreeMap<String, Given>,
    /// The agent's program's own settings, none where no layer gives its
    /// program an entry (195).
    pub own: Option<Own>,
    layers: Vec<Tables>,
}

/// A program's own settings as the layers resolved them: each key whole,
/// its value as written, with its layer and its file where it came from
/// one, and each file a layer named for the program, most specific first
/// (195).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Own {
    pub keys: BTreeMap<String, Value>,
    pub from: BTreeMap<String, Given>,
    pub files: Vec<OwnFile>,
}

impl Resolved {
    /// Where the setting `key` came from, as a refusal names it.
    fn layer_of(&self, key: &str) -> Layer {
        self.from
            .get(key)
            .map_or(Layer::Default, |given| given.layer)
    }

    /// `program`'s own settings as the layers give them, by its name, key
    /// by key from the most general table to the most specific; none
    /// where no table gives it an entry (32, 195; design 4).
    pub(crate) fn own_for(&self, program: &str) -> Option<Own> {
        let program = program_name(program);
        let mut own: Option<Own> = None;
        for (layer, tables) in self.layers.iter().rev() {
            for table in tables.iter().rev() {
                let Some(entry) = table.programs.get(program) else {
                    continue;
                };
                let own = own.get_or_insert_with(Own::default);
                if let Some(file) = &entry.file {
                    own.files.insert(
                        0,
                        OwnFile {
                            layer: *layer,
                            written: file.written.clone(),
                            read: file.read.clone(),
                            left_out: file.left_out.clone(),
                        },
                    );
                }
                for (key, folded) in fold(entry) {
                    own.keys.insert(key.to_string(), folded.value.clone());
                    own.from.insert(
                        format!("programs.{program}.settings.{key}"),
                        Given {
                            layer: *layer,
                            value: folded.value.to_string(),
                            file: folded.file.map(|file| file.read.clone()),
                        },
                    );
                }
            }
        }
        own
    }
}

/// A stage's agent at its start: its settings as written with their
/// layers, the agent profile that runs its program, and the value that
/// profile passes for its mode, none where no layer gives one (162, 169).
pub(crate) struct Resolution {
    pub resolved: Resolved,
    pub agent: Arc<dyn Agent>,
    pub mode: Option<String>,
}

impl Resolution {
    /// The agent's program's own settings as the layers resolved them,
    /// which its profile takes; none where they give none (195, 196).
    pub(crate) fn own_keys(&self) -> BTreeMap<String, Value> {
        self.resolved
            .own
            .as_ref()
            .map(|own| own.keys.clone())
            .unwrap_or_default()
    }
}

/// Why `agent` refuses `stage`'s own settings for `program` as `own`
/// resolved them, each key named with its layer and its file where it
/// came from one, in the profile's words; none where it takes them or
/// they are empty (195, 196; design 13).
pub(crate) fn own_refused(
    agent: &dyn Agent,
    stage: &str,
    program: &str,
    own: &Own,
) -> Option<String> {
    if own.keys.is_empty() {
        return None;
    }
    let words = agent.own(&own.keys).err()?;
    let keys = own
        .keys
        .keys()
        .map(|key| {
            let given = own.from.get(&format!("programs.{program}.settings.{key}"));
            let layer = given.map_or(Layer::Default, |given| given.layer);
            let file = given
                .and_then(|given| given.file.as_ref())
                .map_or(String::new(), |file| format!(" by the file {file}"));
            format!("`{key}` from the layer `{layer}`{file}")
        })
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!(
        "the stage `{stage}`'s own settings for `{program}`, {keys}, are refused: {words}"
    ))
}

/// The program's name, the last component of the program its settings
/// name, by which its agent profile is bound and its entry of a mode's
/// table of programs taken (105, 169).
pub(crate) fn program_name(program: &str) -> &str {
    std::path::Path::new(program)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(program)
}

/// A deliverable as a refusal names it (24).
fn deliverable_words(deliverable: &Deliverable) -> String {
    match deliverable {
        Deliverable::Commits => "commits".into(),
        Deliverable::ChangeWritten => "a change written".into(),
        Deliverable::TasksTicked => "a change's tasks ticked".into(),
        Deliverable::ChangeArchived => "a change archived".into(),
        Deliverable::Report { path } => format!("a report at {path}"),
        Deliverable::CommittedReport { path } => format!("a committed report at {path}"),
        Deliverable::Unreadable { spelled } => spelled.clone(),
    }
}

/// The value `agent` passes for `stage`'s mode as `resolved` gives it,
/// none where no layer gives one, so nothing is mapped; or why the stage
/// cannot run in it, naming the mode as written, the program, the stage
/// and the layer: a table of programs naming no entry for its program,
/// which no later layer's mode stands in for, `plan` on a stage that is
/// no conversation, or a mode its profile refuses (4, 32, 109, 169).
fn mode_for(
    resolved: &Resolved,
    agent: &dyn Agent,
    stage: &Stage,
) -> Result<Option<String>, String> {
    let Some(mode) = &resolved.settings.mode else {
        return Ok(None);
    };
    let written = mode.written();
    let layer = resolved.layer_of("mode");
    let program = program_name(resolved.settings.program.as_deref().unwrap_or_default());
    let name = &stage.name;
    let entry = mode.entry(program).ok_or_else(|| {
        format!(
            "the stage `{name}`'s mode `{written}`, from the layer `{layer}`, names no entry for `{program}`"
        )
    })?;
    let plan = ModeEntry::Flume(Mode::Plan);
    let planning = entry == plan
        || matches!(&entry, ModeEntry::Own(value) if agent.mode(&plan).as_ref() == Ok(value));
    if planning && !stage.conversation {
        let delivers = stage
            .deliverables
            .iter()
            .map(deliverable_words)
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "the stage `{name}` delivers {delivers}, which an agent in plan mode cannot write, and its mode `{written}`, from the layer `{layer}`, is plan"
        ));
    }
    agent.mode(&entry).map(Some).map_err(|words| {
        format!(
            "the stage `{name}`'s mode `{written}`, from the layer `{layer}`, cannot run on `{program}`: {words}"
        )
    })
}

/// Whether `agent` takes `stage`'s effort as `resolved` gives it, asked
/// only where a layer gives one; or why the stage cannot run at it,
/// naming the effort as written, the program, the stage and the layer,
/// with the profile's words (4, 32, 197).
fn effort_taken(resolved: &Resolved, agent: &dyn Agent, stage: &str) -> Result<(), String> {
    let Some(effort) = &resolved.settings.effort else {
        return Ok(());
    };
    let layer = resolved.layer_of("effort");
    let program = program_name(resolved.settings.program.as_deref().unwrap_or_default());
    agent.effort(effort).map_err(|words| {
        format!(
            "the stage `{stage}`'s effort `{effort}`, from the layer `{layer}`, cannot run on `{program}`: {words}"
        )
    })
}

/// Settings with `beside` and `session` taken away, for a layer that does
/// not say where in the terminal an agent runs (107, 32).
fn placeless(mut settings: Settings) -> Settings {
    settings.beside = None;
    sessionless(settings)
}

/// Settings with `session` taken away, for every layer but the launch's:
/// a kind names nothing a host owns, and a host's agents table names no
/// session (32, 107, 158).
fn sessionless(mut settings: Settings) -> Settings {
    settings.session = None;
    settings
}

/// `value` with a leading `~` read as the home and each `$VAR` or
/// `${VAR}` as `login` gives it, or the first variable `login` lacks
/// (29, C.7).
pub(crate) fn expand(
    value: &str,
    login: impl Fn(&str) -> Option<String>,
) -> Result<String, String> {
    let mut out = String::new();
    let mut rest = value;
    if rest == "~" || rest.starts_with("~/") {
        out.push_str(&login("HOME").ok_or("HOME")?);
        rest = &rest[1..];
    }
    let named = |c: char| c.is_ascii_alphanumeric() || c == '_';
    while let Some(at) = rest.find('$') {
        out.push_str(&rest[..at]);
        let after = &rest[at + 1..];
        let (name, next) = match after.strip_prefix('{') {
            Some(braced) => match braced.find('}') {
                Some(close) => (&braced[..close], &braced[close + 1..]),
                None => ("", after),
            },
            None => {
                let end = after.find(|c: char| !named(c)).unwrap_or(after.len());
                (&after[..end], &after[end..])
            }
        };
        if name.is_empty()
            || name.starts_with(|c: char| c.is_ascii_digit())
            || !name.chars().all(named)
        {
            out.push('$');
            rest = after;
            continue;
        }
        out.push_str(&login(name).ok_or_else(|| name.to_string())?);
        rest = next;
    }
    out.push_str(rest);
    Ok(out)
}

impl Engine {
    /// The eight layers of `stage`'s settings in `place`, most specific
    /// first, each its tables most specific first (design 3). The host's
    /// configuration is read here, at each start, and nowhere else, its
    /// own settings checked as they stand, or the refusal (32, 195, C.7).
    fn layers(&self, place: &PlaceRecord, stage: &Stage) -> Result<Vec<Tables>, String> {
        let host = self
            .host
            .settings(&place.repository, &place.kind.name, &place.stream.name);
        let name = &stage.name;
        let own_of =
            |settings: &KindSettings| settings.stages.get(name).cloned().unwrap_or_default();
        let checked = |settings: &Settings, table: Table| -> Result<Settings, String> {
            self.own_checked(settings, &table).map(placeless)
        };
        let for_kind = |settings: &KindSettings, layer: Layer| -> Result<Vec<Settings>, String> {
            Ok(vec![
                checked(&own_of(settings), Table::stage(layer, name))?,
                checked(&settings.every_stage, Table::every_stage(layer))?,
            ])
        };
        let mut host_repository = for_kind(&host.repository, Layer::HostRepository)?;
        host_repository.push(checked(
            &host.every_kind,
            Table::new(Layer::HostRepository, "for every kind"),
        )?);
        let stream = Table::new(
            Layer::HostStream,
            format!("for the stream `{}`", place.stream.name),
        );
        let repository = &place.repository_settings;
        let declared = &place.kind.agents;
        Ok(vec![
            (
                Layer::Launch,
                vec![
                    place.settings.get(name).cloned().unwrap_or_default(),
                    place.every_stage.clone(),
                ],
            ),
            (Layer::HostRepository, host_repository),
            (Layer::HostKind, for_kind(&host.kind, Layer::HostKind)?),
            (Layer::HostStream, vec![checked(&host.stream, stream)?]),
            (
                Layer::Host,
                vec![checked(
                    &host.every_agent,
                    Table::new(Layer::Host, "for every agent"),
                )?],
            ),
            (
                Layer::Repository,
                vec![
                    placeless(own_of(repository)),
                    placeless(repository.every_stage.clone()),
                    placeless(place.repository_agents.clone()),
                ],
            ),
            (
                Layer::Kind,
                vec![
                    sessionless(stage.agents.first().cloned().unwrap_or_default()),
                    sessionless(own_of(declared)),
                    sessionless(declared.every_stage.clone()),
                ],
            ),
            (Layer::Default, vec![self.defaults.settings.clone()]),
        ])
    }

    /// `stage`'s settings in `place` as each layer wrote them, folded
    /// from the most general layer to the most specific, keeping the
    /// layer of every setting and every variable, and its program's own
    /// settings key by key (32, 195).
    pub(crate) fn resolve(&self, place: &PlaceRecord, stage: &Stage) -> Result<Resolved, String> {
        let mut resolved = Resolved {
            layers: self.layers(place, stage)?,
            ..Resolved::default()
        };
        for (layer, tables) in resolved.layers.iter().rev() {
            let layer = *layer;
            let given = tables
                .iter()
                .rev()
                .fold(Settings::default(), |under, table| under.under(table));
            let whole = [
                ("program", &given.program),
                ("model", &given.model),
                ("effort", &given.effort),
                ("instructions", &given.instructions),
            ];
            for (key, value) in whole {
                if let Some(value) = value {
                    resolved.from.insert(
                        key.into(),
                        Given {
                            layer,
                            value: value.clone(),
                            file: None,
                        },
                    );
                }
            }
            if let Some(mode) = &given.mode {
                resolved.from.insert(
                    "mode".into(),
                    Given {
                        layer,
                        value: mode.written(),
                        file: None,
                    },
                );
            }
            resolved.settings = resolved.settings.under(&given);
        }
        let program = resolved.settings.program.clone().unwrap_or_default();
        resolved.own = resolved.own_for(&program);
        resolved.settings.programs = BTreeMap::new();
        if let Some(own) = &resolved.own {
            resolved.from.extend(own.from.clone());
            if !own.keys.is_empty() {
                resolved.settings.programs.insert(
                    program_name(&program).to_string(),
                    ProgramSettings {
                        file: None,
                        settings: own.keys.clone(),
                    },
                );
            }
        }
        Ok(resolved)
    }

    /// The agent profile `resolved`'s program names, or why there is
    /// none, naming the program, the stage and its layer (4, 32).
    pub(crate) fn agent_for(
        &self,
        resolved: &Resolved,
        stage: &str,
    ) -> Result<Arc<dyn Agent>, String> {
        let layer = resolved.layer_of("program");
        let Some(program) = &resolved.settings.program else {
            return Err(format!("no agent program is named for the stage {stage}"));
        };
        self.agent(program).ok_or_else(|| {
            format!(
                "no agent profile runs the program {program} the stage {stage} names, from the layer {layer}"
            )
        })
    }

    /// `account`'s environment with each value's `~` and variables read
    /// against the host user's login environment, or the variable it
    /// lacks, naming it and the account (27, 29, 144).
    pub(crate) fn account_env(
        &self,
        account: &Account,
        stage: &str,
    ) -> Result<BTreeMap<String, String>, String> {
        account
            .env
            .iter()
            .map(|(name, value)| {
                expand(value, |variable| self.host.login(variable))
                    .map(|read| (name.clone(), read))
                    .map_err(|variable| {
                        format!(
                            "the stage {stage}'s account {}'s env.{name} names ${variable}, which the host user's login environment lacks",
                            account.name
                        )
                    })
            })
            .collect()
    }

    /// The stage's agent at a start: its settings resolved, the agent
    /// profile that runs its program, the value it passes for its mode,
    /// its effort, which the profile takes, and its program's own
    /// settings, which the profile takes; or why the stage is refused (27,
    /// 32, 169, 195, 197).
    pub(crate) fn resolution(
        &self,
        place: &PlaceRecord,
        stage: &Stage,
    ) -> Result<Resolution, String> {
        let resolved = self.resolve(place, stage)?;
        let agent = self.agent_for(&resolved, &stage.name)?;
        let mode = mode_for(&resolved, agent.as_ref(), stage)?;
        effort_taken(&resolved, agent.as_ref(), &stage.name)?;
        if let Some(own) = &resolved.own {
            let program = program_name(resolved.settings.program.as_deref().unwrap_or_default());
            if let Some(refused) = own_refused(agent.as_ref(), &stage.name, program, own) {
                return Err(refused);
            }
        }
        Ok(Resolution {
            resolved,
            agent,
            mode,
        })
    }
}

/// Refuses settings for a stage the kind does not hold, naming the stage
/// and the layer that gave them; the host's are ignored, never refused
/// (4, 32, C.2).
pub(crate) fn no_stage_lacking(
    kind: &Kind,
    launched: &BTreeMap<String, Settings>,
    repository: &KindSettings,
) -> Result<(), String> {
    let named = [
        (Layer::Launch, launched.keys().collect::<Vec<_>>()),
        (Layer::Repository, repository.stages.keys().collect()),
        (Layer::Kind, kind.agents.stages.keys().collect()),
    ];
    for (layer, stages) in named {
        if let Some(stage) = stages
            .into_iter()
            .find(|stage| !kind.stages.iter().any(|held| &&held.name == stage))
        {
            return Err(format!(
                "settings from the layer {layer} name the stage {stage}, which the kind {}@{} does not hold",
                kind.name, kind.version
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn login(name: &str) -> Option<String> {
        match name {
            "HOME" => Some("/home/user".into()),
            "ACCOUNT" => Some("work".into()),
            _ => None,
        }
    }

    #[test]
    fn the_home_and_the_variables_are_read() {
        assert_eq!(
            expand("~/.agent-work", login),
            Ok("/home/user/.agent-work".into())
        );
        assert_eq!(expand("~", login), Ok("/home/user".into()));
        assert_eq!(expand("a~b", login), Ok("a~b".into()));
        assert_eq!(expand("~other", login), Ok("~other".into()));
        assert_eq!(expand("$ACCOUNT-x", login), Ok("work-x".into()));
        assert_eq!(expand("${ACCOUNT}x", login), Ok("workx".into()));
        assert_eq!(expand("~/$ACCOUNT", login), Ok("/home/user/work".into()));
    }

    #[test]
    fn a_dollar_naming_nothing_is_kept() {
        assert_eq!(expand("cost $5", login), Ok("cost $5".into()));
        assert_eq!(expand("a$", login), Ok("a$".into()));
        assert_eq!(expand("${unclosed", login), Ok("${unclosed".into()));
    }

    #[test]
    fn a_variable_the_login_environment_lacks_is_named() {
        assert_eq!(expand("$NOT_SET", login), Err("NOT_SET".into()));
        assert_eq!(expand("${NOT_SET}/x", login), Err("NOT_SET".into()));
        assert_eq!(expand("~", |_| None), Err("HOME".into()));
    }
}
