# A place's readiness is one design across the tools

## Why

On deneb on 2026-11-05, Boomdrive's runner cloned a moon repository
fresh, and flume launched a place there. The place's readiness passed
its toolchain, `devenv print-dev-env`, which activates what
`.prototools` pins and installs none. It then failed `moon setup`,
because the moon that `.prototools` pins was not installed. sluice
installs the pins with `proto install --config-mode upwards` before
running moon in any working copy it runs checks in. flume's readiness
copied two of sluice's steps and missed that one. A point fix, made in
the place `fix--a-places-readiness-installs-its-repositorys-pinned-tools`,
copied the install too. It was held when this analysis was asked for,
and it landed on main as #96 (d81d258) while the analysis was being
done.

The user asked for the analysis, across the tools rather than in this
one repository: "Flume's job is to make the place ready. Sluice creates
the branch, I believe, right? Flume was actually creating the work tree.
I feel like you just gave me a solution without actually doing the
analysis", and "I don't want to feel like, too often, we're doing these
point fixes and we're getting the solutions wrong ... We need to look at
the related tools as a whole."

The analysis, read in the code of flume, sluice, crew and Boomdrive,
found that what readies a working copy is written in five places, and
that they disagree:

- sluice's whole sequence, for candidates and session runs;
- sluice's toolchain alone, for previews and `sluice exec -- <command>`;
- sluice's sequence without setup, at a submission;
- flume's toolchain and setup, without the install;
- each repository's own `devenv.nix`, which installs in a task or on
  every entry to the shell.

crew and Boomdrive ready nothing.

## What Changes

- **The decision proposed.** Record 0036, proposed: a place is made
  ready by its merge queue's equipping of it, as the merge queue equips
  the working copies of its own runs. The repository declares which
  tools a working copy needs, sluice owns how a working copy is equipped,
  and flume owns when a place is made ready and what a failure does.
  crew and Boomdrive ready nothing. On the user's yes, rule core.47 is
  drawn, core.3's details are amended, and flume's statement is amended
  as the design's decision 7 words it (11, 20, 79, C.1, C.2, C.6, C.7,
  the definition of readiness, S89).
- **One step, sluice's.** A place's readiness is
  `sluice equip <base> --at <revision> --json`, run in the place in the
  host user's login environment. The revision is the one the launch
  read, at the launch and at a retry (12, 20). Each step it answers is
  one of the readiness's steps, with its outcome and its words, as
  `toolchain` and `moon setup` are today. flume stops reading
  `[tests] toolchain` and `[tests] runner`, stops naming `moon setup`,
  and no longer keeps `<git-dir>/flume/toolchain.sh`.
- **#96 stays until the build replaces it.** Until then its copied
  install is what readies a fresh clone's place. The build takes the
  install out with the toolchain and the setup, and removes its
  requirement "A moon repository's pins installed before its setup" from
  merge-queue-profile. Its scenario, a fresh clone whose pins nothing
  installed, becomes S89, held by this change's tests against the pinned
  sluice.
- **Changes named in other repositories, launched there on the user's
  yes and not written here:**
  - agentplot/sluice: `sluice equip`, with every one of sluice's own
    paths equipping through it, and clause 147 and S72.
  - afterthought/crew: its records and `kit-toolchain` spec stop saying
    crew readies a worktree, and its `enterShell` stops installing.
  - agentplot/boomdrive: record 0009 says a host's setup installs no
    repository's pins.
  - afterthought/kit-standard: rule toolchain.8, that the toolchain only
    activates and the runner's equip installs.
- **The site** says what a place's readiness is, and drops the kept
  toolchain from the state reference.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `merge-queue-profile`: the place's data is `sluice equip` at the
  launch's revision, with the reading of its steps, and the teardown and
  the base's remote as before; the toolchain and the runner's setup
  leave it, and #96's requirement "A moon repository's pins installed
  before its setup" is removed.
- `place-profile`: readiness is the one step the merge queue declares,
  run at the revision the place's launch read, its answer read as the
  readiness's steps, nothing kept in the place's git directory, and S89.
- `launching`: a retry of a first stage whose readiness failed readies
  the place at the revision its launch read, and S89 over the fake.
- `clause-coverage`: the suite reads the statement at the revision that
  holds the amendment, and cites S89.

## Impact

- **Before the build.** The user's yes on the design and record 0036.
  agentplot/blueprints: flume's and sluice's statements amended on main
  as the design's decision 7 words them (core.19). agentplot/sluice: a
  release with `sluice equip`. Then a chore of this repository's moves
  `devenv.nix`'s `sluiceVersion` from 0.57.1 to that release, checking
  C.6's commands and fields against it (core.22). This change builds
  after all three.
- `docs/adr/0036-…`: proposed here, and accepted on the user's yes.
  `docs/architecture/core.md`: core.47, and core.3's details.
- `crates/flume`: `profiles/merge_queue/sluice/mod.rs` (`for_place`,
  `equipped`; `ForPlace` loses `toolchain`, `install` and `setup`, and
  `PINS` goes), `profiles.rs` (`place_data` takes the revision,
  `BoundPlace`), and `profiles/place/git_worktree/mod.rs` (`Step`,
  `PlaceData`, `ready`, `run`; `TOOLCHAIN` goes). The profile tests are
  `place_git_worktree.rs`, which loses #96's two install tests, and
  `tools/merge_queue_sluice.rs`. S89 is held under the pinned sluice by a
  test of its own in `tools/`.
- `crates/flume-conformance`: the clause list; S89 cited by an engine
  scenario in `engine/launching.rs`; the place seam's
  `ready_installs_the_pins` kept, with the git worktree harness giving
  no pins in check-fast.
- `crates/flume-seams` and `crates/flume-core`: unchanged. `Readiness`,
  `ReadyStep`, `{{unready}}` and the launch's and retry's revision keep
  their shape (core.14).
- `site/`: `guides/places-and-herdr.mdx`, `guides/merge-queue.md`,
  `reference/state.md`, `architecture/overview.md`, and the rules page
  mirroring core.47.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency.
