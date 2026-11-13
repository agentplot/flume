# flume's addresses name boomdrivedev

## Why

blueprints, boom, boomdrive, boomdrive-cloud, crew, flume, site-theme,
sluice, strategy, timber and weir moved from the agentplot owner on
GitHub to boomdrivedev, and their sites moved with them: flume's site
answers at https://boomdrivedev.github.io/flume and the design book at
https://boomdrivedev.github.io/blueprints/flume/, while
https://agentplot.github.io/flume answers 404 (read on 2026-11-12).

The clause-coverage spec names the statement's revision on
`agentplot/blueprints` main. Each commit of agentplot/blueprints is the
same commit in boomdrivedev/blueprints, so the citation moves its owner
and keeps its meaning.

## What Changes

- The clause-coverage scenario that names the statement's revision names
  it on `boomdrivedev/blueprints` main. No clause, scenario or behaviour
  changes.
- Beside it, the tree's addresses of the moved tools name boomdrivedev:
  the README, CLAUDE.md, `openspec/config.yaml`, the rules, the flake,
  the crates' git sources, the shell's sluice release, the site and the
  plugins, as record 0065 decides.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `clause-coverage`: the statement's revision is named on
  boomdrivedev/blueprints main.

## Impact

- The herdr plugin's id and action, `agentplot.flume` and
  `agentplot.flume.places`, which the herdr-plugin spec names, are names
  and stay, as record 0065 decides.
- No code reads the spec's address: the suite's revision is a commit
  (`crates/flume-conformance/src/clauses.rs`), the same in either
  owner's repository.
