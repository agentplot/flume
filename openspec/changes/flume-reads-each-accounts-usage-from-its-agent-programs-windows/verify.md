# Verify: flume-reads-each-accounts-usage-from-its-agent-programs-windows

Verified at `3d233282` on
`change/flume-reads-each-accounts-usage-from-its-agent-programs-windows`,
against its base `bolt/flumes-settings-read-as-its-records-say` at
`2d180f0b`, which the branch holds.

**Verdict: holds. Archived.** The whole checks pass, every task is
ticked done, and every requirement and scenario of the change's nine
specs has code and a test that the run passed.

## The whole checks

`sluice exec bolt/flumes-settings-read-as-its-records-say`, run once
with the host's sluice 0.93.2: **passed**, `exit 0`. sluice ran the
checks `3d23328` affects against the base, the bolt's `#check-fast`:
40 tasks completed, and sluice recorded 39 passed results under their
keys for `3d23328`, `site:install` being the site's dependency and no
check of its own. No task failed.

`devenv shell -- sluice exec …` stopped before any check ran. Inside the
shell the sluice the tree pins for the merge-queue profile's tests,
0.71.0, comes first on `PATH`, and it refused the host's
`~/.config/sluice/config.toml` (`unknown field credential`, line 21).
The run above uses the host's own sluice, which enters the devenv
toolchain itself as `.config/sluice.toml`'s `[tests] toolchain` says.

What passed: `root:format`, `root:hooks`, `root:adrs`, `root:specs`
(49 passed), `root:changes` (2 passed, this change among them),
`root:marketplace`, `root:toolchain`, `root:layers`, `root:versions`;
`seams`, `core`, `fakes`, `conformance` and `flume`, each `format`,
`lint` and `test`, and `conformance:coverage`; `nix:format`,
`nix:lint`, `nix:typecheck`, `nix:test`; `plugin-claude:validate`,
`format`, `typecheck`, `engine-types` and `test`;
`plugin-herdr:manifest`, `lint` and `test`; `site:install`,
`site:prose` (0 errors, 0 warnings) and `site:build`.
`conformance:test`'s `no_tool_is_named_outside_a_profile` passed: no
tool is named outside a profile (core.1).

The run's warnings are moon's: from the gate's sandbox it could not
write its output archives. No task failed because of them.

`openspec validate flume-reads-each-accounts-usage-from-its-agent-programs-windows --strict`:
valid.

## The tasks

All 23 tasks, 1.1 to 11.1, are ticked done, and each check a task names
exists. Every check that falls inside `#check-fast` passed in the run,
task 11.1's `conformance:test` among them. The check-tools build that
task 6.4 names, `devenv shell -- cargo test -p flume --features tools
--test tools --no-run`, was run beside the gate and built.

## The specs

Each scenario has a test the run passed. The code was read against
each requirement.

- **account-pools** (14 scenarios): `account_pools.rs` holds each:
  S86's four places, no reading anywhere, the affinity first; a fresh
  and a stale reading; a window past its reset; an account no session
  reported for, and a program whose profile gives no windows, each with
  its words; an account flume runs no agent on; nothing asked of the
  program, where the fake agent's log shows no `windows` read; a report
  taken, an earlier report, and no account or another secret, through
  `Engine::take_reading` in `crates/flume-core/src/readings.rs`, which
  refuses a start that is not live or a secret not its start's, takes
  nothing under no account or with no report, and writes by
  compare-and-set, keeping the spent keys and writing nothing where the
  kept reading was taken at the same second or later; and a spent time
  and a reading. A reading is fresh for 15 minutes, read from
  `taken_at`.
- **agent-profile** (2): `profiles/agent/claude/windows.rs` reads the
  engine's `{"rateLimits": […]}`, each window's `kind` a non-empty
  string, its `percentUsed` 0 or more in hundredths and past 100 read
  as 100, its `resetsAt` by RFC 3339, and a report with any window in
  no such form, or an empty list, read as none. `Claude::windows` reads
  `windows.json` in the start's record with the started ack's session.
  `reports_windows` is offered. `agent_claude.rs` passes the seam's
  windows scenarios.
- **claude-code-plugin** (10): `hooks/usage.ts` runs `flume hook usage`
  after `next(e)`, only where `FLUME_PLACE` and `FLUME_AGENT` are set,
  `changed` names `rateLimits` and `rateLimits` holds a window, with a
  10-second bound, not awaited, any failure caught.
  `tests/usage.test.ts` holds its five scenarios. `hooks/flume.ts`, the
  one module `hooks.json` lists, registers it beside `line.ts`.
  `hooks_claude.rs` holds the four `hook usage` scenarios, 107.5 read
  as 100 among them. The engine types are pinned:
  `plugin-claude:engine-types` passed.
- **clause-coverage** (3): `clauses.rs` is at `55b89a4`, with 211 in
  Part B, `SCENARIOS` 123, and S121 to S123 named unbuilt by records
  0060 and 0061's changes. `conformance:coverage` passed.
- **host-module** (1): `nix/modules/options.nix` declares no `usage`.
  `nix/tests/options.nix` reads `name`, `program`, `env` and `agents`.
  `nix:test` passed.
- **place-status** (4): status's JSON form only gains fields:
  `ReadingStatus::Read` keeps `read_at` and gains `agent` and `session`,
  and `Unread` gives why. `place_status.rs` holds "A pool with
  readings". `print.rs` gives `taken at <time> from <agent>`, `stale, `
  before it where the reading is not fresh, and `no reading: <why>`.
- **seam-contracts** (6): `seams/agent.rs` holds the windows scenarios,
  run against the fake both ways (`FakeAgentHarness::reporting_no_windows`).
  `round_trip.rs` holds "A pool and a reading" and "An account's
  document", an earlier release's document reading as spent with no
  reading. `seams/store.rs` holds a document with both. `Host::usage`
  and `Account::usage` are gone.
- **seam-fakes** (2): the fake agent's `report_windows` and
  `reports_no_windows` are tested in `fakes.rs`, each `windows` read
  logged. The fake host's `set_usage` and `usage` are gone.
- **store-profile** (9): `host/config.rs` refuses `usage` as
  `[pools.work.accounts[0]] usage is no word of an account`, its tests
  naming the program `agent`, which no profile holds (core.1). C.7's
  example reads whole. `store_state_dir.rs` holds "A reading beside a
  spent time" and "An account's document an earlier release wrote".
  `host/usage.rs` is gone.

Nothing the specs ask for is missing.
