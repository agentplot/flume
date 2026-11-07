# Tasks

Each group is one session's work for an agent that reads this change
(proposal, design, specs) and its group, and builds on main and on the
groups before it. A group's last task runs the checks it touched
through sluice. Every conformance scenario a group adds cites its
clauses as data (106).

Group 1 is built once the user has read the design. The groups after
it are built once flume's statement is amended on agentplot/blueprints
main as design 12.1 words it, and once the change
`a-roles-address-may-be-held-by-a-standing-agent` has landed on this
repository's main or been dropped, since both read a role's entry. They
build on `a-host-declares-where-its-kinds-come-from`, landed on main,
since a source's declaration and its packages are read by it. Group 7 is built once record 0046's change
`an-answer-reaches-the-open-session-within-an-hour` has landed too,
since an answer handed to an agent kept open is recorded in the end's
answer this group's key reads. An answer of the user's that changes the
design is written into the design, the specs and these tasks before
group 2 starts.

Tests reach no real account, no repository of the user's, no person's
queue and no network. Every git call in a test runs under the scratch
environment `crates/flume/tests/common` gives. An agent in a test is the
stand-in (`crates/flume/tests/profiles/stand-in/`) or the fake. Every
host configuration, source, roles file and state directory a test names
is a scratch one of the test's own, and every address in one is a role
of a scratch stream, such as `east/ops` of the stream `east`. The engine's scenarios
name no program, no terminal, no queue and no caller's tool (core.1).

## 1. The decision recorded

- [x] 1.1 Record in record 0039 the user's read of the design, quoting
  their words with the date, and lift the proposed mark from the parts
  they read. Its consequences give the agentplot/blueprints revision
  that holds the statement's amendment (section 3's address, 4, 7, 8,
  28, 41, 42, 43, 49, 53, 56, 130, 132, 143, 191, 192, 193, C.4, C.7,
  S93, S94, S98, S99; core.19). True when
  `sluice exec main -- moon run root:adrs root:format` passes.
- [x] 1.2 Add to records 0009, 0013, 0014 and 0040 an amendment line
  each, naming record 0039 and what it changes of that record. 0009: a
  caller's start also sees what waits on it, and a notice names the
  brief and the key. 0013: an address is `<stream>/<role>`, a role of a
  stream the host declares, where the record wrote `<partition>/<role>`;
  a role is declared by a source, never by the host's `[roles]`; a run
  is named per address, runs in its address's stream, carries its
  settings and intent, and is over at its delivery; a stop-short may
  name the role of any stream it is for. 0014: the answerer behind a stream is a
  recipient of every stall. 0040: a source's declaration may name its
  roles, read as its packages are. Remove and reword nothing the records
  wrote (core.19). True when `sluice exec main -- moon run root:adrs
  root:format` passes.
- [x] 1.3 Amend `docs/architecture/core.md`:
  - core.24's rule: a stalled place is said to each recipient, to the
    role its question named and to the answerer behind its stream.
  - core.25's details: a caller's own session collecting at its start by
    the plugin, and the inbox's `waiting` marking nothing.
  - core.28's rule and details: a role's address is `<stream>/<role>`,
    a role of a stream the host declares; a role is declared by a source
    of the host and never by the host's configuration; a run is named
    per address, runs in its address's stream, with its settings and
    intent from its source's entry, declared done at its delivery with
    no caller's word, and its
    stall to the answerer; a launch naming an address that wakes nothing
    refused only where no answerer stands behind its stream. Its details
    rule out a `[roles]` table or an answerer read from the host's
    configuration, a role's kind taken from another source than its
    entry's, a role's entry naming an `env`, a pool or an account
    (core.30), a role's run in a stream its address does not name, an
    address's first part read as anything but a stream, a source's
    roles fault refusing another source's role or launch (191), and
    flume writing a roles file.
  - Draw a rule under "Settling, surfaces and state", the next free id
    when this lands, citing 4, 7, 41, 42, 49, 130, 193 and record 0039:
    "Every place has a recipient whose run is woken for its ends or that
    collects them, a stop-short may name the role it is for, its own
    stream's by the role alone and any other stream's by its address,
    the answerer behind a place's stream is delivered every stall
    besides the place's recipients, and an answer
    naming a stop-short's key is taken only while the place waits on
    that stop-short."
    - Its details rule out: a launch taken with no recipient into a
      stream no role stands behind; a stall the answerer is not
      delivered; a stop-short's end sent to the place's recipients when
      it named a role; a question kept to its place's stream; flume
      choosing the role a question goes to,
      reading its words; a second answer to one key taken, by a retry or
      a tell; a way to stop short that leaves a stage's agent to ask a
      person itself; and flume making a card, or deciding which question
      is a person's (69, 72).
    - They do not rule out: a retry with no key, taken as before; a
      launch naming a recipient it collects as itself; a stop-short that
      names no role.

  Have `site/src/content/docs/architecture/rules.md` mirror the new rule
  and core.28 as written (core.19, core.35). True when `sluice exec main
  -- moon run root:format site:prose site:build` passes.
