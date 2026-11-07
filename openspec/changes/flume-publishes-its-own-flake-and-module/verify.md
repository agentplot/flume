# Verify: flume-publishes-its-own-flake-and-module

Verified at `3985a85d` on `change/flume-publishes-its-own-flake-and-module`,
against its base `bolt/flume-names-each-repository-by-its-address` at
`f33f6dce`.

**Verdict: falls short. Not archived.** The whole checks fail on one
task, `conformance:test`. sluice also reports that the branch would
conflict with its base in `docs/architecture/core.md`. Every
requirement and scenario of the change's specs is otherwise met by a
check that passed, and every task is ticked done.

## The whole checks

`devenv shell -- sluice exec bolt/flume-names-each-repository-by-its-address`,
run once: **failed**, 39 tasks completed and 1 failed (`exit 1`). sluice
recorded the 39 results under their keys for `3985a85`.

### What fails

- **`conformance:test`**, `tests/no_tool.rs`,
  `no_tool_is_named_outside_a_profile` (core.1). The new test
  `the_modules_files_read_whole` in `crates/flume/tests/host.rs` names
  the program `claude` outside a profile, at lines 1412, 1413, 1419,
  1420, 1468 (twice), 1469, 1476, 1500 and 1517:

  ```text
  a tool is named outside a profile (core.1):
  …/crates/flume/tests/host.rs:1412: claude
  …/crates/flume/tests/host.rs:1413: claude
  …/crates/flume/tests/host.rs:1419: claude
  …/crates/flume/tests/host.rs:1420: claude
  …/crates/flume/tests/host.rs:1468: claude
  …/crates/flume/tests/host.rs:1468: claude
  …/crates/flume/tests/host.rs:1469: claude
  …/crates/flume/tests/host.rs:1476: claude
  …/crates/flume/tests/host.rs:1500: claude
  …/crates/flume/tests/host.rs:1517: claude
  test result: FAILED. 1 passed; 1 failed
  ```

  The cause is the names in `every-key.nix` and `every-key.toml`:
  `program = "claude"`, `CLAUDE_CONFIG_DIR`, `programs.claude` and
  `.claude-work-*`. The Rust test asserts on those names. Task 3.2's
  narrow check, `flume:test`, does not run core.1's search, so the task
  passed its own check.

- **A conflict with the base.** sluice printed `would conflict:
  docs/architecture/core.md with bolt/flume-names-each-repository-by-its-address`.
  The base has moved since this place was cut (`4b2f3eca` to `f33f6dce`).
  Its `3a11cbe5`, "rule core.55, a repository named by its address,
  else its folder", changes `docs/architecture/core.md` beside this
  change's task 7.1 amendment of core.45's details. `git merge-tree`
  shows that file as the only conflict. `crates/flume/tests/host.rs` and
  the configuration reference merge cleanly.

### What passes

- `nix:format`, `nix:lint`, `nix:typecheck` (`nix: typecheck: every case
  holds`) and `nix:test` (`nix: test: every case holds`), all in the
  gate's sandbox against the dummy store.
- `root:toolchain` names nix 2.34.8, nixfmt 1.5.0, deadnix 1.3.2 and
  statix 0.5.8-unstable-2026-07-17, each at its pin.
- `root:specs` (46 passed), `root:changes` (2 passed), `root:layers`,
  `root:format`, `root:adrs`, `root:hooks`, `root:versions` and
  `root:marketplace`.
- `core:test`, including `own::tests::each_shape_is_a_credential`,
  `a_value_that_passes` and `a_variable_named_rather_than_a_value` over
  `credential-shapes.json`.
- `flume:test`, every suite passing, including
  `the_modules_files_read_whole`.
- `flume:lint`, `conformance:lint`, `conformance:coverage`, every
  `seams`, `fakes` and `plugin-*` task, `site:prose` (0 errors),
  `site:build` and `site:install`.

## The specs

Each scenario is held by a case under `nix/tests` or by a Rust test, and
each of those passed in the run above.

### flake

