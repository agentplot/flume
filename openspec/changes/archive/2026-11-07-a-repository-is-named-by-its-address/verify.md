# Verify

The change `a-repository-is-named-by-its-address`, on
`change/a-repository-is-named-by-its-address` at 2cddc91, cut from
`bolt/flume-names-each-repository-by-its-address` at 4b2f3ec. Verified
2026-11-07.

## The whole checks

`devenv shell -- sluice exec bolt/flume-names-each-repository-by-its-address`,
run once, exited 0: sluice ran the checks 2cddc91 affects against the
base, 22 tasks completed and 21 checks passed, their results recorded
under their keys for 2cddc91. The base is a bolt, so its `check-fast`
layer was run, and `check-tools` waits for the bolt's merge onto main.

| project | tasks |
|---|---|
| `seams` | format, lint, test (98 passed) |
| `core` | format, lint, test (44 passed, both `address::` tests among them) |
| `fakes` | format, lint, test (64 passed) |
| `conformance` | format, lint, test, coverage (every clause of Parts A and B, I1 to I8 and S1 to S119 at blueprints 9c060ab cited by the suite's 869 scenarios) |
| `flume` | format, lint, test (every target passed, the six `by_address_` tests, `a_rewritten_origin_is_read_as_configured`, `the_history_scenarios_pass_against_git`, `a_launch_takes_its_stream_by_its_repositorys_address` and `a_launch_given_an_address_finds_no_clone` among them) |
| `plugin-claude` | test |
| `site` | install, prose, build |
| `root` | format, changes |

moon warned that it could not write its output archives and shared
cache blobs, under the place's `.moon/cache/outputs` and the shared
moon home. That is caching only, and no check failed because of it.

`openspec validate a-repository-is-named-by-its-address --strict`:
valid.

## Tasks

All 11 tasks are ticked, and the work is on the branch as each task
names it.

- **1.1** `crates/flume-core/src/address.rs`, `of_origin`, declared in
  `lib.rs`, takes the scp-like form and the six schemes, drops a user,
  password and port, a trailing `/` and one `.git`, and gives exactly
  two segments with their case kept. Its two unit tests cover every URL
  in the spec's two scenarios and more.
- **2.1** `History::origin_url` is documented as design 2 says.
  `GitHistory` reads it by `git config --get remote.origin.url` through
  `Git::at`. `FakeGit` has `set_origin_url` and `remove_origin_url`. The
  `HistoryHarness` has `origin_named` and `origin_removed` over git and
  over the fake, and the history scenario has been added.
- **2.2** `a_rewritten_origin_is_read_as_configured` checks that git
  rewrites the URL it reaches and that `origin_url` still reads the URL
  as configured.
- **3.1** `RepositoryName` with `of_folder`. `Host::stream_for`,
  `label_for` and `settings` take it. `HostConfig::table_for` gives the
  address only where `repos` holds that key. `RealHost::settings` and
  `FakeHost` read every repository-keyed map under one key. The module
  comment is updated, and there are six `by_address_` tests.
- **4.1** `PlaceRecord.address` uses serde default and is skipped where
  none. `repository_name()` is built from it. `a_held_state` round-trips
  `acme/acme`, and a document with no `address` reads and writes none.
- **4.2** `Engine::repository_name` reads the URL, takes its address and
  drops the URL. A launch reads it before `stream_of` and keeps it in
  the record. `layers` and `label_of` pass `place.repository_name()`.
  The refusal for an undeclared stream names the address where there is
  one. `PlaceStatus.address` is skipped where none. There are three
  launching scenarios.
- **4.3** `a_launch_takes_its_stream_by_its_repositorys_address` and
  `a_launch_given_an_address_finds_no_clone` are in
  `crates/flume/tests/commands.rs`.
- **5.1** Rule core.55 comes after core.45 under "Settling, surfaces and
  state", worded as in design 8 and carrying the line "Drawn 2026-11-07
  from record 0055". Its details come after core.45's, with source
  clauses 4, 8, 32, 56, 143, 180, C.7 and record 0055.
- **5.2** `reference/configuration.md` gives an example by address next
  to one by folder. Its new section "A repository's name" says the
  address's table comes first and whole, and `stream`, `label`, the
  kind's agents and the repository's agents point to it.
  `guides/agent-settings.md` names repositories by address. Both pages
  speak to any host's operator (core.35).

## Specs

Every requirement and scenario of the three delta specs is met, and
each scenario has the test named against it.

### history-and-host

- **The repository's origin read through git.** The URL is read as
  configured, through `Git::at`, and never rewritten.
  - "The origin's URL as written": the history scenario run over git
    and over the fake. It gives the URL with `user:token` as written,
    and none once origin is removed.
  - "A rewritten origin read as configured":
    `a_rewritten_origin_is_read_as_configured`.

### host-configuration

- **A repository's tables are named by its address, else its folder.**
  - "The address's table": `by_address_the_address_table_gives_stream_label_and_settings`
    checks the stream from `host-repository`, the label from
    `host-repository-kind`, the kind's model `haiku` and every kind's
    `keep_open` `2h`.
  - "A repository with no address":
    `by_address_a_repository_with_no_address_takes_its_folders_table`.
  - "Matched exactly": `by_address_a_key_is_matched_exactly`. Neither
    `Acme/acme` nor `acme/*` is taken.
- **The address's table stands whole, else the folder's.**
  - "A folder's table where no table names the address":
    `by_address_a_folder_table_reads_where_no_table_names_the_address`.
  - "The address's table whole over the folder's":
    `by_address_the_address_table_stands_whole_over_the_folders`. The
    stream is `work`, and the folder's kind stream and label are not
    read.
- **The local layer finds an address's table by its key.**
  - "An address's stream set locally":
    `by_address_a_stream_is_set_locally_and_pinned`. The local layer
    gives `personal`. With the pin, the managed layer gives `work`, and
    `passed_over` names the key.

### launching

- **A launch reads its repository's address from its clone's origin.**
  - "Each form of a forge origin":
    `address::tests::an_origin_on_a_forge_gives_its_address`.
  - "No forge origin": `address::tests::an_origin_on_no_forge_gives_none`.
- **A place keeps its address and its starts take its tables.**
  - "Its starts take the address's tables": the engine scenario
    `its_starts_take_the_addresss_tables`, with the model `haiku` from
    `host-repository` and the label `construction` from
    `host-repository-kind`.
  - "A place held with no address": the engine scenario
    `a_place_held_with_no_address`. The origin is set after the launch,
    and the retry's agent takes `opus` from the folder's table.
  - "A launch over git": `a_launch_takes_its_stream_by_its_repositorys_address`.
    The place runs in `work` from `host-repository`, and
    `status --json` gives `address` `acme/acme` and does not give the
    origin's URL.
- **Status gives a place's address and nothing holds its origin's URL.**
  - "The address kept and given, the URL never": the engine scenario
    `the_address_kept_and_given`. Status gives `acme/acme` beside
    `/r`, and neither the held state nor status holds `token`.
- **flume finds no clone by its address.**
  - "No clone found by its address": `a_launch_given_an_address_finds_no_clone`.
    The refusal names the canonical `<folder>/acme/acme`, and no folder,
    place or held state is made.

## Notes

- Task 2.1 asks for the history scenario to cite 143 and C.7, but it
  cites 143 alone. No scenario in the suite cites a section of Part C,
  and the coverage check counts numbered clauses, so this follows the
  suite's practice and changes nothing the spec asks.
- No rule of `docs/architecture/core.md` is departed from. The URL
  appears only in the history's answer and in `Engine::repository_name`,
  where it is dropped (core.30). The JSON form gains `address` and
  loses no field (core.14). The core names git alone (core.1).
