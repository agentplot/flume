# Tasks

Each group is one session's work for an agent that reads this change
(proposal, design, specs) and its group, and builds on main and on the
groups before it. A group's last task runs the checks it touched
through sluice. Every conformance scenario a group adds cites its
clauses as data (106).

Group 1 is built once the user says yes to the design, record 0039 and
its two other Open Questions. The groups after it are built once
flume's statement is amended on agentplot/blueprints main as design 11.1
words it. An answer of the user's that changes the design is written
into the design, the specs and these tasks before group 2 starts.

Tests reach no real account, no repository of the user's, no person's
queue and no network. Every git call in a test runs under the scratch
environment `crates/flume/tests/common` gives. An agent in a test is the
stand-in (`crates/flume/tests/profiles/stand-in/`) or the fake. Every
host configuration and state directory a test names is a scratch one of
the test's own. The engine's scenarios name no program, no terminal and
no queue (core.1).

## 1. The decision recorded

- [ ] 1.1 Accept record 0039. Its status reads accepted by the user, with
  their words and the date, and its deciders name their answers to the
  three Open Questions. Its consequences give the agentplot/blueprints
  revision that holds the statement's amendment (4, 7, 8, 42, 43, 49,
  56, 130, 132, 190, C.4, C.5, C.7, S91 to S94; core.19). True when
  `sluice exec main -- moon run root:adrs root:format` passes.
- [ ] 1.2 Add to records 0009, 0013 and 0014 an amendment line each,
  naming record 0039 and what it changes of that record. 0009: a
  caller's start also sees what waits on it, and a notice names the
  brief and the key. 0013: a run is named by its whole address, and a
  session may serve an address. 0014: the host's answerer is a
  recipient of every stall. Remove and reword nothing the records wrote
  (core.19). True when `sluice exec main -- moon run root:adrs
  root:format` passes.
- [ ] 1.3 Amend `docs/architecture/core.md`:
  - core.24's rule: a stalled place is said to each recipient and to the
    host's answerer.
  - core.25's details: a caller's own session collecting at its start
    by the plugin, and the inbox's `waiting` marking nothing.
  - core.28's rule and details: a run named by its whole address, a
    session serving an address, and a launch naming an address that
    wakes nothing refused.
  - Draw a rule under "Settling, surfaces and state", the next free id
    when this lands (core.49 as the branches stand on 2026-11-05),
    citing 4, 7, 42, 49, 130, 190 and record 0039: "Every place has a
    recipient that collects its ends or has a run woken for them, the
    host's answerer is delivered every stall besides the place's
    recipients, and an answer naming a stop-short's key is taken only
    while the place waits on that stop-short."
    - Its details rule out: a launch taken with no recipient on a host
      that names no answerer; a stall the answerer is not delivered; a
      second answer to one key taken; and flume making a card, or
      deciding which question is a person's (69, 72).
    - They do not rule out: a retry with no key, taken as before; a
      launch naming a recipient it collects as itself.

  Have `site/src/content/docs/architecture/rules.md` mirror the new rule
  and core.28 as written (core.19, core.35). True when `sluice exec main
  -- moon run root:format site:prose site:build` passes.
- [ ] 1.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run root:format root:adrs site:prose
  site:build` passes.

## 2. The host's answerer and a launch's recipients

- [ ] 2.1 Give `flume-seams`' `Host` the answerer for a stream, with the
  key it came from, and the fakes' host a configured one. Have the host's
  configuration reader in `crates/flume` read `[host] answerer` and
  `[streams.<name>] answerer`, as the configuration stands when asked,
  and refuse a value that is no address, naming its key (design 2;
  143, 190, C.7). True when `sluice exec main -- moon run seams:test
  fakes:test flume:test` passes, with the history-and-host spec's three
  scenarios.
- [ ] 2.2 In `launch.rs`, `make_place`:
  - Have a launch's recipients end at the answerer, after
    `$FLUME_RECIPIENT` and the terminal's session, with where each came
    from kept in held state.
  - Refuse, before anything is made, a launch that finds none of the
    four, and one whose recipients hold an address that `[roles]` names
    no run for and no session serves, naming each as the launching spec
    has it (design 1, 3; 4, 7, 132, 190; core.8).

  True when `sluice exec main -- moon run core:test conformance:test
  flume:test` passes. `engine/launching.rs` holds the launching spec's
  scenarios, the address a session serves among them once group 3 gives
  serving, and every existing scenario passes. The ones that launched
  from no session with no recipient name one now.
- [ ] 2.3 Have status give each place's recipients with where each came
  from, and the host's and each stream's answerer, in the JSON form, as
  fields added (design 8; 56; core.14). True when `sluice exec main --
  moon run core:test conformance:test flume:test` passes, with the
  place-status spec's scenario of recipients and their sources in
  `engine/place_status.rs` and `print.rs`'s tests.
- [ ] 2.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test fakes:format fakes:lint fakes:test
  conformance:format conformance:lint conformance:test flume:format
  flume:lint flume:test` passes.

## 3. Serving an address, and a run named by its whole address

