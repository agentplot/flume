# kinds-profile Specification

## Purpose
The kinds seam, Part B.2, through `weir-kinds`: the binding that
resolves a kind to one declaration, the launch's over the repository's
over flume's, and flume's own kinds, fix, chore and change, with the
prompt of each stage.

## Requirements

### Requirement: Kinds read through weir's crate

The kinds profile SHALL read every declaration, a repository's
`.config/flume.toml` at the base's revision, a launch's `--kinds` file
and flume's own, through `weir-kinds` at a pinned revision, with no
reader of flume's own, mapping the format's fields onto the seam's
types and ignoring every section it does not know; a declaration weir
refuses SHALL be answered as could not be read, naming every field that
fails (19, 20, 23, 84, 85, core.4).

#### Scenario: Seam scenarios over the profile

- **WHEN** the kinds seam's scenarios run against the profile over scratch repositories
- **THEN** each passes, the same revision giving the same kinds and a launch's file read the same way (85)

#### Scenario: A kind at version 1, then 2

- **WHEN** a scratch repository commits a kind at version 1, then version 2, and a place is launched after each
- **THEN** the first place keeps version 1's stages and status names each place's kind at its version (S13)

#### Scenario: An elaboration section

- **WHEN** a launch's declaration holds a kind with an elaboration section
- **THEN** the kind is read and the section ignored (23, C.2)

### Requirement: The binding from the default branch

The profile SHALL read the `[profiles]` table at the default branch's
current revision, the remote's after one fetch of it where the
repository has `origin` and the local branch's otherwise, and nothing
from another tool's files; a seam the table leaves out SHALL be the
place profile `git-worktree` or unbound (20, 105, C.2, core.3).

#### Scenario: No table

- **WHEN** a scratch repository has no `.config/flume.toml` and fix is launched
- **THEN** the launch is refused naming the kind and the merge queue it needs (S33)

#### Scenario: A worktrunk binding

- **WHEN** a repository's table names `place = "worktrunk"`
- **THEN** the launch is refused naming the place profiles there are, and nothing is made (C.1, core.3)

### Requirement: The default branch is the remote's

The default branch the binding is read at SHALL be the one
`origin/HEAD` names, set from `origin`'s own `HEAD` where the repository
lacks it, and the one the repository's own `HEAD` names only where it
has no `origin`; a branch that names no commit SHALL never be the
default, and the binding SHALL then be answered as could not be read,
never as empty (20, C.2).

#### Scenario: A bare repository's HEAD on a branch never made

- **WHEN** a repository made by `git init --bare`, its `HEAD` naming `master`, a branch never made, is given `origin`, whose `main` binds the merge queue, and is fetched with no `origin/HEAD` set
- **THEN** the binding is the one at `origin`'s `main`, and with `origin` removed the binding is answered as could not be read, naming `master` (20, C.2)

### Requirement: flume's own three kinds and their prompts

flume's own kinds SHALL be built into the binary one declaration per
version, each with its prompts beside it, and each read like any other:
fix@1, fix@2, chore@1, chore@2 and change@1 to change@4 as C.2 and
record 0064 give them, change@2's code stage repeating per task group
and every other change's not, each prompt naming its deliverable and
the way to stop short, and none asking for a submit, a push or a rebase
(22, 28, 71, 140, 157, C.2).

#### Scenario: The kinds built in

- **WHEN** flume's own kinds are read
- **THEN** they are fix@1 and fix@2 on `fix/`, chore@1 and chore@2 on `chore/`, and change@1 to change@4 on `change/`, each change with construct, code and verify, change@2's code repeating per task group and change@1's, change@3's and change@4's not, all landing, each last stage's `on_fail` `code`

#### Scenario: Each version's prompts are its own

- **WHEN** flume's own declarations are read
- **THEN** each version's prompts are read from its own directory, change@1's code prompt is the one flume held before change@2, change@2's construct and verify are change@1's, change@3's three prompts are none of change@1's or change@2's, and change@4's, chore@2's and fix@2's are as their versions' before them with the one way to ask (156, 157)

