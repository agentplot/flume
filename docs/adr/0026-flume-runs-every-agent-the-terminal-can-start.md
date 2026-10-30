---
number: 26
title: Flume runs every agent the terminal can start, each part of the agent seam built for its program or plainly degraded
status: proposed
date: 2026-10-30
decision-makers:
- Chuck Swanberg
---

# Flume runs every agent the terminal can start, each part of the agent seam built for its program or plainly degraded

Status: proposed to the user on 2026-10-30; accepted only on their yes.

Deciders: the user, 2026-10-30: "Are the mode values properly encapsulated along the agent seam so that it won't be awkward when we go to add support for other agents? In fact we should be supporting all agents out of the box since our multiplexer supports all the agents". What the agent seam requires of a program, what flume does where a program lacks a part, how a profile is declared, where the line is drawn, how a model is named, which program a stage with none named runs, and which profiles are built first are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at a11f6d5, clauses 3, 31, 32, 35, 38, 88, 89, 93, 105, 135, 144, 164, 170 to 174, B.4, C.4, C.5 and S73 to S75, as amended with this record; records 0003, 0016, 0017, 0018, 0020, 0021, 0024; rules core.1, core.2, core.9, core.10, core.29, core.30; `crates/flume-seams/src/agent.rs` (the `Agent` trait: `command`, `started`, `received`, `asking`, `own_work`, `ended`, `last_words`, `spent`, `started_without`, `stop_short`, `send_back`, `guard`, `on_quiet`), `crates/flume/src/profiles.rs` (`agent`, which binds Claude Code's profile and resolves any other program to nothing), `crates/flume/src/profiles/terminal/herdr/mod.rs` (`KINDS`, herdr 0.9.5's 24 agent kinds); herdr 0.9.5's `agent start --help` (`--kind`, "Success means the expected agent was detected in the same terminal and is ready for input"), `agent prompt --help` (`--wait`, `--until`, `agent_prompt_stalled`), `agent explain --help` and `integration status`; herdr's documentation at <https://herdr.dev/llms.txt>, its Agents page (which agents herdr reads by screen, which report their own state, "state requires the integration" for OMP and MastraCode, "state only, less tested" for Gemini CLI and Cline, idle with background tasks still running for Antigravity and Grok, and "Adding a completely new agent still requires a Herdr binary update"); the `--help` of Codex 0.165.0, Gemini CLI 0.66.0, GitHub Copilot CLI 1.0.104 and OpenCode 1.19.4, read on 2026-10-30.

## Context and problem statement

flume has one agent profile, Claude Code's (C.4), and `profiles.rs` resolves any other program to nothing, so a launch that names one is refused. Its terminal, herdr, starts and reads 24 kinds of agent program: pi, claude, codex, gemini, cursor, devin, agy, cline, omp, mastracode, opencode, copilot, kimi, kiro, droid, amp, grok, hermes, kilo, qodercli, qwen, letta, maki and muse. For each, `herdr agent start --kind <kind>` starts the program in a pane and answers once it is ready for input, `herdr agent prompt` submits a prompt and can wait for the turn it starts, and herdr reads the agent's state, working, idle, done or blocked, from its screen or from the reports of its own integration. A stream's pool may hold accounts of several programs (record 0020), and a mode is now flume's own word that each profile maps (record 0024). The user asked that every agent herdr runs be supported out of the box.

The seam asks more than herdr gives. Claude Code's profile answers every part from the agent's own record through flume's plugin: its word that it started under its start's identity and settings, that it received a prompt, that it asks a person, its background work, its usage limit, its last words, a guard on submitting, a send-back while its deliverable is missing, and a settle the moment it goes quiet. Most programs have hooks of some kind, but each in its own form and release, and some have none. Clause 93 says an answer the agent's own record does not give is unknown, never guessed from the terminal, and clause 31 that an agent whose own work flume cannot read is never quiet: as written, a program with no profile of its own could start but its stage could never end.

## Considered options

For which programs flume runs:

1. Only programs with a profile built for them, as today; each new program waits for its profile.
2. Every program herdr has a kind for: a profile built for a program where flume carries one, and one common profile for every other program, reading what every program gives; each part of the seam either built or degraded, as the profile declares.
3. Every program, the common profile reading the terminal for every part, and no profile of a program's own.

For how a program's entry is declared, its name, its terminal kind, its model flag and its map of modes:

1. A table built into flume, one entry per program, each checked against the release flume pins.
2. Data shipped with flume, a file per program that flume reads at its start.
3. Both, a host's data adding programs or overriding the built-in table.

For an agent's start where the profile reads no word of the program's own:

1. herdr's ready alone.
2. A launcher of flume's own, run in the pane under the start's identity and secret, which checks the start's record and then becomes the program, its word with herdr's ready the start.

For an agent's model:

1. The program's own name, passed as written, a layer giving one name or a table of programs as for a mode.
2. Flume's own names for classes of model, mapped by each profile.

For the program of a stage whose layers name none:

1. flume's default program, `claude`, always.
2. The program of the account chosen from its pool, among the accounts whose program can run the stage; flume's default only where there is no pool.

For the profiles built first:

1. Claude Code's alone, the common profile later.
2. The common profile, with entries for the programs the host can check today, then Codex's own profile.
3. Full profiles for Codex, Gemini CLI, Copilot CLI and OpenCode before the common profile.

## Decision outcome

Programs option 2, declaration option 1, start option 2, model option 1, program option 2, and first-built option 2.

**Every program herdr starts is a program flume runs** (170). Each is bound by an agent profile: its own, where flume carries one, and otherwise the common profile. Out of the box means that every one of herdr's 24 kinds is accepted as a program; that a stage whose settings give no model and no mode starts any of them with nothing more; and that each part of the agent seam is either built for the program or degraded as its profile declares, and status names each degraded part with the agent, so nobody reads a degraded part as a built one. A profile of a program's own replaces the common profile's readings part by part, so a program gains its parts one at a time, each checked against its pinned release.

**The common profile reads what every program gives** (171). Where the profile reads no word of the program's own for a part:

- **Start.** The program is the herdr kind's canonical executable, or a path the settings name (C.5's check of the pane's foreground holds for it as for Claude Code). The start runs flume's launcher in the pane, under the start's identity and secret in its environment: it compares the digest of each variable the start's record names with its own environment, keyed by the secret, and the program it is to run with the one the settings name (161), records the start's word with its own process id and whether anything differed, and then becomes the program in the same process. The agent is started once that word is recorded with nothing differing and the terminal reads that process, by its id, running the program and ready for input; a program in the pane under another process is not that start's agent, as a resumed or restarted one would be. So the start check's program and environment are generic, and the mode is compared only where a profile reads it back (record 0024).
- **Prompt delivery and ready for input.** herdr's `agent start` answers once the program is ready for input, and `agent prompt` submits the prompt, for every program; a program herdr reads blocked at its start is held at a question as C.5 says. Where the profile has no word of the program's own that it received a prompt, `agent prompt --wait --until working --until blocked` within the 60 seconds of 38 is read as received: herdr's word that the turn the submission began was seen; `agent_prompt_stalled` or a timeout is read as not received, and the prompt is sent once more as 38 says.
- **Working, quiet and waiting.** The terminal's state, as it is for every program today (96): working as working, idle and done as waiting. Blocked is asking where the profile leaves no mark of its own (90); Claude Code's profile keeps its mark.
- **Own work** (31, 91). Where the profile reads none, the agent's own work is read as none once the terminal has read it waiting at two settles a minute or more apart. Clause 31 would otherwise keep such an agent from ever being quiet and its stage from ever ending. Background work an agent started and that the terminal cannot see, as herdr says of Antigravity and Grok, can be cut short; status says own work is read from the terminal, so the kind's author and the host know.
- **Usage limits** (146, 151). Unknown, the seam's default today. An account whose limit the profile cannot read is never recorded spent; its agent goes quiet without its deliverable and its stage is stuck at its limit (44) with the screen as its last words, for its caller to retry on another account. Status says spent is not read.
- **Last words and ended** (43, 134). The terminal's screen, and its stopped.
- **Hooks.** Optional. With no hook flume can call, send-back (115), guard (116) and on-quiet (121) are not offered, as the seam already allows: the agent may go quiet with its deliverable missing and is read stuck at its limit; a submission that gets past no guard is read as any other of the place's branch (108), and status names the guard not offered; and herdr's watch settles the place at each change of the agent's state (122).
- **Stop-short.** `flume needs` for every program, run by the agent from a shell command with its start's identity and secret in its environment (41, 92). A program that keeps the environment from the commands it runs cannot stop short, and status says so.
- **Model.** The program's own name for a model, passed as written (172), by the flag its entry names; flume maps no model's name from one program to another. A layer may give a table of programs, `model = { claude = "opus", codex = "gpt-5.5" }`, as it may a mode. A program whose entry names no model flag refuses a model given, naming the program.

