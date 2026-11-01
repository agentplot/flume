---
number: 28
title: An answer reaches the asking agent's open session within an hour, and past it flume ends the agent and the answer is a retry
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# An answer reaches the asking agent's open session within an hour, and past it flume ends the agent and the answer is a retry

Status: accepted by the user on 2026-11-01, on their Pending You card (req_515a9d7e2f794a76b85d, "Keep it open an hour, then end it"), which followed their cards req_e4f68873 and req_d757bae5.

Deciders: the user, on those three cards, 2026-11-01. An agent that stops short asking a question stays open for up to an hour, a setting, one hour by default, the life of the prompt cache. An answer that comes while the agent is live and within the hour goes into its open session as a tell. flume records it as the stage's answer, the question and the answer kept with the stage as a retry's words are, and the stage carries on in that session: no fresh agent, the stage not restarted. When the hour passes with no answer, flume ends the agent itself, freeing its room and its account. The stage keeps waiting on its answer, and a later answer is a retry with a fresh agent whose prompt carries the question and the answer, as before. An agent that dies within the hour, by a restart or a spent account, is the same as past the hour. Status says which. The setting's name and layers, which of a caller's commands carry the answer, what flume types into the session, and how the session's close is read are the writing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 92afa99, section 3's settings, prompt cache, live, quiet, end, retry and tell, clauses 31, 32, 41, 42, 44, 49, 50, 53 to 56, 70, 96, 118, 120, 122, 130, 134, 146, C.2, C.4 (the send-back row), C.5 (the printed command sends nothing to a running agent), C.7 and S4; records 0003, 0011, 0016, 0017, 0020; rules core.10, core.11, core.13, core.26, core.27, core.30, core.32; `crates/flume-core/src/agent_acts.rs` (`needs` ends the stage short and leaves its agent live), `crates/flume-core/src/settle.rs` (a stage ended short needing an answer takes no step), `crates/flume-core/src/acts.rs` (`retry` ends the place's agent first and starts the stage afresh, its words `answered` with the question before the answer; `tell` delivers to any live agent that waits and is not asking, whatever its stage's state).

## Context and problem statement

Under records 0003 and 0011 a stage whose agent stops short needing an answer ends short, and its answer is a retry. flume ends the agent and starts a fresh one, whose prompt carries the question and the answer (42, 49, 50). Between the stop-short and the retry, flume leaves the asking agent live. It holds a place in the host's room, its stream's and its account's for as long as the answer takes (70, 145), and the retry ends it anyway. The fresh agent then reads the place again from nothing. What the asking agent had read, tried and decided is gone, and so is its prompt cache. Most answers come within minutes, while that agent waits with all of it in hand.

## Considered options

1. Always a fresh agent, the asking agent ended at once: flume ends the agent once it is quiet after its stop-short, and the answer is a retry, as today but with the room freed.
2. Open until answered: the agent stays live until its answer comes, however long, and the answer goes into its session.
3. Open an hour, then ended: the agent stays live, its session open for its answer, for a time its settings give, one hour by default, the life of its program's prompt cache. An answer in that time goes into the session. Past it, flume ends the agent once it is quiet, and a later answer is a retry with a fresh agent.

## Decision outcome

Option 3.

**How long a session stays open.** `keep_open` is a setting of the agent's, given as `<n>`, `<n>s`, `<n>m` or `<n>h`, the forms a launch's stall bound takes. It is resolved at the agent's start as record 0017 orders the layers, from three of them alone: the launch's settings, then the host's `[repos."<repository>".kinds.<kind>.agents]`, then its `[kinds.<kind>.agents]`, a stage's own first in each, else flume's own one hour. The prompt cache's life is the host's and its accounts' to know, so a repository's file and a kind's declaration give none: a `keep_open` there refuses the launch, as an `env` does. `0` gives option 1 to a host that wants it. A session is open only where the host's terminal can hand its agent a tell (53). Under the printed command, which types into no running agent (C.5), no session is open, and every answer is a retry, as today.

**The answer reaches the open session.** The answer is what it was: a retry naming the stage with the answer as its words (42, 49). A tell to the place is an answer too, since the stage waits on nothing else. While the session is open, flume hands the agent the question and the answer, as a retry's prompt carries them, the way a tell is handed: once the agent waits, received or sent once more, never into a question, within the tell's bound (53, 54). It records both with the stage's start, with the act that gave them and when they were sent and received, and the stage carries on in that session under the same start, begun again. No agent starts and none is ended, and no start is counted (48, 49, 50, 55). An answer the agent does not take is refused with nothing typed, and the stage still waits on it. A retry with no words is no answer: it ends the agent once quiet and starts a fresh one with the question, as today. A caller that wants a fresh agent to take the answer within the hour stops the agent first (51).

**The session closes.** It closes once its time has passed since the stop-short, once the agent is read stopped or its own record says its session ended (96, 134), as a restart of the host or its terminal leaves it, or once its account is spent, by its own record or the host's documents (146). A settle reads it closed and records when and why; no process of flume's waits for the time (120; core.13), and under a terminal that keeps watch the watch settles the place within its bound (122). Once the agent is quiet, flume ends it, never forced (31), freeing its room and its account (70, 145). The stage keeps waiting on its answer, under the same move and since, so a place that stalled does not stall again. A later answer is a retry, whose fresh agent's prompt carries the question with the answer (42, 49), and a tell is refused, naming that retry (53).

**What carries on.** A stage carried on after its answer is a running stage. The stop check sends its agent back as it would any (110). Its quiet limit and its working past the limit run from the answer's receipt as from a beginning (44, 118). Its next stop-short is an end of its own, recorded and delivered as the first was (43). A wait on the stage answers an end read after the answer, never the stop-short it answered. A fresh start flume makes on its own of a stage answered in its session, after a spent account or a start begun without its settings, carries after flume's own words the words the earlier start was given and each answer it took, so no answer is lost with its agent (146, 161).

**Status.** For a stage ended short needing an answer, status says whether the answer reaches its agent's open session and until when, or when and why the session closed and whether its agent is ended. It gives each answer the stage's current start took in its session, with its question (56, 130). The end's notice says until when its session is open.

**Why not the others.** Option 1 frees the room at once, but throws away every asking agent's context and cache, even when the answer comes a minute later. Option 2 holds room and an account's allowance for as long as a question waits, a weekend's for one asked on a Friday. Past the cache's life the session's next request reads its whole context in afresh anyway, so keeping it buys little that a fresh agent with the question and the answer would not.

### Consequences

- Records 0003 and 0011 carry an amendment line naming this record.
- Rule core.27 is amended, and core.27's and core.30's details with it.
- The statement's section 3 settings, clauses 32, 41, 42, 49, 50, 53 to 56, 118 and 130, C.2, C.4's send-back row, C.5, C.7 and S4 are to be amended, and scenarios added for the session's close and an agent dying within it, as design 12 of the OpenSpec change `an-answer-reaches-the-open-session-within-an-hour` words them (core.19). Until they are, `docs/architecture/_open.md` names the wait.
- The OpenSpec change `an-answer-reaches-the-open-session-within-an-hour` builds it: the setting, the terminal seam's word on whether it hands a running agent words, the session kept open and closed, the answer handed to it, status, and walkthroughs under the pinned herdr with the stand-in for Claude Code.
- A stage for review (163; record 0024) keeps its own tell and retry, and is built apart.
