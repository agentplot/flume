# Tasks

Each group is one session's work for an agent that reads this change
(proposal, design, specs) and its group, and builds on main and on the
groups before it. A group's last task runs the checks it touched
through sluice. Every conformance scenario a group adds cites its
clauses as data (106).

Group 1 is built once the user has read the design and answered its
Open Questions. The groups after it are built once flume's statement is
amended on agentplot/blueprints main as design 11.1 words it, and once
the change `a-host-declares-where-its-kinds-come-from` has landed on
this repository's main, since a source's declaration and its packages
are read by it. An answer of the user's that changes the design is
written into the design, the specs and these tasks before group 2
starts.

Tests reach no real account, no repository of the user's, no person's
queue and no network. Every git call in a test runs under the scratch
environment `crates/flume/tests/common` gives. An agent in a test is the
stand-in (`crates/flume/tests/profiles/stand-in/`) or the fake. Every
host configuration, source, roles file and state directory a test names
is a scratch one of the test's own, and every address in one is a
scratch partition's, such as `acme/` or `east/`. The engine's scenarios
name no program, no terminal, no queue and no caller's tool (core.1).

## 1. The decision recorded

- [ ] 1.1 Record in record 0039 the user's answers to the design's Open
  Questions, quoting their words with the date, and lift the proposed
  mark from the parts they read. Its consequences give the
  agentplot/blueprints revision that holds the statement's amendment
  (4, 7, 8, 28, 41, 42, 43, 49, 56, 130, 132, 143, 191, 192, 193, C.4,
  C.7, S92 to S94, S98; core.19). True when `sluice exec main -- moon
  run root:adrs root:format` passes.
- [ ] 1.2 Add to records 0009, 0013, 0014 and 0040 an amendment line
  each, naming record 0039 and what it changes of that record. 0009: a
  caller's start also sees what waits on it, and a notice names the
  brief and the key. 0013: a role is declared by a source, never by the
  host's `[roles]`; a run is named per address, carries its stream,
  settings and intent, and is over at its delivery; a stop-short may
  name the role it is for. 0014: the answerer behind a stream is a
  recipient of every stall. 0040: a source's declaration may name its
  roles, read as its packages are. Remove and reword nothing the records
  wrote (core.19). True when `sluice exec main -- moon run root:adrs
  root:format` passes.
- [ ] 1.3 Amend `docs/architecture/core.md`:
  - core.24's rule: a stalled place is said to each recipient, to the
    role its question named and to the answerer behind its stream.
  - core.25's details: a caller's own session collecting at its start by
    the plugin, and the inbox's `waiting` marking nothing.
  - core.28's rule and details: a role is declared by a source of the
    host and never by the host's configuration; a run is named per
    address, with its stream, settings and intent from its source's
    entry, declared done at its delivery with no caller's word, and its
    stall to the answerer; a launch naming an address that wakes nothing
    refused only where no answerer stands behind its stream. Its details
    rule out a `[roles]` table or an answerer read from the host's
    configuration, a role's kind taken from another source than its
    entry's, a role's entry naming an `env`, a pool or an account
    (core.30), a source's roles fault refusing another source's role or
    launch (191), and flume writing a roles file.
  - Draw a rule under "Settling, surfaces and state", the next free id
    when this lands, citing 4, 7, 41, 42, 49, 130, 193 and record 0039:
    "Every place has a recipient whose run is woken for its ends or that
    collects them, a stop-short may name the role of its place's
    partition it is for, the answerer behind a place's stream is
    delivered every stall besides the place's recipients, and an answer
    naming a stop-short's key is taken only while the place waits on
    that stop-short."
    - Its details rule out: a launch taken with no recipient into a
      stream no role stands behind; a stall the answerer is not
      delivered; a stop-short's end sent to the place's recipients when
      it named a role; flume choosing the role a question goes to,
      reading its words; a second answer to one key taken; and flume
      making a card, or deciding which question is a person's (69, 72).
    - They do not rule out: a retry with no key, taken as before; a
      launch naming a recipient it collects as itself; a stop-short that
      names no role.

  Have `site/src/content/docs/architecture/rules.md` mirror the new rule
  and core.28 as written (core.19, core.35). True when `sluice exec main
  -- moon run root:format site:prose site:build` passes.
