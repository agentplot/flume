//! flume's binary's library: the profiles of Part C built in, git as
//! they read a repository, and the host's facts and git's history as the
//! core reads them. The binary's commands build the core over
//! them; the tests under `tests/` run the seams' scenarios against them
//! (106).

pub mod git;
pub mod host;
pub mod profiles;
