---
number: 36
title: A place is made ready by its merge queue's equipping, as the merge queue equips its own working copies
status: proposed
date: 2026-11-04
decision-makers:
- Chuck Swanberg
---

# A place is made ready by its merge queue's equipping, as the merge queue equips its own working copies

Status: proposed, for the user's yes on the design of the change `a-places-readiness-is-one-design-across-the-tools`. Nothing here binds until then.

Deciders: the user, in the brief of that change on 2026-11-04, after an answer was proposed to them without the analysis: "The thing is, Flume's job is to make the place ready. Sluice creates the branch, I believe, right? Flume was actually creating the work tree. I feel like you just gave me a solution without actually doing the analysis. I think you actually need a coding agent on [it], thinking about it comprehensively." And on why: "I don't want to feel like, too often, we're doing these point fixes and we're getting the solutions wrong ... We can't just be myopic, thinking about the one repo. We need to look at the related tools as a whole." The user's words of 2026-10-28 in record 0014 stand: one place profile, plain `git worktree`, "readied as sluice readies its own places". The analysis and the shape are the constructing agent's, put to the user in the change's design.
Sources: `agentplot/blueprints:design/flume/requirements.md` at ebfbc0d, the definition of readiness, clauses 11, 12, 13, 20, 79, C.1, C.2, C.6, C.7, S16; `design/sluice/requirements.md` at ebfbc0d, clauses 31, 33, 56, 84, C.1, C.2, section 9; records 0014 and 0027; sluice records 0013, 0018, 0045, 0050 and 0066; this repository's `crates/flume/src/profiles/merge_queue/sluice/mod.rs` (`for_place`), `crates/flume/src/profiles.rs` (`BoundPlace::data`), `crates/flume/src/profiles/place/git_worktree/mod.rs` (`ready`, `run`, `TOOLCHAIN`) and `devenv.nix` (`flume:proto:install`); `agentplot/sluice` at 571c83f6, `crates/sluice/src/profiles/moon.rs` (`plan`, `install`, `kept_out`), `crates/sluice/src/profiles/checks.rs` (`Checks::start`, `toolchain`), `crates/sluice-core/src/session.rs` (`session_request`) and `crates/sluice-core/src/preview.rs`; `afterthought/crew` at teamless/deneb 83a5cec0, `plugin/lib/kit.py`, `plugin/lib/plan.py` (`SUBMIT`), `devenv.nix` and records 0023, 0024 and 0028; `agentplot/boomdrive` at c5396d9, `crates/boomdrive/src/clone.rs`, `view.rs`, `flume.rs`, `docs/architecture/core.md` (core.14) and record 0009; `afterthought/kit-standard` at 20468ea, rules gate.2, gate.5, toolchain.2, toolchain.4 and toolchain.5; deneb, 2026-11-05, a place of a fresh clone failing `moon setup` after its toolchain passed.

## Context and problem statement

On deneb on 2026-11-05 Boomdrive's runner cloned a moon repository fresh, and flume launched a place there. The place's readiness ran the toolchain the policy names, `devenv print-dev-env`, which activates what `.prototools` pins and installs none, and passed. It then ran `moon setup`, which failed: the moon `.prototools` pins was not installed.

sluice equips every working copy it runs a moon check in by one sequence, outside its executor: the toolchain, its output kept as the environment of the steps after it; `proto install --config-mode upwards` where the working copy holds a `.prototools`; `moon --version`, refused below 2.6; moon's queries; and `moon setup` (`moon.rs`, `plan` and `install`). flume's sluice profile copied two of those steps, the toolchain and `moon setup`, into a place's readiness (`for_place`), to keep record 0014's words "readied as sluice readies its own places". The copy missed the install. A fix made in the place `fix--a-places-readiness-installs-its-repositorys-pinned-tools` copies the install too. It was held when the user asked for this analysis, and landed on main as #96 (d81d258) while the analysis was being done. Its copy decides from the pins at the base, where sluice's decides from the working copy, and it still lacks the version check.

What readies a working copy is today written in five places, and they disagree. sluice's candidate and session runs run the whole sequence. sluice's previews and `sluice exec -- <command>` run the toolchain alone (`preview.rs`; `session.rs`, `session_request`). sluice's reading at a submission runs it up to the queries, with no setup (`kept_out`). flume's readiness runs the toolchain and `moon setup`. And each repository's own `devenv.nix` installs its pins either in a devenv task that `print-dev-env` never runs, as flume's, sluice's and Boomdrive's do, or on every entry to the shell, as crew's does. Who should make a place ready, and how, so that one thing has one owner?

