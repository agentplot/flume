//! A program's own settings as the core takes them from one table of a
//! layer (191; design 6, 7). Each key the program's agent profile
//! reserves is left out of the table's named file unread, a nested key
//! out of the table that holds it, its path kept in the file's
//! `left_out`, and refused where the table's `settings` gives it. Then
//! every string left, at any depth, is checked for a credential's shape
//! before anything holds it, a refusal naming the key's path, the layer
//! and the file and never the value (29; core.8, core.30). A table's
//! `settings` are folded over its file key by key, each key taken whole
//! (32). The check rests on shapes and is the same for every program:
//! flume reads nothing else inside a value, no `~` and no `$VAR`.

use crate::Engine;
use flume_seams::{Kind, KindSettings, Layer, NamedFile, ProgramSettings, Reserved, Settings};
use serde_json::Value;
use std::collections::BTreeMap;

/// A table of a layer as a refusal names it: its layer, and what it
/// gives settings to, `for every stage` or ``for the stage `code` ``.
#[derive(Clone, Debug)]
pub(crate) struct Table {
    pub layer: Layer,
    pub of: String,
}

impl Table {
    pub(crate) fn new(layer: Layer, of: impl Into<String>) -> Table {
        Table {
            layer,
            of: of.into(),
        }
    }

    /// A layer's table for every stage of the kind.
    pub(crate) fn every_stage(layer: Layer) -> Table {
        Table::new(layer, "for every stage")
    }

    /// A layer's table for the stage `stage` alone.
    pub(crate) fn stage(layer: Layer, stage: &str) -> Table {
        Table::new(layer, format!("for the stage `{stage}`"))
    }
}

/// The issuers' prefixes a token opens with (design 7).
const PREFIXES: [&str; 21] = [
    "sk-",
    "ghp_",
    "gho_",
    "ghu_",
    "ghs_",
    "ghr_",
    "github_pat_",
    "glpat-",
    "xoxb-",
    "xoxp-",
    "xoxa-",
    "xapp-",
    "AKIA",
    "ASIA",
    "AIza",
    "ya29.",
    "npm_",
    "pypi-",
    "hf_",
    "sk_live_",
    "rk_live_",
];

/// The endings of a key's name, in lower case with `_` and `-` taken
/// out, whose value is a secret (design 7).
const SECRET_NAMES: [&str; 10] = [
    "token",
    "secret",
    "password",
    "passwd",
    "apikey",
    "privatekey",
    "accesskey",
    "clientsecret",
    "authorization",
    "credentials",
];

