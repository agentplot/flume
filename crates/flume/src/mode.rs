//! An agents table's `mode`, read alike from a declaration of kinds and
//! from the host's configuration, each TOML (162, 169, C.2, C.7; design
//! 2): a word, one of flume's four, or a table of programs whose entries
//! are strings, each kept as written, since only the agent's profile can
//! say whether a value of its program's own is one it names (169).

use flume_seams::ModeSetting;
use std::collections::BTreeMap;
use toml::Value;

/// The mode `value` gives, or words saying why it gives none, which the
/// caller puts after the key it read.
pub fn read(value: &Value) -> Result<ModeSetting, String> {
    match value {
        Value::String(word) => ModeSetting::from_word(word),
        Value::Table(table) => {
            let mut programs = BTreeMap::new();
            for (program, entry) in table {
                let Value::String(entry) = entry else {
                    return Err(format!("the entry for `{program}` must be a string"));
                };
                programs.insert(program.clone(), entry.clone());
            }
            ModeSetting::from_programs(programs)
        }
        _ => Err(
            "must be one of flume's four, plan, ask, edits and full, or a table of programs".into(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flume_seams::Mode;

    fn mode(text: &str) -> Result<ModeSetting, String> {
        let table: toml::Table = format!("mode = {text}").parse().unwrap();
        read(&table["mode"])
    }

    #[test]
    fn one_of_the_four_or_a_table_of_programs() {
        assert_eq!(mode("\"edits\""), Ok(ModeSetting::Flume(Mode::Edits)));
        assert_eq!(
            mode("{ agent = \"careful\", other = \"edits\" }"),
            Ok(ModeSetting::Programs(BTreeMap::from([
                ("agent".to_string(), "careful".to_string()),
                ("other".to_string(), "edits".to_string()),
            ])))
        );
    }

    #[test]
    fn a_mode_that_is_neither_is_refused_naming_why() {
        for (text, named) in [
            ("\"careful\"", "`careful`"),
            ("[\"edits\"]", "table of programs"),
            ("3", "table of programs"),
            ("{}", "names no program"),
            ("{ agent = 3 }", "`agent`"),
        ] {
            let refused = mode(text).unwrap_err();
            assert!(refused.contains(named), "{text}: {refused}");
        }
    }
}
