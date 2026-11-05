# Verify: a-pool-chooses-the-seat-with-the-most-quota-left

Checked on 2026-11-05 at
`change/a-pool-chooses-the-seat-with-the-most-quota-left` 6305b64. The
branch last took main at 0372242 (1c1324b). main's head on `origin` is
still 0372242, so no merge was needed. The check covers this change's
proposal, design, six specs and tasks.

**The work holds, and the change is archived.** All 23 tasks are
ticked. Every requirement and scenario of the six specs is held by a
test that reads what its scenario says. The gate passes on the branch.
Archived over a copy of `openspec/`, the change leaves `openspec
validate --specs --strict` passing 38 of 38.

## Tasks

All 23 tasks of the seven groups are ticked, `[x]`, and none is open.
The gate run below backs each group's closing task: 1.4, 2.5, 3.4, 4.3,
5.2, 6.2 and 7.2. 7.2 names `root:changes` and `root:specs`. The gate
runs `root:changes`, and the archive over a copy runs what `root:specs`
runs.

- **1.1 and 1.3's revision.** Record 0035 is accepted on the user's
  words, "Approve both: start their code", on 2026-11-05. It names
  agentplot/blueprints main at 6188a10, which states 184, 185 and S86
  to S88. The suite's `REVISION` is 6188a10 and `SCENARIOS` is 91.
  blueprints main has since moved two commits to ff616eb (a465ba7,
  record 0038's `--append-system-prompt-file`; ff616eb, record 0040's
  sources and 191, 192, S95 to S97). Both touch only
  `design/flume/requirements.md` and belong to other changes. They
  change nothing of 184, 185 or S86 to S88. The clause-coverage spec
  takes the revision the build read.
- **1.2.** core.32's rule names record 0035 and 184 and 185. Its
  details rule out a reading asked, kept or written by flume, and an
  account past its threshold taken. core.30's detail reads `~` and
  `$VAR` in an account's `usage` beside its `env`. The site's rules
  page mirrors core.32's line.
- **7.3.** The account-pools delta's "An account is chosen under the
  room lock" is 497 characters. Its text cites 184 in its closing
  citation, beside 145, 147 and 185. Fill-first's choice reads "first
  usable one". The requirement asks what it asked before: the
  affinity, then the strategy's choice of fill-first, least-busy or
  most-left; usable meaning not spent, not past its threshold, under
  its own bound and of the stage's program; and the wait while the pool
  holds an account of the program. Its scenarios are unchanged.

## Checks

- **The gate.** `sluice exec main` at 6305b64 ran the checks 6305b64
  affects against main at 0372242. It passed 23 tasks of `#check-fast`
  and `#check-tools`, with `site:install` run for them, and sluice
  recorded the pass under its key for 6305b64:
  - `seams`, `core`, `fakes` and `conformance`: format, lint and test,
    and `conformance:coverage`;
  - `flume`: format, lint, test and `tools`, 83 tests, the walkthrough
    of this change and the herdr profile against the pinned herdr among
    them;
  - `plugin-claude`: test;
  - `site`: prose and build;
  - `root`: format, adrs and changes.

  `root:specs` did not run, since the branch changes nothing under
  `openspec/specs/`. The archive below runs what it runs.
- **The change.** `openspec validate
  a-pool-chooses-the-seat-with-the-most-quota-left --strict` passes.
- **The archive.** `openspec archive
  a-pool-chooses-the-seat-with-the-most-quota-left --yes`, run over a
  copy of `openspec/`, applies 11 requirements added and 5 modified
  across the six specs. `openspec validate --specs --strict` then
  passes 38 of 38. The synced account-pools spec's longest
  requirements are "An account is chosen under the room lock" at 497
  characters, "An intent keeps an account per program" at 481 and "An
  account's reading" at 452. Each is under strict's 500.
- **core.1 and core.35.** No line the change adds to `flume-core`,
  `flume-seams`, `flume-fakes` or `flume-conformance` names a tool or a
  program. No line it adds to `site/` names a tool, team, host or word
  of ours as the reader's own.

## Requirements and scenarios

Each scenario below is held by the test named, which reads what the
scenario says. The conformance scenarios are in
`crates/flume-conformance/src/engine/`.

### account-pools

- **A stage's pool** (modified: `most-left`, a threshold, a reading).
  Its two scenarios stand unchanged: `the_streams_pool_before_the_hosts`
  and `no_pool_anywhere`. The new fields are read by the store-profile
  and seam-contracts tests below.
