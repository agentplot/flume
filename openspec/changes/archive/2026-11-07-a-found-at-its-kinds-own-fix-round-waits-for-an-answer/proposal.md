# A found at its kind's own fix round waits for an answer

## Why

crew's fix place `fix--a-landing-in-another-hosts-queue-reads-as-in-the-queue`
ran on flume main at 376c17ca. On each of runs 1 to 3 its code agent
found a blocker outside the place and stopped short with `flume needs
--found`. Each time flume sent the place back to code, the stage that
found, with a fresh agent and the findings as its words, and counted a
going-back. The fresh agent met the same blocker. After three, the place
waited past its bound, and its caller heard of it as a going-back too
many rather than as a question.

flume sends a found from the kind's last stage to the stage the last
stage names for a failed submission, or to that last stage where it
names none (`sent_back_by` and `fix_round` in
`crates/flume-core/src/submit.rs`). fix@1 and chore@1 are one code
stage, their last stage and their fix round both (C.2), so a found from
fix's code goes back to fix's code. No stage before it exists to do
that work again. And Claude Code's way to stop short offers `--found`
for "work this stage cannot deliver past", which a blocker outside the
place is, while clause 41 says a found is work a stage before the
agent's own must do again.

The statement now says so: on agentplot/blueprints main at 734d60c, a
found from a last stage that is its kind's fix round is taken as
needing an answer, since no stage before it can do that work again
(131), and flume starts a stage again on its own on a found as on a
failed submission (48). Record 0011's amendment of 2026-11-06, the
design agent's, decides it, and rule core.27 takes it.

## What Changes

- **A found goes back only to a stage before it (131).** A found from
  the kind's last stage starts the fix round only where the fix round is
  a stage before the last, as change's code is before verify.
- **A found at its kind's own fix round waits for an answer (42, 131).**
  Where the last stage is the fix round, as fix@1's and chore@1's code,
  a repository's kind whose last stage names itself, or one whose last
  stage names none, the found is taken as needing an answer, as a found
  from any other stage is today. No stage starts and no going-back is
  counted. The place waits on a retry of that stage. Status, the line
  and the list say it waits on an answer, and status and the end say
  the agent found work, with its words. The retry's prompt carries the
  findings with the answer (49), and a retry counts no going-back (48).
- **One reading of a kind's fix round.** The stage a send-back starts,
  the move status gives a place past its bound, and whether a found
  sends the place back are all read from one place in the core.
- **Every agent is told what a found is (28, 41).** Claude Code's way to
  stop short, in every prompt and in the stop check's words, says
  `flume needs --found` is for work a stage before this one must do
  again. `flume needs --found`'s help says the same. flume's own kinds'
  prompts are unchanged (156).
- **Docs.** The kinds, callers, merge-queue, prompt-parts, commands and
  configuration pages say when a found sends the place back and when it
  waits on an answer, and what a found is.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `running-through`: found work from the kind's last stage starts the
  fix round only where the fix round is a stage before it. Found work
  that no stage before it can do again, from any stage but such a last
  one, is taken as needing an answer.
- `stage-runs`: a stop-short starts the kind's fix round only where the
  found is from the kind's last stage and the fix round is a stage
  before it. Any other found is taken as needing an answer.
- `agent-profile`: Claude Code's way to stop short tells the agent that
  a found is work a stage before its own must do again.
- `command-line`: `flume needs --found`'s help says what a found is.

## Impact

- `crates/flume-core/src/submit.rs` (`fix_round`, `sent_back_by` and a
  reading of the kind's fix round they share) and
  `crates/flume-core/src/status.rs` (`waiting_on`).
- `crates/flume/src/profiles/agent/claude/mod.rs` (`stop_short`),
  `crates/flume/src/commands/mod.rs` (`--found`'s help),
  `crates/flume-seams/src/agent.rs` (`StopKind`'s documentation) and
  `crates/flume-fakes/src/agent.rs` (the fake's way to stop short).
- Conformance scenarios in
  `crates/flume-conformance/src/engine/running_through.rs`. A
  walkthrough in `crates/flume/tests/profiles/tools/walkthroughs.rs`,
  whose `STOP_SHORT` takes the new words. A test of Claude Code's words
  in `crates/flume/tests/profiles/agent_claude.rs`.
- Record 0011's amendment and rule core.27, written with this change,
  and the site's mirror of core.27.
- The site's guides and reference.
- A repository's kind whose last stage names no `on_fail`, or names
  itself, now waits on an answer at a found from that stage instead of
  starting it again. A place already launched is read so at its next
  settle. A place already waiting past its bound keeps waiting with
  its words.
- No clause is added or removed, so the conformance suite's clause list
  stays as it is (106).
