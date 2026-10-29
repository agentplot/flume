---
number: 18
title: herdr starts an agent by agent start in a pane made with its environment, and finds the pane again by the agent's name
status: accepted
date: 2026-10-28
decision-makers:
- Chuck Swanberg
---

# herdr starts an agent by agent start in a pane made with its environment, and finds the pane again by the agent's name

Status: accepted by the user, 2026-10-28, in their design session ("write 18"), on a trial of herdr 0.9.5's `agent start` against flume's herdr profile.

Deciders: the user, who chose `agent start` and agreed the host's `claude` wrapper keeps a `CLAUDE_CONFIG_DIR` already set; the readings of herdr's answers, and the bounds, are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 7e324d4, clauses 32, 34, 35, 38, 43, 77, 94 to 98, 132, 135, 136, C.4, C.5, S39 to S41, S45 and S46, as amended with this record; records 0016 and 0017; rules core.10, core.12, core.29; herdr 0.9.5's own skill text (`herdr --skill`) and `herdr agent start --help`; a trial of herdr 0.9.5 on a server of its own on a scratch socket, 2026-10-28.

## Context and problem statement

Record 0016 had the herdr profile open a pane, type one line into its shell running a launcher file in the place's state folder, read the pane every 200 ms until the agent waited, look for the folder-trust question in the pane's visible text, and give the agent a herdr name for display only. Each of those is the profile guessing at something herdr 0.9.5 can now tell it.

A trial of herdr 0.9.5 on its own server found:

- `herdr agent start <name> --kind claude --pane <id> --timeout <ms> -- <arguments>` types the program's name into the pane's shell, so the shell's `PATH` and startup files apply, and hands it the arguments after `--` byte for byte: instructions of 6,000 and 20,000 characters with quotes, `$`, backticks and non-ASCII arrived whole. It answers `agent_started` with `interactive_ready: true` only once the program is ready; in a trusted folder Claude Code's SessionStart hook ran at 0.43 s and the start answered at 3.9 s.
- At the folder-trust question it answers at once, exit 1, `agent_not_ready` ("agent … is blocked during startup and is not ready for prompts"), and the agent stays `blocked` under its name. `agent send-keys <agent> down enter` and then `agent wait <agent> --until idle` came back idle in 0.3 s.
- It reads Claude Code's first run's theme picker and its sign-in choice as ready. A program that is missing, or ends while it starts, shows only as `timeout` ("timed out waiting for agent startup") once the whole bound has passed. Its other refusals are `agent_pane_busy` (the pane is not at a shell), `agent_name_taken`, `agent_pane_not_found` and `invalid_agent_timeout`; a kind it does not know exits 2 in plain text.
- `agent start` takes no environment and no directory. `pane split --cwd <dir> --env K=V`, and `tab create` and `workspace create` likewise, set what the program started in the pane sees: a `CLAUDE_CONFIG_DIR` so given reached Claude Code.
- The agent commands take the agent's name or its pane's id (`w1:p4`), never `<pane>@<socket>` or a terminal's id. A pane moved to another tab kept its id and its agent's name; moved to another workspace its id changed (`w1:p6` to `w2:p1`) and its agent's name went with it. A name leaves `agent list` when its program exits.
- `agent wait --until <state> --timeout <ms>` answers `timeout` past its bound; with no `--timeout` it waits for ever.
- A 70,000-character line through `pane run` ran, so the terminal's line limit record 0016 gave as the launcher file's reason was not met in the trial; herdr's skill text still says `pane run` and `agent start` refuse a line past the terminal's canonical input limit on macOS.

Record 0016 also said a pane is held by id and socket and a herdr name is for display only, since herdr drops a name whenever the pane's session changes. The trial shows the other half: a name follows its agent where the pane's id does not.

## Considered options

1. Keep record 0016's open: a launcher file typed into the shell, the pane polled until the agent waits, the folder-trust question found in the screen.
2. herdr's agent start, in a pane made with the agent's environment, herdr's answers read as this record says, the pane found again by the agent's name.

## Decision outcome