- [ ] 3.1 Have `role_of` give the whole address as a run's name,
  `<partition>-<role>`. Have `run_of` find a run by its role field first,
  so a run an earlier flume named by its role alone is found (design 4;
  3, 132). True when `sluice exec main -- moon run seams:test core:test
  conformance:test` passes. `engine/role_waking.rs` holds the
  role-waking spec's scenarios of the run's session, the two partitions'
  planners, named `s94_two_partitions_planners`, and the run an earlier
  flume named.
- [ ] 3.2 Add serving:
  - The store seam's document of an address's serving session, with
    since when, written by compare-and-set, and the state directory's
    `addresses/<digest>.json` (C.7; core.16).
  - `Engine::serve`, answering what the inbox answers for the address.
  - `deliver_ends` delivering an address's end into its live run's pane,
    else its live serving session.
  - `wake_address` waking no run for an address a live session serves,
    and `wake` answering naming that session.
  - status giving each address's serving session.

  (design 3; 43, 132). True when `sluice exec main -- moon run seams:test
  core:test conformance:test flume:test` passes. `engine/role_waking.rs`
  holds the role-waking spec's serving scenarios and the wake of a
  served address, with S93 named `s93_an_address_that_wakes_nothing`.
  `store_state_dir.rs` holds the document's round trip.
- [ ] 3.3 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test conformance:format conformance:lint
  conformance:test flume:format flume:lint flume:test` passes.

## 4. The stall reaches the answerer, and a hold on the waiting move

- [ ] 4.1 Have `EndOf::Stall` carry the stop-short the place waits on,
  its words, needs and key, read with none where an earlier flume wrote
  none. Have the stall end record the answerer the configuration names
  when `read_stall` reads it. Have the end stand pending for the
  answerer besides the place's recipients where it is none of them.
  Have a stall an earlier flume recorded take the host's answerer at
  the first settle that reads it, while the place still waits on its
  caller under no hold's words (design 5; 43, 130, 190). True when
  `sluice exec main -- moon run seams:test core:test conformance:test`
  passes. `engine/caller_waits.rs` holds the caller-waits spec's
  scenarios of a stall and of a stall recorded before, and every
  existing scenario passes.
- [ ] 4.2 Have `Engine::hold` take the waiting move, `HoldOn::Waiting`,
  with its words and an optional bound, refused on a place that waits on
  no move of its caller's. Have `read_stall` read a place held so as
  not stalled until the bound passes, then lapse the hold, recorded with
  when, and stall the place. Have a retry naming the stage or the
  submission, and a drop, end it (design 6; 47, 49, 52, 130). True when
  `sluice exec main -- moon run seams:test core:test conformance:test`
  passes, with the caller-waits spec's four scenarios of the hold.
- [ ] 4.3 Have status give a hold on the waiting move with its words,
  since when, its bound or that it lapsed, and a stall's stop-short
  (56, 130; core.14). True when `sluice exec main -- moon run core:test
  conformance:test flume:test` passes, with `print.rs`'s tests of both
  forms.
- [ ] 4.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test conformance:format conformance:lint
  conformance:test flume:format flume:lint flume:test` passes.

## 5. An answer taken once, by its key

- [ ] 5.1 Give each end its key as callers see it,
  `stage/<stage>/<start>/<state>`, `submission/<n>/<outcome>` and
  `stall/<since>`, made from `EndOf::key`'s parts and parsed back,
  refused in any other form, naming the forms (design 7; 42, 43). True
  when `sluice exec main -- moon run seams:test` passes, with a round
  trip of every end's key in `round_trip.rs`.
- [ ] 5.2 Have `Engine::retry` take an optional key. It is taken only
  while the place waits on the stop-short the key names, and records on
  that end the answer's words, who gave them, as the retrying command's
  recipient names it, and when. It is refused otherwise, naming the
  answer taken, who gave it and when, or the move the place waits on
  now. A retry with no key records the answer with none named. A retry
  refused for its key ends no agent (design 7; 42, 49, 50). True when
  `sluice exec main -- moon run core:test conformance:test` passes.
  `engine/caller_acts.rs` holds the caller-acts spec's four scenarios,
  with S91's engine part named `s91_a_question_from_a_shell_answered_once`.
- [ ] 5.3 Have status give each end's key, and a stop-short's answer
  with who gave it and when (56; core.14). True when `sluice exec main
  -- moon run core:test conformance:test flume:test` passes, with the
  place-status spec's scenario of an answered question.
