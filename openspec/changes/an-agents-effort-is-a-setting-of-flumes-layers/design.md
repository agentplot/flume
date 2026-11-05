# Design

## Context

crew's main-level roles name a model and an effort in their agent
definitions, `plugin/roles/<role>.md` in Claude Code's agent format.
flume gives an agent its model in its layers (32, 172) and no effort,
so a role run as a flume kind starts at its account's default. The
user ruled on 2026-11-05 that the planner and the design agent move
onto their kinds only once flume carries an effort ("That order;
planner and design agent wait for an effort setting"), in the
questions design `every-question-a-place-asks-reaches-someone-who-answers-it`.

### What the family and the programs do now

Read on 2026-11-05 before anything here was designed.

**Claude Code, the pinned 2.1.296 and the fleet's 2.1.312.** Both
releases hold the same four things, read from each binary's program:

- The option `--effort <level>`, "Effort level for the current session
  (low, medium, high, xhigh, max)", the five words the release's list
  holds, `["low","medium","high","xhigh","max"]`. An unknown word is
  not refused. The release writes "Unknown --effort value '<word>' —
  ignoring it and using the default effort" to stderr and starts at
  the default. `med` is read as `medium`, and `ultracode` as `xhigh`
  with the release's dynamic workflows on.
- `effortLevel` in its settings schema,
  `U(["low","medium","high","xhigh"]).optional().catch(void 0)`,
  "Persisted effort level for supported models", and the same key per
  model under `modelSettings.<model>`. `max` is no value of it: the
  release calls `max` "session-only (nothing saved)", and
  `.catch(void 0)` drops any other value with no word. `maxEffortLevel`
  is a cap: "Anything above it (an /effort or /model pick, --effort,
  CLAUDE_CODE_EFFORT_LEVEL, a model default) is clamped to it".
- The order it reads them in. A hook's effort comes first, then
  `CLAUDE_CODE_EFFORT_LEVEL`, then the turn's, then the session's, then
  the model's default. The session's is `--effort` where given, and
  otherwise the settings, each source's entry for the model before its
  top-level `effortLevel`. The level is then clamped to what the model
  offers, `max` or `xhigh` read as `high` on a model without them, and
  to `maxEffortLevel`.
- The hooks. SessionStart's input gives `model` and no effort. The
  plugin's engine types say a hook in a tool-use context, PreToolUse,
  PostToolUse or Stop, gets `effort.level` "after any silent downgrade
  for the selected model", absent for session-lifecycle hooks.

**The program-settings change**, `an-agents-program-settings-come-from-flumes-layers`,
in code in its place. It hands Claude Code the keys the layers resolve
as `--settings <file>`, a source above the user's, the project's and
the local settings. Its profile reserves `model` for flume's model and
`permissions.defaultMode` for flume's mode, and leaves `effortLevel`
open. Once it lands, a layer can give
`programs.claude.settings.effortLevel = "xhigh"` with nothing of this
change.

