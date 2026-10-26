# Tasks

## 1. Rust in the gate, and the seams' crate

- [x] 1.1 Pin Rust 1.98.1 in `.prototools`, have `.moon/toolchains.yml` read it with `versionFromPrototools` and the clippy and rustfmt components, add the cargo workspace `Cargo.toml` at the root with serde and serde_json as workspace dependencies, ignore `target/`, have `treefmt.nix` claim `*.rs` with the pinned toolchain's rustfmt and exclude `target/**` with its reason, name the cargo checks and the coverage check in `CLAUDE.md`'s check-fast row and in `.moon/workspace.yml`'s layer comment (kit:testing.2), and remove the item that the gate has no code from `docs/architecture/_open.md` (design 14; core.21, core.22). Then add `crates/flume-seams` to `members` and as moon project `seams` in `.moon/workspace.yml`, language rust, with check-fast tasks `format`, `lint` and `test`, one check each, each declaring its inputs with `/devenv.nix`, `/devenv.lock` and `/.prototools` (kit:testing.3). True when `devenv shell -- moon run seams:format seams:lint seams:test` passes.

## 2. The types and the seven traits

- [x] 2.1 Add the types the seams pass: a launch, a place's held state, a kind with its stages as read-kinds returns them, the six deliverable forms, a prompt and its parts, an identity, a start secret and a stop-short, a submission and its outcomes, agent and stage states, and each seam's unknown and could-not-read answers (design 2, 4), each serializing to JSON and back unchanged. True when `devenv shell -- moon run seams:test` passes with a round-trip test per type.
- [x] 2.2 Add the seven traits `Place`, `Kinds`, `Change`, `Agent`, `Terminal`, `MergeQueue` and `Store`, each with the operations its section of Part B names and the three of design 2 beyond them, and the `History` and `Host` traits of design 3, with no tool named in any (core.1, core.2). True when `devenv shell -- moon run seams:lint seams:test` passes.

## 3. The fakes and the seam scenarios

- [x] 3.1 Add `crates/flume-fakes` to `members` and as moon project `fakes` with check-fast `format`, `lint` and `test` declaring their inputs, holding one scriptable fake per seam and for `History` and `Host`: every answer settable, refusals and unknowns among them, every call logged in order, the clock moving only when advanced or slept on, and one resolver from profile names to fakes. True when `devenv shell -- moon run fakes:format fakes:lint fakes:test` passes.
- [x] 3.2 Add `crates/flume-conformance` to `members` and as moon project `conformance` with check-fast `format`, `lint` and `test` declaring their inputs, holding the seam scenarios, one file per seam, each scenario generic over its trait, citing its clauses as data, and sitting below its seam's marker, run here against the fakes, every operation and guarantee of Part B exercised (seam-contracts, seam-fakes specs). True when `devenv shell -- moon run conformance:format conformance:lint conformance:test` passes.
- [x] 3.3 Add the clause list of Parts A and B, I1 to I8 and S1 to S34 at the statement's revision 6246a48, and the coverage check as a test of its own logic and a binary run by a check-fast `coverage` task declaring its inputs, failing on a clause, invariant or scenario cited by nothing, a scenario citing nothing, or a citation of no clause (clause-coverage spec). True when `devenv shell -- moon run conformance:test` passes, its tests of the check among them; the `coverage` task passes only once group 9 is done.
- [x] 3.4 Add a test that searches the sources of `crates/` for `wt`, `worktrunk`, `openspec`, `claude`, `herdr` and `sluice` as words and fails on a hit outside a `profiles` directory (core.1). True when `devenv shell -- moon run conformance:test` passes.

## 4. The core: held state and launches

- [x] 4.1 Add `crates/flume-core` to `members` and as moon project `core` with check-fast `format`, `lint` and `test` declaring their inputs, holding the engine built over a resolver, the host's terminal and store, `History`, `Host` and `Defaults` (design 2, 3, 10). True when `devenv shell -- moon run core:format core:lint core:test` passes.
- [x] 4.2 Build the held state's document and its versioned read and write, with a refused write re-read and decided again (design 4; held-state spec). True when `devenv shell -- moon run core:test conformance:test` passes, with the held-state scenarios in the suite.
- [x] 4.3 Build the launch: resolution of the kind over the three declarations and of the binding, the base revision from the merge queue seam or the local base, every refusal of the launching spec made before anything is made or undone after, readiness, and the first stage started, held or waiting (design 4; 1 to 12, 19 to 23). True when `devenv shell -- moon run conformance:test` passes with the launching scenarios, S11, S13, S15, S16, S22, S31 and S33 among them.

## 5. The core: a stage's run and its end

