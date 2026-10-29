# Design

## Context

The core landed from `bolt/flume-the-core` (4e8bb15): the seven seam
traits, the engine over them, one fake per seam, the conformance suite
citing S1 to S34, and the binary's crate with no command and no
profile. The statement (`agentplot/blueprints:design/flume/requirements.md`
at e5f92f6) is the source of every behaviour here, and it moved after
the core was built: records 0010 to 0015 rewrote 7, 8, 22, 41 to 43,
56, 62, 70, 111 to 113, 120, 124, 130, 131, 133, C.1, C.2, C.4, C.6,
C.7, S1 and S19, and added 134 and S35 to S38. Where the core and the statement differ the
statement wins (core.19), so this change first brings the core to it,
then builds the profiles of Part C and two of the three surfaces of
section 9. Records 0014 and 0015 are accepted; their rule texts
(core.3, core.5, core.11, core.24, core.25, core.27, core.28) are cited
here as those records word them. Record 0016, accepted on 2026-10-28,
brings crew's lessons driving Claude Code through herdr to the
statement at blueprints 4fe1522 (35, 37, 38, 43, 77, 89, 90, 95, 96, 98,
135, 136, C.4, C.5, S39 to S43) and adds core.29; groups 7 and 9 carry
it, groups 1 to 6 having landed before it.

The work is Crew's dropped bolts `flume-the-profiles`,
`a-kind-runs-through-on-its-own`, `flume-keeps-the-hosts-queue-by-priority`,
and the command line and Claude Code plugin of `flume-the-surfaces`,
carried as one change in ten groups, each sized for one implementing
session. The capture plan's L3 is its done (`design/capture/plan.md`),
read with record 0012: no submission is waited on.

The one rules page is `docs/architecture/core.md`. No area page exists,
and this change adds none: the profiles and the plugin bind under
core.1 to core.3 and core.21 as they stand.

## Goals / Non-Goals

**Goals:**

- A fix, a chore or a change launched on one host runs from its first
  stage to its landing with no caller's word between the stages, sent
  back to its kind's fix round by a failed submission or by found work,
  and tidied by flume (131).
- The host keeps its bound on agents, and starts what waits by its
  callers' priorities (70).
- Every seam reached through a profile of Part C, each passing its
  seam's scenarios against the real tool where the gate can run it.
- The command line and flume's Claude Code plugin, each answering from
  the one core.
- flume's gate as sluice keeps its own.

**Non-Goals:**

- A hold put after the launch, a stalled place, an end delivered to its
  recipient as a notice, `flume inbox`, a wait, a role's address (43,
  130, 132, S38), and the herdr plugin's watch and sidebar: the next
  change's. Under this change no terminal keeps watch, so flume runs
  attended (119), and the agent's going quiet moves a place on (121).
- A stage's made things (129; record 0007), and a place's document
  kept past its tidy while an end is pending (62): the next change's,
  with delivery.
- A worktrunk place profile (C.1; record 0014).

## Decisions

### 1. The gate brought to sluice's setup

sluice's setup (`agentplot/sluice` at 08636da1) is the reference; each
item below is where flume's falls short of it today.

