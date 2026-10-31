# Design

## Context

flume's tree at 359f8ce resolves an agent's program, model and
instructions from six layers at each start. `Engine::resolve` in
`crates/flume-core/src/settings.rs` folds them, keeping each setting's
layer in `from`. `Engine::resolution` binds the program to its agent
profile, and `Engine::launch` runs that check for every stage before
anything is made. `Settings` in `crates/flume-seams/src/kinds.rs` holds
`program`, `model`, `instructions` and `beside`. The kinds profile's
`map_agent` (`crates/flume/src/profiles/kinds/weir/mod.rs`) and the
host's `agent_settings` (`crates/flume/src/host/config.rs`) read an
agents table's keys, and `commands/launch.rs` reads `--agent`. The
Claude Code profile's `command` passes `--model` and
`--append-system-prompt` and no `--permission-mode`. Its SessionStart
hook (`claude/hook.rs`, `compare` and `ack_started`) checks the
program and each variable's digest against the start's `StartRecord`
before it acks. A start whose agent began otherwise is unsettled, ended
once quiet and started afresh, and a second unsettled start in a row is
never begun (`crates/flume-core/src/unsettled.rs`).

Record 0024 (accepted 2026-10-30) is stated at blueprints 9c7490c and
a11f6d5: 4, 32, 88, 135, 161 and C.2, C.4 and C.7 amended, and 162,
169, S66 and S67 added. The statement's main, 5caff76, holds them as
this change reads them. Weir's C.4 names `mode` a field of an agents
table, one word or a table of programs, "the reader's to check", so
weir-kinds returns it as written and flume's pin of it stays (core.4).

