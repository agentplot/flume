# Verify: an-intents-affinity-is-kept-per-program

Checked on 2026-11-02 at `change/an-intents-affinity-is-kept-per-program`
050ad0c. The branch was cut from main at f9a51c2 (#86), and main's
head on `origin` is still f9a51c2, so no merge was needed. The check
covers this change's proposal, design, five specs and tasks.

**The work holds, and the change is archived.** Every task is ticked,
and every requirement is built. Each of the specs' 27 scenarios is held
by a test. The gate passes at 050ad0c against main at f9a51c2,
all 22 of its tasks run and none replayed. A trial archive over a copy of `openspec/` leaves
`openspec validate --specs --strict`, which is `root:specs`, passing 34
of 34. No synced requirement is over 450 characters, and every scenario
main holds is kept.

## Tasks

All 18 tasks of the five groups are ticked, `[x]`, and none is open.
The gate run below backs each group's closing task (1.4, 2.5, 3.3, 4.2,
5.3). Task 5.4 split account-pools' "An intent keeps an account per
program" and seam-contracts' "The types of streams, pools and a start's
record" under the strict check's 500 characters, and the trial archive
below backs it.

Every sentence of the two requirements is kept, in its modified block
or in one of three ADDED requirements:

- account-pools' "An intent moves for one program alone", with S80's
  scenario and the start on another program's;
- account-pools' "An intent's document written before reads as its
  account's program's", with its two scenarios;
- seam-contracts' "An affinity written before reads with its one
  account", with its scenario.

## Checks

- **The gate.** `sluice exec main` at 050ad0c ran the checks 050ad0c
  affects against main at f9a51c2. It passed all 22 `#check-fast` and
  `#check-tools` tasks, and replayed none:
  - `seams`, `core`, `conformance` and `flume`: format, lint and test;
  - `fakes`: lint and test;
  - `conformance:coverage`;
  - `flume:tools`, 72 of 72, among them
    `walkthroughs::a_pool_with_no_account_of_a_stages_program_refuses_it_at_its_launch_and_at_a_later_start`
    (S81);
  - `plugin-claude:test`;
  - `site:prose` and `site:build`;
  - `root:format`, `root:changes` and `root:adrs`.

  sluice did not record these passes for 050ad0c, because by the end
  of the run the worktree held this report and `_open.md`'s new item.
  `root:specs` was not among the tasks, since 050ad0c changes nothing
  under `openspec/specs/`. The trial archive below runs it over the
  specs as the archive leaves them.
- **The archive.** `openspec archive an-intents-affinity-is-kept-per-program --yes`
  (openspec 1.14.3) over a copy of `openspec/` at 050ad0c syncs the
  five specs with no error. It adds 4 requirements, modifies 7 and
  renames 1: account-pools' "An intent keeps to its account" becomes
  "An intent keeps an account per program". The strict check over the
  specs it leaves passes 34 of 34. Every scenario heading main's five
  specs hold is in the synced specs. The counts go from 16 to 23 in
  account-pools, 22 to 23 in place-status, 31 to 33 in seam-contracts,
  10 to 11 in store-profile, and stay 5 in clause-coverage.
- **The statement.** The clause list reads a15d927, the revision of
  agentplot/blueprints main that states S80 and S81. Part A and Part B
  are as the clause-coverage spec gives them, with S1 to S81. The
  unbuilt set names nothing of this change. S80 is cited by
  `s80_two_programs_of_one_intent` and the place-status scenario of an
  intent that moved for one program. S81 is cited by
  `s81_a_pool_with_no_account_of_the_stages_program` and
  `a_launch_on_a_pool_of_another_programs_accounts`.

## Requirements and scenarios

Each test named below exists under its name at 050ad0c. I read each
one's assertions against its scenario's WHEN and THEN. The code and
tests are as they stood at d253e50. Every commit since then touches
only this change's directory. The engine's scenarios run inside
`conformance:test`'s suite and `core:test`.

Short names: AP is `crates/flume-conformance/src/engine/account_pools.rs`,
PS `engine/place_status.rs`, SS `crates/flume-conformance/src/seams/store.rs`,
RT `crates/flume-seams/src/round_trip.rs`, SD
`crates/flume/tests/profiles/store_state_dir.rs`, PR
`crates/flume/src/commands/print.rs`, and WT
`crates/flume/tests/profiles/tools/walkthroughs.rs`.

