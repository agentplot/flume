---
number: 19
title: A stage may repeat per task group, a fresh agent for each group, and change@2's code stage does
status: accepted
date: 2026-10-29
decision-makers:
- Chuck Swanberg
---

# A stage may repeat per task group, a fresh agent for each group, and change@2's code stage does

Status: accepted by the user, 2026-10-29, in their design session ("Yes, write it and plan the build"), on how a change's code stage builds a change of many task groups without one agent holding all of it.

Amended 2026-10-29 by record 0022, accepted by the user: flume's own change@1 and change@2 are both kept, side by side, each a package of its own; a launch that names no version takes the version its repository's lock records, else change@2, the newest; and a repository picks change@1 by locking `flume/change@1`, a launch by naming `change@1`. The versioning below, under which flume's own `change` moved to version 2 outright and a repository wanting change@1 declared it itself, no longer holds.

Deciders: the user, who chose a fresh agent per task group with flume as the orchestrator, the two prompt parts, the notes flume never reads, and that change@2 repeats its code stage; the field's home in weir, the reading of a repetition's end, the prompt's order and the versioning as the code allows are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at d75cf53, clauses 22, 23, 30, 86, 140 to 142, C.2, C.3, C.7, S44 and S50 to S52, and `design/weir/requirements.md` clauses 53, 57, C.4 and S10, both as amended with this record; records 0003, 0004, 0008, 0011, 0017; rules core.4, core.10, core.31; weir-kinds at a214b95 (`Stage`, `STAGE_FIELDS`); `crates/flume-core/src/prompt.rs` and `start.rs`; `crates/flume/src/profiles/kinds/weir/flume.toml`.

## Context and problem statement

change@1's code stage is one agent for the whole change (30; record 0003). Its prompt tells it to work through every task in order. flume's own changes are written as six to nine task groups, each "one session's work for an agent that reads this change and its group", and a code stage that runs them all in one agent re-reads its whole conversation on every turn as that conversation grows. Somewhere in the fourth group it compacts, and what it compacts away is the detail of the earlier groups it was asked to keep consistent with.

The groups were written to be worked apart, so a person running a change today starts an agent per group by hand, or tells the one agent to start a subagent per group. Neither is something flume sees: status shows one long code stage, its quiet limit and its working-past-limit cover the whole stage, and a stop or a retry throws away every group's progress.

## Considered options

1. Prompt only: change@1's code prompt tells its agent to start a subagent per task group.
2. Keep one long-lived agent for the stage and let it compact.
3. A stage may repeat per task group: flume starts a fresh agent for each group in turn, in the same place, and settles between them.

## Decision outcome

Option 3.

**The declaration.** A stage says it repeats with `repeat = "per-task-group"`, and only a stage whose deliverable is `tasks` may. The field is the kind format's, so it is weir's to read: `Stage` in weir-kinds at a214b95 holds `name`, `prompt`, `end`, `agents`, `makes`, `wait`, `quiet`, `on_fail`, `max_send_backs` and `sections`, and a `repeat` written today would land in `sections`, which flume ignores (23, 56). flume reading it from there would be a field of the format defined in flume (core.4). So weir's statement names it (53, 57, C.4), weir-kinds reads and checks it, and flume's pin moves once it does, as it did for `makes` and for record 0017's `agents`.

**Each repetition.** A repetition is a start of the stage (33, 48), so its identity, secret and start count are a start's as they are today. Its agent is fresh, in the same place, and is given the first task group of the place's change that holds a task unticked at the head it starts at. It delivers once every task of its group is ticked at the branch's head, in a commit touching the change made since it started (24, 25). Once it has delivered and is quiet, flume ends it and starts the next repetition as it would start a next stage, waiting for room like any start (70) and holding no slot between repetitions (core.26). The stage delivers once every task is ticked, and only then is its end delivered to the place's recipients (43): a repetition's delivery is no end. A start that finds every task already ticked, a failed submission's, a verify finding's or a retry's after the stage delivered, is one repetition with no group, which records the fix as a task ticked and delivers as change@1's code stage does (112, 131; record 0011).

