# Design

## Context

flume's main at f29f182 runs a kind through on its own, unattended
under herdr, with ends delivered to recipients, holds and stalls
(`flume-runs-unattended-and-wakes-roles`, archived 2026-10-29). Each
start of a stage is one fresh agent (30; core.10): `Engine::start_stage`
in `crates/flume-core/src/start.rs` claims the start under the host's
room lock, fills the prompt by `Engine::fill` and `prompt::render`, and
opens the agent; `Engine::read_end` in `end.rs` reads the deliverable by
`Engine::delivered` in `reading.rs`, and `Engine::advance` in
`settle.rs` starts the next stage once the agent has ended. A stage's
declaration reaches flume through weir-kinds, pinned at a214b95, whose
`Stage` has no field for repeating; `map_stage` in
`crates/flume/src/profiles/kinds/weir/mod.rs` maps it onto
`flume_seams::Stage`. The OpenSpec profile's `tasks` reads the
checkbox lines of `tasks.md` and nothing of its headings.

Record 0019 (accepted 2026-10-29) lets a stage repeat per task group.
The statement carries it at agentplot/blueprints d75cf53: 140 to 142
added, 22, 23, 30, 86, C.2, C.3, C.7 and S44 amended, S50 to S52 added,
and weir's 53, 57, C.4 and S10 amended. The rules this design relies on
are core.4 (the format is weir's), core.9 (an end read from the
deliverable at one head), core.10 (one fresh agent per start, a
repetition among them), core.15 (status reads where facts live),
core.16 (nothing of flume's in a repository), core.24 (a hold's words),
core.26 (no slot held for a place), core.27 (the fix round) and core.31
(the prompt's order and the notes). The one rules page is
`docs/architecture/core.md`; this change touches no area another page
would own. Where flume has no rule, the kit standard's code profile
binds (kit:specs.3, kit:testing.5).

## Goals / Non-Goals

**Goals:**

- A stage that declares `repeat = "per-task-group"` is worked one task
  group at a time, one fresh agent per group, with flume settling
  between them (140, 142).
- flume's own change at version 2 repeats its code stage, its prompt
  ordered so each repetition's start is read from the prompt cache (22,
  C.2; core.31).

**Non-Goals:**

- Repeating by anything but a change's task groups, or a stage whose
  deliverable is not tasks ticked: the format refuses it (weir 57).
- Several agents at once in one place, or groups worked side by side
  (30; core.10).
- Reading, seeding or summarizing the notes (141; core.31).
- Keeping change@1 as a second built-in kind: a declaration holds one
  kind per name (19), so places launched at change@1 keep it (21) and a
  repository that wants it declares it.

## Decisions

### 1. `repeat` is weir's, mapped onto flume's `Stage`

weir-kinds' `Stage` gains `repeat: Option<Repeat>`, `Repeat` an enum with
the one variant `PerTaskGroup`, spelled `per-task-group`, and
`STAGE_FIELDS` gains `repeat`. The read refuses, naming the field, any
other value, and `repeat` on a conversation or a stage whose deliverable
is not `tasks` (weir 53, 57, C.4). That lands in agentplot/weir first,
under weir's own rules, and flume takes the new revision in
`Cargo.toml` and `Cargo.lock` (core.4, core.22).

`flume_seams::Stage` gains `repeats: bool`, serde default false, so a
held state written before reads as before; `map_stage` sets it from
`repeat`. `Engine::runs` refuses nothing new: weir has refused the
shapes flume cannot run.

Rejected: reading `repeat` from the stage's `sections` in flume, which
is a field of the format defined in flume (core.4).

### 2. A change's task groups

`flume_seams::Task` gains `group: Option<usize>`, an index into a new
`ChangeFacts::groups: Vec<TaskGroup>`, each `TaskGroup { number:
Option<String>, title: Option<String> }`, both serde default.
`ChangeFacts` gains `first_unticked_group()` and `tasks_of(group)`. The
OpenSpec profile's `tasks` reads each `## <n>. <title>` heading of
`tasks.md` as a group, every checkbox line under it in that group, and
checkbox lines above every heading in a group of their own with no
number or title; any other heading starts no group (C.3). The heading's
reading is the profile's, so the core names no change tool (core.1,
core.2). The fake change takes groups in its script.

### 3. A repetition is a start with a group

