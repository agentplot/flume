# Open

Decisions waiting, and code that stands against a rule. An item leaves
when the decision lands in the statement or a record, or the code is
brought to the rule.

- **Two of the statement's section 10 questions are unanswered.** They
  wait on flume's reconstruction and the operator's ruling: a
  repository's own deliverable forms (24); a kind that lands only part
  of its work (67, 111). How long a hold may wait (47) is record 0008's;
  how a caller learns an end (43, 46) is record 0009's, accepted on 2026-10-28; a verify stage's finding (42, 112) is record 0011's, accepted on
  2026-10-28: the finding sends the place back to the kind's fix round
  by itself. The other four were ruled on
  2026-10-26, in the section. None is a rule's source until ruled.
- **Part C's readings of herdr 0.9.5 and sluice 0.55.0 wait on the
  operator's ruling.** The commands, state names, error codes and
  fields the herdr, printed-command and sluice profiles read, and what
  herdr and sluice were seen to do (herdr's `done` only after a prompted
  turn on an unfocused pane, an agent left listed `unknown` after it
  exits; a conflict found before testing with no `tested`, a failure's
  `notice` naming its check), are written into C.5 and C.6 of
  agentplot/blueprints `design/flume/requirements.md` at 4cb8c59, a change the
  operator rules (Part C's preamble; core.19). The item leaves when it
  lands.
- **Clause 117 has submit answered before the merge queue decides.**
  sluice 0.57.1, on a base that takes pull requests, can answer a
  no-wait submit with the record of an entry it found decided as it
  took it in, exiting as the outcome does, and records the outcome
  delivered to the place, so no inbox gives it (133). The merge-queue
  seam's submit answers such a submission with its outcome, which the
  settle that submitted acts on as it acts on the inbox's (112). The
  code stands ahead of 117, C.6 and core.6, which reads an outcome
  only at the settles that follow; they say so once the operator rules
  the change (core.19). The item leaves when it lands.
- **A pull request's entry and outcome are read where C.6 does not
  say.** sluice 0.57.1, on a base that takes pull requests, answers a
  submit whose request intake has not taken in with `{pull_request,
  branch, state}` and no entry, records the entry intake takes in
  later from `pull/<n>` with the place's branch as its `from.head`, and
  delivers its outcome to the request rather than the place. The
  merge-queue seam's submit answers such a submission as the request
  held; read-submission takes an entry submitted from the branch by a
  worktree or from a pull request opened from it; and the place's
  entry taken in from its request has its landing, failure or conflict
  read from read-submission as well as the inbox, acted on once. The
  code stands ahead of 111 and 133, which settle a place by the
  inbox's outcome alone, and of C.6's table of what the profile reads,
  which names `from.worktree` alone; they say so once the operator
  rules the change (core.19). The item leaves when it lands.
- **C.1 makes every repository's places at `<repository>/places/`.**
  Record 0027, proposed on 2026-10-31 on the operator's brief, makes a
  clone's places under flume's own data directory, since a clone's
  directory is its main worktree, and keeps a bare repository's beside
  its worktrees (core.40). The code stands ahead of C.1's make column,
  which says so once the operator rules the change in agentplot/blueprints
  `design/flume/requirements.md` (core.19). The item leaves when it
  lands.
- **A dialog before the prompt waits on a person, and a retry leaves
  an agent whose stage began.** On deneb, 2026-10-31, a conversation's
  Claude Code stopped at its dialog approving a repository's MCP
  server; flume read the start never begun, a person answered the
  dialog and the agent began, and the operator's retry ended that
  working session. A start the terminal reads held at its program's
  question, at the open or at a settle before the agent takes its
  prompt, now stays starting with the question's words, the place
  waiting on a person's answer (`waiting_on` `answer`, `dialog`), never
  read never begun while the dialog shows. A stage read never begun
  whose live agent then acks its start, before any sending, or its
  prompt, goes on started or begun. A retry is refused, forced or not,
  while the place's agent is live and its stage started or began,
  naming the agent and `flume stop`. The code stands ahead of 35 and
  C.4's paragraph after its table, which read a start held at a
  question never begun at once, of S40 and C.5's herdr rows, of 50,
  which has a retry end a quiet agent, and of core.12's details; they
  say so once the operator rules the change (core.19). The item leaves
  when it lands.
- **Record 0021 waits on a plan.** The standing prompt with cache use
  in status (record 0021; core.33), accepted 2026-10-29 and stated at
  blueprints 6e5aae2, is built only once the user approves a plan for
  it. Until then the code stands against core.33. The item leaves when
  the code is brought to it.
- **weir's crate does not yet read a stage's `standing`.** Record 0021
  makes it a field of the kind format (weir 53, C.4, at blueprints
  6e5aae2), and flume reads it only through weir-kinds (core.4). The
  item leaves when weir-kinds reads it and flume's pin moves.
- **Record 0022 waits on a plan.** Kinds named by source, name and
  version, versions side by side, the repository's and the host's
  locks with a content hash per kind, packages held under the host's
  kinds directory and `flume kinds lock` (record 0022; core.34),
  accepted 2026-10-29 and stated at blueprints 57a1ba9 (154 to 160;
  weir 88 to 93), are built only once the user approves a build plan
  for them, weir-kinds' packages, identities, hash and locks first,
  since the format is weir's (core.4). Until then flume runs no kind
  from outside, and a launch names a kind by its name, or by its name
  and version, never by its source. The item leaves
  when the code is brought to core.34.
- **Records 0024 and 0025 wait on their build.** An agent's mode as
  a layered setting and a plan stage accepted by its caller's finish
  (record 0024; core.10, core.12 and core.30 as amended, core.37), and
  a place's points with a stream's points bound and pace (record 0025;
  core.26 as amended, core.38), stated at blueprints 9c7490c (161 to
  168; weir 54, 57), were accepted on 2026-10-30. The mode is built
  first, before the crew refactor, once the user approves its plan;
  the plan stage and points come after it. Until then the code stands
  against core.37 and core.38 and against those amendments. The item
  leaves when the code is brought to them.
- **Record 0026 waits on its build.** Every program herdr starts runs
  under an agent profile, its own or the common one, each part of the
  agent seam built or plainly degraded (record 0026; core.1's details,
  core.10 and core.30 as amended, core.39), and record 0024's modes are
  flume's own four, each profile mapping them or refusing, stated at
  blueprints a11f6d5 (169 to 174) and accepted on 2026-10-30. Until it
  is built `profiles.rs` binds Claude Code's profile alone and refuses
  every other program, against core.39. The common profile is built
  with entries for Codex, Gemini CLI, GitHub Copilot CLI and OpenCode,
  then Codex's own profile. The item leaves when the code is brought
  to core.39.
- **The command line still says `<partition>/<role>`.** `flume wake
  --help` and the refusals of a name that is no address, in
  `commands/mod.rs`, `host/config.rs` and `flume-core`'s `wake.rs`,
  name a role's address `<partition>/<role>`, a word core.35 rules out;
  the site says `<group>/<role>`. The change touches the `flume` crate,
  so its gate runs the herdr profile's tests, which open a pseudo-terminal
  that sluice 0.56.0's sandbox refuses (sluice #289 allows it). The item
  leaves when the strings say `<group>/<role>`.
