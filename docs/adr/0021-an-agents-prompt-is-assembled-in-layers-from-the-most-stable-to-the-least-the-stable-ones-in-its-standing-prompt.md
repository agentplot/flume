---
number: 21
title: An agent's prompt is assembled in layers from the most stable to the least, the goals, the intent's brief and the stage's standing instructions in its standing prompt, so fresh agents read them from the cache
status: accepted
date: 2026-10-29
decision-makers:
- Chuck Swanberg
---

# An agent's prompt is assembled in layers from the most stable to the least, the goals, the intent's brief and the stage's standing instructions in its standing prompt, so fresh agents read them from the cache

Status: accepted by the user, 2026-10-29, in their design session, on how flume's fresh agents, one per stage and one per task group, stop paying again for text every one of them is sent.

Deciders: the user, who set the five layers and their order, the system prompt for the first three and the first message for the last two, the guarantee that the stable layers carry nothing that varies, cache hits in status, and that a stage's instructions are the kind's portable part while the goals and the briefs are the caller's; the stage's standing instructions as a field of the kind format, where the settings' instructions go, the intent's brief varying the second layer, where callers give each layer, and starting Claude Code with its own per-place sections moved out of the system prompt are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 6e5aae2, clauses 4, 7, 8, 23, 29, 56, 141, 148, 149, 152, C.2 and C.4, S57 and S58, and `design/weir/requirements.md` clause 53 and C.4, both as amended with this record; records 0017, 0019, 0020; rules core.4, core.31, core.33; `crates/flume-core/src/prompt.rs` (`render`), `crates/flume-seams/src/prompt.rs` (`PromptPart`), `crates/flume/src/profiles/agent/claude/mod.rs` (`command`, which passes the settings' instructions as `--append-system-prompt`), `crates/flume/src/profiles/kinds/weir/prompts/`; Claude Code 2.1.296's `--help` for `--append-system-prompt` and `--exclude-dynamic-system-prompt-sections`, and its transcript's `message.usage`.

## Context and problem statement

A stage's agent is started with its settings, whose instructions Claude Code takes as `--append-system-prompt`, and sent its stage's prompt as the first message. The prompt is the kind's template filled with the place's parts. flume's own templates open with the place's name, location and branch, `You are the code stage of the change {{name}}, working in the place {{place}}…`, and only then say what the stage is for. So the first bytes of every prompt differ from place to place, and everything after them, the instructions the stage gives every place alike, is read again in full by every fresh agent.

flume starts many fresh agents: one per stage (record 0003), one per task group (record 0019), and a fresh start whenever an account is spent (record 0020). The prompt cache reads a request's leading text cheaply when an earlier request on the same account began with the same bytes, within the cache's lifetime. What fresh agents share is wasted unless it comes first, is the same byte for byte, and they start on one account.

Claude Code puts sections of its own into its system prompt that differ from place to place: the working directory, the environment, its memory's paths and the repository's git status. Text appended after them differs in its prefix from one place to the next, however stable it is itself. The pinned 2.1.296 has `--exclude-dynamic-system-prompt-sections`, which moves those sections into the first message.

And kinds are to be shared from host to host. The stage's instructions are what a kind is; the project's goals and the briefs are the user's and the caller's, and a shared kind cannot carry them.

## Considered options

1. One prompt the kind writes freely, as today, with whatever parts wherever it likes.
2. Everything in the first message, the system prompt left to the agent program.
3. Fixed layers from the most stable to the least: the goals, the intent's brief and the stage's standing instructions in the system prompt, the same for every agent that shares them; the unit's brief and the run's details in the first message.

## Decision outcome

Option 3.

**The layers.** Every agent is started with its standing prompt (148), in this order:

1. the goals: what the repository's work is for;
2. the intent's brief: what the intent its place serves is for (record 0020);
3. the stage's standing instructions: what the stage is for, how it works and what it delivers, the same for every place;

and then its settings' instructions (32), which a host or a repository may add and a launch may replace whole. Its prompt, the first message, then holds:

4. the unit's brief and the place's facts: the launch's brief, the name, the place, the branch, the change;
5. this run's details: a repetition's group and the notes' path (record 0019), and a retry's, a failed submission's, a finding's or flume's own words (record 0020), last.

For Claude Code the standing prompt is `--append-system-prompt`, and flume adds `--exclude-dynamic-system-prompt-sections`, so the system prompt ahead of it is Claude Code's own and the same for every agent of one release (C.4). The settings' instructions go after layer 3 and not before it, since a launch may give its own: a place that does so loses the cache only for what follows the layers it shares.

**The standing instructions are the kind's.** A stage's declaration gains `standing`, a path beside the declaration as `prompt` is, whose text is the stage's standing instructions (23). It is a field of the kind format, so weir's statement names it (53, C.4) and weir-kinds reads it, and flume reads it only through weir-kinds (core.4). The record 0017 setting `instructions` is not used for this: a host that set `instructions` for a kind would replace the stage's instructions whole and break the stage. flume's own kinds move what each prompt tells every place alike into a standing file, and their templates keep the parts. A kind's version names its stages and deliverables, which do not change, so no version moves for this; a place keeps the declaration its launch read (21).

**Byte for byte.** Layers 1 to 3 are the same, byte for byte, for every agent of one repository, intent and stage of a kind, in whatever stream (148). flume fills no part in them: a stage whose standing instructions hold a `{{` part refuses the launch, naming the stage (4, C.2). It puts one fixed separator between layers and nothing for an empty layer. Nothing that varies between starts goes in them: no time, identity, secret, start number, place location or branch. Those stay in the environment (`FLUME_*`) and in the prompt. The same holds for every agent of a kind, repository and stream wherever the caller gives no intent, since layer 2 is then empty. Layer 2 varies by intent, not by stream, because the user's order puts the intent ahead of the stage: an intent's agents share layers 1 and 2 across all its stages.

**Where the caller gives each layer.** The goals are the launch's, else the text of the file the repository names in its `.config/flume.toml`, `[prompt] goals = "<path>"`, read at the base's revision with its kinds (149, C.2). The `[prompt]` table is flume's, beside `[profiles]`, and no part of any kind. The intent's brief is the launch's, given with the intent's name. The unit's brief is the launch's brief, as today. Each is given to the launch as text. The command line reads a file the caller names once, as `--brief-file` does today, and the launch keeps the text with the place, so every stage of the place sends the same bytes however the file changes later (8, 21). Callers pass prompt parts, not files flume reads again: crew and Flywheel give the goals and the intent's brief in the launch's JSON form.

**Seen in status.** The agent seam gains `usage` (152). For Claude Code that is the transcript's assistant records' `message.usage`: `cache_read_input_tokens`, `cache_creation_input_tokens` and `input_tokens`. Status gives, with the current agent, how much of its input was read from the cache on its first request, which shows whether the standing prompt was a hit, and over its session (56). `flume prompt` renders the standing prompt as well as the prompt (29).

**With record 0019.** A repeating stage's prompts already differ only in their last parts (141; rule core.31). Under this record the parts every repetition shares that do not depend on the place move into the standing instructions. The place's paths, then the group, the notes' path and the words, stay in the prompt in that order. What the repetitions share reliably is the system prompt. Claude Code's moved sections, the git status among them, now open the first message and differ from one repetition to the next once commits land, so the first message's shared prefix is short. That is the cost of putting the place's facts where they cannot spoil the stable layers.

**With the cache's lifetime and record 0020.** Claude Code's cache lives five minutes from its last read by default, and an hour where the account or the environment enables it (C.4). The standing prompt pays wherever agents that share it start on one account within that time of each other: a bolt's units starting together, a stage's repetitions one after another, the next stage of a place, the stage started afresh after a limit on the same account. Record 0020's affinity keeps an intent's agents on one account. Without it least-busy would spread them over the pool, and each account would hold its own cold copy.

**Kinds stay shareable.** Layer 3 is the shared, portable part. It travels with the kind wherever the kind is declared, and holds nothing of a host, a repository or a caller. Layers 1, 2 and 4 come from the user, the repository or the caller, never from a kind. A kind declared alike on many hosts gives every agent of a stage the same layer 3. It is read from the cache only by agents on one account, so a widely shared kind saves nothing across accounts: caching still rests on record 0020's affinity.

**Why not the others.** Option 1 is today's: a template that opens with the place's name has nothing in common with the next place's after its first line, and flume can guarantee nothing a kind writes freely. Option 2 puts the stable text in the first message, after whatever the agent program puts there that differs, the moved per-place sections or the git status, so it is never read from the cache. And it leaves nowhere for a kind's portable instructions that a caller's text cannot come before.

### Consequences

- Clauses 4, 7, 8, 23, 29, 56, C.2 and C.4 of flume's statement are amended and 148, 149, 152, S57 and S58 added, and weir's 53 and C.4 amended, at blueprints 6e5aae2. Rule core.33 is drawn from this record, and core.4's details name `standing` as weir's.
- weir-kinds reads `standing`, and flume's pin of it moves once it does; until then `_open.md` names the wait.
- flume's own kinds' prompts are split into standing files and templates, change@2's with record 0019's order kept.
- Nothing is built until the user approves a plan; `_open.md` names the wait.
