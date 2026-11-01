---
number: 28
title: Statechart kinds come as a second kind format beside today's simple kinds, on one branch, built after crew's rewrite onto flume
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# Statechart kinds come as a second kind format beside today's simple kinds, on one branch, built after crew's rewrite onto flume

Status: accepted by the user on 2026-11-01, on the updates ledger of agentplot/flywheel-cloud's domain model ("yes to all", item ty-v1), as they ruled it the same day on their Pending You card (req_f1987285, "One branch: statecharts as a second kind format beside today's").

Deciders: the user, in ruling T1 of 2026-10-31 on agentplot/flywheel-cloud's domain model, "Does Flume run the kinds as statecharts?": "Yes, as kinds v2, after crew is rewritten. Flume's simple kinds stay as v1 until then."; in the ledger's item ty-v1, "An ADR that Flume's simple kinds stay as they are, and that statechart kinds are v2, sequenced after the crew rewrite", accepted with the ledger on 2026-11-01; and on their card the same day: "One branch: statecharts as a second kind format beside today's". Simple kinds stay as they are, statechart kinds come as a second format beside them on the same branch with no release split, and statechart kinds are built after crew's rewrite onto flume. The words, what of a simple kind stays fixed, and the clauses the statements gain are the recording agent's.
Sources: `agentplot/flywheel-cloud:design/domain-model.html` at 2ff6630, sections 2 ("Kinds become statecharts, run by Flume"), 3 ("Flume's kinds today: a fixed stage machine in code", and its table of the original's types against a simple kind), 12 (rulings T1, T2, T3 and the word "kind") and 13 (the updates ledger: items ty-v1, T2, T1.a, T1.b, T3.a and T1.c); `agentplot/flywheel-next:design/models/statechart/model.md` at f2ce8e3, sections 1.3 (nesting, regions and ownership; a type as a file of templates and atoms), 10.1 to 10.4 (the session, a stage's join, blocked and stalled as decisions) and 10.7 (the type registry: `name@version`, hashed, never edited, work in flight keeping its version), with its `machines/stage.yaml` (stage@4: `agents`, `join`, `on_fail`, `max_send_backs`, `deliverables`) and `machines/unit-types/persona-test@3.yaml`; `agentplot/blueprints:design/flume/requirements.md` at 92afa99, clauses 4, 18 to 25, 30, 140, 154 to 160, C.2, I2; `agentplot/blueprints:design/weir/requirements.md` at 92afa99, clauses 51 to 58, 81, 88 to 93, C.4; records 0004, 0019, 0022; rules core.4, core.8, core.10, core.34; `afterthought/crew` records 0047 (move 5: teams leave crew once flume runs a unit's whole state machine) and 0050 (crew moves onto flume one team at a time); `crates/flume-seams/src/kinds.rs` (`Kind`, its stages in order) and `held.rs` (`StageState`), moved by `crates/flume-core` (`start.rs`, `end.rs`, `answer.rs`, `submit.rs`, `settle.rs`).

## Context and problem statement

A kind today is an ordered list of stages (18). Each stage has one agent at a time and one deliverable, a failed submission sends the place back to the stage the kind names (23, 112), and a stage's life, from not reached to finished, is one enum flume's own code moves (`held.rs`, `flume-core`). A kind that asks for more is refused (4, 23).

The original Flywheel ran a type as a statechart: a hierarchical state machine defined as data, with regions, states, transitions in priority order under named guards, named effects each with its proof, and decisions as states, built from the stage, session and place templates and addressed as `name@version`, hashed and never edited (model §1.3, §10.7). A stage of it may run several sessions joined by `all`, `any` or a count, name several deliverables, and raise a decision a person answers. persona-test@3 runs one session per persona in one stage; self-closing@3 delivers five things and raises a stalled decision after two idle hours; standing@3 keeps running until its finish. None of the three is a list of stages.

On 2026-10-31 the user ruled that flume runs kinds as statecharts, after crew is rewritten (ruling T1), and the domain model's ledger carried it as simple kinds staying until then (item ty-v1). That leaves open whether the statechart format replaces the list at a release, or comes beside it for good.

## Considered options

1. Statechart kinds replace simple kinds: a release of flume that reads statecharts alone, every kind rewritten to them, and the list kept only until that release.
2. Statechart kinds as a second kind format beside simple kinds, on one branch: both formats read and run by every release from the first that runs statecharts, with no release split.
3. Simple kinds alone: the original's types written as lists of stages, and what a list cannot hold left out.

## Decision outcome

Option 2, as the user ruled.

1. **Two formats.** A kind is a **simple kind**, an ordered list of one or more stages as 18 and 23 say today, or a **statechart kind**, a hierarchical state machine of regions, states and transitions built from the stage, session and place templates and run by the engine record 0029 adopts. Both are weir's kind format, defined and read once by weir's crate (record 0004; core.4): the format grows to carry the statechart (weir C.4; ledger item T1.b), and how a declaration says which format it is in is the format's to say. Both are named by their identity, resolved, packaged and locked as every kind is (19, 154 to 160; record 0022). A statechart kind's version is never edited and a change to it is a new version, as the original's registry kept its types (model §10.7) and 156 keeps flume's own kinds.
2. **Simple kinds stay as they are.** Every simple kind, flume's own five (22), a repository's, a launch's and a package's, is declared, resolved, locked and run as today: clauses 18 to 25 and 140 hold for it unchanged, flume's own code runs its stages, and status reports its phase and stage states as now. No simple kind is refused, rewritten, deprecated or run otherwise because statechart kinds exist. A kind declared again in the other format is a new version of it, beside the earlier one (154, 156).
3. **One branch, no release split.** Both formats are built on flume's main. Every release from the first that runs statechart kinds reads and runs both, and none drops simple kinds; no branch, release line or build of flume holds one format apart from the other. A repository, a launch and a host mix them freely, and a place runs whichever format its kind is in (21).
4. **Statechart kinds are built after crew's rewrite onto flume.** Simple kinds carry the work while crew moves onto flume one team at a time (crew records 0047, 0050), and the build that runs statechart kinds (ledger item T1.c) starts once that rewrite is done. The statements' clauses and weir's format may come first. Until the build, flume runs simple kinds alone and refuses a kind that asks for more, naming it (4, 23; core.8).
5. **The words.** flume says a simple kind and a statechart kind, never v1 and v2: a kind's version is already its own number (154), so "a v2 kind" reads as change@2. "Type" stays the original's word (ruling "name").

Option 1 splits the releases. Every kind, flume's own five and every repository's and package's, is rewritten at once and every lock of record 0022 pinned again. A place launched under a simple kind before that release still finishes under it after (21), so the release could not drop the list anyway. And crew's rewrite onto flume would be built on a format about to go. A list is also the right size for most work: fix, chore, plan and both changes are lists; the original's with-operator fits a simple kind whole, and default@5 and chore@2 nearly do (the domain model's section 3); a statechart says the same in more lines. Option 3 never runs what a list cannot hold, persona-test@3's join over one session per persona, self-closing@3's deliverables and decision, standing@3, which ruling T1 says flume runs; and Flywheel's kinds, which are statecharts, would have no reader in the family's one format (record 0004).

