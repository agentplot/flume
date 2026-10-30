//! The usage-limit words the Claude Code profile reads, against the
//! release the tree pins (design 6 of record 0020's change; C.4;
//! core.22): the program the `claude` wrapper from `devenv.nix` runs is
//! read as a file, and each source fragment of its usage-limit words the
//! profile's reading rests on must stand in it byte for byte. A release
//! that changes the words, or only renames what its minifier names in
//! those fragments, fails here until the reading is made again. No agent
//! runs.

use flume::profiles::agent::claude::{limit, wrapper};
use std::path::Path;

/// The Claude Code version `devenv.nix` pins.
fn pinned_version() -> String {
    let nix =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../devenv.nix"))
            .expect("devenv.nix");
    nix.lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("claudeVersion = \"")?
                .split('"')
                .next()
                .map(str::to_string)
        })
        .expect("devenv.nix pins Claude Code's version")
}

#[test]
fn the_pinned_release_holds_the_usage_limit_words() {
    let version = pinned_version();
    let path = std::env::var("PATH").unwrap_or_default();
    let found =
        wrapper::located("claude", &path).expect("a claude on PATH: run in the devenv shell");
    let resolved = std::fs::canonicalize(&found).unwrap_or_else(|_| found.clone());
    assert!(
        resolved
            .to_string_lossy()
            .contains(&format!("-claude-code-{version}/")),
        "the first claude on PATH, {}, is not the pinned {version}: run in the devenv shell",
        resolved.display()
    );
    let program = wrapper::unwrapped(&resolved)
        .unwrap_or_else(|| {
            panic!(
                "{} is no wrapper of the pinned release's program",
                resolved.display()
            )
        })
        .program;
    let bytes =
        std::fs::read(&program).unwrap_or_else(|error| panic!("{}: {error}", program.display()));
    let fragments = limit::fragments();
    let mut found = vec![false; fragments.len()];
    for at in 0..bytes.len() {
        for (index, (_, fragment)) in fragments.iter().enumerate() {
            let fragment = fragment.as_bytes();
            if !found[index] && bytes[at] == fragment[0] && bytes[at..].starts_with(fragment) {
                found[index] = true;
            }
        }
    }
    let missing: Vec<String> = fragments
        .iter()
        .zip(&found)
        .filter(|(_, found)| !**found)
        .map(|((name, fragment), _)| format!("{name}: {fragment}"))
        .collect();
    assert!(
        missing.is_empty(),
        "Claude Code {version}, at {}, does not hold the fragments the usage-limit reading rests on:\n{}",
        program.display(),
        missing.join("\n")
    );
}
