//! What a launch's words give the core (2 to 8, 32, 70, 130, 162; design
//! 5a, 16a): each `--agent [<stage>.]<field>=<value>` one setting of
//! every stage's agent or one stage's, `[<stage>.]mode.<program>=<value>`
//! one entry of its mode's table of programs, and each `--hold
//! <stage>=<words>` a stage on hold with its words. Nothing is resolved
//! or refused here but words that name no setting, a mode none of
//! flume's four, and a mode given both as a word and as a table: the
//! core refuses the rest (4, 169).

use flume_seams::{ModeSetting, Settings};
use std::collections::BTreeMap;

/// The fields `--agent` sets (32, 162).
const FIELDS: [&str; 5] = ["program", "model", "instructions", "beside", "mode"];

/// The settings `--agent` gives, as its refusals name them.
const SETTINGS: &str =
    "the settings are program, model, instructions, beside and mode, or mode.<program> for one program's entry of a mode";

/// Sets the one setting `given` names, `[<stage>.]<field>=<value>`, in
/// `every_stage` or in its stage's entry of `stages` (7, 32); a mode's
/// entries for programs build one table, which a mode given as a word
/// refuses, and the word it (169).
pub fn agent(
    given: &str,
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
        "instructions" => settings.instructions = text,
        "beside" => settings.beside = text,
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
        let mut every = Settings::default();
        let mut stages = BTreeMap::new();
        for word in words {
            agent(word, &mut every, &mut stages)?;
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
                refused.contains("beside and mode") && refused.contains("mode.<program>"),
                "{refused}"
            );
        }
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
    fn a_hold_with_its_words_or_none() {
        assert_eq!(
            hold("verify=after review").unwrap(),
            ("verify".into(), Some("after review".into()))
        );
        assert_eq!(hold("code").unwrap(), ("code".into(), None));
        assert!(hold("=words").is_err());
    }
}