- [ ] 1.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run root:format root:adrs site:prose
  site:build` passes.

## 2. A source's roles and the answerer

- [ ] 2.1 Give `flume-seams`' `RoleRun` its source, `name`, `intent`,
  `agents` (the settings a launch's `--agent` gives), `answers`,
  `answerer` and `carried`. Give `Host` the run a source declares for an
  address, or why none is; the answerer behind a stream, with its
  source, or the two that stand behind it; and the roles of a partition,
  every address a source declares with `answers`, each with its words.
  Have the fakes' host script each (design 1, 3, 5; 132, 193, C.7). True
  when `sluice exec main -- moon run seams:test fakes:test` passes.
- [ ] 2.2 Have the host's configuration reader in `crates/flume` read a
  source's `roles`, an absolute path or one under `~`, and the file it
  names, as it stands when asked, writing nothing. Keep each fault as
  design 1 has it: a file that cannot be read declaring no role; an
  entry naming a kind not its source's, an `env`, a pool or an account,
  mixing `carried` with a run's keys, or giving `answerer` with no
  `stream`, declaring nothing for its address; an address two entries
  declare, and two addresses making one place, declared by neither;
  each named, and none failing the configuration or another source.
  Have a `config.toml` that gives `[roles]` unreadable, naming it and
  that a source declares a role. Remove the reading of `[roles]` from
  `config.rs`, and name the roles file in the module doc (design 1, 5;
  132, 191, 193, C.7; core.30). True when `sluice exec main -- moon run
  flume:test` passes, with `crates/flume/tests/host.rs` holding, over a
  scratch `XDG_CONFIG_HOME`, the role-waking spec's scenarios of a
  source's roles and its faults and the history-and-host spec's
  scenarios of the answerer and of a partition's roles.
- [ ] 2.3 Have a role's kind resolved as its source's own, `<source>/<kind>`,
  with its version the locks', and refused at its wake as a launch's
  kind is where no lock records it (design 1; 154, 155, 193; core.34).
  True when `sluice exec main -- moon run core:test conformance:test`
  passes, with the role-waking spec's *A role configured* in
  `engine/role_waking.rs`.
- [ ] 2.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  fakes:format fakes:lint fakes:test core:format core:lint core:test
  conformance:format conformance:lint conformance:test flume:format
  flume:lint flume:test` passes.

## 3. A launch's recipients

- [ ] 3.1 In `launch.rs`, `make_place`:
  - Have a launch's recipients end at the answerer behind its stream,
    after `$FLUME_RECIPIENT` and the terminal's session, with where each
    came from kept in held state.
  - Refuse, before anything is made, a launch that finds none of the
    four; one into a stream two roles stand behind; and one whose
    recipients hold an address no source declares, or two declare, where
    no answerer stands behind its stream, each named as the launching
    spec has it (design 4; 4, 7, 132, 193; core.8).

  True when `sluice exec main -- moon run core:test conformance:test
  flume:test` passes. `engine/launching.rs` holds the launching spec's
  scenarios, and every existing scenario passes. The ones that launched
  from no session with no recipient name one now.
- [ ] 3.2 Have status give each place's recipients with where each came
  from, each stream's answerer with its source, each source's roles or
  why they cannot be read, and for each address an end stands pending
  for, its run, or that it is carried or wakes nothing and why, in the
  JSON form, as fields added (design 1, 4; 56; core.14). True when
  `sluice exec main -- moon run core:test conformance:test flume:test`
  passes, with the place-status spec's scenarios of recipients and of an
  address that wakes nothing in `engine/place_status.rs` and `print.rs`'s
  tests.
