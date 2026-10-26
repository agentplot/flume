//! No module outside a profile names a worktree tool, a change tool, an
//! agent program, a terminal or a merge queue (core.1).

use std::path::{Path, PathBuf};

/// The tools' names, spelled in pieces so this file names none of them.
pub fn words() -> [String; 6] {
    [
        ["w", "t"].concat(),
        ["work", "trunk"].concat(),
        ["open", "spec"].concat(),
        ["cla", "ude"].concat(),
        ["her", "dr"].concat(),
        ["slu", "ice"].concat(),
    ]
}

/// Whether `path` lies in a profile's part of the tree: under a
/// `profiles` directory, or its `profiles.rs` module file.
pub fn in_profiles(path: &Path) -> bool {
    path.components()
        .any(|component| component.as_os_str() == "profiles")
        || path.file_stem().is_some_and(|stem| stem == "profiles")
}

/// Each tool's name `text` holds as a word, with its line.
pub fn hits(text: &str) -> Vec<(usize, String)> {
    let words = words();
    let mut found = vec![];
    for (number, line) in text.lines().enumerate() {
        let lower = line.to_lowercase();
        for token in lower.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
            if words.iter().any(|word| word == token) {
                found.push((number + 1, token.to_string()));
            }
        }
    }
    found
}

/// Every Rust source under `root`, `target` aside.
pub fn sources(root: &Path) -> Vec<PathBuf> {
    let mut found = vec![];
    let mut todo = vec![root.to_path_buf()];
    while let Some(directory) = todo.pop() {
        let Ok(entries) = std::fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name != "target") {
                    todo.push(path);
                }
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// Each hit in the sources under `root` outside a profile, as
/// `path:line: word`.
pub fn outside_profiles(root: &Path) -> Vec<String> {
    sources(root)
        .into_iter()
        .filter(|path| !in_profiles(path.strip_prefix(root).unwrap_or(path)))
        .flat_map(|path| {
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            hits(&text)
                .into_iter()
                .map(move |(line, word)| format!("{}:{line}: {word}", path.display()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_is_found_as_a_word_only() {
        let name = words()[5].clone();
        assert_eq!(hits(&format!("run {name} submit")), vec![(1, name.clone())]);
        assert_eq!(
            hits(&format!("// the {}'s", name.to_uppercase())),
            vec![(1, name.clone())]
        );
        assert!(hits(&format!("{name}_queue and x{name}")).is_empty());
        let short = words()[0].clone();
        assert_eq!(hits(&format!("a\n{short}-switch")), vec![(2, short)]);
    }

    #[test]
    fn a_profile_may_name_its_tool() {
        assert!(in_profiles(Path::new("flume/src/profiles/queue.rs")));
        assert!(in_profiles(Path::new("flume/src/profiles.rs")));
        assert!(!in_profiles(Path::new("flume-core/src/settle.rs")));
    }
}
