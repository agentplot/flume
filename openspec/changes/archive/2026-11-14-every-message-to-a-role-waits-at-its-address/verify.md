# Verify: every-message-to-a-role-waits-at-its-address

Verified 2026-11-14 on `change/every-message-to-a-role-waits-at-its-address`
at aa8d2c5f, cut from `bolt/flumes-roles-sleep-and-wake` at 2d180f0b,
which is still the base's head. The code is as it stood at 866aaca7:
the commits since change only the change's delta specs, its tasks and
this report.

## The whole checks: pass

The gate's checks ran once, with the host's sluice 0.93.2, as the
merge queue gates:
`devenv shell -- /etc/profiles/per-user/chuck/bin/sluice exec bolt/flumes-roles-sleep-and-wake`.
The sluice the tree pins, 0.71.0, cannot read the host's sluice
configuration, whose `[forge]` gives `credential`; ops answered that
the checks run with the host's sluice until a fix of main moves
`sluiceVersion` to 0.93.2 for the bolt to take in. As a place on a
bolt, it ran the bolt's `check-fast` against 2d180f0: 40 tasks
completed, 39 checks passed and none failed, and sluice recorded the
39 results under their keys for aa8d2c5. The run exited 0. moon warned
that it could not archive each task's outputs into `.moon/cache`,
which the sandbox does not let it write; no check reads that archive.

Among them, `root:specs` passes 49 specs and `root:changes` the open
changes; `conformance:test` runs
`no_tool::no_tool_is_named_outside_a_profile` (core.1) and
`the_engine_passes_its_scenarios`; `fakes:test` runs
`the_terminal_holds_recipients_sessions_by_name` and `two_of_one_name`;
and `flume:test` runs the MCP tests `status_over_mcp`,
`the_mcp_tools_listed`, `message_over_mcp`,
`launch_over_mcp_refused_as_the_command_line_refuses_it` and
`a_role_runs_launch_over_mcp_reports_to_its_address`, and the three
`hooks_claude::hook_deliver_*` tests. Every one passes.

## The archive

Archived in a copy of `openspec/` by `openspec archive
every-message-to-a-role-waits-at-its-address --yes`, the change applies
26 requirements added, 20 modified, 1 removed and 3 renamed, and
`openspec validate --specs --strict --no-interactive`, as `root:specs`
runs it, passes all 49 specs with none failing. Every requirement the
delta specs give is 500 characters or fewer: caller-acts's "An answer
names the stop-short it answers by its key" and end-delivery's "The
settle delivers each end once to each recipient" are cut, the batch
that left the second held by end-delivery's ADDED "A recipient takes
every delivery pending for it as one prompt" and "The deliver part
takes the place of the terminal"; and claude-code-plugin's "The MCP
tools take the command line's words", place-status's "Status gives a
role's messages and the places reporting to it", role-waking's "A
role's run's first prompt carries its batch" and terminal-profiles's
"herdr types only into a pane that waits" each hold their own
scenarios, herdr's "reaches a recipient by pane and socket" left as
the base gives it.

## Tasks

Every task of `tasks.md`, 1.1 to 9.1 with 1.3, 4.7 and 7.2, is ticked
done, 37 in all. 1.3 holds as the archive above shows. Every test a
task names exists under the name it gives, and `openspec validate
every-message-to-a-role-waits-at-its-address --strict` passes.

## Requirements and scenarios

Each scenario of the delta specs maps to the test that covers it. A
quoted name is the engine's or the seams' scenario, run by
`the_engine_passes_its_scenarios` or the seams' suite. Scenarios that
this change only reworded from tell to message keep the test they had
before.

### agent-profile

| Scenario | Covered by |
|---|---|
| The pinned release | `profiles::agent::claude::tests::deliver_is_not_offered_at_the_pinned_release` |
| A profile that offers it | `…::the_command_names_flume_deliver_only_where_deliver_is_offered` |
| A message waiting for the take | `hooks_claude::hook_deliver_prints_the_message_waiting` |
| Another start's secret | `hooks_claude::hook_deliver_prints_nothing_for_another_secret` |
| An agent asking | `hooks_claude::hook_deliver_prints_nothing_while_the_agent_asks` |

### caller-acts

