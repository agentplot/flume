# Tasks

Each group is one session's work for an agent that reads this change
(proposal, design, specs) and its group, and builds on main as group 1
merges it and the groups before it. A group's last task runs the checks
it touched through sluice; every conformance scenario a group adds
cites its clauses as data (106). Tests reach no real account and no
configuration directory of the user's: every git call in a test runs
under the scratch environment `crates/flume/tests/common` gives; every
herdr server a test starts is its own, on a scratch socket and a
scratch session home; and every host configuration a test names is a
scratch file of the test's own.

## 1. Before the code

- [ ] 1.1 Start only once the user has read this design and answered its two questions, agentplot/blueprints main holds design 12's amendment, 199, S106 and S107 among it, and `flume-opens-a-streams-multiplexer` has landed on flume's main. Merge main into the branch. Check that clause 199, S106, S107 and record 0048 are still free on every branch and in every place on the host, renumbering here and in record 0048 where another took them, and fit every delta that names `Stream`, `StreamStatus`, `open_stream`, `flume open` or the local layer's keys to the specs and code as main then holds them; where the user chose otherwise than design 1 or 2, rewrite this change to their answer before any code (design 11; core.19). True when the merge is committed, `git log` shows the multiplexer change landed, and `sluice exec main -- moon run root:changes root:specs root:adrs` passes.

## 2. The seams

