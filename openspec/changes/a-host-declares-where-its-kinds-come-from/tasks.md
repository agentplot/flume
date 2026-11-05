# Tasks

Each group is one session's work for an agent that reads this change
(proposal, design, specs) and its group, and builds on main as this
branch merged it at 201086c and the groups before it. A group's last
task runs the checks it touched through sluice; every conformance scenario a group adds
cites its clauses as data (106). Tests reach no real account and no
configuration or data directory of the user's: every git call in a test
runs under the scratch environment `crates/flume/tests/common` gives,
and every `XDG_CONFIG_HOME`, `XDG_DATA_HOME`, sources directory, lock and
`kinds_dir` a test names is a scratch directory or file of the test's
own. The user approved the design on 2026-11-05, and the statement
states the ruling on agentplot/blueprints' main at ff616eb. Group 2 and
the groups after it are built once weir-kinds reads packages,
identities, hashes and locks (design, *What this change rests on*).

## 1. The decision recorded

- [x] 1.1 Write record 0040 in `docs/adr/`, accepted as the user ruled on this change's design, holding what the record carries beyond record 0022:
  - the host's sources, one file each in its configuration's sources directory (decision 1);
  - a source holding packages and never a file of declarations (decision 2);
  - a bare name never reaching a host's source (decision 4);
  - a lock that cannot be read refusing only the launches it bears on (decision 5);
  - `flume kinds lock --hash` (decision 6);
  - status naming where a kind came from (decisions 8, 9).

  Its sources are record 0022 and the statement at ff616eb, which states the ruling, and its considered options are the design's. Amend core.34's details in the same commit:
  - **Rules out:** a bare name resolved to a kind of a host's source; a source's file, or anything in a `kinds_dir`, written by flume; a source's fault refusing a launch that does not name its source.
  - **Not ruled out:** a program that manages the host writing a source's file and running `flume kinds lock --host --hash`.
  - **Sources:** the record among them.

  True when `sluice exec main -- moon run root:adrs root:format` passes (decisions 1 to 9; 154 to 160, 191, 192; core.19, core.34).
- [x] 1.2 Merge main at 201086c into the change, and fit the proposal, design, specs and tasks to it: the record numbered 0040, since 0030 to 0039 are taken; no `_open.md`, which main no longer holds; the Boomdrive runner (record 0034); the sources the managed layer's (C.7); and the statement's 191, 192 and S95 to S97, as blueprints states them at ff616eb, cited (core.19). True when `sluice exec main -- moon run root:changes root:format` passes.
- [x] 1.3 Cite the statement's amendment where blueprints' main took it, at ff616eb, in record 0040, the proposal, the design and these tasks, and have the clause list's scenario hold every scenario that revision holds, which numbers none from S92 to S94 (core.19). True when `sluice exec main -- moon run root:adrs root:changes root:format` passes.

## 2. weir-kinds' pin

- [x] 2.1 Move the `weir-kinds` pin in `Cargo.toml`, with `Cargo.lock`, to the revision of agentplot/weir's main that reads an identity, a package with its content hash, and a lock, and writes a lock (*What this change rests on* 1; weir 88 to 93, C.4). Move nothing else. Where weir's main holds no such revision, the build cannot go on: stop short naming it (core.4, core.22). True when `sluice exec main -- moon run seams:test core:test fakes:test conformance:test flume:lint flume:test` passes with every existing test unchanged.

## 3. The seams and the fakes

- [x] 3.1 Add to `flume-seams` the following, as design 10 has them (19, 21, 154 to 160; core.4, core.14):
  - the types `KindIdentity`, `KindNamed`, `Packaged`, `PackageRead`, `SourceRead`, `LockEntry`, `LockRead` and `KindFrom`;
  - `Declared::Host`;
  - `PlaceRecord::kind_from`, read with a default so held state written before it reads with none;
  - in `flume-core`'s answer, `PlaceStatus::identity` and `PlaceStatus::kind_from`, and the host part's `sources`, each written only where set;
  - the kinds seam's `read_named`, `read_source`, `read_package`, `read_lock_at`, `read_lock_file` and `write_lock`, each refusing by default, so a profile not yet built answers that it reads none;
  - `flumes_own` answering `Packaged`, the kinds profile reading each version of `OWN` with its identity and its hash over `OWN`'s paths and bytes through weir-kinds, and the core taking the kind from each as before;
  - `Host::sources()`, each a `HostSource`.

  True when `sluice exec main -- moon run seams:test flume:test` passes with `round_trip.rs` holding the seam-contracts spec's held state with and with no `kind_from`, a value of each new type round-tripping, and every existing test of flume's own kinds passing unchanged.
