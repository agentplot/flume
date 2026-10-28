//! The profiles of Part C built into the binary, one module each, bound
//! by the repository's binding and the host's (105, core.3). Each seam
//! has one marker below which a profile unit appends its module line;
//! the lists are append-only, so the units that write the profiles stay
//! apart (design 13).

use flume_core::Engine;

// ---- the place seam's profiles: a profile unit appends its module line below ----
pub mod place {
    pub mod git_worktree;
}

// ---- the kinds seam's profiles: a profile unit appends its module line below ----
pub mod kinds {
    pub mod weir;
}

// ---- the change seam's profiles: a profile unit appends its module line below ----
pub mod change {
    pub mod openspec;
}

// ---- the agent seam's profiles: a profile unit appends its module line below ----

// ---- the terminal seam's profiles: a profile unit appends its module line below ----
pub mod terminal {
    pub mod herdr;
    pub mod printed;
}

// ---- the merge queue seam's profiles: a profile unit appends its module line below ----

// ---- the store seam's profiles: a profile unit appends its module line below ----
pub mod store {
    pub mod state_dir;
}

/// The core over the profiles built in: the resolver from a binding's
/// profile names to them, and the host's terminal, store, history and
/// clock. No terminal profile and no resolver are built in yet, so the
/// core names what it lacks.
pub fn core() -> Result<Engine, String> {
    Err(
        "no terminal profile and no resolver are built in: the core cannot be built over them yet"
            .into(),
    )
}
