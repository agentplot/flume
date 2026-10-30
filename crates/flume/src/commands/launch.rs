//! What a launch's words give the core (2 to 8, 32, 70, 130; design 5a,
//! 16a): each `--agent [<stage>.]<field>=<value>` one setting of every
//! stage's agent or one stage's, and each `--hold <stage>=<words>` a stage
//! on hold with its words. Nothing is resolved or refused here but words
//! that name no setting: the core refuses the rest (4).

use flume_seams::Settings;
use std::collections::BTreeMap;

/// The fields `--agent` sets (32).
const FIELDS: [&str; 4] = ["program", "model", "instructions", "beside"];

/// Sets the one setting `given` names, `[<stage>.]<field>=<value>`, in
/// `every_stage` or in its stage's entry of `stages` (7, 32).
pub fn agent(
    given: &str,
    every_stage: &mut Settings,
    stages: &mut BTreeMap<String, Settings>,
) -> Result<(), String> {
    let Some((key, value)) = given.split_once('=') else {
        return Err(format!(
            "--agent {given} gives no value: it is [<stage>.]<field>=<value>"
        ));
    };
    let (stage, field) = match key.split_once('.') {
        Some((head, _)) if FIELDS.contains(&head) => (None, key),
        Some((stage, field)) => (Some(stage), field),
        None => (None, key),
    };
    let settings = match stage {
        Some(stage) => stages.entry(stage.to_string()).or_default(),
        None => every_stage,
    };
    let value = Some(value.to_string());
    match field {
        "program" => settings.program = value,
        "model" => settings.model = value,
        "instructions" => settings.instructions = value,
        "beside" => settings.beside = value,
        _ => {
            return Err(format!(
                "--agent {given}: no setting is named {field}; the settings are program, model, instructions and beside"
            ))
        }
    }
    Ok(())
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
        ] {
            let refused = given(&[words]).unwrap_err();
            assert!(refused.contains(words), "{refused}");
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