**crew.** Every role's frontmatter names `model` and `effort`: `xhigh`
for the planner, the design agent and the coder, `high` for construct,
verify, the reviewer, the conductor, ops and main-ops, and `medium` for
the dispatcher and the operator. A team or a partition overrides
either under `roles.<role>` in the teams file (crew's `teams.6`), and
crew's words for an effort are Claude Code's five (`EFFORTS` in
`plugin/lib/crew.py`). `crew-role` starts a role with
`claude --model <model> --effort <effort>`. On `teamless/deneb`, a
main-level role run as a flume kind has a `roles.rec` record with its
`Model` and its `Effort`, an enum of the same five, and the runner
launches the run with the record's model and "with its effort once
Flume carries one, until then the pool account's". crew's
`plugin/flume/kinds.toml` names no model and no effort: the launch
gives them.

**A kind.** weir's format holds an agents table's settings as written,
each the reader's to check (weir 52, C.4). sounding's elaboration kind
already writes `effort = "high"` in a stage's `agents` entry, as a
test in `crates/flume/tests/profiles/kinds_weir.rs` reads it, and
flume's `map_agent` ignores it today as a key that is "the runner's".

**Codex 0.165.0.** `model_reasoning_effort` in its configuration, given
for one run by `-c model_reasoning_effort=<word>`, its value TOML, and
a `plan_mode_reasoning_effort` beside it. Its binary holds `none`,
`minimal`, `low`, `medium`, `high`, `xhigh` and `max` in one run of
its strings, then `ultra` and `persistent`, which may be another list's;
which words it takes is read when its entry is built. **Gemini CLI 0.66.0.** No option: a thinking level or budget
lives only in its settings' `modelConfigs` overrides,
`generateContentConfig.thinkingConfig.thinkingLevel` or
`thinkingBudget`.

**The rest of the family.** sluice and timber start no agent. crew is
the one tool that gives an agent an effort, by Claude Code's option.

### Why it does not belong in the program-settings change

With that change landed, a layer could give Claude Code's
`effortLevel` and the planner and the design agent would run at
`xhigh` with no code of this change. It falls short three ways:

1. **`max` cannot be given.** Claude Code's settings take four of
   crew's five words and drop `max` with no word, so a partition that
   moves a role to `max` gets the account's default, while status names
   `max` from its layer.
2. **A kind names one program.** An `effortLevel` names Claude Code's
   key, so a kind run on a pool of another program's accounts needs
   one key per program in its own words, and Gemini CLI has no key for
   it at all. The model, the effort's companion, is flume's own setting
   for exactly this reason (record 0026).
3. **The effort follows the model.** The program clamps an effort to
   what the model offers. A host that sets a stream's model in flume's
   layers would set its effort in the program's own settings, two
   places for one choice.

So the effort is a setting of flume's beside the model, and the
program-settings change's part in it is one reserved key (design 6).

## Goals / Non-Goals

**Goals:**

- A launch, the host, a repository and a kind each give an agent's
  effort, for every stage or for one, in clause 32's layers, taken
  whole from the first that gives it, as the model is.
- The effort is the program's own word, passed by each profile its own
  way, and refused before any agent starts where the program does not
  take it.
- No effort passed where no layer gives one.
- Status names each agent's effort with its layer, and says where the
  account's environment outranks it.
- crew's runner can give a role's run its definition's effort at the
  launch, `max` among them.

**Non-Goals:**

- A scale of flume's own mapped to each program. Programs' words
  differ, Claude Code's five beside Codex's `none` and `minimal`, and
  Gemini CLI's thinking levels in its model configurations, and no mapping is
  "no wider and no narrower", as a mode's must be (169).
- A table of programs for the effort. The model's table (172) is built
  by record 0026's change, with the common profile, and the effort's is
  built with it; until then a table is refused, as the model's is.
- An account usable only where its program takes the effort. It is
  173's, built with the rest of 173 by record 0026's change. Every
  program flume runs today is Claude Code, which takes every word its
  profile names, so no choice of account changes meanwhile.
- The effort read back. Claude Code's SessionStart input gives none,
  and a tool-use hook's gives the level after the model's clamp, which
  is not the word asked. Status says the effort was passed and not read
  back.
- Codex's and Gemini CLI's ways (design 11), built with their entries.
- `maxEffortLevel` and `modelSettings`, which stay open as Claude
  Code's own settings (design 6).
- crew's runner, which is crew's to change (design 12).

## Decisions

### 1. The effort is a setting of flume's, beside the model

`Settings` gains `effort: Option<String>`, taken whole by
`Settings::under` as the model is, and read with none from a held
state written before. Clause 32 lists it among the settings, between
the model and the mode, and the host's four tables give it as they
give the model (32).

Alternatives considered:

- **Claude Code's `effortLevel` through the program-settings change.**
  No mechanism of this change's. Rejected for the three reasons in the
  Context, `max` first. It is the user's to rule on, as the question
  below says.
- **A scale of flume's own, mapped per program, as a mode is.**
  Rejected: a mode means the same on every program (169), but an effort
  is a level of one model, and the levels differ by program and by
  model. A mapping would choose for the host what `high` means on
  Codex.

### 2. One word, the program's own

An effort is one string, the agent program's own word, as written. A
layer that gives a table, a number or any other value is refused,
naming the table and `effort`, as a model's table is today (172). When
record 0026's change builds the model's table of programs, the effort's
is built with it, taken whole as a mode's table is (169), and this
change's refusal of a table goes.

### 3. In every layer, as the model is

Every agents table reads `effort` beside `model`:

- a kind's declaration, `[kinds.<kind>.agents]`, a stage's `agents`
  entry, and a repository's `.config/flume.toml` for a kind it does not
  declare, through the table weir's crate returns as written (weir 52,
  C.4; core.4);
