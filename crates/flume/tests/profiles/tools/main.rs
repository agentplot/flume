//! The profiles against the tools the tree pins, the check-tools layer:
//! each seam's scenarios against its profile, on a server or a scratch
//! repository each test makes, never the host's own (106, core.22,
//! core.29), the usage-limit words the Claude Code profile reads, found
//! in the pinned release's program, a fresh clone's place equipped by the
//! pinned sluice, the walkthroughs, the built binary with the stand-in
//! for Claude Code (design 18), and a stream's view opened through the
//! binary under the pinned herdr (S90, S91). Built only with the `tools`
//! feature.

#[path = "../../common/mod.rs"]
mod common;

mod claude_words;
mod merge_queue_sluice;
mod open;
mod place_git_worktree;
mod stand_in;
mod terminal_herdr;
mod walk;
mod walkthroughs;

/// The exit by which a check says it could not run here, which
/// `.config/sluice.toml`'s `[tests] not_run` names, so sluice records no
/// outcome under the check's key and runs it again (sluice 114; record
/// 0047, rule core.51).
const NOT_RUN: i32 = 77;

/// Ends the run, the host lacking what a test needs of it, with `why` on
/// stderr past the harness's capture, which a process's exit would drop
/// (record 0047, rule core.51).
fn could_not_run(why: &str) -> ! {
    use std::io::Write as _;
    let _ = writeln!(std::io::stderr(), "could not run here: {why}");
    std::process::exit(NOT_RUN)
}
