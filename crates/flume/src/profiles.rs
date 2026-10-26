//! The profiles of Part C built into the binary, one module each, bound
//! by the repository's binding and the host's (105, core.3). Each seam
//! has one marker below which a profile unit appends its module line;
//! the lists are append-only, so the units that write the profiles stay
//! apart (design 13).

use flume_core::Engine;

// ---- the place seam's profiles: a profile unit appends its module line below ----

// ---- the kinds seam's profiles: a profile unit appends its module line below ----

// ---- the change seam's profiles: a profile unit appends its module line below ----

// ---- the agent seam's profiles: a profile unit appends its module line below ----

// ---- the terminal seam's profiles: a profile unit appends its module line below ----

// ---- the merge queue seam's profiles: a profile unit appends its module line below ----

// ---- the store seam's profiles: a profile unit appends its module line below ----

/// The core over the profiles built in: the resolver from a binding's
/// profile names to them, and the host's terminal, store, history and
/// clock. No profile is built in yet, so the core names what it lacks.
pub fn core() -> Result<Engine, String> {
    Err("no terminal, store, history or host profile is built in: the core cannot be built over them yet".into())
}