- **An account is chosen under the room lock** (modified: most-left,
  usable meaning not past its threshold). Its four scenarios stand
  unchanged: `s54_fill_first_and_least_busy`,
  `two_starts_and_one_accounts_last_room`,
  `an_account_for_another_program` and `nothing_usable`.
  `readings_under_the_threshold` also holds fill-first and least-busy
  choosing as before when every reading is under the threshold.
  `AccountRoom::short` answers `Spent`, then `Threshold`, then `Full`
  (`short_answers_spent_then_threshold_then_full` in
  `flume-core/src/accounts.rs`).
- **An account's reading.**
  - *A fresh reading*: `a_fresh_reading` sets work-1 at 40% and 80%,
    read two minutes ago. It ranks work-1 above an account with 19 left
    and below one with 21, so work-1 has 20 left. `what_a_fresh_reading_leaves`
    holds the same in the core. Status giving both windows with their
    resets is `place_status.rs`'s `an_account_past_its_threshold`. The
    reading given fresh is `a_pool_with_readings`.
  - *A stale reading*: `a_stale_reading` holds work-1's reading, twenty
    minutes old and leaving 90, ranked after work-3's fresh one leaving
    15, and then with work-2, which names no reading, by fewest
    running. `a_pool_with_readings` gives a twenty-minute-old reading
    `fresh: false`.
  - *A window past its reset*: `a_window_past_its_reset` holds work-1's
    `5-hour` window, 95% used and reset an hour ago, taking the start.
    `a_window_past_its_reset_reads_as_nothing_used` holds it in the core.
  - *A reading that cannot be read*: `a_reading_that_cannot_be_read`
    takes a missing file, one not JSON and a `usage` naming `$GONE`.
    Each time the start runs on work-1 with no refusal, and `$GONE`
    asks the host for no file. Status's why with no path is
    `a_pool_with_readings`'s work-3 and `Engine::reading`'s words for a
    missing variable, which name the variable.
  - *Nothing written*: `nothing_written` reads each file at the choice
    and again when status is asked. Each reads back as the host wrote
    it. No document of the host's is written but the intent's, which
    holds no `read_at` or `windows`, and no spent time is recorded.
- **A most-left pool chooses the account with the most left.**
  - *Four places of four intents*: `s86_four_places_of_four_intents`.
    The first and second start on work-2. Eighteen minutes on, work-2's
    reading is twenty minutes old and work-1's is fresh, so the third
    starts on work-1. With work-1 spent, the fourth starts on work-3,
    running none, ahead of work-2, running two. No place records a move.
  - *No reading anywhere*: `most_left_with_no_reading_anywhere`
    alternates work-1, work-2, work-1.
  - *The affinity first*: `most_left_after_the_affinity`. The intent's
    next start stays on work-1, `from` the affinity, with no move,
    while work-2's reading leaves more. A place of another intent then
    takes work-2.
- **An account past its threshold takes no new start.**
  - *Past ninety percent*: `s87_past_ninety_percent`. work-1 runs two
    agents and reads `5-hour` 92% until 17:00. The intent's next stage
    starts on work-2 by the strategy. Both agents stay begun on work-1,
    and nothing is closed. Status gives the intent's move for `agent`
    with `why` `threshold`, `5-hour` and 17:00. After 17:00, with no
    newer reading, a place of another intent starts on work-1 and the
    intent stays on work-2 by its affinity.
  - *An old reading holds*: `an_old_reading_holds`. A three-hour-old
    reading of `7-day` 95% holds the stage waiting on the pool until
    Monday, and the start comes at Monday's settle.
  - *A window with no reset*: `a_window_with_no_reset` holds the stage
    until the reading is fifteen minutes old, then starts it.
  - *Under least-busy*: `under_least_busy` sends three starts to work-2
    while work-1, past its threshold, runs none.
  - *A threshold the pool states*: `a_threshold_the_pool_states`. A
    threshold of 100 takes 99% and refuses 100%. It does the same for
    50 at 49.99% and 50%. `the_threshold_is_held_at_its_own_share`
    holds the edges in the core.
- **The soonest time counts a threshold's end.**
  *Every account past its threshold or spent*:
  `s88_every_account_past_its_threshold_or_spent`. work-1 is past
  `7-day` until Monday, and work-2 is spent until 15:00 at 50%. The
  stage waits on the pool and not on its caller, with 15:00 as the
  soonest. A 10% reading at 14:00 leaves it waiting, and the first
  settle after 15:00 starts it on work-2.
- **A reading leaves a spent pause as it stands.**
  - *A reading under the threshold while spent*:
    `a_reading_under_the_threshold_while_spent` holds work-2 until
    15:00 though it reads 10%.
  - *A reading at its limit*: `a_reading_at_its_limit` sends the start
    to work-2 and records no spent time for work-1 at 100%.

