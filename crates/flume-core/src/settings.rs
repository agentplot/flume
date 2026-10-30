//! An agent's settings, resolved at its start from the six layers clause
//! 32 orders and from nothing else (design 16a; record 0017; core.30):
//! the launch's, the host's for the repository and kind, the host's for
//! the kind, the repository's for a kind it does not declare, the kind's
//! declaration, and flume's default. Within a layer a stage's own come
//! before those for every stage, and every setting is taken whole. The
//! host's layers are read as they stand at the start, every other layer
//! as the launch read it. An agent's environment is its account's alone,
//! each value's `~` and variables read against the host user's login
//! environment (29, 144).

use crate::Engine;
use flume_seams::{Account, Agent, Given, Kind, KindSettings, Layer, PlaceRecord, Settings, Stage};
use std::collections::BTreeMap;
use std::sync::Arc;

/// A stage's settings with each value as its layer wrote it, and the
/// layer of each, by `program`, `model` or `instructions` (32, 56).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Resolved {
    pub settings: Settings,
    pub from: BTreeMap<String, Given>,
}

impl Resolved {
    /// Where the program came from, as a refusal names it.
    fn program_from(&self) -> Layer {
        self.from
            .get("program")
            .map_or(Layer::Default, |given| given.layer)
    }
}

/// A stage's agent at its start: its settings as written with their
/// layers, and the agent profile that runs its program.
pub(crate) struct Resolution {
    pub resolved: Resolved,
    pub agent: Arc<dyn Agent>,
}

/// Settings with `beside` taken away, for a layer that does not say
/// where in the terminal an agent runs (107, 32).
fn placeless(mut settings: Settings) -> Settings {
    settings.beside = None;
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
    /// The six layers of `stage`'s settings in `place`, most specific
    /// first, each a stage's own over those for every stage. The host's
    /// configuration is read here, at each start, and nowhere else (32,
    /// C.7).
    fn layers(&self, place: &PlaceRecord, stage: &Stage) -> [(Layer, Settings); 6] {
        let host = self.host.settings(&place.repository, &place.kind.name);
        let launched = place.settings.get(&stage.name).map_or_else(
            || place.every_stage.clone(),
            |own| place.every_stage.under(own),
        );
        let declared = place
            .kind
            .agents
            .for_stage(&stage.name)
            .under(&stage.agents.first().cloned().unwrap_or_default());
        [
            (Layer::Launch, launched),
            (
                Layer::HostRepository,
                placeless(host.repository.for_stage(&stage.name)),
            ),
            (Layer::HostKind, placeless(host.kind.for_stage(&stage.name))),
            (
                Layer::Repository,
                placeless(place.repository_settings.for_stage(&stage.name)),
            ),
            (Layer::Kind, declared),
            (Layer::Default, self.defaults.settings.clone()),
        ]
    }

    /// `stage`'s settings in `place` as each layer wrote them, folded
    /// from the most general layer to the most specific, keeping the
    /// layer of every setting and every variable (32).
    pub(crate) fn resolve(&self, place: &PlaceRecord, stage: &Stage) -> Resolved {
        let mut resolved = Resolved::default();
        for (layer, given) in self.layers(place, stage).into_iter().rev() {
            let whole = [
                ("program", &given.program),
                ("model", &given.model),
                ("instructions", &given.instructions),
            ];
            for (key, value) in whole {
                if let Some(value) = value {
                    resolved.from.insert(
                        key.into(),
                        Given {
                            layer,
                            value: value.clone(),
                        },
                    );
                }
            }
            resolved.settings = resolved.settings.under(&given);
        }
        resolved
    }

    /// The agent profile `resolved`'s program names, or why there is
    /// none, naming the program, the stage and its layer (4, 32).
    pub(crate) fn agent_for(
        &self,
        resolved: &Resolved,
        stage: &str,
    ) -> Result<Arc<dyn Agent>, String> {
        let layer = resolved.program_from();
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

    /// The stage's agent at a start: its settings resolved, and the
    /// agent profile that runs its program; or why the stage is refused
    /// (27, 32).
    pub(crate) fn resolution(
        &self,
        place: &PlaceRecord,
        stage: &Stage,
    ) -> Result<Resolution, String> {
        let resolved = self.resolve(place, stage);
        let agent = self.agent_for(&resolved, &stage.name)?;
        Ok(Resolution { resolved, agent })
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