| Spec | Requirement | Scenarios | Tests | Met |
|---|---|---|---|---|
| account-pools | An account is chosen under the room lock | 4 | AP `s54_fill_first_and_least_busy`, `two_starts_and_one_accounts_last_room`, `an_account_for_another_program` (a pool of an `agent` account and an `other` one, the stage of `other` on the second), `nothing_usable` | yes |
| account-pools | An intent keeps an account per program | 4 | AP `s56_three_units_of_one_intent`, `never_back`, `dropped_an_hour_after_the_last_place`, `no_intent_named`, each reading the affinity's account for `agent` | yes |
| account-pools | An intent moves for one program alone | 2 | AP `s80_two_programs_of_one_intent`, `a_start_on_another_program` | yes |
| account-pools | An intent's document written before reads as its account's program's | 2 | AP `a_document_written_before`, `a_document_written_before_whose_account_is_gone` | yes |
| account-pools | A pool that holds no account of a stage's program refuses it | 3 | AP `a_launch_on_a_pool_of_another_programs_accounts`, `s81_a_pool_with_no_account_of_the_stages_program`, `a_stage_waiting_when_its_account_is_dropped`; WT `a_pool_with_no_account_of_a_stages_program_refuses_it_at_its_launch_and_at_a_later_start` | yes |
| place-status | Status gives a place's stream, intent, account and moves | 3 | PS `a_stage_that_moved` (its intent's move now naming `agent`), `an_intent_that_moved_for_one_program` (JSON form); PR `an_agents_account` ("from the affinity for other"), `an_intents_move_for_its_program_and_one_recorded_before`; the start without its settings as before, `start_integrity.rs`'s `a_variable_missing` and the scenarios beside it | yes |
| seam-contracts | The types of streams, pools and a start's record | 2 | RT `a_held_state_with_a_stream_an_account_and_a_record` (its intent's move naming `agent`), `an_affinity_of_two_programs`; SS `the_hosts_own_documents` | yes |
| seam-contracts | An affinity written before reads with its one account | 1 | RT `an_affinity_written_before` | yes |
| store-profile | The host's own documents | 3 | SD `two_processes_record_one_account_spent`, `a_spent_time_is_seen_by_another_process_and_kept_from_other_users`, `an_intents_document_an_earlier_release_wrote` | yes |
| clause-coverage | The suite knows the statement's clauses; A clause cited by nothing fails the suite | 3 | `clauses.rs`'s `the_clauses_listed` and its unbuilt list; `coverage.rs`'s `a_clause_left_out_is_named`; `conformance:coverage` | yes |

## Design and rules

- **Decision 1.** `Affinity` holds `accounts` keyed by the program,
  `AffinityAccount`, and `pool` and `account` read with a default and
  skipped where none. `Affinity::left(program, account)` counts only an
  intent's move for that program. `MoveOn::Intent { program }` is
  skipped where none.
- **Decision 2.** `Engine::affinity` folds a document written before
  through `folded`. It finds the account's program in the pool the
  host's configuration declares now and keeps it unless that program's
  entry stands. It gives the program to each intent move that names
  none, and drops the account where the pool no longer declares it.
  The next change writes the new shape.
- **Decision 3.** `choose_account` takes the account for the stage's
  program in the stage's pool. It records the intent's move, naming the
  program, only where that account is spent or full and the affinity
  for that program never left the account chosen. `affinity_after`
  writes the first account on a program and a move of that program's
  entry, and leaves every other program's entry. `claim` passes the
  program it resolved to both.
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
  both". It has its context, options, outcome, why not the others and
  consequences, and the status line names both amendments. core.32's
  rule and details, and core.26's and core.30's details, are amended as
  design 7 says. The site's `rules.md` gives core.32's rule word for
  word as `core.md` does. `_open.md` holds no item on this build, which
  5.2 removed. Record 0026's item gives its build of 173 the halves of
  145 and 147 and S80's walkthrough.
- **Decision 9 and core.35.** The guide, the state reference and the
  configuration reference say what the design says. None of the words
  core.35 rules out is in the site's diff.
- **core.1.** The core and the suite name no program. The engine's
  scenarios use the fake's `agent` and `other`. Only the walkthroughs
  under `tests/profiles/` name `claude` and `codex`.

## Outside the change

- **`root:changes` passes a requirement too long to archive.**
  openspec 1.14.3 measures no delta's requirement text. So a change can
  pass `root:changes`, and every check its tasks name, and still fail
  `root:specs` once archived. This change's task 5.4 fixed that for its
  own specs. The gap bears on every change, and `docs/architecture/_open.md`
  now holds it as an item.