### place-status

- **Status gives the host's pools** (modified).
  - *A pool with a spent account* stands unchanged.
  - *A pool with readings*: `a_pool_with_readings` gives the JSON form:
    most-left and 90; work-1 `5-hour` 42, 17:00 and fresh; work-2
    stale, at 12.5; work-3 `unread` "the file is missing". No reading's
    path is in the answer. The person's form is `print.rs`'s
    `a_pools_threshold_and_its_accounts_readings`, and the walkthrough
    reads both forms over a real file.
  - *An account past its threshold*: `an_account_past_its_threshold`
    gives `past` `7-day` until Monday, and that window `past: true`.
- **Status says a move off an account past its threshold.**
  - *The intent's move past a threshold*:
    `an_intents_move_past_a_threshold` gives the move in the JSON form
    with `why` `threshold`, `window` `5-hour` and `until` 17:00. The
    person's form says "work-1 past its threshold in 5-hour until …".
    `a_move_off_an_account_past_its_threshold` in `print.rs` holds the
    same words. `a_stage_moved_off_a_threshold` holds a stage's move
    and its intent's, each naming the window that holds latest.
  - *A move recorded before*: `a_move_recorded_before` gives a `spent`
    move with no window or end as written, in both forms.

### store-profile

- **A pool's strategy and threshold and an account's reading in the
  configuration.**
  - *A most-left pool with readings*: `a_most_left_pool_with_readings`
    in `crates/flume/src/host/config.rs`. It reads most-left, 85 and
    the `usage` as written, with its `~` unexpanded. The expansion is
    at the reading, in `Engine::reading`.
  - *C.7's example with readings*: the C.7 example test reads both
    accounts' `usage` and `work` fill-first with no threshold stated,
    so 90 holds.
  - *Values that refuse*: `a_threshold_a_usage_and_a_strategy_that_refuse`
    refuses `threshold` 0, 101, `"90%"` and 90.5, `usage = 7` and an
    empty `usage`, each naming the key. `round-robin`'s words name all
    three strategies. `a_threshold_and_a_usage_are_the_managed_layers`
    takes neither key from the local layer and gives `usage` as
    `<set>`.
- **An account's reading is read from its file.**
  - *The form read*: `usage.rs`'s `the_form_read` reads C.7's example
    with 71.5 as 7150 hundredths and a window with no reset. It ignores
    `login` and `kind`.
  - *A missing file*: `a_missing_file`, "the file is missing".
  - *A document not in the form*: `a_document_not_in_the_form` covers
    not JSON, no `read_at`, a `read_at` with no offset and `used` 120 at
    `windows[1]`, with eleven more forms. `tests/host.rs`'s
    `an_accounts_reading_from_its_file` holds `RealHost::usage` over a
    scratch file written whole. The file is unchanged after each
    reading, read anew at the next asking, and no refusal names a path.

### seam-contracts

- **The host gives an account's reading.** *The fake host's reading*:
  `the_hosts_reading` gives work-1's reading, work-2's refusal and
  work-3's missing file.
- **The types of a reading and a move past a threshold.** *A pool and
  a reading*: `round_trip.rs`'s `a_pool_and_a_reading`. *A move past a
  threshold*: `a_move_past_a_threshold`, the earlier move reading with
  no window and no end.

### seam-fakes

- **The fake host scripts each account's reading.** *A scripted
  reading*: `a_scripted_reading` gives the later reading at the next
  asking, and the log names both readings asked. The fake reads no
  file: `set_usage` keeps a map, and any other path is missing.

### clause-coverage

- **The suite knows the statement's clauses.** *The clauses listed*:
  `clauses.rs` reads 6188a10, with Part A through 186 and 190, Part B
  through 189, and the scenarios to S91. It names S89 unbuilt by record
  0036's change and 186 to 190, S90 and S91 by record 0037's. It names
  nothing of 184, 185 or S86 to S88. Its tests assert each.
- **A clause cited by nothing fails the suite.** *The suite as this
  change leaves it*: `conformance:coverage` passes in the gate. 184,
  185 and S86 to S88 are cited by this change's scenarios.

### The walkthrough

`walkthroughs.rs`'s
`a_most_left_pool_starts_on_the_account_with_most_left_and_none_past_its_threshold`
runs under the printed command in `flume:tools`. Two stand-in
accounts' readings are scratch files written whole. The first fix
starts on the account leaving more. That reading is then written at
93%, and the intent's next fix starts on the other, its move naming
`5-hour` and the reset. Status gives the threshold, both readings and
`past`, in the JSON form and the person's, with no path. The first fix
lands under its own account's home, and neither reading's file is
written.
