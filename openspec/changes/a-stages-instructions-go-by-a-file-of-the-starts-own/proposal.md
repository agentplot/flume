# A stage's instructions go by a file of the start's own

## Why

A stage whose settings give instructions of several lines cannot start
under herdr. deneb showed it on 2026-11-05, under flume 0.6.4 and herdr
0.9.5, at code:2 and code:3 of a place whose host table gave a whole
Markdown document as its instructions: `herdr refused
(invalid_agent_argument): agent arguments cannot be encoded safely for
the target shell`. herdr's agent start quotes each argument for the
pane's shell and types the start there, and refuses any argument
holding a control character, a newline among them. Claude Code's profile
passed the instructions as `--append-system-prompt <text>`, so every
such start was refused. The prompt was never the cause: herdr's agent
start does not carry it, and flume sends it once the agent waits (C.5).

Once the standing prompt of 148 holds the goals and the briefs, nearly
every start's would hold a newline. Record 0038 decides how the profile
hands Claude Code a text it already hands it (32, 88, 148, C.4, C.5).

## What Changes

- **The instructions in a file (148, C.4).** The agent seam's command is
  given the place's own folder. Claude Code's profile writes the
  settings' instructions as given, newlines and all, to a file of the
  start's own beside its record, and names it by
  `--append-system-prompt-file`, which the pinned 2.1.296 reads as it
  reads `--append-system-prompt`'s text. No argument the profile gives
  holds a newline.
- **herdr's refusal held (C.5).** The tools layer reproduces herdr's
  refusal of an argument holding a newline against the pinned herdr, and
  starts an agent from the profile's command with instructions of many
  lines.
- **The stand-in** for Claude Code refuses a file that is not there, as
  2.1.296 does, so the walkthroughs prove the profile writes it before
  the start.
- **The site** names `--append-system-prompt-file` in the command and in
  the guide to an agent's settings.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `agent-profile`: the command for a code stage names the instructions'
  file by `--append-system-prompt-file`, and no argument holds a
  newline.

## Impact

- `crates/flume-seams`: the agent seam's `command` takes the place's
  folder, and its round trip.
- `crates/flume-core`: the start passes the folder to the command.
- `crates/flume-fakes`: the fake agent's command, and its tests.
- `crates/flume-conformance`: the agent seam's scenarios pass the folder.
- `crates/flume`: Claude Code's profile, `STANDING_PROMPT_FLAG` and
  `Record::standing_prompt`; its tests, the stand-in, and the tools
  layer's herdr test.
- `site/`: the guide to Claude Code and the guide to agent settings.
- `docs/`: record 0038, proposed, which amends record 0021; rule
  core.33's details name it.
- agentplot/blueprints: C.4's command row and C.5's open row are
  amended at c8cab06, on the branch
  `design/flume-a-standing-prompt-goes-by-file` (core.19).
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool.
