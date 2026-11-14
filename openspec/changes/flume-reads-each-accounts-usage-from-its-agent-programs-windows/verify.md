# Verify: flume-reads-each-accounts-usage-from-its-agent-programs-windows

Verified at `77d915c3` on
`change/flume-reads-each-accounts-usage-from-its-agent-programs-windows`.
The branch was cut from `bolt/flumes-settings-read-as-its-records-say`
at `2d180f0b`. The base has since moved to `ea8939fd`, where
`a-labels-share-is-owed-and-reserved` landed (#73, record 0063).

**Verdict: falls short. Not archived.** The whole checks pass, every
task is ticked done, and the code meets every requirement and scenario
of the change's nine specs. But the branch no longer lands on its base.
It conflicts with `ea8939fd` in three files. Its host-module delta also
modifies a requirement the base has since modified, so archiving the
change now would drop record 0063's words from
`openspec/specs/host-module/spec.md`, or would force that spec to be
edited by hand when the base is merged. The change goes back to its code
stage.

## The shortfall: the base moved under the change

### Three files conflict

`sluice exec` reported the conflict before running the checks, and
`git merge-tree` against `ea8939fd` shows the same three files:

```
would conflict: nix/tests/options.nix, site/src/content/docs/reference/configuration.md, site/src/content/docs/reference/state.md with bolt/flumes-settings-read-as-its-records-say
```

Each file holds a part that both changes rewrote. Task 8.1 drops an
account's `usage` from the case "Only the keys flume reads are declared"
in `nix/tests/options.nix`. Record 0063's change adds `ends_expire`,
`owed` and `reserved` to the same case and takes out `reserve` and
`keep_free`. Task 10.1 and record 0063's change both rewrite the
configuration and state references.

### One requirement is modified by both changes

This change's `specs/host-module/spec.md` modifies "Only the keys flume
reads are declared". Its text was written against the requirement as it
stood at `2d180f0b`. At `ea8939fd` the same requirement and its scenario
"The declared keys" also say that a label's `reserve` and `keep_free`
pass undeclared (record 0063), and the scenario gives `host`'s
`ends_expire` and a label's `owed` and `reserved`.

- **The change archived over the base's specs** (tried on a copy):
  `openspec archive` replaces the requirement with this change's text.
  `openspec validate --specs --strict` passes (49 passed), but the spec
  loses record 0063's words. The base's spec names `keep_free` or
  `ends_expire` three times. The result names them once.
- **The change archived on the branch, then the base merged** (tried
  with `git merge-file` on copies): `openspec/specs/seam-contracts/spec.md`
  merges cleanly. `openspec/specs/host-module/spec.md` conflicts in two
  hunks, the requirement's text and its scenario. Resolving those hunks
  means editing a spec file directly, which the repository's
  instructions forbid.

### What the code stage must do

Merge `bolt/flumes-settings-read-as-its-records-say` (`ea8939fd` or
later) into the branch, resolving the three files so each keeps both
changes. Rewrite this change's host-module MODIFIED block over the
requirement as it stands at the base, so it keeps record 0063's words
(`reserve` and `keep_free` undeclared; `host`'s `ends_expire`; a label's
`owed` and `reserved`) and adds an account's `usage` and the account's
declared keys. The change is not archived, so verify can then archive
it over the merged base.

## The whole checks

`devenv shell -- sluice exec bolt/flumes-settings-read-as-its-records-say`
ran no check. Inside the devenv shell, the sluice the tree pins for the
merge-queue profile's tests, 0.71.0, comes first on `PATH`. It refused
the host's `~/.config/sluice/config.toml` (`unknown field credential`,
line 21) and exited 3.

`sluice exec bolt/flumes-settings-read-as-its-records-say`, run once with
the host's sluice 0.93.2, which enters the devenv toolchain itself as
`.config/sluice.toml`'s `[tests] toolchain` says: **passed**, `exit 0`.
sluice "ran the checks 77d915c affects against
bolt/flumes-settings-read-as-its-records-say (ea8939f)", the bolt's
`#check-fast`. 40 tasks completed, and sluice recorded 39 passed results
under their keys for `77d915c`. `site:install` is the site's dependency
and has no check of its own. No task failed.

What passed: `root:format`, `root:hooks`, `root:adrs`, `root:specs`
(49 passed), `root:changes` (2 passed, this change among them),
`root:marketplace`, `root:toolchain`, `root:layers`, `root:versions`;
`seams`, `core`, `fakes`, `conformance` and `flume`, each `format`,
`lint` and `test`, and `conformance:coverage`; `nix:format`,
`nix:lint`, `nix:typecheck`, `nix:test`; `plugin-claude:validate`,
`format`, `typecheck`, `engine-types` and `test`;
`plugin-herdr:manifest`, `lint` and `test`; `site:install`,
`site:prose` (0 errors, 0 warnings) and `site:build`.

The run's warnings are moon's: from the gate's sandbox it could not
write its output archives. No task failed because of them.

## The tasks

All 24 tasks, 1.1 to 11.2, are ticked done, and each check a task names
exists. Every check that falls inside `#check-fast` passed in the run.
Task 11.2's own check passes: `openspec archive` on a copy of the
branch's `openspec`, then `openspec validate --specs --strict`, gives
49 passed. The three requirements it shortened are now 467, 471 and 455
characters long.

## The specs

The code has not changed since `3d233282`, which the last verify read
against every requirement and found complete. Since then only the text
of three MODIFIED requirements has changed (task 11.2). The detail
taken out of each requirement is now in one of its scenarios, and an
existing test holds that detail:

- **place-status**, "A pool with readings": `read_at` gives when flume
  took the reading, and the JSON form keeps every field it gave before.
  `ReadingStatus::Read` keeps `read_at` and gains `agent` and `session`;
  `place_status.rs` holds the scenario.
- **seam-contracts**, "A pool and a reading": `taken_at` is RFC 3339's
  with its offset, and `used` is a share from 0 to 100.
  `round_trip::a_pool_and_a_reading` asserts `2026-11-05T14:02:00Z`,
  `used` 42 and 23.5, and reads a reading written with an offset.
- **store-profile**, "A reading beside a spent time": the reading is in
  C.7's form. `store_state_dir.rs` holds the scenario.

Everything else the last verify found still holds:

- **account-pools** (14 scenarios): `account_pools.rs`.
- **agent-profile** (2): `profiles/agent/claude/windows.rs` and
  `agent_claude.rs`.
- **claude-code-plugin** (10): `hooks/usage.ts`, `tests/usage.test.ts`,
  `hooks/flume.ts` and `hooks_claude.rs`.
- **clause-coverage** (3): `clauses.rs` at `55b89a4`.
- **host-module** (1): `nix/modules/options.nix` and
  `nix/tests/options.nix`.
- **place-status** (4), **seam-contracts** (6), **seam-fakes** (2) and
  **store-profile** (9): the code and tests the last verify named.

Against `2d180f0b`, the code meets everything the specs ask for. The
shortfall is the moved base above.
