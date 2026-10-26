# The core

## Why

flume is designed and ruled but holds no code. Every profile of Part C
codes against the seams of Part B, so the seam traits, the types they
pass and an engine that runs a kind's stages over them must exist, and
be held to the statement by a conformance suite, before any profile
unit can start (phase 1, Bolt 1, the core). This change builds that
core, end to end over fakes: nothing a person uses yet.

## What Changes

- **The seams.** A crate holding the seven seam traits of Part B, the
  place, the kinds, the change, the agent, the terminal, the merge queue
  and the store, one trait each with the operations Part B names, and
  the types they pass: a launch, a place and its held state, a kind's
  stages as read-kinds returns them, the six deliverable forms, a
  prompt and its parts, an agent's identity and start secret, a
  submission and its outcomes. Beside them, the two things the core
  reads that are no seam: git's history (section 9) and the host's
  clock, secrets and file times.
- **The engine.** A crate holding flume's one core: a launch makes one
  place and starts, holds or leaves waiting its first stage, or is
  refused whole naming why; each stage starts one fresh agent with its
  prompt filled; a stage ends only on its deliverable read at one head,
  its agent's stop-short under that start's identity and secret, or a
  conversation's finish; a settle moves a place on, submits a landing
  kind's place and sends it back on a failed submission up to its
  kind's count; retry, stop, drop, done, finish, tell and the caller's
  hold act as the statement says; status reads every fact where it
  lives; a tidy removes what is over and keeps what must be kept. Every
  answer is one JSON form that only gains fields.
- **The fakes.** A crate holding one scriptable fake per seam, and for
  git and the host, so a test sets what each seam answers and reads
  back what the core asked of it.
- **The conformance suite.** A crate holding the seam scenarios, each
  generic over its seam's trait so a profile unit runs them against its
  real tool, and the engine scenarios, among them every scenario of
  section 11, all over the fakes. Every scenario cites its clauses, and
  a coverage check fails when a clause of Part A or Part B, an
  invariant or a scenario of section 11 is cited by nothing.
- **The binary's crate**, holding the entry that hands each answer out
  as JSON and one marker per seam where a profile unit adds its module,
  and no command yet.
- **The gate.** Rust pinned in `.prototools`, each crate a moon project
  of its own with check-fast tasks that format, lint and test it, the
  coverage check a check-fast task, and treefmt claiming the new file
  types.

## Capabilities

### New Capabilities

- `seam-contracts`: the seven seam traits, the operations and guarantees
  each gives, the types they pass, and git's and the host's reads beside
  them.
- `seam-fakes`: one scriptable fake per seam, and for git and the host,
  each passing its seam's scenarios.
- `launching`: a launch making one place and starting, holding or
  leaving waiting its first stage, or refused whole naming why.
- `stage-runs`: a stage's one fresh agent, its prompt filled and
  delivered, and how its end is read: deliverable, stop-short, finish,
  and what is reported as stuck, lost or never begun.
- `settling`: what moves a place on, attended and under a watch, and a
  landing kind's submission with its send-back.
- `caller-acts`: retry, stop, drop, done, finish, tell and the caller's
  hold.
- `place-status`: status of one place or every place, read where each
  fact lives, in one JSON form that only gains fields.
- `tidying`: removing a place whose work is over, and keeping a place
  or a branch that must be kept.
- `held-state`: one versioned document per place written by
  compare-and-set, repeat-safe acts, and a restart that loses nothing.
- `clause-coverage`: the suite's scenarios each citing their clauses,
  and the check that fails when a clause is cited by nothing.

### Modified Capabilities

None: flume has no specs yet.

## Impact

- New crates `crates/flume-seams`, `crates/flume-core`,
  `crates/flume-fakes`, `crates/flume-conformance` and `crates/flume`,
  and the cargo workspace at the root, with serde and serde_json among
  its dependencies.
- The gate gains Rust: `.prototools` pins it, `.moon/toolchains.yml`
  reads the pin, `.moon/workspace.yml` lists the five crates, treefmt
  claims Rust and excludes the build directory, and `CLAUDE.md` names
  what check-fast now runs.
- The open item that the gate has no code yet leaves
  `docs/architecture/_open.md`.
- No profile, command or plugin: those are Bolt 2's and Bolt 3's, and
  they code against what this change lands.

## Touches

```
Cargo.toml
Cargo.lock
crates/flume-seams/**
crates/flume-core/**
crates/flume-fakes/**
crates/flume-conformance/**
crates/flume/Cargo.toml
crates/flume/moon.yml
crates/flume/src/*
.moon/workspace.yml
.moon/toolchains.yml
.prototools
treefmt.nix
.gitignore
CLAUDE.md
docs/architecture/_open.md
```