- [x] 1.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run root:format root:adrs site:prose
  site:build` passes.
- [x] 1.5 Write the statement's amendment as design 12.1 words it, on a
  new agentplot/blueprints branch cut from main at a4eeec3, unpushed,
  each amended clause fitted to main's text and 193, S93, S94, S98 and
  S99 renumbered where main takes them. Reconcile it with the change
  `a-roles-address-may-be-held-by-a-standing-agent`: a standing agent's
  entry is declared with its source's roles, `agent` and an optional
  `session`, `answers` and `answerer`, so a role's crossing to its kind
  trades `agent` for `kind` in its source's entry (132, 193, C.5, C.7;
  record 0039's amendment). Bring record 0039, the design, the
  proposal, the role-waking spec and these tasks to it. True when the
  branch `design/flume-every-question-a-place-asks-reaches-someone-who-answers-it`
  holds the commit, and `sluice exec main -- moon run root:changes
  root:format root:adrs` passes.

## 2. A source's roles and the answerer

- [x] 2.1 Have `flume-seams`' `is_address` and `role_of` read an address
  as `<stream>/<role>`, with `stream_of` beside them, their docs naming
  the stream, and `held.rs`'s role field likewise (design 1; 132, 143).
  Give `RoleRun` its source, `name`, `intent`, `agents` (the settings a
  launch's `--agent` gives), `answers`, `answerer` and `carried`, its
  stream its address's and no `stream` of its own, and give a standing
  agent's entry its source, `answers` and `answerer` with its stream its
  address's. Give `Host` the run or the standing agent a
  source declares for an address, or why none is; the answerer behind a
  stream, with its source, or the two that stand behind it; and the
  host's answering roles, every address a source declares with
  `answers`, each with its stream and its words. Have the fakes' host
  script each (design 1, 2, 4, 6; 132, 193, C.7). True when `sluice exec
  main -- moon run seams:test fakes:test` passes, `round_trip.rs`'s
  address test named for a stream and a role.
- [x] 2.2 Have the host's configuration reader in `crates/flume` read a
  source's `roles`, an absolute path or one under `~`, and the file it
  names, as it stands when asked, writing nothing, each entry a run's, a
  standing agent's (`agent`, an optional `session`, `answers` and
  `answerer`) or a carried address's. Where
  `a-roles-address-may-be-held-by-a-standing-agent` has landed, move its
  standing entry's reading from `[roles]` to the roles file, its stream
  its address's, and bring that change's archived requirements under
  this change's specs by MODIFIED and REMOVED deltas, so `openspec
  validate --strict` passes. Keep each fault as
  design 2 has it: a file that cannot be read declaring no role; an
  entry naming a kind not its source's, an `env`, a pool, an account or
  a `stream`, whose address names a stream the host does not declare and
  that is not carried, giving `agent` beside `kind` or a run's other
  keys, or mixing `carried` with a run's keys, `agent` or
  `answerer`, declaring nothing for its address; an address two entries
  declare, and two addresses making one place, declared by neither;
  each named, and none failing the configuration or another source.
  Have a `config.toml` that gives `[roles]` unreadable, naming it and
  that a source declares a role. Remove the reading of `[roles]` from
  `config.rs`, with its refusal's `<partition>/<role>`, and name the
  roles file and `<stream>/<role>` in the module doc (design 1, 2, 6;
  132, 143, 191, 193, C.7; core.30, core.35). True when `sluice exec
  main -- moon run flume:test` passes, with `crates/flume/tests/host.rs`
  holding, over a scratch `XDG_CONFIG_HOME`, the role-waking spec's
  scenarios of a source's roles, a standing agent's entry, an address's
  stream and its faults, and
  the history-and-host spec's scenarios of the answerer and of the
  host's answering roles.
- [x] 2.3 Have a role's kind resolved as its source's own, `<source>/<kind>`,
  with its version the locks', and refused at its wake as a launch's
  kind is where no lock records it (design 2; 154, 155, 193; core.34).
  True when `sluice exec main -- moon run core:test conformance:test`
  passes, with the role-waking spec's *A role configured* in
  `engine/role_waking.rs`.
- [x] 2.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  fakes:format fakes:lint fakes:test core:format core:lint core:test
  conformance:format conformance:lint conformance:test flume:format
  flume:lint flume:test` passes.

