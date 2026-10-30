//! Claude Code's map of flume's four modes and its own values (88, 161,
//! 169, C.4): each of the four maps to the one `--permission-mode` choice
//! of 2.1.296 that means it, never a nearer one, wider or narrower; a
//! value of the program's own, given under `claude` in a table of
//! programs, is passed as written where it is one of that release's
//! choices and refused otherwise. 2.1.296 reads `manual` as its own
//! `default`, the alias its settings schema names, and reports the mode a
//! session began in as every hook input's `permission_mode`, so the mode
//! read back is `default` read as `manual` and every other choice as
//! itself. Every fragment of the release this map rests on is in
//! [`fragments`], which a test of the check-tools layer finds byte for
//! byte in the release the tree pins.

use flume_seams::{Mode, ModeEntry};

/// The option that passes the mode.
pub const FLAG: &str = "--permission-mode";

/// Every hook input's field naming the mode the session runs in.
pub const FIELD: &str = "permission_mode";

/// The value 2.1.296 reads as its own [`DEFAULT`].
pub const ALIAS: &str = "manual";

/// The choice 2.1.296 reads [`ALIAS`] as, and reports a session begun in
/// `manual` by.
pub const DEFAULT: &str = "default";

/// 2.1.296's own `--permission-mode` choices, in its list's order, its
/// `default` given by [`ALIAS`]: each passed as written.
pub const OWN: [&str; 6] = [
    "acceptEdits",
    "auto",
    "bypassPermissions",
    ALIAS,
    "dontAsk",
    "plan",
];

/// The choice of 2.1.296's own that `mode` means (169).
pub fn value(mode: Mode) -> &'static str {
    match mode {
        Mode::Plan => "plan",
        Mode::Ask => ALIAS,
        Mode::Edits => "acceptEdits",
        Mode::Full => "bypassPermissions",
    }
}

/// The value `entry` passes: one of the four by [`value`], or an own
/// value of [`OWN`] as written; any other own value refused naming it,
/// `claude` and the values it names (169, C.4).
pub fn map(entry: &ModeEntry) -> Result<&'static str, String> {
    match entry {
        ModeEntry::Flume(mode) => Ok(value(*mode)),
        ModeEntry::Own(own) => OWN
            .iter()
            .find(|value| *value == own)
            .copied()
            .ok_or_else(|| {
                let (last, rest) = OWN.split_last().expect("six values");
                format!(
                    "`{own}` is no mode of `{}`, whose own are {} and {last}",
                    super::NAME,
                    rest.join(", ")
                )
            }),
    }
}

/// The mode a hook input's `permission_mode` names, as the start's record
/// names it: [`DEFAULT`] as [`ALIAS`], and every other choice as itself
/// (161).
pub fn began(reported: &str) -> &str {
    if reported == DEFAULT {
        ALIAS
    } else {
        reported
    }
}

/// The source fragments of 2.1.296 the map rests on, each named and as
/// the release's program holds it: its list of its own choices, the alias
/// it reads `manual` by, the option that passes the mode, and every hook
/// input's field naming it.
pub fn fragments() -> Vec<(&'static str, String)> {
    let choices: Vec<String> = OWN
        .map(|value| format!("\"{}\"", if value == ALIAS { DEFAULT } else { value }))
        .to_vec();
    vec![
        (
            "the list of the release's own modes",
            format!("[{}]", choices.join(",")),
        ),
        (
            "the alias it reads manual by",
            format!("e===\"{ALIAS}\"?\"{DEFAULT}\":e"),
        ),
        (
            "the option that passes the mode",
            format!("\"{FLAG} <mode>\",\"Permission mode to use for the session\""),
        ),
        ("every hook input's mode", format!("{FIELD}:o().optional()")),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fragments_are_written_as_the_release_writes_them() {
        let fragments: Vec<String> = fragments().into_iter().map(|(_, text)| text).collect();
        assert_eq!(
            fragments,
            [
                r#"["acceptEdits","auto","bypassPermissions","default","dontAsk","plan"]"#,
                r#"e==="manual"?"default":e"#,
                r#""--permission-mode <mode>","Permission mode to use for the session""#,
                "permission_mode:o().optional()",
            ]
        );
    }

    #[test]
    fn default_is_taken_by_its_alias_alone() {
        // `default` passed would read back as `manual`, never as the value
        // its start recorded, so only its alias is taken (161).
        let words = map(&ModeEntry::Own(DEFAULT.into())).expect_err("default taken");
        assert!(words.contains("`default`"), "{words}");
        assert_eq!(map(&ModeEntry::Own(ALIAS.into())), Ok(ALIAS));
    }

    #[test]
    fn each_value_passed_reads_back_as_itself() {
        for value in OWN.into_iter().chain(Mode::ALL.map(super::value)) {
            let reported = if value == ALIAS { DEFAULT } else { value };
            assert_eq!(began(reported), value);
        }
    }
}
