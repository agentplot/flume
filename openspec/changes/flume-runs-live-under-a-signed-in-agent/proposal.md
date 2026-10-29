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

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `agent-profile`: a wrapped Claude Code is started as the program its
  wrapper runs.
- `the-gate`: the `check-live` layer's walkthroughs pass under the
  pinned Claude Code, run by the operator.

## Impact

- `crates/flume`: none expected; the profile's unwrapping and the herdr
  test of a wrapper are on main. A failure the live run finds is fixed
  here.
- The operator: a test account, `claude setup-token`, and one run.
