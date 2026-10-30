# the-gate Specification

## Purpose
The checks a change to flume is held to and how they run: the commit
hooks, treefmt, the root project and each crate's and part's moon
project, the tools the tree pins, the layers `check-fast`, `check-tools`
and `check-live` and the branches that require each, and CI.

## Requirements

### Requirement: The commit hooks run in every worktree

The commit hooks SHALL read the hook config of the worktree the commit
is in, or the main checkout's where the worktree's shell has not written
one, and SHALL skip only where neither exists, so no commit needs
`PREK_ALLOW_NO_CONFIG`. They SHALL run convco on the message, treefmt on
the staged files, and `cargo clippy --workspace --all-targets -- -D
warnings` when a Rust file or the cargo manifests change (core.21).

#### Scenario: A commit in a fresh worktree

- **WHEN** a worktree whose devenv shell never ran commits a Rust file, and `root:hooks` drives that commit in a scratch clone
- **THEN** prek runs with the main checkout's config, convco, treefmt and clippy each run, and no `PREK_ALLOW_NO_CONFIG` is set

### Requirement: Every tracked file has a formatter or a named reason

treefmt SHALL fail on a tracked file no formatter claims and no exclude
names, every exclude SHALL carry a comment saying why, and the root
project's `format` task SHALL run `treefmt --ci` over the tree in the
check-fast layer (core.21).

#### Scenario: A file of a new type

- **WHEN** `root:format` runs over a tree holding a tracked file of a type no formatter claims
- **THEN** it fails naming the file

### Requirement: The tree's tools are the versions it pins

Every tool a gate task runs SHALL be the version `devenv.nix` or
`.prototools` pins, Claude Code, OpenSpec, herdr and sluice among them,
and `root:toolchain` SHALL fail when rustc, cargo, clippy or rustfmt is
not the build `.prototools` pins (core.22).

#### Scenario: A toolchain from outside the pin

- **WHEN** `root:toolchain` runs with a rustc on PATH other than the pinned one first
- **THEN** it fails naming the tool and both versions

### Requirement: The root project checks what spans the tree

The root project SHALL hold one check-fast task per check that spans
the tree: `format`, `toolchain`, `adrs` (no two records share a number),
`specs` and `changes` (`openspec validate --strict` of the specs and the
open changes) and `marketplace` (the marketplace manifest and each
plugin it lists) (core.21).

#### Scenario: Two records with one number

- **WHEN** `root:adrs` runs over `docs/adr` holding two records numbered 0015
- **THEN** it fails naming both

#### Scenario: An open change that does not validate

- **WHEN** `root:changes` runs over an open change whose requirement has no scenario
- **THEN** it fails naming the change and the requirement

### Requirement: Three layers, the gate and CI

The checks SHALL be tagged `check-fast`, `check-tools` or `check-live`;
`.config/sluice.toml` SHALL require `#check-fast` of a bolt and
`#check-fast` and `#check-tools` of main, under the sandboxed executor;
and CI SHALL run `:#check-fast` on Linux and macOS in the devenv shell
(core.21, core.22).

#### Scenario: The layers each task is in

- **WHEN** `root:layers` reads every project's tasks
- **THEN** every task carries exactly one of the three layer tags, or none with a comment saying why the gate cannot run it

### Requirement: The live walkthroughs pass under the pinned Claude Code

The `check-live` layer's walkthroughs SHALL carry a fix and a change
from their launch to their landing with the pinned Claude Code under
herdr, signed in by a test account's token, and SHALL leave a stage
whose pane goes quiet with no deliverable not ended (39, S3); the
operator runs them by hand, outside `sluice exec`.

#### Scenario: The operator's run

- **WHEN** the operator runs `moon run flume:live` from the devenv shell with a test account's token
- **THEN** both walkthroughs land, and the quiet stage with no deliverable is not ended