- [x] 3.2 Have `FakeKinds` and `FakeHost` script what the seam-fakes spec and design 10 list, logging each call. Add the new operations' scenarios to `crates/flume-conformance/src/seams/kinds.rs` below the seam's marker, generic over the trait, each citing its clause (design 10; 85, 154 to 157, 160). True when `sluice exec main -- moon run seams:test fakes:test conformance:test` passes with the seam-contracts spec's *Each operation over the fake*, and a `fakes.rs` test of each scripted reading.
- [x] 3.3 Run the checks of the projects this group touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test conformance:format conformance:lint conformance:test core:lint core:test flume:lint flume:test` passes.

## 4. The kinds profile and the host's sources

- [x] 4.1 Give the kinds profile, through weir-kinds alone, each of the following, and name each in `profiles/kinds/weir/mod.rs`'s module doc (design 2, 7, 10; 155 to 160, C.2; core.4):
  - `read_named`;
  - `read_source`, which ignores entries of any other shape;
  - `read_package`, with 157's and 158's refusals naming the field;
  - `read_lock_at`, the repository's `.config/flume.lock` at a revision, read as git gives it;
  - `read_lock_file`;
  - `write_lock`, written beside the lock and renamed over it.

  True when `sluice exec main -- moon run flume:test` passes with `kinds_weir.rs` holding, over scratch directories and repositories, the kinds-profile spec's five scenarios, and the kind-sources spec's *A package at another identity* and *A part outside the package and an environment* at the profile.
- [x] 4.2 Add to `crates/flume/tests/profiles/built_in.rs` a test that holds the hash of each version of flume's own kinds, as 4.1 computes it over `own/` as it stands, and fails naming the version whose hash moves (design 7; 156). True when `sluice exec main -- moon run flume:test` passes with the kind-sources spec's *The hashes recorded*, and fails with one byte of change@1's code prompt changed.
- [ ] 4.3 Have `crates/flume/src/host/config.rs` read `sources/` beside `config.toml` as design 1 has it. It reads each `.toml` file whose name begins with no `.`, its name the source's and `kinds_dir`'s leading `~` the home, and ignores every other entry. Each fault is kept as that source's, never failing the configuration. Give the core `Host::sources()` from it, and name the sources in the module doc (design 1; 159, C.7). True when `sluice exec main -- moon run flume:test` passes with `crates/flume/tests/host.rs` holding, over a scratch `XDG_CONFIG_HOME`, the kind-sources spec's three scenarios of *The host declares each source in a file of its own*, and each fault its second requirement names read as that source's.
- [ ] 4.4 Run the checks of the project this group touched. True when `sluice exec main -- moon run flume:format flume:lint flume:test` passes.

## 5. The core resolves an identity and checks the locks

- [ ] 5.1 Have `resolve_kind` in `crates/flume-core/src/launch.rs` do the following, as decisions 3 to 5 have it (design 3 to 5, 8, 11; 4, 19 to 22, 132, 154 to 160; core.4, core.8, core.34):
  - take the kind's text through `read_named` and drop `named_version`;
  - search as design 3's table says, a bare name as decision 4 says;
  - pick a version named none as decision 5 says;
  - read the repository's lock at the base's revision and the host's lock, as the configuration stands, only where the launch's kind bears on them;
  - refuse, in the design's words and before anything is made, a kind from outside that no lock records, a hash that is not a lock's, two locks giving one identity two hashes, and a lock that cannot be read;
  - return `kind_from`, which the launch writes into the place's held state.

  Have a role's wake resolve its `kind` the same way. True when `sluice exec main -- moon run core:test conformance:test` passes with:
  - a new `engine/kind_sources.rs` holding each scenario of the kind-sources spec that the core decides over the fakes: every scenario of its requirements on a source's fault, side-by-side packages, an identity, a bare name, locks, the version taken, a lock of flume's own and a place keeping its kind;
  - `engine/launching.rs` gaining the launching spec's scenarios;
  - `engine/role_waking.rs` the role-waking spec's;
  - the seam-contracts spec's *A kind's text read by the seam* and *Sources scripted on the fake host*, and the seam-fakes spec's *A scripted package edited after a launch*;
  - each citing its clauses, S59 to S63 among them, and every existing scenario passing unchanged.
- [ ] 5.2 Have status give a place's `identity` and `kind_from` as its held state holds them, the source from `declared` where it holds none, and `declared` `host`. Have the host part give `sources` from `Host::sources()` and `read_source`, read when asked (design 8, 9; 19, 21, 56; core.14, core.15). True when `sluice exec main -- moon run core:test conformance:test` passes with `engine/place_status.rs` gaining each scenario of the place-status spec.
- [ ] 5.3 Add `crates/flume-core/src/kinds_lock.rs`, `Engine::lock_kinds`, as design 6 has it (design 6; 74, 155, 156; core.16, core.17):
  - each whole identity read through `read_named` and resolved to its content;
  - its hash checked against `--hash`;
  - `repo` and `launch` refused;
  - the lock read, never overwritten where it cannot be read, its entries added or removed, and written through `write_lock`;
  - answered as `Answer::Locked` with the lock's file and its entries.

  True when `sluice exec main -- moon run core:test conformance:test` passes with `engine/kind_sources.rs` gaining the kind-sources spec's *A hash the content does not have*, *An identity no lock records* and the removal of *Two writers at once, and a removal* over the fakes.
- [ ] 5.4 Run the checks of the projects this group touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test core:format core:lint core:test conformance:format conformance:lint conformance:test flume:lint flume:test` passes.

