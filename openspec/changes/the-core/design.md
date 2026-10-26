# Design

## Context

The tree holds the site, the docs, the shell and the gate, and no
crate. The statement (`agentplot/blueprints:design/flume/requirements.md`
at 6246a48) is the source of every behaviour here; the phase 1 plan
(`design/flume/models/phase-1-bolts.md`, Bolt 1, at cdd7c43) fixes what
this unit owns; `design/flume/models/reconstruction-1.md` gave the
shapes (a data model, a state machine per stage and per place, ports and
adapters, one delivery protocol), and where it and the statement differ
the statement wins, as it does over this repository (core.19). It
predates the statement's settle with no process of flume's own (120),
the six deliverable forms (24) and flume's own submission (111), and
those come from the statement. The records behind the rules are 0002 to
0006. The one rules page is `docs/architecture/core.md`; no area page
exists, and this change adds none.

## Goals / Non-Goals

**Goals:**

- The seam traits and types Bolt 2's seven profile units code against,
  with the scenarios each profile must pass.
- The engine, end to end over the fakes, for every scenario of section
  11.

**Non-Goals:**

- Any profile of Part C, any command, either plugin.
- A real reader of git's history or of the host: the core reads both
  through interfaces with fakes here; the implementations through `git`
  and the host's own calls arrive with the first unit that runs flume
  on a real repository.
- Answering the statement's section 10. Where the engine must take a
  reading, it takes the clause's plain one (decision 11) and the gap is
  named for the design agent.

## Decisions

### 1. Five crates, one way down

`flume-seams` holds the traits and types and depends on nothing of
flume's. `flume-core` holds the engine over the traits. `flume-fakes`
holds the fakes over the traits. `flume-conformance` holds the
scenarios, the clause list and the coverage check, over core and fakes.
`flume` is the binary's crate. A profile depends on `flume-seams` alone,
so it implements its interface and nothing of the core (core.2), and the
core imports no profile (core.1). This is tailrace's Bolt 1 layout,
held to the same rule. One crate with modules was rejected: a profile
could then reach the core, and the fakes would ship in the binary.

### 2. The seven traits

One trait per section of Part B, its methods named as the section
names its operations, each answer a type of `flume-seams`, and an
answer a seam may not give either way (unknown, could not be read) a
variant of its own, never an error the core must guess from (93):

| trait | operations | clauses | profiles that will implement it (Part C) |
|---|---|---|---|
| `Place` | make, ready, read, remove, remove_branch | 78 to 83 | worktrunk, git worktree (C.1) |
| `Kinds` | read_kinds at a revision, read a launch's declaration, flume's own, read_binding | 84, 85, 19, 20, 105 | the kind format through weir's crate (C.2) |
| `Change` | read_change at a revision, path | 86, 87 | OpenSpec (C.3) |
| `Agent` | command, received, asking, own_work, stop_short, send_back, guard, on_quiet | 88 to 93, 115, 116, 121 | Claude Code (C.4) |
| `Terminal` | open, send, state, close, watch | 94 to 98, 122 | herdr, the printed command (C.5) |
| `MergeQueue` | submit, read_submission, wait, base_revision | 99, 101, 117, 125, 9 | sluice (C.6) |
| `Store` | read one or every place, write at a version, folder | 102 to 104 | the host's state directory (C.7) |

Three operations are beyond Part B's lists, each because Part A or a
rule needs a fact only a profile knows: `MergeQueue::base_revision`,
where the base's current revision lives or why it cannot be read (9,
core.7, C.1); `Change::path`, the change's location in the place, for
the prompt's change part; `Store::folder`, the place's own folder
beside its held state, for a report kept there (123, C.7). `Agent::stop_short`
is clause 92 as the trait gives it: the text of the one way to stop
short, filled into every prompt (28). `on_quiet` and `send_back` and
`guard` answer whether the program offers each (115, 116, 121); the
core does not drive them, the profile does, calling the core's settle
and stop check.

The binding (105, core.3) is the kinds seam's `read_binding`: for each
of the place, change and merge-queue seams, the profile name the
repository's binding gives, or none. The engine is built with a
resolver from profile names to trait objects; the fakes' resolver maps
names to fakes, and the binary's resolver is Bolt 2's. An unbound place
seam resolves to the place profile that needs nothing beyond git; an
unbound change or merge queue stays unbound, and a launch whose kind
reads a change or lands is then refused (4, 105). The agent seam is
resolved per stage from its settings' program (32, 105); the terminal
and the store once for the host.

### 3. Git and the host, beside the seams

Section 9 fixes git, so the core may read git's history (core.1 names
`git` as allowed), and must read the clock. Both go through two traits
of `flume-seams` that are not seams of Part B, so the core runs over
fakes and its tests run in no real time:

- `History`: a branch's head; whether head H holds a commit start head
  S does not (H not reachable from S, 25); the newest commit at H
  touching a path, and whether it is since S; whether a path exists in
  a commit since S at H; whether a branch name is valid.
- `Host`: now, sleep (the fake advances its clock), a fresh secret, a
  file's write time, and whether a waiting command named in held state
  still lives.

### 4. The held state

One document per place (103, C.7), read and written whole:

- The place: repository, kind name, version and the declaration it came
  from, the kind's stages as resolved (21), name, branch, base, the base
  revision it was cut from, location `<repository>/places/<kind>--<name>`
  (C.1), the binding's profile names, brief, mark, settings, holds and
  the submission's hold (7, 8).
- Its phase: making, live, over (landed with its commit, dropped or
  done), kept (what it holds, or the failed step), or gone.
- Readiness: each step's outcome and words (12, 79).
- Each stage: its state, of the vocabulary's list plus refused (27);
  its start count (48); its current start: started-at, start head,
  words, identity, secret, agent record, the prompt with its sendings
  and when received, first quiet reading, last head move (44, 118); its
  end as reported then (43): head, report path, stop-short words.
- The submission: how many times flume submitted (48), how many
  send-backs taken (112), the waiting command's id (124), the last
  verdict's words.

No deliverable, change or submission fact is kept between asks (58);
an end keeps only what was reported at that moment (43).

The order of a launch is: resolve and check every refusal that needs
nothing made (3, 4, 9, 27); create the document, phase making, by a
first write that fails when one exists (75, 103); make; ready; start,
hold or wait the first stage; write phase live. A refusal after the
create removes what was made and the document, so nothing is left (2,
76). Every later act reads with the version, decides, and writes at it;
a refused write re-reads and decides again (75).

### 5. A stage's end, read at one head

A settle reads H, the branch's head, once, and every fact of the
deliverable at H; if H moved by the end of the read, it reads again
(25). By form (24, C.2):

| form | delivered when, at H, against start head S and start time T |
|---|---|
| commits | H holds a commit S does not |
| change written | the change exists with planning complete, and its newest commit is since S |
| tasks ticked | the change has a task, every task ticked, and its newest commit is since S |
| change archived | the change archived, and H holds a commit S does not |
| report | the file at the path, under the store's folder for `{{state}}`, written after T |
| committed report | the path in a commit since S |

A stage delivered ends once its agent is quiet (40). Quiet is the
terminal's state waiting or stopped, the agent not asking and no own
work; unknown in any of them is not quiet, and status names it (31).
Stuck runs from the first quiet reading with nothing delivered and
restarts on any not-quiet reading or head move; working past the limit
runs from the beginning or the last head move (44, 118). Lost is the
terminal's stopped with nothing delivered or said (45). A conversation
reads none of these but lost, and ends only on finish (109).

### 6. Prompts and tells

Rendering fills `{{brief}}`, `{{name}}`, `{{kind}}`, `{{stage}}`,
`{{branch}}`, `{{base}}`, `{{place}}`, `{{change}}` (the change seam's
path), `{{report}}`, `{{words}}`, `{{unready}}` and `{{needs}}` (the
agent seam's stop-short text), with `{{#words}}…{{/words}}` rendered
only when there are words (C.2). The launch checks `{{brief}}` for every
stage (4); any other bare part with nothing to fill it leaves the stage
refused naming it (27). The parts are flume's own prompt vocabulary,
not fields of the kind format (core.4).

Delivery (37, 38, 54): wait until the terminal reads waiting and the
agent not asking; skip a text already received; send; within 60
seconds read received; where the terminal can send, send once more and
read again; else never begun. Under a terminal that refuses send, the
agent's command carries the prompt and the start is the one sending. A
tell waits for waiting up to its bound, ten minutes by default (54).

### 7. The settle

`settle(place)` is the one mover (46, record 0005), run by the agent
profile's on-quiet, by the terminal's watch, by the submitting command
when the verdict comes, and, where the terminal keeps no watch, by every
acting command for every place first (119). In order: a making place is
finished or undone; a place over is tidied; else the current stage's
state is read (sections 5, 6) and the next step taken: send its prompt,
end a delivered stage's quiet agent, start the next stage unless held
(40, 47), submit after the last (111), wait again on an open submission
nobody waits on (124). Status and rendering settle nothing (57, 119).

The submit (111, 117): the settle records itself as the waiter in held
state, calls `MergeQueue::submit` at the head, and on the verdict
writes it and settles again. Landed makes the work over and the tidy
removes what the merge queue left (113, 66). Failed sends the place
back to the stage the last stage's `on_fail` names, the last stage when
it names none, with the verdict's words, while the send-backs taken are
fewer than the kind's count; past it the place waits (112). Refused or
withdrawn leaves the place waiting with its reason (112). A landing
read by `read_submission` before flume submits is the landing (63).

### 8. Caller acts and the stop check

Retry, stop, drop, done, finish and tell (the caller-acts spec) each
end an agent only through the terminal's close,
and only when quiet or forced; a close the terminal refuses is answered
as a failure (31). A retry of the first stage of a place whose
readiness failed runs readiness again as its launch read it (12).
`needs(identity, secret, words)` is the stop-short (41): taken when the
identity is the current start's, the secret is that start's, and the
agent is live. `stop_check(identity)` answers the agent profile's
send-back (110, 115): words naming the deliverable and the stop-short
text when the stage is no conversation, nothing is delivered or said,
there is no own work and no open question; else none.

### 9. Answers

Every act returns one `Answer`, serialized by serde to JSON (61): a
place's status, a launch's place, a refusal with its reason, a tidy's
removed, kept and branches kept. The suite holds each answer's field
paths, read from a full instance of it, and fails when one disappears;
a new field is added to the list in the commit that adds it.

### 10. Defaults

The statement fixes three: a stage's quiet limit of 30 minutes (44), 60
seconds to read a prompt received (38), ten minutes for a tell (54).
These are constants of the core. Two it does not fix: the count of
send-backs when a kind sets none (section 10, its last question) and
the agent settings when neither launch nor stage gives them (32). The
core takes both as a `Defaults` value it is built with and holds no
value of its own; every scenario that sends a place back declares its
kind's count.

### 11. Where the statement leaves a reading open

The engine takes each clause's plain reading and names it here, so the
design agent can rule against it:

- The stop check sends back each time it is asked; no count bounds it
  (110; section 10 asks how many).
- A conversation's agent that stops with no finish reads lost (45; 109
  ends it only on finish).