| Scenario | Held by | Met |
|---|---|---|
| Every system's package evaluates | `typecheck.nix`, "Every system's package evaluates" | yes |
| The package's version and toolchain are the tree's | `packages.nix`, same name | yes |
| The package builds | `ci.yml` job `flake`, held by `workflow.nix`; `nix:build` is in no layer by design and was not run here | by CI |
| A git crate's revision moves without its hash | `packages.nix`, same name, over a scratch lock | yes |
| A git crate with no entry | `packages.nix`, same name | yes |
| The inputs a taker follows | `packages.nix`, same name, over `flake.nix` and `flake.lock` | yes |
| The flake's outputs | `outputs.nix`, same name | yes |
| A module imported twice | `darwin.nix` and `nixos.nix`, "The module imported twice", `enable` declared once | yes |

### host-module

| Scenario | Held by | Met |
|---|---|---|
| No package | `darwin.nix` and `nixos.nix`, "No package", the link still run | yes |
| Enabled on nix-darwin | `darwin.nix`: profile, post-activation as alice under `/Users/alice`, `requiresPrimaryUser`, no launchd job | yes |
| Enabled on NixOS | `nixos.nix`: profile, the user activation guarded by `id -un` = alice, no systemd unit | yes |
| No user to write for | `darwin.nix` "No home for the primary user", `nixos.nix` "No such user on NixOS" | yes |
| A mistyped declared key | `options.nix`, plus `scripts.sh` over `fails.nix` reading the error's path | yes |
| The declared keys | `options.nix`, "Only the keys flume reads are declared" | yes |
| A key the module does not declare | `options.nix`, same name | yes |
| Two modules' settings merge | `options.nix`, same name, text compared whole | yes |
| Every key renders its file | `file.nix` and `options.nix`, byte for byte against `every-key.toml` | yes |
| No settings | `file.nix` and `options.nix`, against `empty.toml`, the first line alone | yes |
| A path value | `file.nix` (refusals) and `scripts.sh` (the evaluation's error) | yes |
| flume reads the rendered files | `host.rs`, `the_modules_files_read_whole` | yes, but it breaks core.1 (above) |
| A token in an account's env | `options.nix`: the path named, the text absent, `$NAME` said | yes |
| A variable named instead | `options.nix`: passes, and the file holds `$AGENT_TOKEN` | yes |
| One table of cases for both | `credential.nix` over `credential-shapes.json`, beside `own::tests`, with the port's prefixes and secret names held to `own.rs` | yes |
| A fresh home | `scripts.sh`, same name | yes |
| A file aside already | `scripts.sh`, same name | yes |
| A link of an earlier activation | `scripts.sh`, same name | yes |
| A hand-written file there | `scripts.sh`, same name, `config.local.toml`, `sources/` and `flume.lock` compared before and after, plus a folder case | yes |

### the-gate

| Scenario | Held by | Met |
|---|---|---|
| The build task's layer | `nix/moon.yml`: four tasks in check-fast, `build` in none with its comment; `root:layers` passed | yes |
| The checks in the gate's sandbox | `nix:typecheck` and `nix:test` under `sluice exec`, each saying `every case holds` | yes |
| No inputs given | `scripts.sh`, "No inputs given" | yes |
| The workflow's build job | `workflow.nix`, same name | yes |
| The nix tools at their pins | `root:toolchain`, above | yes |

## The tasks

Every task in `tasks.md`, 1.1 to 7.3, is ticked `[x]`. Each one's work
is in the tree as the task describes. Task 7.1's amendment to core.45
holds as written but conflicts with the base's core.55, as noted above.

## What goes back to the code stage

1. Make `conformance:test` pass. core.1's search reads the sources
   under `crates/`, so the names fail in `host.rs` alone. Have
   `the_modules_files_read_whole` name no agent program outside a
   profile. One way is to give `every-key.nix`, and the
   `every-key.toml` rendered from it, a program, a variable and a
   folder that are no tool's. If so, move `file.nix`'s and
   `fails.nix`'s `programs.claude.file` path cases with them, so the
   rendered file and the Rust assertions stay byte for byte the same.
2. Merge the base's current revision (`f33f6dce`) into the branch and
   resolve `docs/architecture/core.md`, keeping the base's core.55 and
   this change's core.45 details.