- [ ] 3.3 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run core:format core:lint core:test
  conformance:format conformance:lint conformance:test flume:format
  flume:lint flume:test` passes.

## 4. A role's run

- [ ] 4.1 In `wake.rs`, have `wake_address` name the run by its entry's
  `name`, else `<partition>-<role>`, and launch it with its entry's
  stream, intent and `agents` as the launch's. Have `run_of` find a run
  by its role field first, so a run an earlier flume named by the role
  alone is found. Have a carried address wake nothing, and an address
  declared by nothing, or by two, say why in its wake's refusal (design
  1; 3, 7, 32, 132, 143, 147, 193). True when `sluice exec main -- moon
  run seams:test core:test conformance:test` passes.
  `engine/role_waking.rs` holds the role-waking spec's scenarios of the
  run's session, its intent and settings, its stream and label, the two
  partitions' planners, named `s98_one_declaration_two_partitions`, the
  run an earlier flume named, the carried address and a fault's address.
- [ ] 4.2 Have the settle that reads a role's run's last stage delivered
  declare it done, with no caller's word, and tidy it, so a later end
  wakes another run. Have a role's run's stall delivered to the answerer
  behind its stream, and never to its own address (design 1, 5; 114,
  130, 132, 193). True when `sluice exec main -- moon run core:test
  conformance:test` passes, with the role-waking spec's scenarios of a
  run over at its delivery, the next end, and a run's stall, S98's done
  among them, and every existing scenario passes.
- [ ] 4.3 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test conformance:format conformance:lint
  conformance:test` passes.

## 5. A question addressed to its role

- [ ] 5.1 Have the way to stop short in a rendered prompt list each
  address the agent may name, with its `answers` words: the roles of
  the partition of the place's recipients' addresses, else of the
  answerer behind its stream, read from the sources at each start and
  placed after every part a stage's repetitions share (design 3; 28, 41,
  193; core.31). True when `sluice exec main -- moon run core:test
  conformance:test` passes, with the stage-runs spec's three scenarios
  of the list, and every scenario that reads a rendered prompt brought
  to it.
- [ ] 5.2 Have a stop-short take an optional address. Take it only
  among the addresses the start's prompt listed, and refuse any other
  naming the list, the stage going on. Record it on the stop-short's
  end, and have the end stand pending for that address alone, not for
  the place's recipients, so the settle wakes its run. Have status give
  whom it named (design 3; 41, 43, 56, 132). True when `sluice exec
  main -- moon run seams:test core:test conformance:test` passes, with
  the stage-runs spec's scenarios of a stop-short addressed, S92's
  engine part named `s92_a_question_for_ops_answered_once` once group 7
  gives the key, and every existing stop-short scenario passes.
- [ ] 5.3 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test conformance:format conformance:lint
  conformance:test` passes.

## 6. The stall, and a hold on the waiting move

- [ ] 6.1 Have `EndOf::Stall` carry the stop-short the place waits on,
  its words, needs, key and addressee, read with none where an earlier
  flume wrote none. Have the stall end record the answerer behind the
  place's stream as the sources declare it when `read_stall` reads it.
  Have the end stand pending for the stop-short's addressee and the
  answerer besides the place's recipients, each once. Have a stall an
  earlier flume recorded take the answerer at the first settle that
  reads it, while the place still waits on its caller under no hold's
  words (design 5; 43, 130, 193). True when `sluice exec main -- moon
  run seams:test core:test conformance:test` passes.
  `engine/caller_waits.rs` holds the caller-waits spec's scenarios of a
  stall, of the role a question named, and of a stall recorded before,
  and every existing scenario passes.
- [ ] 6.2 Have `Engine::hold` take the waiting move, `HoldOn::Waiting`,
  with its words and an optional bound, a later hold replacing them,
  refused on a place that waits on no move of its caller's. Have
  `read_stall` read a place held so as not stalled until the bound
  passes, then lapse the hold, recorded with when, and stall the place
  again with the lapsed hold's words. Have a retry naming the stage or
  the submission, and a drop, end it (design 6; 47, 49, 52, 130). True
  when `sluice exec main -- moon run seams:test core:test
  conformance:test` passes, with the caller-waits spec's scenarios of
  the hold and its lapse, S94's engine part named
  `s94_a_card_held_and_lapsed`.
- [ ] 6.3 Have status give a hold on the waiting move with its words,
  since when, its bound or that it lapsed, and a stall's stop-short with
  its addressee (56, 130; core.14). True when `sluice exec main -- moon
  run core:test conformance:test flume:test` passes, with `print.rs`'s
  tests of both forms.
- [ ] 6.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test conformance:format conformance:lint
  conformance:test flume:format flume:lint flume:test` passes.

