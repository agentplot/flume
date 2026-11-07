---
number: 54
title: A change is coded by one agent, each task checked by its own check, and the repository's whole checks run once, at verify
status: proposed
date: 2026-11-06
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# A change is coded by one agent, each task checked by its own check, and the repository's whole checks run once, at verify

Status: proposed to the user on 2026-11-06, as `briefs.15` has it; accepted only on their yes.

Amends record 0019 in part, and record 0022 where it names change@2 the newest, each marked so where it stands.

Deciders: the user, in agentplot-design's pane on 2026-11-06, on hearing that a change's code stage runs a fresh agent for each task group and that each group ends with a run of the repository's whole checks: "They split the coding task into groups for each set of OpenSpec changes. What's happening is that we're running the [...] tests for each of those runs and it's [...] slow", and "what are we going to do about the [...] design for the OpenSpec task groups? It's not right. I've never done it that way". Asked how a change's coding should run, they chose "One agent, test once": one agent builds the whole change, task by task, the groups only headings; each task checks only what it touched; the whole suite runs once at the end, in verify, then in the merge queue. agentplot-design for the shape below: a new version of flume's own change, the older versions kept, and the stage-repeat field kept in the format.

Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 22, 140 to 142, 154, 155, 157, C.2 and S119, as amended 2026-11-06; records 0003, 0019 and 0022; rule core.31; `crates/flume/src/profiles/kinds/weir/own/change@2/` (its code prompt asks for no check of a group; its verify prompt runs "the checks it names"); the places of deneb on 2026-11-06: `change--a-queues-test-keeps-the-slot-it-counted` in agentplot/sluice, whose six task groups each end with a task running `sluice exec main -- moon run` over most of the repository's projects, its group 4's run waiting 71 minutes for room on the host, and `sluice exec` runs of whole suites waiting 40 minutes to over two hours that evening under a bound of three runs; `openspec/config.yaml` of agentplot/sluice ("Every task names its verification"); the archived change `2026-10-28-flume-runs-a-kind-through` of this repository, whose nine groups each end with "Run the group's checks through sluice and commit".

## Context and problem statement

change@2's code stage starts a fresh agent for each task group of the place's change (record 0019). Nothing in its prompts asks a group to run the repository's checks. The construct agents that write the changes end every group with a task that does, copying the changes archived before them and the repositories' planning rules that every task names its verification. So a change of six groups runs the repository's whole checks six times through `sluice exec`, each run waiting for room under the host's bound, then once more at verify, which runs the checks the change names, and once more in the merge queue. On a host with twenty agents, each group's run waits longer than the group took to write, and the user named it wrong.

## Considered options

1. **change@3: one agent for the whole change, each task its own check, the whole checks once at verify.** The construct gives each task the narrowest check that shows it done and no task runs the whole checks or a group's. The code stage is change@1's, one agent working through every task. The verify stage runs the repository's whole checks once before it checks the work against the change.
2. change@2 kept, its construct told to give each group only its own tasks' checks. Fewer whole-suite runs, still a fresh agent per group, which the user did not want.
3. change@1 named in each repository's lock. One agent, but its construct still writes each group a check of the whole repository, and its verify still runs every check the change names.

## Decision outcome

Option 1.

- **flume's own change at version 3.** A construct stage that writes the place's change, each task a checkbox line naming the narrowest check that shows that task done: a test by name, one package's tests, a lint of the files it touched. No task runs the repository's whole checks, and no task checks a group. Task groups stay the change's headings, for reading.
- **One agent codes the whole change.** change@3's code stage does not repeat: it is change@1's, one agent working through every task in order, ticking each as it is done and running each task's own check as it goes.
- **The whole checks run once, at verify.** change@3's verify stage runs the repository's whole checks once, as the repository's instructions name them, then checks the work against the change as change@2's does, and stops short finding where either fails. The merge queue runs its gate as it always does.
- **The newest is the default.** change@3 is the newest version of flume's own change, so a launch naming no version runs it where the repository's lock records none (154, 155; record 0022). change@1 and change@2 stay flume's own side by side, and a place launched at either keeps it to its end (21).
- **The repeat field stays the format's.** `repeat = "per-task-group"` stays for a repository's own kinds (140 to 142); flume's own change@3 does not use it.

### Consequences

- Clauses 22, 154 and 157 and C.2's list of flume's own kinds and its kinds table are amended, and S119 added, each marked proposed, at agentplot/blueprints.
- Places already launched at change@2 keep their per-group agents and the checks their constructs wrote.
- A long change's one coding agent may fill its conversation and compact, which record 0019 was made to avoid. Changes are written small enough for one agent to code.
- The repositories' planning rules that each task names its verification say it is the narrowest check that shows the task done, never the repository's whole checks.
- No unit is queued from this record while it is proposed.
