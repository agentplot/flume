# Verify: a-labels-share-is-owed-and-reserved

Verified on `change/a-labels-share-is-owed-and-reserved` at 94eb963,
cut from `bolt/flumes-settings-read-as-its-records-say` (2d180f0), on
2026-11-14.

## Verdict

Archived. The gate passes on 94eb963, all 14 tasks are ticked and hold
in the tree, and every requirement and scenario of the change's specs
is met. Once archived, every spec under `openspec/specs/` passes
`openspec validate --specs --strict`, 49 of 49.

## The gate's checks

`sluice exec bolt/flumes-settings-read-as-its-records-say` was run once
on 94eb963. It completed 27 tasks and exited 0 in 1m 0s. sluice
recorded 26 results under their keys for 94eb963. The 27th task,
`site:install`, sets up the site's build and is not a check.

The run used the host's sluice, 0.93.2. The devenv shell's sluice,
0.71.0, cannot read the host's `~/.config/sluice/config.toml`, which
the host's Nix writes for the host's sluice.

All 27 tasks passed:

- `seams`, `core`, `fakes`, `conformance` and `flume`: format, lint and
  test each.
  - `seams:test` ran 109 tests, `core:test` 53 and `fakes:test` 65.
  - `flume:test` ran 289 library tests, and its integration suites ran
    62, 32 and 202 among them.
  - `conformance:test` passed.
- `conformance:coverage`: every clause of Parts A and B, I1 to I8 and
  S1 to S120 at 31f9f1c is cited by the suite's 933 scenarios.
- `nix`: format, lint, typecheck and test.
- `plugin-claude:test`.
- `site`: install, prose and build.
- `root`: format, adrs and changes.

`root:specs` was not among them, since no file under `openspec/specs/`
differs from the base. The archive changes those files, so their strict
validation was checked separately (see the archive section below).
moon warned on each task that it could not package the task's outputs
into `.moon/cache/outputs`. These warnings fail no task.

## Tasks

All 14 tasks, 1.1 to 7.1, are ticked `[x]`. The code has not changed
since 98ae281. After it, only the change's two delta specs and its
tasks changed (94eb963). Each task holds in the tree:

| Task | What the tree holds |
|---|---|
| 1.1 | `Share` and `LabelRoom` hold `owed` and `reserved`. The methods are `below_owed`, `reserved_for` and `reserved_unused` in `room.rs`. `shared` takes `(label, owed, reserved)`. `a_label_and_its_share` reads `{"owed":2}`. |
| 1.2 | `shares_of` reads `owed` and `reserved` and refuses either as "is not a whole number of agents from 0". Other keys stay unknown, `reserve` and `keep_free` among them, with no alias. `LOCAL_KEYS` gives `streams.*.labels.*.owed` and `.reserved`. `a_labels_reserve_and_keep_free_are_read_unknown` and `a_local_label_reserve_is_not_taken` are present. |
| 2.1 | `fault_of` uses section 3's words ("a label's table holds owed and reserved alone; its stream's pool, points and pace are its budget"). `RoomWanted::KeptFree` uses section 4's words ("the rest held reserved for"), and the `kept-free` tag is kept. The labels and host-queue scenarios carry the specs' names. |
| 2.2 | `what_a_label_is_owed_set_locally` cites `181` and `S85` alone. `conformance:coverage` passes. |
| 3.1 | `LabelStatus` gives `owed` and `reserved` beside `reserve` and `keep_free`. The answers' field list gives `status.host.streams[].labels[].owed` and `.reserved`. `the_labels_in_status` checks all four. |
| 3.2 | `label_lines` gives `label <name>: owed <n>, reserved <m>, …` and `what its labels are owed is passed over: it states no bound`. |
| 4.1 | `[host] agents` is read by `bound_of`, in the words "is not a whole number from 1". `a_hosts_bound_of_zero_is_refused` is present. |
| 4.2 | streams' MODIFIED "A host declares its streams" keeps the streams, each one's bound, priority and pool, `default` and the host's cap. A bound's range is in the ADDED "A bound on agents is a whole number from 1", with "A host's bound of zero" under it. Once archived, `streams` validates strict. |
| 5.1 | `options.nix` declares `owed` and `reserved` (unsigned), each `agents` as positive, and `host.ends_expire`, with section 7's descriptions. Its header says `reserve` and `keep_free` pass undeclared. `nix:test` passes. |
| 5.2 | `the_modules_files_read_whole` reads owed and reserved and an `ends_expire` of 48 hours. `flume:test` passes. |
| 5.3 | host-module's MODIFIED "settings names the managed file's tables and keys" keeps the declared tables and each number's type and range, with "A mistyped declared key" and "A bound of zero". The descriptions are in the ADDED "Each declared number says its unit, range and default", with its scenario under it. Once archived, `host-module` validates strict. |
| 6.1 | The configuration reference gives each number's unit, range and default. It gives the labels' table in `owed` and `reserved`, the local keys, and the refusals' words. `site:prose` and `site:build` pass. |
| 6.2 | The agent settings guide, the state reference and the roadmap say `owed` and `reserved`. The state reference gives `reserve` and `keep_free` as the same numbers under the names used before record 0063. |
| 7.1 | Record 0063 holds "The construction's choices", each with the path not taken. It names the construction agent among its deciders for those choices alone. `root:adrs` passes. |

