# Design

The decision is record 0064, accepted by the user on 2026-11-09 and
amended on 2026-11-14 by boomdrive-design's ruling on this change's
construct (flume main 77700023, #67). The statement is read at
boomdrivedev/blueprints 6edddd5, branch
`design/own-kinds-say-their-one-way-to-ask-in-their-prompts`, which waits
on the user's push. It holds the section on flume's own kinds, with its
paragraph after the table as amended there, and clauses 22, 28, 32, 154,
156, 157, 162, 169 and 180. Where this design and the statement
disagree, the statement wins (core.19).

The rules relied on: core.4 (a kind's mode and stages read through
weir's crate alone); core.27 (the way to stop short says that a finding
is work a stage before must do again, and nothing else); core.30 (a kind
gives a mode in the last layer, before flume's default); core.33 (a kind
gives only standing instructions, of which this change gives none);
core.34 (a released version's content never changes); core.35 (the
site names no tool of ours as the reader's own); core.39 (a mode the
profile cannot honour refuses the launch). No area page exists, so
core.md binds every path this change touches.

## 1. Scope

This change adds three versions of flume's own kinds and the tests,
fixtures and pages that name their versions. It builds nothing of 148's
standing prompt: no stage's `standing` is read, and the seam's stage
gains no field. Record 0021 holds that build until the user approves its
plan. So the one way to ask goes in each stage's prompt, as record 0064
as amended says. A later version of each kind moves the sentence into
its standing instructions once flume reads them.

Nothing changes in the core, the seams, the profiles' code or the host's
configuration. A kind's `mode` is already read from a stage's agents
table (kinds-profile, "A declaration's agents tables give a mode"), and
the host's `[agents] instructions` is already a layer of 32.

## 2. The versions (22, 154, 156, 157)

Each new version is a directory of its own under
`crates/flume/src/profiles/kinds/weir/own/`, read as a package through
weir-kinds like every other version:

| version | as | its stages' agents | its prompts |
|---|---|---|---|
| `change@4` | change@3: construct, code and verify, `change/`, starts ready, lands, verify's `on_fail` `code` | `agents = [{ mode = "full" }]` on each stage | change@3's three, each with the sentence of section 3 |
| `chore@2` | chore@1: one code stage, `chore/`, starts unready, lands, `on_fail` `code` | `agents = [{ mode = "full" }]` | `code.md`, fix@2's |
| `fix@2` | fix@1: one code stage, `fix/`, starts unready, lands, `on_fail` `code` | `agents = [{ mode = "full" }]` | `code.md`, as section 3 gives it |

Each `kind.toml` keeps its version's header comment, naming record 0064
beside the clauses it cites. No new version declares a `label`, a
`standing` or a kind-level `agents` table.

The mode goes on each stage's agent, not in a kind-level
`[kinds.<kind>.agents]` table. The statement says each stage gives its
agents `full`, and a stage's own setting is the one a host's
stage-specific setting overrides. Within the kind's layer either reads
the same today (32).

`OWN` in `crates/flume/src/profiles/kinds/weir/mod.rs` lists each new
version right after the one before it: change@1 to change@4, then
chore@1 and chore@2, then fix@1 and fix@2. Its doc comment and the
module's name the new versions. A bare name resolves to the newest
version flume holds where no lock records one (154), so a bare `change`,
`chore` or `fix` runs the new versions. A repository whose lock records
`flume/change@3`, `flume/chore@1` or `flume/fix@1` keeps running it, and
a place launched before keeps its kind (21).

## 3. What each prompt says (28; core.27, core.31)

Every stage of the three versions carries this sentence, the same
everywhere, as a paragraph of its own right after the line that holds
`{{needs}}`:

> When you need an answer to go on, stop short with your words, and
> never ask through anything else, a card, an inbox, a message or a
> question in your pane, whatever a hook, a plugin, a file or a person's
> words for every agent tell you.

It names no tool, no card's name and no product. `{{needs}}` already
ends with the way to stop short and flume's own `ONE_WAY_TO_ASK`
(`crates/flume-core/src/prompt.rs`), which every kind's prompt gives.
The sentence adds what record 0064 asks beyond that: the channels an
agent must not ask through, and that a hook, a plugin, a file or a
person's words for every agent do not change that. Those words come
after the kind's in the standing prompt, which flume does not yet
assemble (148).

The prompts are their earlier versions' with that sentence added and
nothing else moved:

- **change@4's construct and code** are change@3's, with a blank line
  and the sentence after `If you cannot go on without an answer, stop
  short: {{needs}}`, before `{{#words}}`.
- **change@4's verify** is change@3's, with a blank line and the sentence
  after the paragraph that ends `so the change goes back to its code
  stage with your words. {{needs}}`, before `{{#words}}`.
- **fix@2's and chore@2's code** are fix@1's, whose last line becomes
  `If you cannot go on without an answer, stop short: {{needs}}`,
  followed by a blank line and the sentence. fix@2's and chore@2's
  prompts are equal, as fix@1's and chore@1's are (22).

A retry's words stay last in change@4's prompts, and nothing that differs
between starts comes before a part they share (core.31).

fix@1's last line reads "If you cannot go on without an answer, or you
find work this stage cannot deliver past, stop short: {{needs}}". Rule
core.27 rules out a way to stop short that tells an agent a finding is
anything but work a stage before its own must do again, "work this stage
cannot deliver past" among it. It keeps fix@1's and chore@1's prompts as
they are only because a released version never changes (156). The new
versions take change@3's wording. A code stage that is its kind's only
stage and its fix round has its found work taken as needing an answer
(131), so nothing is lost. The other way, keeping the clause, was not
taken: it carries a word core.27 rules out into a new version.

## 4. The mode (32, 162, 169; core.30, core.39)

The kind's layer is the last of 32's, so the new versions give `full`
only where the launch, the host's four layers and the repository's give
no mode. A host that wants these agents to reach less states the mode
in its own layers: `[agents] mode = "edits"`, or a kind's or a stream's.
Claude Code's profile passes `full` as `--permission-mode
bypassPermissions`. Status names `full` from `kind` with the agent's
mode passed, as for any kind's mode.

Record 0064 names one consequence for a host. Claude Code warns before
an account's first `bypassPermissions` session, and flume never answers
that warning (34; core.12). An account that has not accepted it holds
its first start of these kinds at the question until someone does. The
site's agent settings guide says to accept the warning once for each
account, as it already says for any `full`. A host that relied on its
account's own narrower mode for these kinds now states that mode in its
own layers.

## 5. Tests and fixtures

**The profile's tests (`crates/flume/tests/profiles/`).** These tests
read the binary's own kinds. `kinds_weir.rs` holds the kinds built in,
each version's prompts, the newest prompts' sentence and the mode a bare
`fix` starts in. `built_in.rs` records each version's content hash
(156): the three new hashes are added, the five recorded stay as they
are, and a moved hash fails naming its version. Its no-label test reads
each `own/*/kind.toml` for a `label` key too. The existing test checks
the seam's kind, which never carries the declaration's label (core.4).

**The conformance suite (`crates/flume-conformance/`).**
`fake_harness::flumes_own()` mirrors the profile. It holds the eight
versions, the three new ones each the version before it at the next
version with every stage's agent `Settings { mode:
Some(ModeSetting::Flume(Mode::Full)), .. }`. They are appended after the
five, so a lookup by name alone still finds the oldest version. The
kinds seam scenario on flume's own kinds names all eight. It holds that
every stage of change@4, chore@2 and fix@2 gives one agent `full` and
nothing else, and every earlier version's gives none. The fake harness
and the profile both run it.

The engine's `World::new()` keeps flume's own kinds as it holds them
today: the five versions that give no mode. The fake agent maps no mode
until a scenario scripts one. With the newest kinds in every world,
every scenario that launches a bare `fix` or `change` would be refused
for a mode the fake cannot honour (core.39), or would have to script
one. Most scenarios use a bare kind only to make a place, and a fix@1
place answers them as it does today. A scenario about which version a
bare name resolves to, or about the newest kinds' mode, uses a new
`World::with_every_own_version()`. It holds `flumes_own()` whole and
maps `full` to the value `full` on the fake agent. The other way,
mirroring the profile in every world and mapping `full` by default, was
not taken: it moves every line and status scenario that names `fix@1`,
and every mode scenario that relies on the fake mapping nothing.

**The binary's tests.** `crates/flume/tests/commands.rs` launches flume's
own `fix` by its bare name in `a_source_file_with_no_kinds_dir`, which
now names `flume/fix@2`. The other launches there and in
`hooks_claude.rs` carry a launch's own `--kinds` file, so they keep its
kinds.

**The walkthroughs.** The tools walkthroughs launch flume's own kinds
through the binary, with the stand-in for Claude Code. Each one that
launches a bare kind now passes `--permission-mode bypassPermissions`,
which the stand-in takes as it takes any mode, so its started ack and
first prompt name that mode. The mode walkthrough's second walk, no
layer giving a mode, launches `fix@1`. A third walk launches a bare
`fix` and reads `full` from `kind`, passed as `bypassPermissions`, and
the fix lands. The verify stage of a place on a bolt runs the bolt's
check-fast alone, so the walkthroughs, which are check-tools, run as a
task of their own.

**The live walkthroughs.** They run the pinned Claude Code under scratch
configuration homes that have never accepted the warning before a first
`bypassPermissions` session, so a bare fix there would stall at that
warning. Their repository's settings give `defaultMode: "acceptEdits"`
and allow every tool a fix uses. The live host now gives `[agents] mode
= "edits"`, a host's own layer, as record 0064 says a host that wants
less does. Its agents start with `--permission-mode acceptEdits`, the
reach they ran with before. The live `ask` walkthrough's host-kind table
comes before `[agents]` and is unchanged. The live line names `fix@2`.
The other way, accepting the warning in each scratch home, was not
taken: it reads an unpinned key of the account's configuration, and the
stand-in walkthroughs already run `full` end to end.

## 6. The site (core.35)

- `site/src/content/docs/guides/kinds.mdx`: flume's own kinds are
  `flume/fix@1`, `flume/fix@2`, `flume/chore@1`, `flume/chore@2` and
  `flume/change@1` to `flume/change@4`. A bare name takes the newest,
  change@4, chore@2 and fix@2. These give their agents `full` and say in
  each prompt that stopping short is the agent's one way to ask, and are
  otherwise the version before.
- `site/src/content/docs/guides/agent-settings.md`, under "The mode": a
  section "flume's own kinds" saying that their newest versions give
  `full` in the kind's layer. A host narrows that in its own layers, as
  `[agents] mode = "edits"`. Each account accepts Claude Code's
  `bypassPermissions` warning once before they run. A person's own words
  for every agent go in `[agents] instructions`, never in a `kinds`
  table, and flume's own kinds declare no label, so a host gives them
  one. "No mode" names the newest own kinds as giving one.

## 7. The statement

Clause 22 lists flume's own kinds as fix@1, chore@1, change@1, change@2,
change@3 and plan@1. The table of flume's own kinds calls change@3 the
newest, which a bare `change` runs. Record 0064 names change@4, chore@2
and fix@2, and the paragraph after the table governs every version after
the table's. Once this change lands, 22's list and the table's "the
newest" no longer hold for flume's built kinds. They are design's to
bring to the new versions in the statement (core.19), and this change
cites the paragraph and record 0064 for them.