**A built-in table, one entry per program.** Each entry gives the program's name, its herdr kind, its model flag where it has one, its map of the four modes (record 0024), a flag that trusts the folder where it has one, and which parts a profile of its own builds. The table is code, checked as every profile is against the release flume pins, so an entry passes the conformance suite or is no entry (106; core.29). Every herdr kind has an entry from the first build, holding the program's name and kind; an entry with no model flag refuses a model, and one with no map of modes refuses a mode, so each is accepted and plainly degraded until its flags are checked. Part C writes each entry's flags when they are built.

**Where the line is drawn.**

- A program herdr has no kind for does not run under herdr (C.5); under the printed command, any program with an entry runs.
- A program with no model flag runs with the model its account's configuration chooses, and a model given refuses.
- A program whose mode map lacks a mode refuses that mode (169), never runs wider or narrower.
- A stage that delivers a plan runs only on a program whose profile offers the plan part (164), Claude Code's today.
- A program whose start asks a question flume does not answer is never begun with the screen (34, 35); the common profile answers the folder-trust question only where its entry names a flag or a question checked against the release.
- A program whose herdr state needs herdr's integration, OMP and MastraCode in herdr 0.9.5, refuses the launch while `herdr integration status` reports the integration not installed, naming the command that installs it.
- A program herdr reads by its screen alone, Gemini CLI and Cline among them, runs, and status names its state as read from the screen.

**The program a stage with none named runs** (173). Where no layer names a program and the agent has a pool, the account chosen names it: the pool's usable accounts for the stage are those whose program can run it, its mode mapped, its model given or not needed, and, for a plan stage, the plan part offered (164, 144). So a kind that names no program and gives flume's modes runs on whichever program the stream's pool offers, as record 0020 meant a pool of several programs to. Where there is no pool, flume's default program, `claude`, runs it, as today.

**Built first.** The common profile, with entries checked for the four programs other than Claude Code that the host holds today and so can pin and test, Codex, Gemini CLI, GitHub Copilot CLI and OpenCode: their model flags and maps of modes (record 0024's table), and entries holding name and kind for the other nineteen. Then Codex's own profile: after Claude Code it is the agent the host uses most, it maps all four modes, herdr's integration for it stands on the host, and its hooks are the likeliest to give the started, received, guard, send-back and on-quiet parts. The common profile first makes every herdr kind run at once, so the user's ask holds from the first build, and each profile after it adds parts to a program that already runs.

**Why not the others.** Programs option 1 keeps every program but Claude Code out until its whole profile is built, which is what the user asked to change. Option 3 would degrade Claude Code's profile to the terminal's readings and throw away the guard, the send-back, the usage limit and the received ack it already has. Declaration option 2 lets a file flume did not test map a mode: an entry that put `edits` on a program's full-access flag would run every agent of it wider than asked, and nothing would check it against the release; herdr itself adds an agent only with a binary update. Option 3 adds a second source for the same entry, and a host's override is that unchecked file again. Start option 1 would take herdr's ready, which the herdr profile already reads as a second signal and never the start (35, C.5), since herdr reads Claude Code's first-run screens as ready; with no word under the start's identity and secret, a pane brought back without its environment, or a program a person started by hand, would be taken as the stage's agent. Model option 2 would need flume to know every program's models and keep up with them; the program's own name, scoped by a table where a host mixes programs, is what a host writes today. Program option 1 would leave a pool's accounts of every program but Claude Code unusable for any stage whose layers name no program, which is every flume kind.

### Consequences

- Clauses 3, 31, 32, 35, 38, 88, 89, 93, 105, 135, 144, 164, C.4 and C.5 of flume's statement are amended, and 170 to 174 and S73 to S75 added, at blueprints a11f6d5. Rule core.1's details, core.10 and core.30 are amended and core.39 drawn from this record, each marked proposed.
- The agent seam gains a profile's declaration of which parts it builds, which status reads to name the degraded ones; the common profile and the launcher are flume's own, in `profiles/agent/`, and name no program outside its table (core.1).
- The herdr profile's `send` waits on `agent prompt` for the turn where the agent's profile reads no received ack, and its `open` accepts every kind in `KINDS`, as it already does.
- A host with a pool of several programs gains nothing until it writes accounts of them (144); a host that writes none sees no change.
- Public docs say "every agent program herdr starts", and name the programs as products.