- **The commit hooks.** devenv writes `.pre-commit-config.yaml` (ignored)
  when a worktree's shell first runs, and flume's hook execs prek with
  no config elsewhere, so a commit in a fresh worktree fails unless
  `PREK_ALLOW_NO_CONFIG=1`. `devenv.nix` installs sluice's hook shim
  instead: prek with the worktree's own config, or the main checkout's
  (git's main worktree, or `main` beside a bare repository), skipping
  only where neither exists. The hooks gain sluice's `clippy-gate`
  (`cargo clippy --workspace --all-targets -- -D warnings`, on `*.rs`
  and `Cargo.{toml,lock}`) beside convco and treefmt.
- **treefmt.** `settings.on-unmatched = "fatal"`, so a tracked file no
  formatter claims fails (core.21); every exclude carries its reason;
  Biome formats and lints JSON, TypeScript and JavaScript (the site's
  scripts and config, the plugin's manifests) in place of excluding
  `*.json` and `site/**` whole, with `site/**/*.astro` excluded with
  sluice's reason; taplo claims `.prototools`; yamlfmt claims
  `.versionrc`; shellcheck then shfmt claim shell scripts;
  actionlint claims the workflows. The rustfmt wrapper stays the one
  that runs the pinned toolchain's.
- **The root project.** `moon.yml` at the root, project `root` in
  `.moon/workspace.yml`, with check-fast tasks one per check: `format`
  (`treefmt --ci` over the tree), `toolchain` (`check-toolchain.sh`:
  rustc, cargo, clippy and rustfmt are the build `.prototools` pins,
  never cached), `adrs` (`check-adrs.sh`: no two records share a
  number), `specs` and `changes` (`openspec validate --specs` and
  `--changes`, `--strict --no-interactive`, by the pinned OpenSpec, no
  telemetry), `marketplace` (the marketplace manifest validates and
  every plugin it lists has one), `hooks` (the commit hooks driven in a
  scratch clone's fresh worktree: convco, treefmt and clippy run with
  the main checkout's config) and `layers` (every task carries one layer
  tag, or a comment saying why the gate cannot run it). Each crate keeps its own `format`,
  `lint` and `test`.
- **The pinned tools.** `devenv.nix` pins, by their release builds as
  sluice pins its own: Claude Code (sluice's version, 2.1.296), OpenSpec
  (1.14.3, sluice's), herdr (the release the terminal profile is
  checked against), and sluice (the release that holds `sluice inbox`,
  sluice's change `outcomes-are-delivered`, which is built before this
  change's group 7). Each profile's tests run the pinned tool, never one on the
  host's PATH (core.22).
- **The shell.** As sluice's: a run sluice starts gets `SLUICE_JOBS`
  cores (`CARGO_BUILD_JOBS`, `RUST_TEST_THREADS`); a session's own shell
  compiles under sccache; `disk` and `lint` scripts.
- **The layers.** `check-fast` (the fakes; every profile over real git
  and files on scratch repositories; the plugin's hooks against the
  built binary), `check-tools` (the sluice and herdr profiles against
  the pinned tools, and the walkthroughs with a stand-in agent program),
  and `check-live` (the walkthroughs with a real Claude Code, outside
  the sandboxed gate and run by hand in the devenv shell). `.config/sluice.toml`: the bolts require `#check-fast`,
  `main` `#check-fast` and `#check-tools`; the executor is the sandbox
  with localhost networking, trusting runs as sluice's own gate does.
- **CI.** `.github/workflows/ci.yml` runs `devenv shell -- moon run
  :#check-fast` on Linux and macOS, so CI and `sluice exec` hold a tree
  to the same rules (core.22).

### 2. The merge queue seam without a wait

`MergeQueue` takes the operations B.6 now names (117, 133, 99, 125,
126) and loses `wait`:

| operation | answer |
|---|---|
| `submit(place, head)` | `Submitted { submission }` once the merge queue holds it, or `Refused { reason }`; no wait, nothing removed (117) |
| `inbox(place)` | `Outcomes { outcomes }`, each `Landed { commit }`, `Failed { words }` or `Conflict { files, base, words }`, each delivered to the place once; or `CouldNotRead { reason }` (133). A withdrawal is never delivered: it is read from `read_submission` (99) |
| `read_submission(repository, branch)` | unchanged (99) |
| `step(repository, base)` | `Stepped`, `MovesOnItsOwn`, or `CouldNotRead { reason }` (125) |
| `base_revision(repository, base)` | unchanged (126) |

`Verdict`, the outcome an answer reports, gains `Conflict { files,
words }` and keeps its variants and fields, `place_removed` and
`branch_removed` always false under a merge queue that removes no
place (113), since an answer's form only gains fields (61). The held
submission loses its waiter: no command of flume's waits (core.6,
core.13).

### 3. The settle reads the submission it made

A settle of a live place whose kind lands, in order (46, 111, 124):

1. Where flume has submitted the place, collect its pending outcomes
   through `inbox` and act on each in the order given: landed makes the
   work over; failed or in conflict starts the fix round (decision 5)
   with the outcome's words. An inbox that cannot be read is named in
   status and acts on nothing (68).
2. Read the newest submission of the branch through `read_submission`:
   a withdrawal of flume's submission leaves the place waiting with why
   (112), and where flume has not submitted, a landing there is the
   place's landing (63, S30).
3. Where a submission is open, drive its base one round through `step`,
   once per repository and base in one command, however many places
   wait on it (125).
4. Read the current stage and take its step, as the core does today.
5. Submit once the last stage delivered and its agent ended, unless held
   or over (111).

A landed place is tidied by flume's tidy through the place seam (66,
113, core.18). A restart loses nothing: the outcome stays pending in
the merge queue until the next settle collects it (124, S34).

### 4. The stop-short's two kinds

`StopShort` gains `kind: NeedsAnswer | FoundWork` (41, 92), and a
stage's end keeps it with the words (`StageEnd::needs`). `flume needs
"<words>"` sends `NeedsAnswer`, `flume needs --found "<words>"`
`FoundWork` (C.4), each refused unless `FLUME_AGENT` names the agent of
the place's current stage and `FLUME_SECRET` is that start's, while the
agent is live (core.9).

### 5. A kind runs through on its own

- **Delivery starts the next stage** (40), the host's room allowing
  (decision 6), unless the launch holds it (47).
- **The fix round** is the stage the kind's last stage names in
  `on_fail`, that last stage where it names none (23, C.2). A failed or
  conflicting submission starts it with the merge queue's words (112);
  a conflict's words carry the base's revision the merge queue tested
  against, and the stage's agent takes the base in by a merge of that
  revision it commits in the place, resolving the files contested,
  before anything else, never by a rebase: the agent's commit in its
  place, never flume's (14, core.5);
  found work from the kind's last stage starts it with the findings
  (131). Both count toward one count, the place's goings-back, against
  the last stage's `max_send_backs`, three where none is set (23;
  section 10, ruled 2026-10-26). Past it the place waits on its caller,
  status saying so with the words (130). The fix round starts at once
  even where it waits on its caller's word; the later stages that wait
  still wait (47; section 10, ruled 2026-10-26).
- **Found work from any other stage** is taken as needing an answer
  (131): the stage ends short and the place waits for a retry, status
  giving the kind the agent said.
- **Needing an answer** ends the stage short and starts nothing (42).
  The answer is a retry naming the stage; the fresh agent's
  `{{#words}}` carries the question, then the answer: `The question:
  <stop-short's words>` and `The answer: <retry's words>` (49, C.2).
  flume reads no tell, card or pane for an answer (core.27).
- **flume's own kinds** are data the kinds profile builds in (decision
  7): fix@1 and chore@1 send back to `code`, and change@1 sends back to
  `code`, never to verify (22, C.2).

The core holds no kind's name: change@1's send-back to code is its
declaration's `on_fail`, read like any kind's (core.4).

### 5a. Recipients and holds at the launch

A launch names the recipients of its ends (7): each `--recipient`, or
`$FLUME_RECIPIENT` where the launch names none, or else the session
that launched it as the terminal names it (the terminal seam's
`session`: under herdr the launching pane, under the printed command
none). A hold at the launch carries its words, `--hold
<stage>=<words>` (7, 130). Both are fixed at the launch (8), kept in
the place's held state and given by status. This change stores them;
delivering an end to a recipient is the next change's (43).

### 6. The host's queue

- **The bound** is the host's fact, `[host] agents` in
  `$XDG_CONFIG_HOME/flume/config.toml` (C.7), read through the `Host`
  facts (design 3 of `the-core`) as `bound() -> Option<u32>`; none
  means no queue (70).
- **Agents running** are the live starts across every place in the
  store, every caller's counted together. A place whose stage waits
  holds none.
- **A start is admitted under the room lock.** Every start of a stage,
  a launch's first, a delivery's next, a fix round's, a retry's, takes
  the host's room lock (`Host::room() -> RoomGuard`), counts the agents
  running, and either writes its start as live and opens its agent
  after the lock is released, or marks the stage `WaitingForRoom` with
  since when. So two settles at once never start past the bound (70,
  75). An open the terminal refuses ends the start, freeing its room.
- **Every settle starts what waits**, after its own place's step: under
  the lock, the stages waiting for room on the host, highest priority
  first and equals in the order they came to wait, as many as there is
  room for (46, 70). No agent is ended for room (31).
- **Priority** is an integer, higher first, zero where the launch gives
  none, kept in the place's held state beside what the launch fixed,
  since a caller changes it after (7, 8, 70). `flume priority <place>
  <n>` changes it; it reorders only what has not started, and is
  refused once the place's work is over. flume assigns none.
- **Status** gives, for the host, the agents running, the bound, the
  room, and the places waiting for room in the order they would start,
  each with its stage, priority and since when (56). A place waiting
  for room waits on the host, never on its caller (130).

### 7. The kinds through weir's crate

`weir-kinds` from agentplot/weir, a git dependency pinned to a
revision of weir's main (379d68e as this change is written), moved by a
chore when weir changes (phase 1, "Weir's crates"). flume has no reader
of its own (core.4).

- A repository's declaration is `.config/flume.toml` at the base's
  revision (`git show <revision>:.config/flume.toml`); none is a
  declaration with no kinds. A launch's is the file `--kinds` names.
  Prompt paths resolve against the declaration's file, read at the
  same revision for a repository's (20, C.2).
- flume's own declaration and its prompts are files under the profile's
  directory, built into the binary and read through `weir-kinds` like
  any other: fix@1, chore@1 and change@1 as C.2's table gives them.
  The prompts: code fixes what the brief or the words name and commits;
  under every kind, code sent back with a conflict's words first merges
  the base's revision the words name into the place's branch with `git
  merge`, resolves the files contested and commits the merge, before
  anything else, and never rebases (112, core.5);
  under change@1, code sent back with words first moves an archived
  change back out of `openspec/changes/archive/`, then records the fix
  as a task of the change, ticked; verify checks the work against the
  change, commits its report as `verify.md` in the change's directory,
  and archives the change on the branch, or stops short having found
  work with what fails (22, C.2).
- The map onto the seams' types: version, branch prefix,
  `starts_unready`, `lands`; each stage's name, prompt, end, `wait`,
  `quiet`, `on_fail` and `max_send_backs`; each agent table's
  `program`, `model`, `instructions`, `env` and `beside`. A stage with
  two agents or two deliverables keeps both, so the core refuses the
  kind (4). Sections the format does not name are ignored (23).
- The binding is the declaration's `[profiles]` table, read at the
  default branch: `origin/HEAD` after one `git fetch` where the
  repository has a remote, the local default branch otherwise (20,
  105). With no table, or a seam left out of it, the place seam is
  `git-worktree` and the change and merge queue are unbound (C.2). A
  place profile the binary does not hold, `worktrunk` among them, is
  refused at the launch naming the place profiles there are (C.1,
  core.3); the resolver lists them, so the core names no profile.
- flume's own repository gains `.config/flume.toml` with its
  `[profiles]`: `git-worktree`, `openspec`, `sluice`.

### 8. The change through OpenSpec

Read at a revision with git alone for the spec-driven schema (C.3, 59):
the change exists when `openspec/changes/<name>/` does; planning is
complete when the artifacts the schema puts before tasks exist
(`proposal.md`, a `specs/**/spec.md`, `design.md`, `tasks.md`); its
tasks are the `- [ ]` and `- [x]` lines of `tasks.md`; it is archived
when `openspec/changes/archive/<date>-<name>/` exists. A change of any
other schema is read through `openspec status --change <name> --json`
in a checkout of the revision made for the read and removed after it.
A test holds the file reading to `openspec status` on the same
revisions (59). `locate` is `openspec/changes/<name>` (127).

### 9. The place through plain git worktree

- **make**: `git worktree add -b <prefix>/<name> <location> <revision>`
  at `<repository>/places/<kind>--<name>`, with `core.hooksPath` set to
  an empty directory for the call so no hook runs (C.1, 78). The
  repository is the directory holding the bare repository and its
  worktrees (`project/project.git`, `project/main`), or the main
  worktree of a repository that is not bare.
- **ready**: the steps the merge queue's profile declares for the
  repository, handed to the place profile as data (decision 13): under
  sluice, `[tests] toolchain` from `.config/sluice.toml` at the base's
  revision, its printed environment kept in the place's own git
  directory (`<git-dir>/flume/toolchain.sh`), then the runner's setup
  (`moon setup` for the moon runner) under it. Each runs with an
  environment cleared to the host user's login environment (`HOME`,
  `USER`, `LOGNAME`, `SHELL`, `TMPDIR`, `LANG` and the login shell's
  `PATH`), nothing of the caller's session (11, 79, C.7). With no merge
  queue bound, nothing runs.
- **read**: `git status --porcelain=v2 --untracked-files=all` and
  `git worktree list --porcelain` for a lock (80).
- **remove**: the merge queue profile's teardown steps (under sluice,
  `sluice release` in the place), then `git worktree remove <location>`,
  never `--force`; a failing step keeps the place named with it (66,
  81, C.6).
- **remove-branch**: `git cherry <base> <branch>` against the base where
  the merge queue's profile says it lives; any `+` line keeps the
  branch with those commits, otherwise `git branch -D` (67, 82).

The readiness, the teardown and the base's ref are the merge queue
profile's data, which the binary's resolver hands to the place profile
when it binds a repository; neither profile calls the other (core.2).
The place seam's `ready` and `remove` are unchanged in shape.

### 10. The store and the host's configuration

- Each place's document is `$XDG_STATE_HOME/flume/places/<flat>.json`,
  its version inside it, and its own folder `<flat>/` beside it, the
  location flattened to one file name (C.7, 102, 128).
- A write takes `<flat>.lock` (flock), reads the version, writes a
  temporary file and renames it over the document; a version that moved
  refuses it. A new place's first write creates the document by a link
  that fails when one exists (75, 103). A write of none removes the
  document and the folder (62).
- The room lock is `$XDG_STATE_HOME/flume/room.lock` (decision 6).
- The host's configuration is `$XDG_CONFIG_HOME/flume/config.toml`:
  `[host] agents` (C.7), `[host] terminal` (decision 12), and the
  agents' settings by kind and by repository and kind (decision 16a).

### 11. Git's history and the host, read for real

`History` through `git` in the repository: `rev-parse` for a head,
`merge-base --is-ancestor` both ways for a commit since a start, `log
-1 --format=%H -- <path>` for the newest commit touching a path, `log
<start>..<head> -- <path>` for a path in a commit since, and
`check-ref-format --branch` (25, C.1). `Host`: the system clock, a
sleep, a secret of 32 random bytes in hex, a file's modification time,
a command named by process id and start time with whether it still
lives, the bound and the room lock (decisions 6, 10).

### 12. The terminal through herdr and the printed command

The terminal is bound once for the host by `[host] terminal`, `herdr`
or `printed`, `printed` where none is named (105). C.7 names no such
word in C.7 (105).

- **herdr** (C.5; records 0016, 0018): every command, state name, error code
  and field is checked against the pinned herdr, 0.9.5 or later
  (core.29), and every call runs under a bound, ten seconds where the
  call states none, read unknown or refused in herdr's words past it,
  with the caller's `HERDR_*` stripped and the recorded server's socket
  passed (98), since a crew team once waited two hours inside an
  `agent prompt` that never returned. open makes a pane in the herdr
  session the launch's settings name, flume's own otherwise: in the
  workspace of the place's repository and base, labelled `<repository>
  <base>` and made `--no-focus` when the first place on that base
  opens, one tab per place named `<kind>--<name>`, whose panes are the
  place's agents, each recorded by id and socket in the host's state; or a split beside
  the pane the settings name, left where its caller put it. The pane is
  made with `--cwd <place>` and an `--env` for each variable of the
  agent's environment and flume's `FLUME_*`, `PATH` led by a folder of
  the start's own linking the kind's name to the program the settings
  name, since herdr's `agent start` takes no environment and types the
  kind's name into the shell. open then runs `agent start <name> --kind
  <kind> --pane <id> --timeout 120000 -- <arguments>`, bounded ten
  seconds past it, which hands the arguments over whole; nothing is
  typed and no launcher file written. `agent_not_ready` is the start
  held at a question: the folder-trust question, found in the pane's
  visible lines, is answered with the keys the agent profile names (34,
  decision 14) and `agent wait --until idle --timeout 30000` follows;
  any other is answered held with the screen, and `timeout` not come up
  with the screen, each read never begun by the core (decision 12b).
  herdr's `agent_started` is a second word: the started ack is the
  start (35). Once started, the program in the pane's foreground,
  read from `pane process-info` and the host's process table, must be
  the one the settings name, or the start is closed and refused naming
  what ran (32). The pane is held by its id and its server's socket and
  found again by the agent's herdr name where the id no longer answers,
  as after a move to another workspace: a name of at most 32 of herdr's
  legal characters made from the identity and ending in its digest,
  unique on the server, a pane found by it taken only once herdr lists
  it under that name and its new id recorded before anything is sent.
  A kind the pinned herdr does not list cannot run under herdr and
  refuses the launch (32). `session` is
  `herdr:<pane>@<socket>` (7). send clears the held input with one
  ctrl+c while the agent waits, types the text and submits it, received
  then read through the agent (89, 95); the core's second sending and a
  prompt never begun clear it again, so nothing is left typed. state
  maps herdr's reading: working as working; idle and done as waiting,
  herdr's done being a finished turn nobody has looked at; blocked as
  asking, which the core takes as asking only where the agent seam's
  asking reads yes and as unknown otherwise, so the terminal reads no
  profile's mark itself (core.2); herdr's unknown as unknown; stopped
  only when the pane is gone, herdr lists no agent in it and
  process-info shows the shell in its foreground, or herdr lists the
  pane with a `restore_error` after its server's restart (96). screen
  is `pane read --source visible` (136). close ends the pane and its
  process group and reads that it is gone (97). tidy closes the place's
  recorded tab, and the base's recorded workspace once it holds no tab
  of a live place and no pane flume did not open; either is kept,
  named, where a pane flume did not open stands, and nothing flume did
  not make is closed (137, 138). show reports `pane report-metadata
  <pane> --source flume --title "<stage> · <state>" --display-agent
  <kind>--<name>` and `workspace report-metadata <w> --source flume
  --token flume=<summary>`, a failure logged (139). watch is none: the
  herdr plugin keeps it in the next change (122). A fresh pane per
  start makes moot crew's escape codes left by a bad exit and the
  exit's question about background work.
- **printed command** (C.5): open answers the command, the prompt
  carried in it, which status gives with the agent; send and close are
  refused, tidy and show do nothing, so a tell is refused and a prompt not received is never
  begun (38); screen is nothing (136); state is unknown from the terminal, and the agent is read
  stopped once the agent profile's own record says its session ended
  (decision 14); watch is none, so flume runs attended (119).

### 12a. A start is starting until its agent says it started

Record 0016 (35, 37, 38, 43, 77, 135, 136). The agent seam gains
`started`: whether the agent has said it started under its start's
identity with flume's guard and stop check in force, or unknown. The
terminal seam gains `screen`. `StageState` gains `Starting`, the state
from the start until `started` reads yes. The core:

- sends no prompt while `started` does not read yes (37);
- reads a start never begun at once when the terminal reads its agent
  stopped with no started word; at the stage's limit (44) while it
  stays starting, a dialog no hook precedes holding it; or on its
  caller's stop or retry; status gives the screen meanwhile (35);
- reads a terminal's blocked with no asking mark as unknown, never as
  waiting, so nothing is typed into a dialog it cannot name (90);
- takes an agent the terminal brought back with no started word for
  its start as no agent of the stage's: the stage is lost and the agent
  ended once quiet (77, 31);
- delivers stuck, lost and never-begun ends with the agent's last
  words, the commits on the place's branch since the start and, for
  never begun, the screen (43).

The clause list moves to blueprints 4fe1522, adding 135, 136 and S39 to
S43, each cited by an engine scenario.

### 12b. A start held, a place's terminal tidied, and clues shown

Record 0018 (35, 94, 137 to 139, S45 to S49). `AgentCommand` gains the
program's kind, which the agent profile gives (`claude`, C.4). The
terminal seam's `open` answers, beside opened and refused, a start held
at a question its program asks before it starts and a program that did
not come up within the terminal's bound, each with the screen; the core
reads either as never begun at once, delivered with the screen, and
sends no prompt (35, 43). The seam gains `tidy`, which the core calls
when a place goes, landed or dropped (137, 138), status naming what
the terminal kept and why; and `show`, which the core calls at each
settle that changes a stage's state with the stage, its state and the
count of the places on its base running, asking, waiting on their caller
and stalled, a failure logged and failing nothing, and nothing read
back (139). The clause list moves to blueprints f44ec91, adding 137 to
139 and S45 to S49.

### 13. The merge queue through sluice

- **submit**: `sluice submit <base> --no-wait --json` in the place, its
  environment stripped of every multiplexer's session (`HERDR_*`,
  `TMUX`, `TMUX_PANE`, `ZELLIJ*`), so sluice records the place's
  worktree as the submitter and pushes into no pane; never `--check`,
  never `wt submit`; a refusal is its exit and its words (C.6, core.6).
- **inbox**: `sluice inbox --json` in the place (133; sluice clause
  51): each record landed with its commit, failed with the merge
  queue's words, or in conflict with the files contested, and its
  `notice` field, the one-line words sluice gives the outcome, carried
  as the outcome's words, which start the fix round (112); a conflict's
  base revision is read from the record's `tested.on`, the revision of
  the tested combination's base (`tested: {combination, on,
  submissions}`), never from inside `notice`; the fix round's words are
  `notice` with the files contested and that revision. A withdrawal
  is never in the inbox; it is read from `sluice status` (99).
- **read-submission**: `sluice status <base> --json`, read once per
  repository in one command; the newest entry from the place's branch,
  landed as sluice records it with its landing commit (99, 101).
- **step**: `sluice step <base>`, run by each settle that finds the
  place's submission undecided; beside a live `sluice run` it finds the
  branch's lock held and does nothing, so no liveness is read (124,
  125; sluice clause 135).
- **base**: `origin/<base>` after one fetch where `.config/sluice.toml`
  lands the base on a remote, the local base otherwise (126, C.1).
- **The place's data**: the readiness, the teardown (`sluice release`
  in the place) and the base's ref, read from `.config/sluice.toml`
  once sluice is bound (core.3), handed to the place profile (decision
  9).

Every field read from sluice's JSON is checked against the pinned
sluice in `check-tools`, and Part C gains the commands and fields as
built (Part C's preamble).

### 14. The agent through Claude Code

- **command** (88, C.4): `claude --plugin-dir <plugin> [--model <m>]
  [--append-system-prompt <instructions>]`, the prompt as its argument
  under the printed command. The plugin is the binary's own, written
  under `$XDG_DATA_HOME/flume/plugins/claude/<version>/` once per
  version of flume, so an agent loads the plugin of the flume that
  started it. The environment is the host user's login environment,
  the settings' over it, and `FLUME_AGENT`, `FLUME_SECRET`,
  `FLUME_PLACE`, `FLUME_STAGE` (29). The identity is
  `flume:<repository-id>:<kind>--<name>:<stage>:<n>`, the repository's
  id the first twelve hex digits of the SHA-256 of its common git
  directory's path (33). The command carries the program's
  folder-trust question and the key that answers it, as data the
  terminal reads (34).
- **The agent's own record** is the session's transcript and the
  plugin's acks and marks. The plugin's SessionStart hook runs `flume
  ack started` with the start's identity, its secret, the plugin's
  version, the session's id, its transcript's path and its source, and
  records the session in the place's own folder under the start's
  identity, so every reading reads that start's record and no other
  (77, 93, 135). A plugin version other than the binary's is reported;
  a session herdr resumed bare loads no `--plugin-dir` and carries no
  `FLUME_*`, so it acks nothing. The hooks name the binary that wrote
  the plugin by its absolute path. The seam's readings are handed the place's folder with the
  identity (decision 15).
- **started** (135): the start's started ack. **received** (89): the
  start's received ack, which the plugin's UserPromptSubmit hook runs
  with the submitted prompt's digest when Claude Code submits it,
  queued prompts among them, so text typed and never submitted acks
  nothing; only where no ack can come, the transcript's user turn or
  queued `enqueue` record holding the prompt, a slash command as its
  `<command-name>` and arguments, whitespace collapsed, a transcript not
  yet written read as not received. **own-work**: background shells and tasks the transcript
  shows started and not finished (91). **asking**: the mark the
  PreToolUse hook leaves on `AskUserQuestion` and `ExitPlanMode`, and
  the permission-request or permission-prompt notification hook leaves
  where the pinned Claude Code has one, open until the transcript holds
  the answer, and cleared at UserPromptSubmit and SessionStart too, so
  a dismissed question never stays open (90). **last words** (43): the
  transcript's last message, or the reason the StopFailure hook
  records where the pinned Claude Code has it, a refused sign-in or a
  usage limit among them. Each answers unknown when
  the transcript cannot be read.
- **stop-short** (92): the text naming `flume needs "<words>"` and
  `flume needs --found "<words>"`, filled into every prompt's
  `{{needs}}` (28).
- **send-back** (115): the Stop hook runs `flume hook stop`, which asks
  the core's stop check and blocks the stop with its words when it
  answers; it never blocks a conversation's agent (109).
- **guard** (116): the PreToolUse hook on `Bash` refuses a command that
  runs `sluice submit`, `wt submit`, `wt merge` or `git push`, read as
  words of each command a shell line runs, its reason saying flume
  submits the place, and appends the act to the mark status reads
  (108). `git merge` into the place's branch is never refused: a
  conflict's fix merges the base in (112).
- **on-quiet** (121): the Stop hook, when it lets the agent stop,
  starts `flume settle <place> --on-quiet` detached, which reads the
  agent until the terminal reads it quiet, for up to 60 seconds, then
  settles; so the place moves on the moment its agent goes quiet
  (46, S29).
- **ended** (134): the SessionEnd hook marks the start's session ended, which
  the printed command's state reads (decision 12).

The hooks' entry, `flume hook <event>`, is the profile's code under its
profile directory; the command line hands it the core (core.1). Outside
an agent flume started (no `FLUME_AGENT`), every hook does nothing.

### 15. The seams' types, as this change leaves them

Beyond decisions 2 and 4: the agent seam's readings take the place's
folder with the identity, and the seam gains `ended` (134);
the terminal seam gains `session`, the caller's session as the
terminal names it, for a launch's default recipient (7); the launch
and the held state gain the recipients and each hold's words (7, 8,
130); `AgentCommand` gains
the folder-trust question and its answer; `StageState` gains
`WaitingForRoom`, and `Starting` (decision 12a); the agent seam gains
`started` (135) and the terminal seam `screen` (136); `AgentCommand`
gains the program's kind, the terminal seam's `open` its held and
not-come-up answers with the screen, and the seam gains `tidy` (138)
and `show` (139) (decision 12b); the held state gains the place's priority and, per
stage, since when it waits for room; `Host` gains `bound` and `room`;
the resolver lists the place profiles it holds and binds a place
profile with the merge queue profile's data. Every type round-trips
through JSON; every answer only gains fields (61).

### 16. The command line

One binary, clap, every command an answer from the one core, printed
for a person or as JSON with `--json` (60, 61, section 9):

| command | act |
|---|---|
| `launch --kind <k> --name <n> [--base <b>] [--brief <text> \| --brief-file <f>] [--kinds <file>] [--hold <stage>=<words>]… [--hold-submission] [--recipient <r>]… [--mark <m>] [--priority <n>] [--agent [<stage>.]<field>=<value>]…` | 2 to 8, 32, 43, 70 |
| `status [<place>]` | 56 to 58, 70 |
| `prompt <place> <stage>` | 29 |
| `tell <place> <words> [--within <time>]` | 53 to 55 |
| `stop`, `finish`, `drop`, `done <place> [--force]` | 51, 109, 52, 114 |
| `retry <place> [--stage <s> \| --submission] [--words <w>] [--force]` | 49, 50 |
| `tidy [<place>]` | 63 to 68 |
| `settle [<place>] [--on-quiet]` | 46, 119 to 121 |
| `needs [--found] <words>` | 41, C.4 |
| `priority <place> <n>` | 70 |

A place is named by its location, any word holding a `/`, or by
`<kind>--<name>` in the repository the command runs in. The repository
is the one the command runs in, or `--repository`: the directory above
the git directory its worktrees share, so from inside a place it is
still the repository the place was launched in. A launch naming no base
takes the repository's default branch, `origin/HEAD`'s where it names
one. `settle --on-quiet` reads the place's current agent through status
once a second until it is waiting or stopped, not asking and with no own
work, for up to 60 seconds, then settles; `needs` refuses before the
core is built when `FLUME_AGENT` or `FLUME_SECRET` is missing. Exit
status 0 for an answer, 1 for a refusal, 2 for words that are no
command. `flume hook <event>` and `flume mcp` are the plugin's.
The engine is built per command over the resolver, the host's terminal
and store, the real history and host, and `Defaults`: Claude Code with
no model where no layer gives settings (32, decision 16a), three
send-backs (23).

### 16a. An agent's settings, layered (record 0017)

An agent's program, model, instructions and environment are resolved at
each start from six layers, each setting from the first that gives it
(32): the launch's; the host's for the repository and kind; the host's
for the kind; the repository's for a kind it does not declare; the
kind's declaration; `Defaults`. Within a layer a stage's own setting
comes before the kind's. `Settings::under` already merges `env` by
variable and takes the rest whole; the resolver folds the layers through
it, most general first, and keeps, for each setting and each
environment variable, the layer that gave it.

- **The launch.** `--agent <field>=<value>` gives every stage's agent a
  setting, `--agent <stage>.<field>=<value>` one stage's, the field one
  of `program`, `model`, `instructions`, `env.<NAME>` and `beside`. The
  launch and its held state carry the kind-level settings beside the
  per-stage map they carry now (7, 8).
- **The host.** The store profile reads `[kinds.<kind>.agents]` and
  `[repos."<repository>".kinds.<kind>.agents]` from the host's
  configuration, each with `.stages.<stage>` tables, the repository
  keyed by its location as a launch names it, `~` expanded. `Host`
  gains `settings(repository, kind)`, both host layers as written, read
  again at every start, never cached in held state (32, C.7). Settings
  for a kind or a stage the place does not run are ignored.
- **The repository and the kind.** weir's crate, at the pin that reads
  them (weir 52, 57, C.4), gives a kind's `agents` table with its
  `stages` tables, and a settings-only `[kinds.<kind>]` table apart
  from the kinds; the kinds profile maps both onto the seams' types.
  The launch keeps, in the place's held state, the repository's
  settings for the kind the name resolved to and the kind's own, as it
  read them at the base's revision, so a later start reads them from
  held state, never from the base again (8, 20, 21). The repository's
  settings-only tables for a kind apply to whichever declaration the
  kind's name resolves to, a kind the launch's own `--kinds` file
  declares included (32, C.2; record 0017).
- **Expansion.** Host-layer `env` values only: a leading `~` is the host
  user's home and `$VAR` or `${VAR}` a variable of the login
  environment the agent profile already reads (29); a variable it lacks
  refuses the start naming it. Nothing in a repository's file or a
  launch is expanded.
- **Refusals.** At the launch, every stage resolved with the host's
  configuration as it stands then: a program the resolver binds no
  agent profile for, or the host's terminal cannot run, refuses the
  launch naming the program, the stage and the layer (4); so does an
  `env` in a repository's `.config/flume.toml`, and settings for a stage
  the kind does not hold. At a later start, a program or a missing
  variable the host's configuration has changed to leaves the stage
  refused, naming it and its layer, with no agent started, until a
  retry (27).
- **Status.** The current agent's `settings` gain `from`: for each of
  `program`, `model`, `instructions` and each `env` variable, its layer
  (`launch`, `host-repository`, `host-kind`, `repository`, `kind`,
  `default`), the value as that layer wrote it, before expansion (56,
  61: a field gained, none removed). An agent already started keeps the
  settings it started with in its start's record; a change to the
  host's configuration shows at the next start.

### 17. flume's Claude Code plugin

`plugins/claude/`, a moon project of its own (core.21), listed in a
marketplace at `.claude-plugin/marketplace.json` so any session can
install it, and built into the binary for the agents it starts:

- `.claude-plugin/plugin.json`;
- `hooks/hooks.json`: SessionStart, UserPromptSubmit, PreToolUse,
  PostToolUse, Stop and SessionEnd, and StopFailure and the
  permission-request or permission-prompt notification hook where the
  pinned Claude Code has them, each `flume hook <event>` by the
  binary's absolute path (decision 14; record 0016). Every hook exits 0
  and never holds up a start or a turn; the acks skip a subagent's
  input, and the guard refuses a subagent's command as its agent's;
- `commands/launch.md`: `/launch <brief>`, which has the session choose
  the kind and a name with the operator and run `flume launch --json`,
  then report the place and how to follow it with `flume status`;
- `.mcp.json`: `flume mcp`, a stdio server with one tool, `status`,
  answering what `flume status --json` answers, settling nothing
  (57, 60).

Its project's check-fast tasks: `validate` (`claude plugin validate
--strict` by the pinned Claude Code), `test` (each hook run by the built
binary against fixture transcripts and hook inputs on a scratch state
directory: the stop check, the guard, the asking mark, the on-quiet
settle, a session outside flume), and the root's `format` and
`marketplace` over its files.

### 18. The conformance suite

- Each seam's scenarios, generic over its trait, run against the real
  profile in `crates/flume/tests/<seam>_<profile>.rs` with
  `flume-conformance` as a dev-dependency, in the layer that can run the
  tool (106).
- Engine scenarios for S35, S36 and S37, and for 70, 131 and 133; S1,
  S19 and S34 rewritten as the statement reads them.
- The clause list moves to e5f92f6: Part A gains 129 to 132, Part B
  126 to 128, 133 and 134, the scenarios run to S38; 129, 132 and S38
  are named in the list as the next change's, and the coverage check
  passes over every other clause, invariant and scenario.
- The walkthroughs: from the built binary, on scratch repositories with
  a local remote, sluice bound and landing: a stand-in for Claude Code
  in `check-tools` (an executable first on `PATH` that writes a
  transcript as Claude Code does, runs the plugin's hooks as
  `hooks.json` names them, and acts a script: commit, tick tasks,
  write a report, `flume needs`), and the real Claude Code in
  `check-live`.

## Risks / Trade-offs

- [The sluice profile needs `sluice inbox`] → sluice's change
  `outcomes-are-delivered` is built first; group 7 pins its release.
- [No terminal keeps watch in this change] → a place moves on when its
  agent goes quiet (121), and after the last stage, at the next command
  that acts or `flume settle` (119); a landing is read then, not within
  a watch's bound. The herdr plugin's watch removes the gap in the next
  change.
- [The on-quiet settle may run before the terminal reads the agent
  quiet] → it reads again for up to 60 seconds inside its own command
  (decision 14), and a later command settles what it missed.
- [An archived change sent back to code] → the code prompt moves it
  back before ticking its fix; verify archives it again (decision 7).
  A prompt is guidance, not a check: the deliverable's reading is the
  check (24).
- [Claude Code's transcript format is not a contract] → started and
  received come from the plugin's own acks (record 0016), and the
  transcript is read for the rest at the pinned version only, the
  profile's tests holding fixture transcripts of that version.
- [Which dialogs Claude Code shows before any hook] → the bypass
  warning, a sign-in, a first run's setup and an MCP servers' approval
  are read as showing before SessionStart; a start held at one stays
  starting with its screen until the stage's limit (35), so a dialog
  measured to come after the hook costs nothing but the limit.
- [Which program herdr's `agent start` ran] → herdr types the kind's
  name, so a shell startup file can put another `claude` ahead of the
  start's own folder on `PATH`; the profile reads the pane's foreground
  program from the host's process table and refuses the start naming
  what ran (record 0018), so it is seen at the first start.
- [herdr headless in the gate] → the herdr profile's tests start a
  server of their own on a private socket; if the pinned herdr cannot,
  those tests move to `check-live`.
- [`check-live` needs the network and a signed-in Claude Code] → it
  stays out of the sandboxed gate and is run by hand in the devenv
  shell.
- [Ten groups in one change] → each group lands its own checks green
  and names what it leaves to the next; the seams' changes come first,
  so the profile groups code against settled traits.