/// A letter, a digit, `_` or `-`: a character of a token's run.
fn token(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

/// Each `$VAR` and `${VAR}` of `text`, with where it starts and ends, as
/// an account's `env` reads one (29).
fn variables(text: &str) -> Vec<(usize, usize)> {
    let named = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let first = |c: char| c.is_ascii_alphabetic() || c == '_';
    let mut found = vec![];
    for (at, _) in text.match_indices('$') {
        let after = &text[at + 1..];
        let end = match after.strip_prefix('{') {
            Some(braced) => braced.find('}').and_then(|close| {
                let name = &braced[..close];
                (name.starts_with(first) && name.chars().all(named)).then_some(close + 2)
            }),
            None => {
                let length = after.find(|c: char| !named(c)).unwrap_or(after.len());
                after.starts_with(first).then_some(length)
            }
        };
        if let Some(end) = end {
            found.push((at, at + 1 + end));
        }
    }
    found
}

/// `text` with each variable it names put out of every run (design 7).
fn without_variables(text: &str) -> String {
    let mut out = String::new();
    let mut from = 0;
    for (start, end) in variables(text) {
        if start < from {
            continue;
        }
        out.push_str(&text[from..start]);
        out.push(' ');
        from = end;
    }
    out.push_str(&text[from..]);
    out
}

/// Whether a run opening at `at` of `text` stands on its own, not the
/// tail of a longer word.
fn opens_a_run(text: &str, at: usize) -> bool {
    text[..at]
        .chars()
        .next_back()
        .is_none_or(|before| !token(before) && before != '.')
}

/// The length of the run of a token's characters `text` opens with.
fn run(text: &str) -> usize {
    text.find(|c: char| !token(c)).unwrap_or(text.len())
}

/// Whether `text` holds a JSON web token at `at`: three runs of
/// base64url joined by dots, the first opening `eyJ`.
fn web_token_at(text: &str, at: usize) -> bool {
    let rest = &text[at..];
    let first = run(rest);
    let Some(rest) = rest[first..].strip_prefix('.') else {
        return false;
    };
    let second = run(rest);
    if second == 0 {
        return false;
    }
    rest[second..]
        .strip_prefix('.')
        .is_some_and(|rest| run(rest) > 0)
}

/// Whether `text`, the value of the key `name`, is shaped as a
/// credential, as design 7 lists the shapes: a token's known prefix
/// followed by sixteen or more of its characters, a private key's
/// opening, a JSON web token, a run of 32 or more mixing upper case,
/// lower case and digits apart from any variable it names, or the value
/// of a key named as a secret's that names no variable.
pub fn shaped_as_credential(name: &str, text: &str) -> bool {
    let prefixed = PREFIXES.iter().any(|prefix| {
        text.match_indices(prefix)
            .any(|(at, _)| opens_a_run(text, at) && run(&text[at + prefix.len()..]) >= 16)
    });
    let private_key = text.contains("-----BEGIN") && text.contains("PRIVATE KEY-----");
    let web_token = text
        .match_indices("eyJ")
        .any(|(at, _)| opens_a_run(text, at) && web_token_at(text, at));
    let mixed = without_variables(text)
        .split(|c: char| !token(c))
        .filter(|run| run.len() >= 32)
        .any(|run| {
            run.chars().any(|c| c.is_ascii_uppercase())
                && run.chars().any(|c| c.is_ascii_lowercase())
                && run.chars().any(|c| c.is_ascii_digit())
        });
    let named: String = name
        .to_lowercase()
        .chars()
        .filter(|c| *c != '_' && *c != '-')
        .collect();
    let secret_named = SECRET_NAMES.iter().any(|secret| named.ends_with(secret))
        && !text.is_empty()
        && variables(text).is_empty();
    prefixed || private_key || web_token || mixed || secret_named
}

/// The path of the first string under `value`, the value of the key
/// `name` at `path`, that is shaped as a credential, a member by `.` and
/// an item of a list by `[n]`; none where every string passes.
fn first_credential(path: &str, name: &str, value: &Value) -> Option<String> {
    match value {
        Value::String(text) => shaped_as_credential(name, text).then(|| path.to_string()),
        Value::Array(items) => items
            .iter()
            .enumerate()
            .find_map(|(at, item)| first_credential(&format!("{path}[{at}]"), name, item)),
        Value::Object(members) => members
            .iter()
            .find_map(|(key, member)| first_credential(&format!("{path}.{key}"), key, member)),
        _ => None,
    }
}

/// Takes the key at `path`, `permissions.defaultMode` for a nested one,
/// out of `keys`, unread, the rest of the table that holds it kept;
/// whether it was there.
fn take_out(keys: &mut BTreeMap<String, Value>, path: &str) -> bool {
    match path.split_once('.') {
        None => keys.remove(path).is_some(),
        Some((top, rest)) => match keys.get_mut(top) {
            Some(Value::Object(members)) => take_out_of(members, rest),
            _ => false,
        },
    }
}

fn take_out_of(members: &mut serde_json::Map<String, Value>, path: &str) -> bool {
    match path.split_once('.') {
        None => members.remove(path).is_some(),
        Some((key, rest)) => match members.get_mut(key) {
            Some(Value::Object(inner)) => take_out_of(inner, rest),
            _ => false,
        },
    }
}

/// Whether `keys` give the key at `path`, a nested one by its path.
fn holds(keys: &BTreeMap<String, Value>, path: &str) -> bool {
    let mut parts = path.split('.');
    let Some(mut value) = parts.next().and_then(|top| keys.get(top)) else {
        return false;
    };
    for part in parts {
        match value.get(part) {
            Some(inner) => value = inner,
            None => return false,
        }
    }
    true
}

/// `entry`, `program`'s own settings as `table` gives them, with each key
/// of `reserved` left out of its named file, unread, its path kept in the
/// file's `left_out`; or why the table cannot be taken: a reserved key
/// its `settings` give, naming the key, the program, the table, the
/// layer and the key's owner, or a string shaped as a credential, naming
/// its key's path, the layer and the file where it came from one, and
/// never the value (29, 191; design 6, 7).
pub(crate) fn checked(
    program: &str,
    entry: &ProgramSettings,
    reserved: &[Reserved],
    table: &Table,
) -> Result<ProgramSettings, String> {
    let Table { layer, of } = table;
    let mut entry = entry.clone();
    if let Some(file) = entry.file.as_mut() {
        for Reserved { key, .. } in reserved {
            if take_out(&mut file.keys, key) && !file.left_out.contains(key) {
                file.left_out.push(key.clone());
            }
        }
    }
    if let Some(Reserved { key, owner }) = reserved
        .iter()
        .find(|reserved| holds(&entry.settings, &reserved.key))
    {
        return Err(format!(
            "`programs.{program}.settings.{key}`, which the layer `{layer}` gives {of}, is a key `{program}`'s agent profile reserves for {owner}: it is given there, and never among a program's own settings"
        ));
    }
    let given = entry
        .file
        .iter()
        .flat_map(|file| file.keys.iter().map(move |key| (key, Some(file))))
        .chain(entry.settings.iter().map(|key| (key, None)));
    for ((key, value), file) in given {
        let at = format!("programs.{program}.settings.{key}");
        if let Some(path) = first_credential(&at, key, value) {
            let by = file.map_or(String::new(), |file| format!(" by the file {}", file.read));
            return Err(format!(
                "`{path}`, which the layer `{layer}` gives {of}{by}, holds a value shaped as a credential, which flume never holds: a credential goes in the account's `env`, and the program reads it from its environment"
            ));
        }
    }
    Ok(entry)
}

/// One key of a table's own settings, folded: its value as written, and
/// the file it came from, none for a key the table's `settings` give.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Folded<'a> {
    pub value: &'a Value,
    pub file: Option<&'a NamedFile>,
}