The rules this design relies on are core.1 and core.2 (the core names
no program, and only the Claude Code profile names `--permission-mode`
or its choices), core.4 (the format is weir's, and flume reads `mode`
from the table weir returns), core.10 (an agent is its stage's only
where its word matches its start's settings, its mode among them),
core.12 (flume answers nothing for an agent in any mode), core.14 (the
JSON form only gains fields), core.22 (the map is checked against the
pinned Claude Code), core.30 (the mode is a setting of the six layers,
none passed where none is given, and a mode a profile cannot honour
refused), core.35 (the site's neutral voice) and core.39 (a profile's
map is flume's own code, and a mode it cannot honour refuses naming the
program). The one rules page is `docs/architecture/core.md`; this
change touches no area another page would own. Where flume has no rule,
the kit standard's code profile binds (kit:specs.3, kit:testing.5).

## Goals / Non-Goals

**Goals:**

- A kind, a repository, a host or a launch says how far a stage's
  agent may act without asking a person, in words that mean the same
  on every program (162, 169).
- A host that says nothing changes nothing: no mode is passed, and the
  account's own configuration decides (162).
- No agent runs a stage in a mode other than its start's, wider or
  narrower, by flume's doing or by its account's configuration (161,
  169; core.10).
- Every refusal names the mode, the program, the stage and the layer,
  so a host can mend the layer that gave it (4, 32).

**Non-Goals:**

- The plan stage, the `plan` deliverable, the plugin's deny of
  `ExitPlanMode`, a finish that accepts a plan, `{{plan}}` and flume's
  own plan@1 (163, 164, S68, S72): record 0024's plan part, built
  after the mode. Until then no stage delivers a plan, so a conversation
  is the only stage that takes `plan`.
- Points, a stream's points bound and its pace (record 0025).
- The common profile, every other program's map of modes, a model
  given as a table of programs, the program an account names, and the
  degraded parts (170 to 174, S71, S73 to S75; record 0026).
  `profiles.rs` still binds Claude Code's profile alone.
- Anything flume answers for an agent. A mode widens nothing flume says
  on an agent's behalf (34; core.12): an agent in `ask` or `edits` that
  asks a person waits asking, as any agent does today (90).
- A field of the kind format. `mode` is weir's, and weir-kinds already
  returns it (core.4).

## Decisions

### 1. A mode in the seams

`flume-seams` gains `Mode`, flume's four, spelled `plan`, `ask`,
`edits` and `full` (169). `ModeSetting` is a mode as a layer gives it:
`Flume(Mode)`, or `Programs(BTreeMap<String, String>)`, a table of
programs, each entry as written. It serializes as the word or as a
JSON object. It deserializes through `ModeSetting::from_word`, which
takes the four and refuses any other word, naming it and saying that a
program's own value goes under its program's name, as `mode = {
<program> = "<value>" }`. A table that names no program is refused too.
So held state, the launch's JSON form and every reader share one
spelling. `ModeSetting::written` gives it as a layer would write it,
`edits` or `{ claude = "auto", codex = "edits" }`, the keys in order.
`ModeSetting::entry(program)` is the agent's entry: one of the four
given whole, or the table's entry for the program's name, as
`ModeEntry::Flume(Mode)` where the entry is one of the four and
`ModeEntry::Own(String)` otherwise; none where the table names no entry
for the program.

`Settings` gains `mode: Option<ModeSetting>`, read with a default and
written only where set, so held state written before it reads.
`Settings::under` takes it whole, as every setting is (32).
`StartRecord` gains `mode: Option<String>`, the value as the start's
profile passes it, none where the settings give no mode (161).
`Unsettled` gains `mode: Option<ModeBegan>`, `ModeBegan { asked:
String, began: Option<String> }`: the value the start passed, and the
one the agent's word says it began in, none where its word names none.
`Unsettled::words` adds "in the mode `<began>`, where its start asked
`<asked>`", or "in a mode its word did not name" (56, 135).

### 2. `mode` read in every layer

One reader takes an agents table's `mode`, TOML in both files: a
string through `ModeSetting::from_word`; a table each of whose values is
a string as `ModeSetting::Programs`; anything else fails, naming the
key. The kinds profile's `map_agent` calls it for every agents table,
a kind's, a stage's and the repository's for a kind it does not declare,
pushing a problem naming `<table>.mode` where it fails, so the
declaration cannot be read and the launch is refused naming the field
(4, 23, C.2). The host's `agent_settings` calls it for
`[kinds.<kind>.agents]`, `[repos."<repository>".kinds.<kind>.agents]`
and each `.stages.<stage>` under them. A host table that fails refuses
the configuration naming the key, and every launch and start with it,
as a configuration that cannot be read does today (32, C.7). A table's
entries are kept as written; only the agent's profile can say whether
its own value is one it names (decision 4).

`--agent [<stage>.]mode=<mode>` gives one of the four, and `--agent
[<stage>.]mode.<program>=<value>` one entry of a table, several building
one table. A word and an entry for the same stage, or for every stage,
refuse the launch naming both: a mode is one word or one table. `mode`
joins the fields `--agent` lists in its refusals (7, C.2). The launch's
JSON form takes `mode` as `Settings` serializes it.

### 3. The mode resolved

`Engine::resolve` folds `mode` with the rest, taken whole from the first
of the six layers that gives it, a stage's own before every stage's in
each (32). `from["mode"]` is the layer with `ModeSetting::written`. The
host's two layers give `mode` as they give `model` and `instructions`,
read as the configuration stands at the start; the launch's, the
repository's and the kind's are as the launch read them (8, 21).

`Resolution` gains `mode: Option<String>`, the value the agent's
profile passes. `Engine::resolution` takes the agent's entry by its
program's name, the last component of the program its settings name,
the same name by which the resolver binds the program to its profile.
It then asks the profile to map that entry (decision 4). With no mode in
any layer, `mode` is none, no entry is asked for and nothing is mapped
(decision 6).

### 4. The agent seam's mode part, and Claude Code's map

The agent seam gains `mode(&self, entry: &ModeEntry) -> Result<String,
String>`, the value of its program's own that the entry maps to, or
words naming the mode and the program where it has none (88, 169). By
default it refuses every entry, since a profile that maps no mode
refuses one (170). It also gains `mode_read_back(&self) -> Offered`:
whether the program's word at its start, its word that it started or
its word at its start's first prompt, names the mode it began in,
`NotOffered` by default (161, 169). `command` gains `mode:
Option<&str>`, the value the claim's resolution mapped, which the
profile passes as its program's setting. The command never meets a
value its own `mode` would refuse, since the claim refused it first.

The Claude Code profile's map is `claude/mode.rs`, flume's own code
(core.39): `plan` to `plan`, `ask` to `manual`, `edits` to
`acceptEdits`, `full` to `bypassPermissions`. Its own values are the
pinned 2.1.296's `--permission-mode` choices, `acceptEdits`, `auto`,
`bypassPermissions`, `manual`, `dontAsk` and `plan`, each passed as
written. Any other own value refuses, naming it, `claude`, and the
values it names (C.4). Each of the four maps to the one choice that
means it, and nothing is read as a nearer one, wider or narrower (169).
`command` adds `--permission-mode <value>` where `mode` is given, and
nothing where it is none. `mode_read_back` is `Offered`.

### 5. Refusals at the launch and at a later start

`Engine::launch` asks `Engine::resolution` of every stage before
anything is made, as it asks for the program today, so a mode no layer
can run refuses the launch whole (4; core.8). A later start whose
resolution fails leaves the stage refused with `refused_setting`,
starting no agent until a retry, as a program no profile binds does
today (27, 32). The words name the mode as written, the program, the
stage and the layer:

- the profile's refusal: "the stage `<stage>`'s mode `<written>`, from
  the layer `<layer>`, cannot run on `<program>`: `<the profile's
  words>`";
- a table with no entry: "the stage `<stage>`'s mode `<written>`, from
  the layer `<layer>`, names no entry for `<program>`". The table is
  taken whole, so a later layer's mode is never read in its place
  (169);
- `plan` on a stage that is not a conversation: "the stage `<stage>`
  delivers `<deliverable>`, which an agent in plan mode cannot write,
  and its mode `<written>`, from the layer `<layer>`, is plan" (4,
  169).