`StartHeld` gains `group: Option<GroupHeld>`, serde default, holding the
group's place among the change's groups, its number, its title and its
tasks' texts as read at the start's head. `Engine::claim` reads the
change at the head it records as `start_head` and, for a repeating
stage, records the first group holding a task unticked, or none when
every task is ticked or the change holds none there (140). A
repetition is otherwise a start: its identity's start number, its
secret, its prompt's sendings, its `first_quiet`, `begun_at` and
`past_limit` are a start's own, so the quiet limit (44) and working
past it (118) already count per repetition.

`Engine::delivered` reads a repetition with a group as delivered when
the change at the head holds a group of the same number and title with
no task unticked, and the newest commit touching the change was made
since the repetition started (25); a group no longer found is not
delivered. A repetition with no group, and a stage that does not
repeat, read tasks ticked as today (24). Every fact is read at one head
(core.9).

### 4. The settle between repetitions

`StageState` gains `Repeating`: a repetition delivered, its agent to be
ended, and a task still unticked. `Engine::read_end` reads a delivered
repetition at the same head: every task ticked is `Delivered`, the
stage's end, recorded in `ends` and delivered to recipients (43);
otherwise `Repeating`, with nothing recorded in `ends` (140). `step`
treats `Repeating` as it treats `Delivered`: once the agent has ended,
`Engine::advance` starts the same stage again by `start_stage`, which
waits for room like any start (70; core.26), unless a hold names the
stage, which leaves it `OnHold` (decision 6). A failed submission's or
a finding's return, and a retry, start the stage as today; the claim
finds the group (decision 3), so they need no path of their own (112,
131; core.27).

Rejected: keeping the place's room between repetitions, a slot core.26
rules out.

### 5. `{{group}}` and `{{notes}}`

`PromptPart` gains `Group` and `Notes`. `Engine::fill` fills `Group`
for a repeating stage with the start's group, `Group <n>. <title>` and
its tasks' lines as the change gives them, or with words saying there
is no group, since every task is ticked, and that the words below say
what to fix, recorded as a new task ticked; and `Notes` with the path
`notes/<stage>.md` under the place's own folder,
`Store::folder` (141, C.7). Neither fills for a stage that does not
repeat, which leaves it refused naming the part (27). flume creates,
reads and removes nothing at that path; the folder goes at the tidy
(62; core.16, core.31). The stop check's words for a repetition name
its group and the tasks left unticked (110).

### 6. Status, holds, stops, ends and retries per group

`StageStatus` gains `group`, with its place among the change's groups at
the head, how many there are, its number and its title, read from the
change when status is asked (58; core.15), never from the notes. The
person's form prints the stage as `code 3/7`, and the herdr clue's title
follows it (139). A stage's end and the notice of it carry the group
where it ended short, stuck, lost or never begun (43, 142). A stop's
answer and status name the stopped repetition's group (51).

`Engine::hold` takes a hold on a repeating stage while it runs, as a
hold on its next repetition: the running repetition goes on, and once
it delivers, `advance` leaves the stage `OnHold` with the hold's words,
status naming the group it would take (130, 142; core.24). A retry
naming the stage ends the hold and starts a repetition, which takes the
first unticked group: the group that ended short, never the stage's
first (49, 142). The stall counts from the moment the place began to
wait, which is the repetition's end (130).

### 7. change@2 and its code prompt

flume's own declaration, `crates/flume/src/profiles/kinds/weir/flume.toml`,
gives `change` version 2 and its code stage `repeat = "per-task-group"`;
construct and verify are as they were (22, C.2). Its code prompt,
`prompts/change-code.md`, is in this order: the stage's instructions,
which say the agent is one repetition, reads the change at `{{change}}`
and the notes, builds only its group, ticks each task and commits as it
goes, leaves in the notes what the next repetition needs, and never
commits the notes; the place, branch and base; the way to stop short
(`{{needs}}`); and last its group (`{{group}}`), the notes' path
(`{{notes}}`) and the `{{#words}}` section, with the merge, the
unarchive and the fix it asks for today (112, 131). Everything before
the group is the same for every repetition of a place, so a fresh
agent's prompt matches the one before it up to its group (141;
core.31). fix@1's and chore@1's prompt is unchanged.

## Risks / Trade-offs

- A repetition re-reads the change and the notes once per group. The
  record weighs that against a long-lived agent's re-reading on every
  turn; nothing in flume measures it.
- A group renamed by its own repetition is not found at the head, and
  the stop check sends the agent back; the prompt tells it to leave its
  group's heading as it is.
- The notes can grow without bound across many groups. flume never reads
  them, so it cannot trim them; the prompt asks for short notes.
