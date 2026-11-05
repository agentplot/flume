# A place's readiness installs its repository's pinned tools

## Why

A place made from a fresh clone cannot be made ready: its toolchain
step passes, then `moon setup` fails with `moon: command not found`.
deneb showed it on 2026-11-05, in a place launched from a clone of
agentplot/flywheel-cloud. That repository's toolchain line, `devenv
print-dev-env`, sets `PROTO_HOME` to `<git common dir>/proto` and
activates proto's pins, but installs none, and in a fresh clone nothing
has installed them. Many repositories have the same shape: their tools
are installed only when a person enters `devenv shell`.

sluice reads the same policy and has no such gap. Its moon runner runs
`proto install --config-mode upwards` before any moon command, where the
checkout has a `.prototools`, with the repository's toolchain sourced so
proto is on the path. flume's sluice profile reads that policy in
`for_place` and gives a moon repository the readiness `moon setup`
alone (11, 12, 20, 79, C.1, C.6).

## What Changes

- **The install of what the base pins (11, 20, C.6).** Where the policy's
  runner is moon and the base's revision the policy is read at holds a
  `.prototools`, the sluice profile's `for_place` names
  `proto install --config-mode upwards` as the place's install. It reads
  the file at that revision, never from a place's tree, since the place
  is cut from it (20).
- **A step of its own (12, 79).** The resolver hands the place profile the
  toolchain, the install and `moon setup` as three steps, the install
  run under the toolchain's environment. Status names the install with
  its words. An install that fails stops the readiness there, with
  proto's words, and `moon setup` is not run. A repository that pins
  nothing runs no install.
- **Conformance** gains a place seam scenario: a repository whose pins
  nothing installed is made ready, the install a step of its own with
  its words, before the step that runs what it installed. The git
  worktree profile holds it with a stand-in for proto over a toolchain
  that activates proto's pins and installs none.
- **The site** lists the install between the toolchain and `moon setup`
  in the guide to places, and in the merge queue's guide beside the
  moon runner's row.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `merge-queue-profile`: a moon repository's pins installed before its
  setup, a readiness step of its own under the toolchain's environment.

## Impact

- `crates/flume`: the sluice profile's `for_place`, `ForPlace`'s
  `install`, `PINS` and `policy_at`; the resolver's `place_data` in
  `profiles.rs`; the place profile's and the sluice profile's tests.
- `crates/flume-conformance`: the place seam's scenario, `Pinned` and
  `PlaceHarness::pinned`, with the fake's and the lax harness's.
- `site/`: the guide to places and the merge queue's guide.
- agentplot/blueprints: the statement is not amended here. C.1's ready
  column and C.6's place row name the toolchain and the runner's setup
  alone, C.6's readiness paragraph reads a place's readiness from
  `.config/sluice.toml` alone, and C.2's `merge_queue = "sluice"` line
  names only where the base lives. Whether each names the install is
  the operator's to rule (core.19).
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool: proto is the repository's own, run under its
  toolchain.
