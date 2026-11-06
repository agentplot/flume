## MODIFIED Requirements

### Requirement: The commit hooks run in every worktree

The commit hooks SHALL read the hook config of the worktree the commit
is in, or the main checkout's where the worktree's shell has not written
one, and SHALL skip only where neither exists, so no commit needs
`PREK_ALLOW_NO_CONFIG`. They SHALL run convco on the message, taking a
message `git revert` wrote, its first line starting `Revert "`, as CI's
conventional check takes the commit by `--ignore-reverts`, treefmt on
the staged files, and `cargo clippy --workspace --all-targets -- -D
warnings` when a Rust file or the cargo manifests change (core.21,
core.22).

#### Scenario: A commit in a fresh worktree

- **WHEN** a worktree whose devenv shell never ran commits a Rust file, and `root:hooks` drives that commit in a scratch clone
- **THEN** prek runs with the main checkout's config, convco, treefmt and clippy each run, and no `PREK_ALLOW_NO_CONFIG` is set

#### Scenario: A revert committed after `--no-commit`

- **WHEN** `root:hooks` commits in that worktree a revert `git revert --no-commit` prepared, with the message git wrote, and then a message with no type
- **THEN** convco passes the revert, which is committed, and refuses the message with no type