Option 2.

1. **The open.** The profile makes the pane in the place, `--cwd <place>`, or splits beside a named pane, with an `--env` for each variable of the agent's environment: the environment resolved for it (record 0017) and flume's own `FLUME_*`. It then runs `herdr agent start <name> --kind <kind> --pane <id> --timeout 120000 -- <arguments>`, the call bounded at its timeout and ten seconds more (core.29). Nothing is typed into the pane's shell and no launcher file is written.
2. **A start held at a question.** `agent_not_ready` is the program's question before it starts. The profile reads the pane's visible lines: the folder-trust question alone is answered (34; record 0016), by `agent send-keys` with the keys the agent profile names, then `agent wait --until idle` under a bound of 30 seconds. Any other question, the warning about bypassing permissions or a sign-in among them, is never answered, and the start is never begun at once, delivered with the screen (35, 38, 43). This replaces record 0016's start that stays starting at such a dialog until the stage's limit, wherever the terminal can say the start is held; under a terminal that cannot, the printed command, that reading stands.
3. **herdr's ready is a second word.** The started ack from the agent's own SessionStart hook stays the start (35, 135; record 0016), since herdr reads a first run's setup screens as ready. herdr's `agent_started` sends no prompt by itself; flume still reads the screen. A `timeout` from `agent start` with no started ack is a start never begun, delivered with the pane's last lines.
4. **A pane is held by its id and socket, and found again by the agent's name.** The agent's herdr name is made from the identity, at most 32 of herdr's legal characters ending in a digest of the whole identity, so it is unique on the server; `agent_name_taken` refuses the open, as an agent already live under the identity (98). Where the pane's id no longer answers, as after a move to another workspace, `agent get <name>` finds the pane; it is taken as the agent's only once herdr lists it under that name, and its id is recorded before anything is sent to it. A role run's pane is its live start's and is found again the same way (132). A pane found by name and not checked is never sent to.
5. **A herdr kind per agent program.** `agent start` needs a kind herdr knows. The agent profile names the program's kind with its command (C.4), `claude` for Claude Code, and the herdr profile takes only a kind the pinned herdr lists. A program herdr has no kind for cannot run under herdr, which refuses the launch naming it, as record 0017's refusal of a program the host's terminal cannot run says (32).
6. **Bounds.** Every `agent start` and `agent wait` call passes `--timeout`, since `agent wait` with none waits for ever (core.29). The start's 120 seconds leaves room for SessionStart hooks slow under load, which crew once read as a start never begun (record 0016); the trust answer's wait is 30 seconds, herdr's own default for a start.

### Consequences

- A pane's environment is not kept across a restart of herdr's server, and that is accepted: an agent herdr restores, or resumes bare, has no started ack for its start, so record 0016 already reads its stage lost and ends it once quiet (77); a retry opens a fresh pane with the environment set again. Before restarting herdr's server on purpose, the operator drains flume's places first.
- `agent start --kind claude` runs whatever `claude` the pane's shell finds. On a host where a wrapper sets `CLAUDE_CONFIG_DIR` itself, the wrapper must keep a value already set, or record 0017's environment for the kind is overridden and the agent runs under the wrapper's account. The statement's C.4 states it as the host's requirement; it is no code of flume's.
- herdr's skill text says `agent start` refuses a line past the terminal's canonical input limit on macOS; the trial's 20,000 characters of instructions passed. A start so refused is a refusal in herdr's words, and an instructions file read by the program is the answer should one be met.
- The statement's clauses 35 and 94, C.4, C.5 and scenarios S39 to S41 are amended, and S45 and S46 added, at blueprints 7e324d4. Rule core.29's details gain `agent start` and its bounds, core.10's the pane found again by name, core.12's the start never begun at a question it does not answer. Record 0016 is amended: its launcher file and its herdr name for display only are this record's open and name.
- The OpenSpec change `flume-runs-a-kind-through` carries it in group 9, reworking the open its group 7 built; `flume-runs-unattended-and-wakes-roles` carries the role run's pane found again by name in group 1.
