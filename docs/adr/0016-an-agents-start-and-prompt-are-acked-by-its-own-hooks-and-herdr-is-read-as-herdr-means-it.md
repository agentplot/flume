---
number: 16
title: An agent's start and its prompt are acked by its own hooks, and herdr is read as herdr means it
status: accepted
date: 2026-10-28
decision-makers:
- Chuck Swanberg
---

# An agent's start and its prompt are acked by its own hooks, and herdr is read as herdr means it

Status: accepted by the user on 2026-10-28, on their Pending You card ("Approved"), on the review of crew's lessons driving Claude Code agents through herdr against flume's design.

Deciders: the user; the recommendations are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 4fe1522, clauses 35, 37, 38, 43, 77, 89, 90, 95, 96, 98, 135, 136, C.4, C.5, S39 to S43; crew's rules teams.2, teams.6, teams.7, teams.8, teams.11, teams.15, teams.19, read.1, read.2, its records 0007 and 0046, and `plugin/lib/crew.py`, `plugin/lib/transcript.py`, `plugin/hooks/`; sluice's `profiles/worktrunk/herdr.rs` and `plugins/herdr`; herdr 0.9.5's own skill text (`herdr --skill`); rules core.9 to core.12.

## Context and problem statement

crew drove Claude Code agents through herdr for a month and paid for each corner it met: prompts typed and never submitted (crw-2, 2026-10-10; the reviewer's batches 17 and 19, 2026-10-26), a start taken as never begun while the agent worked behind SessionStart hooks running for minutes under load (crw-1, 2026-10-11), a tell typed into a question herdr did not see (2026-10-07), panes herdr listed with no terminal behind them after a restart (2026-10-18 and 19), herdr's bare resume of every agent losing its environment and arguments (2026-10-18 and 19), and a herdr call that never returned and held a team's lock for two hours (deneb, 2026-10-27). Flume's first build was about to meet each again, and its design had three faults of its own: it read herdr's `done`, a finished turn nobody has looked at, as the agent stopped, so an unwatched stage would read lost; it named herdr agents by an identity herdr's name rule (`[a-z][a-z0-9_-]{0,31}`) refuses, and herdr drops names whenever a pane's session changes; and it typed a command carrying the agent's instructions into a shell whose line limit herdr enforces.

The user also asked whether flume's own plugin could say an agent started and received its prompt, in place of time rules and of reading the transcript.

## Considered options

1. Keep the readings as designed: the start judged from the terminal, received from the transcript's user turn, herdr's states mapped as written.
2. The agent's own acks: SessionStart's `flume ack started`, carrying the start's identity, secret and the plugin's version, and UserPromptSubmit's `flume ack received`, carrying the submitted prompt's digest; the transcript kept as the fallback, and herdr's states read as herdr documents them.

## Decision outcome

Option 2.

- **A start is starting until its agent says it started** (35, 135): the started ack proves the program runs with flume's plugin, so the guard and the stop check are in force and the identity reached it. No prompt is sent before it (37). A program that ends with no ack is never begun at once; one held at a dialog no hook precedes, the bypass warning, a sign-in, a first run's setup or a repository's MCP servers' approval, stays starting with the terminal's screen in status (136), and is reported never begun at the stage's limit (44) or on its caller's stop or retry. The folder-trust question alone is answered (34).
- **Received is the agent's own word** (38, 89): UserPromptSubmit fires only when Claude Code submits a prompt, queued ones among them, so text left typed acks nothing; the transcript's user turn or queued record answers only where no ack can come.
- **A bare resume is lost** (77): herdr's resume loads no `--plugin-dir` and carries no `FLUME_*`, so it acks nothing; the stage is lost and the agent ended once quiet.
- **Send clears held input first and leaves nothing typed** (95); **state reads stopped only when the program no longer runs** (96): under herdr, `idle` and `done` are waiting, `unknown` is unknown, `blocked` is asking only with the plugin's mark and unknown without it, and stopped is the pane gone, or no agent listed with the shell in its foreground, or the pane unrestored after herdr's restart. The asking mark covers permission prompts too (90) and clears at UserPromptSubmit and SessionStart as well as when the transcript holds the answer.
- **Every terminal call is bounded** (98): under herdr, ten seconds where the call states none, with the caller's `HERDR_*` stripped and the recorded server's socket passed. Panes are held by id and socket, a session named `herdr:<pane>@<socket>`, and a herdr agent name of at most 32 legal characters is for display only. A launcher file in the place's state folder is what is typed into the shell.
- **Stuck, lost and never-begun ends carry what was read** (43): the agent's last words, StopFailure's reason among them where the pinned Claude Code has that hook, the commits since the start, and for never begun the screen.

### Consequences

- Rule core.29 is new: no herdr call without a bound, every herdr fact checked against the pinned herdr. Rules core.9, core.10, core.11 and core.12 gain the cases above in their details, and core.10 and core.11 their words for the started ack and the cleared input.
- The OpenSpec change `flume-runs-a-kind-through` carries it in groups 7 and 9 (the herdr profile; the agent profile, its acks and the seam's `started`), and `flume-runs-unattended-and-wakes-roles` in groups 1 and 6 (recipients by pane and socket; one listing per tick and a lease keyed on its process's start time).
- Crew's lessons that a fresh pane per start makes moot stay moot: escape codes left by a bad exit, the exit's question about background work, a pane reused across stages.