## 3. A launch's recipients

- [x] 3.1 In `launch.rs`, `make_place`:
  - Have a launch's recipients end at the answerer behind its stream,
    after `$FLUME_RECIPIENT` and the terminal's session, with where each
    came from kept in held state.
  - Refuse, before anything is made, a launch that finds none of the
    four; one into a stream two roles stand behind; and one whose
    recipients hold an address no source declares, or two declare, where
    no answerer stands behind its stream, each named as the launching
    spec has it (design 5; 4, 7, 132, 193; core.8).

  True when `sluice exec main -- moon run core:test conformance:test
  flume:test` passes. `engine/launching.rs` holds the launching spec's
  scenarios, and every existing scenario passes. The ones that launched
  from no session with no recipient name one now.
- [x] 3.2 Have status give each place's recipients with where each came
  from, each stream's answerer with its source, each source's roles or
  why they cannot be read, and for each address an end stands pending
  for, its run, or that it is carried or wakes nothing and why, in the
  JSON form, as fields added (design 2, 5; 56; core.14). True when
  `sluice exec main -- moon run core:test conformance:test flume:test`
  passes, with the place-status spec's scenarios of recipients and of an
  address that wakes nothing in `engine/place_status.rs` and `print.rs`'s
  tests.
- [x] 3.3 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run core:format core:lint core:test
  conformance:format conformance:lint conformance:test flume:format
  flume:lint flume:test` passes.

## 4. A role's run

- [x] 4.1 In `wake.rs`, have `wake_address` name the run by its entry's
  `name`, else `<stream>-<role>`, and launch it in its address's stream
  with its entry's intent and `agents` as the launch's. Have `run_of`
  find a run by its role field first, so a run an earlier flume named by
  the role alone is found. Have a carried address wake nothing, and a
  standing agent's address wake nothing and have its ends delivered into
  that agent, read from its source's entry (record 0039's amendment), and an
  address declared by nothing, or by two, or whose stream the host does
  not declare, say why in its wake's refusal. Have `wake`'s refusal of a
  name of another form, and `flume wake`'s help in `commands/mod.rs`,
  give `<stream>/<role>`, and the module doc say what an address is
  (design 1, 2; 3, 7, 32, 132, 143, 147, 193; core.35). True when
  `sluice exec main -- moon run seams:test core:test conformance:test
  flume:test` passes. `engine/role_waking.rs` holds the role-waking
  spec's scenarios of an address's stream, the run's session, its intent
  and settings, its stream and label, the two streams' planners, named
  `s99_one_declaration_two_streams`, the run an earlier flume named, the
  carried address, a standing design agent, its crossing to its kind and
  a standing door, and a fault's address, and `crates/flume/tests/commands.rs`
  the wake's refusal and help.
- [x] 4.2 Have the settle that reads a role's run's last stage delivered
  declare it done, with no caller's word, and tidy it, so a later end
  wakes another run. Have a role's run's stall delivered to the answerer
  behind its stream, and never to its own address (design 2, 6; 114,
  130, 132, 193). True when `sluice exec main -- moon run core:test
  conformance:test` passes, with the role-waking spec's scenarios of a
  run over at its delivery, the next end, and a run's stall, S99's done
  among them, and every existing scenario passes.
- [x] 4.3 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test conformance:format conformance:lint
  conformance:test` passes.