## Requirements and scenarios

`openspec validate a-labels-share-is-owed-and-reserved --strict
--no-interactive` passes, under the shell's OpenSpec, 1.14.3.

- **stream-labels.** "What a label is owed and holds reserved" has five
  scenarios. Each is a labels scenario: S84's owed agent lent and
  reserved agent held empty, one settle ranking again, no room on the
  host, a stream with no bound, and what a label is owed set locally.
  - "A stream whose labels are at fault starts no stage" has six cases,
    each a labels scenario: a pool, `reserve`, `keep_free`, owed past
    the bound, more reserved than owed, and the configuration mended.
    The reader side is `a_labels_reserve_and_keep_free_are_read_unknown`.
  - "Status gives what each label is owed and holds reserved" is covered
    by `the_labels_in_status` and `waiting_on_an_agent_held_reserved`.
    The person's form is covered by `a_streams_labels`.
- **host-queue.** "An agent another label holds reserved" and "A label
  below what it is owed before a place's priority" are host-queue
  scenarios under those names.
- **host-configuration.** "A label's share set locally" and "A label's
  old key set locally" are the two halves of
  `a_local_label_reserve_is_not_taken`.
- **seam-contracts.** "The fake host's sessions, labels and layers" is
  `the_hosts_sessions_labels_and_layers`, with `operation` owed 2 and
  reserved 1.
- **streams.** "A host declares its streams" keeps its two scenarios,
  which still pass. "A host's bound of zero", under "A bound on agents
  is a whole number from 1", is `a_hosts_bound_of_zero_is_refused`.
- **host-module.** `nix:test` covers four scenarios:
  - "A mistyped declared key" and "A bound of zero" under "settings
    names the managed file's tables and keys";
  - "The declared keys" under "Only the keys flume reads are declared";
  - "Each number says its unit, range and default" under "Each declared
    number says its unit, range and default".

The code keeps `reserve` and `keep_free` only where the change means
it to:

- the JSON form's two fields;
- the tests of a refused old key;
- the module's header and its tests of undeclared keys;
- the site's words on the refusal;
- the `kept-free` tag.

## The archive

A throwaway copy of the place's `openspec/` was archived with
`openspec archive a-labels-share-is-owed-and-reserved --yes`. The copy
then passed `openspec validate --specs --strict --no-interactive`, 49
passed and 0 failed. The copy was discarded. The change is archived
on the branch with this report.

## The statement

The change rests on boomdrivedev/blueprints 695c446 ("design(flume):
C.7 gives each number flume reads its unit, range and default, and S84
says reserved"). That commit is on the branch
`design/flume-a-labels-share-is-owed-and-reserved` in the host's clone
`~/Code/github_agentplot/blueprints`, and no remote holds it yet. The
orchestrator pushes it, as the design's section 8 says. The
conformance suite reads the statement at 31f9f1c.
