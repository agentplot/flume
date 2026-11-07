//! What a launch's words give the core (2 to 8, 32, 70, 107, 130, 162,
//! 195, 197; design 5a, 16a): each `--agent [<stage>.]<field>=<value>` one
//! setting of every stage's agent or one stage's,
//! `[<stage>.]mode.<program>=<value>` one entry of its mode's table of
//! programs, `[<stage>.]programs.<program>.settings.<key>=<value>` one of a
//! program's own settings and `[<stage>.]programs.<program>.file=<path>`
//! its settings file, read now, and each `--hold <stage>=<words>` a stage
//! on hold with its words. Nothing is resolved or refused here but words
//! that name no setting, a mode none of flume's four, a mode given both
//! as a word and as a table, one of a program's own settings or its file
//! given twice for one stage, and a file flume cannot read: the core
//! refuses the rest, a value shaped as a credential among it (4, 169,
//! 195).

use flume_seams::{ModeSetting, Settings};
use serde_json::Value as Json;
use std::collections::BTreeMap;
use std::path::Path;

/// The fields `--agent` sets (32, 107, 162, 195, 197).
const FIELDS: [&str; 8] = [
    "program",
    "model",
    "effort",
    "instructions",
    "beside",
    "session",
    "mode",
    "programs",
];

/// The settings `--agent` gives, as its refusals name them.
const SETTINGS: &str =
    "the settings are program, model, effort, instructions, beside, session and mode, or mode.<program> for one program's entry of a mode, and programs, programs.<program>.settings.<key> for one of a program's own settings and programs.<program>.file for its settings file";

/// Sets the one setting `given` names, `[<stage>.]<field>=<value>`, in
/// `every_stage` or in its stage's entry of `stages` (7, 32); a mode's
/// entries for programs build one table, which a mode given as a word
/// refuses, and the word it (169); a program's own settings build one
/// entry, its file read now, relative to `directory`, the command's
/// working directory (21, 195).
pub fn agent(
    given: &str,
    directory: &Path,
    every_stage: &mut Settings,
    stages: &mut BTreeMap<String, Settings>,
) -> Result<(), String> {
    let Some((key, value)) = given.split_once('=') else {
        return Err(format!(
            "--agent {given} gives no value: it is [<stage>.]<field>=<value>; {SETTINGS}"
        ));
    };
    let (stage, field) = match key.split_once('.') {
        Some((head, _)) if FIELDS.contains(&head) => (None, key),
        Some((stage, field)) => (Some(stage), field),
        None => (None, key),
    };
    let prefix = stage.map(|stage| format!("{stage}.")).unwrap_or_default();
    let settings = match stage {
        Some(stage) => stages.entry(stage.to_string()).or_default(),
        None => every_stage,
    };
    let text = Some(value.to_string());
    match field {
        "program" => settings.program = text,
        "model" => settings.model = text,
        "effort" => settings.effort = text,
        "instructions" => settings.instructions = text,
        "beside" => settings.beside = text,
        "session" => settings.session = text,
        "mode" => {
            let word = ModeSetting::from_word(value).map_err(|reason| {
                format!(
                    "--agent {given}: {reason}; on the command line, --agent {prefix}mode.<program>={value}"
                )
            })?;
            if matches!(settings.mode, Some(ModeSetting::Programs(_))) {
                return Err(both(given, &prefix, stage, settings));
            }
            settings.mode = Some(word);
        }
        _ if field.starts_with("programs.") => {
            let whose = match stage {
                Some(stage) => format!("the stage {stage}"),
                None => "every stage".to_string(),
            };
            own(given, field, value, directory, &prefix, &whose, settings)?;
        }
        _ => match field.strip_prefix("mode.") {
            Some("") => {
                return Err(format!(
                    "--agent {given} names no program: it is [<stage>.]mode.<program>=<value>"
                ))
            }
            Some(program) => {
                let mut programs = match &settings.mode {
                    None => BTreeMap::new(),
                    Some(ModeSetting::Programs(programs)) => programs.clone(),
                    Some(ModeSetting::Flume(_)) => {
                        return Err(both(given, &prefix, stage, settings))
                    }
                };
                programs.insert(program.to_string(), value.to_string());
                settings.mode = Some(ModeSetting::from_programs(programs)?);
            }
            None => {
                return Err(format!(
                    "--agent {given}: no setting is named {field}; {SETTINGS}"
                ))
            }
        },
    }
    Ok(())
}