/// `entry`'s keys, its `settings` over its file key by key, each key
/// taken whole (32, 191; design 2, 4).
pub(crate) fn fold(entry: &ProgramSettings) -> BTreeMap<&str, Folded<'_>> {
    let mut keys = BTreeMap::new();
    if let Some(file) = &entry.file {
        for (key, value) in &file.keys {
            keys.insert(
                key.as_str(),
                Folded {
                    value,
                    file: Some(file),
                },
            );
        }
    }
    for (key, value) in &entry.settings {
        keys.insert(key.as_str(), Folded { value, file: None });
    }
    keys
}

impl Engine {
    /// The keys the agent profile binding `program` reserves, none where
    /// no profile binds it (191; design 7).
    pub(crate) fn reserved_of(&self, program: &str) -> Vec<Reserved> {
        self.agent(program)
            .map(|agent| agent.reserved())
            .unwrap_or_default()
    }

    /// `settings` with every program's entry checked as `table`'s, each
    /// against its own profile's reserved keys, or why the table cannot
    /// be taken (191; design 7).
    pub(crate) fn own_checked(
        &self,
        settings: &Settings,
        table: &Table,
    ) -> Result<Settings, String> {
        let programs = settings
            .programs
            .iter()
            .map(|(program, entry)| {
                checked(program, entry, &self.reserved_of(program), table)
                    .map(|entry| (program.clone(), entry))
            })
            .collect::<Result<_, _>>()?;
        Ok(Settings {
            programs,
            ..settings.clone()
        })
    }

    /// A kind's table of `layer` checked, for every stage and each stage
    /// it names (191).
    pub(crate) fn own_checked_kind(
        &self,
        settings: &KindSettings,
        layer: Layer,
    ) -> Result<KindSettings, String> {
        let stages = settings
            .stages
            .iter()
            .map(|(stage, own)| {
                self.own_checked(own, &Table::stage(layer, stage))
                    .map(|own| (stage.clone(), own))
            })
            .collect::<Result<_, _>>()?;
        Ok(KindSettings {
            every_stage: self.own_checked(&settings.every_stage, &Table::every_stage(layer))?,
            stages,
        })
    }