- [ ] 5.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test conformance:format conformance:lint
  conformance:test flume:format flume:lint flume:test` passes.

## 6. What a restarted recipient sees

- [ ] 6.1 Have `Engine::inbox`, and `serve` with it, answer `waiting`
  beside `ends`: every place that waits on its caller and that the
  recipient is a recipient of, or was given a stall of as the answerer,
  with what the end-delivery spec names. It marks nothing for them
  (design 8; 43, 130; core.25). True when `sluice exec main -- moon run
  core:test conformance:test flume:test` passes, with the end-delivery
  spec's scenarios of the inbox in `engine/end_delivery.rs`.
- [ ] 6.2 Have `notice` name the place, its brief's first line cut at 60
  characters, the end, its key, and `flume inbox` for an end that waits
  on a move or `flume status <place>` for any other (design 8; 43;
  core.11). True when `sluice exec main -- moon run core:test
  conformance:test` passes, with the end-delivery spec's two scenarios
  of a notice and every scenario that reads a notice's words brought to
  the new line.
- [ ] 6.3 In the Claude Code profile's `hook`, have SessionStart in a
  session flume did not start run `serve` of the address
  `$FLUME_RECIPIENT` names, else the inbox as the terminal's session.
  Have it give the agent, as SessionStart's added context, each end and
  each waiting place with its key and the three ways on, or nothing.
  Every other hook still does nothing there, and none fails or holds up
  a start (design 8; 43, 132, C.4). Update the hooks' description in
  `plugins/claude/hooks/hooks.json`. True when `sluice exec main -- moon
  run flume:test plugin-claude:test` passes, with the claude-code-plugin
  spec's scenarios in `hooks_claude.rs` and `plugins/claude/tests/`, the
  fixture `session-start-another-session.json` among them.
- [ ] 6.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run core:format core:lint core:test
  conformance:format conformance:lint conformance:test flume:format
  flume:lint flume:test plugin-claude:format plugin-claude:validate plugin-claude:test` passes.

## 7. The command line and the walkthroughs

- [ ] 7.1 Add `flume serve <address>`, `flume hold … --waiting [--for
  <time>]` and `flume retry … --answers <key>` to the command line, with
  their help naming the clauses, and `flume launch --recipient`'s help
  naming the host's answerer (design 3, 6, 7; 7, 42, 49, 61, 130, 132).
  True when `sluice exec main -- moon run flume:test` passes, with the
  command-line spec's scenarios.
- [ ] 7.2 Add to `crates/flume/tests/profiles/tools/walkthroughs.rs`,
  under the pinned herdr with the stand-in, every recipient a scratch
  one of the test's own:
  - **S91.** A change launched from no session, its recipient the
    scratch host's answerer, whose run the scratch `[roles]` names as a
    kind of the test's own. Its construct stops short needing an answer.
    The answerer's run is woken and collects it with its key. The run
    holds the waiting move. A retry naming the key starts construct
    afresh with the question and the answer. A second retry naming the
    key is refused.
  - **S92.** A change launched from a scratch pane, whose stand-in is
    restarted after the notice. The new session's SessionStart gives it
    the place, and five minutes of the walkthrough's clock later the
    stall reaches the answerer.

  (43, 130, 132, 190, S91, S92). True when `sluice exec main -- moon run
  flume:tools` passes with them, and every existing walkthrough passes.
- [ ] 7.3 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run flume:format flume:lint flume:test
  flume:tools` passes.

## 8. The site

- [ ] 8.1 Write the design into the site, to any host's operator, naming
  no host, team or word of ours (core.35):
  - `guides/callers-and-holds.md`: who answers a place, the hold on the
    waiting move, the answer's key, and holding before asking a person.
  - `guides/roles-and-wake.md`: the host's answerer, serving an address,
    a run named by its whole address, and an answerer's run's way with a
    question that is a person's, one card keyed by the place and the end.
  - `guides/claude-code.mdx`: what SessionStart gives a session of the
    operator's own.
  - `reference/configuration.md`: `[host] answerer` and a stream's.
  - `reference/commands.md`: `serve`, `hold --waiting` and `retry
    --answers`.
  - `reference/state.md`: the served addresses' documents, an end's key
    and answer, and a hold on the waiting move.

  True when `sluice exec main -- moon run site:prose site:build` passes.
- [ ] 8.2 Run the checks of the project this group touched. True when
  `sluice exec main -- moon run site:prose site:build root:format`
  passes.

## 9. Coverage and the gate

- [ ] 9.1 Have the suite's clause list read the statement at the
  revision of agentplot/blueprints main that holds record 0039's
  amendment, Part A with 190 and the scenarios to S94. Have 190 and S91
  to S94 cited by this change's scenarios, and nothing of this change's
  named unbuilt (clause-coverage spec; core.19). True when `sluice exec
  main -- moon run conformance:test conformance:coverage` passes.
- [ ] 9.2 Run the checks of the projects this change touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test fakes:format fakes:lint fakes:test
  conformance:format conformance:lint conformance:test
  conformance:coverage flume:format flume:lint flume:test flume:tools
  plugin-claude:format plugin-claude:validate plugin-claude:test site:prose site:build
  root:format root:adrs root:changes root:specs` passes.
- [ ] 9.3 Have every requirement this change modifies stay under the
  strict check's length once archived, splitting one that grows past it
  into an added requirement of its own with the scenarios that show it.
  True when `openspec archive
  every-question-a-place-asks-reaches-someone-who-answers-it --yes` over
  a copy of `openspec/` leaves `openspec validate --specs --strict`
  passing, and `sluice exec main` passes.
