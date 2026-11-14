# Verify: a-labels-share-is-owed-and-reserved

Verified on `change/a-labels-share-is-owed-and-reserved` at 98ae281,
cut from `bolt/flumes-settings-read-as-its-records-say` (2d180f0), on
2026-11-14.

## Verdict

Archived. The gate passes, every task is ticked and holds in the tree,
and every requirement and scenario of the change's specs is met.

## The gate's checks

`sluice exec bolt/flumes-settings-read-as-its-records-say`, run once on
98ae281: 27 tasks completed, exit 0, in 1m 17s.

The run used the host's sluice, 0.93.2. The devenv shell's sluice,
0.71.0, refused before running any check, exit 3: it cannot read the
host's `~/.config/sluice/config.toml`, whose `[forge] credential` key
it does not know (`unknown field credential, expected one of app_id,
key, discovery`). The host's Nix writes that file for the host's sluice.
Nothing of the gate ran under 0.71.0, so this is the gate's one run.

The 27 tasks, all passed:

- `seams`, `core`, `fakes`, `conformance` and `flume`: format, lint and
  test each. `seams:test`, `core:test` 53, `fakes:test` 65,
  `conformance:test`, and `flume:test` 289 lib with its integration
  suites, 62, 32 and 202 among them.
- `conformance:coverage`: every clause of Parts A and B, I1 to I8 and
  S1 to S120 at 31f9f1c is cited by the suite's 933 scenarios. Task 2.2
  fixed the one problem the first verify found, a C.7 citation the
  suite's statement does not hold.
- `nix`: format, lint, typecheck and test.
- `plugin-claude:test`.
- `site`: install, prose (0 errors, 0 warnings) and build.
- `root`: format, adrs and changes.

moon warned on each task that it could not package the task's outputs
into `.moon/cache/outputs`. The warnings fail no task.

## Tasks

All 12 tasks, 1.1 to 7.1, are ticked `[x]`. Each holds in the tree:

| Task | What the tree holds |
|---|---|
| 1.1 | `Share` and `LabelRoom` hold `owed` and `reserved`. The methods are `below_owed`, `reserved_for` and `reserved_unused`. `shared` takes `(label, owed, reserved)`. `a_label_and_its_share` reads `{"owed":2}`. |
| 1.2 | `shares_of` reads `owed` and `reserved` and refuses either as "is not a whole number of agents from 0". `reserve` and `keep_free` stay unknown, with no alias. `LOCAL_KEYS` gives `streams.*.labels.*.owed` and `.reserved`. `a_labels_reserve_and_keep_free_are_read_unknown` and `a_local_label_reserve_is_not_taken` are present. |
| 2.1 | `fault_of` and `RoomWanted::KeptFree` use section 3's and section 4's words, and the `kept-free` tag is kept. The labels and host-queue scenarios are renamed, and the `reserve` and `keep_free` scenarios are added. |
| 2.2 | `what_a_label_is_owed_set_locally` cites `181` and `S85` alone. `conformance:coverage` passes. |
| 3.1 | `LabelStatus` gains `owed` and `reserved` beside `reserve` and `keep_free`, which room.rs fills with the same numbers. The answers' field list gains both. `the_labels_in_status` checks all four in the JSON form. |
| 3.2 | `label_lines` says `label <name>: owed <n>, reserved <m>, …` and `what its labels are owed is passed over: it states no bound`. |
| 4.1 | `[host] agents` is read by `bound_of`, in the words "is not a whole number from 1". `a_hosts_bound_of_zero_is_refused` is present. |
| 5.1 | `options.nix` declares `owed` and `reserved` (unsigned), each `agents` as positive, and `host.ends_expire`, with section 7's descriptions. Its header says `reserve` and `keep_free` pass undeclared. `every-key.nix` and `every-key.toml` match. `options.nix`'s tests "A bound of zero" and "Each number says its unit, range and default" are present. |
| 5.2 | `the_modules_files_read_whole` reads owed and reserved and an `ends_expire` of 48 hours. |
| 6.1 | The configuration reference gives each number's values and default, the labels' table in `owed` and `reserved`, the local keys and the refusals' words. |
| 6.2 | The agent settings guide, the state reference and the roadmap say `owed` and `reserved`, and the state reference gives `reserve` and `keep_free` as the same numbers under the names used before them. |
| 7.1 | Record 0063 gains "The construction's choices", each with the path not taken, and names the construction agent among its deciders for those choices alone. `root:adrs` passes. |

## Requirements and scenarios

`openspec validate a-labels-share-is-owed-and-reserved --strict`
passes.

- **stream-labels.** "What a label is owed and holds reserved": each of
  its five scenarios is a labels scenario: S84's owed agent lent and
  reserved agent held empty, one settle ranking again, no room on the
  host, a stream with no bound, and owed set locally.
  "A stream whose labels are at fault starts no stage": a pool,
  `reserve`, `keep_free`, owed past the bound, more reserved than owed,
  and the configuration mended are each a labels scenario. The reader
  side is `a_labels_reserve_and_keep_free_are_read_unknown`.
  "Status gives what each label is owed and holds reserved" is covered
  by `the_labels_in_status` and by `waiting_on_an_agent_held_reserved`,
  whose JSON is `kept-free` and whose words are "held reserved". The
  person's form is covered by `a_streams_labels`.
- **host-queue.** "An agent another label holds reserved" and "A label
  below what it is owed before a place's priority" are host-queue
  scenarios under those names. The requirements' other scenarios are
  unchanged and pass.
- **host-configuration.** "A label's share set locally" and "A label's
  old key set locally" are the two halves of
  `a_local_label_reserve_is_not_taken`.
- **seam-contracts.** "The fake host's sessions, labels and layers" is
  `the_hosts_sessions_labels_and_layers` in `streams.rs`, with
  `operation` owed 2 and reserved 1.
- **streams.** "A host's bound of zero" is
  `a_hosts_bound_of_zero_is_refused`. The configuration reference gives
  each bound as from 1 and `priority` as higher starting first.
- **host-module.** `nix:test` passes. That covers "A mistyped declared
  key", "A bound of zero", "Each number says its unit, range and
  default" and "The declared keys".

The code keeps `reserve` and `keep_free` only where the change means
it to:

- the JSON form's two fields;
- the tests of a refused old key;
- the module's header and its tests of undeclared keys;
- the site's words on the refusal;
- the `kept-free` tag.

## The statement

The change rests on boomdrivedev/blueprints 695c446 ("design(flume):
C.7 gives each number flume reads its unit, range and default, and S84
says reserved"). It is on the branch
`design/flume-a-labels-share-is-owed-and-reserved` in the host's clone
`~/Code/github_agentplot/blueprints`, and no remote holds it yet. The
orchestrator pushes it, as the design's section 8 says. The conformance
suite reads the statement at 31f9f1c.
