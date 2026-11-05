# Tasks

The build starts once the user has approved this change and
agentplot/blueprints main states design 1's clauses, as approved
(core.19). Each group is one session's work for an agent that reads
this change (proposal, design, specs) and its group, and builds on main
and on the groups before it. A group's last task runs the checks it
touched through sluice. Every conformance scenario a group adds cites
its clauses as data (106). The engine's scenarios name no program and
no host's reader: the fake's `agent` and its accounts stand for any
(core.1). Tests reach no real account, no configuration directory of
the user's and no maker's usage report. Every reading, host
configuration, state directory and `CLAUDE_CONFIG_DIR` a test names is
a scratch one of the test's own, and every git call in a test runs
under the scratch environment `crates/flume/tests/common` gives. An
agent in a test is the stand-in (`crates/flume/tests/profiles/stand-in/`)
or the fake.

## 1. The decision recorded

- [x] 1.1 Mark record 0035 accepted on the user's words approving this change, with the day, and name in its Sources and Consequences the revision of agentplot/blueprints main that states 184, 185 and S86 to S88. Where the user's approval changed a clause, make the record's outcome say what they approved; remove and reword nothing else (core.19). True when `sluice exec main -- moon run root:adrs root:format` passes.
- [ ] 1.2 Amend `docs/architecture/core.md`'s core.32 as design 10 has it: its rule, its note naming record 0035, and its details, each citing 145, 147, 184, 185 or S86 to S88 where it bears; and core.30's detail on `~` and `$VAR`, an account's `usage` beside its `env`, citing 184 and record 0035. Have `site/src/content/docs/architecture/rules.md` mirror core.32's rule as written (core.19, core.35). True when `sluice exec main -- moon run root:format site:prose site:build` passes.
- [ ] 1.3 Have the suite's clause list read the statement at the revision 1.1 names: `REVISION` that revision, `SCENARIOS` the last scenario it holds, and Parts A and B every clause it holds, 184 and 185 in Part A. Name 184, 185 and S86 to S88 unbuilt by this change, a constant beside the others, until the scenarios that cite them land, and every other clause of that revision another change has not yet built unbuilt by that change (clause-coverage spec; core.19). True when `sluice exec main -- moon run conformance:test conformance:coverage` passes.
- [ ] 1.4 Run the checks of the projects this group touched. True when `sluice exec main -- moon run root:format root:adrs site:prose site:build conformance:test conformance:coverage` passes.

## 2. The reading