## 5. A question addressed to its role

- [x] 5.1 Have the way to stop short in a rendered prompt list each
  address the agent may name, with its `answers` words: every address
  the host's sources declare with `answers`, but the place's own role's
  where it is a role's run, the place's own stream's first by the role
  alone and every other stream's after by its whole address, read from
  the sources at each start and placed after every part a stage's
  repetitions share, and have it say that a stop-short is the agent's
  one way to ask and that it writes no card and no message to a person
  (design 4; 28, 41, 193; core.31). True when `sluice
  exec main -- moon run core:test conformance:test` passes, with the
  stage-runs spec's three scenarios of the list, and every scenario that
  reads a rendered prompt brought to it.
- [x] 5.2 Have a stop-short take an optional role or address. Take it
  only among those the start's prompt listed, a role alone read as the
  place's own stream's, and refuse any other naming the list, the stage
  going on. Record the whole address on the stop-short's end, and have
  the end stand pending for that address alone, not for the place's
  recipients, so the settle wakes its run in its own stream. Have status
  give the address it named (design 1, 4; 41, 43, 56, 132). True when
  `sluice exec main -- moon run seams:test core:test conformance:test`
  passes, with the stage-runs spec's scenarios of a stop-short
  addressed, its own stream's role and another stream's, S93's engine
  part named `s93_a_question_for_ops_answered_once` once group 7 gives
  the key, and every existing stop-short scenario passes.
- [x] 5.3 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test conformance:format conformance:lint
  conformance:test` passes.

## 6. The stall, and a hold on the waiting move

- [x] 6.1 Have `EndOf::Stall` carry the stop-short the place waits on,
  its words, needs, key and addressee, read with none where an earlier
  flume wrote none. Have the stall end record the answerer behind the
  place's stream as the sources declare it when `read_stall` reads it.
  Have the end stand pending for the stop-short's addressee, of
  whichever stream, and the answerer behind the place's own stream
  besides the place's recipients, each once. Have a stall an
  earlier flume recorded take the answerer at the first settle that
  reads it, while the place still waits on its caller under no hold's
  words (design 6; 43, 130, 193). True when `sluice exec main -- moon
  run seams:test core:test conformance:test` passes.
  `engine/caller_waits.rs` holds the caller-waits spec's scenarios of a
  stall, of the role a question named, of its own stream and another's,
  and of a stall recorded before, and every existing scenario passes.
- [x] 6.2 Have `Engine::hold` take the waiting move, `HoldOn::Waiting`,
  with its words and an optional bound, a later hold replacing them,
  refused on a place that waits on no move of its caller's. Have
  `read_stall` read a place held so as not stalled until the bound
  passes, then lapse the hold, recorded with when, and stall the place
  again with the lapsed hold's words. Have a retry naming the stage or
  the submission, and a drop, end it (design 7; 47, 49, 52, 130). True
  when `sluice exec main -- moon run seams:test core:test
  conformance:test` passes, with the caller-waits spec's scenarios of
  the hold and its lapse, S98's engine part named
  `s98_the_users_question_held_and_lapsed`.
- [x] 6.3 Have status give a hold on the waiting move with its words,
  since when, its bound or that it lapsed, and a stall's stop-short with
  its addressee (56, 130; core.14). True when `sluice exec main -- moon
  run core:test conformance:test flume:test` passes, with `print.rs`'s
  tests of both forms.
- [x] 6.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test conformance:format conformance:lint
  conformance:test flume:format flume:lint flume:test` passes.