#### Scenario: No prompt asks to submit

- **WHEN** every prompt of flume's own kinds is rendered for a place
- **THEN** each holds `flume needs` and none holds a submit, a push or a rebase, nor a merge but the base's in a conflict's words (28, 71, 112)

#### Scenario: A place launched at change@1

- **WHEN** a place launched at change@1 is settled by a flume that holds change@1 and change@2
- **THEN** it keeps change@1's stages as its launch read them, its code stage not repeating, and status names change@1 (21)

#### Scenario: A place launched at change@2

- **WHEN** a place launched at change@2 is settled by a flume that holds change@1 to change@4
- **THEN** it keeps change@2's stages as its launch read them, its code stage repeating per task group, and status names change@2 (21)

### Requirement: What flume's own prompts ask

change@2's, change@3's and change@4's verify prompts SHALL commit
`verify.md` in the change's directory and archive the change, or stop
short having found work; their code prompts sent with words SHALL
restore an archived change and record the fix as a ticked task; every
code prompt sent back with a conflict's words SHALL have its agent `git
merge` the base's revision named, resolve the files contested and
commit, first, and never rebase (22, 112, C.2).

#### Scenario: A code prompt sent back with a conflict

- **WHEN** the code prompts of fix@1, fix@2, chore@1, chore@2 and change@2 to change@4 are rendered with a conflict's words naming the files contested and the base's revision
- **THEN** each tells the agent to `git merge` that revision into the place's branch, resolve those files and commit, before anything else, and never to rebase (112)

### Requirement: A stage's repeat read through weir's crate

The kinds profile SHALL read a stage's `repeat` through weir-kinds alone
and map `per-task-group` onto the stage as repeating; a declaration weir
refuses for `repeat`, any other value or the field on a stage that does
not deliver `tasks`, SHALL be answered as could not be read, naming the
field (23, 84, C.2; weir 53, 57; core.4).

#### Scenario: A repeating stage read

- **WHEN** a scratch repository's declaration gives a stage delivering `tasks` the field `repeat = "per-task-group"`
- **THEN** the kind is read with that stage repeating and every other stage not

#### Scenario: Repeat on a stage of commits

- **WHEN** a declaration gives `repeat = "per-task-group"` to a stage delivering `commits`
- **THEN** the launch is refused, naming the stage's `repeat` field, and nothing is made (4)

### Requirement: change@2's code prompt puts what differs last

change@2's code prompt SHALL hold every part the same for each
repetition of a place, its instructions, the place, branch, base and
change and the way to stop short, before its group, the notes' path and
its words, which come last, and SHALL tell the agent to read and add to
the notes and never commit them (141, C.2; core.31).

#### Scenario: Two repetitions' prompts

- **WHEN** change@2's code prompt is rendered for groups 1 and 2 of one place, the second with a retry's words
- **THEN** the two texts are equal up to the first character of the group, and differ only after it

### Requirement: A declaration's agents tables give a mode

The kinds profile SHALL read `mode` from every agents table a
declaration holds, a kind's, a stage's and the repository's for a kind
it does not declare, from the table weir's crate returns as written: a
word, one of flume's four, or a table of programs whose entries are
strings, each kept as written; any other SHALL fail the declaration
naming the field (23, 162, 169, C.2, core.4).

#### Scenario: A mode for every stage and a table for one

- **WHEN** a scratch repository's `.config/flume.toml` gives `[kinds.bump.agents] mode = "edits"` and a stage's `agents = [{ mode = { claude = "auto" } }]`
- **THEN** the kind's settings read `edits` for every stage and the table for that stage (23, 162, C.2)

#### Scenario: A mode that is neither

- **WHEN** a declaration's agents table gives `mode = "auto"`, `mode = ["edits"]` or `mode = {}`
- **THEN** the declaration cannot be read, naming the table's `mode` (4, 169, C.2)

### Requirement: Packages, identities and locks through weir's crate

The kinds profile SHALL read a kind as a launch writes it, a package with
its content hash, a source's listing, the repository's lock at a revision
and a lock on disk, and SHALL write a lock, each through `weir-kinds` at
its pinned revision alone; it SHALL spell every identity it answers as
weir's crate spells it, and flume's own code SHALL parse, spell or hash
none (core.4; weir 88 to 93, C.4).

#### Scenario: A package's hash wherever it lies

- **WHEN** one package is copied to two scratch directories, one under a link, and each is read at its identity
- **THEN** both read the same kind and the same hash, and the hash changes when one prompt's byte does (156; weir 90)

#### Scenario: A source's listing

- **WHEN** a scratch `kinds_dir` holds `review@1/` and `review@2/`, each with a `kind.toml`, and `notes/`, `lint@x/` and `lint@1/` with no `kind.toml`
- **THEN** the listing answers acme/review@1 and acme/review@2 alone (159)

#### Scenario: The repository's lock at a revision

- **WHEN** a scratch repository commits `.config/flume.lock` recording `flume/change@1`, then a second revision recording nothing
- **THEN** the lock at the first revision records flume/change@1 with its hash, the one at the second records nothing, and one at a revision with no lock is answered as none (20, 155)

#### Scenario: A lock written and read back

- **WHEN** the profile writes a lock of acme/review@1 and acme/review@2 to a scratch file and reads it
- **THEN** it reads both entries as written, and a write that fails leaves the file as it was (155; weir 91)

#### Scenario: Flume's own as packages

- **WHEN** flume's own kinds are read
- **THEN** each is a package with its identity, `flume/fix@1`, `flume/fix@2`, `flume/chore@1`, `flume/chore@2` and `flume/change@1` to `flume/change@4`, and its hash (156, 157)

### Requirement: A declaration's agents tables give own settings

The kinds profile SHALL read `programs` from every agents table a
declaration holds, a kind's, a stage's and the repository's for a kind
it does not declare, from the table weir's crate returns as written:
each entry's `file` and `settings`, kept as JSON values. Any other
shape or a date SHALL fail the declaration naming the field (23, 158,
195, C.2; core.4).

#### Scenario: Own settings for every stage and for one

- **WHEN** a scratch repository's `.config/flume.toml` gives `[kinds.bump.agents.programs.claude.settings] tui = "fullscreen"` and a stage's `agents = [{ programs = { claude = { file = "claude-bump.json" } } }]`, and `.config/claude-bump.json` at the revision gives a `statusLine`
- **THEN** the kind's settings read `tui` for every stage, and for that stage the file's `statusLine` with the file's path, read at the revision (23, 157, 195, C.2)

#### Scenario: An entry flume cannot take

- **WHEN** a declaration's agents table gives `programs = { claude = "x" }`, or `programs = { claude = { tui = "fullscreen" } }`
- **THEN** the declaration cannot be read, naming the field, and nothing is launched (4, 195, C.2)

### Requirement: A declaration's settings file is read beside it

A file an agents table of a declaration names SHALL be read by a path
relative to the declaration's file, as its prompts are, at the same
revision for a repository's and on disk for a launch's; one that cannot
be read, does not parse or lies outside the repository SHALL fail the
declaration naming the field (157, 195, C.2; core.34).

#### Scenario: A file outside the repository or not at the revision

- **WHEN** a declaration's agents table gives `programs = { claude = { file = "../../claude.json" } }`, past the repository's root, or names a file the revision does not hold
- **THEN** the declaration cannot be read, naming the field, and nothing is launched (4, 157; core.34)

### Requirement: A repository's settings for every kind

The kinds profile SHALL read a repository's `[agents]`, flume's own
table of `.config/flume.toml` beside `[profiles]` and no part of any
kind, from the sections weir's crate returns unnamed, at the revision
the kinds are read at, giving `program`, `model`, `effort`,
`mode`, `instructions` and `programs` as the repository's kind tables
do (21, 23, 32, 195, 197, C.2; core.4).

#### Scenario: A repository's agents for every kind

- **WHEN** a scratch repository's `.config/flume.toml` at the revision gives `[agents.programs.claude.settings] attribution = { commit = "Assisted-by: Claude Code", pr = "" }` beside a `[kinds.fix.agents]` table
- **THEN** the kinds read give the repository's settings for every kind with that `attribution` as written, apart from its settings for fix (S102, 32, C.2)

### Requirement: A repository's agents table gives no stage, stream or environment

A `stages`, `streams`, `env` or `beside` in a repository's `[agents]`
SHALL fail the declaration naming the field, and a launch's declaration
and a package's SHALL not be read for the table (4, 32, C.2).

#### Scenario: A repository's agents table that names a stage

- **WHEN** a repository's `[agents]` gives `stages = { code = { model = "opus" } }`, or `env = { X = "1" }`
- **THEN** the declaration cannot be read, naming `agents.stages` or `agents.env`, and nothing is launched (4, C.2)

### Requirement: A declaration's agents tables give an effort

The kinds profile SHALL read `effort` from every agents table a
declaration holds, a kind's, a stage's, the repository's for a kind it
does not declare and the repository's `[agents]` for every kind, from
the table weir's crate returns as written: a
string, kept as written; any other, a table of programs among it until
the model's is built with it, SHALL fail the declaration naming the
field (23, 158, 172, 197, C.2; core.4).

#### Scenario: An effort for every stage and one for a stage

- **WHEN** a scratch repository's `.config/flume.toml` gives `[kinds.bump.agents] effort = "high"` and a stage's `agents = [{ effort = "xhigh" }]`
- **THEN** the kind's settings read `high` for every stage and `xhigh` for that stage (23, 197, C.2)

#### Scenario: An effort for every kind of a repository

- **WHEN** a scratch repository's `.config/flume.toml` gives `[agents] effort = "medium"` beside a `[kinds.fix.agents]` table giving `effort = "high"`
- **THEN** the kinds read give the repository's settings for every kind with `medium`, and its settings for fix with `high` (32, 197, C.2)

#### Scenario: An effort that is no word

- **WHEN** a declaration's agents table gives `effort = 3` or `effort = { claude = "high" }`
- **THEN** the declaration cannot be read, naming the table's `effort` (4, 197, C.2)

### Requirement: A declaration gives no keep_open

A declaration's agents table, a kind's, a stage's, or a repository's
for every kind or for a kind it does not declare, that gives
`keep_open` SHALL fail to read, naming the field and that the host or
the launch gives it, so the launch is refused whole (4, 158, 198, C.2).

#### Scenario: A repository's keep_open

- **WHEN** a repository's `.config/flume.toml` gives `[kinds.fix.agents] keep_open = "2h"`
- **THEN** a launch of a fix is refused naming `kinds.fix.agents.keep_open`, and nothing is made (4, 198)

#### Scenario: A kind's keep_open

- **WHEN** a package's kind declares a stage whose `agents` entry gives `keep_open = "5m"`
- **THEN** a launch of that kind is refused naming the stage's `keep_open`, and nothing is made (4, 158)

### Requirement: A kind that lands with no stages read through weir's crate

The kinds profile SHALL read a kind declaring `stages = []` and
`lands = true` through `weir-kinds`, at the revision that reads it, as
a kind with no stages; a kind declaring no stages that does not land
SHALL be refused in weir's words, naming the field; and a table that
leaves `stages` out SHALL read as settings for a kind declared
elsewhere, as before (4, 18, 23, 202; weir 51, 57; core.4).

#### Scenario: Three tables over a scratch repository

- **WHEN** a scratch repository's `.config/flume.toml` declares `train` with `lands = true` and `stages = []`, `idle` with `lands = false` and `stages = []`, and a `[kinds.fix.agents]` table with no stages
- **THEN** `train` reads as a kind that lands with no stages, `idle` is refused naming its stages, and `fix`'s table is read as settings for flume's own fix (18, 57)

### Requirement: change@3's construct gives each task its own check

change@3's construct prompt SHALL ask each task to name the narrowest
check that shows it done, one test by name, one package's tests or a
lint of the files it touches, and SHALL rule out a task that runs the
repository's whole checks or checks a task group (22, C.2, S119).

#### Scenario: change@3's construct prompt rendered

- **WHEN** change@3's construct prompt is rendered for a place
- **THEN** it asks each task for the narrowest check that shows it done, and rules out a task running the repository's whole checks or a task group's (22, C.2)

### Requirement: change@3's code runs each task's own check alone

change@3's code prompt SHALL have its one agent work through every task
of the change in order, run each task's own check and tick the task once
that check passes, run no other check, and put a retry's words last
(22, 131, C.2; core.33).

#### Scenario: change@3's code prompt rendered

- **WHEN** change@3's code prompt is rendered for a place
- **THEN** it asks for each task's own check before its tick and rules out the repository's whole checks and a task group's (22, C.2)

#### Scenario: A retry's words come last

- **WHEN** change@3's code prompt is rendered for one place without words and with a verify's words
- **THEN** the two texts are equal up to the first character of the words' section, and the second holds the words and asks the fix be recorded as a task naming its own check (131; core.33)

### Requirement: change@3's verify runs the whole checks once

change@3's verify prompt SHALL run the repository's whole checks once,
as the repository's instructions name them, before it checks the work
against the change, and SHALL stop short having found work where the
checks fail or the work falls short (22, 131, C.2, S119).

#### Scenario: change@3's verify prompt rendered

- **WHEN** change@3's verify prompt is rendered for a place
- **THEN** it runs the repository's whole checks once before it checks the specs and the tasks, names their outcome in `verify.md`, and archives the change only where both hold (22, 131, C.2)

### Requirement: A kind with no stages names its landing fix through weir's crate

The kinds profile SHALL map a kind's `landing_fix`, as `weir-kinds`
reads it, onto the seam's kind of its fixes, spelled as weir's crate
spells the identity, and its `max_landing_fixes` onto the seam's bound,
taking neither from anything else, in a repository's declaration, a
launch's and a package's alike; a kind that gives neither SHALL name no
fix and no bound (23, 157; weir 52, 89; weir record 0005; core.4).

#### Scenario: A landing fix and its bound over a scratch repository

- **WHEN** a scratch repository's `.config/flume.toml` declares `train`, landing with `stages = []`, `landing_fix = "repo/mend@1"` and `max_landing_fixes = 2`; `yard`, landing with `stages = []` and neither field; and `mend`, landing with one code stage delivering commits
- **THEN** `train` reads naming `repo/mend@1` the kind of its fixes with a bound of 2, and `yard` and `mend` name no fix and no bound (23; weir 52)

#### Scenario: A package's landing fix

- **WHEN** a scratch source's `kinds_dir` holds `train@1/kind.toml`, declaring `train` landing with `stages = []`, `landing_fix = "flume/fix@1"` and `max_landing_fixes = 1`, and the package is read at `acme/train@1`
- **THEN** its kind names `flume/fix@1` the kind of its fixes with a bound of 1 (157; weir 89)

### Requirement: A landing fix weir or flume refuses

A `landing_fix` weir refuses, a bare name or a name with no source or
no version among them, and either field on a kind with stages, SHALL be
answered as could not be read in weir's words, naming the field; a
bound past what flume counts SHALL be refused naming the field; and
`fix` and `max_fixes` SHALL be read as the sections weir returns them
as, and ignored (4, 23, 84; weir 57; weir record 0005; core.4).

#### Scenario: A landing fix weir refuses

- **WHEN** the scratch repository's `train` gives `landing_fix = "mend"`, then `landing_fix = "repo/mend"`, each at a revision of its own, and then a one-stage kind `bump` gives `landing_fix = "flume/fix@1"` and `max_landing_fixes = 1`
- **THEN** each revision's kinds are answered as could not be read: the first two naming `kinds.train.landing_fix` and that it is no identity, the third naming `kinds.bump.landing_fix` and `kinds.bump.max_landing_fixes` and that each is on a kind with stages, all in weir's words (57; weir record 0005)

#### Scenario: A bound past what flume counts

- **WHEN** the scratch repository's `train` gives `landing_fix = "repo/mend@1"` and `max_landing_fixes = 4294967296`
- **THEN** its kinds are answered as could not be read, naming `kinds.train.max_landing_fixes` and that it is more than flume counts (4)

#### Scenario: The fields' old spellings

- **WHEN** the scratch repository's `train`, landing with `stages = []`, gives `fix = "repo/mend@1"` and `max_fixes = 2`
- **THEN** `train` reads naming no fix and no bound (23; weir record 0005)

### Requirement: A base place's launch resolves the landing fix the profile reads

A base place launched under a kind the kinds profile read SHALL resolve
the landing fix its kind names as a launch's whole identity resolves,
at the version the identity names, and hold it with where it came from;
a landing fix that resolves to no kind SHALL refuse the launch, naming
it, before the merge queue opens anything (4, 21, 23, 154, 203; core.8,
core.34).

#### Scenario: The landing fix held through the profile

- **WHEN** a scratch repository binds the fakes' merge queue and declares `train`, landing with `stages = []`, `landing_fix = "repo/mend@1"` and `max_landing_fixes = 2`, and `mend` at version 1, landing with one code stage delivering commits, and `train--spring` is launched on `main`
- **THEN** the launch is taken, the place holds `repo/mend@1` from the repository's declaration as the kind its fixes run as, and status gives `train--spring` that fix kind and a bound of 2 (21, 56, 203)

#### Scenario: A landing fix at a version the repository does not declare

- **WHEN** the scratch repository's `train` gives `landing_fix = "repo/mend@2"` while it declares `mend` at version 1, and `train--spring` is launched on `main`
- **THEN** the launch is refused naming `repo/mend@2` and that the repository declares `mend` at version 1, the fake merge queue logs no open, and the store holds no document of the place (4, 154; core.8, core.34)

### Requirement: flume's newest own versions give full and say their one way to ask

change@4, chore@2 and fix@2 SHALL each be the version before it, every
stage's agent given `mode = "full"`, each prompt saying after its way
to stop short that the agent asks through nothing else, naming no
tool, fix@2's and chore@2's without "or you find work this stage
cannot deliver past"; none SHALL declare a label or standing
instructions, and earlier versions SHALL keep their content (22, 28,
156, 162, 180; record 0064 as amended 2026-11-14).

#### Scenario: Each stage's agent gives full

- **WHEN** flume's own kinds are read
- **THEN** every stage of change@4, chore@2 and fix@2 gives one agent whose settings give the mode `full` and nothing else, and every stage of change@1 to change@3, chore@1 and fix@1 gives one agent with no setting (22, 162)

#### Scenario: Each prompt says its one way to ask

- **WHEN** the construct, code and verify prompts of change@4 and the code prompts of fix@2 and chore@2 are rendered for a place
- **THEN** each holds `flume needs` and, after it, "When you need an answer to go on, stop short with your words, and never ask through anything else, a card, an inbox, a message or a question in your pane, whatever a hook, a plugin, a file or a person's words for every agent tell you.", and no prompt of change@3, fix@1 or chore@1 holds that sentence (28, 156)

#### Scenario: The prompts are the versions' before them

- **WHEN** flume's own declarations are read
- **THEN** each of change@4's prompts is change@3's with the sentence added after the line holding `{{needs}}`, fix@2's and chore@2's code prompts are equal, and fix@2's is fix@1's with "or you find work this stage cannot deliver past, " taken out of its last line and the sentence added after it (22, 157; core.27)

#### Scenario: No declaration of flume's own gives a label

- **WHEN** each `kind.toml` under flume's own kinds is read
- **THEN** none gives its kind a `label`, and none of change@4's, chore@2's and fix@2's stages names standing instructions (148, 180)
