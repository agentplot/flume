---
number: 38
title: Claude Code's standing prompt goes by a file of the start's own, since herdr types no argument holding a newline
status: proposed
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# Claude Code's standing prompt goes by a file of the start's own, since herdr types no argument holding a newline

Status: proposed, the fixing agent's own choice, which the written rules leave to it and which is cheap to change: how the agent profile hands Claude Code a text it already hands it. It waits on the user's yes.

Deciders: the fixing agent, on the fix `every-stages-agent-starts-whatever-its-arguments-hold` of 2026-11-05, whose brief asked that every stage's agent start, a value herdr cannot carry carried another way where flume can.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 4ec26f7, clauses 32, 35, 88, 148, C.4 (the command row) and C.5 (the open row and what herdr 0.9.5 was seen to do), as amended with this record at c8cab06; herdr 0.9.5's source, `src/app/agents.rs` (`start_agent`, which refuses any argument holding a control character as `invalid_agent_argument`, "agent arguments cannot be encoded safely for the target shell", before it quotes the start for the pane's shell and types it) and `src/agent_resume.rs` (`validate_resume_argv`, the same rule for a resumed agent); Claude Code 2.1.296's `--append-system-prompt-file`, absent from its help and read in an interactive session as `--append-system-prompt`'s text, as that release passes it to sessions it starts itself; records 0017, 0018 and 0021; rules core.29, core.33, core.39; `crates/flume/src/profiles/agent/claude/mod.rs` (the command), `crates/flume/src/profiles/terminal/herdr/mod.rs` (the open).

## Context and problem statement

On deneb on 2026-11-05, under flume 0.6.4 and herdr 0.9.5, the code stage of `fc5cc3283a9c/change--the-fleet-installs-flume-pinned-as-sluice-is` could not start its second group at code:2 and code:3: `herdr refused (invalid_agent_argument): agent arguments cannot be encoded safely for the target shell`. So did `245ca3631bdd/research--sluice-signs-its-landings` at talk:2, `4d33e0a8cac8/change--a-view-writes-its-vs-code-workspace` at verify:1 and verify:2, and `fc5cc3283a9c/change--the-runner-runs-on-its-own-on-the-hosts-that-carry-it` at code:4. Each refused start's settings held instructions from the host's table of a whole Markdown document, many lines long. The starts that came up, code:4 of the first place among them, held the host's one-line instructions, which the host's configuration gave by then (32). The prompt was never the cause: herdr's agent start does not carry it, and flume sends it once the agent waits (C.5).

Claude Code's profile passed the instructions as `--append-system-prompt <text>`. herdr's agent start quotes each argument for the pane's shell and types the start there, and it refuses any argument holding a control character, a newline or a tab among them, since a newline typed into an interactive shell ends the line. C.5 recorded what herdr was seen to carry, 20,000 characters with quotes, `$`, backticks and non-ASCII, and no newline among them. Once the standing prompt of 148 holds the goals and the briefs, nearly every start's would hold one.

## Considered options

1. The standing prompt in a file of the start's own, named by `--append-system-prompt-file`.
2. The newlines replaced by spaces, or the text escaped, before it is passed.
3. The text in a variable set on the pane, and the start passing `"$VAR"` for the shell to expand.
4. A script in the start's own folder, linked as the kind's name, that runs Claude Code with the text.
5. Nothing of flume's changed, a report sent to herdr asking it to type such arguments.

## Decision outcome

Option 1.

1. **The file.** The agent seam's command is given the place's own folder (88), as every reading of the seam is. Claude Code's profile writes the instructions as given to `standing-prompt.md` beside the start's record, `<folder>/claude/<identity>/`, and names it by `--append-system-prompt-file`, which 2.1.296 reads as it reads `--append-system-prompt`'s text. So no argument of the command holds a newline, under herdr or the printed command. The file goes with the place's folder at its tidy (62), and a start's file is never another start's.
2. **Nothing else changes.** The text reaches Claude Code's system prompt byte for byte as it did, so 148's layers and the prompt cache are as they were. Under the printed command the line a person runs names the file in place of the text.
3. **herdr's refusal stands.** herdr refuses by design what it cannot type into every shell it supports, and says so before it types anything. The open reads `invalid_agent_argument` as refused in herdr's words, as it reads `invalid_agent_timeout` (C.5), and nothing of flume's checks herdr's rule ahead of it.

Option 2 changes the text the agent reads, and a document's lists and code blocks with it. Option 3 does not work: herdr quotes each argument in single quotes, so the shell expands no variable in it. Option 4 runs a program of flume's in place of the one the settings name, which the foreground check reads as another program unless it execs, and adds a script where the program already reads a file. Option 5 leaves every start that holds a standing prompt of several lines refused until herdr changes, and herdr's rule is a sound one for a tool that types into a shell.

### Consequences

- `Agent::command` takes the place's own folder, and every profile, fake and stand-in takes it with it.
- Claude Code's command names `--append-system-prompt-file <folder>/claude/<identity>/standing-prompt.md` wherever its settings give instructions, and the file holds them as given. A file that cannot be written is still named: Claude Code's words that it is missing are the screen of a start that never came up (35), as for the plugin.
- `--append-system-prompt-file` is not in 2.1.296's help, so a later release that drops it is found by the walkthroughs' stand-in, which refuses a missing file as that release does, and by the live walkthrough, before a pin moves (core.22).
- The tools layer reproduces herdr's refusal against the pinned herdr, and starts an agent from Claude Code's profile with instructions of many lines.
- C.4's command row and C.5's open row and what herdr 0.9.5 was seen to do say so (blueprints c8cab06; core.19). Record 0021's sentence naming `--append-system-prompt` is amended with this record. Rule core.33's details name it.
- Neither of the family's other tools meets this refusal. sluice starts no agent in herdr: its herdr plugin opens its pane by `plugin pane open`, and its worktrunk profile only prompts an agent by `agent prompt`, which carries a newline, as flume's own prompts do. crew types a one-line command into a pane by `pane run`, and its role's brief, many lines long, reaches Claude Code as `--append-system-prompt` from its own launcher script's argument list, which herdr never types.
