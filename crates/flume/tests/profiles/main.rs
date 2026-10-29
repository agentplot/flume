//! The profiles' tests: each seam's scenarios against its profile on
//! scratch repositories, one module per profile (106, design 18). They
//! sit under `profiles/`, as the profiles do, since each names its tool
//! (core.1).

#[path = "../common/mod.rs"]
mod common;

mod built_in;
mod change_openspec;
mod kinds_weir;
mod place_git_worktree;
mod store_state_dir;
mod terminal_printed;