/// Sets in `settings` what `field`, `programs.<program>.settings.<key>` or
/// `programs.<program>.file`, gives: one of the program's own settings, a
/// top-level key whose `value` is read as JSON where it parses as JSON and
/// as the text written otherwise, or its settings file, made absolute
/// against `directory` and read now by flume's one reader of them; one
/// given twice for `whose` refused naming it (7, 21, 195).
fn own(
    given: &str,
    field: &str,
    value: &str,
    directory: &Path,
    prefix: &str,
    whose: &str,
    settings: &mut Settings,
) -> Result<(), String> {
    let forms = "it is [<stage>.]programs.<program>.settings.<key>=<value> or [<stage>.]programs.<program>.file=<path>";
    let Some((program, part)) = field
        .strip_prefix("programs.")
        .and_then(|entry| entry.split_once('.'))
        .filter(|(program, _)| !program.is_empty())
    else {
        return Err(format!("--agent {given} names no program: {forms}"));
    };
    let entry = settings.programs.entry(program.to_string()).or_default();
    let twice = |named: &str| {
        format!("--agent {given} gives {prefix}{named} a second time for {whose}: give it once")
    };
    if part == "file" {
        if entry.file.is_some() {
            return Err(twice(field));
        }
        let path = directory.join(value).to_string_lossy().into_owned();
        let file = crate::own::named_file(&path, &|path| {
            std::fs::read_to_string(path)
                .map(|text| (path.to_string(), text))
                .map_err(|error| format!("{path} could not be read: {error}"))
        })
        .map_err(|why| format!("--agent {given}: {why}"))?;
        entry.file = Some(file);
        return Ok(());
    }
    match part.strip_prefix("settings.") {
        Some(key) if !key.is_empty() && !key.contains('.') => {
            if entry.settings.contains_key(key) {
                return Err(twice(field));
            }
            let value = serde_json::from_str(value).unwrap_or_else(|_| Json::String(value.into()));
            entry.settings.insert(key.to_string(), value);
            Ok(())
        }
        Some(key) if !key.is_empty() => {
            let top = key.split('.').next().unwrap_or(key);
            Err(format!(
                "--agent {given}: a program's own setting is given whole by its top-level key, as --agent {prefix}programs.{program}.settings.{top}=<its value as JSON>"
            ))
        }
        _ => Err(format!(
            "--agent {given}: no setting is named {field}: {forms}; {SETTINGS}"
        )),
    }
}

/// The refusal of `given` where `settings` already hold a mode in the
/// other form, naming both: a mode is one word or one table (169).
fn both(given: &str, prefix: &str, stage: Option<&str>, settings: &Settings) -> String {
    let mut words: Vec<String> = match &settings.mode {
        Some(ModeSetting::Flume(mode)) => vec![format!("--agent {prefix}mode={mode}")],
        Some(ModeSetting::Programs(programs)) => programs
            .iter()
            .map(|(program, value)| format!("--agent {prefix}mode.{program}={value}"))
            .collect(),
        None => vec![],
    };
    words.push(format!("--agent {given}"));
    let last = words.pop().unwrap_or_default();
    let whose = match stage {
        Some(stage) => format!("the stage {stage}'s"),
        None => "every stage's".to_string(),
    };
    format!(
        "{} and {last} give {whose} mode both as a word and as a table of programs: a mode is one of flume's four or one table of programs, never both",
        words.join(", ")
    )
}

