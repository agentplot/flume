//! flume's binary's library: the profiles of Part C built in, git as
//! they read a repository, the shell lines the terminal profiles write,
//! and the host's facts and git's history as the core reads them. The binary's commands build the core over
//! them; the tests under `tests/` run the seams' scenarios against them
//! (106).

pub mod commands;
pub mod git;
pub mod host;
pub mod profiles;
pub mod shell;

/// flume's version: a release's, from its tag (`FLUME_VERSION` when the
/// release is built), and the cargo workspace's in any other build.
pub const VERSION: &str = match option_env!("FLUME_VERSION") {
    Some(version) if !version.is_empty() => version,
    _ => env!("CARGO_PKG_VERSION"),
};