## 7. An answer taken once, by its key

- [x] 7.1 Give each end its key as callers see it,
  `stage/<stage>/<start>/<state>`, with `/<answered>` after it where
  `EndOf::Stage`'s `answered` is not zero, `submission/<n>/<outcome>`
  and `stall/<since>`, made from `EndOf::key`'s parts and parsed back,
  refused in any other form, naming the forms (design 8; 42, 43, 198).
  True
  when `sluice exec main -- moon run seams:test` passes, with a round
  trip of every end's key in `round_trip.rs`.
- [x] 7.2 Have `Engine::retry`, and `Engine::tell` where it answers an
  agent kept open (198), take an optional key. It is taken only while
  the place waits on the stop-short the key names, and records in that
  end's answer, `EndHeld::answer` as record 0046's change writes it for
  an answer handed to a session, the answer's words, who gave them, as
  the answering command's recipient names it, the key and when. It is
  refused otherwise, naming the answer taken, who gave it and when, or
  the move the place waits on now. An answer with no key records it with
  none named. An answer refused for its key ends, starts and types
  nothing (design 8; 42, 49, 50, 53, 54). True when `sluice exec main --
  moon run core:test conformance:test` passes. `engine/caller_acts.rs`
  holds the caller-acts spec's seven scenarios, with S93's engine part,
  `s93_a_question_for_ops_answered_once`.
- [x] 7.3 Have status give each end's key, and a stop-short's answer
  with who gave it and when (56; core.14). True when `sluice exec main
  -- moon run core:test conformance:test flume:test` passes, with the
  place-status spec's scenario of an answered question.
- [x] 7.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run seams:format seams:lint seams:test
  core:format core:lint core:test conformance:format conformance:lint
  conformance:test flume:format flume:lint flume:test` passes.

## 8. What a restarted recipient sees

- [x] 8.1 Have `Engine::inbox` answer `waiting` beside `ends`: every
  place that waits on its caller and that the recipient is a recipient
  of, was named by the stop-short it waits on, or was given a stall of
  as the answerer, with what the end-delivery spec names. It marks
  nothing for them (design 9; 43, 130, 193; core.25). True when `sluice
  exec main -- moon run core:test conformance:test flume:test` passes,
  with the end-delivery spec's scenarios of the inbox in
  `engine/end_delivery.rs`.
- [x] 8.2 Have `notice` name the place, its brief's first line cut at 60
  characters, the end, its key, and `flume inbox` for an end that waits
  on a move or `flume status <place>` for any other (design 9; 43;
  core.11). True when `sluice exec main -- moon run core:test
  conformance:test` passes, with the end-delivery spec's two scenarios
  of a notice and every scenario that reads a notice's words brought to
  the new line.
- [x] 8.3 In the Claude Code profile's `hook`, have SessionStart in a
  session flume did not start run the inbox as the session the terminal
  names, never as an address. Have it give the agent, as SessionStart's
  added context, each end and each waiting place with its key and the
  three ways on, or nothing. Every other hook still does nothing there,
  and none fails or holds up a start (design 9; 43, C.4). Update the
  hooks' description in `plugins/claude/hooks/hooks.json`. True when
  `sluice exec main -- moon run flume:test plugin-claude:test` passes,
  with the claude-code-plugin spec's scenarios in `hooks_claude.rs` and
  `plugins/claude/tests/`, the fixture
  `session-start-another-session.json` among them.
- [x] 8.4 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run core:format core:lint core:test
  conformance:format conformance:lint conformance:test flume:format
  flume:lint flume:test plugin-claude:format plugin-claude:validate
  plugin-claude:test` passes.