- [ ] 2.1 Give `flume-seams` design 3's types: `Strategy::MostLeft`, spelled `most-left`; `Pool`'s `threshold` and `Account`'s `usage`, each skipped where none; `Usage` and `Window`, `used` in hundredths; `MoveWhy::Threshold` and `AccountMove`'s `window` and `until`, each skipped where none; and `Host::usage`. Bring every place that builds a `Pool`, an `Account` or an `AccountMove` to the new shape, the fakes', the engine's and the tests' among them. Add to `round_trip.rs` the seam-contracts spec's two scenarios of the types (147, 184, 185; core.14). True when `sluice exec main -- moon run seams:test core:test fakes:test conformance:test flume:test` passes, every existing test passing unchanged.
- [ ] 2.2 Give the fake host a reading or a refusal per path, `set_usage`, answering that the file is missing for any other path and logging each reading asked, as design 4 has it. Add to the conformance suite the seam-contracts spec's scenario of the fake host's reading and the seam-fakes spec's scripted reading (106, 184). True when `sluice exec main -- moon run fakes:test conformance:test` passes.
- [ ] 2.3 Add the real host's reading, `RealHost::usage`, in a module of `crates/flume/src/host/` that parses C.7's form as design 4 has it: `serde_json`, `chrono`'s RFC 3339 times with their offset, every unknown key ignored, and each refusal in words that name no path. Its unit tests hold the store-profile spec's three scenarios of a reading read from its file, and `crates/flume/tests/host.rs` holds `RealHost::usage` over a scratch file and a missing one (184, C.7). True when `sluice exec main -- moon run flume:test` passes.
- [ ] 2.4 Have `pools_declared` read a pool's `threshold` and `most-left` and an account's `usage`, and refuse the values design 7 names, naming the key, the strategy's words naming all three. Its tests hold the store-profile spec's three scenarios of the configuration, C.7's example as design 1 amends it among them. Where the host's configuration is read in two layers, the keys are the managed layer's alone, and the local layer's keys gain nothing (144, 184, 185, C.7). True when `sluice exec main -- moon run flume:test` passes.
- [ ] 2.5 Run the checks of the projects this group touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test core:test conformance:format conformance:lint conformance:test flume:format flume:lint flume:test` passes.

## 3. The choice and the threshold

- [ ] 3.1 Give `AccountRoom` its `reading`, read in `Engine::queue` under the room lock with the account's `usage` expanded against the login environment, and from it, at the host's clock, a window's use now, freshness, the threshold that holds the account with its window and end, and what it has left, `THRESHOLD` 90 and `FRESH` 900 seconds, as design 5 has them. Have `short` answer `Threshold` after `Spent` and before `Full`, and `Usable::None` give the soonest back of the pool's accounts of the program. Add to `engine/account_pools.rs` the account-pools spec's scenarios of an account's reading, an account past its threshold, the soonest time and a spent pause, S87's and S88's among them, named `s87_past_ninety_percent` and `s88_every_account_past_its_threshold_or_spent` (31, 145, 146, 147, 153, 184, 185, S87, S88). True when `sluice exec main -- moon run core:test conformance:test conformance:coverage` passes, every existing scenario passing unchanged.
- [ ] 3.2 Give `PoolRoom::strategy` `Strategy::MostLeft` as design 6 has it. Add to `engine/account_pools.rs` the account-pools spec's scenarios of a most-left pool, S86's named `s86_four_places_of_four_intents` (145, 147, 184, S86). True when `sluice exec main -- moon run core:test conformance:test conformance:coverage` passes, every existing scenario passing unchanged.
- [ ] 3.3 Have `choose_account` and `stage_moved` give a move off an account past its threshold `why` `threshold` with the window that held it and its end, as design 6 has it, and the intent's affinity move as it moves off a spent or full account, never back (147, 185). True when `sluice exec main -- moon run core:test conformance:test` passes, S87's scenario reading the intent's move with `5-hour` and 17:00.
- [ ] 3.4 Run the checks of the projects this group touched. True when `sluice exec main -- moon run core:format core:lint core:test conformance:format conformance:lint conformance:test conformance:coverage` passes.

## 4. Status

- [ ] 4.1 Give `PoolStatus` its `threshold` and `AccountStatus` its `reading` and `past`, as design 8 has them, `used` given as a percentage in the JSON form, and `RoomWanted::Pool`'s `until` the soonest back. Add to `engine/place_status.rs` the place-status spec's scenarios of a pool with readings, an account past its threshold, and a move past a threshold and one recorded before, in the JSON form (56, 145, 184, 185; core.14, core.30). True when `sluice exec main -- moon run core:test conformance:test` passes, the scenario of a pool with a spent account passing unchanged.
- [ ] 4.2 Have the person's form print a pool's threshold, each account's reading, fresh, stale or unread, an account past its threshold, and a move off one, as design 8 has them, and no reading's path. Add `print.rs` unit tests of each line (56, 185). True when `sluice exec main -- moon run flume:test` passes.
- [ ] 4.3 Run the checks of the projects this group touched. True when `sluice exec main -- moon run core:format core:lint core:test conformance:test flume:format flume:lint flume:test` passes.

## 5. The walkthrough

- [ ] 5.1 Add to `crates/flume/tests/profiles/tools/walkthroughs.rs` design 11's walkthrough under the printed command: a most-left pool of two of the stand-in's accounts, each `usage` a scratch file written whole by the test; a fix starting on the account whose reading leaves more; its reading then written past ninety percent and a second fix starting on the other; and `flume status --json` giving both readings and the first account past its threshold, with no path (145, 184, 185). True when `sluice exec main -- moon run flume:tools` passes with it, and every existing walkthrough passes unchanged.
- [ ] 5.2 Run the checks of the project this group touched. True when `sluice exec main -- moon run flume:format flume:lint flume:test flume:tools` passes.

## 6. The site

- [ ] 6.1 Write design 12 into `site/src/content/docs/guides/agent-settings.md`, `reference/configuration.md` and `reference/state.md`: `most-left` in the strategies' table; "An account's reading", with C.7's form, how often a host writes it and that flume reads and never writes it and fetches nothing; "A threshold"; status's pool threshold and account reading; `strategy`'s three values, `threshold` and `usage` with their refusals; and the reading among what flume reads on the host and the new status fields. Write it to any host's operator, naming no host, team, tool or word of ours (core.35). True when `sluice exec main -- moon run site:prose site:build` passes.
- [ ] 6.2 Run the checks of the project this group touched. True when `sluice exec main -- moon run site:prose site:build root:format` passes.

## 7. Coverage and the gate

- [ ] 7.1 Have the suite's clause list name nothing of this change unbuilt, its constant gone, and 184, 185 and S86 to S88 cited by this change's scenarios, as the clause-coverage spec's scenarios have them (core.19). True when `sluice exec main -- moon run conformance:test conformance:coverage` passes.
- [ ] 7.2 Run the checks of the projects this change touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test core:format core:lint core:test conformance:format conformance:lint conformance:test conformance:coverage flume:format flume:lint flume:test flume:tools site:prose site:build root:format root:adrs root:changes root:specs` passes.