`plan` is the entry `ModeEntry::Flume(Mode::Plan)`, or an own value
equal to what the profile maps `plan` to, as Claude Code's own `plan`
under `claude`. A conversation takes `plan`: its agent plans with its
operator, who answers its ask to leave plan mode in its pane (109).

### 6. None where no layer gives one

With `Settings::mode` none after the fold, the resolution's `mode` is
none, no `--permission-mode` is passed, the start's record names no
mode, and no mode is compared (161, 162). The account's own
configuration decides, the one setting the agent program's own
configuration may choose (32).

### 7. The start's record and the first prompt's comparison

`Engine::claim` writes the resolution's `mode` into the start's
`StartRecord::mode` (161). 2.1.296 gives a SessionStart input no
`permission_mode`. It builds that input as
`{...$d(ke,se()),hook_event_name:"SessionStart",...}`, and its builder
`$d(e,n,r,s)` sets `permission_mode:r`, so the field is undefined there,
as it is for Setup, SessionEnd, StopFailure and Notification. A hook's
environment names no mode either, and the transcript records one only
when the session's metadata is written again, never at its start. The
events whose builder is handed the session's mode carry it,
UserPromptSubmit among them: its input is
`{...$d(r.session,se(),n),hook_event_name:"UserPromptSubmit",prompt:e`,
with `n` the session's mode as it stands (`qtt(e,de(o).mode,o,`). So
SessionStart compares the program and the variables alone and acks
started where they match, and the mode is compared at the start's
first prompt.

