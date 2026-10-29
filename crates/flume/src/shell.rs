//! Words written for a POSIX shell, as the terminal profiles write a
//! command a person or a pane's shell runs.

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

/// The words as one shell line, each quoted.
pub fn words<'a>(words: impl IntoIterator<Item = &'a str>) -> String {
    words.into_iter().map(quote).collect::<Vec<_>>().join(" ")
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
    }
}
