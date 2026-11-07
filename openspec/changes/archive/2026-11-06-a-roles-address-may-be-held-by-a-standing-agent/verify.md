# Verify: a-roles-address-may-be-held-by-a-standing-agent

Read on 2026-11-06 at `e16381e`, the head of the branch
`change/a-roles-address-may-be-held-by-a-standing-agent`. The check
covers the change's proposal, design, six delta specs and tasks. The
branch holds flume's main at `c591179` (#128), merged by task 1.1.

## Verdict

**Holds; archived.**

- Every task of groups 1 to 6 is ticked: 18 of 18.
- Every requirement of the six delta specs is built, and each of their
  36 scenarios is held by a test that asserts its THEN (below).
- `openspec validate a-roles-address-may-be-held-by-a-standing-agent
  --strict` passes.
- `sluice exec main` at `e16381e` passes all 23 of the check tasks the
  head affects, flume:tools 111 of 111 among them (below).
- The statement the change cites is on agentplot/blueprints main:
  `6d322be` and `1c89abc` are both ancestors of its main, and C.5's
  herdr table holds the standing row as the design gives it.

Two things stand beside the verdict, neither a shortfall of the change
against itself (below): flume's main has moved past `c591179`, and a
trial merge conflicts in four files; and the statement has moved past
`1c89abc`, its C.7 now reading a role's entry from a source's file.

## The tasks

All 18 are ticked, each by a commit that names its check:

- 1.1 by `51c4bdb`, after the merge of `c591179` at `44473ed`.
- 2.1 by `67990f2`, 2.2 to 2.4 by `1203bb0`.
- 3.1 by `7d1368a`, 3.2 by `840dd52`.
- 4.1 by `8488b46`, 4.2 by `1aa4283`, 4.3 by `fdc4998`, 4.4 by
  `5df46c5`.
- 5.1 and 5.4 by `59246fa`, 5.2 and 5.3 by `9f5bdbb`, 5.5 by `cd862be`.
- 6.1 by `09d1ec1`, 6.2 by `e16381e`.

## The specs, scenario by scenario

### role-waking

| Scenario | Held by |
|---|---|
| A standing agent's entry read | `crates/flume/src/host/config.rs` `a_standing_agents_entry_read`, beside `a_standing_agent_in_its_stream` and `the_real_host_gives_the_standing_agents` |
| A standing agent and a kind | `config.rs` `a_standing_agent_and_a_kind_are_refused`: "gives both a standing agent and a kind" |
| A run's key beside a standing agent | `config.rs` `a_runs_key_beside_a_standing_agent_is_refused`, over `repository`, `priority`, `base` and `brief`, naming each |
| A session with no agent | `config.rs` `a_session_with_no_agent_is_refused`: "session is no word of a role's run" |
| Ends pending while the agent is away | `engine/role_waking.rs` `ends_pending_while_the_agent_is_away`, citing S38 |
| A carrier wakes a standing address | `role_waking.rs` `a_carrier_wakes_a_standing_address` |
| A standing agent over a run still live | `role_waking.rs` `a_standing_agent_over_a_run_still_live` |
| A standing agent's entry becomes a run's | `role_waking.rs` `a_standing_entry_becomes_a_runs` |

The design's further refusals, an `agent` or `session` empty or not a
string, are held by `an_agent_or_a_session_empty_or_no_string_is_refused`,
and a local layer's `[roles]` not taken (core.45) by
`a_standing_agent_from_the_local_layer_is_not_taken`.

### end-delivery

| Scenario | Held by |
|---|---|
| A standing agent waiting | `engine/end_delivery.rs` `a_standing_agent_waiting` |
| A standing agent asking | `end_delivery.rs` `a_standing_agent_asking` |
| No agent of the name | `end_delivery.rs` `no_agent_of_the_name` |
| Two agents of the name | `end_delivery.rs` `two_agents_of_the_name` |
| An agent revived under its name | `end_delivery.rs` `an_agent_revived_under_its_name` |
| The session from the entry's stream | `end_delivery.rs` `the_session_from_the_entrys_stream` |
| The default stream's own session | `end_delivery.rs` `the_default_streams_own_session` |
| A stream the host does not declare | `end_delivery.rs` `a_stream_not_declared` |
| The inbox collects for a standing address | `end_delivery.rs` `the_inbox_collects_for_a_standing_address` |
| A standing agent under herdr through the binary | `crates/flume/tests/profiles/tools/walkthroughs.rs` `a_standing_agent_under_herdr_takes_a_fixs_end_through_the_binary`: one notice taken, the same at the next settle, one place on the host, and `flume wake w/planner` answered held |