**The prompt's parts.** `{{group}}` renders the group's number, title and tasks as the change gives them, or that there is none. `{{notes}}` renders the path of the stage's notes, `notes/<stage>.md` in the place's own folder beside its held state (C.7). Each repetition reads the notes first and leaves in them what the next needs and the code will not tell it. flume keeps the path and never reads, writes or carries the notes' content (141), and they go with the place (62, 123), so nothing of them is a deliverable or a fact status reads. The change profile gives each task its group (86), from the `## <n>. <title>` headings of `tasks.md` (C.3).

**Between repetitions flume settles** (142):

- Status gives the stage with its group's place among the change's groups and how many there are, read at the head when asked (58), as `code 3/7`.
- The quiet limit (44), working past it (118) and a stall (130) run from each repetition, never from the stage's first start. `stall_after` is a place's wait on its caller and the quiet limit an agent's silence; both count per repetition.
- A hold and a stop name the group. A hold put on a repeating stage while it runs holds its next repetition, where today a hold on a started stage is refused (record 0008).
- A retry naming the stage starts the first group with a task unticked again, never the stage's first group: a fresh repetition always takes the first unticked group, so the retry needs no group of its own.
- A repetition that goes quiet with its group unticked is sent back by the stop check naming the group's tasks (110), and one that stops short, is stuck, lost or never begun is read as a stage that ends so is today (38, 41 to 45), its end naming its group.

**The prompt's order, for the prompt cache.** A repeating stage's prompts differ only in their last parts. change@2's code prompt puts its stable instructions first, then the place's and the change's paths, the way to stop short, and last the group, the notes' path and any retry's or failed submission's words. flume's renderer fills parts where the template puts them and appends only the way to stop short, which is the same for every start, so the order is the prompt's to keep (core.31). A fresh agent still benefits from caching. Its start, the system prompt, the tools, the repository's CLAUDE.md, the kind's instructions and the prompt up to its group, is the same for every repetition, and is read from the cache within the cache's lifetime. A long-lived agent instead re-reads its whole conversation on every turn as it grows, and compacts once it is full. The cost of a fresh agent is reading the change and the notes once per group, a cost that stays the same from group to group, where a long-lived agent's grows with every turn.

**Any agent program.** Nothing depends on subagents or on any feature of Claude Code past what a stage already uses: flume starts each repetition as a stage start, through the agent and terminal seams (core.1, core.2).

**Versioning.** A kind's version is the number its declaration gives it (19, 23), and a declaration holds one kind per name: weir keys kinds by name under `kinds`, and the launch resolves a name, never a version (`resolve_kind` in `crates/flume-core/src/launch.rs`). So change@1 and change@2 cannot both be flume's own. flume's own `change` moves to version 2 with its code stage repeating; its construct and verify stages are change@1's. change@1 stays as it was in the sense the format allows: a place launched at change@1 keeps it to the end (21), and a repository that wants change@1 declares it in its own `.config/flume.toml` with the code prompt of flume's history. fix@1 and chore@1 deliver commits, have no task groups, and do not change.

**Why not the others.** Option 1 needs an agent program that starts subagents, and flume would still see one long stage: its idle time, its limit and its status cover every group at once, and a stop or a retry cannot name a group. It stays open to a repository that wants it, by declaring its own change kind with that prompt. Option 2 keeps the cost the decision exists to remove: the agent re-reads more on every turn and loses detail of the earlier groups each time it compacts.

### Consequences

- Clauses 22, 23, 30, 86, C.2, C.3, C.7 and S44 of flume's statement are amended and 140 to 142 and S50 to S52 added, and weir's 53, 57, C.4 and S10 amended, at blueprints d75cf53. Rule core.10 counts a repetition as a start; core.4's details name `repeat` as weir's; core.31 is drawn from this record for the prompt's order and the notes.
- weir-kinds reads and checks `repeat`, and flume's pin of it moves once it does; until then `_open.md` names the wait.
- The OpenSpec change `a-stage-repeats-per-task-group` carries it: weir's field first, then the task groups of the change profile, the repetition in the core, change@2 with its prompt, status and the caller's acts per group, and the clause list at d75cf53.
