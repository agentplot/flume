# Verify

The verify stage of `a-standing-agents-command-acts-as-its-address`, on
`change/a-standing-agents-command-acts-as-its-address` at 06945b7, cut
from `bolt/flumes-places-need-no-hand` at 4e7fa04, on 2026-11-14.

## The whole checks

`sluice exec bolt/flumes-places-need-no-hand`, run once with the host's
sluice (0.93.2), passed: the bolt requires `#check-fast`, and sluice ran
the 24 tasks 06945b7 affects, every one passing, and recorded 23
results under their keys for 06945b7.

| project | tasks |
|---|---|
| `seams` | format, lint, test |
| `core` | format, lint, test |
| `fakes` | lint, test |
| `conformance` | format, lint, test, coverage |
| `flume` | format, lint, test (202 profile tests and the rest) |
| `plugin-claude` | test |
| `nix` | typecheck, test |
| `site` | install, build, prose |
| `root` | format, adrs, changes |

Vale gave suggestions only (`Flume.SentenceLength`, `Flume.Passive`),
none an error. moon warned once that it could not archive `flume:test`'s
outputs into its cache, which fails nothing.

The same command run first through `devenv shell --` took the sluice
the tree pins for the merge-queue profile's tests (0.71.0), which
refused the host's `~/.config/sluice/config.toml` (`unknown field
credential`) before any check ran; the run above is the gate's one run.

`check-tools` is main's, not the bolt's. The three tools tests this
change adds were run as tasks 4.1, 4.2 and 5.1 name them, `cargo test -p
flume --features tools --test tools`, under the pinned herdr and Rex: 3
passed, 0 failed.

`openspec validate a-standing-agents-command-acts-as-its-address
--strict`: valid.

## Tasks

Every task of `tasks.md`, 1.1 to 6.1, is ticked done, and each is borne
out by the branch's commits e96fcc4 to 06945b7.

## Requirements and scenarios

### role-waking

**A standing agent's command acts as the address it holds.**
`Engine::standing_here` (`crates/flume-core/src/standing.rs`) reaches
each standing address's agent as a delivery does and holds it where the
one agent's `at` is the session its stream's terminal names for the
command; `Engine::recipient` (`wait.rs`) takes it after
`$FLUME_RECIPIENT` and a role's run, before `caller_session`;
`recipient_of` and `answering` (`crates/flume/src/commands/mod.rs`) pass
it to confirm, inbox, wait, retry and tell.

| scenario | test | outcome |
|---|---|---|
| A confirm from the standing agent's pane | `role_waking.rs` `a_confirm_from_the_standing_agents_pane` | met |
| An inbox and a wait from the standing agent's pane | `an_inbox_and_a_wait_from_the_standing_agents_pane` | met |
| An answer from the standing agent's pane | `an_answer_from_the_standing_agents_pane` | met |
| A standing agent under herdr confirms through the binary | `walkthroughs::a_standing_agent_under_herdr_confirms_its_end_naming_no_recipient` | met, passing under the pinned herdr |

**Elsewhere a command acts as before.**

| scenario | test | outcome |
|---|---|---|
| What the command names comes first | `what_the_command_names_comes_first` | met |
| A pane no standing agent holds | `a_pane_no_standing_agent_holds` | met |
| Two agents of the standing agent's name | `two_agents_of_the_standing_agents_name` | met |
| Nothing asked where no standing agent is declared | `nothing_asked_where_no_standing_agent_is_declared`, the fake terminal's `agents_in` count unchanged | met |
| A part that cannot be read | `a_part_that_cannot_be_read` | met |

**An agent standing for more than one address names which.**
`StandingHere::Several` gives the refusal `several_words` words, the
addresses sorted by the entries' `BTreeMap`.

| scenario | test | outcome |
|---|---|---|
| One agent for two addresses | `one_agent_for_two_addresses` | met |
| The address named | `the_address_named` | met |
| A retry from the pane of two addresses | `a_retry_from_the_pane_of_two_addresses` | met |

### launching

**A launch's recipients.** `recipients_of` (`crates/flume-core/src/launch.rs`)
takes the launch's, `$FLUME_RECIPIENT`, the standing agent's one
address as `RecipientsFrom::Standing`, `caller_session`, then the
answerer, and refuses a pane of several addresses before anything is
made.

| scenario | test | outcome |
|---|---|---|
| A launch from a standing agent's pane | `launching.rs` `a_launch_from_a_standing_agents_pane` | met |
| A launch from the pane of an agent standing for two addresses | `a_launch_from_the_pane_of_two_addresses`: no held state, branch or submission | met |
| Recipients at the launch; a launch from no session takes its stream's answerer; each stream its own answerer | existing scenarios, passing unchanged | met |

### place-status

**Status gives who answers a place.**

| scenario | test | outcome |
|---|---|---|
| A recipient from a standing agent's launch | `print.rs` `recipients_with_where_they_came_from` for the person's form; `a_launch_from_a_standing_agents_pane` for the JSON's `from`, `agent` and `source` | met |
| Recipients and their sources; an answered question | existing scenarios, passing unchanged | met |

### seam-contracts

**The type of a recipient a standing agent's launch gave.**
`RecipientsFrom::Standing { agent, source }` in
`crates/flume-seams/src/held.rs`, serialized `from` `standing`.

| scenario | test | outcome |
|---|---|---|
| Round trips | `round_trip.rs` `a_held_state_with_where_its_recipients_came_from`, the five variants and a state written before | met |

### terminal-profiles

**A command's pane is named as its agents are given.** Neither profile
needed a change: under the pinned releases each `caller` already names
the command's pane in its `agents_in`'s form.

| scenario | test | outcome |
|---|---|---|
| A command in a herdr session's pane | `terminal_herdr::a_command_in_a_herdr_sessions_pane_is_named_as_its_agents_are_given` | met, passing under the pinned herdr |
| A command in a labelled Rex block | `terminal_rex::a_command_in_a_labelled_block_is_named_as_its_agent_is_given` | met, passing under the pinned Rex |

## The record, the rule and the site

- Record 0013 gains "The construct's reading, 2026-11-14", design 7's
  decisions with the paths not taken.
- Rule core.28's line names the standing agent's command, and its
  details what they leave open; `site/src/content/docs/architecture/rules.md`
  holds core.28's line as `docs/architecture/core.md` does, character
  for character.
- `guides/roles-and-wake.md`, `reference/commands.md` and
  `reference/state.md` say design 8; the lines the site gains name none
  of core.35's words.

## The archive

Archiving the change breaks the gate. `openspec archive
a-standing-agents-command-acts-as-its-address --yes` applies the deltas,
and `openspec validate --specs --strict --no-interactive`, `root:specs`
of `check-fast`, then fails `launching`:

```
⚠ [WARNING] requirements[1]: Requirement text is very long (>500 characters). Move examples and edge cases into scenarios, or split it into separate requirements that each state one behavior.
```

The MODIFIED "A launch's recipients" of `specs/launching/spec.md` is 705
characters, 390 in `openspec/specs/launching/spec.md` on the base, and
`--strict` fails a warning. `openspec validate --changes --strict`,
`root:changes`, does not apply the limit to a delta, so the gate passed
with the change open. The archive was undone and nothing archived: the
specs pass as the branch stands (49 passed, 0 failed).

The change's launching delta must keep "A launch's recipients" within
500 characters, for one the refusal of a launch from the pane of an
agent standing for several addresses as a requirement of its own,
ADDED, with the scenario "A launch from the pane of an agent standing
for two addresses" beneath it; and an archive of the change into a
scratch copy then passing `openspec validate --specs --strict`.

## Verdict

The whole checks pass and the code holds: every requirement and
scenario of the five delta specs is met by the code and its tests, and
every task is done. The change falls short of archiving: its launching
delta's modified requirement is over the 500 characters `root:specs`
allows once it is archived. Nothing is archived, and the change goes
back to its code stage.