### place-status

| Scenario | Held by |
|---|---|
| An agent found | `engine/place_status.rs` `a_standing_agent_found` |
| None of the name | `place_status.rs` `none_of_the_name` |
| More than one | `place_status.rs` `more_than_one_of_the_name` |
| A part not read | `place_status.rs` `a_part_not_read` |
| Nothing pending | `place_status.rs` `a_standing_address_with_nothing_pending` |
| Two addresses in one session | `place_status.rs` `two_addresses_in_one_session` |
| The fields kept | `engine/answers.rs` `FIELDS`, which gains `status.host.addresses[].standing` and its eleven paths and the `held` answer's seventeen; `every_answer_reads_back` over `full()`'s `held` and the held answer found none, several and unread; `an_address_before_standing` |

### seam-contracts

| Scenario | Held by |
|---|---|
| Standing and run apart | `engine/role_waking.rs` `standing_and_run_apart`, over the fake host |
| An agent by its name | `crates/flume-conformance/src/seams/terminal.rs`, the scenario "a part's agents are given by their names", run over the fake terminal and by the herdr harness's `named_agent`, a stand-in started by `herdr agent start <name>` |
| A standing address round-trips | `crates/flume-seams/src/round_trip.rs` `a_standing_address_round_trips`, `SessionFrom::Role` serialized `role` among it |

### seam-fakes

| Scenario | Held by |
|---|---|
| An entry set and taken away | `crates/flume-fakes/tests/fakes.rs` `an_entry_set_and_taken_away` |
| An agent added, removed and refused | `fakes.rs` `an_agent_added_removed_and_refused` |
| Two of one name | `fakes.rs` `two_of_one_name` |

### terminal-profiles

| Scenario | Held by |
|---|---|
| A standing agent found and reached | `tools/terminal_herdr.rs` `a_standing_agent_is_found_by_its_name_and_reached` |
| A session that does not run | `terminal_herdr.rs` `a_session_that_does_not_run_holds_no_agent_and_is_not_started`: `x` read as none, and neither running nor listed after |
| An agent with no name | `terminal_herdr.rs` `an_agent_herdr_lists_with_no_name_is_not_given` |
| The agent gone | `terminal_herdr.rs` `a_standing_agent_gone_is_not_given` |
| Agents under the printed command | `crates/flume/tests/profiles/terminal_printed.rs` `agents_under_the_printed_command_are_not_read_and_nothing_is_run` |

`terminal_herdr.rs` also holds a herdr that refuses or does not answer
read as unread in its words (`agents_in_of_a_herdr_that_refuses_or_does_not_answer_is_unread_in_its_words`).

## The code against the design

- **The reach** is `crates/flume-core/src/standing.rs`: the entry's
  stream, else `default`; a stream the host does not declare unread,
  "the host declares no stream …", with no terminal asked; the stream's
  terminal by `stream_terminal`; the entry's session from `role`, else
  `StreamRoom::own_session`; `agents_in` asked once per terminal and
  part in one delivery or one status; counted by name into one, none,
  several or unread (design 2, 3, 4; core.15, core.49).
- **Delivery** (`ends.rs`) reads the entries once per place's delivery,
  sends `send_to(at, notice)` only to the one agent working or waiting,
  and claims nothing otherwise. Nothing of `at` is written to held
  state (design 4; core.11, core.28).
- **The wake** (`wake.rs`): `wake_roles` passes over a standing address,
  recording nothing in `unwoken`; `Engine::wake` settles, launches
  nothing and answers `Answer::Held` (design 5; core.13).
- **Status** gives every standing address, pending or not, its
  `unwoken` none and its `role` unread (design 7).
