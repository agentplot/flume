---
number: 24
title: An agent's mode is a setting layered as its model is, and a plan stage delivers a plan its caller accepts, revised in place by a tell
status: proposed
date: 2026-10-30
decision-makers:
- Chuck Swanberg
---

# An agent's mode is a setting layered as its model is, and a plan stage delivers a plan its caller accepts, revised in place by a tell

Status: proposed to the user on 2026-10-30, on flume-sluice-build's card req_8f361bf504424a5b975b; accepted only on their yes.

Deciders: the user, on flume-sluice-build's card req_8f361bf504424a5b975b, 2026-10-30, who set that an agent's permission mode is a setting in the same layers as its model and instructions, the kind's, the repository's, the host's and the launch's, its values the agent profile's, passed to Claude Code as `--permission-mode`, nothing passed where no layer sets it so the account's own settings decide; that the start check records the mode, so a session started in another mode is unsettled; that a plan kind's stage starts in plan mode and delivers the plan file the agent writes when it leaves plan mode, which the caller shows to the user, the user's notes returning by `flume tell` to the live session or by `flume retry` carrying them, the stage ending when the caller accepts; and that mode is built first, before the crew refactor. How acceptance is said, what flume does with the agent's ask to leave plan mode, where the accepted plan is kept for the next stage, the mode a plan stage starts in whatever the layers say, and flume's own plan@1 are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 9c7490c, clauses 1, 4, 22, 24, 32, 34, 40, 44, 109, 130, 135, 161 to 164, C.2, C.4 and S66 to S68, and section 3's settings, mode, plan, finish and for review, and `design/weir/requirements.md` clauses 54, 57 and C.4, as amended with this record; records 0003, 0016, 0017, 0020, 0021; rules core.9, core.10, core.12, core.27, core.30; `crates/flume/src/profiles/agent/claude/mod.rs` (`asking`, "The mark the plugin leaves on a question, a plan or a permission prompt"), `crates/flume-core/src/acts.rs` (a finish refused for a place that runs no conversation), `plugins/claude/hooks/hooks.json` (PreToolUse on `ExitPlanMode`); the pinned Claude Code 2.1.296's `--permission-mode` choices, `acceptEdits`, `auto`, `bypassPermissions`, `manual`, `dontAsk` and `plan`, read from its `--help`, and the `permission_mode` every hook's input carries in that release; swancloud record 0042, which writes `bypassPermissions` into each account's `settings.json`.

## Context and problem statement

An agent's reach in its place is decided today by nothing flume says. Every Claude Code agent flume starts runs in whatever mode its account's `settings.json` sets, and swancloud writes `bypassPermissions` there for every account (swancloud record 0042). A kind cannot ask for less, a host cannot say one kind runs with more care than another, and a start that came up in a mode nobody meant is taken as the stage's agent.

The user also wants work whose first step is a plan they read before any code is written. Claude Code has the mode for it: in plan mode an agent reads and plans, and leaves the mode by putting its plan for approval, `ExitPlanMode`, its plan in a file of the account's plans directory. flume's plugin already sees that ask and marks the agent asking (clause 90). What it lacks is a stage built on it: none of flume's six deliverables is a plan, a stage that delivered would end at its agent's next quiet (40) and lose the session the plan was written in, and the user's notes have no way back to that session while it holds a question open, since a tell to an asking agent is refused (53).

## Considered options

For the mode:

1. Leave the mode to each account's configuration, as today.
2. `mode` a setting in clause 32's layers, the launch's, the host's two tables, the repository's and the kind's, for every stage or one; none passed where no layer gives one; the start records it.
3. As 2, but only the host's tables may give it, since it widens what an agent may do on the host.

For the plan stage's end:

1. The plan a report: the stage delivers once the plan exists and ends at the agent's next quiet, as any other; the caller's notes go to a retry.
2. The plan a seventh deliverable, and the stage for review once it exists and its agent is quiet: the agent kept live, the notes by a tell into the same session or by a retry carrying them, the plan accepted by the caller's **finish**.
3. As 2, accepted by the caller's **done**.
4. As 2, accepted by a **retry** naming the stage after the plan stage.
5. As 2, with flume leaving Claude Code's own approval dialog open and answering it with the notes or the acceptance.

## Decision outcome

Mode option 2, and plan option 2.

**The mode is a setting like the model.** Clause 32 resolves an agent's program, model, mode and instructions from the six layers, each taken whole from the first that gives it, a stage's own before one given for every stage. The host's two tables, `[kinds.<kind>.agents]` and `[repos."<repository>".kinds.<kind>.agents]`, give `mode` beside `model` and `instructions`; the kind's and the repository's agents tables give it, as does a launch, `flume launch --agent [<stage>.]mode=<mode>`. Its values are the agent profile's: for Claude Code the pinned 2.1.296's `--permission-mode` choices, `acceptEdits`, `auto`, `bypassPermissions`, `manual`, `dontAsk` and `plan`. A value the profile does not name refuses the launch naming the mode, the stage and its layer (4), or, where a host's table changed after the launch, leaves the stage refused at its start (32), as a program no profile binds does. The Claude Code profile passes it as `--permission-mode`. Status gives the mode with its layer, as every setting.

**None where no layer gives one.** flume then passes no `--permission-mode`, and the account's own configuration decides, as today, where swancloud's accounts run in `bypassPermissions`. This is the one setting the agent program's own configuration may choose (32), and only where every layer is silent; the user's word was that nothing changes for a host that says nothing.

**Who may give it.** Option 3 would keep a repository or a kind from widening its agents' reach on a host. But a mode names nothing a host owns (158): it is the work's own word on how carefully it is done, which a kind's author knows best, and a kind from outside runs only as a lock records it, which is the act of trust (155). A host that wants the last word states the mode in its own tables, which come before the repository's and the kind's. So every layer may give it.

