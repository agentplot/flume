# A stage repeats per task group

## Why

change@1's code stage is one agent for a whole change (30). flume's
changes are written as task groups, each one session's work, and one
agent working every group re-reads its whole conversation on every turn
as it grows, then compacts and loses the detail of the groups before.
Record 0019 (accepted 2026-10-29) lets a stage repeat: flume starts a
fresh agent for each task group in turn, in the same place, and settles
between them, so no agent holds a growing context and flume sees each
group (140 to 142).

## What Changes

- **The kind format's `repeat`.** weir-kinds reads and checks
  `repeat = "per-task-group"` on a stage that delivers `tasks`, refusing
  any other value or stage (weir 53, 57, C.4); flume takes that weir
  revision and maps the field onto its `Stage` (core.4).
- **Task groups.** The change seam gives each task its group, number and
  title, read by the OpenSpec profile from the `## <n>. <title>` headings
  of `tasks.md` (86, C.3).
- **Repetitions.** A repeating stage starts one fresh agent per group,
  each a start of the stage given the first group with a task unticked.
  A repetition delivers once its group is ticked in a commit since it
  started; flume ends it and starts the next, waiting for room like any
  start, and the stage delivers once every task is ticked. A start with
  every task ticked is one repetition with no group (140).
- **Two prompt parts.** `{{group}}` renders the repetition's group, and
  `{{notes}}` the path of the stage's notes, `notes/<stage>.md` in the
  place's own folder, which flume names and never reads (141, C.7;
  core.31).
- **Between repetitions.** Status gives `code 3/7`; the quiet limit,
  working past it and a stall count per repetition; an end, a stop and a
  hold name the group, and a hold on a running repeating stage holds its
  next repetition; a retry starts the first unticked group again (142).
- **change@1 and change@2.** flume's own kinds are held one declaration
  per version, and change@2 joins change@1 beside it, its code stage
  repeating, with a code prompt whose stable parts come first and whose
  group, notes and words come last, so each fresh agent's start is read
  from the prompt cache (22, C.2; core.31). change@1 stays as it is. A
  launch that names no version takes change@2, the newest, and one may
  name `change@1` (154; record 0022). Places launched at change@1 keep
  it (21).

## Capabilities

### New Capabilities

- `repeating-stages`: a stage worked one task group at a time, a fresh
  agent for each; a repetition's delivery and the stage's; the group and
  notes parts; what is counted, named and retried per repetition.

### Modified Capabilities

- `kinds-profile`: `repeat` read through weir-kinds; flume's own kinds
  one declaration per version, change@1 and change@2 side by side,
  change@2 with its repeating code stage and its prompt's order.
- `launching`: a launch's kind named with its version, or with none for
  the newest of flume's own.
- `change-profile`: each task with its task group.
- `caller-waits`: a hold on a running repeating stage holds its next
  repetition, and names its group.
- `place-status`: a repeating stage with its group among the change's
  groups.
- `clause-coverage`: the suite's list at blueprints d75cf53, 140 to 142
  and S50 to S52 among it.

## Impact

- agentplot/weir: weir-kinds gains `repeat` on a stage, landed there
  through sluice before flume reads it (core.4).
- `crates/flume-seams`: `Stage` gains its repeat, `Task` its group,
  `PromptPart` `Group` and `Notes`, and a start its group.
- `crates/flume-core`: the repetition in the settle, the deliverable of
  a repetition, the parts filled, status, holds, stops and retries per
  group, and the launch resolving a kind's name and version.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake change's
  groups; the engine scenarios for each new requirement, S50 to S52
  among them; the clause list at d75cf53.
- `crates/flume`: the weir revision in `Cargo.toml` and `Cargo.lock`,
  the kinds profile's mapping, the OpenSpec profile's groups, flume's
  own kinds one declaration per version with change@2 and its code
  prompt beside change@1, the status line, and
  the walkthrough of a repeating change with the stand-in agent.
- No new crate, plugin or site part, so no new moon project.