- [ ] 2.1 Add to `flume-seams`: `Stream::kinds`, a map of `KindSessions { session, stages, unknown }` by the kind's name, `StageSession { session, unknown }`, `SessionFrom::StreamKind` and `SessionFrom::StreamStage`, serialized `stream-kind` and `stream-stage`, and the stream status's `sessions`, each `NamedSession { name, kinds, stages }` with each value's layer; read every new field with none where written before and write it only where set (design 7, 10; 56, 107, 143, 199; core.14). True when `sluice exec main -- moon run seams:test` passes with the seam-contracts spec's "A start's session from its stream's word for its stage" in `round_trip.rs`, and every existing round trip passing unchanged.
- [ ] 2.2 Have the fake host give a stream's `kinds` as a test scripts them (design 10; 199). True when `sluice exec main -- moon run fakes:test conformance:test` passes with the seam-contracts spec's "The fake host's sessions for a kind and a stage" beside "The fake host's sessions, labels and layers", citing its clauses.
- [ ] 2.3 Run the checks of the projects this group touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test conformance:format conformance:lint conformance:test` passes.

## 3. The host's configuration

- [ ] 3.1 Have `streams_declared` in `crates/flume/src/host/config.rs` take `kinds` among a stream's keys and read `[streams.<stream>.kinds.<kind>]` with `session` and `stages` and each `.stages.<stage>` with `session`, keeping any other key of either table in its `unknown`, and refusing the configuration naming the table and the key for a `session` that is not a string or a `stages` that is not a table of tables; add `streams.*.kinds.*.session` and `streams.*.kinds.*.stages.*.session` to `LOCAL_KEYS`, so a pin of either or of a table holding them passes the local key over; and name the tables in the module's doc (design 2, 4, 6; 143, 199, C.7). True when `sluice exec main -- moon run flume:test` passes with `config.rs` tests of the host-configuration spec's "A kind's session and a stage's set locally", "A kind's table key the local layer cannot set" and "A kind's session pinned", and the stream-sessions spec's "A session that is no word", and every existing configuration test passing unchanged.
- [ ] 3.2 Run the checks of the project this group touched. True when `sluice exec main -- moon run flume:format flume:lint flume:test` passes.

## 4. Where an agent opens

- [ ] 4.1 Have `StreamRoom` keep its stream's `kinds`, and replace `StreamRoom::session` with `session_for(stream, kind, stage)`, answering the session the stream names for the kind's stage, else for the kind, else its own as before, each with its `SessionFrom`, or the refusal naming the table and its other key; have `Queue::session_of` take the place's kind's name and the stage's name, a pane and then the launch's session still first; and have `Engine::claim` refuse the stage on that refusal, as it refuses a stage whose stream's labels are at fault, every other start running (design 3, 4, 5, 10; 4, 27, 70, 107, 143, 182, 199; core.44). True when `sluice exec main -- moon run core:test conformance:test` passes with `engine/sessions.rs` gaining the stream-sessions spec's "A kind's session and a stage's", "A stage's session before its kind's", "Another stream's word", "A kind's session changed between starts", "A launch's session over the stream's word for a stage", "A pane before the stream's word for a stage", "A table's other key", "A stage the kind does not hold" and "Room and shares unchanged", each citing its clauses, S106 and S107 among them, and every existing scenario passing unchanged.
- [ ] 4.2 Run the checks of the projects this group touched. True when `sluice exec main -- moon run core:format core:lint core:test conformance:format conformance:lint conformance:test` passes.

## 5. Status

- [ ] 5.1 Give each stream's `sessions` in `StreamStatus`, each session it names with its kinds and `<kind>.<stage>`s and each one's layer, and a table's other key with its refusal; and print the two new origins and each stream's sessions in the person's form of status as design 7 words them (design 7; 56, 199, C.7; core.14). True when `sluice exec main -- moon run core:test conformance:test flume:test` passes with `engine/sessions.rs` gaining the stream-sessions spec's "The sessions a stream names for its kinds and stages", and `print.rs` unit tests of an agent's session from `stream-kind` and from `stream-stage` and of a stream with two named sessions.
- [ ] 5.2 Run the checks of the projects this group touched. True when `sluice exec main -- moon run core:format core:lint core:test conformance:format conformance:lint conformance:test flume:format flume:lint flume:test` passes.

## 6. The view and the terminal

- [ ] 6.1 Have `Engine::open_stream` take a session: the stream's own where none is named or the name is its own; a session the stream names for a kind or a stage, its window titled by its name and the answer giving its origin and the kinds and stages it is named for; and any other refused naming the stream and its sessions before the terminal or the window is asked anything. Give `flume open` its `--session <name>`, named in its help (design 8; 186, 187, 199). True when `sluice exec main -- moon run core:test conformance:test flume:test` passes with the open's engine scenarios gaining the stream-views spec's three, citing 186, 199 and S107, and `commands` tests of the command-line spec's two over a scratch host.
- [ ] 6.2 Add to `crates/flume/tests/profiles/tools/terminal_herdr.rs` a test of the terminal-profiles spec's "A change's design apart, under herdr": under the pinned herdr, with sessions under a scratch home, a construct agent opened in `work-design` and a code agent in `work`, each sent a prompt, read and closed on its own server, and the place tidied in both (design 9; 107, 137, 138, 190, 199, S106; core.29). True when `sluice exec main -- moon run flume:tools` passes with it, and every existing herdr test passing unchanged.
- [ ] 6.3 Run the checks of the projects this group touched. True when `sluice exec main -- moon run core:format core:lint core:test conformance:format conformance:lint conformance:test flume:format flume:lint flume:test flume:tools` passes.

## 7. The clause list and the site

- [ ] 7.1 Read the suite's clause list at the revision of agentplot/blueprints main that lands design 12's amendment, 199 in Part A and S106 and S107 among the scenarios, naming nothing of this change unbuilt and as unbuilt what another change has not yet built (106; core.19). True when `sluice exec main -- moon run conformance:test conformance:coverage` passes, with 199, S106 and S107 each cited by this change's scenarios.
- [ ] 7.2 Write a stream's sessions for its kinds and stages into the pages design 14 names, `guides/places-and-herdr.mdx`, `guides/agent-settings.md`, `reference/configuration.md`, `reference/commands.md` and `reference/state.md`, to any host's operator, naming no host, team or word of ours (design 14; core.35). True when `sluice exec main -- moon run site:prose site:build` passes.
- [ ] 7.3 Run the checks of the projects this group touched. True when `sluice exec main -- moon run conformance:format conformance:lint conformance:test conformance:coverage site:prose site:build root:format root:specs root:changes` passes.