## Considered options

1. Copy sluice's steps into flume's sluice profile, as #96 does, and keep copying them whenever sluice's steps change.
2. sluice equips any working copy by one sequence, the one its own runs use, and offers it as one command, `sluice equip`; flume's readiness of a place is that command, asked at the revision the launch read, as its teardown is already `sluice release`.
3. The repository's toolchain installs what it pins, as crew's `enterShell` does, so that activating the toolchain makes a working copy ready.
4. No readiness: a place's agent readies it by entering the repository's shell, whose entry installs the pins.
5. A runner seam of flume's own, with a moon profile beside sluice's.
6. worktrunk's create hooks.
7. Readiness declared in the repository's `.config/flume.toml`.
8. sluice's preparation as a library that flume links.

## Decision outcome

Option 2, proposed.

1. **Who.** The repository declares which tools a working copy needs: the toolchain in its policy, the runner in its policy, and the versions in `.prototools` and the runner's own files. sluice owns how a working copy is equipped for the runner, by one sequence for every working copy it runs anything in. flume owns when a place is made ready, at its launch and at the retry of a first stage whose readiness failed (12), and what happens when it fails (12, S16). crew and Boomdrive ready nothing.
2. **What readiness is.** The toolchain the policy names; the runner's install of what the working copy pins; the runner's check of its own version; and the runner's setup. A check's services, ports and stacks are not readiness: the check brings them up itself (sluice 31, 84; sluice record 0066).
3. **Where and at which revision.** The steps are those of the policy at the revision the launch read (20), which flume names to sluice, so a retry reads them as its launch did (12). Each step runs in the place and reads what the place holds, its `devenv.nix` and its `.prototools`, which at the launch are that same revision.
4. **What changes.** sluice gains `sluice equip <branch> [--at <revision>] --json`, run in a working copy, and runs every working copy's preparation through it, a preview's and `sluice exec -- <command>`'s included. flume's sluice profile declares that command as the place's readiness, reading the steps it answers as the readiness's steps, and keeps no toolchain environment. flume stops reading `[tests] toolchain`, `[tests] runner` and `.prototools`, and stops naming `proto install` and `moon setup`. #96 stays on main until flume's build replaces its install along with the other two copied steps. Until then it is what readies a fresh clone's place, and reverting it first would bring deneb's failure back.

Option 1 keeps a second copy of what sluice's runner does. That copy has already drifted twice: first by missing the install, then, as #96, by reading the pins somewhere other than sluice does. It will drift again whenever sluice's runner changes. Option 3 puts an install into what sluice sources inside its executor, where nothing may be written outside the place. It costs crew 2.8 seconds of a warm entry's 3.2 at every command an agent runs, by crew's own measurement. And it is a readiness command the repository writes, which sluice's clause 84 rules out. Option 4 leaves a change's construct to fail where flume could have said why before an agent started (12, S16), and goes against the user's "Flume's job is to make the place ready". Option 5 is two moon profiles in one family. Option 6 runs the place's code with the host user's full permissions outside any executor (record 0014; sluice record 0013). Option 7 declares again what the policy and `.prototools` already declare. Option 8 ties flume's build to sluice's internals where every other reach of flume's into sluice is its pinned command line (C.6).

### Consequences

- C.1's ready column, C.2's `merge_queue` line, C.6's place row and readiness paragraph, C.7's line on the kept toolchain, clauses 11, 20 and 79, and the definition of readiness are amended in flume's statement, and S89 is added, as the change's design words them, once the user approves (core.19).
- sluice's statement gains clause 147, `equip`, and amends 33, 84, C.1, C.2 and section 9's givens, and S72 is added. sluice's change builds `sluice equip`, and flume's pin moves to the release that has it, by a chore of its own, before flume's build.
- Rule core.47 is drawn, and core.3's details are amended.
- crew's records 0023, 0024 and 0028, its `kit-toolchain` spec and its policy's comment stop saying crew readies a worktree. Boomdrive's record 0009 says a host's setup installs the host's tools, never a repository's pins. The kit standard gains a rule that the toolchain only activates, and that the runner's equip installs the pins.
- `<place's git directory>/flume/toolchain.sh` is no longer written. A place made before keeps it until its tidy, which removes it with the worktree.
- #96's requirement "A moon repository's pins installed before its setup" leaves merge-queue-profile with flume's build, and its scenario is carried as S89.