- [x] 5.1 Build a stage's start: settings resolved launch over stage over `Defaults` (32), the identity and a fresh secret per start, the prompt rendered with every part of design 6 or the stage refused naming the part, the agent seam's command and the terminal's open, and rendering with nothing started (27 to 33). True when `devenv shell -- moon run conformance:test` passes with S32 and the prompt scenarios.
- [x] 5.2 Build delivery of a prompt: only to a waiting agent not asking, read within 60 seconds, sent once more where the terminal can send, never begun otherwise, the start judged by the agent's existence (design 6; 35, 37, 38). True when `devenv shell -- moon run conformance:test` passes with S5.
- [x] 5.3 Build the reading of a stage's end at one head for the six forms, quiet from the terminal's and agent seam's readings with unknown as not quiet, stuck, working past its limit and lost, and a conversation that ends only on finish (design 5; 24, 25, 31, 39, 40, 44, 45, 109, 118). True when `devenv shell -- moon run conformance:test` passes with S2, S3, S17 and S23.
- [x] 5.4 Build `needs` and the stop check: a stop-short taken only under the current start's identity and secret while its agent is live, the stage ended short with the words whole, and the stop check's words naming the deliverable and the way to stop short (design 8; 41, 42, 110, 115); and the guard's refusals read into status (108, 116). True when `devenv shell -- moon run conformance:test` passes with S4, S24 and S25.

## 6. The core: settling and submissions

- [x] 6.1 Build `settle` for one place and every place, run attended before every acting command where the terminal keeps no watch, and by nothing for status or a rendering (design 7; 46, 119 to 122). True when `devenv shell -- moon run conformance:test` passes with S7, S27, S28 and S29.
- [x] 6.2 Build the submission: submitted at the head once the last stage delivered and its agent ended, unless held or over; the waiter recorded and the verdict written; landed making the work over; failed sending the place back to `on_fail` with the verdict's words up to the kind's count; refused and withdrawn waiting; a landing read before flume submits taken as the landing; an open submission nobody waits on waited on again (design 7; 63, 111 to 113, 124, 125). True when `devenv shell -- moon run conformance:test` passes with S1, S8, S19, S30, S34 and the count-reached scenario.

## 7. The core: caller acts, status and tidying

- [x] 7.1 Build retry, stop, drop, done, finish and tell, each ending an agent only when quiet or forced and answering a close the terminal refused as a failure, a retry of a first stage whose readiness failed running it again (caller-acts spec; 12, 31, 47 to 55, 109, 114). True when `devenv shell -- moon run conformance:test` passes with S4, S6, S7, S9, S20, S21 and S26.
- [ ] 7.2 Build status of one place or every place, every fact of clause 56 read from its seam at one head per place, nothing written and nothing started (place-status spec; 43, 56 to 62). True when `devenv shell -- moon run conformance:test` passes with S14.
- [ ] 7.3 Build the tidy: only places over with no live agent, kept on what the place seam reads them holding or on a failed teardown, the branch removed only through remove-branch's judgement and named once when kept, nothing removed while the merge queue cannot be read, the held state and the place's folder gone last (tidying spec; 16, 62 to 68). True when `devenv shell -- moon run conformance:test` passes with S9, S10 and S18.
- [ ] 7.4 Build the core rebuilt over the same store and seams reading every place again, an agent that did not survive read lost and a resumed one taken only when its identity and its own record match (77). True when `devenv shell -- moon run conformance:test` passes with S12 and the resumed-agent scenario.

## 8. Answers and the binary's crate

- [ ] 8.1 Make every act's answer one serde type written as JSON, a refusal among them, and add each answer's field list to the suite with a test that fails when an answer lacks a listed field (design 9; 61). True when `devenv shell -- moon run conformance:test` passes.
- [ ] 8.2 Add `crates/flume` to `members` and as moon project `flume` with check-fast `format`, `lint` and `test` declaring their inputs, holding `src/main.rs`, which builds the core and writes an answer as JSON with no command yet, and `src/profiles.rs`, holding one marker comment per seam where a profile unit appends its module line (design 13). True when `devenv shell -- moon run flume:format flume:lint flume:test` passes.

## 9. Coverage and the checks

- [ ] 9.1 Cite, across the seam and engine scenarios, every clause of Parts A and B, I1 to I8 and S1 to S34, adding a scenario wherever a clause has none (clause-coverage spec). True when `devenv shell -- moon run conformance:coverage` passes.
- [ ] 9.2 Run the checks of every project this change touches. True when `devenv shell -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test core:format core:lint core:test conformance:format conformance:lint conformance:test conformance:coverage flume:format flume:lint flume:test` passes.
