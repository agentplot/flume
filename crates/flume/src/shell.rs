//! Words written for a POSIX shell, as the terminal profiles write a
//! command a person or a pane's shell runs, and the end of what a step a
//! shell ran printed, as the place's readiness and teardown keep it.

/// `word` as one word to a POSIX shell: as it is where it holds only
/// characters no shell reads, single-quoted otherwise.
pub fn quote(word: &str) -> String {
    let plain = !word.is_empty()
        && word
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_./:@%+,".contains(c));
    if plain {
        word.to_string()
    } else {
        format!("'{}'", word.replace('\'', "'\\''"))
    }
}

/// `word` as one argument to a POSIX shell, standing after a command's
/// name: as [`quote`] gives it, `=` plain too, so a variable `env` is
/// given reads as written (188).
pub fn argument(word: &str) -> String {
    let plain = !word.is_empty()
        && word
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_./:=@%+,".contains(c));
    if plain {
        word.to_string()
    } else {
        quote(word)
    }
}

/// The words as one shell line, each quoted.
pub fn words<'a>(words: impl IntoIterator<Item = &'a str>) -> String {
    words.into_iter().map(quote).collect::<Vec<_>>().join(" ")
}

/// The most of a step's words kept, its end.
const WORDS: usize = 4000;

/// The end of what a step printed, trimmed, as every step's words are
/// cut, whichever profile reads them.
pub fn tail(words: &str) -> String {
    let words = words.trim();
    match words.char_indices().rev().nth(WORDS) {
        Some((at, _)) => words[at..].to_string(),
        None => words.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_quoted_only_where_a_shell_reads_it() {
        assert_eq!(quote("--model"), "--model");
        assert_eq!(quote("/r/places/fix--one"), "/r/places/fix--one");
        assert_eq!(quote(""), "''");
        assert_eq!(quote("two words"), "'two words'");
        assert_eq!(quote("it's $HOME"), "'it'\\''s $HOME'");
        assert_eq!(words(["a", "b c"]), "a 'b c'");
        assert_eq!(quote("A=b"), "'A=b'");
        assert_eq!(argument("A=/b"), "A=/b");
        assert_eq!(argument("A=/b c"), "'A=/b c'");
    }

    #[test]
    fn a_steps_words_keep_their_end() {
        assert_eq!(tail("  done\n"), "done");
        let long = format!("{}the end", "x".repeat(5000));
        let cut = tail(&long);
        assert_eq!(cut.chars().count(), WORDS + 1);
        assert!(cut.ends_with("the end"));
    }
}
