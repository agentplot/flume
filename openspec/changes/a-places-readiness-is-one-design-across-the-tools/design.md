# Design

## In one page

A working copy is made ready for work by four steps, wherever it is
made. The first is the toolchain the repository's policy names, which
activates its system tools. Then come the install of the tools its
`.prototools` pins, the runner's check of its own version, and the
runner's setup. sluice already runs all four, in that order, before
every candidate's run and every `sluice exec` of the gate's checks
(`moon.rs`, `plan`). flume runs two of them, copied by hand into its
sluice profile. The install it left out is what deneb's place was
missing.

The design gives each part one owner:

- **The repository** declares which tools it needs: the toolchain and
  the runner in `.config/sluice.toml`, and the versions in `.prototools`
  and the runner's own files.
- **sluice** owns how a working copy is equipped. It equips every
  working copy it runs anything in by one sequence, and offers the
  sequence as one command, `sluice equip`, for a working copy it did
  not make.
- **flume** owns when a place is made ready, at its launch and at the
  retry of a first stage whose readiness failed, and what a failure
  does (12, S16). A place's readiness is `sluice equip`, asked at the
  revision the launch read, just as a place's teardown is already
  `sluice release`.
- **crew and Boomdrive** make nothing ready. They ask flume for places
  and sluice for branches, as they do today.

The held point fix is dropped unlanded. sluice's change comes first,
then flume's pin moves to the release that has `sluice equip`, and then
this change is built.

## Context

### What showed it

On deneb on 2026-11-05, Boomdrive's runner cloned a moon repository
fresh, and flume launched a place there. Its readiness ran
`devenv print-dev-env`, which passed: that command activates what
`.prototools` pins and installs none. It then ran `moon setup`, which
failed because the moon `.prototools` pins was not installed. The place
`fix--a-places-readiness-installs-its-repositorys-pinned-tools` holds a
fix that copies sluice's install into flume (0a3f138 and the commits
after it). It is held, and the user asked for the analysis:

> The thing is, Flume's job is to make the place ready. Sluice creates
> the branch, I believe, right? Flume was actually creating the work
> tree. I feel like you just gave me a solution without actually doing
> the analysis.

> I don't want to feel like, too often, we're doing these point fixes
> and we're getting the solutions wrong ... We can't just be myopic,
> thinking about the one repo. We need to look at the related tools as
> a whole.

### What each tool makes, and what it runs to make it ready

The code is read at flume's main 6c38bfb, `agentplot/sluice` 571c83f6,
`afterthought/crew` teamless/deneb 83a5cec0, `agentplot/boomdrive`
c5396d9, and `afterthought/kit-standard` 20468ea. deneb runs Boomdrive
at dc000d9, which differs from c5396d9 in its names and its reading of
older marks and records, and not in what it runs.

**flume.** A place is a worktree at
`$XDG_DATA_HOME/flume/places/<id>/<kind>--<name>` on a new branch
`<prefix>/<name>`. flume makes the branch and the worktree in one
command, `git -c core.hooksPath=/dev/null worktree add --quiet -b
<branch> <location> <revision>`, at the base's revision. Under sluice
that revision is `origin/<base>` after one fetch
(`place/git_worktree/mod.rs`, `make`; C.1). Its readiness is a list of
steps that the sluice profile declares (`merge_queue/sluice/mod.rs`,
`for_place`) and the resolver hands to the place profile
(`profiles.rs`, `BoundPlace::data`). There are two: `[tests] toolchain`,
whose output is kept at `<git-dir>/flume/toolchain.sh` and sourced
before each later step, and `moon setup`, where `[tests] runner` is
`moon`. Each runs under `bash -c`, with the environment cleared to the
host user's login environment (`git_worktree/mod.rs`, `run`, `Login`).
Teardown is `sluice release` in the place. The core passes the launch's
base revision to `ready` (`launch.rs`, `acts.rs`), but the profile
ignores it (`ready(&self, place, _revision)`): `policy()` reads
`.config/sluice.toml` at the local `refs/heads/<base>` first, then at
each remote's tracking copy, at the moment of each call.

**sluice.** sluice makes no branch for a place. Its candidates are bare
commits (`local_refs.rs`, `form`). It moves its base only at a landing.
It makes a bolt's branch only when asked, through `sluice branch create`
(`sluice-core/src/branches.rs`). On a base that takes pull requests, it
pushes a submitter's branch to the remote at the submission
(`pushed_refs.rs`, `pulls.rs`). It makes three kinds of working copy and
adopts a fourth:

- A candidate's place: `git -c core.hooksPath=/dev/null worktree add
  --detach` under `<common>/sluice/.places/` (`places.rs`,
  `materialize`).
- A short-lived reading tree for the affected query, under
  `<common>/sluice/.runs/reading-<pid>/` (`checks.rs`, `ReadingTree`).
- A preview's place under `<common>/sluice/.previews/` (`places.rs`;
  `preview.rs`).
- A session's own worktree, adopted by `sluice exec`: sluice writes a
  record and holds ports for it, and makes nothing in it (sluice record
  0018, "a place sluice equips and never removes").

sluice readies a candidate's place and a session's worktree by one
sequence, in `Checks::start` and `Moon::plan`. Every step runs outside
the executor, with the environment cleared to a kept list:

1. `[tests] toolchain`, run in the working copy, its output kept as
   `toolchain.sh` in the run's directory (`checks.rs`, `toolchain`). Each
   later command runs as `bash -c 'source "$0" >&2; exec "$@"'` over it.
2. `proto install --config-mode upwards`, where the working copy holds a
   `.prototools` (`moon.rs`, `install`).
3. `moon --version`, refused below 2.6 (`check_version`).
4. `moon query tasks`, and the affected query where there is a verified
   revision to compare with.
5. `moon setup`. Its comment says why it runs where it does: "The
   workspace's toolchains, installed here, outside the executor, where a
   download and a write to the user's home are allowed: the run inside
   only uses them."

The run itself, `moon exec … --ci`, then runs under the executor. The
same sequence, without setup, runs in the submitter's worktree at every
`sluice submit` to a moon branch (sluice record 0050; `checks.rs`,
`kept_out`). Two of sluice's own paths run less:

- A preview forces the command runner and gets the toolchain alone
  (`preview.rs`, `preview_request`).
- `sluice exec <branch> -- <command>` does the same (`session.rs`,
  `session_request`).

`sluice release` runs `[tests] release` under the toolchain where the
worktree was adopted. Where it was not, it says "nothing to release"
and exits 0 (`exec.rs`).

**crew** (teamless/deneb). crew makes one worktree itself, a landing's.
It runs `git worktree add -q --no-checkout <kit>/places/land-<bolt>
bolt/<bolt>` and `read-tree`, then `wt submit main --no-wait --json
--no-remove` there (`plan.py`, `SUBMIT`), and removes the worktree with
`git worktree remove`. It readies nothing. sluice's submission reads
what it needs in that worktree (record 0050). Units, chores, fixes and
role runs are flume places: crew writes requests to `requests.rec`,
and Boomdrive's runner launches them (`runner.py`). Bolt branches are
made with `sluice branch create` (`plan.py`, `crew bolt start`).
`kit.py` says: "crew readies no place: a unit's, chore's and fix's
places are Flume's, readied by its kind and the kit's own files". Its
rule plan.17, crew making a worktree ready, was retired on 2026-11-03.

**Boomdrive's runner.** It keeps one bare clone per configured
repository at `<data>/repositories/<owner>/<name>/<name>.git`. A gitfile
sits beside the clone, and the container is the folder `flume launch
--repository` names (`clone.rs`, `container`, `bare`). It fetches the
clone on a two-minute pass, and moves the default branch only by fast
forward, only while no worktree holds it. It keeps one detached view at
`~/Code/<owner>/<name>` (`view.rs`). It runs nothing in either to make
it ready: its only programs are git, flume, sluice and herdr, plus their
`--version`. Its `Keeping::Ready` means "the clone is made". It asks
flume `flume --json launch --repository <container> --kind … --name …
--mark boomdrive:<id>` and reads flume's status (`flume.rs`). Its rule
core.14 says "The runner's clone of a repository is the host's one
clone of it: Flume, sluice and what a launch runs may work in its
container". No rule of its own gives it, or anyone, the job of readying
a working copy.

**The tools they call.**

- **devenv.** `devenv print-dev-env` evaluates `devenv.nix`, realises its
  shell and prints a script holding the environment and `enterShell`. It
  runs none of devenv's tasks. `devenv shell [-- <command>]` runs the
  tasks first, then `enterShell`. In this repository those tasks are
  `devenv:files`, `flume:proto:install` (`rustup toolchain install`,
  then `proto install --config-mode upwards`), `devenv:git-hooks:install`
  and `devenv:treefmt:run`, as seen running in this place.
- **proto.** `proto activate --config-mode upwards` puts the pinned
  versions on the path and installs nothing. `proto install --config-mode
  upwards` installs every version `.prototools` pins into `PROTO_HOME`,
  which is the repository's own, `<common>/proto` (kit:toolchain.4), and
  so is shared by every worktree of the repository.
- **moon.** `moon setup` installs the workspace's toolchains: "Setup the
  environment by installing all toolchains", as moon 2.6.1 describes it.
  It needs moon installed first, which only proto does.
- **worktrunk.** `wt switch --create` runs a repository's create and
  start hooks (`.config/wt.toml`), and `wt remove` runs its remove hooks.
  No tool here makes a worktree through worktrunk. flume and sluice make
  theirs with plain git and hooks off (C.1; sluice record 0013). crew
  removes through `wt remove` in its tidy, and lands through `wt submit`,
  which is sluice's `wt-submit`.

The repositories install their pins in two different ways:

- flume's, sluice's and Boomdrive's `devenv.nix` install in a devenv
  task. Their `enterShell` "Only activates: sluice sources this inside
  its executor, where nothing may be installed".
- crew's `enterShell` runs `proto install` itself whenever a pin is
  missing (crew `devenv.nix`, `enterShell`).

The kit standard says where the homes are (kit:toolchain.4), and that
an agent runs tools through the shell per command (kit:toolchain.5). It
does not say who installs the pins.

### Who makes the branch, the working copy and its readiness

| working copy | its branch | the working copy | made ready by, today |
|---|---|---|---|
| a flume place | flume: `worktree add -b` at the base's revision; sluice pushes it to the forge at the submission where the base takes pull requests | flume's place profile | flume: the toolchain and `moon setup`, copied from sluice; no install |
| sluice's candidate | none: a bare commit; the base moves at the landing | sluice: a detached worktree | sluice: toolchain, install, version, queries, setup |
| sluice's reading tree | none | sluice: a detached worktree | sluice: toolchain, install, version, queries |
| a session's worktree under `sluice exec` | its maker's | its maker: flume, a person, an agent | sluice, at each run: the whole sequence; under `-- <command>`, the toolchain alone |
| a submitter's worktree at `sluice submit` | its maker's | its maker | sluice: toolchain, install, version, queries |
| sluice's preview | none: a landed revision | sluice: a detached worktree | sluice: the toolchain alone |
| crew's landing worktree | the bolt's, levelled by crew; the bolt made by `sluice branch create` | crew: `git worktree add --no-checkout` | nothing; sluice's submission reads what it needs |
| Boomdrive's clone and view | the default branch, fast-forwarded | Boomdrive: a bare clone and a detached view | nothing |
| a person's checkout | the person | the person | the person's `devenv shell` or direnv |

So, as the user believed, sluice makes branches: a bolt's, when crew or
Boomdrive asks, and the published copy of a place's branch on the forge.
But a place's own branch, and its worktree, are flume's, made together
by one `git worktree add -b`.

What readies a working copy is written in five places, and they
disagree:

1. sluice's whole sequence, for candidates and session runs.
2. sluice's toolchain alone, for previews and `exec -- <command>`.
3. sluice's sequence without setup, at a submission.
4. flume's toolchain and setup, without the install.
5. Each repository's `devenv.nix`, which installs in a task or in
   `enterShell`.

deneb's place failed in the gap between the first and the fourth.

### Rules relied on

core.1: proto, moon and devenv are named only in the sluice profile and
its tests, and `sluice` only in profiles. core.2: the sluice profile
declares, the resolver hands over, and neither profile calls the other.
core.3: the sluice profile reads `.config/sluice.toml` once bound. core.7:
the base's revision is the merge queue's. core.13: a launch waits on no
verdict, and the equip is no submission. core.14: the JSON form only
gains fields, and readiness's `steps[]` keep their shape. core.16:
nothing of flume's is written into a repository's content. core.17:
equipping twice changes nothing more. core.22: the sluice the tree pins
is the one the profile is checked against. core.35: the site names
sluice, proto and moon as tools flume integrates with. Group 1 draws
core.47 and amends core.3's details, once the user accepts record 0036.
The one rules page is `docs/architecture/core.md`. Where flume has no
rule, the kit standard's code profile binds (kit:specs.3,
kit:testing.5).

## Goals / Non-Goals

**Goals:**

- One owner for each part of readiness, across flume, sluice, crew and
  Boomdrive: the repository declares, sluice equips, flume decides when,
  and nobody else readies anything.
- A place equipped exactly as sluice equips its own working copies,
  with no step of sluice's runner copied into flume (11, record 0014).
- A place's readiness read at the revision its launch read, at the launch
  and at a retry (12, 20).
- A fresh clone's place installs what it pins before anything runs moon
  (S89).
- Status names each of sluice's steps with its outcome and words, as it
  names flume's steps today (12, S16; core.14).

**Non-Goals:**

- sluice's build of `sluice equip`, and its own paths taking it. That is
  sluice's change, named in decision 7 and launched there once the user
  approves.
- crew's, Boomdrive's and the kit standard's amendments, also named in
  decision 7.
- The teardown. `sluice release` stays as it is.
- A language toolchain's own components, such as `rust-toolchain.toml`'s,
  which this repository installs with `rustup` in a devenv task. Those
  stay the repository's, declared where its runner's setup reads them
  (see Risks).
- Readiness for a repository that binds no merge queue, which stays
  nothing (11).

## Decisions

### 1. The owners

- **Which tools** a working copy needs is the repository's:
  `[tests] toolchain` and `[tests] runner` in `.config/sluice.toml`
  (kit:gate.2); the pins in `.prototools` (kit:toolchain.2); and moon's
  `.moon/toolchains.yml`.
- **How** a working copy is equipped for its runner is sluice's: its
  runner profile's install, version check and setup, under the
  toolchain. The record 0018 that admitted session runs already calls an
  adopted worktree "a place sluice equips".
- **When** a place is made ready is flume's. It does so at the launch,
  and at the retry of a first stage whose readiness failed, as its launch
  read it (12). What a failure does is flume's too: the first stage waits,
  or a fix starts with the step and its words in its prompt (12, S16).
- **Nobody else** readies a working copy. crew's landing worktree is a
  submitter's worktree, read by sluice's submission. Boomdrive's clone
  and view are never worked in: Boomdrive asks flume for places, and
  flume asks sluice to equip them.

| working copy | its branch | the working copy | made ready by |
|---|---|---|---|
| a flume place | flume | flume | flume, by `sluice equip` at the launch's revision |
| sluice's candidate, reading tree, preview | none | sluice | sluice's equip |
| a session's worktree under `sluice exec` | its maker's | its maker | sluice's equip, at each run, the command's included |
| a submitter's worktree at `sluice submit` | its maker's | its maker | sluice's equip, up to the queries |
| crew's landing worktree | the bolt's | crew | sluice's submission, as above |
| Boomdrive's clone and view | the default branch | Boomdrive | nothing: not worked in |
| a person's checkout | the person | the person | the repository's `devenv shell` |

### 2. Readiness's steps

| step | what it does | declared in | run by |
|---|---|---|---|
| toolchain | activates the system tools; its output is the environment of the steps after it | `[tests] toolchain` (kit: `devenv print-dev-env`) | sluice's equip |
| pinned tools | installs what the working copy's `.prototools` pins, moon among them, into the repository's proto home | `.prototools` | sluice's moon runner: `proto install --config-mode upwards` |
| runner's check | refuses a moon below 2.6 | sluice's runner | `moon --version` |
| runner's setup | installs the workspace's toolchains | `.moon/toolchains.yml` | `moon setup` |

These are not readiness, and each has its own owner:

- **What a check brings up**, such as services, stacks and a container
  engine: the check itself (sluice 31, 84; sluice record 0066, "a check
  makes itself ready").
- **Ports and a session's scratch:** `sluice exec`.
- **Commit hooks:** prek's shim in the common hooks directory, which
  works in a worktree no shell has entered (kit:agents.4).
- **moon's cache link** (`.moon/cache/outputs`): `enterShell`, at every
  sourcing of the toolchain.
- **A project's dependencies:** moon's tasks.
- **An agent's settings, plugin and folder trust:** flume's agent
  profile (C.4).

flume's readiness gains no step of its own (13).

### 3. Where each step is read, and at which revision

- **Which steps run** is read from the policy at the revision the launch
  read (20). flume names that revision to sluice as `--at <revision>`,
  so a retry of the first stage reads the steps as its launch did (12).
  This is what the code fails to do today. The core passes the revision,
  but `policy()` reads whatever the local base holds at the moment of the
  call, and the local base can lag the `origin/<base>` the place was cut
  from.
- **What each step installs** is what the place holds: its
  `devenv.nix`, `devenv.lock`, `.prototools` and
  `.moon/toolchains.yml`. At the launch that is the same revision. At a
  retry it includes the agent's commits, so a fix that bumps a pin is
  readied with the pin it works under. sluice does the same with a
  candidate: its policy comes from the branch, and its toolchain runs in
  the candidate's tree (sluice 56; `moon.rs` reads a change to
  `.prototools` or `devenv.*` as touching every check). The held fix
  instead reads `.prototools` from the base at each call. At a retry
  that installs pins the place may no longer use.
- **How the runner equips** is sluice's code, at the release flume pins
  (core.22).

### 4. `sluice equip`, as sluice's change would give it

sluice's change builds it. Its words are proposed here because flume's
build depends on its answer:

- `sluice equip <branch> [--at <revision>] [--json]`, run in a working
  copy. It reads `[tests]` of the branch's policy at `<revision>`, or at
  the branch's current revision where none is named, as a preview reads
  its previewed revision's `[tests]` (sluice record 0045).
- It runs the same steps, through the same code, as a candidate's run,
  in the working copy and outside the executor: the toolchain, then the
  runner's install, version check and setup. Under the command runner
  that is the toolchain alone, and with no toolchain and no runner that
  equips, nothing.
- It keeps nothing after it ends. Its toolchain script lives in a run
  directory of its own, removed at the end, as the reading at a
  submission's is.
- It adopts nothing. It holds no ports, writes no session record, counts
  against no bound and takes no turn, because it is no run. A later
  `sluice release` in that working copy still says "nothing to release".
- `--json` answers `{"steps": [{"step", "passed", "words"}]}`, each
  step named as sluice names it: `toolchain`, `proto install`,
  `moon --version`, `moon setup`. It stops at the first step that fails,
  exiting 1, and exits 0 when every step passed, none run included. A
  refusal, such as a branch it cannot read, is `{"error": {"message"}}`,
  as sluice's other `--json` refusals are.
- sluice's own paths all equip through it: a candidate's run, both forms
  of `sluice exec`, a preview, and a submission's reading, which stops
  before setup. So a preview and an `exec -- <command>` gain the install
  and setup they lack today.

The name is `equip`, the word of sluice's record 0018, and not `ready`.
The user removed the `ready` probe from sluice: "There is no fucking
ready command" (sluice record 0066). That probe was a command the
repository wrote, asked whether a host could run its checks. `equip`
is neither. It is sluice's own preparation, which already runs before
every run, offered for a working copy sluice did not make.

### 5. flume's side

- **The sluice profile.** `for_place` gives the place's readiness as
  one step, `sluice equip <base> --at <revision> --json`, with the
  profile's reading of its answer, `equipped`. It no longer gives the
  toolchain or a setup, or reads `[tests] toolchain` or
  `[tests] runner`. `equipped` reads `steps[]` as the readiness's steps,
  each with its `step`, `passed` and `words`, and the words are cut as
  flume cuts any step's. An answer with no `steps` is one failed step,
  `sluice equip`, with sluice's `error.message`, or its errors where it
  gave none. A sluice that cannot run is one failed step with the reason.
  `teardown` and `base_remote` stay.
- **The resolver** (`BoundPlace` in `profiles.rs`) hands the place
  profile the readiness step with its reading, built for the revision
  `ready` is given, as it hands the teardown today. Neither profile
  calls the other (core.2). The reading is a function the sluice profile
  owns, handed as data, so the place profile reads nothing of sluice's
  form. The paths not taken: the resolver running `sluice equip` itself
  needs the login environment the place profile keeps; and the place
  profile parsing sluice's form puts sluice's shape in a profile of
  git's.
- **The place profile** (`git_worktree/mod.rs`) runs the step as it runs
  every step: `bash -c` in the place, with the environment cleared to
  the host user's login environment (11, 79). Where the step comes with
  a reading, its readiness is the steps the reading gives of what the
  step printed. `ready` takes the revision it is given, so a retry
  equips at the launch's revision. `Step::toolchain`,
  `prints_environment` and `TOOLCHAIN` go: nothing of a place's
  readiness is kept in its git directory. A place made before keeps its
  `flume/toolchain.sh` until its tidy removes it with the worktree.
- **The core** is unchanged. `launch.rs` and `acts.rs` already pass the
  launch's base revision, and `Readiness`, `ReadyStep`, `{{unready}}`
  and `StageState::WaitingOnReadiness` keep their shape (core.14).
- **Status and prompts.** The person's form prints a line per step, as
  it does today, now `toolchain`, `proto install`, `moon --version`,
  `moon setup`. `{{unready}}` names the step that failed and its words
  (12).

### 6. What each tool stops doing, and starts calling

| tool | stops | starts |
|---|---|---|
| flume | reading `[tests] toolchain` and `[tests] runner`; naming `moon setup`; keeping `flume/toolchain.sh`; reading the policy at the local base at each call | asking `sluice equip <base> --at <revision> --json` as a place's readiness; reading its steps |
| sluice | readying a preview and `exec -- <command>` with the toolchain alone; keeping the sequence in three variants | one `equip` for every working copy it runs in; offering it as `sluice equip` |
| crew | its records and spec saying crew readies a worktree; its `enterShell` installing pins | nothing new: its landing worktree is a submitter's |
| Boomdrive | nothing | nothing; its record 0009 says a host's setup installs the host's tools, never a repository's pins |
| a repository's `devenv.nix` | installing in `enterShell` (crew's) | nothing: the install task stays for a person's or an agent's `devenv shell` |

### 7. The statements' clauses, for the user to approve

Each statement is amended on agentplot/blueprints main before its
tool's build reads it (core.19). Numbers are the next free ones at
blueprints ebfbc0d, given the changes in flight. flume's 180 to 185 and
S84 to S88 are taken by the stream and pool changes. sluice's 142 to 146
and S69 to S71 are on blueprints' `research/sluice-signs-its-landings`.
A number taken first by another landing moves to the next free one.

**flume, `design/flume/requirements.md`:**

- **readiness** (section 3): "the steps that equip a worktree for work
  before work starts in it: the tools its repository declares, installed
  and set up as its merge queue equips the working copies of its own
  runs."
- **11**: "flume makes a place ready only by its merge queue's equipping
  of it, the steps the merge queue runs to equip the working copies of
  its own runs, asked at the revision the launch read (20) and run in
  the place with an environment that carries nothing of the caller's
  session. A repository that binds no merge queue has nothing run. flume
  names no repository's script, runs no readiness step of its own,
  assumes no tool, and requires no version of any."
- **20**, its first sentence: "A repository's kinds, its lock (155) and
  the revision its readiness is asked at are read from its base's
  revision at the launch." Its last sentence: "None is read from a
  place's tree or from a change not committed; what a readiness step
  installs is what the place it runs in holds, which at the launch is
  that revision."
- **79**: "**ready**: given a place and the revision its launch read,
  each step its merge queue equips it with at that revision, run in the
  place in order with an environment that carries nothing of the
  caller's, each with its outcome and its words, stopping at the first
  that fails (11, 12)."
- **C.1**, the ready column: "where sluice is the merge queue,
  `sluice equip <base> --at <revision> --json` run in the place in the
  host user's login environment, the revision the launch read, each of
  its `steps[]` one step of the readiness with its outcome and words
  (C.6); nothing where no merge queue is bound". The sentence "What the
  toolchain step makes is kept in the place's own git directory (C.7)"
  goes.
- **C.2**, the `merge_queue` line's comment: "reads .config/sluice.toml
  for where the base lives, and has sluice equip and release each place
  (C.1, C.6); none when omitted, and a kind that lands is refused (4)".
- **C.6**: "Readiness and where the base lives (C.1) are read from the
  same `.config/sluice.toml`" becomes "Where the base lives (C.1) is
  read from the same `.config/sluice.toml`. A place's readiness is
  sluice's equipping of it, `sluice equip`, which reads that policy at
  the revision flume names and runs in the place what sluice runs before
  its own runs (sluice 147)." The place row becomes: "`sluice equip
  <base> --at <revision> --json` in the place as its readiness, its
  `steps[]` each `{step, passed, words}`, exiting 1 where one failed; an
  answer with no `steps` read as one failed step, `sluice equip`, with
  its `error.message`; and `sluice release` as its teardown, handed to
  the place profile through the binding". "What the profile reads of
  sluice 0.55.0" names the release flume pins.
- **C.7**: "What readiness makes for a place, its toolchain among it, is
  kept in the place's own git directory, `git rev-parse --git-dir` in
  the place, and goes with the worktree" becomes "flume keeps nothing
  that a place's readiness makes: what sluice's equipping installs goes
  where the repository's own tools put it, under homes the repository
  sets."
- **S89** (added): "A repository is cloned on a host for the first time,
  its `.prototools` pinning a moon nothing on the host has installed,
  and its toolchain activating the pins and installing none. A change is
  launched there. Its place's readiness, sluice's equipping of it,
  installs what the place pins, checks moon's version and runs moon's
  setup, each step named in status as it passes, and construct starts
  (11, 12, 20, 79, C.1, C.6)."

**sluice, `design/sluice/requirements.md`:**

- **147** (added, in B.1 after 35): "**equip**: given a working copy
  and a revision of a branch, the working copy made ready for the
  branch's runner as every place sluice runs in is made ready. First the
  toolchain the branch's policy at that revision names is run in the
  working copy, and its environment is kept for the steps after it. Then
  come the runner's install of the tools the working copy pins, its
  check of its own version, and its setup. Each step runs outside the
  executor, with an environment cleared as a run's is. Each step's
  outcome and words are given, stopping at the first that fails. Every
  working copy sluice runs anything in is equipped this way: a
  candidate's, a session's, a preview's, and a submitter's, which last
  is read before any setup. So is a working copy sluice did not make, at
  its caller's asking, which sluice adopts no further: it holds no ports,
  writes no record and counts against no bound. What a check needs
  beyond its tools, it brings up itself (31, 84)."
- **33**: "a place holding exactly that revision, with the environment
  the repository declares" becomes "a place holding exactly that
  revision, equipped (147)".
- **84**, after "sluice runs no readiness command the repository writes,
  before a run or after one": "What equips a place is the runner's own
  (147): the toolchain the policy names, then the runner's install and
  setup. It is never a command the repository writes, and never a
  probe."
- **C.1**, the git worktree row: "nothing run in it" becomes "no hook run
  in it, equipped before its checks (147)".
- **C.2**, the moon row's run column begins: "equipped first (147), by
  `proto install --config-mode upwards` where the working copy holds a
  `.prototools`, `moon --version`, refused below 2.6, and `moon setup`,
  under the toolchain's environment and outside the executor; then …".
- The commands: `sluice equip <branch> [--at <revision>] [--json]`, as
  decision 4 has it.
- **Section 9**: "Places are managed by worktrunk, which runs a
  repository's hooks and tethers processes to a place" becomes "Places
  are plain git worktrees with no hook run (record 0013); worktrunk
  drives sessions alone (C.5)."
- **S72** (added): "A repository is cloned on a host for the first time,
  its `.prototools` pinning a moon nothing has installed. In a session's
  worktree, `sluice exec main -- moon run app:test` installs the pins and
  runs moon's setup before the command. `sluice preview main <revision>`
  does the same before its command, and a caller's `sluice equip main`
  in a worktree sluice did not make does the same and adopts nothing,
  each as a candidate's run is equipped (147)."

**crew** (teamless/deneb, records and specs; it has no statement in
blueprints):

- Records 0023, 0024 (point 4) and 0028 gain an amendment: crew readies
  no worktree; a place is equipped by sluice at flume's asking (flume
  record 0036). Their readiness text is superseded.
- `openspec/specs/kit-toolchain/spec.md` loses the requirements that
  crew reads `[tests]`, runs commands under the toolchain, and runs
  `moon --version` and `moon setup`.
- `.config/sluice.toml`'s opening comment, and `README.md`'s line on what
  readies a place, say so.
- `devenv.nix`'s `enterShell` stops running `proto install`, and a
  devenv task installs instead, as the kit rule below has it.

**Boomdrive** (no statement in blueprints): record 0009's "installing the
tools" says the host's tools (the runner, flume, sluice and herdr), never
a repository's pins, which flume's places are equipped with by sluice.

**The kit standard**, `afterthought/kit-standard`
`docs/architecture/toolchain.md`: "**[toolchain.8]** The toolchain
`[tests] toolchain` names only activates: sourcing it installs nothing.
What `.prototools` pins is installed by sluice's equip in every working
copy sluice or flume readies, and by a devenv task on entering the shell
for a person's or an agent's command." Its details rule out an
`enterShell` that runs `proto install`, citing crew's `devenv.nix`.

### 8. The held point fix

The place `fix--a-places-readiness-installs-its-repositorys-pinned-tools`
is dropped and never landed, with its branch on origin. It is Option 1
of the alternatives below: a third copy of sluice's sequence, reading
`.prototools` at the local base at each call (decision 3). Its
`_open.md` edits are moot, since `_open.md` is gone. Its scenario lives
on as S89: a fresh clone whose pins nothing installed, its toolchain
passing and installing nothing. This change's tests hold it against the
pinned sluice, with a stand-in proto and moon on the login path, as the
fix's own test did.

Until flume's build lands, a place on a fresh clone whose readiness
fails at `moon setup` can be retried once the repository's pins are
installed (12). One way is to enter the repository's devenv shell in
any worktree of it: proto's home is the repository's, shared by every
worktree (kit:toolchain.4), and the kit's `devenv shell` installs the
pins on entering. A fix's place starts unready anyway, and its agent's
first `devenv shell --` installs them. Landing the held fix as a
stopgap is the one path that departs from this design. It is the
user's to choose, and is not recommended: it ships the copy this
design removes.

### 9. Order across the repositories

1. The user's yes on this design and record 0036. The statements are
   amended on blueprints main as decision 7 words them.
2. sluice's change, launched in agentplot/sluice: `sluice equip`, and
   every path of sluice's through it, with clause 147, S72 and sluice's
   record. Then its release.
3. A chore in this repository: flume pins that sluice. The move from
   0.57.1 checks every command and field C.6 names against it (core.22).
4. This change's build, groups 2 to 4 of its tasks.
5. crew's, Boomdrive's and the kit standard's amendments, launched in
   each, in any order after step 1.

## Alternatives considered

1. **Copy sluice's steps into flume, as the held fix does.** The code
   shows the copy drifting. `for_place` (`merge_queue/sluice/mod.rs`)
   took the toolchain and `moon setup` from sluice's `Moon::plan` and
   missed `install` and `check_version`, which deneb found. sluice keeps
   three variants of its own sequence (`plan`, `kept_out`,
   `preview_request`), so even sluice's own copies differ. Every change
   to sluice's runner, such as another runner, or a step before setup,
   becomes a flume change found only when a place fails. It keeps record
   0014's words true only until sluice next changes.
2. **sluice equips, flume asks (chosen).** flume already tears a place
   down with `sluice release`, a sluice command run in the place (C.6;
   `for_place`'s `teardown`). sluice already equips a session's worktree
   it did not make, at each `sluice exec` (sluice record 0018), and runs
   most of the sequence in a submitter's worktree at each submission
   (sluice record 0050). `sluice equip` exposes, as one command, what
   already runs. Its cost: a sluice change and release, and flume's pin
   moving from 0.57.1 to it. And a repository bound to sluice is
   readied only where sluice runs, which C.6 already requires of its
   teardown.
3. **The repository's toolchain installs.** crew's `enterShell` does
   this (crew `devenv.nix`, `enterShell`). Against it:
   - flume's, sluice's and Boomdrive's `devenv.nix` refuse it, because
     sluice sources the toolchain's script inside its executor, where
     writing `<common>/proto` is refused (sluice C.2, sandbox).
   - It costs crew 2.8 seconds of a warm entry's 3.2 at every command an
     agent runs through the shell, by crew's own measurement in that
     file.
   - It makes the install a readiness command the repository writes,
     which sluice's 84 and its rule core.37 rule out.
4. **No readiness: the agent readies its place.** In a kit repository an
   agent runs every tool as `devenv shell -- <command>`
   (kit:toolchain.5), whose entry runs the install task (this
   repository's `devenv.nix`, `flume:proto:install`). This mirrors sluice
   record 0066, "a check makes itself ready". Against it:
   - It goes against the user's "Flume's job is to make the place ready".
   - It loses 12 and S16. A change's construct waits with the failing
     step named, rather than an agent spending its stage on a place that
     cannot run its checks.
   - It readies nothing in a repository whose shell does not install.
5. **A runner seam of flume's own.** A moon profile in flume beside
   sluice's `moon.rs` is two owners of how moon is readied, in two
   repositories, pinned at two versions.
6. **worktrunk's create hooks.** They run the place's code with the host
   user's full permissions, outside any executor (record 0014; sluice
   record 0013). flume makes places with hooks off (C.1, `make`), and
   kit:gate.7 keeps worktrunk's hooks out of every gate.
7. **Readiness in `.config/flume.toml`.** The repository would declare
   again what its policy and `.prototools` already declare, and the two
   files of one repository would drift as the copies in option 1 do.
8. **sluice's preparation as a library flume links.** One body of code,
   but flume's binary would build sluice's internals at whatever version
   cargo resolved. Every other reach of flume's into sluice goes through
   the command line of the release the tree pins, checked against it
   (C.6, core.22).

## Where the statements and the code disagree

1. flume's C.1 says a place is readied "as sluice readies its own
   places". The code runs two of sluice's steps, and not the install or
   the version check (`for_place`).
2. flume's 12 and 20 read readiness at the base's revision at the launch,
   and a retry "as its launch read it". The profile ignores the revision
   it is given, and reads the policy at the local base at each call
   (`git_worktree/mod.rs`, `ready`; `merge_queue/sluice/mod.rs`,
   `policy`).
3. flume's C.6 describes sluice 0.55.0. The tree pins 0.57.1
   (`devenv.nix`, `sluiceVersion`), and sluice is at 0.65.3.
4. sluice's C.1 says of its git worktree place, "nothing run in it". Its
   code runs the toolchain, the install, the version check, the queries
   and the setup there before the checks (`Checks::start`, `Moon::plan`).
5. sluice's C.2 moon row names no install and no setup. The code runs
   both (`moon.rs`).
6. sluice's 33 materializes a place "with the environment the repository
   declares", and its 84 says sluice runs "no readiness command the
   repository writes". The code agrees with both, but neither names the
   runner's own install and setup, which it runs.
7. sluice's section 9 says "Places are managed by worktrunk, which runs a
   repository's hooks". sluice's record 0013 dropped that, and its code
   uses worktrunk for no place.
8. sluice's own previews and `exec -- <command>` run the toolchain alone
   (`preview.rs`, `session.rs`). Its candidates and session runs run the
   whole sequence. A preview of a moon repository on a fresh clone would
   fail as deneb's place did.
9. crew's records 0023 (point 4: "When crew makes a bolt's, unit's or
   fix's worktree, it runs the toolchain step and then the test adapter's
   setup there") and 0024 (point 4), its `kit-toolchain` spec and its
   policy's comment say crew readies a worktree. Its code readies none
   (`kit.py`), and its rule plan.17 is retired.
10. Boomdrive's record 0009 says setup covers "installing the tools".
    There is no setup command, and nothing says whose tools.
11. The kit standard says where proto's home is and how an agent runs
    tools (kit:toolchain.4, toolchain.5), but not who installs the pins.
    The repositories answer differently: in a task (flume, sluice,
    Boomdrive) or in `enterShell` (crew).

## Found beside readiness

These are for the orchestrator to carry to their repositories. This
change does not touch them.

- Boomdrive's record 0003, rule core.14 and clones spec assume flume's
  places sit in the runner's container under `places/`. flume's record
  0027 puts them under `$XDG_DATA_HOME/flume/places/`. Its code counts
  any worktree of the clone, so it copes.
- crew's landing worktree is made at `<kit>/places/land-<bolt>` inside
  the folder flume names as the repository, which is Boomdrive's
  container, and removed with plain `git worktree remove`. crew's rule
  plan.9 asks for a sluice release and the kit's remove hooks first.
- sluice's records 0012 (`[tests] shell`), 0047 and 0066 (`ready`,
  `READY_LIMIT`), and 0013, 0018 and 0045 (checkout caches) describe
  what its code no longer does.

## Risks / Trade-offs

- **A readiness needs sluice's release first.** flume's build waits on
  sluice's change, its release and the pin's chore. The pin moves eight
  minor versions, and C.6's tables are checked again against it. Until
  then deneb keeps its workaround (decision 8).
- **A launch takes as long as sluice's equip.** On a fresh clone that is
  devenv's evaluation, proto's downloads and moon's setup, as long as a
  first `sluice exec` takes. Today's readiness already runs the toolchain
  and the setup synchronously in the launch, so only the install is new
  time.
- **proto's install of a language toolchain may leave out what the
  language's own file asks for.** This repository installs Rust with
  `rustup toolchain install` in its devenv task, because proto's install
  "leaves out" `rust-toolchain.toml`'s components and races rustup's
  proxies (`devenv.nix`, `flume:proto:install`). sluice's equip runs the
  runner's install, as sluice's own runs do. A repository whose language
  toolchain needs more declares it where its runner's setup reads it,
  `.moon/toolchains.yml`. That is the kit's to rule, and its rule
  toolchain.8 is the place.
- **Status's step names change** from `toolchain` and `moon setup` to
  sluice's four. The JSON shape is kept (core.14). A reader that matched
  on `moon setup` still finds it.
- **The sluice profile's reading of `equip`'s answer is checked against
  the pinned sluice**, in the tools layer, as every other command it
  reads is (C.6).

## Migration Plan

- A place made before keeps `<git-dir>/flume/toolchain.sh` until its
  tidy removes the worktree. Nothing reads it any more.
- A place launched before, whose first stage is retried after the build,
  is equipped by `sluice equip` at its recorded base revision.
- The site's guides and references say what a place's readiness now is.

## Open Questions

None for the build. What is the user's: their yes on this design and
record 0036, and whether to land the held fix as a stopgap (decision 8,
not recommended).