## 7. An answer taken once, by its key

- [ ] 7.1 Give each end its key as callers see it,
  `stage/<stage>/<start>/<state>`, `submission/<n>/<outcome>` and
  `stall/<since>`, made from `EndOf::key`'s parts and parsed back,
  refused in any other form, naming the forms (design 7; 42, 43). True
  when `sluice exec main -- moon run seams:test` passes, with a round
  trip of every end's key in `round_trip.rs`.
- [ ] 7.2 Have `Engine::retry` take an optional key. It is taken only
  while the place waits on the stop-short the key names, and records on
  that end the answer's words, who gave them, as the retrying command's
  recipient names it, and when. It is refused otherwise, naming the
  answer taken, who gave it and when, or the move the place waits on
  now. A retry with no key records the answer with none named. A retry
  refused for its key ends no agent (design 7; 42, 49, 50). True when
  `sluice exec main -- moon run core:test conformance:test` passes.
  `engine/caller_acts.rs` holds the caller-acts spec's four scenarios,
  with S92's engine part, `s92_a_question_for_ops_answered_once`.
- [ ] 7.3 Have status give each end's key, and a stop-short's answer
  with who gave it and when (56; core.14). True when `sluice exec main
  -- moon run core:test conformance:test flume:test` passes, with the
  place-status spec's scenario of an answered question.
- [ ] 7.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test conformance:format conformance:lint
  conformance:test flume:format flume:lint flume:test` passes.

## 8. What a restarted recipient sees

- [ ] 8.1 Have `Engine::inbox` answer `waiting` beside `ends`: every
  place that waits on its caller and that the recipient is a recipient
  of, was named by the stop-short it waits on, or was given a stall of
  as the answerer, with what the end-delivery spec names. It marks
  nothing for them (design 8; 43, 130, 193; core.25). True when `sluice
  exec main -- moon run core:test conformance:test flume:test` passes,
  with the end-delivery spec's scenarios of the inbox in
  `engine/end_delivery.rs`.
- [ ] 8.2 Have `notice` name the place, its brief's first line cut at 60
  characters, the end, its key, and `flume inbox` for an end that waits
  on a move or `flume status <place>` for any other (design 8; 43;
  core.11). True when `sluice exec main -- moon run core:test
  conformance:test` passes, with the end-delivery spec's two scenarios
  of a notice and every scenario that reads a notice's words brought to
  the new line.
- [ ] 8.3 In the Claude Code profile's `hook`, have SessionStart in a
  session flume did not start run the inbox as the session the terminal
  names, never as an address. Have it give the agent, as SessionStart's
  added context, each end and each waiting place with its key and the
  three ways on, or nothing. Every other hook still does nothing there,
  and none fails or holds up a start (design 8; 43, C.4). Update the
  hooks' description in `plugins/claude/hooks/hooks.json`. True when
  `sluice exec main -- moon run flume:test plugin-claude:test` passes,
  with the claude-code-plugin spec's scenarios in `hooks_claude.rs` and
  `plugins/claude/tests/`, the fixture
  `session-start-another-session.json` among them.
- [ ] 8.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run core:format core:lint core:test
  conformance:format conformance:lint conformance:test flume:format
  flume:lint flume:test plugin-claude:format plugin-claude:validate
  plugin-claude:test` passes.

## 9. The command line and the walkthroughs