- A failed submission starts the stage it sends back to even when that
  stage waits on its caller's word; the later stages wait as 47 says
  (112; section 10 asks).
- A held stage or submission is never reported for its wait (47).

### 12. The suite

`flume-conformance` holds:

- `seams/<seam>.rs`, each scenario a function generic over its trait,
  run here against the fake and by a profile unit against its tool,
  below one marker per seam where a profile unit appends (phase 1, how
  the units stay apart).
- `engine/`, the scenarios over the fakes, S1 to S34 among them, each
  named for its S number where it has one.
- `clauses.rs`, the clause list of Parts A and B, I1 to I8 and S1 to
  S34 at the statement's revision; and the coverage check, a test and a
  binary its moon task runs, failing on a clause, invariant or scenario
  cited by nothing, a scenario citing nothing, or a citation of no
  clause (106, section 12). No register of pending clauses: over the
  fakes every clause can be cited, and Part B's guarantees are the seam
  scenarios'.

Each scenario carries its citations as data the check reads, not as a
comment.

### 13. The binary's crate and the shared files

`crates/flume/src/main.rs` builds the core and writes each answer as
JSON; it holds no command yet, which Bolt 3 adds under `src/commands/`.
`crates/flume/src/profiles.rs` holds one marker per seam, below which a
profile unit appends its module line, append-only like the suite's
markers. The workspace `Cargo.toml` takes a dependency line per profile
unit, append-only.

### 14. The gate

Rust is pinned in `.prototools` at 1.98.1, the version tailrace pins,
and `.moon/toolchains.yml` reads it with `versionFromPrototools` and
the clippy and rustfmt components (core.22, kit:toolchain.2). Each of
the five crates is a moon project in `.moon/workspace.yml`, language
rust, with check-fast tasks `format` (`cargo fmt --check -p`), `lint`
(`cargo clippy -p … --all-targets -- -D warnings`) and `test` (`cargo
test -p`), each declaring its inputs with `/devenv.nix`, `/devenv.lock`
and `/.prototools` (core.21, kit:testing.1, kit:testing.3); the
conformance project adds `coverage`. treefmt claims `*.rs` with the
rustfmt of that same toolchain, so the hook and the format tasks run
one rustfmt (core.21, core.22), and excludes `target/**` with its
reason; `target/` is ignored. `CLAUDE.md`'s layer table names the cargo
checks in check-fast (kit:testing.2), and the open item that the gate
has no code leaves `_open.md`.

## Risks / Trade-offs

- [The three operations beyond Part B's lists are a contract the
  statement does not write] → each is named in decision 2 for the
  design agent to write into Part B, and none holds a tool's name.
- [The fakes admit what a real tool would not] → every fake passes its
  seam's scenarios, the same ones a profile runs (106).
- [A real `History` and `Host` are left for later] → the traits are
  small and fixed here, so the unit that writes them codes against what
  the engine already uses.
- [The answers' field list is hand-kept] → it is read from a full
  instance of each answer, so a new field fails nothing and a removed
  one fails the suite.
- [The plain readings of decision 11 may be ruled otherwise] → each is
  one branch of the settle or the stop check, named, with a scenario.
