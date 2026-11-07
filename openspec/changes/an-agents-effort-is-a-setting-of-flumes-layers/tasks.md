# Tasks

Each group is one session's work for an agent that reads this change
(proposal, design, specs) and its group, and builds on main, which
holds `an-agents-program-settings-come-from-flumes-layers` since #128,
and on the groups before it. A group's last task runs the checks it touched
through sluice; every conformance scenario a group adds cites its
clauses as data (106). Tests reach no real account and no
configuration directory of the user's: every git call in a test runs
under the scratch environment `crates/flume/tests/common` gives; an
agent in a test is the stand-in (`crates/flume/tests/profiles/stand-in/`)
or the fake, but in the live walkthrough of group 5; and every
`CLAUDE_CONFIG_DIR`, Claude Code settings file and host configuration a
test names is a scratch directory or file of the test's own.

## 1. Fitted to main

- [x] 1.1 Once `an-agents-program-settings-come-from-flumes-layers` has landed on main, merge main into this change, and fit the change to it: each delta that names the host's tables for every agent, a stream and a repository, or Claude Code's reserved keys, to the specs as they then stand; clause-coverage's list to the revision of blueprints main then holding 197 and S103, or this change's branch; and record 0044, core.30 and core.39 to what main then holds, clause 197, S103 and record 0044 renumbered where another change took them first (design 12, 13; core.19). Should that change have been dropped, read the effort in the layers main holds and drop design 6's reserved key and its scenarios. True when `sluice exec main -- moon run root:changes root:specs root:adrs root:format` passes, and `openspec validate an-agents-effort-is-a-setting-of-flumes-layers --strict` with it.

## 2. An effort in the seams and every layer

- [x] 2.1 Add `Settings::effort`, an optional string taken whole by `Settings::under` and read with none from a held state written before it, and `Agent::effort`, refusing every word by default in design 5's words, and `Agent::effort_variable`, none by default, to `flume-seams`. Have `Agent::command` pass `settings.effort` as the profile's way gives it. Have the fake agent script the words it takes and its variable, its command passing `--effort <word>` and its log naming each `effort` asked. Add the seam's scenarios to `seams/agent.rs` below the seam's marker, generic over the trait (design 1, 5; 88, 197). True when `sluice exec main -- moon run seams:test fakes:test conformance:test flume:test` passes with the seam-contracts spec's held state with an effort in `round_trip.rs`, its two agent scenarios, and the seam-fakes spec's scripted effort in `fakes.rs`.
- [x] 2.2 Read `effort` beside `model` in the kinds profile's `map_agent`, which reads a repository's `[agents]` for every kind as well, a string kept as written and any other value a problem naming `<table>.effort`, and name it in `map_agent`'s doc among the settings, so it is no longer the runner's (design 2, 3; 23, 158, 197, C.2; core.4). True when `sluice exec main -- moon run flume:test` passes with `kinds_weir.rs` tests of the kinds-profile spec's three added scenarios over scratch repositories.
- [x] 2.3 Read `effort` in every agents table of the host's configuration in `crates/flume/src/host/config.rs`, `[agents]`, `[agents.streams.<stream>]` and `[repos."<repository>".agents]` among them, a key the local layer may set, refusing any value but a string naming the table and `effort`, and name it in the module's doc (design 3; 32, 197, C.7; record 0032). True when `sluice exec main -- moon run flume:test` passes with a `config.rs` test of each of the store-profile spec's three scenarios, and every existing configuration test passing unchanged.
- [x] 2.4 Give `--agent [<stage>.]effort=<word>` in `commands/launch.rs`, naming `effort` among the settings in its help and every refusal (design 3; 7, 197). True when `sluice exec main -- moon run flume:test` passes with `commands/launch.rs` unit tests of the command-line spec's two scenarios' words, and a `commands.rs` test over a scratch repository of its effort at the launch, read from the place's held settings.
- [x] 2.5 Fold `effort` in `Engine::resolve` as the model is, taken whole from the first layer that gives it, a stage's own first, with `from["effort"]` its layer. Have `Engine::resolution` ask the agent's profile of it and refuse in design 8's words, `Engine::launch` ask it of every stage before anything is made, and the claim pass it in the settings `Agent::command` is given. With no effort in any layer, ask nothing and pass nothing (design 3, 4, 5, 8; 4, 8, 21, 27, 32, 197; core.8, core.30). True when `sluice exec main -- moon run core:test conformance:test` passes with a new `engine/agent_effort.rs` holding each of the agent-effort spec's scenarios, each citing its clauses, and every existing scenario passing unchanged.
- [x] 2.6 Run the checks of the projects this group touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test core:format core:lint core:test conformance:format conformance:lint conformance:test flume:format flume:lint flume:test` passes.

## 3. Claude Code's effort

- [x] 3.1 Add `crates/flume/src/profiles/agent/claude/effort.rs`: the release's five words, the refusal naming the word, `claude` and the five, `CLAUDE_CODE_EFFORT_LEVEL` as the variable read over it, and `fragments()`, the option with its words, the list of five and the variable's reading, each as 2.1.296 holds it. Give the profile `effort` and `effort_variable` from it, and have its command pass `--effort <word>` after `--model` where the settings give one (design 5, 6, 7; 88, 197, C.4; core.2, core.39). True when `sluice exec main -- moon run flume:test` passes with `agent_claude.rs` tests of the agent-profile spec's command and word scenarios.
- [x] 3.2 Add `effortLevel` to `claude/settings.rs`'s reserved keys with the owner flume's `effort` and its declaration in the release's settings schema past its name, `()=>U(["low","medium","high","xhigh"]).optional().catch(void 0).describe("Persisted effort level for supported models.")` through the opening of its description, as its neighbours are (design 6; 195, 197, C.4). True when `sluice exec main -- moon run core:test conformance:test flume:test` passes with the agent-profile spec's two `effortLevel` scenarios, in `agent_claude.rs` and `engine/own_settings.rs`.
- [x] 3.3 Have the `check-tools` test in `crates/flume/tests/profiles/tools/claude_words.rs` require each fragment of `effort::fragments()` byte for byte in the pinned Claude Code, the file the `claude` wrapper from devenv.nix execs, naming any it does not find (design 6; C.4; core.22). True when `sluice exec main -- moon run flume:tools` passes with it.
- [x] 3.4 Run the checks of the project this group touched. True when `sluice exec main -- moon run flume:format flume:lint flume:test flume:tools` passes.

## 4. The effort in status

- [x] 4.1 Give `AgentStatus` its `effort`, the word, its layer, the value passed, `read_back: false` and `outranked_by` naming the account's variable where design 7 finds it, only where a layer gave one, in the JSON form, and print it on the agent's line of the person's form (design 7, 9; 56, 197; core.14, core.30). True when `sluice exec main -- moon run core:test conformance:test flume:test` passes with `engine/place_status.rs` gaining the place-status spec's three scenarios and `print.rs` unit tests of them, no value of the variable in either form.
- [x] 4.2 Run the checks of the projects this group touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test core:format core:lint core:test conformance:format conformance:lint conformance:test flume:format flume:lint flume:test` passes.

