# Verify: a-labels-share-is-owed-and-reserved

Verified on `change/a-labels-share-is-owed-and-reserved` at 8d084ba,
cut from `bolt/flumes-settings-read-as-its-records-say`, on 2026-11-14.

## Verdict

Not archived. The gate fails one task, `conformance:coverage`. The work
otherwise meets every requirement and scenario of the change's specs,
and every task is ticked.

## The gate's checks

`sluice exec bolt/flumes-settings-read-as-its-records-say`, run once on
8d084ba: 26 tasks completed and 1 failed, exit 1.

Failed:

```
conformance:coverage | 1 problems with the suite's citations of the statement at 31f9f1c:
conformance:coverage |   the scenario "labels: what a label is owed, set by the local layer's streams.work.labels.operation.owed, is held from the next start" cites C.7, which the statement does not hold
```

The scenario is `what_a_label_is_owed_set_locally`, at
`crates/flume-conformance/src/engine/labels.rs:154` to 156. Task 2.1
renamed it and added `"C.7"` to its `cites`, which were
`["181", "S85"]` before. The suite's clause list
(`crates/flume-conformance/src/clauses.rs`, the statement at 31f9f1c)
holds no C.7, and no other scenario cites a C clause. Task 2.1's own
check, `the_engine_passes_its_scenarios`, does not run the coverage
check, so the task's test passed.

Passed, among the rest: `site:prose` (0 errors, 0 warnings), `site:build`,
`seams:test` (109), `core:test` (53), `fakes:test` (65), `conformance:test`,
`flume:test` (289 lib, 62, 32 and 202 integration, among others),
`plugin-claude:test`, `root:changes`, and each crate's format and lint.

## Tasks

All 11 tasks, 1.1 to 7.1, are ticked `[x]`. Each holds in the tree:

| Task | What the tree holds |
|---|---|
| 1.1 | `Share` and `LabelRoom` hold `owed` and `reserved`. The methods are `below_owed`, `reserved_for` and `reserved_unused`. `shared` takes `(label, owed, reserved)`. `a_label_and_its_share` reads `{"owed":2}`. |
| 1.2 | `shares_of` reads `owed` and `reserved` and refuses either as "is not a whole number of agents from 0". `reserve` and `keep_free` are kept unknown, with no alias. `LOCAL_KEYS` gives the new two. Both new tests are present. |
| 2.1 | `fault_of` and `RoomWanted::KeptFree` use section 3's and section 4's words, and the `kept-free` tag is kept. The labels and host-queue scenarios are renamed, and the `reserve` and `keep_free` scenarios are added. |
| 3.1 | `LabelStatus` gains `owed` and `reserved` beside `reserve` and `keep_free`, which room.rs fills with the same numbers. The answers' field list gains both. `the_labels_in_status` checks all four in the JSON form. |
| 3.2 | `label_lines` says `label <name>: owed <n>, reserved <m>, …` and `what its labels are owed is passed over: it states no bound`. |
| 4.1 | `[host] agents` is read by `bound_of`, in the words "is not a whole number from 1". `a_hosts_bound_of_zero_is_refused` is present. |
| 5.1 | `options.nix` declares `owed` and `reserved` (unsigned), each `agents` as positive, and `host.ends_expire`, with section 7's descriptions. `every-key.nix` and `every-key.toml` match. `options.nix`'s tests "A bound of zero" and "Each number says its unit, range and default" are present. |
| 5.2 | `the_modules_files_read_whole` reads owed and reserved and an `ends_expire` of 48 hours. |
| 6.1 | The configuration reference gives each number's values and default, the labels' table, the local keys and the refusals' words. |
| 6.2 | The agent settings guide, the state reference and the roadmap say `owed` and `reserved`. |
| 7.1 | Record 0063 gains "The construction's choices", each with the path not taken, and names the construction agent among its deciders for those choices alone. |

## Requirements and scenarios

- **stream-labels.** "What a label is owed and holds reserved": S84's
  lent and reserved agents, one settle ranking again, no room on the
  host, a stream with no bound, and owed set locally are each a labels
  scenario. "A stream whose labels are at fault starts no stage": a
  pool, `reserve`, `keep_free`, owed past the bound, more reserved than
  owed, and the configuration mended are each a labels scenario, and
  `a_labels_reserve_and_keep_free_are_read_unknown` covers the reader.
  "Status gives what each label is owed and holds reserved" is covered
  by `the_labels_in_status` and `waiting_on_an_agent_held_reserved`,
  whose JSON is `kept-free` and whose words are "held reserved", and by
  `a_streams_labels` for the person's form.
- **host-queue.** Covered by "An agent another label holds reserved"
  (`an_agent_another_label_holds_reserved`) and "A label below what it
  is owed before a place's priority"
  (`a_label_below_what_it_is_owed_before_a_places_priority`). The other
  scenarios are unchanged and pass.
- **host-configuration.** Both halves of `a_local_label_reserve_is_not_taken`
  cover "A label's share set locally" and "A label's old key set
  locally".
- **seam-contracts.** "The fake host's sessions, labels and layers" is
  `the_hosts_sessions_labels_and_layers` in `streams.rs`, with
  `operation` owed 2 and reserved 1.
- **streams.** "A host's bound of zero" is `a_hosts_bound_of_zero_is_refused`.
  The configuration reference gives each bound as from 1 and `priority`
  as higher starting first.
- **host-module.** `nix:test` passed in the gate, so "A mistyped declared
  key", "A bound of zero", "Each number says its unit, range and
  default" and "The declared keys" all hold.

The code keeps `reserve` and `keep_free` only where the change keeps
them on purpose: the JSON form's two fields, the tests of a refused old
key, the module's header and its tests of undeclared keys, and the
`kept-free` tag.

## The statement

The change rests on boomdrivedev/blueprints 695c446 ("design(flume):
C.7 gives each number flume reads its unit, range and default, and S84
says reserved"). It is on the branch
`design/flume-a-labels-share-is-owed-and-reserved` in the host's clone
`~/Code/github_agentplot/blueprints`, cut from 81288ba. The branch is
not on GitHub yet; the orchestrator pushes it, as the design's section 8
says. The conformance suite reads the statement at 31f9f1c.

## What the code stage must do

Make `conformance:coverage` pass. Either drop `"C.7"` from the cites of
`what_a_label_is_owed_set_locally` (labels.rs:155), leaving `181` and
`S85`, which already say a local layer's share, or cite only clauses the
suite's statement holds. Then run the gate again.