## 9. The command line and the walkthroughs

- [ ] 9.1 Add `flume needs --to <role or address>`, `flume hold …
  --waiting [--for <time>]`, `flume retry … --answers <key>` and `flume
  tell … --answers <key>` to the command line, with their help naming
  the clauses, `needs --to`'s
  saying that a role alone is the place's own stream's, and `flume
  launch --recipient`'s help naming a role's address of any stream and
  the answerer behind a stream (design 1, 4, 7, 8; 7, 41, 42, 49, 61,
  130, 132, 193). True when `sluice exec main -- moon
  run flume:test` passes, with the command-line spec's scenarios.
- [ ] 9.2 Add to `crates/flume/tests/profiles/tools/walkthroughs.rs`,
  under the pinned herdr with the stand-in, every source, roles file,
  recipient and address a scratch one of the test's own:
  - **S93.** A scratch host declaring the stream `east`, and a scratch
    source `acme`, its one package locked on the host, whose roles file
    declares runs for `east/operator`, standing behind `east`, and
    `east/ops`, each with its words. A change launched from no session
    in `east`, its recipient `east/operator`. Its construct stops short
    naming `ops`. The run of `east/ops` is woken in `east` and collects
    the question with its key, holds the waiting move, and retries
    naming the key, which reaches construct's stand-in, kept open for
    its answer, in its session. A second retry naming the key is
    refused. No run of
    `east/operator` is launched. The run of `east/ops` delivers and is
    tidied with no caller's word.
  - **S94.** A change launched in `east` from a scratch pane, whose
    stand-in is restarted after the notice. The new session's
    SessionStart gives it the place, and five minutes of the
    walkthrough's clock later the stall reaches `east/operator`.

  (41, 42, 43, 130, 132, 191, 193, 198, S93, S94). True when `sluice exec main --
  moon run flume:tools` passes with them, and every existing walkthrough
  passes.
- [ ] 9.3 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run flume:format flume:lint flume:test
  flume:tools` passes.

## 10. The site

- [ ] 10.1 Write the design into the site, to any host's operator,
  naming no host, team or word of ours (core.35):
  - `guides/callers-and-holds.md`: who answers a place, a stage's agent
    asking by `flume needs` alone, addressing a role with `flume needs
    --to`, its own stream's by name and another's by its address, the
    hold on the waiting move, the answer's key on a retry or a tell, and
    holding before putting a question to a person.
  - `guides/roles-and-wake.md`: a role's address `<stream>/<role>`, the
    role of a stream the host declares, where the page says
    `<group>/<role>`; roles declared by a source, once for every stream,
    and what of a host remains; a run named per address, in its
    address's stream, with its settings and intent, its end at its
    delivery;
    the answerer behind a stream; a carried address; and an answering
    run's way with a question: hold, answer, pass, or say it is a
    person's, which its source's own code then puts to them once, keyed
    by the place and the end, and whose answer that code carries back as
    the place's retry naming the key.
  - `guides/claude-code.mdx`: what SessionStart gives a session of the
    operator's own.
  - `reference/configuration.md`: a source's `roles`, its file's form
    and faults, and that the host's configuration names no role.
  - `reference/commands.md`: `needs --to`, `hold --waiting`, `retry
    --answers` and `tell --answers`.
  - `reference/state.md`: an end's key and answer, a stop-short's
    addressee, and a hold on the waiting move.

  True when `sluice exec main -- moon run site:prose site:build` passes.
- [ ] 10.2 Run the checks of the project this group touched. True when
  `sluice exec main -- moon run site:prose site:build root:format`
  passes.

## 11. Coverage and the gate

- [ ] 11.1 Have the suite's clause list read the statement at the
  revision of agentplot/blueprints main that holds record 0039's
  amendment, Part A with 193 and the scenarios to S118. Have 193, S93,
  S94, S98 and S99 cited by this change's scenarios, and nothing of this
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
