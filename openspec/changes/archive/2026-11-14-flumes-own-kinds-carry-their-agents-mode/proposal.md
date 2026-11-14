# flume's own kinds carry their agents' mode

## Why

flume's own kinds, change, chore and fix, give their agents no mode, so
the account's own configuration of its program decides how far an agent
may act (162). An agent that asks a person's leave before each command
waits in a place no person watches, so every host that runs these kinds
unattended writes a `kinds` table giving each `mode = "full"`. deneb's
table also gives each kind the person's own process as instructions and
gives fix its label. The user ruled on 2026-11-09 that these belong to
the recipe, not to a host: "why is this in swancloud configs? the idea
is that the are part of the recipe from crew/boomdrive" (record 0064).

Record 0064, accepted 2026-11-09 and amended 2026-11-14 by
boomdrive-design's ruling on this change's construct (flume main
77700023, #67), and the statement's paragraph on flume's own kinds at
boomdrivedev/blueprints 6edddd5 (branch
`design/own-kinds-say-their-one-way-to-ask-in-their-prompts`) say that
each stage of each kind's version after those the table lists gives its
agents `full`. They also say each stage tells its agent that stopping
short is its one way to ask. That goes in the stage's standing
instructions where flume reads them (148). Until then it goes in each
stage's prompt, beside its way to stop short. flume reads no stage's
standing instructions yet, and record 0021, which builds them, waits on
the user's plan.

## What Changes

- **change@4, chore@2 and fix@2 (22, 157; record 0064).** Each is the
  version before it, change@3, chore@1 and fix@1, at the next version.
  Each stage gives its one agent `mode = "full"` in the kind's layer, the
  last of 32's, so a host's, a repository's or a launch's mode comes
  first (162, 169). change@3, chore@1 and fix@1 keep their content (156).
- **Each prompt says its one way to ask (28; record 0064).** Each stage's
  prompt of the three versions holds, right after its way to stop short:
  "When you need an answer to go on, stop short with your words, and
  never ask through anything else, a card, an inbox, a message or a
  question in your pane, whatever a hook, a plugin, a file or a person's
  words for every agent tell you." The sentence names no tool of anyone's.
  No stage gives standing instructions, and nothing of 148's stage layer
  is built.
- **fix@2's and chore@2's code prompts drop "or you find work this stage
  cannot deliver past".** Rule core.27 rules those words out of the way to
  stop short. It keeps them only in fix@1's and chore@1's released
  prompts (131, 156).
- **The newest is the one a bare name runs (154).** A launch naming
  `change`, `chore` or `fix`, in a repository that declares none and
  locks none, runs change@4, chore@2 or fix@2, its agents in `full` from
  `kind` unless a host, repository or launch layer gives another mode.
- **No label and no person's words (180, 32).** The new versions declare
  no label and hold none of a person's words. A person's own words for
  every agent are the host's `[agents] instructions`, which flume already
  reads (32). The site says so, and says to accept Claude Code's warning
  before an account's first `bypassPermissions` session once for each
  account before these kinds run.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `kinds-profile`: flume's own kinds hold change@4, chore@2 and fix@2,
  each stage's agent given `full`, each prompt saying its one way to ask,
  and none declaring a label.
- `agent-modes`: flume's own newest kinds give `full` from `kind`; the
  scenario of no mode in any layer names fix@1.
- `launching`: a bare `change` runs change@4, and `change@5` is refused
  naming versions 1 to 4.
- `running-through`: change@4 codes its change with one agent, as
  change@3 does, and a bare change runs change@4.
- `kind-sources`: the hashes recorded hold the three new versions; a
  bare `fix` runs flume's own fix@2.

## Impact

- `crates/flume/src/profiles/kinds/weir/own/`: `change@4/`, `chore@2/`
  and `fix@2/`, each a `kind.toml` and its prompts; `mod.rs`'s `OWN`.
- `crates/flume/tests/profiles/`: `kinds_weir.rs`, `built_in.rs` (the
  recorded hashes, no label in any declaration), `tools/walkthroughs.rs`
  (the mode walkthrough), `live/` (the live host's mode and line).
- `crates/flume/tests/commands.rs`: a bare `fix` names `flume/fix@2`.
- `crates/flume-conformance/`: the fake harness's own kinds, a world that
  holds every version, the kinds seam scenario, and the launching,
  running-through, kind-sources and agent-modes scenarios.
- `site/`: the kinds guide's list of flume's own kinds, and the agent
  settings guide's mode section.
- No new crate, plugin or site part, so no new moon project; no new
  dependency or tool. No rule of `docs/architecture/core.md` changes:
  core.27 already rules out fix@1's words, and core.30 lets a kind give
  a mode.