/// A stage on hold, `<stage>=<words>` or `<stage>` alone, with its words
/// where it gives some (7, 130).
pub fn hold(given: &str) -> Result<(String, Option<String>), String> {
    let (stage, words) = match given.split_once('=') {
        Some((stage, words)) => (stage, Some(words.to_string())),
        None => (given, None),
    };
    if stage.is_empty() {
        return Err(format!(
            "--hold {given} names no stage: it is <stage>=<words>"
        ));
    }
    Ok((stage.to_string(), words.filter(|words| !words.is_empty())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use flume_seams::Mode;

    fn given(words: &[&str]) -> Result<(Settings, BTreeMap<String, Settings>), String> {
        given_in(Path::new("/nowhere"), words)
    }

    fn given_in(
        directory: &Path,
        words: &[&str],
    ) -> Result<(Settings, BTreeMap<String, Settings>), String> {
        let mut every = Settings::default();
        let mut stages = BTreeMap::new();
        for word in words {
            agent(word, directory, &mut every, &mut stages)?;
        }
        Ok((every, stages))
    }

    #[test]
    fn a_setting_for_every_stage_and_one_for_a_stage() {
        let (every, stages) = given(&[
            "model=opus",
            "verify.model=sonnet",
            "code.instructions=a=b",
            "instructions=be brief",
        ])
        .unwrap();
        assert_eq!(every.model.as_deref(), Some("opus"));
        assert_eq!(every.instructions.as_deref(), Some("be brief"));
        assert_eq!(stages["verify"].model.as_deref(), Some("sonnet"));
        assert_eq!(stages["code"].instructions.as_deref(), Some("a=b"));
    }

    #[test]
    fn an_effort_for_every_stage_and_for_one() {
        // 7, 32, 197: `effort=high` for every stage and `verify.effort=max`
        // for one, each its program's own word kept as written.
        let (every, stages) = given(&["effort=high", "verify.effort=max"]).unwrap();
        assert_eq!(every.effort.as_deref(), Some("high"));
        assert_eq!(stages["verify"].effort.as_deref(), Some("max"));
        assert_eq!(stages["verify"].model, None);
    }

    #[test]
    fn a_refusal_names_the_effort_among_the_settings() {
        // 7, 32: words naming no setting are refused naming the words, and
        // the settings `--agent` gives, `effort` among them.
        let refused = given(&["colour=blue"]).unwrap_err();
        assert!(refused.contains("colour"), "{refused}");
        assert!(
            refused.contains("program, model, effort, instructions"),
            "{refused}"
        );
    }

    #[test]
    fn words_naming_no_setting_are_refused() {
        for words in [
            "colour=red",
            "verify.colour=red",
            "env.A=1",
            "code.env.B=1",
            "model",
            "mode.=edits",
        ] {
            let refused = given(&[words]).unwrap_err();
            assert!(refused.contains(words), "{refused}");
        }
        for words in ["colour=red", "model"] {
            let refused = given(&[words]).unwrap_err();
            assert!(
                refused.contains("beside, session and mode") && refused.contains("mode.<program>"),
                "{refused}"
            );
        }
    }

    #[test]
    fn a_session_for_every_stage_and_one_for_a_stage() {
        // 7, 32, 107: `session` as `beside` is, at the launch's layer.
        let (every, stages) = given(&["session=review", "verify.session=late"]).unwrap();
        assert_eq!(every.session.as_deref(), Some("review"));
        assert_eq!(stages["verify"].session.as_deref(), Some("late"));
        assert_eq!(stages["verify"].beside, None);
    }

    #[test]
    fn a_mode_and_a_table_at_the_launch() {
        // 7, 32, 162: one of the four for every stage, and one stage's
        // table built from its entries.
        let (every, stages) = given(&[
            "mode=edits",
            "code.mode.agent=careful",
            "code.mode.other=ask",
        ])
        .unwrap();
        assert_eq!(every.mode, Some(ModeSetting::Flume(Mode::Edits)));
        assert_eq!(
            stages["code"].mode,
            Some(ModeSetting::Programs(BTreeMap::from([
                ("agent".to_string(), "careful".to_string()),
                ("other".to_string(), "ask".to_string()),
            ])))
        );
        assert_eq!(
            stages["code"].mode.as_ref().unwrap().written(),
            "{ agent = \"careful\", other = \"ask\" }"
        );
    }

    #[test]
    fn a_word_and_a_table_for_one_stage_are_refused_naming_both() {
        // 169: a mode is one word or one table, for a stage or for every
        // stage, in either order.
        for (words, whose) in [
            (["code.mode=edits", "code.mode.agent=careful"], "code's"),
            (["code.mode.agent=careful", "code.mode=edits"], "code's"),
            (["mode=edits", "mode.agent=careful"], "every stage's"),
            (["mode.agent=careful", "mode=edits"], "every stage's"),
        ] {
            let refused = given(&words).unwrap_err();
            for word in words {
                assert!(refused.contains(word), "{words:?}: {refused}");
            }
            assert!(refused.contains(whose), "{words:?}: {refused}");
        }
        let (every, stages) = given(&["mode=edits", "code.mode.agent=careful"]).unwrap();
        assert!(every.mode.is_some() && stages["code"].mode.is_some());
    }

    #[test]
    fn a_programs_own_value_given_bare_is_refused_naming_it() {
        // 169: a value none of the four goes under its program's name.
        for (words, under) in [
            ("mode=careful", "--agent mode.<program>=careful"),
            ("code.mode=careful", "--agent code.mode.<program>=careful"),
        ] {
            let refused = given(&[words]).unwrap_err();
            assert!(
                refused.contains("`careful`")
                    && refused.contains("its program's name")
                    && refused.contains(under),
                "{refused}"
            );
        }
    }

    #[test]
    fn own_settings_at_the_launch() {
        // 7, 32, 195: one key for every stage and one for a stage, each
        // read as JSON where it parses and as the text written otherwise,
        // several building one entry.
        let (every, stages) = given(&[
            r#"programs.agent.settings.statusLine={"type":"command","command":"line"}"#,
            "programs.agent.settings.count=3",
            r#"programs.agent.settings.quoted="3""#,
            "code.programs.agent.settings.tui=fullscreen",
            "code.programs.other.settings.tui=",
        ])
        .unwrap();
        let agent = &every.programs["agent"];
        assert_eq!(agent.file, None);
        assert_eq!(
            agent.settings,
            BTreeMap::from([
                (
                    "statusLine".to_string(),
                    serde_json::json!({ "type": "command", "command": "line" })
                ),
                ("count".to_string(), serde_json::json!(3)),
                ("quoted".to_string(), serde_json::json!("3")),
            ])
        );
        let code = &stages["code"].programs;
        assert_eq!(
            code["agent"].settings["tui"],
            serde_json::json!("fullscreen")
        );
        assert_eq!(code["other"].settings["tui"], serde_json::json!(""));
        assert!(!every.programs.contains_key("other"));
    }

    #[test]
    fn a_key_given_twice_is_refused_naming_it() {
        // 7: one key, or a file, given twice for one stage, or twice for
        // every stage; once for every stage and once for a stage is not
        // twice.
        for (words, named, whose) in [
            (
                [
                    "programs.agent.settings.tui=fullscreen",
                    "programs.agent.settings.tui=default",
                ],
                "programs.agent.settings.tui a second time",
                "every stage",
            ),
            (
                [
                    "code.programs.agent.settings.tui=fullscreen",
                    "code.programs.agent.settings.tui=default",
                ],
                "code.programs.agent.settings.tui a second time",
                "the stage code",
            ),
            (
                ["programs.agent.file=/a.json", "programs.agent.file=/b.json"],
                "programs.agent.file a second time",
                "every stage",
            ),
        ] {
            let scratch = Scratch::new("twice");
            scratch.write("a.json", "{}");
            let words = words.map(|word| word.replace("=/", &format!("={}/", scratch.0.display())));
            let words: Vec<&str> = words.iter().map(String::as_str).collect();
            let refused = given_in(&scratch.0, &words).unwrap_err();
            assert!(
                refused.contains(named) && refused.contains(whose),
                "{words:?}: {refused}"
            );
        }
        let (every, stages) = given(&[
            "programs.agent.settings.tui=fullscreen",
            "code.programs.agent.settings.tui=default",
        ])
        .unwrap();
        assert_eq!(
            every.programs["agent"].settings["tui"],
            serde_json::json!("fullscreen")
        );
        assert_eq!(
            stages["code"].programs["agent"].settings["tui"],
            serde_json::json!("default")
        );
    }

    #[test]
    fn a_file_at_the_launch() {
        // 7, 21, 195: named relative to the working directory or
        // absolutely, read now, and kept by its absolute path with its
        // keys as read; one flume cannot read refused naming it.
        let scratch = Scratch::new("file");
        let file = scratch.write("agent.json", r#"{ "tui": "fullscreen" }"#);
        scratch.write("other.toml", "tui = \"default\"\n");
        let absolute = file.display().to_string();
        let (every, stages) = given_in(
            &scratch.0,
            &[
                "programs.agent.file=agent.json",
                &format!(
                    "code.programs.other.file={}",
                    scratch.0.join("other.toml").display()
                ),
            ],
        )
        .unwrap();
        let named = every.programs["agent"].file.clone().expect("the file");
        assert_eq!(named.written, absolute);
        assert_eq!(named.read, absolute);
        assert_eq!(named.keys["tui"], serde_json::json!("fullscreen"));
        assert_eq!(
            stages["code"].programs["other"].file.as_ref().unwrap().keys["tui"],
            serde_json::json!("default")
        );
        for (word, named) in [
            (
                "programs.agent.file=agent.yaml",
                "agent.yaml is named neither .json nor .toml",
            ),
            (
                "programs.agent.file=gone.json",
                "gone.json could not be read",
            ),
        ] {
            let refused = given_in(&scratch.0, &[word]).unwrap_err();
            assert!(
                refused.starts_with(&format!("--agent {word}: ")) && refused.contains(named),
                "{refused}"
            );
        }
    }

    #[test]
    fn words_naming_no_own_setting_are_refused_naming_programs() {
        // 195, C.2: programs is named among the settings.
        for (words, named) in [
            ("programs=x", "no setting is named programs;"),
            ("programs.agent=x", "names no program"),
            ("programs..file=x", "names no program"),
            (
                "programs.agent.tui=x",
                "no setting is named programs.agent.tui",
            ),
            ("code.programs.agent.settings.=x", "no setting is named"),
            (
                "programs.agent.settings.permissions.allow=[]",
                "--agent programs.agent.settings.permissions=<its value as JSON>",
            ),
        ] {
            let refused = given(&[words]).unwrap_err();
            assert!(refused.contains(named), "{words}: {refused}");
        }
        for words in ["colour=red", "programs.agent.tui=x"] {
            let refused = given(&[words]).unwrap_err();
            assert!(
                refused.contains("and programs, programs.<program>.settings.<key>"),
                "{refused}"
            );
        }
    }

    /// A scratch directory of the test's own, removed when dropped.
    struct Scratch(std::path::PathBuf);

    impl Scratch {
        fn new(label: &str) -> Scratch {
            let dir =
                std::env::temp_dir().join(format!("flume-launch-{label}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("the scratch directory");
            Scratch(dir)
        }

        fn write(&self, name: &str, text: &str) -> std::path::PathBuf {
            let path = self.0.join(name);
            std::fs::write(&path, text).expect("the file");
            path
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn a_hold_with_its_words_or_none() {
        assert_eq!(
            hold("verify=after review").unwrap(),
            ("verify".into(), Some("after review".into()))
        );
        assert_eq!(hold("code").unwrap(), ("code".into(), None));
        assert!(hold("=words").is_err());
    }
}
