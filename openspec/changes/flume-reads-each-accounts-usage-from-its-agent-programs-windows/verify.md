# Verify: flume-reads-each-accounts-usage-from-its-agent-programs-windows

Verified at `cace1e68` on
`change/flume-reads-each-accounts-usage-from-its-agent-programs-windows`,
against its base `bolt/flumes-settings-read-as-its-records-say` at
`2d180f0b`, which the branch holds.

**Verdict: falls short. Not archived.** The whole checks fail:
`conformance:test` fails core.1's check, which this change's own tests
broke. Every task is ticked done, and every requirement and scenario of
the change's nine specs has code and a test that the run passed, but the
work is not done while the gate fails. The change goes back to its code
stage.

## The whole checks

`sluice exec bolt/flumes-settings-read-as-its-records-say`, run once
with the host's sluice 0.93.2: **failed**, `exit 1`. sluice ran the
checks `cace1e6` affects against the base, the bolt's `#check-fast`. Of
40 tasks, 39 completed and 1 failed: `failed: conformance:test`. sluice
recorded the 39 results under their keys for `cace1e6`.

The first attempt, `devenv shell -- sluice exec …`, stopped before any
check ran. Inside the shell the sluice the tree pins for the merge-queue
profile's tests, 0.71.0, comes first on `PATH`, and it refused the
host's `~/.config/sluice/config.toml` (`unknown field credential`, line
21). The run above uses the host's own sluice, which enters the devenv
toolchain itself as `.config/sluice.toml`'s `[tests] toolchain` says.
The verify of `a-stream-runs-its-agents-in-rex` ran the same way.

### The failure

`conformance:test`, `tests/no_tool.rs`,
`no_tool_is_named_outside_a_profile`:

```
a tool is named outside a profile (core.1):
crates/flume/src/host/config.rs:2740: claude
crates/flume/src/host/config.rs:2750: claude
crates/flume/src/host/config.rs:2774: claude
```

All three lines are in tests that commit `786419f1` added, task 6.1's:
`a_most_left_pool`, at 2740 and 2750, gives the account
`program = "claude"`, and `an_accounts_usage_is_no_word_of_an_account`,
at 2774, does the same. `host/config.rs` is no profile. core.1 rules out
naming `claude` in any module outside a profile directory, and its check
scans test modules too. The tests before them in the same file name a
program `agent`, as the second half of `a_most_left_pool` already does
(`program = "agent"`). The store-profile scenarios "A most-left pool"
and "An account naming usage" name `claude` as their example, but
nothing they assert turns on the program's name.

What the code stage must do: in those two tests, name a program no
profile holds (`agent`, as the file's other pools do), or move each test
where core.1 lets `claude` be named. Then `sluice exec <base> -- moon run
conformance:test` passes.

### What passed

`root:format`, `root:hooks`, `root:adrs`, `root:specs` (49 passed),
`root:changes` (2 passed, this change among them), `root:marketplace`,
`root:toolchain`, `root:layers`, `root:versions`; `seams`, `core`,
`fakes` and `flume`, each `format`, `lint` and `test`; `conformance`'s
`format`, `lint` and `coverage`; `nix:format`, `nix:lint`,
`nix:typecheck`, `nix:test`; `plugin-claude:validate`, `format`,
`typecheck`, `engine-types` and `test`; `plugin-herdr:manifest`, `lint`
and `test` (27 passed); `site:install`, `site:prose` (0 errors, 0
warnings) and `site:build`. Of `conformance:test`'s other test binaries,
every one passed.

The 37 warnings in the run are moon's: from the gate's sandbox it could
not write its output archives. No task failed because of them.

`openspec validate flume-reads-each-accounts-usage-from-its-agent-programs-windows --strict`:
valid.

## The tasks

All 22 tasks, 1.1 to 10.1, are ticked done. Each check a task names
exists. Every check that falls inside `#check-fast` passed in the run,
except task 6.1's tests, which pass on their own but break core.1's
check. The check-tools build that task 6.4 names
(`cargo test -p flume --features tools --test tools --no-run`) was not
run: the change goes back for the failure above, and the next verify
runs it.

## The specs

Each scenario has a test the run passed. The code was read against
each requirement.

- **account-pools** (14 scenarios): `account_pools.rs` holds each:
  S86's four places, no reading anywhere, the affinity first; a fresh
  and a stale reading; a window past its reset; an account no session
  reported for, and a program whose profile gives no windows, each with
  its words from `Engine::no_reading`; an account flume runs no agent
  on; nothing asked of the program, where the fake agent's log shows no
  `windows` read; a report taken, an earlier report, and no account or
  another secret, through `Engine::take_reading` in `readings.rs`, by
  compare-and-set, keeping the spent keys, and writing nothing where
  the kept reading was taken at the same second or later; and a spent
  time and a reading. A reading is fresh for `FRESH`, 15 minutes, read
  from `taken_at`.
- **agent-profile** (2): `profiles/agent/claude/windows.rs`.
  `the_engines_form_read` reads 23.5 as 2350 hundredths, 7 with no
  reset as 700, and 112 as 10000. `Claude::windows` reads
  `windows.json` in the start's record, with the started ack's session.
  `reports_windows` is offered. `agent_claude.rs` passes the seam's
  windows scenarios through `report_windows`.
- **claude-code-plugin** (10): `hooks/usage.ts` and
  `tests/usage.test.ts` hold the five module scenarios. The module is
  registered by `hooks/flume.ts`, the one module `hooks.json` lists, and
  runs `flume hook usage` with a 10-second bound, not awaited, catching
  any failure. `hooks_claude.rs` holds the four `hook usage` scenarios,
  107.5 read as 100 among them. The engine types are pinned:
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
  readings". `print.rs` gives `taken at <time> from <agent>`, a `stale, `
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
  `[pools.work.accounts[0]] usage is no word of an account`. C.7's
  example reads whole. `store_state_dir.rs` holds "A reading beside a
  spent time" and "An account's document an earlier release wrote".
  `host/usage.rs` is gone.

Nothing the specs ask for is missing. The one shortfall is the gate's
failure above.