## 5. The walkthroughs

- [x] 5.1 Have the stand-in read `--effort <word>` and give the word it began at to its log, and, for a word outside the five, write the pinned release's warning and run on at its default, as that release does. Add to `crates/flume/tests/profiles/tools/walkthroughs.rs` S103 under the printed command: a repository's kind `bump` whose `[kinds.bump.agents]` gives `effort = "xhigh"`, launched with `--agent review.effort=max`, its code agent's stand-in started with `--effort xhigh`, its review's with `--effort max`, status naming each with its layer, and the place landing (design 6, 9; S103; 32, 197). True when `sluice exec main -- moon run flume:tools` passes with it, and every existing walkthrough passing unchanged.
- [x] 5.2 Have the live fix walkthrough under herdr with the pinned Claude Code launch with `--agent effort=low`, its started ack taken and the place landing, status naming `low` from `launch` (design 6; S103; 197, C.4). True when `sluice exec main -- moon run flume:lint` passes, which builds the live harness under `--all-features`, and `FLUME_LIVE_CLAUDE_CODE_OAUTH_TOKEN=<token> devenv shell -- moon run flume:live` passes with it and every other live walkthrough, run by the operator by hand outside the gate as `CLAUDE.md` says, the token a test account's from `claude setup-token`.
- [x] 5.3 Run the checks of the project this group touched. True when `sluice exec main -- moon run flume:format flume:lint flume:test flume:tools` passes.

## 6. The site

- [x] 6.1 Write the effort into `site/src/content/docs/guides/agent-settings.md` beside the model, with S103's kind and launch and the advice to give an effort in the layers rather than an account's environment; `effort` in an agents table into `guides/kinds.mdx`; `--effort` and `effortLevel`'s owner into `guides/claude-code.mdx`; `effort` in the host's tables into `reference/configuration.md`; `--agent [<stage>.]effort=<word>` into `reference/commands.md`; and the agent's `effort` into `reference/state.md`. Write it all to any host's operator, naming no host, team or word of ours (design 3 to 9; core.35). True when `sluice exec main -- moon run site:prose site:build` passes.
- [x] 6.2 Run the checks of the project this group touched. True when `sluice exec main -- moon run site:prose site:build root:format` passes.

## 7. Coverage and the gate

- [ ] 7.1 Have the suite's clause list read the statement at the revision task 1.1 fitted, `c94c496` or a later one of blueprints main, holding 197 and S103, and cite 197 and S103 from this change's scenarios, so neither is named unbuilt (clause-coverage spec; 106). True when `sluice exec main -- moon run conformance:test conformance:coverage` passes.
- [ ] 7.2 Run the gate's checks over the change as it stands. True when `sluice exec main` passes.
