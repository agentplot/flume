# Verify: an-intents-affinity-is-kept-per-program

Checked on 2026-11-02 at `change/an-intents-affinity-is-kept-per-program`
d253e50. The branch was cut from main at f9a51c2 (#86), and main's
head, local and on `origin`, is still f9a51c2, so no merge was needed.
The check covers this change's proposal, design, five specs and tasks.

**The work falls short, and the change is not archived.** Every task
is ticked, and every requirement is built. Every one of the specs' 27
scenarios has a test, and the gate passes at d253e50 against main at f9a51c2. But the archive this stage
delivers fails the gate. Once synced, two of the change's requirements
are longer than openspec's strict check allows, so `root:specs`, a
`check-fast` task every branch requires, fails on the archive commit
(F1). The change goes back to its code stage to shorten or split them.

## What fails

- **F1. Two synced requirements fail `root:specs`.**
  `openspec archive an-intents-affinity-is-kept-per-program --yes` ran
  over a copy of `openspec/` at d253e50. It syncs the five specs. But
  `openspec validate --specs --strict`, which is `root:specs`, then
  passes only 32 of 34. Two specs fail on openspec's warning
  "Requirement text is very long (>500 characters)", which `--strict`
  reads as an error:
  - `account-pools`, requirements[4], "An intent keeps an account per
    program": about 740 characters as the change's MODIFIED block
    writes it.
  - `seam-contracts`, requirements[13], "The types of streams, pools
    and a start's record": about 590 characters, its last sentence on
    an affinity written before taking it over.

  The same check over main's `openspec/specs/` at f9a51c2 passes 34 of
  34, and none of main's requirements is over 500 characters. So the
  archive commit would fail the gate's `check-fast` layer on this
  branch and on main.

  The change's own check misses this. `openspec validate
  an-intents-affinity-is-kept-per-program --strict`, as `root:changes`
  runs it, passes with no warning: openspec 1.14.3 does not measure a
  delta's requirement text. So task 5.3 passes as written. It does not
  check the specs as the archive leaves them.

  Each requirement needs to be 500 characters or fewer, with its
  meaning kept. One way is to move part of each into a requirement of
  its own, an ADDED block whose scenarios move with it. For
  account-pools, that part could be what a document written before
  reads as. For seam-contracts, it could be the affinity written
  before. Then run the trial archive and its strict check again before
  the stage that archives.

## Tasks

All 17 tasks of the five groups are ticked, `[x]`, and none is open.
The gate run below backs each group's closing task (1.4, 2.5, 3.3, 4.2,
5.3), each of which names only the checks it ran. None of them names
the check F1 fails, which only the archived specs can fail.

## Checks

- **The gate.** `sluice exec main` at d253e50, with the worktree
  clean, ran the checks d253e50 affects against main at f9a51c2. It
  passed all 22 `#check-fast` and `#check-tools` tasks, none replayed,
  and recorded their results for d253e50:
  - `seams`, `core`, `conformance` and `flume`: format, lint and test;
  - `fakes`: lint and test;
  - `conformance:coverage`;
  - `flume:tools`, 72 of 72, among them
    `walkthroughs::a_pool_with_no_account_of_a_stages_program_refuses_it_at_its_launch_and_at_a_later_start`
    (S81);
  - `plugin-claude:test`;
  - `site:prose` and `site:build`;
  - `root:format`, `root:changes` and `root:adrs`.

  `root:specs` was not among them. Task 5.3 names it, but the branch
  changes nothing under `openspec/specs/`, so the gate does not count
  it as affected. Run by hand at d253e50, it passes 34 of 34 over
  main's specs. It is the check F1 fails once the archive changes those
  specs.
- **The archive.** The trial archive is F1. It writes the five specs
  and renames account-pools' "An intent keeps to its account" to "An
  intent keeps an account per program", with no error. Only the length
  check fails.
- **The statement.** The clause list reads a15d927, the revision of
  agentplot/blueprints main that states S80 and S81. Part A and Part B
  are as the clause-coverage spec gives them, with S1 to S81. The
  unbuilt set names nothing of this change. S80 is cited by
  `s80_two_programs_of_one_intent` and the place-status scenario of an
  intent that moved for one program. S81 is cited by
  `s81_a_pool_with_no_account_of_the_stages_program` and
  `a_launch_on_a_pool_of_another_programs_accounts`.

## Requirements and scenarios

Each test named below exists under its name at d253e50. I matched each
one to its scenario and read its assertions against the scenario's
WHEN and THEN.
Every test named here passes in the gate run above. The engine's
scenarios run inside `conformance:test`'s suite and `core:test`.

Short names: AP is `crates/flume-conformance/src/engine/account_pools.rs`,
PS `engine/place_status.rs`, SS `crates/flume-conformance/src/seams/store.rs`,
RT `crates/flume-seams/src/round_trip.rs`, SD
`crates/flume/tests/profiles/store_state_dir.rs`, PR
`crates/flume/src/commands/print.rs`, and WT
`crates/flume/tests/profiles/tools/walkthroughs.rs`.

| Spec | Scenarios | Tests | Met |
|---|---|---|---|
| account-pools | 15 | AP `s54_fill_first_and_least_busy`, `two_starts_and_one_accounts_last_room`, `an_account_for_another_program` (a pool of an `agent` account and an `other` one, the stage of `other` on the second), `nothing_usable`; `s80_two_programs_of_one_intent`, `a_start_on_another_program`, `a_document_written_before`, `a_document_written_before_whose_account_is_gone`, `s56_three_units_of_one_intent`, `never_back`, `dropped_an_hour_after_the_last_place`, `no_intent_named`; `a_launch_on_a_pool_of_another_programs_accounts`, `s81_a_pool_with_no_account_of_the_stages_program`, `a_stage_waiting_when_its_account_is_dropped`; WT `a_pool_with_no_account_of_a_stages_program_refuses_it_at_its_launch_and_at_a_later_start` | yes (F1) |
| place-status | 3 | PS `a_stage_that_moved` (its intent's move now naming `agent`), `an_intent_that_moved_for_one_program` (JSON form); PR `an_agents_account` ("from the affinity for other"), `an_intents_move_for_its_program_and_one_recorded_before`; the start without its settings as before, `start_integrity.rs`'s `a_variable_missing` and the scenarios beside it | yes |
| seam-contracts | 3 | RT `a_held_state_with_a_stream_an_account_and_a_record` (its intent's move naming `agent`), `an_affinity_of_two_programs`, `an_affinity_written_before`; SS `the_hosts_own_documents` | yes (F1) |
| store-profile | 3 | SD `two_processes_record_one_account_spent`, `a_spent_time_is_seen_by_another_process_and_kept_from_other_users`, `an_intents_document_an_earlier_release_wrote` | yes |
| clause-coverage | 3 | `clauses.rs`'s `the_clauses_listed` and its unbuilt list; `coverage.rs`'s `a_clause_left_out_is_named`; `conformance:coverage` | yes |

The specs marked F1 are met by the code. F1 is about how their
requirement text is written, not about what the code does.

## Design and rules

- **Decision 1.** `Affinity` holds `accounts` keyed by the program,
  `AffinityAccount`, and `pool` and `account` read with a default and
  skipped where none. `Affinity::left(program, account)` counts only an
  intent's move for that program. `MoveOn::Intent { program }` is
  skipped where none.
- **Decision 2.** `Engine::affinity` folds a document written before
  through `folded`. It finds the account's program in the pool the
  host's configuration declares now, keeps it unless that program's
  entry stands, and gives the program to each intent move that names
  none. It drops the account where the pool no longer declares it.
  `write_affinity` compares the folded document with what `change`
  makes, so a start that changes nothing writes nothing (core.17). The
  next change writes the new shape.
- **Decision 3.** `choose_account` takes the account for the stage's
  program in the stage's pool. It records the intent's move, naming the
  program, only where that account is spent or full and the affinity
  for that program never left the account chosen. `affinity_after`
  writes the first account on a program, a move of that program's
  entry, and nothing of another program's. `claim` passes the program
  it resolved to both.
- **Decisions 4 and 5.** `Choice::NoAccount` and `no_account`'s words,
  which name the stage, the pool and the program, are shared by the
  claim and the launch. The claim refuses as it refuses
  `Choice::Undeclared`. `RoomWanted::Program` is gone. `Queue::runs`
  and `PoolRoom::runs` read whether the pool holds an account of the
  program. `start_waiting` claims a waiting stage that has none ahead
  of the host's room, and `must_wait` passes over it. `make_place`
  resolves every stage first, so a program no profile binds is refused
  first, then calls `pool_runs` before anything is made (core.8,
  core.30).
- **Decision 6.** The JSON form gains `program` on an intent's move
  (core.14). The person's form prints "(from the affinity for
  <program>)" and "the intent moved for <program> from …". A move with
  no program reads as before.
- **Decision 7.** Record 0020's second amendment reads accepted by the
  user on 2026-11-02, on their Pending You card req_f861af4d, "Fix
  both". It gives the deciders, the sources, both sets of options, why
  the others were not taken, and the consequences. The status line
  names both amendments, and nothing the decision or the carry's
  amendment wrote is changed. core.32's rule and details, and core.26's
  and core.30's details, are amended as design 7 says. The site's
  `rules.md` gives core.32's rule word for word as `core.md` does.
  `_open.md` holds no item on this build, which 5.2 removed. Record
  0026's item gives its build of 173 the halves of 145 and 147 and
  S80's walkthrough.
- **Decision 9 and core.35.** The guide's "A pool that cannot run a
  stage" and "An intent keeps an account per program", the state
  reference's `accounts`, `program` and queue, and the configuration
  reference's account `program` say what the design says. Every link
  to the old anchor follows the new one, and none of the words core.35
  rules out is in the site's diff.
- **core.1.** The core and the suite name no program. The engine's
  scenarios use the fake's `agent` and `other`. Only the walkthroughs
  under `tests/profiles/` name `claude` and `codex`.

## Outside the change

- **`root:changes` does not catch a requirement too long to archive.**
  The `changes` task's comment in `moon.yml` says a change's strict
  validation fails "a requirement archive would refuse" in the change's
  own checks, not first at its merge. openspec 1.14.3 does not measure
  a delta's requirement text, so a change can pass `root:changes`, and
  every check its tasks name, and still fail `root:specs` once archived,
  as F1 shows. A later change to the gate could add a trial archive of
  each open change, with the strict check over its result, as a task.
  This bears on every change, not only this one.