- [ ] 9.1 Add `flume needs --to <address>`, `flume hold … --waiting
  [--for <time>]` and `flume retry … --answers <key>` to the command
  line, with their help naming the clauses, and `flume launch
  --recipient`'s help naming the answerer behind a stream (design 3, 6,
  7; 7, 41, 42, 49, 61, 130, 193). True when `sluice exec main -- moon
  run flume:test` passes, with the command-line spec's scenarios.
- [ ] 9.2 Add to `crates/flume/tests/profiles/tools/walkthroughs.rs`,
  under the pinned herdr with the stand-in, every source, roles file,
  recipient and address a scratch one of the test's own:
  - **S92.** A scratch source `acme`, its one package locked on the
    scratch host, whose roles file declares runs for `acme/operator`,
    standing behind the stream `work`, and `acme/ops`, each with its
    words. A change launched from no session in `work`, its recipient
    `acme/operator`. Its construct stops short naming `acme/ops`. The
    run of `acme/ops` is woken and collects the question with its key,
    holds the waiting move, and retries naming the key, starting
    construct afresh with the question and the answer. A second retry
    naming the key is refused. No run of `acme/operator` is launched.
    The run of `acme/ops` delivers and is tidied with no caller's word.
  - **S93.** A change launched from a scratch pane, whose stand-in is
    restarted after the notice. The new session's SessionStart gives it
    the place, and five minutes of the walkthrough's clock later the
    stall reaches `acme/operator`.

  (41, 43, 130, 132, 191, 193, S92, S93). True when `sluice exec main --
  moon run flume:tools` passes with them, and every existing walkthrough
  passes.
- [ ] 9.3 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run flume:format flume:lint flume:test
  flume:tools` passes.

## 10. The site

- [ ] 10.1 Write the design into the site, to any host's operator,
  naming no host, team or word of ours (core.35):
  - `guides/callers-and-holds.md`: who answers a place, addressing a
    role with `flume needs --to`, the hold on the waiting move, the
    answer's key, and holding before asking a person.
  - `guides/roles-and-wake.md`: roles declared by a source, once for
    every partition, and what of a host remains; a run named per
    address, its stream, settings and intent, its end at its delivery;
    the answerer behind a stream; a carried address; and an answering
    run's way with a question: hold, answer, pass, or one card keyed by
    the place and the end.
  - `guides/claude-code.mdx`: what SessionStart gives a session of the
    operator's own.
  - `reference/configuration.md`: a source's `roles`, its file's form
    and faults, and that the host's configuration names no role.
  - `reference/commands.md`: `needs --to`, `hold --waiting` and `retry
    --answers`.
  - `reference/state.md`: an end's key and answer, a stop-short's
    addressee, and a hold on the waiting move.

  True when `sluice exec main -- moon run site:prose site:build` passes.
- [ ] 10.2 Run the checks of the project this group touched. True when
  `sluice exec main -- moon run site:prose site:build root:format`
  passes.

## 11. Coverage and the gate

- [ ] 11.1 Have the suite's clause list read the statement at the
  revision of agentplot/blueprints main that holds record 0039's
  amendment, Part A with 193 and the scenarios to S98. Have 193, S92 to
  S94 and S98 cited by this change's scenarios, and nothing of this
  change's named unbuilt (clause-coverage spec; core.19). True when
  `sluice exec main -- moon run conformance:test conformance:coverage`
  passes.
- [ ] 11.2 Run the checks of the projects this change touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test fakes:format fakes:lint fakes:test
  conformance:format conformance:lint conformance:test
  conformance:coverage flume:format flume:lint flume:test flume:tools
  plugin-claude:format plugin-claude:validate plugin-claude:test
  site:prose site:build root:format root:adrs root:changes root:specs`
  passes.
- [ ] 11.3 Have every requirement this change modifies stay under the
  strict check's length once archived, splitting one that grows past it
  into an added requirement of its own with the scenarios that show it.
  True when `openspec archive
  every-question-a-place-asks-reaches-someone-who-answers-it --yes` over
  a copy of `openspec/` leaves `openspec validate --specs --strict`
  passing, and `sluice exec main` passes.
