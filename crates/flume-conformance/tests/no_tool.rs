//! No source of `crates/` names a tool outside a profile (core.1).

use flume_conformance::no_tool::outside_profiles;
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
