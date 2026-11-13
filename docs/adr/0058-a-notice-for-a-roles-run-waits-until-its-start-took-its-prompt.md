---
number: 58
title: A notice for a role's run waits until the run's start took its prompt
status: accepted
date: 2026-11-07
decision-makers:
- the code stage of the fix flume-sessions-start-their-mcp-servers--s93-holds-steady
---

# A notice for a role's run waits until the run's start took its prompt

Status: accepted on 2026-11-08 by agentplot-design. Written by the code stage of the fix `flume-sessions-start-their-mcp-servers--s93-holds-steady`: its direction is the brief's and the statement's, and its choices are the stage's own, cheap to change and none of them the user's alone, so the design agent settles them.

Deciders: the code stage, on clauses 37, 38, 43 and 132 and rule core.11.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` on main at 00d90e9, clauses 33, 37, 38, 43 and 132; sluice's refusal of #159 at 2a2101a and its logs `flume:tools.log` of runs c139 and c140 on deneb, 2026-11-07; the role-waking spec's requirements "An end pending for an address with no run wakes the role" and, from this record's change, "A role's run takes no notice before its prompt"; `crates/flume-core/src/settle.rs`, `ends.rs`, `standing.rs`, `deliver.rs` and `start.rs`; S93's walkthrough, `crates/flume/tests/profiles/tools/walkthroughs.rs`.

## Context and problem statement

sluice refused #159 because S93's walkthrough failed once on inputs that passed ten minutes later, and flume's policy reads a flaky check as failed. The run of `east/ops` read lost: `ops--east-ops · ops@1 · answer · run 1 · lost`, while construct had taken the run's answer.

A role's run is woken inside a settle. Its start opens the agent, then sends the prompt only where the agent has said it started and the terminal reads it waiting (37). On a loaded host one of those readings can miss, an answer of herdr's late past its bound among them, and the prompt is left to the settles that follow. A settle settles every place in turn, and the place whose end is pending comes before the run, so its delivery reaches the address's session first. The run's agent waits there for its prompt, and the notice is typed into it. The agent takes the notice as its first prompt. The stand-in's script runs one turn, so it did the run's work on the notice, committed and stopped. Its stage's prompt was never received (38), and a start whose agent stops before its prompt is received reads lost. With S93's run's launch kept from sending the prompt, the walkthrough fails so on every run.

Claude Code would not stop there. It would work its turn on the notice, with its standing instructions but without the unit's brief and facts its prompt holds (33). The stage's prompt, sent once more after 60 seconds of a turn that is still working, is not received, and a long turn reads the start never begun.

## Considered options

1. **A role's run is reached for a notice only once its current start took its prompt.** While the start is starting, or started with its prompt not received, the end stays pending.
2. **The settle settles a role's run before the places whose ends are pending for it.**
3. **S93's stand-in takes two turns, so the run's prompt is received after the notice's turn.**

## Decision outcome

Option 1. The delivery's reach of a recipient that is a role's address and no standing agent passes over the address's run while that run's current start is live and starting or started (`standing.rs`, `awaits_its_prompt`). The end stays pending, as it stays while the recipient asks a question (43), and the first settle that reads the start begun sends the notice, unless the run's `flume inbox` collected the end first. A standing agent of the address is reached as before, since flume starts no prompt into it. The read is the host's held places, made only for an address with an end pending.

Option 2 narrows the window and leaves it open. A run's start can still miss its prompt within the settle that settles it, and a later settle then delivers before the prompt again. A carrier's wake or another command's settle can come between too. Option 3 hides the race in the test. The run's first turn still runs on the notice without its prompt, which 33, 37 and 38 do not allow, and Claude Code's case reads never begun.

### Consequences

- Good: a role's run takes its stage's prompt first, as every other start does, and S93 no longer turns on which reading misses.
- Good: S94's run of `east/operator`, woken by a stall, is held the same way.
- Bad: an end for a role's run whose start never takes its prompt stays pending until the start ends, is started again or the run's inbox collects it. That is the run's own failure, which its line and its stall already report.
- Neutral: the statement is unchanged. Clause 43 delivers a role's end into its live run's pane, and 37 and 38 decide that the run's prompt comes first; rule core.11's details name the case.
