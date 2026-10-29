# flume runs live under a signed-in agent

## Why

The walkthroughs pass with the stand-in for Claude Code, and the herdr
tests pass with a wrapped `claude` in the settings, but no run has yet
carried a fix and a change through with the pinned Claude Code itself,
signed in, under herdr. That run needs a test account's token, which
only the operator holds, so it is the operator's to run by hand, outside
`sluice exec`, whose sandbox allows localhost only.

## What Changes

- **The live walkthroughs pass.** `flume:live` (`crates/flume/tests/profiles/live/`)
  runs the fix and change walkthroughs with the pinned Claude Code under
  herdr in place of the stand-in, and a stage whose pane goes quiet with
  no deliverable is not ended (39, S3), each agent in a scratch
  `CLAUDE_CONFIG_DIR` signed in by a test account's token.
- **A nix-wrapped `claude` starts as itself.** The Claude Code agent
  profile names the program a nix wrapper `bin/claude` execs, with the
  wrapper's variables, so herdr sees `claude` and flume's foreground
  check takes it, and `flume:live` names the pinned `claude` itself
  (record 0018; C.5).

- **No environment value is printed or kept by a start.** Status, a
  refusal's words, the record of a start and the printed command name
  each variable of an agent's environment and give no value, since a
  value may be a token; the printed command's environment is a file the
  host's user alone may read, and the launch's settings, which later
  starts read, are kept in held state the host's user alone may read
  (32, 56; record 0017).
- **The folder-trust question is answered from the screen.** The herdr
  profile reads Claude Code's trust dialog off the pane whatever herdr's
  `agent start` answered, moves the focus to "Yes, I trust this folder",
  confirms only there, and reads again until the dialog is gone, within
  30 seconds (34; record 0018).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `agent-profile`: a wrapped Claude Code is started as the program its
  wrapper runs.
- `the-gate`: the `check-live` layer's walkthroughs pass under the
  pinned Claude Code, run by the operator.
- `place-status`: an agent's environment is named with no value.
- `terminal-profiles`: herdr answers the folder-trust question from the
  screen; the printed command keeps its environment in a file.

## Impact

- `crates/flume`, `crates/flume-core`, `crates/flume-seams`: the
  failures the live run found, a token printed in status and a
  folder-trust question left unanswered, are fixed here; the folder-trust
  seam names the focus mark and the keys that move and confirm it.
- The operator: a test account, `claude setup-token`, and one run.