**The start check.** A start records the mode its settings give (161), and the SessionStart hook compares it with the `permission_mode` its input carries. An agent that began in another mode, as Claude Code does when the account's configuration disables the one asked, is not the stage's agent: flume ends it once quiet and starts the stage afresh, and a fresh start that begins in the other mode again is never begun, naming the mode asked and the mode it began in. A start whose settings give no mode records none and compares none. What a mode lets an agent do never widens what flume answers for it: the warning Claude Code shows before a session in `bypassPermissions` where the account has not accepted it stays one flume never answers (34; core.12).

**A plan is a seventh deliverable.** A stage that delivers `plan` starts in the profile's planning mode, Claude Code's `plan`, whatever the layers give, and status names the deliverable as where its mode came from: a plan stage in any other mode could write code before its plan was read, and a layer's mode given for every stage of a kind must not undo the one stage that plans. The plan exists once the agent's own record says it put one for review since the stage started (24). In Claude Code that is `ExitPlanMode`: in a plan stage the plugin's PreToolUse hook records the plan the tool names, its `planFilePath`, or its `plan` text where it names no file, and denies the tool with flume's words, that the plan is put for review and that notes or an acceptance will come as its next message. The agent stays in plan mode and ends its turn, waiting rather than asking, and no asking mark is left. The deny is the one answer flume gives for an agent beyond the folder-trust question, and it narrows: it keeps the agent planning, and flume never accepts a plan (34; core.12 as amended). The agent seam gains **plan** (164) for it.

**For review.** Once the plan exists and the agent is quiet, the stage is for review. It does not end, and its agent is not ended (40): the plan, with where it lives, is delivered to each recipient of the place as an end is (43), once for each plan put, and the place waits on its caller (130), stalled after five minutes under no hold's words as any place is. The stop check sends back an agent that would go quiet with no plan put; a stage for review is never stuck (44). The caller, crew today, shows the plan to the user however it shows things, and answers with a command it already has:

- **`flume tell`** with the user's notes: the agent is waiting, so the tell reaches it (53), still in plan mode, and it revises the plan in its own session, its cache warm where the review was quick, and puts it again, which is delivered again;
- **`flume retry`** naming the stage with the notes as its words: a fresh agent in plan mode, whose prompt carries the path of the plan last put for review before the notes, as a stop-short's question stands before its answer (49);
- **`flume finish`**: the plan is accepted.

**Acceptance is the finish.** A finish already means "this stage is done on my word" for a conversation (109), the one other stage that ends on its caller rather than on a deliverable read, and it already ends the agent once quiet, forced when not, and counts the stage delivered so the next starts as 40 and 47 allow. A plan stage is the same shape. A done (option 3) is the place's word, that a kind that does not land is complete (114); used for a stage it would either end the whole place or change what done means. A retry naming the next stage (option 4) starts it, but leaves the plan stage neither delivered nor ended in status, and a retry already means "again, with these words". So the finish accepts the newest plan put for review; one before any plan was put is refused naming the place, and a finish of a stage that is neither a conversation nor a plan stays refused as today. A kind whose plan stage is its last and that does not land is ended by its caller's done as any such kind is (114), the plan where status said it lives.

**The plan accepted is kept for the next stage.** At the finish flume copies the plan as it stands into the place's own folder, `plan.md` beside the held state (C.7), and the prompt part `{{plan}}` names it. The copy fixes what was accepted, whatever the account's plans directory holds later, and the next stage's agent may run under another account whose configuration home is another directory. It goes when the place is tidied (123); a caller that keeps the plan after keeps it from what it was delivered.

**flume's own plan@1.** A plan stage, then fix@1's code stage with `{{plan}}` in its prompt, delivering commits, landing, a failed submission sending it back to code; branches `plan/<name>`. It is the kind the user asked for, and the smallest one that shows the stage end to end; a repository or a package declares any other kind with a plan stage in it, since `plan` is a deliverable of the kind format (weir 54).

**Why not the others.** Mode option 1 is today's: nothing a kind or a host can say, and a start in the wrong mode taken. Plan option 1 ends the agent at its first quiet, so every round of notes costs a fresh agent and a plan written again from the start; the user asked that a tell reach the live session. Option 5 needs flume to type into Claude Code's own approval dialog, a screen read and keys sent, where a hook's deny is one line the pinned release defines; and it holds the agent asking, so a tell is refused and the dialog's own "yes" would leave plan mode in the same session, letting the planner write the code.

### Consequences

- Clauses 1, 4, 22, 24, 32, 34, 40, 44, 109, 130, 135, 161, C.2 and C.4 of flume's statement are amended, and 162 to 164 and S66 to S68 added, with section 3's settings, finish and stage states and its new mode, plan and for review; weir's 54, 57 and C.4 are amended, at blueprints 9c7490c. Rules core.10, core.12 and core.30 are amended and core.37 drawn from this record, each marked proposed.
- A seventh deliverable is a field value of the kind format, so weir's crate reads `plan` and flume's pin of it moves before flume reads it (core.4); `mode` is a field of an agents table, which weir returns as read.
- The user's order: the mode is built first, before the crew refactor: the setting in every layer, `--permission-mode`, the start's record and the hook's comparison, and status. The plan stage, plan@1 and `{{plan}}` are built after, with the rest of flume's first build.
- An agent for review is live, so it holds a place in its host's and its stream's room and an account's seat while the user reads (70). A caller that would free the room retries with the notes rather than telling them. Points (record 0025) are in flight meanwhile, since the work is not done.
- Public docs say "a plan stage" and "the caller shows the plan"; they name no tool that shows it.
