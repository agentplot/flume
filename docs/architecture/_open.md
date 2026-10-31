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
- **Records 0024 and 0025 wait on the user's yes.** An agent's mode as
  a layered setting and a plan stage accepted by its caller's finish
  (record 0024; core.10, core.12 and core.30 as amended, core.37), and
  a place's points with a stream's points bound and pace (record 0025;
  core.26 as amended, core.38), are stated at blueprints 9c7490c (161
  to 168; weir 54, 57) and proposed on flume-sluice-build's card. Once
  accepted, the mode is built first, before the crew refactor, and the
  plan stage and points after it. The item leaves when both are
  accepted or rejected.
- **The statement reads Claude Code's mode at SessionStart.** C.4's
  started and received rows and 161's word that it started, at
  agentplot/blueprints 5caff76, have the SessionStart hook compare the
  `permission_mode` "every hook's input carries in 2.1.296". That
  release gives a SessionStart input none, so the Claude Code profile
  compares the mode at the start's first prompt, blocking it where it
  differs, as the user ruled on 2026-10-31 (design 7 of the change
  agent-mode-is-a-layered-setting; core.10). The item leaves when the
  statement says so (core.19).
- **Record 0026 waits on the user's yes.** Every program herdr starts
  runs under an agent profile, its own or the common one, each part of
  the agent seam built or plainly degraded (record 0026; core.1's
  details, core.10 and core.30 as amended, core.39), and record 0024's
  modes are flume's own four, each profile mapping them or refusing,
  stated at blueprints a11f6d5 (169 to 174). Until then `profiles.rs`
  binds Claude Code's profile alone and refuses every other program.
  Once accepted, the common profile is built with entries for Codex,
  Gemini CLI, GitHub Copilot CLI and OpenCode, then Codex's own
  profile. The item leaves when it is accepted or rejected.
- **The command line still says `<partition>/<role>`.** `flume wake
  --help` and the refusals of a name that is no address, in
  `commands/mod.rs`, `host/config.rs` and `flume-core`'s `wake.rs`,
  name a role's address `<partition>/<role>`, a word core.35 rules out;
  the site says `<group>/<role>`. The change touches the `flume` crate,
  so its gate runs the herdr profile's tests, which open a pseudo-terminal
  that sluice 0.56.0's sandbox refuses (sluice #289 allows it). The item
  leaves when the strings say `<group>/<role>`.