The UserPromptSubmit hook compares at the start's first prompt, the
first with no received ack where no word that the agent began otherwise
stands. Where the record names a mode, it compares its input's
`permission_mode` with the record before it acks received. 2.1.296 reads
`--permission-mode manual` as its own `default`, the alias its settings
schema names ("'manual' is accepted as an alias for 'default'"), so the
profile reads `default` back as `manual` and every other choice as
itself (`claude/mode.rs`, `began`). Where the value read is not the
record's, or the input carries none, the hook answers `{"decision":
"block"}` with words naming both modes. 2.1.296 reads that as a blocking
error: the prompt is held back and no query is made, so the agent takes
no turn in a mode its start did not ask. The hook acks nothing and
writes `unsettled.json`, naming the mode in `Unsettled::mode` with both
values and the started ack's session and transcript beside it. It then
starts the settle, as SessionStart does for a variable. `flume ack
received` gains `--mode <value>`, the mode the session runs in as the
program names it, which it compares at the start's first prompt as the
hook does. `flume ack started` compares no mode.

A later prompt or tell, and a SessionStart after a resume, a clear or a
compaction, compare no mode. The mode is read at the agent's start, and
its first prompt is the first word of the program's that names it
(162). Once a session has started, its mode is the session's own: a
person may move it in the pane, and a conversation's operator may let
its agent out of plan mode. A started agent is never unsettled for it,
and its later acks still record its transcript.

This reads the mode where the statement's C.4 at 5caff76 does not. Its
started row has SessionStart compare "the hook input's
`permission_mode`, which every hook's input carries in 2.1.296", and
161 takes the word that it started only where the mode matches. Neither
holds of the pinned release. The operator ruled on 2026-10-31 to read
the mode at the start's first prompt, keeping C.4's and S67's
guarantee whole: no agent takes a turn of its stage in a mode its start
did not ask, and a start begun in another mode is ended and started
afresh, a second never begun. Those rows and 161's words are the
operator's to amend in the statement (core.19).

### 8. A start in another mode starts afresh

A start whose word names a mode other than its record's reads as
`started_without` with `Unsettled::mode`, and the core's unsettled path
does the rest (35, 161). The core reads `started_without` while the
start is started as well as while it is starting, since Claude Code's
word on its mode comes at its first prompt, after its started ack. No
prompt is sent, or none again where the word came at the first prompt,
and the stage's limit holds as it does for a start still starting. The agent is
ended once quiet, and the stage starts afresh under the settings
resolved then, counting toward no bound and waiting on no caller. A
second start in a row that began otherwise is never begun, its end
naming the mode asked and the mode it began in, as it names variables
today (S67). A start on a program whose profile does not read the mode
back records the value passed and compares none (169).

### 9. Status and the JSON form

The JSON form only gains fields (core.14). An agent's settings give
`mode` as `Settings` serializes it, and `from.mode` its layer and its
value as written (56). `AgentStatus` gains `mode: Option<ModeStatus>`,
none where no layer gave one: `passed`, the value from the start's
record; `entry`, the program whose entry of a table the agent took,
none for one of the four given whole; and `read_back`, whether its
profile reads the mode back. An unsettled start and a never-begun end
give `mode` with `asked` and `began` through `Unsettled`. The person's
form prints the mode on the agent's line with its layer and the value
passed, "not read back" where it was not, and each unsettled start's
modes on its own line.

### 10. The map pinned to the release

A `check-tools` test in `crates/flume/tests/profiles/tools/claude_words.rs`
reads the pinned Claude Code, the file its `claude` wrapper from
devenv.nix execs, and requires the fragments `claude/mode.rs` rests on
byte for byte (core.22; C.4):

- the release's own modes,
  `["acceptEdits","auto","bypassPermissions","default","dontAsk","plan"]`;
- the alias, `e==="manual"?"default":e`;
- the flag, `"--permission-mode <mode>","Permission mode to use for the session"`;
- the hook input's builder naming the mode by its third argument,
  `cwd:n,permission_mode:r,`;
- UserPromptSubmit's input built with it,
  `{...$d(r.session,se(),n),hook_event_name:"UserPromptSubmit",prompt:e`;
- the session's mode handed to it, `qtt(e,de(o).mode,o,`;
- a block read as a blocking error,
  `case"block":V.permissionBehavior="deny",V.blockingError={blockingError:e.reason||"Blocked by hook"`;
- a prompt so blocked returned unsent,
  `if(s.blockingError){let a=M(p,o.abortController.signal);return{blocked:`.

A bump that renames a choice, drops the alias, moves the field or stops
holding back a blocked prompt fails the gate until the map is read
again from the new release.

### 11. The stand-in and the walkthroughs

The stand-in for Claude Code (`crates/flume/tests/profiles/stand-in/claude.rs`)
reads `--permission-mode <value>` and gives the inputs of
UserPromptSubmit, PreToolUse, PostToolUse and Stop the `permission_mode`
2.1.296 would: the value passed, `manual` read as `default`, and
`default` where none was passed. The inputs of SessionStart, StopFailure
and SessionEnd name none, as that release's do not. A script act
`{"begin_in": "<value>"}` gives that value instead, wherever the act
stands, as a session whose account's configuration refuses the mode
asked begins in another. A prompt its UserPromptSubmit hook blocks is
held back: the stand-in makes no act and takes no turn, and its session
ends.

Two walkthroughs in `crates/flume/tests/profiles/tools/walkthroughs.rs`
run with the stand-in under the printed command:

- S66: a fix on a host whose `[kinds.fix.agents]` gives `mode =
  "edits"`, while the scratch repository's `.config/flume.toml` gives
  `[kinds.fix.agents] mode = "full"`. The printed command holds
  `--permission-mode acceptEdits`, the started ack is taken, the
  stand-in's first prompt names `acceptEdits` and is received, so code
  begins, status names `edits` from `host-kind` passed as `acceptEdits`,
  and the place lands. A second fix
  on a host and a repository that give no mode prints no
  `--permission-mode`, its start records no mode, and it lands.
- S67: a fix launched with `--agent mode=full`, whose stand-in script
  holds `{"begin_in": "default"}`. The first start acks started, its
  first prompt is blocked, and it is ended, and code starts afresh. That start begins in `default` again
  and is never begun, status naming `bypassPermissions` asked and
  `manual` begun in.

### 12. A mode reaching a real Claude Code agent

One walkthrough in `crates/flume/tests/profiles/live/walkthroughs.rs`
runs a fix under herdr with the pinned Claude Code, on a walkthrough
host whose `[kinds.fix.agents]` gives `mode = "ask"`. The scratch
repository's Claude Code settings give `defaultMode: acceptEdits` and
allow the tools the fix uses. So the session begins in `default`, the
release's word for `manual`, only where `--permission-mode manual`
reached it over the repository's own default. The code agent's
started ack is taken, and its first prompt is received, which the
UserPromptSubmit hook takes only once the real `permission_mode`
matched, so code begins. Status names `ask` from `host-kind` passed as
`manual`, and the place lands. `ask` asks before an edit or a
command the session's rules do not allow, and the settings allow every
tool the fix uses, so the agent asks nothing.

### 13. The site

The agent-settings guide gains `mode` in its table of settings, the
four modes and what each lets an agent do, the table of programs,
Claude Code's map and its own values, nothing passed where no layer
gives a mode, the refusals, and the start check's comparison of the
mode. It says that `ask` and `edits` wait on a person flume never
answers for, and that an account must accept Claude Code's warning
before its first session in `bypassPermissions`, since flume never
answers that warning either. The kinds guide gives `mode` in an agents
table; the configuration reference gives it in the host's two tables;
the commands reference gives `--agent mode` and `mode.<program>`, and
`flume ack received --mode`. All of it speaks to any host's operator and
names no host or team of ours (core.35).

## Risks / Trade-offs

- The comparison rests on how 2.1.296 reports its mode. A release that
  reports another word for a choice would leave every start of that
  mode unsettled twice and never begun. The `check-tools` test fails at
  the bump instead, and the live walkthrough proves `manual` read back
  as `default` on the real program.
- An account whose configuration disables the mode asked, such as
  `bypassPermissions`, never runs that stage. That is S67's outcome:
  status names both modes, and the host mends its tables or the
  account.
- `ask` and `edits` stop an unattended stage at its first question
  until a person answers in its pane. Status says the agent is asking.
  flume answers no question to keep it going (34), so the docs keep
  these modes for watched work.
- A session's mode after its start is not checked, so a person who
  moves it in the pane is not stopped. The mode is the work's word on
  how its agent starts, and the pane is the person's (162).
- Claude Code names its mode only with a prompt, so a start's started
  ack is taken before its mode is known, and status reads it started
  until its first prompt. That prompt is held back where the mode
  differs, so the agent does no work of its stage first. A start whose
  prompt is one of Claude Code's own commands fires no
  UserPromptSubmit, so its mode is compared only at the first prompt it
  does submit.
