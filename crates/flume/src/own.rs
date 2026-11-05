//! An agents table's `programs`, read alike from a declaration of kinds,
//! from the host's configuration and from a launch's words (191, C.2,
//! C.7; design 1, 2, 11): a table of programs by name, each entry
//! flume's, giving `file`, the path of a settings file in the program's
//! own words, `settings`, the program's own keys as written, or both,
//! each value kept as JSON. A file is read through a function its caller
//! gives, so a declaration's is read where its prompts are and a host's
//! on the host, as JSON where its name ends `.json` and as TOML where it
//! ends `.toml`, its top level a table. Nothing inside a value is read,
//! and no value is checked for a credential here: that is the core's,
//! once a program's reserved keys are out of its files (core.30,
//! core.50).

use flume_seams::{NamedFile, ProgramSettings};
use serde_json::{Map, Number, Value as Json};
use std::collections::BTreeMap;
use toml::Value;

/// What reads a settings file from its path as written: the path it was
/// read at and its text, or why it cannot be read, naming the file.
pub type Reader<'a> = &'a dyn Fn(&str) -> Result<(String, String), String>;

/// The programs `value` gives, the `programs` of the agents table at
/// `at`, each file read by `file`; or the fault, named by its key under
/// `at`, and by the file where the file is at fault.
pub fn read(
    value: &Value,
    at: &str,
    file: Reader,
) -> Result<BTreeMap<String, ProgramSettings>, String> {
    let at = format!("{at}.programs");
    let Value::Table(programs) = value else {
        return Err(format!(
            "{at}: must be a table of programs, each entry giving file, settings or both"
        ));
    };
    let mut read = BTreeMap::new();
    for (program, entry) in programs {
        let at = format!("{at}.{}", key(program));
        let Value::Table(entry) = entry else {
            return Err(format!(
                "{at}: must be a table giving file, settings or both"
            ));
        };
        let mut own = ProgramSettings::default();
        let entry_at = at;
        for (name, given) in entry {
            let at = format!("{entry_at}.{}", key(name));
            match (name.as_str(), given) {
                ("file", Value::String(written)) => {
                    own.file =
                        Some(named_file(written, file).map_err(|why| format!("{at}: {why}"))?)
                }
                ("file", _) => return Err(format!("{at}: must be the path of a settings file")),
                ("settings", Value::Table(settings)) => {
                    for (name, given) in settings {
                        let at = format!("{at}.{}", key(name));
                        own.settings.insert(name.clone(), json(given, &at)?);
                    }
                }
                ("settings", _) => {
                    return Err(format!(
                        "{at}: must be a table of the program's own settings"
                    ))
                }
                _ => {
                    return Err(format!(
                        "{at}: is no word of a program's entry, which gives file and settings: \
                         a program's own settings go under settings, as {entry_at}.settings.{}",
                        key(name)
                    ))
                }
            }
        }
        read.insert(program.clone(), own);
    }
    Ok(read)
}

/// The settings file at `written`, read by `file`: as JSON where its name
/// ends `.json` and as TOML where it ends `.toml`, its top level a table
/// whose keys are kept as read; or why it cannot be taken, naming the
/// file (design 2).
pub fn named_file(written: &str, file: Reader) -> Result<NamedFile, String> {
    let json_named = written.ends_with(".json");
    if !json_named && !written.ends_with(".toml") {
        return Err(format!(
            "{written} is named neither .json nor .toml, the two a settings file is read as"
        ));
    }
    let (read, text) = file(written)?;
    let keys = if json_named {
        match serde_json::from_str::<Json>(&text) {
            Ok(Json::Object(keys)) => keys.into_iter().collect(),
            Ok(_) => return Err(format!("{read}'s top level is not a table of settings")),
            Err(error) => return Err(format!("{read} does not parse as JSON: {error}")),
        }
    } else {
        let table: toml::Table = text.parse().map_err(|error: toml::de::Error| {
            format!(
                "{read} does not parse as TOML: {}",
                error.message().trim_end()
            )
        })?;
        let mut keys = BTreeMap::new();
        for (name, given) in &table {
            let value = json(given, &key(name)).map_err(|why| format!("{read}'s key {why}"))?;
            keys.insert(name.clone(), value);
        }
        keys
    };
    Ok(NamedFile {
        written: written.to_string(),
        read,
        keys,
        left_out: vec![],
    })
}

