# Verify: flume-reads-each-accounts-usage-from-its-agent-programs-windows

Verified at `4f238814` on
`change/flume-reads-each-accounts-usage-from-its-agent-programs-windows`.
The branch holds its base, `bolt/flumes-settings-read-as-its-records-say`,
at `ea8939fd`, merged in by task 11.3. The base has since moved to
`ea6d499d`, where `flumes-own-kinds-carry-their-agents-mode` landed
(#74, record 0064).

**Verdict: holds. Archived.** The whole checks pass against the base as
it stands, every task is ticked done, and the code meets every
requirement and scenario of the change's nine specs.

## The whole checks

`devenv shell -- sluice exec bolt/flumes-settings-read-as-its-records-say`
ran no check. Inside the devenv shell the sluice the tree pins for the
merge-queue profile's tests, 0.71.0, comes first on `PATH`, and it
refused the host's `~/.config/sluice/config.toml` (`unknown field
credential`, line 21), exiting 3 before any check ran.

`sluice exec bolt/flumes-settings-read-as-its-records-say`, run once with
the host's sluice 0.93.2, which enters the devenv toolchain itself as
`.config/sluice.toml`'s `[tests] toolchain` says: **passed**, `exit 0`.
sluice ran "the checks 4f23881 affects against
bolt/flumes-settings-read-as-its-records-say (ea6d499)", the bolt's
`#check-fast`, so the run holds the base's newest landing. 40 tasks
completed and sluice recorded 39 passed results under their keys for
`4f23881`; `site:install` is the site's dependency and has no check of
its own. No task failed.

What passed: `root:format`, `root:hooks`, `root:adrs`, `root:specs`
(49 passed), `root:changes` (2 passed, this change among them),
`root:marketplace`, `root:toolchain`, `root:layers`, `root:versions`;
`seams`, `core`, `fakes`, `conformance` and `flume`, each `format`,
`lint` and `test`, and `conformance:coverage`; `nix:format`,
`nix:lint`, `nix:typecheck`, `nix:test`; `plugin-claude:validate`,
`format`, `typecheck`, `engine-types` and `test`;
`plugin-herdr:manifest`, `lint` and `test`; `site:install`,
`site:prose` (0 errors, 0 warnings) and `site:build`.

## The base

`ea6d499d` is not an ancestor of the branch, but `git merge-tree` merges
the two with no conflict, and #74 changes only the agent-modes,
kind-sources, kinds-profile, launching and running-through specs, none
of which this change modifies. sluice's run above checked the branch
over it.

## The tasks

All 26 tasks, 1.1 to 11.4, are ticked done, and each check a task names
exists. Every check that falls inside `#check-fast` passed in the run.
The two task checks that run outside it pass:

- **11.2 and 11.4**: `openspec archive` on a copy of the branch's
  `openspec`, then `openspec validate --specs --strict`, gives 49
  passed, and the archived `openspec/specs/host-module/spec.md` names
  `keep_free` and `ends_expire` twice each and `no \`usage\`` once, so
  record 0063's words stand beside this change's.

## The specs

Each requirement and scenario is met by the code and test named.

- **account-pools** (3 modified or added requirements, 14 scenarios):
  the most-left choice, the reading as what the program reported, a
  live start's report taken and spent time beside a reading are in
  `crates/flume-core` (`accounts.rs`, `room.rs`, `readings.rs`,
  `spent.rs`). `crates/flume-conformance/src/engine/account_pools.rs`
  holds every scenario: `a_fresh_reading`, `a_stale_reading`,
  `a_window_past_its_reset`, `an_account_no_session_reported_for`,
  `a_program_whose_profile_gives_no_windows`,
  `an_account_flume_runs_no_agent_on`, `nothing_asked_of_the_program`,
  `a_report_taken`, `an_earlier_report`, `no_account_or_another_secret`,
  `a_spent_time_and_a_reading`, `most_left_with_no_reading_anywhere`
  and the four-intent and affinity cases.
- **agent-profile** (2): `profiles/agent/claude/windows.rs` reads the
  engine's form and keeps `windows.json` on the start's record;
  `agent_claude.rs` runs the seam's windows scenarios.
- **claude-code-plugin** (10, and the pinned engine): `hooks/usage.ts`
  passes a `session.measure` whose `changed` names `rateLimits` and that
  holds a window, after `next(e)`, unawaited, under a ten-second bound,
  a failure caught, and nothing outside a session flume started;
  `tests/usage.test.ts` holds its five scenarios; `hooks/flume.ts`
  registers it beside `line.ts`; `hooks_claude.rs` holds the four of
  `flume hook usage`; `plugin-claude:typecheck` and `engine-types` hold
  the engine to the release the tree pins.
- **clause-coverage** (3): `clauses.rs` at `55b89a4`, 211 in Part B,
  123 scenarios, S121 to S123 named as built before the list held them;
  `conformance:coverage` passed.
- **host-module** (1): `nix/modules/options.nix` declares an account's
  `name`, `program`, `env` and `agents` alone, and the case "Only the
  keys flume reads are declared" in `nix/tests/options.nix` asserts
  them beside the host's `multiplexer` and `ends_expire`, a label's
  `owed` and `reserved`, and no `terminal`, `reserve`, `keep_free`,
  `points` or `pace`; "A key the module does not declare" passes
  `host.terminal` undeclared.
- **place-status** (4): `ReadingStatus::Read` with `read_at`, `agent`
  and `session`, and `Unread` with why; `place_status.rs` holds "A pool
  with readings"; `commands/print.rs`'s
  `a_pools_threshold_and_its_accounts_readings` gives the person's form.
- **seam-contracts** (5): `Agent::windows` and `Agent::reports_windows`,
  `AccountReading` in C.7's form and `AccountDocument`, with
  `round_trip::the_windows_an_agent_reported`, `a_pool_and_a_reading`
  and `an_accounts_document`; `seams/agent.rs` and `seams/store.rs` hold
  the seam scenarios; `Host::usage` and `Account::usage` are gone.
- **seam-fakes** (2): the fake agent's `report_windows` and
  `reports_no_windows`; the fake host's readings are gone.
- **store-profile** (9): `host/config.rs` refuses `usage` as
  `[pools.<pool>.accounts[<n>]] usage is no word of an account`, with
  `a_most_left_pool` and `an_accounts_usage_is_no_word_of_an_account`;
  `host/usage.rs` is gone; `store_state_dir.rs` holds "A reading beside
  a spent time" and "An account's document an earlier release wrote".

The site says the same: `reference/configuration.md` names `usage`
among the words an account refuses beside record 0063's label keys and
`ends_expire`; `reference/state.md` gives the account's document's
`reading` and status's `reading` with `read_at`, `agent`, `session` and
why there is none; `reference/commands.md` gives `flume hook usage`;
`guides/claude-code.mdx` gives the module that passes the windows.
