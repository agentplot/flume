# Verify: flumes-module-writes-the-hosts-sources

Verified at `ef09a4be` on `change/flumes-module-writes-the-hosts-sources`,
against its base `bolt/flumes-module-writes-its-sources` at `8414774a`,
which the branch holds.

**Verdict: holds. Archived.** The whole checks pass. Every requirement
and scenario of the change's spec is met by a check that passed. Every
task is ticked done.

## The whole checks

`devenv shell -- sluice exec bolt/flumes-module-writes-its-sources`,
run once: **passed**, `exit 0`. sluice ran the checks `ef09a4b` affects
against the base: 15 tasks completed and 14 passed. It recorded the 14
results under their keys for `ef09a4b`.

The tasks it ran: `nix:format`, `nix:lint`, `nix:typecheck` (`nix:
typecheck: every case holds`), `nix:test` (`nix: test: every case
holds`), `flume:format`, `flume:lint`, `flume:test`, `conformance:test`,
`plugin-claude:test`, `root:format`, `root:adrs` (58 records, each with
a number of its own), `root:changes` (2 passed, this change among them),
`site:prose` (0 errors, 0 warnings), `site:build` (every internal link
valid) and `site:install`.

The run's warnings are moon's: its shared cache and output archives
could not be written from the gate's sandbox. No task failed for them.

Of note:

- `flume:test`: `the_modules_sources_read_whole` and
  `the_modules_files_read_whole` ok.
- `conformance:test`: `no_tool_is_named_outside_a_profile` (core.1) ok.
- `openspec validate flumes-module-writes-the-hosts-sources --strict`:
  valid. The three MODIFIED requirements are named in
  `openspec/specs/host-module/spec.md` as the delta names them.

## The spec: host-module

Each scenario is held by a case under `nix/tests` or by a Rust test,
each of which passed in the run above.

### ADDED

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| services.flume.sources declares each source's file | No kinds_dir | `scripts.sh` "No kinds_dir", over `fails.nix`'s `noKindsDir` | yes |
| | A kinds_dir under no home | `scripts.sh` "A kinds_dir under no home", over `kindsDirUnderNoHome` | yes |
| | A key the module does not declare in a source | `options.nix`, same name: no failure, `fromTOML` of the text is `kinds_dir` and `shelf` alone, the first line names the option | yes |
| A source's name is one flume takes | Names flume refuses | `options.nix`, same name: `flume`, `repo`, `launch`, `.acme`, `ac.me`, `a/b` each fail one assertion naming its option, quoted where no bare key, and why; `acme-2_b` passes | yes |
| Each source's file is rendered at evaluation | The sources render their files | `file.nix`, same name: `acme` and `tools` byte for byte, `every-key` and `empty` as before | yes |
| | A path value in a source | `scripts.sh`, same name, over `sourcePath`: names `services.flume.sources.acme.shelf` and the values the file takes | yes |
| flume reads the rendered sources whole | flume reads the rendered sources | `crates/flume/tests/host.rs` `the_modules_sources_read_whole`: `acme` and `tools`, each `HostSourceRead::Read`, `acme` under the home with its roles, `tools` at `/opt/kinds/tools` with none | yes |
| The activation links each declared source | Sources in a fresh home | `scripts.sh`, same name: both links, the folder holds them alone, the two lines | yes |
| | A source's link of an earlier activation | `scripts.sh`, same name: relinked, nothing aside, no name beginning with `.` | yes |
| | No source in a fresh home | `scripts.sh`, same name: exit 0, silent, no folder | yes |
| A declared source's file that is no link moves aside once | A hand-written file of a declared source | `scripts.sh`, same name: moved, linked, both paths named | yes |
| | A source's file aside already | `scripts.sh`, same name: non-zero, both paths named, both files as they were, `tools.toml` linked | yes |
| A sources that is no folder is left as it stands | A sources that is no folder | `scripts.sh`, same name: non-zero naming it, then exit 0 and silent with no source, the file as it was | yes |
| The activation removes only its own links | A source no longer declared | `scripts.sh`, same name: `acme.toml` removed and said, `tools.toml` linked, `mine.toml`, `theirs.toml` (leading where it did) and `old.toml.before-nix` as they were | yes |
| Each platform runs the sources step after the link | Sources on nix-darwin | `darwin.nix`, same name: no failure, `sources acme <file> tools <file> \|\| true` after the link as alice under `/Users/alice` | yes |
| | Sources on NixOS | `nixos.nix`, same name: the same step after the link inside the check that `id -un` is alice | yes |
| | No source declared | `darwin.nix`, same name, over nix-darwin and NixOS; and every case of both files that reads `links base` | yes |

### MODIFIED

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| flume reads the rendered file whole | flume reads the rendered files | `host.rs` `the_modules_files_read_whole`, passing over `sources/` | yes |
| Nothing shaped as a credential reaches the store | A token in an account's env | `options.nix`, as before | yes |
| | A variable named instead | `options.nix`, as before | yes |
| | A token in a source's file | `options.nix`, same name: names `services.flume.sources.acme.token`, holds no token's text | yes |
| The activation writes no other file | A hand-written file there | `scripts.sh`, as before | yes |
| | The sources beside the rest of the folder | `scripts.sh`, same name: linked, `config.toml`, `config.local.toml` and `flume.lock` unchanged by checksum, the folder holding them and `sources` alone | yes |

## The tasks

All nine are ticked, and each is in the branch:

| Task | Commit | Held |
|---|---|---|
| 1.1 Record 0055 amended, core.45's details | `fc1c03f5` | the amendment line under the status, the second point, boomdrive record 0034 among the sources; core.45's "Also rules out" and "Not ruled out" as design 8 has them, its words and every other rule unchanged; `root:format` and `root:adrs` pass |
| 2.1 The renderer takes its option | `ccdecc3c` | `toml.nix`'s `render option value`; `text` and `refusals` are `settings`', its header and failures unchanged |
| 2.2 `services.flume.sources`, the credential walk, the cases | `9df5dfc3` | `kinds_dir` and `roles` typed `/.*\|~\|~/.*`, `roles` null by default, free-form otherwise; the walk covers each source |
| 2.3 A source's name asserted | `06e239ad` | `nameFault`, one assertion per source |
| 2.4 flume's reader of the sources | `1a0bc31e` | both tests in `host.rs` |
| 3.1 The activation's `sources` step | `174279ef` | `activate.sh sources`, its usage line and its header comment |
| 3.2 Each platform runs it | `a7d62013` | `darwin.nix` and `nixos.nix`, each source as `flume-source-<source>.toml`, by the names' order, after the link, `\|\| true` |
| 4.1 The operator's pages | `ef09a4be` | `install.mdx` under "Install with Nix", `configuration.md` under "The host's sources"; no reader's own tool named (core.35) |

## Notes

None blocks the archive.

- `nix/modules/toml.nix`'s header comment carries one line that runs
  past the rest, where "Top-level keys come before tables" joined the
  line before it. nixfmt leaves comments as written, and `nix:format`
  passes.
- The rules the design relies on hold in the diff: core.1 (the module
  names no tool, `no_tool_is_named_outside_a_profile` ok), core.45 as
  amended, core.50 (the credential walk over the sources), core.13 (the
  step is the host's activation, no process of flume's) and core.21 (the
  rendered files are under `nix/tests/files/**`, which `treefmt.nix`
  excludes already).