### Consequences

- flume's statement gains, in changes the user rules (ledger items T1.a and T3.a; core.19):
  - in section 3, **simple kind** and **statechart kind**;
  - 18: a kind is a simple kind or a statechart kind, and a simple kind is an ordered list of one or more stages;
  - 23 and C.2: flume reads a simple kind's places and stages as today, and the refusal of a stage worked by several agents or with several deliverables is a simple kind's;
  - 30 and I2: a place has at most one agent at any moment under a simple kind, and a statechart kind's stage runs its sessions as its join says;
  - 4: a launch of a statechart kind is refused while flume runs none, and afterwards only for what neither format holds;
  - clauses after 174 (T1.a): a statechart kind is a file built from the stage, session and place templates, with named guards, named effects with their proofs, declared decisions and a label, addressed as `name@version`, hashed when it is registered and never edited; and both formats are read and run side by side, a simple kind as 18 to 25 and 140 say whatever statechart kinds do, neither translated into the other;
  - clauses after those (T3.a): a kind of either format is built into flume, declared in a repository's configuration, or a package from a plugin marketplace, as Claude Code's and herdr's plugins are, amending 19 and 155 to 159; flume checks and draws a kind when it loads it, and a repository's gate checks it too;
  - scenarios after S75: one repository declaring a simple kind and a statechart kind, each launched and each run as its format says; a place launched under a simple kind before the release that runs statecharts, finishing under it after (21).
- weir's statement gains (ledger item T1.b): 51 says a kind is a simple kind or a statechart kind; 52, 53 and 57 hold for a simple kind; clauses after 93 say what a statechart kind's declaration holds, its regions, the stage template's parameters, its decisions and its label, checked whole and refused naming the field (57); 81's read-kinds gives a statechart kind with its machine; C.4 spells it in TOML; and scenarios follow S18.
- Rule core.41 holds it, and core.4's details name the second format. Rules core.8, core.10 and core.34 stand as they are until the statement's clauses land.
- Nothing is built for this record. `crates/flume-seams/src/kinds.rs` and `held.rs` stay as they are, and an item in `docs/architecture/_open.md` stands until the build.
