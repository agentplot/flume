//! Claude Code's effort (88, 197, C.4; design 5 to 7 of record 0044's
//! change): the five words 2.1.296's `--effort` takes, each passed as
//! written after `--model`, and any other refused naming it, `claude` and
//! the five, `med` and `ultracode` among them, which the release reads as
//! other words, and every word it does not know, which it ignores with a
//! warning and starts at its default. The release reads
//! [`VARIABLE`] over `--effort`, a level an account's `env` may give,
//! which status names by the variable alone. Every fragment of the
//! release this rests on is in [`fragments`], which a test of the
//! check-tools layer finds byte for byte in the release the tree pins.

/// The option that passes the effort.
pub const FLAG: &str = "--effort";

/// 2.1.296's own `--effort` words, in its list's order: each passed as
/// written.
pub const WORDS: [&str; 5] = ["low", "medium", "high", "xhigh", "max"];

/// The variable 2.1.296 reads over `--effort`, ahead of the turn's and
/// the session's levels.
pub const VARIABLE: &str = "CLAUDE_CODE_EFFORT_LEVEL";

/// `word` taken where it is one of [`WORDS`]; any other refused naming
/// it, `claude` and the five (197, C.4).
pub fn take(word: &str) -> Result<(), String> {
    if WORDS.contains(&word) {
        return Ok(());
    }
    let (last, rest) = WORDS.split_last().expect("five words");
    Err(format!(
        "`{word}` is no effort {} takes: {} or {last}",
        super::NAME,
        rest.join(", ")
    ))
}

/// The source fragments of 2.1.296 the effort rests on, each named and as
/// the release's program holds it: the option, its help naming the
/// words of the release's list, which its minifier names `Ia`; that list;
/// the reading of [`VARIABLE`], `auto` and `unset` read as none; and the
/// resolution of a session's level, which takes the hook's first, then
/// the variable's, the turn's and the session's, which `--effort` gives,
/// in that order.
pub fn fragments() -> Vec<(&'static str, String)> {
    let words: Vec<String> = WORDS.map(|word| format!("\"{word}\"")).to_vec();
    vec![
        (
            "the option that passes the effort, its help naming the list's words",
            format!("\"{FLAG} <level>\",`Effort level for the current session (${{Ia.join(\", \")}})`"),
        ),
        (
            "the list of the release's efforts",
            format!("Ia=[{}]", words.join(",")),
        ),
        (
            "the variable read as a level, auto and unset as none",
            format!("function Y0(){{let e=a.{VARIABLE};return e?.toLowerCase()===\"unset\"||e?.toLowerCase()===\"auto\"?null:z1(e)}}"),
        ),
        (
            "the resolution of a level, the session's its second argument and the turn's and the hook's named",
            "function kE(e,n,{turnEffort:r,hookEffortValue:s,carriedEffort:f=O(e)}={})".into(),
        ),
        (
            "the variable's level taken over the turn's and the session's",
            "let u=Y0();if(u===null&&!d)return;let S=INn(e,f),m=u??(u===null?S:void 0)??r??n??S;".into(),
        ),
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
                r#""--effort <level>",`Effort level for the current session (${Ia.join(", ")})`"#,
                r#"Ia=["low","medium","high","xhigh","max"]"#,
                r#"function Y0(){let e=a.CLAUDE_CODE_EFFORT_LEVEL;return e?.toLowerCase()==="unset"||e?.toLowerCase()==="auto"?null:z1(e)}"#,
                "function kE(e,n,{turnEffort:r,hookEffortValue:s,carriedEffort:f=O(e)}={})",
                "let u=Y0();if(u===null&&!d)return;let S=INn(e,f),m=u??(u===null?S:void 0)??r??n??S;",
            ]
        );
    }

    #[test]
    fn a_word_refused_names_itself_claude_and_the_five() {
        assert_eq!(
            take("med"),
            Err("`med` is no effort claude takes: low, medium, high, xhigh or max".into())
        );
    }
}