| Scenario | Covered by |
|---|---|
| A message into a question, then into work | "message: refused into a question with nothing typed; given once the agent waits" |
| A message once the agent is let go | "answer: a message to a place whose code waits on an answer once its agent was let go is refused …" |
| A message taken by the deliver part | "message: the fake agent's profile offers deliver and a caller messages its place while it works …" |
| A message the deliver part never takes | "message: … with a bound of a minute and the agent works past it …" |
| A message taken and not received | "message: … takes a message and reads it not received within 60 seconds …" |
| A message to a role with no run live | "message: the pane ops messaging east/planner, whose source gives it a run and none is live …" |
| A message to an address no source declares | "message: a message to east/nobody, which no source declares …" |
| A bound with an address | "message: a message to east/planner with a bound of a minute is refused …" |
| An address and a place | `command_line::a_target_names_an_address_or_a_place` |
| A tell's answer, A message's answer | `command_line::a_tell_naming_its_key`, `a_message_naming_its_key`; "answer: a message of notes.md …" |
| An answer taken before its session ends | "answer: a code agent kept open that takes a message's answer, commits and ends its session …" |
| An answer the agent does not take | "answer: one given while the asking agent asks a person in its pane …" |
| A tell naming an answered key | "answer: a message from the pane ops naming the same key …" |
| The other modified scenarios | kept from before, their words moved from tell to message |

### claude-code-plugin

| Scenario | Covered by |
|---|---|
| status over MCP | `commands::status_over_mcp`, kept |
| The tools listed | `commands::the_mcp_tools_listed`; `mcp.rs` gives `message` `target`, `words`, `within` and `answers`, `launch` the command's long names, arrays where the flag repeats, `wake` `address`, and marks `status` alone read-only |
| message over MCP | `commands::message_over_mcp` |
| launch over MCP, refused as the command line refuses it | `commands::launch_over_mcp_refused_as_the_command_line_refuses_it` |
| A later prompt and a compaction after the mode moved | kept from before |
| Words at a turn's end | `deliver.test.ts`: "words at a turn's end are submitted once as the person's own, the prompt box untouched" |
| Nothing to deliver | `deliver.test.ts`: "nothing printed, or a run that exits 1 or fails, submits nothing" |
| Idle ticks | `deliver.test.ts`: "idle ticks run the hook every ten seconds, and none while a turn runs" |
| A session without the variable | `deliver.test.ts`: the two cases of "a session naming … asks no clock for deliveries and runs nothing" |
| The plugin written for an agent | `profiles::agent::claude::tests::the_plugin_written_carries_its_module_naming_the_binary`; `plugin-claude:validate` |

`hooks/hooks.json` names `./flume.ts` as its one module, and
`hooks/line.ts` is gone.

### command-line

| Scenario | Covered by |
|---|---|
| A tell's key | `command_line::a_tell_naming_its_key` |
| A message's key | `command_line::a_message_naming_its_key` |
| A caller not yet moved | `command_line::tell_runs_as_message` |
| Through the binary | `command_line::a_message_to_an_address_answers_with_its_key` |
| The other modified scenarios | kept from before; the helps test renamed `the_helps_of_needs_hold_retry_message_and_launch` |

### deliveries

| Scenario | Covered by |
|---|---|
| A message recorded | "message: on a host whose ends_expire is 48h, the pane ops messaging east/planner records one weir-delivery/1 …" |
| A message from no one the command names | "message: a command that reads no recipient of its own messaging east/planner …" |
| A claim round-trips | `round_trip::a_deliveries_document_with_a_claim_round_trips` |
| weir's pack over a document with claims | "store: flume's store of deliveries passes every delivery case of weir's pack, its deliveries document holding a claim …" |
| Confirmed, then let go of | "message: a message at east/planner handed to the address, confirmed by it, and the host settled is let go of …" |
| Expired and kept | "message: a message at east/planner past its expiry and never confirmed stays pending …" |

### end-delivery

