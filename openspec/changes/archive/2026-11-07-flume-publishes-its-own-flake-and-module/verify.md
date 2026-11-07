# Verify: flume-publishes-its-own-flake-and-module

Verified at `f93482fc` on `change/flume-publishes-its-own-flake-and-module`,
against its base `bolt/flume-names-each-repository-by-its-address` at
`f33f6dce`, which the branch holds.

**Verdict: holds. Archived.** The whole checks pass. Every requirement
and scenario of the change's specs is met by a check that passed. Every
task is ticked done.

## The whole checks

`devenv shell -- sluice exec bolt/flume-names-each-repository-by-its-address`,
run once: **passed**, `exit 0`, with 40 tasks completed and 39 passed.
sluice recorded the 39 results under their keys for `f93482f`. sluice
reported no conflict with the base. The base is in the branch, and
`docs/architecture/core.md` differs from it only in core.45's details.
That line holds task 7.1's two clauses and record 0055 among its
sources, and the base's core.55 stands as the base gives it.

The run's warnings are moon's: its shared cache and output archives
could not be written from the gate's sandbox, and with no network it
skipped the Rust dependency install. No task failed for either.

Of note:

- `conformance:test`, `no_tool_is_named_outside_a_profile` (core.1):
  ok. `the_modules_files_read_whole` and its fixtures name `scribe`,
  `SCRIBE_HOME` and `.scribe-work-*`. No agent profile's table holds
  any of those names (task 3.3).
- `nix:format`, `nix:lint`, `nix:typecheck` (`nix: typecheck: every case
  holds`) and `nix:test` (`nix: test: every case holds`), all in the
  gate's sandbox against the dummy store.
- `root:toolchain` names nix 2.34.8, nixfmt 1.5.0, deadnix 1.3.2 and
  statix 0.5.8-unstable-2026-07-17, each at its pin.
- `root:specs` (46 passed), `root:changes` (2 passed, this change among
  them), `root:layers`, `root:format`, `root:adrs`, `root:hooks`,
  `root:versions` and `root:marketplace`.
- `core:test`, including `own::tests::each_shape_is_a_credential`,
  `a_value_that_passes` and `a_variable_named_rather_than_a_value` over
  `credential-shapes.json`.
- `flume:test`, every suite passing, `the_modules_files_read_whole`
  among them.
- `flume:lint`, `conformance:lint`, `conformance:coverage`, every
  `seams`, `fakes` and `plugin-*` task, `site:prose` (0 errors, 0
  warnings), `site:build` and `site:install`.

## The specs

Each scenario is held by a case under `nix/tests` or by a Rust test.
Each of those passed in the run above.

### flake

| Scenario | Held by | Met |
|---|---|---|
| Every system's package evaluates | `typecheck.nix`, same name | yes |
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
| A path value | `file.nix` (refusals) and `scripts.sh` (the evaluation's error), at `kinds.change.agents.programs.scribe.file` | yes |
| flume reads the rendered files | `host.rs`, `the_modules_files_read_whole`, naming no agent program (core.1) | yes |
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

Every task in `tasks.md`, 1.1 to 7.3 with 3.3 among them, is ticked
`[x]`, 18 in all, and none is open. Each one's work is in the tree as
the task describes. Task 7.1's amendment of core.45's details stands
beside the base's core.55.
