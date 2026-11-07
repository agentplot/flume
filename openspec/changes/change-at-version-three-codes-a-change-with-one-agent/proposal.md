# change at version 3 codes a change with one agent

## Why

change@2's code stage starts a fresh agent for each task group (record
0019), and the constructs that write the changes end every group with a
task that runs the repository's whole checks. A change of six groups
runs the whole suite six times through the merge queue's runner, each
run waiting for room under the host's bound, then again at verify and
again at the merge. On deneb on 2026-11-06 one group's run waited 71
minutes for room, and whole-suite runs waited 40 minutes to over two
hours. The user named it wrong and chose "One agent, test once" (record
0054, accepted 2026-11-06): one agent codes the whole change, each task
checks only what it touched, and the whole checks run once, at verify.
The statement carries it at agentplot/blueprints 49b9d53 and 9371cda:
22, 154, 157 and C.2 amended and S119 added.

## What Changes

- **flume's own change@3.** A new version of flume's own `change`,
  built into the binary beside change@1 and change@2, each version a
  package of its own whose content never changes (156, 157):
  - its **construct** writes the change with each task naming the
    narrowest check that shows that task done, one test by name, one
    package's tests or a lint of the files it touched, and no task
    running the repository's whole checks or checking a task group, the
    groups only headings;
  - its **code** stage does not repeat: one agent works through every
    task in order, running each task's own check and ticking it once the
    check passes;
  - its **verify** runs the repository's whole checks once, as the
    repository's instructions name them, then checks the work against
    the change as change@2's does, and stops short having found work
    where either fails (22, 131, C.2).
- **The newest is the default.** change@3 is the newest version of
  flume's own change, so a launch naming no version, in a repository
  whose lock records none, runs it (154, 155). change@1 and change@2
  stay side by side, a launch may name either, and a place launched at
  either keeps it to its end (21).
- **The repeat field stays the format's.** `repeat = "per-task-group"`
  stays for a repository's own kinds and for change@2 (140 to 142);
  change@3 does not use it.
- **This repository's planning rules.** `openspec/config.yaml` says each
  task names the narrowest check that shows it done and that no task
  runs the whole checks or a group's; `CLAUDE.md` says the gate's checks
  are run once, by a change's verify stage (record 0054's consequences).
- **The suite reads the statement at the revision holding S119.** The
  conformance suite's clause list moves to that revision, an engine
  scenario cites S119, and every clause and scenario the statement
  added since the list's revision that no change has built is named
  unbuilt by the change that builds it (106; core.19).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `kinds-profile`: flume's own kinds gain change@3, its code stage not
  repeating, with its three prompts; change@1's and change@2's prompts
  unchanged; flume's own packages gain `flume/change@3`.
- `launching`: a launch naming no version runs change@3, and one naming
  change@2 runs its repeating code stage.
- `kind-sources`: the hashes recorded for flume's own kinds gain
  change@3's, the others unchanged.
- `running-through`: a change@3 place runs through with one agent coding
  its whole change (S119).
- `clause-coverage`: the suite's list at the revision holding S119,
  S119 cited and what other changes build named unbuilt.

## Impact

- `crates/flume/src/profiles/kinds/weir/own/change@3/`: a new
  `kind.toml` and its three prompts; `crates/flume/src/profiles/kinds/weir/mod.rs`
  registers it in `OWN`.
- `crates/flume/tests/profiles/kinds_weir.rs`, `built_in.rs` and
  `tools/walkthroughs.rs`: change@3 among flume's own, its hash, its
  prompts, and the walkthroughs of a repeating stage naming change@2.
- `crates/flume-conformance`: the fake harness's own kinds gain
  change@3; the engine's and the kinds seam's scenarios that name the
  newest or rely on repetition; a scenario citing S119; the clause list.
- `CLAUDE.md` and `site/src/content/docs/guides/kinds.mdx`;
  `openspec/config.yaml`'s rules for record 0054, the base's since #1,
  kept as they stand.
- No new crate, plugin or site part, so no new moon project; no seam,
  core or weir change: flume's own kinds are data the profile reads.