## 6. The command line

- [ ] 6.1 Have `flume launch --kind` and a role's `kind` pass the text whole. Add `flume kinds lock [--host] [--remove] [--hash <hash>] <identity>…` to `commands/mod.rs`, answering through the core in the JSON and person's forms. Without `--host` it writes the lock of the working copy it runs in. It holds an advisory lock kept under flume's state directory while it reads and writes the lock, and leaves nothing else in the working copy (design 6; 74, 155; core.14, core.16, core.17). True when `sluice exec main -- moon run flume:test` passes with `crates/flume/tests/commands.rs` holding, through the binary over a scratch repository and a scratch host:
  - the kind-sources spec's five scenarios of *flume kinds lock records a kind's content*, the two writers run as two processes at once;
  - its *Unlocked, then locked by the repository*;
  - its *A lock that cannot be read*;
  - its *A file with no kinds_dir*.
- [ ] 6.2 Give the person's form of status the kind's line and the host's sources as design 9 has them (design 9; 56). True when `sluice exec main -- moon run flume:test` passes with `print.rs` unit tests of the place-status spec's three places and the host's sources with a fault.
- [ ] 6.3 Add to `crates/flume/tests/profiles/tools/walkthroughs.rs` design 13's walkthrough under the printed command with the stand-in (design 12, 13; 21, 154, 155, 159):
  - a scratch host's source `acme` holds `review@1`, a one-stage kind that commits and lands, locked by `flume kinds lock --host --hash`;
  - a launch of `acme/review` lands, status naming `acme/review@1` from the source's file and the kind's directory with its hash, locked in the host's lock;
  - `review@2` is added and locked, and `acme.toml` is renamed over by one naming a directory that holds `review@2` alone;
  - a second launch of `acme/review` runs review@2, and a place of review@1 launched before the rename runs on to its landing.

  True when `sluice exec main -- moon run flume:tools` passes with it, and every existing walkthrough passes unchanged.
- [ ] 6.4 Run the checks of the project this group touched. True when `sluice exec main -- moon run flume:format flume:lint flume:test flume:tools` passes.

## 7. The site and `/launch`

- [ ] 7.1 Write what design 14 lists into the site, all to any host's operator, naming no host, team or program of ours (design 12, 14; core.35):
  - `site/src/content/docs/guides/kinds.mdx`;
  - `reference/configuration.md`;
  - `reference/commands.md`;
  - `reference/state.md`;
  - `guides/roles-and-wake.md`.

  True when `sluice exec main -- moon run site:prose site:build` passes.
- [ ] 7.2 Have `plugins/claude/commands/launch.md` say that a kind of a source the host declares is named `<source>/<name>`, beside flume's own and the repository's (design 14; core.35). True when `sluice exec main -- moon run plugin-claude:format plugin-claude:validate plugin-claude:test` passes.
- [ ] 7.3 Run the checks of the projects this group touched. True when `sluice exec main -- moon run site:prose site:build plugin-claude:format plugin-claude:validate plugin-claude:test root:format` passes.

## 8. Coverage and the gate

- [ ] 8.1 Have the suite's clause list in `crates/flume-conformance/src/clauses.rs` hold the following, as the clause-coverage spec says (clause-coverage spec; 154 to 160, S59 to S63):
  - Part A, Part B, I1 to I8 and the scenarios at the statement's revision that states the user's ruling on this change, or the later one on blueprints' main when the group is built;
  - `REVISION` and `SCENARIOS` moved to it;
  - `PACKAGES` and its entries gone from `unbuilt`;
  - the clauses and scenarios stating the host's sources cited by this change's scenarios in `engine/kind_sources.rs`;
  - `unbuilt`'s test renamed for what it then names.

  True when `sluice exec main -- moon run conformance:test conformance:coverage` passes.
- [ ] 8.2 Run the checks of the projects this change touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test core:format core:lint core:test conformance:format conformance:lint conformance:test conformance:coverage flume:format flume:lint flume:test flume:tools plugin-claude:format plugin-claude:validate plugin-claude:test site:prose site:build root:format root:adrs root:changes root:specs` passes.