    /// `kind` with its declaration's own settings checked, its table for
    /// every stage and each stage's own (158, 191).
    pub(crate) fn own_checked_declaration(&self, kind: &Kind) -> Result<Kind, String> {
        let mut kind = kind.clone();
        kind.agents = self.own_checked_kind(&kind.agents, Layer::Kind)?;
        for stage in &mut kind.stages {
            let table = Table::stage(Layer::Kind, &stage.name);
            stage.agents = stage
                .agents
                .iter()
                .map(|agents| self.own_checked(agents, &table))
                .collect::<Result<_, _>>()?;
        }
        Ok(kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const TWENTY: &str = "a1B2c3D4e5F6g7H8i9J0";

    fn host() -> Table {
        Table::new(Layer::Host, "for every agent")
    }

    fn reserved() -> Vec<Reserved> {
        vec![
            Reserved {
                key: "env".into(),
                owner: "the account".into(),
            },
            Reserved {
                key: "perms.mode".into(),
                owner: "flume's mode".into(),
            },
        ]
    }

    fn inline(keys: Value) -> ProgramSettings {
        ProgramSettings {
            file: None,
            settings: serde_json::from_value(keys).expect("a table"),
        }
    }

    fn by_file(keys: Value) -> ProgramSettings {
        ProgramSettings {
            file: Some(NamedFile {
                written: "agent.json".into(),
                read: "/host/flume/agent.json".into(),
                keys: serde_json::from_value(keys).expect("a table"),
                left_out: vec![],
            }),
            settings: BTreeMap::new(),
        }
    }

    #[test]
    fn each_shape_is_a_credential() {
        let shapes = [
            ("line", format!("ghp_{TWENTY}")),
            ("line", format!("run --key sk-{TWENTY}")),
            ("line", format!("AKIA{}", "IOSFODNN7EXAMPLE")),
            ("line", format!("ya29.{TWENTY}")),
            ("line", format!("github_pat_{TWENTY}")),
            ("line", format!("sk_live_{TWENTY}")),
            (
                "line",
                "-----BEGIN OPENSSH PRIVATE KEY-----\nabc\n-----END OPENSSH PRIVATE KEY-----"
                    .into(),
            ),
            (
                "line",
                "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIn0.c2lnbmF0dXJl".into(),
            ),
            (
                "line",
                "Bearer eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIn0.c2ln".into(),
            ),
            ("line", format!("x={TWENTY}{TWENTY}")),
            ("Authorization", "Bearer abc".into()),
            ("api_key", "short".into()),
            ("clientSecret", "s".into()),
            ("GITHUB_TOKEN", "t".into()),
            ("credentials", "c".into()),
        ];
        for (name, text) in &shapes {
            assert!(shaped_as_credential(name, text), "{name} = {text}");
        }
    }

    #[test]
    fn a_value_that_passes() {
        let passing = [
            ("line", "fullscreen"),
            ("line", "~/bin/line $HOME"),
            // A store path's digest is lower case, and a prefix inside a
            // word opens no token.
            (
                "command",
                "/nix/store/0c4wbxzh7m2l1nyjzcpdv0ydr3crkx2l-status-line/bin/line",
            ),
            ("command", "task-cleanup-0123456789abcdefghij"),
            ("commit", "Co-Authored-By: Agent <noreply@example.com>"),
            ("commit", "Assisted-by: an agent"),
            ("commit", ""),
            ("maxTokens", "4096"),
            // A short run, and a long one of one case.
            ("line", "ghp_short"),
            ("line", "abcdefghijklmnopqrstuvwxyz0123456789"),
        ];
        for (name, text) in &passing {
            assert!(!shaped_as_credential(name, text), "{name} = {text}");
        }
    }

    #[test]
    fn a_variable_named_rather_than_a_value() {
        assert!(!shaped_as_credential(
            "Authorization",
            "Bearer ${API_TOKEN}"
        ));
        assert!(!shaped_as_credential("token", "$AGENT_TOKEN"));
        assert!(!shaped_as_credential(
            "line",
            "run ${A_VERY_LONG_VARIABLE_NAME_FOR_THE_TOKEN_0123456789}"
        ));
        assert!(
            shaped_as_credential("Authorization", "Bearer $"),
            "a bare `$` names no variable"
        );
        assert!(
            shaped_as_credential("line", &format!("$HOME/ghp_{TWENTY}")),
            "a variable beside a token hides none of it"
        );
    }

    #[test]
    fn a_credential_is_refused_naming_its_path_and_never_its_value() {
        let token = format!("sk-{TWENTY}");
        let entry = by_file(json!({ "line": { "command": format!("run --key {token}") } }));
        let refused = checked("agent", &entry, &[], &host()).unwrap_err();
        assert!(
            refused.contains("`programs.agent.settings.line.command`"),
            "{refused}"
        );
        assert!(refused.contains("`host`"), "{refused}");
        assert!(refused.contains("/host/flume/agent.json"), "{refused}");
        assert!(refused.contains("account's `env`"), "{refused}");
        assert!(!refused.contains(TWENTY), "{refused}");
        let inline_entry = inline(json!({ "headers": { "Authorization": "Bearer abc" } }));
        let refused = checked("agent", &inline_entry, &[], &host()).unwrap_err();
        assert!(
            refused.contains("`programs.agent.settings.headers.Authorization`"),
            "{refused}"
        );
        assert!(!refused.contains("by the file"), "{refused}");
        assert!(!refused.contains("abc"), "{refused}");
        let listed = inline(json!({ "hooks": [{ "command": "a" }, { "command": token }] }));
        let refused = checked("agent", &listed, &[], &host()).unwrap_err();
        assert!(
            refused.contains("`programs.agent.settings.hooks[1].command`"),
            "{refused}"
        );
    }

    #[test]
    fn a_reserved_key_of_a_file_is_left_out() {
        let entry = by_file(json!({
            "env": { "PEOPLE": "1" },
            "perms": { "mode": "full", "allow": ["a"] },
            "line": "people",
        }));
        let taken = checked("agent", &entry, &reserved(), &host()).expect("taken");
        let file = taken.file.as_ref().expect("a file");
        assert_eq!(file.left_out, vec!["env".to_string(), "perms.mode".into()]);
        assert_eq!(
            serde_json::to_value(&file.keys).unwrap(),
            json!({ "perms": { "allow": ["a"] }, "line": "people" })
        );
        assert_eq!(
            checked("agent", &taken, &reserved(), &host()),
            Ok(taken.clone()),
            "taking it again changes nothing"
        );
    }

    #[test]
    fn a_reserved_key_inline_is_refused() {
        let entry = inline(json!({ "env": { "X": "1" } }));
        let table = Table::every_stage(Layer::Launch);
        let refused = checked("agent", &entry, &reserved(), &table).unwrap_err();
        for named in [
            "`programs.agent.settings.env`",
            "`agent`",
            "`launch`",
            "for every stage",
            "the account",
        ] {
            assert!(refused.contains(named), "{named}: {refused}");
        }
        assert!(!refused.contains("\"1\""), "{refused}");
    }

    #[test]
    fn a_nested_reserved_key() {
        let entry = inline(json!({ "perms": { "mode": "full" } }));
        let refused = checked(
            "agent",
            &entry,
            &reserved(),
            &Table::stage(Layer::Repository, "code"),
        )
        .unwrap_err();
        assert!(
            refused.contains("`programs.agent.settings.perms.mode`"),
            "{refused}"
        );
        assert!(refused.contains("for the stage `code`"), "{refused}");
        assert!(refused.contains("flume's mode"), "{refused}");
        let allowed = inline(json!({ "perms": { "allow": ["a"] } }));
        assert!(checked("agent", &allowed, &reserved(), &host()).is_ok());
        let nested = by_file(json!({ "perms": { "inner": { "mode": "x" }, "mode": "y" } }));
        let reserved = [Reserved {
            key: "perms.inner.mode".into(),
            owner: "flume's mode".into(),
        }];
        let taken = checked("agent", &nested, &reserved, &host()).expect("taken");
        let file = taken.file.expect("a file");
        assert_eq!(
            serde_json::to_value(&file.keys).unwrap(),
            json!({ "perms": { "inner": {}, "mode": "y" } })
        );
        assert_eq!(file.left_out, vec!["perms.inner.mode".to_string()]);
    }

    #[test]
    fn a_credential_under_a_key_left_out_is_never_read() {
        let entry = by_file(json!({
            "env": { "TOKEN": format!("ghp_{TWENTY}") },
            "line": "people",
        }));
        assert!(checked("agent", &entry, &[], &host()).is_err());
        let taken = checked("agent", &entry, &reserved(), &host()).expect("taken");
        let held = serde_json::to_string(&taken).unwrap();
        assert!(!held.contains(TWENTY), "{held}");
        assert!(held.contains("\"env\""), "env named in left_out: {held}");
    }

    #[test]
    fn a_tables_settings_fold_over_its_file() {
        let mut entry =
            by_file(json!({ "tui": "fullscreen", "line": "file", "hooks": { "a": 1 } }));
        entry.settings = serde_json::from_value(json!({ "line": "inline", "hooks": {} })).unwrap();
        let folded = fold(&entry);
        let file = entry.file.as_ref();
        assert_eq!(
            folded.get("tui"),
            Some(&Folded {
                value: &json!("fullscreen"),
                file
            })
        );
        assert_eq!(
            folded.get("line"),
            Some(&Folded {
                value: &json!("inline"),
                file: None
            })
        );
        assert_eq!(
            folded.get("hooks"),
            Some(&Folded {
                value: &json!({}),
                file: None
            }),
            "a key taken whole"
        );
        assert!(fold(&ProgramSettings::default()).is_empty());
    }
}