- the host's `[kinds.<kind>.agents]`,
  `[repos."<repository>".kinds.<kind>.agents]` and their `.stages.<stage>`
  tables, and, once the program-settings change has landed, its
  `[agents]`, `[agents.streams.<stream>]` and
  `[repos."<repository>".agents]`, each a key the local layer may set
  over the managed one (C.7; record 0032);
- the launch, `flume launch --agent [<stage>.]effort=<word>`.

It resolves in `Engine::resolve` as the model does: taken whole from the
first layer that gives it, a stage's own before every stage's, with
`from["effort"]` naming its layer (32). The launch's, the
repository's and the kind's are those the launch read, and the host's
as they stand at each start (21, 32).

A kind's declaration may give an effort: like a model, it names nothing
a host owns (158). crew's planner kind could say
`[kinds.crew-planner.agents] effort = "xhigh"`. crew gives it at the
launch instead, since its role's effort comes from the definition and
the teams file alone (crew's `teams.6`).

### 4. None where none is given

With no effort in any layer, nothing is asked of the profile, its
command passes no effort, and the account's own configuration decides,
its settings' `effortLevel` or the model's default among it. Status
gives no effort (162, 197).

### 5. Each profile takes its program's words or refuses

The agent seam gains one reading:

```rust
/// Whether the agent's program takes `word` as its effort, which its
/// command passes as written; words naming the effort, the program
/// and the words it takes where it does not, and every word refused by
/// default, since a profile that names no way to give an effort
/// refuses one (197).
fn effort(&self, word: &str) -> Result<(), String>;
```

`Agent::command` passes `settings.effort` as the profile's way gives it,
and nothing where it is none; the core asks `Agent::effort` before any
command is built, so a command never holds a word the profile refused.

Each profile names its program's words from the release it pins, and
refuses any other. The model is passed as written because its names
are open: aliases, full names, new models with no release of the
program. An effort's words are a closed list in each release, and
Claude Code ignores an unknown one with a warning and starts at its
default, so a word passed unchecked could start an agent at a level
other than the one status names. Claude Code's profile takes `low`,
`medium`, `high`, `xhigh` and `max`, and refuses `med` and `ultracode`,
which the release reads as other words.

### 6. Claude Code by `--effort`

`claude/effort.rs` holds the release's five words and `fragments()`:
the option with its words, `"--effort <level>",\`Effort level for the
current session (${...join(", ")})\``, as each release names its list,
and the list itself, `["low","medium","high","xhigh","max"]`. A
`check-tools` test in `crates/flume/tests/profiles/tools/claude_words.rs`
requires each byte for byte in the pinned release (C.4; core.22). The
profile's command passes `--effort <word>` after `--model`, where the
settings give one.

`--effort` is the session's own level, which the release reads before
any settings file's, the account's and the layers' own among them.
`effortLevel` becomes the effort's: once the program-settings change has
landed, `claude/settings.rs` reserves it with the owner "flume's
`effort`", beside `model`, its declaration in the release's schema
among the fragments that change's `check-tools` test reads. A layer's
`settings` that writes it is refused naming the key and its owner, and
a named file's is left out and named in status (195). So the effort has
one owner, as the model has.

`modelSettings` and `maxEffortLevel` stay open. `modelSettings` holds a
model's compaction window and cap beside its effort, and a layer's
entry for a model reaches the agent only where no layer gives flume's
effort, since `--effort` comes first; status names it as the own
setting it is. `maxEffortLevel` is a cap a host may set on every agent,
which clamps flume's effort as it clamps a person's pick.

### 7. The account's variable named in status

Claude Code reads `CLAUDE_CODE_EFFORT_LEVEL` over `--effort`. An
account's `env` is the account's to give (144), and flume neither
refuses nor removes a variable of it. The agent seam gains
`Agent::effort_variable() -> Option<&'static str>`, none by default,
`CLAUDE_CODE_EFFORT_LEVEL` for Claude Code, held in `claude/effort.rs`
with the release's fragment naming it. Where the agent's account's
`env` names that variable and a layer gives an effort, status says the
account's variable outranks the effort passed, naming the variable and
never its value (56; core.30). Where no layer gives one, the variable
is the account's configuration deciding, and status says nothing more
than it says of any variable.

### 8. Refusals at the launch and at a later start

`Engine::launch` resolves every stage's effort and asks the agent's
profile of it before anything is made, as it does a mode (4, 8). A
word the profile refuses, or an effort for a program whose profile
names no way to give one, refuses the launch:

```
launch refused: the stage code's effort `med`, from host-kind, is no
effort claude takes: low, medium, high, xhigh or max
```

A host's table changed after the launch is read at the next start, and
a word refused then leaves the stage refused in the same words, starting
no agent, until a retry (27, 32).

### 9. Status and the JSON form

`AgentStatus` gains `effort`, an object only where a layer gave one:
the word, its layer, `passed` as the profile passed it, `read_back:
false`, and `outranked_by` naming the account's variable where design 7
finds it. The person's form prints it on the agent's line,
`effort xhigh (launch), passed, not read back`, adding "the account's
`CLAUDE_CODE_EFFORT_LEVEL` outranks it" where it does. The JSON form
only gains fields (56; core.14).

### 10. The start's record

A start records its program, mode, account and variables (161), and no
model. It records no effort either: nothing reads the effort back to
compare. A start under an effort its layers gave is the stage's on the
same word as any other (35).

### 11. Every other program's way

Written into Part C when each entry is built and checked against the
release it pins (170; core.39):

| program | how it takes an effort | its words |
|---|---|---|
| Codex 0.165.0 | `-c model_reasoning_effort=<word>` | its release's, read when its entry is built from the list its binary holds, `minimal` to `max` among it |
| Gemini CLI 0.66.0 | none: a thinking level only in its settings' `modelConfigs` | its entry refuses an effort |

Until each entry is built, `Agent::effort`'s default refuses, so a host
that gives an effort for a stage running another program sees its
launch refused naming the program.

### 12. What this change needs and what it hands on

- **The program-settings change lands first.** This change builds on
  its eight layers, its host tables for every agent, a stream and a
  repository, and its list of Claude Code's reserved keys. Its code
  stage merges main once that change has landed, and every delta here
  that names those layers is fitted to the specs as they then stand.
  Should that change be dropped, the effort is read in clause 32's
  layers as main holds them and design 6's reserved key goes.
- **crew's runner.** It gives a role's run `--agent effort=<Effort>`
  from the role's `roles.rec` record beside `--agent model=<Model>`, and
  crew's README and its run's record lose "until then the pool
  account's". That is crew's change, told to crew's operator with the
  user's words; nothing here waits for it.
- **The questions change.** Its role kinds, the planner's and the design
  agent's, run on a pool whose accounts are Claude Code's, so `xhigh`
  and `max` are taken.

### 13. The statement, the record and the rules

Clause 197 and S103 are new, after the program-settings change's 195,
196 and S100 to S102, since the designs in flight hold 193, 194 and S93
to S99. The statement's amendment is on the blueprints branch
`design/flume-an-agents-effort`, cut from
`design/flume-an-agents-program-settings` at 94b4edb, since clause 32
and C.4 there hold the program-settings change's layers and reserved
keys, at `17a9116`, committed and not pushed:

- **197** (A.4, after 172), an agent's effort: its program's own word
  for how hard its model works on each turn, given in every layer of 32
  and taken whole as the model is; passed as its profile takes it; a
  word its profile does not take, or an effort for a program whose
  profile names no way to give one, refused at the launch or the
  stage; none where no layer gives one, the account's configuration
  deciding; status giving it with its layer as passed, and the
  variable of the account's that outranks it.
- **Section 3**, settings: "its program, its model, its effort, its
  mode, …".
- **4**: a launch whose stage's agent resolves to an effort its
  program's profile does not take is refused, naming the effort, the
  program and the stage (197).
- **32**: the effort among the settings and the host's tables' settings;
  an effort its profile does not take among a later start's refusals.
- **88**: the command passes the effort as its profile takes it.
- **158**: a kind may name an effort, by its program's word.
- **170**: an entry names how its program is given an effort and the
  words it takes, and an entry with neither refuses one.
- **173**: an account usable where its program takes the effort given
  or none is given.
- **195**: the effort among the settings whose keys a profile reserves.
- **C.2**: `effort` in an agents table, a word, a table refused until
  the model's is built.
- **C.4**: the command row's `--effort`; `effortLevel` reserved as
  flume's effort's; `CLAUDE_CODE_EFFORT_LEVEL` named; the common
  profile's entries refuse an effort until each names its way.
- **C.7**: the host's agents tables give `effort`.
- **S103**: a planner's kind at `xhigh` and a launch at `max`.
- **weir C.4**: the example's agents table gains
  `effort = "high"`, "the agent's effort, its program's own word; the
  reader's to check". weir's crate returns the table as written, so
  flume's pin of weir-kinds stays (core.4).

Record 0044, `an-agents-effort-is-a-setting-layered-as-its-model-is`,
is proposed with this change, amending records 0026 and 0043. Rules
core.30 and core.39 are amended with it: core.30's model sentence gains
the effort, and core.39's refusal names an effort its profile cannot
honour.

## Questions for the user's ruling

1. **Is the effort a setting of flume's, or Claude Code's own
   `effortLevel`?** Recommended: flume's, beside the model, as this
   design has it. It carries `max`, holds for every program, and sits
   with the model it follows. The other answer is the program-settings
   change with nothing added: a layer gives
   `programs.claude.settings.effortLevel`, `max` cannot be given, and
   this change becomes one paragraph of that change's docs and a
   scenario of S102's kind. If the user rules so, this change is
   withdrawn, record 0044 is rejected, and the blueprints branch is
   dropped.

## Risks / Trade-offs

- **An account's variable outranks the layers.** An account whose `env`
  sets `CLAUDE_CODE_EFFORT_LEVEL` runs every agent at that level
  whatever the layers give. Status names it (design 7), and the site's
  guide tells a host to give its effort in its layers rather than in
  an account.
- **The model clamps the effort.** `xhigh` or `max` on a model that
  does not offer it runs at `high`, and status names the word passed.
  The Stop hook's `effort.level` could read the level run, a later
  change's if status should give it.
- **`effortLevel` refused where a table writes it.** A host that gives
  `programs.claude.settings.effortLevel` after the program-settings
  change lands has its next start refused, naming the key and
  `effort`, until it moves the word to `effort`. A named file's is left
  out, and status names it.
- **A kind's `effort` is read where it was ignored.** A declaration
  that writes `effort` today, sounding's among them, is read once this
  lands, and a word its program does not take refuses its launch. No
  kind of flume's own names one.
- **Numbers.** Clause 197, S103 and record 0044 are free on every branch
  and in every place's draft on this host on 2026-11-05, and the
  program-settings change was renumbered once already that day. Task
  1.1 checks them again before the code builds on them.
- **The order of landing.** Built before the program-settings change,
  the effort would miss its new layers. Group 1's first task merges
  main once that change has landed, and not before.
