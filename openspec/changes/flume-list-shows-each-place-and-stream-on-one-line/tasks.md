# Tasks

Each group is one session's work for an agent that reads this change
(proposal, design, specs) and its group, and builds on main and on the
groups before it. The change is built once the user has accepted record
0049 and the statement's amendment stands on agentplot/blueprints main.
A group's last task runs the checks it touched through sluice. Every
conformance scenario a group adds cites its clauses as data (106). The
engine's scenarios name no tool (core.1). Tests reach no herdr session
of the host's and no configuration directory of the user's: every host
configuration, session home and state directory a test names is a
scratch one of the test's own, and a herdr session a test starts is
named by one letter of the test's own, `w` for a stream's.

## 1. The decision recorded

- [x] 1.1 Add record 0049, proposed, "flume's list gives a line for each place and each stream, read from what flume keeps", quoting the user's words of 2026-11-05, with its deciders, sources (the statement at be85a32 on `design/flume-a-list-of-places-and-streams`; records 0005, 0006, 0031 and 0037), context, nine considered options, outcome, why not the others and consequences (57, 62, 64 to 66, 68, 70, 107, 111, 130, 186, 200, 201, C.5, S108; core.19). True when `sluice exec main -- moon run root:adrs root:format` passes.
- [x] 1.2 Draw core.52 in `docs/architecture/core.md` under "Settling, surfaces and state", after core.15, with its details citing record 0049, and have `site/src/content/docs/architecture/rules.md` mirror it as written (core.19, core.35). True when `sluice exec main -- moon run root:format site:prose site:build` passes.
- [x] 1.3 Run the checks of the projects this group touched. True when `sluice exec main -- moon run root:format root:adrs root:changes site:prose site:build` passes.

## 2. The seams and their fakes

- [x] 2.1 Give `flume-seams` design 5's `SessionRuns` and `Terminal::runs`, with no default, re-exported. Have every `Terminal` implement it: the fake as design 10 has it, the printed command answering `NoPart` as design 6 has it, herdr answering `Unread` with `not yet built` until 4.1, and every stand-in terminal a test or a harness holds answering `NotRunning` for the part named. Add to `round_trip.rs` the seam-contracts spec's scenario of each answer (201; core.2). True when `sluice exec main -- moon run seams:test fakes:test conformance:test core:test flume:test` passes, every existing test passing unchanged.
- [x] 2.2 Give the fake terminal `runs_as(part, SessionRuns)` and its `runs <part>` log line, its own part named `own`, and add to `crates/flume-fakes/tests/fakes.rs` the seam-fakes spec's scenario (106, 201). True when `sluice exec main -- moon run fakes:test` passes.
- [ ] 2.3 Add to `flume-conformance`'s `seams/terminal.rs` runs' scenario over the fake, as the seam-contracts spec has it, citing 201. True when `sluice exec main -- moon run conformance:test` passes.
- [ ] 2.4 Run the checks of the projects this group touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test conformance:format conformance:lint conformance:test core:format core:lint core:test flume:format flume:lint flume:test` passes.

## 3. The core's list

- [ ] 3.1 Add `crates/flume-core/src/list.rs` with `Engine::list` as designs 1 to 4 have it, and `Answer::List` with `Listed`, `WaitsOn` and `StreamListed`. Make `line.rs`'s `stage_at` and `state_words` `pub(crate)` and give the list's state from them. Read each document's held state and each live start's asking mark alone, and each declared stream's terminal once by runs. No lock is taken, no document written, nothing settled, and neither `bound`, `facts`, `read_agent`, `place_clues` nor `host_status` called (56, 57, 62, 64 to 66, 68, 70, 90, 107, 111, 130, 139, 186, 190, 200, 201; core.14, core.16, core.17, core.52). True when `sluice exec main -- moon run core:test conformance:test` passes, `list.rs`'s unit tests holding the kept words' order and the places' order, and `engine/list.rs` holding the-list spec's scenarios over the fakes, the one S108 cites named `s108_the_list_of_fifty_places`.
- [ ] 3.2 Where `where-an-agent-opens-is-given-by-kind-and-by-stage` has landed on main when this group is built, give each session a stream names for a kind or a stage a line of its own under the stream's, `<stream> · session <name> for <kind>[.<stage>] · <runs>`, by one runs each, as design 9 has it, with a scenario in `engine/list.rs` citing 199 and 200. Where it has not landed, leave its sessions to its build and name that in the group's summary (186, 199, 200). True when `sluice exec main -- moon run core:test conformance:test` passes.
- [ ] 3.3 Add `list`'s fields to `engine/answers.rs`'s field list, and the-list spec's JSON scenario, a list written and read back (60, 61, 200). True when `sluice exec main -- moon run conformance:test` passes, the field list failing naming the answer and the field where one is dropped.
- [ ] 3.4 Run the checks of the projects this group touched. True when `sluice exec main -- moon run core:format core:lint core:test conformance:format conformance:lint conformance:test` passes.

## 4. The profiles and the command

- [ ] 4.1 Give the herdr profile runs as design 6 has it: one `herdr session list --json` through `run`, bounded at one second, found by `name`, its own `flume`, or by `socket_path` for an own server held by its socket alone, starting no server, and name the call in the module's header beside attach (201, C.5; core.13, core.29). True when `sluice exec main -- moon run flume:tools` passes, `tests/profiles/tools/terminal_herdr.rs` holding the terminal-profiles spec's four herdr scenarios, the bound's over a stand-in herdr of the test's own.
- [ ] 4.2 Hold the printed command's runs in `tests/profiles/terminal_printed.rs` as the terminal-profiles spec has it (187, 201). True when `sluice exec main -- moon run flume:test` passes.
- [ ] 4.3 Add `flume list` to `commands/mod.rs`, needing no repository and settling nothing, with its help as the command-line spec has it, and `print.rs`'s person's form as design 7 has it, from the answer's own lines (57, 60, 61, 200; core.14, core.35). True when `sluice exec main -- moon run flume:test` passes, `print.rs`'s tests holding the form with places in two repositories, a host with no places and a stream of each runs answer, and `tests/commands.rs` holding the command-line spec's five new scenarios, the fifty places' within a second among them.
- [ ] 4.4 Run the checks of the projects this group touched. True when `sluice exec main -- moon run flume:format flume:lint flume:test flume:tools` passes.

## 5. The site and the clause list

- [ ] 5.1 Give `site/src/content/docs/reference/commands.md` `flume list` in its table, among the commands that only read under "Settling first", and a section of its own with design 7's example; and `reference/state.md` the list's fields, to any host's operator (61, 200; core.35). True when `sluice exec main -- moon run site:prose site:build` passes.
- [ ] 5.2 Have `crates/flume-conformance/src/clauses.rs` read the revision of blueprints main that lands record 0049's amendment, 200 in Part A, 201 in Part B and S108 among the scenarios, as the clause-coverage spec has it (106; core.19). True when `sluice exec main -- moon run conformance:test conformance:coverage` passes, 200, 201 and S108 each cited by this change's scenarios.
- [ ] 5.3 Run the checks of the projects this change touched. True when `sluice exec main -- moon run root:format root:adrs root:specs root:changes seams:test fakes:test core:test conformance:test conformance:coverage flume:test flume:tools site:prose site:build` passes.