| Scenario | Covered by |
|---|---|
| A recipient at work | "end delivery: no notice is sent into a recipient asking a question, read unknown or at work …"; "batch: an end read while its recipient's session works …" |
| A message collected | "message: a message pending at east/planner with an end of a fix, flume inbox --recipient east/planner …" |
| A message to a standing agent at the watch's settle | "batch: a message to madswan/planner, whose standing agent works …" |
| Two messages and an end while a role works | "batch: two messages and a fix's end for east/ops come while its run's agent works …"; walkthrough `s126_a_role_at_work_takes_two_messages_and_an_end_as_one_prompt` |
| Something handed before | "batch: an end handed to madswan/ops by its inbox, then a message for it …" |
| Two settles at once | "batch: two settles reading ops's batch at once …" |
| A send refused | "batch: a send of ops's batch the terminal refuses …" |
| A settle ended mid-send | "batch: a claim of ops's end left sending …" |
| A run whose agent takes its batch | "batch: the fake agent of east/ops's run offers deliver …" |
| The other modified scenarios | kept from before |

### herdr-plugin

| Scenario | Covered by |
|---|---|
| Retry from the sidebar | `run.sh`: "keys: retry from the sidebar, one key and the words as lines", kept |
| Message, stop and open, each one key | `run.sh`: "keys: message, stop with the force asked, and open, each one key"; "keys: message with no words runs nothing" |

`places.sh` binds `m` to `flume message` and lists `m message` in its
line of keys.

### place-status

| Scenario | Covered by |
|---|---|
| An address that wakes nothing | kept from before |
| A role's messages and the places reporting to it; A place whose work is over | "message: status gives east/ops, whose run two working fixes report to …; once one lands and is tidied, reporting names the other alone …"; walkthrough `s128_…` |
| Two answers in one session | "status: a fix whose code took one answer by a retry and one by a message …" |

### role-waking

| Scenario | Covered by |
|---|---|
| A message wakes the role | "message: a message to madswan/planner while no planner run is live or waiting launches one planner run by its own act …" |
| A run not yet given its prompt | "role waking: an end for east/planner whose run's agent waits in its session before its start took its prompt …" |
| What came after the prompt | "batch: an end for east/planner that comes after its run's prompt was filled …" |
| The first prompt carries the batch | "batch: a message and a fix's end pending for east/planner when its run's start fills its prompt …" |
| A start that never began | "batch: a run's start carrying a batch, read never begun …" |
| The run starts | "role waking: a run's inbox with no recipient named, confirming on receipt, collects every end and every message …" |
| A message while the run waits on its answer | "batch: a message for east/ops while its run's plan ended short needing an answer …" |
| A role's run launches its project | "message: the run of east/ops launches a fix naming no recipient …" |
| Through MCP | `commands::a_role_runs_launch_over_mcp_reports_to_its_address`; walkthrough `s128_a_roles_run_launches_two_places_that_report_to_its_address` |
| The other modified scenarios | kept from before |

### seam-contracts

| Scenario | Covered by |
|---|---|
| A session by its name | kept from before |
| A session at work | "terminal: send_to refuses a session that reads working, typing nothing …", over the fake, herdr and Rex |
| Two answers; State written before | kept from before, one answer written by `message` |
| An answer by tell | `round_trip::an_answer_by_tell_reads_as_a_message` |
| The fake agent's deliver | `round_trip::a_start_with_messages_waiting_round_trips` |
| A profile that names no deliver | `round_trip::a_profile_that_names_no_deliver_offers_none` |

### stage-runs

Both scenarios are kept from before, with "a message wakes it" where
they said a tell.

### terminal-profiles

| Scenario | Covered by |
|---|---|
| A recipient at work (herdr) | `terminal_herdr::a_recipient_at_work_is_typed_nothing`; herdr's `send_to` types only into a pane it reads waiting, refusing working, asking and unknown before any `agent send-keys` or `agent prompt` |
| A recipient's block at work (Rex) | `terminal_rex::a_recipient_block_at_work_is_typed_nothing` |
| The other modified scenarios | kept from before |

## Outcome

The whole checks pass on the branch as it stands, every task is ticked,
every scenario of the delta specs has a test, and the change archived
gives specs that pass `root:specs`. The walkthroughs `s126_…` and
`s128_…` are `check-tools`'s, which a place on a bolt leaves to the
bolt's merge onto main; each scenario they cover also has an engine
scenario or a commands test that passed here. The change is archived
with this report.