- **herdr** (`profiles/terminal/herdr/mod.rs`): one bounded
  `session list --json`, a session not listed running read as none and
  no server started; `agent list` through the bounded `ask`; agents with
  a string `name` at `herdr:<pane_id>@<socket>`, their status mapped by
  the same `status_state` the state row uses; every call with `HERDR_*`
  stripped (design 8; core.29). The printed command keeps the seam's
  default.
- **The rules.** No module outside a profile names herdr (core.1;
  `f0e58ac` fixed the print tests that did). Record 0013 gains the
  construct's section and cites `6d322be` and `1c89abc`; core.28's
  details rule out a standing agent's pane kept and an agent sought in
  another terminal. The site names no host or team of ours (core.35),
  and the roadmap's item moves to what flume does.
- `crates/flume/tests/profiles/tools/open.rs` loses 134 lines: its
  short-link host under herdr moved to `walk.rs` as `UnderHerdr`, shared
  with the walkthrough, unchanged in what it does (core.51).

## The checks

`sluice exec main` at `e16381e` ran the gate's layers, `#check-fast`
and `#check-tools`, over the projects the branch affects. It passed 23
check tasks with none failing, in 5m 18s:

- `seams`, `fakes`, `core` and `conformance`: format, lint and test,
  and `conformance:coverage`.
- `flume`: format, lint, test (163, 207, 48, 19, 2 and 2 tests in its
  suites) and tools (111 of 111, the walkthrough
  `a_standing_agent_under_herdr_takes_a_fixs_end_through_the_binary`
  and the herdr profile's run of the terminal scenarios among them).
- `plugin-claude:test`, `site:prose`, `site:build`, `root:format`,
  `root:changes` and `root:adrs`.

sluice did not record the pass under its key. The worktree held this
report, not yet committed, beside the head when the run ended. The
merge queue runs the checks again at the submission in any case, since
main trusts only a confined or sandboxed run's record.

The archive at `4f67237` applies 13 requirements added across the six
specs. Over the specs it writes, `root:specs` passes 44 of 44,
`root:changes` 1 of 1 and `root:format` cleanly. They ran directly in
the devenv shell, since the host's sluice bound was full and their run
through sluice had waited 40 minutes for a turn. `openspec validate
--specs --strict` passes 44 of 44 too.

## Beside the verdict

### flume's main has moved

origin's main is at `5668b5d`, nine landings past `c591179`, #130
`change/a-lost-conversation-resumes-its-session` among them. A trial
merge (`git merge-tree`) of `e16381e` and `5668b5d` conflicts in four
files:

- `crates/flume-conformance/src/engine/end_delivery.rs`
- `crates/flume-conformance/src/engine/place_status.rs`
- `crates/flume/src/commands/print.rs`
- `crates/flume/tests/profiles/tools/walkthroughs.rs`

`ends.rs`, `answer.rs` and the rest merge without one, as design 11
foresaw for #130. No task of the change asks for main past `c591179`,
so the change holds against itself, and the merge queue reads the
conflict at the submission and sends it to the kind's stage for a
failed submission with its words (core.27). The path not taken was
stopping short to have the code stage merge main first: it is the same
stage the conflict reaches, and the archive's own lines in
`openspec/specs/` are part of what that merge fits.

### The statement has moved

agentplot/blueprints main has moved past `1c89abc`. At `3394602`,
"every question a place asks reaches someone who answers it", C.7 reads
a role's entry from a source's roles file (193), a host configuration
giving `[roles]` unreadable, and a source's standing entry gives
`agent`, `session`, `answers` and `answerer` but no `stream`, its
stream its address's. At `6dc9399` the terminal starts no settle and
says nothing of attended. Neither changes 43's or 132's delivery, the
reach, status, the wake's held answer, the terminal seam or C.5's
standing row, which this change builds.

flume's code still reads `[roles]` from the host's configuration for a
run as for a standing agent. Moving both to a source's file is the
change `every-question-a-place-asks-reaches-someone-who-answers-it`,
whose tasks at `773e8fa` move this change's standing entry from
`[roles]` to the roles file, its stream its address's, once this change
has landed, as this change's design 11 left it to do. So no clause is
worked around here (core.19).
