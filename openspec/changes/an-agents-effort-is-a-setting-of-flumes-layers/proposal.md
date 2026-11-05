# An agent's effort is a setting of flume's layers

## Why

crew's main-level roles name a model and an effort in their agent
definitions: the planner, the design agent and the coder run at
`xhigh`, most other roles at `high`, the dispatcher and the operator at
`medium`, and a partition may move a role to `max`. crew starts each
role with `claude --model <model> --effort <effort>`. flume gives an
agent its model in its layers (32, 172) but no effort, so a role run as
a flume kind starts at its account's default effort. crew's README
says so: the runner launches a role's run "with its effort once Flume
carries one, until then the pool account's".

The user, 2026-11-05, in the questions design
`every-question-a-place-asks-reaches-someone-who-answers-it`: "That
order; planner and design agent wait for an effort setting." The
planner and the design agent move onto their kinds only once flume
carries one.

Claude Code's own settings have an effort, `effortLevel`, which the
program-settings change would let any layer give. That key takes `low`,
`medium`, `high` and `xhigh` and silently drops anything else, `max`
among them, since `max` is a level for one session only. It is also
Claude Code's own word, so a kind that gives it names one program. The
model is the effort's companion, and the program clamps the effort to
what the model offers. The model is a setting of flume's, given in the
layers, passed by each profile its own way or refused (172). The
effort becomes one beside it (design 1).

## What Changes

- **An effort in every layer.** `effort` is a field of every agents
  table, the kind's, a stage's, the repository's and the host's, and of
  a launch's settings, `flume launch --agent [<stage>.]effort=<word>`.
  It resolves as the model does, taken whole from the first layer of
  clause 32 that gives it, a stage's own first (32, 197).
- **The program's own word.** An effort is the agent program's own word
  for how hard its model works on each turn, such as Claude Code's
  `xhigh`. flume maps no effort from one program to another and
  defines no scale of its own (197).
- **None where none is given.** With no effort in any layer, flume
  passes none and the account's own configuration decides, as for a
  mode (162, 197).
- **Each profile takes its program's words or refuses.** Each agent
  profile names the words its program takes, read from the release it
  pins, and passes one its own way. A word its program does not take,
  or an effort for a program whose profile names no way to give one,
  refuses the launch, or at a later start the stage, naming the word,
  the program, the stage and the layer (4, 32, 197).
- **Claude Code by `--effort`.** Claude Code's profile passes
  `--effort <word>` and takes the pinned release's five words, `low`,
  `medium`, `high`, `xhigh` and `max`. A `check-tools` test finds the
  option and the five words in the pinned release (C.4; core.22).
- **`effortLevel` is the effort's.** Once the program-settings change
  has landed, Claude Code's profile reserves its settings' `effortLevel`
  for flume's effort, as it reserves `model` for flume's model. A layer
  that writes it is refused, and a named file's is left out and named
  in status (195, C.4).
- **Status.** Each agent's effort, with its layer and the value passed,
  says it is not read back. Where the account's environment names
  Claude Code's `CLAUDE_CODE_EFFORT_LEVEL`, which the program reads
  over `--effort`, status says so by the variable's name and never its
  value (56, 197; core.30).
- **Outside this change.** A table of programs for the effort, built
  with the model's by record 0026's change. An account usable only
  where its program takes the effort, which is 173's and built with the
  rest of 173 by that change. Codex's and Gemini CLI's ways, written
  into their entries when they are built. The effort read back from
  the agent's own record. crew's runner passing a role's effort, which
  is crew's.

## Capabilities

### New Capabilities

- `agent-effort`: an agent's effort, its program's own word, given in
  every layer of clause 32 and taken whole as the model is; none passed
  where none is given; each profile takes its program's words or
  refuses; refused at the launch or a later start.

### Modified Capabilities

- `agent-profile`: Claude Code passes the effort by `--effort`, takes
  the pinned release's five words, reserves `effortLevel` for it, and
  names `CLAUDE_CODE_EFFORT_LEVEL` as the variable it reads over it.
- `place-status`: an agent's effort, its layer, the value passed, and
  the account's variable that outranks it.
- `command-line`: `--agent [<stage>.]effort=<word>`.
- `kinds-profile`: a declaration's agents tables give `effort`.
- `store-profile`: the host's agents tables give `effort`.
- `seam-contracts`: the agent seam takes an effort or refuses it, and
  names the variable its program reads over it.
- `seam-fakes`: the fake agent scripts the efforts it takes.
- `clause-coverage`: the list at the statement's revision holding 197
  and S103.

## Impact

- agentplot/blueprints: the statement gains clause 197 and S103 and
  amends section 3's settings, 4, 32, 88, 158, 170, 173, 195, C.2, C.4
  and C.7. weir's C.4 example gives `effort` in an agents table, the
  reader's to check. They are at blueprints `17a9116` on the branch
  `design/flume-an-agents-effort`, cut from
  `design/flume-an-agents-program-settings` at 94b4edb, committed and
  not pushed (design 13).
- `docs/adr`: record 0044, proposed; rules core.30 and core.39
  amended with it.
- `crates/flume-seams`: `Settings::effort`, taken whole by
  `Settings::under`; `Agent::effort` and `Agent::effort_variable`, each
  refusing or naming nothing by default; status fields, only added.
- `crates/flume-core`: `settings.rs` folds the effort as the model,
  checks it through the agent's profile at the launch and at each
  start; `status.rs` gives it.
- `crates/flume`: the kinds profile's `map_agent`, the host's agents
  tables and `--agent` read `effort`; a new `claude/effort.rs` with the
  release's words and fragments; the command's `--effort`;
  `effortLevel` among `claude/settings.rs`'s reserved keys; the
  person's form of status; the stand-in's `--effort`; one walkthrough
  and the live fix with an effort.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake agent's
  efforts, a new `engine/agent_effort.rs`, and the clause list.
- `site/`: the agent-settings, kinds and Claude Code guides, and the
  configuration, commands and state references.
- Built after `an-agents-program-settings-come-from-flumes-layers`
  lands, on its eight layers and its reserved keys (design 12).
- No new crate, plugin or site part, so no new moon project; no new
  dependency.
