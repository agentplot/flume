//! No source of `crates/` names a tool outside a profile (core.1).

use flume_conformance::no_tool::{in_profiles, outside_profiles, sources};
use std::path::Path;

#[test]
fn no_tool_is_named_outside_a_profile() {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let hits = outside_profiles(&crates);
    assert!(
        hits.is_empty(),
        "a tool is named outside a profile (core.1):\n{}",
        hits.join("\n")
    );
}

#[test]
fn the_search_covers_the_commands() {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let commands: Vec<_> = sources(&crates)
        .into_iter()
        .filter_map(|path| path.strip_prefix(&crates).ok().map(Path::to_path_buf))
        .filter(|path| path.starts_with("flume/src/commands"))
        .collect();
    assert!(
        commands.iter().any(|path| path.ends_with("mod.rs")),
        "the search reads no command: {commands:?}"
    );
    assert!(commands.iter().all(|path| !in_profiles(path)));
}