/// `value`, at `at`, as JSON holds it, one for one; a date or time, which
/// JSON cannot hold, and a float JSON has no number for, refused naming
/// its key (design 1).
fn json(value: &Value, at: &str) -> Result<Json, String> {
    Ok(match value {
        Value::String(text) => Json::String(text.clone()),
        Value::Integer(number) => Json::Number(Number::from(*number)),
        Value::Float(number) => Number::from_f64(*number)
            .map(Json::Number)
            .ok_or_else(|| format!("{at}: is {number}, which JSON has no number for"))?,
        Value::Boolean(truth) => Json::Bool(*truth),
        Value::Datetime(_) => {
            return Err(format!(
                "{at}: is a date or time, which flume cannot pass on: give it as a string"
            ))
        }
        Value::Array(items) => Json::Array(
            items
                .iter()
                .enumerate()
                .map(|(n, item)| json(item, &format!("{at}[{n}]")))
                .collect::<Result<_, _>>()?,
        ),
        Value::Table(table) => {
            let mut object = Map::new();
            for (name, given) in table {
                object.insert(name.clone(), json(given, &format!("{at}.{}", key(name)))?);
            }
            Json::Object(object)
        }
    })
}

/// A key as TOML writes it in a dotted key.
fn key(name: &str) -> String {
    if !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        name.to_string()
    } else {
        format!("{name:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// A reader of the files `files` holds by their paths as written,
    /// each read at `read/<path>`.
    fn files(files: &[(&str, &str)]) -> impl Fn(&str) -> Result<(String, String), String> {
        let files: BTreeMap<String, String> = files
            .iter()
            .map(|(path, text)| (path.to_string(), text.to_string()))
            .collect();
        move |path| {
            files
                .get(path)
                .map(|text| (format!("read/{path}"), text.clone()))
                .ok_or_else(|| format!("{path} is not there"))
        }
    }

    fn programs(text: &str, reader: Reader) -> Result<BTreeMap<String, ProgramSettings>, String> {
        let table: toml::Table = text.parse().unwrap();
        read(&table["programs"], "agents", reader)
    }

    #[test]
    fn inline_settings_kept_as_json_values() {
        let read = programs(
            r#"programs.agent.settings = { tui = "fullscreen", count = 3, ratio = 0.5, on = true, list = ["a", 1], statusLine = { type = "command", command = "~/bin/line $HOME" } }"#,
            &files(&[]),
        )
        .unwrap();
        let agent = &read["agent"];
        assert_eq!(agent.file, None);
        assert_eq!(
            Json::Object(agent.settings.clone().into_iter().collect()),
            json!({
                "tui": "fullscreen",
                "count": 3,
                "ratio": 0.5,
                "on": true,
                "list": ["a", 1],
                "statusLine": { "type": "command", "command": "~/bin/line $HOME" },
            })
        );
    }

    #[test]
    fn a_json_file_and_a_toml_file_beside_inline_keys() {
        let reader = files(&[
            (
                "agent.json",
                r#"{ "tui": "fullscreen", "hooks": { "Stop": [] } }"#,
            ),
            (
                "other.toml",
                "model_reasoning = \"high\"\n[tui]\nnotifications = true\n",
            ),
        ]);
        let read = programs(
            r#"
            [programs.agent]
            file = "agent.json"
            settings = { tui = "default" }
            [programs.other]
            file = "other.toml"
            "#,
            &reader,
        )
        .unwrap();
        let agent = &read["agent"];
        assert_eq!(
            agent.file,
            Some(NamedFile {
                written: "agent.json".into(),
                read: "read/agent.json".into(),
                keys: BTreeMap::from([
                    ("hooks".into(), json!({ "Stop": [] })),
                    ("tui".into(), json!("fullscreen")),
                ]),
                left_out: vec![],
            })
        );
        assert_eq!(agent.settings["tui"], json!("default"));
        let other = &read["other"];
        assert!(other.settings.is_empty());
        let file = other.file.as_ref().unwrap();
        assert_eq!(file.keys["model_reasoning"], json!("high"));
        assert_eq!(file.keys["tui"], json!({ "notifications": true }));
    }

    #[test]
    fn a_file_that_gives_no_key_is_kept() {
        let read = programs(
            r#"programs.agent.file = "empty.json""#,
            &files(&[("empty.json", "{}")]),
        )
        .unwrap();
        let file = read["agent"].file.as_ref().unwrap();
        assert!(file.keys.is_empty());
        assert_eq!(file.read, "read/empty.json");
    }

    #[test]
    fn a_shape_flume_cannot_take_is_refused_naming_its_key() {
        for (text, named, words) in [
            (
                r#"programs = "agent""#,
                "agents.programs:",
                "table of programs",
            ),
            (
                "programs.agent = 3",
                "agents.programs.agent:",
                "file, settings or both",
            ),
            (
                r#"programs = { agent = "x" }"#,
                "agents.programs.agent:",
                "file, settings or both",
            ),
            (
                r#"programs.agent.tui = "fullscreen""#,
                "agents.programs.agent.tui:",
                "a program's own settings go under settings, as agents.programs.agent.settings.tui",
            ),
            (
                "programs.agent.file = 3",
                "agents.programs.agent.file:",
                "path of a settings file",
            ),
            (
                r#"programs.agent.settings = "x""#,
                "agents.programs.agent.settings:",
                "own settings",
            ),
            (
                "programs.agent.settings.since = 2026-11-01",
                "agents.programs.agent.settings.since:",
                "give it as a string",
            ),
            (
                "programs.agent.settings.at = { when = 07:32:00 }",
                "agents.programs.agent.settings.at.when:",
                "give it as a string",
            ),
            (
                "programs.agent.settings.list = [1, 2026-11-01T07:32:00Z]",
                "agents.programs.agent.settings.list[1]:",
                "date or time",
            ),
            (
                "programs.agent.settings.n = nan",
                "agents.programs.agent.settings.n:",
                "no number",
            ),
            (
                r#"programs."my agent".settings."a b" = 1979-05-27"#,
                r#"agents.programs."my agent".settings."a b":"#,
                "date or time",
            ),
        ] {
            let refused = programs(text, &files(&[])).unwrap_err();
            assert!(
                refused.contains(named) && refused.contains(words),
                "{text}: {refused}"
            );
        }
    }

    #[test]
    fn a_file_flume_cannot_take_is_refused_naming_the_field_and_the_file() {
        let reader = files(&[
            ("broken.json", "{ not json"),
            ("broken.toml", "= nope"),
            ("list.json", "[1, 2]"),
            ("text.json", "\"fullscreen\""),
            ("dated.toml", "since = 2026-11-01"),
        ]);
        for (path, words) in [
            (
                "settings.yaml",
                "settings.yaml is named neither .json nor .toml",
            ),
            ("settings", "named neither .json nor .toml"),
            ("missing.json", "missing.json is not there"),
            ("broken.json", "read/broken.json does not parse as JSON"),
            ("broken.toml", "read/broken.toml does not parse as TOML"),
            ("list.json", "read/list.json's top level is not a table"),
            ("text.json", "read/text.json's top level is not a table"),
            (
                "dated.toml",
                "read/dated.toml's key since: is a date or time",
            ),
        ] {
            let refused =
                programs(&format!("programs.agent.file = {path:?}"), &reader).unwrap_err();
            assert!(
                refused.starts_with("agents.programs.agent.file: ") && refused.contains(words),
                "{path}: {refused}"
            );
        }
    }

    #[test]
    fn no_value_is_read_or_checked() {
        // design 7: the credential check is the core's, after the reserved
        // keys are out; the reader takes every value as written.
        let token = format!("ghp_{}", "a1B2c3D4e5F6g7H8i9J0");
        let read = programs(
            &format!(
                "programs.agent.settings = {{ env = {{ TOKEN = {token:?} }}, line = \"$HOME\" }}"
            ),
            &files(&[]),
        )
        .unwrap();
        assert_eq!(read["agent"].settings["env"], json!({ "TOKEN": token }));
        assert_eq!(read["agent"].settings["line"], json!("$HOME"));
    }
}
