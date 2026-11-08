# Tasks

One agent builds this change, task by task in order, reading the
proposal, the design and the specs first. The groups are headings for
reading. Each task names the narrowest check that shows it done, run as
written; none runs the repository's whole checks, which the verify stage
runs once after every task is done (record 0054). Every git call in a
test runs under the scratch environment `crates/flume/tests/common`
gives, and tests reach no real account, no repository of the user's, no
person's queue and no network.

weir's record 0005, the user's, accepted on their card on 2026-11-07,
spells a kind's landing fix `landing_fix`, an identity with its version,
and its bound `max_landing_fixes`. weir's main reads both at `daacda3`,
and the statement carries them in 23 at agentplot/blueprints 8eec6dd.
Record 0056 and its amendment decide what flume does with them; the
core, the fakes and the conformance suite already do it, and this
change touches none of them. Every scenario added cites its clauses.

## 1. The decision recorded

- [ ] 1.1 Hold record 0056's amendment line, core.34's rule and details, core.4's details and the mirror of core.34 in `site/src/content/docs/architecture/rules.md` as this change's construct wrote them at 15d15448, as design 7 words them, citing weir's record 0005 and the statement at agentplot/blueprints 8eec6dd; amend them where the statement the build reads differs (core.19, core.35). True when `devenv shell -- sh check-adrs.sh`, `devenv shell -- treefmt --ci docs/adr/0056-a-base-place-forecast-or-found-to-conflict-with-its-base-or-whose-submission-failed-has-its-fix-launched-by-flume.md docs/architecture/core.md site/src/content/docs/architecture/rules.md` and `devenv shell -- sh -c 'cd site && vale src/content/docs/architecture/rules.md'` pass.

## 2. weir's crate

- [ ] 2.1 Move weir-kinds' pin in `Cargo.toml` to weir's main at `daacda39e59469029b2841f889c6cbd8cd7839d9`, and `Cargo.lock` with it, weir-kinds and weir-capture both, as design 1 has it, leaving the comment above the pin as written (core.4, core.22). True when `devenv shell -- cargo test -p flume --test profiles kinds_weir` passes and `git grep -n 5562af5 -- Cargo.toml Cargo.lock` finds nothing.

## 3. The kinds profile

- [ ] 3.1 Have `map_kind` in `crates/flume/src/profiles/kinds/weir/mod.rs` map weir's `landing_fix` onto `Kind::fix`, spelled by weir's `Display`, and `max_landing_fixes` onto `Kind::fix_limit`, a count past `u32` a problem `kinds.<kind>.max_landing_fixes: is more than flume counts`, as design 2 has it, and from nothing else. Say so in `map_kind`'s doc and the module's, and in the docs of `Kind::fix` and `Kind::fix_limit` in `crates/flume-seams/src/kinds.rs` as design 3 words them. Add the kinds-profile spec's "A landing fix and its bound over a scratch repository" to `crates/flume/tests/profiles/kinds_weir.rs` as `a_kind_with_no_stages_names_its_landing_fix_and_its_bound`, and name the landing fix in the module's doc (23; weir 52; weir record 0005; core.4). True when `devenv shell -- cargo test -p flume --test profiles kinds_weir::a_kind_with_no_stages_names_its_landing_fix_and_its_bound` passes.
- [ ] 3.2 Add the kinds-profile spec's "A landing fix weir refuses" to `kinds_weir.rs` as `a_landing_fix_weir_refuses_is_named_in_weirs_words` (57; weir record 0005). True when `devenv shell -- cargo test -p flume --test profiles kinds_weir::a_landing_fix_weir_refuses_is_named_in_weirs_words` passes.
- [ ] 3.3 Add the kinds-profile spec's "A bound past what flume counts" to `kinds_weir.rs` as `a_landing_fix_bound_past_what_flume_counts_is_refused` (4). True when `devenv shell -- cargo test -p flume --test profiles kinds_weir::a_landing_fix_bound_past_what_flume_counts_is_refused` passes.
- [ ] 3.4 Add the kinds-profile spec's "The fields' old spellings" to `kinds_weir.rs` as `fix_and_max_fixes_are_sections_naming_no_landing_fix` (23; weir record 0005; core.4). True when `devenv shell -- cargo test -p flume --test profiles kinds_weir::fix_and_max_fixes_are_sections_naming_no_landing_fix` passes.
- [ ] 3.5 Add the kinds-profile spec's "A package's landing fix" to `kinds_weir.rs` as `a_packages_kind_names_its_landing_fix`, over a scratch `kinds_dir` with the module's package helpers (157; weir 89). True when `devenv shell -- cargo test -p flume --test profiles kinds_weir::a_packages_kind_names_its_landing_fix` passes.

## 4. A base place's launch through the profile

- [ ] 4.1 Add the kinds-profile spec's "The landing fix held through the profile" to `kinds_weir.rs` as `a_base_places_launch_holds_the_landing_fix_the_profile_reads`, the engine built by `common::engine::engine` with the profile in place of the fake kinds and the scratch declaration binding the fakes' merge queue as design 5 has it (21, 56, 203). True when `devenv shell -- cargo test -p flume --test profiles kinds_weir::a_base_places_launch_holds_the_landing_fix_the_profile_reads` passes.
- [ ] 4.2 Add the kinds-profile spec's "A landing fix at a version the repository does not declare" to `kinds_weir.rs` as `a_landing_fix_at_a_version_not_declared_refuses_the_launch` (4, 154; core.8, core.34). True when `devenv shell -- cargo test -p flume --test profiles kinds_weir::a_landing_fix_at_a_version_not_declared_refuses_the_launch` passes.

## 5. The guides and the reference

- [ ] 5.1 Give `site/src/content/docs/guides/kinds.mdx`'s "A kind with no stages" `landing_fix` and `max_landing_fixes` in its example, naming `flume/fix@1`, and say as design 6 has it what the identity needs, the bound left out and the refusal on a kind with stages; list both in "A kind's fields", to any host's operator (23, 157; core.35). True when `devenv shell -- sh -c 'cd site && vale src/content/docs/guides/kinds.mdx'` passes.
- [ ] 5.2 List `landing_fix` and `max_landing_fixes` in `site/src/content/docs/reference/configuration.md`'s "A kind", and have its `stages` row say `[]` on a kind that lands (23; core.35). True when `devenv shell -- sh -c 'cd site && vale src/content/docs/reference/configuration.md'` passes.
- [ ] 5.3 Have `site/src/content/docs/guides/merge-queue.md`'s "Its fixes" give the two fields, with an example of `train` naming `flume/fix@1`, in place of saying a declaration has no field for either, keeping its paragraph on a kind that names no fix (23, 203; core.35). True when `devenv shell -- sh -c 'cd site && vale src/content/docs/guides/merge-queue.md'` passes.
