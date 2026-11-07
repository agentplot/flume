---
number: 46
title: An answer reaches the asking agent's open session within an hour, and past it flume ends the agent and the answer is a retry
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# An answer reaches the asking agent's open session within an hour, and past it flume ends the agent and the answer is a retry

Status: accepted by the user on 2026-11-01, on their Pending You card (req_515a9d7e2f794a76b85d, "Keep it open an hour, then end it"), which followed their cards req_e4f68873 and req_d757bae5. It was written that day as record 0028 on the branch of the change `an-answer-reaches-the-open-session-within-an-hour`; main had given 0028 to statechart kinds on 2026-10-31, so this record is 0046, the first number free on every branch and in every place on this host on 2026-11-05. Its statement amendment is on agentplot/blueprints, branch `design/flume-an-answer-reaches-the-open-session` at 6bdd075, cut from main at cd8fd73, for the user's word (198, S104, S105).

Deciders: the user, on those three cards, for the decision. An agent that stops short asking a question stays open for up to an hour, a setting, one hour by default, the life of the prompt cache. An answer that comes while the agent is live and within the hour goes into its open session as a tell. flume records it as the stage's answer, the question and the answer kept with the stage as a retry's words are, and the stage carries on in that session: no fresh agent, the stage not restarted. When the hour passes with no answer, flume ends the agent itself, freeing its room and its account. The stage keeps waiting on its answer, and a later answer is a retry with a fresh agent whose prompt carries the question and the answer, as before. An agent that dies within the hour, by a restart or a spent account, is the same as past the hour. Status says which. The change's design stage, for what the user's cards left to it: the setting's name and layers, which of a caller's acts carry the answer, what flume types into the session, where the answer is recorded, and how the session's close is read.

Sources: `agentplot/blueprints:design/flume/requirements.md` at cd8fd73, section 3's settings, prompt cache, live, quiet, stop short, end, retry and tell, clauses 4, 31, 32, 41 to 44, 48 to 50, 53 to 56, 70, 95, 96, 118, 120, 122, 130, 131, 134, 140, 145, 146, 158, 161, 163, 179, 190, 195, 197, C.2, C.4 (the send-back row), C.5 (the printed command sends nothing to a running agent), C.7 and S4; records 0003, 0011, 0016, 0017, 0020, 0035, 0043 and 0044; rules core.10, core.11, core.13, core.25, core.26, core.27, core.30, core.32; on flume's main at c9d58df, `crates/flume-core/src/agent_acts.rs` (`needs` ends the stage short and leaves its agent live), `settle.rs` (a stage ended short needing an answer takes no step), `acts.rs` (`retry` ends the place's agent first and starts the stage afresh, its words `answered` with the question before the answer; `tell` delivers to any live agent that waits and is not asking, whatever its stage's state), `start.rs` (a fresh start after a spent account or an unsettled start has flume's words alone); the design of the change `every-question-a-place-asks-reaches-someone-who-answers-it` (record 0039), which keys each end and records a keyed retry's answer on the stop-short it answers.

## Context and problem statement

Under records 0003 and 0011 a stage whose agent stops short needing an answer ends short, and its answer is a retry. flume ends the agent and starts a fresh one, whose prompt carries the question and the answer (42, 49, 50). Between the stop-short and the retry, flume leaves the asking agent live. It holds a place in the host's room, its stream's and its account's for as long as the answer takes (70, 145), and the retry ends it anyway. The fresh agent then reads the place again from nothing. What the asking agent had read, tried and decided is gone, and so is its prompt cache. Most answers come within minutes, while that agent waits with all of it in hand.

## Considered options

1. Always a fresh agent, the asking agent ended at once: flume ends the agent once it is quiet after its stop-short, and the answer is a retry, as before but with the room freed.
2. Open until answered: the agent stays live until its answer comes, however long, and the answer goes into its session.
3. Open an hour, then ended: the agent stays live, its session open for its answer, for a time its settings give, one hour by default, the life of its program's prompt cache. An answer in that time goes into the session. Past it, flume ends the agent once it is quiet, and a later answer is a retry with a fresh agent.

## Decision outcome

Option 3, the user's "Keep it open an hour, then end it".

**How long an agent is kept open.** `keep_open` is one of the settings clause 32 resolves, as an effort is (197): taken whole from the first layer that gives it, a stage's own before every stage's within a layer, read at the agent's start and kept by it, the host's configuration as it stands then (32; records 0017, 0043, 0044). It is written `<n>`, `<n>s`, `<n>m` or `<n>h`, the forms a launch's stall bound takes, and `0` keeps no agent open. It differs from an effort in two ways. It is flume's own time, the same for every program and passed to none, so it is one value and never a table of programs. And only the launch's settings and the host's four layers give it, with flume's own hour beneath them: what an agent kept open holds is the host's room, its stream's and its account's (70, 145), and how long a prompt cache lives is its account's (section 3), none of which a repository's file or a kind that travels between hosts can know (158). So a repository's settings and a kind's declaration carry no `keep_open`, as they carry no environment, and one that writes it refuses the launch, naming the field (4, C.2).

**Where an agent is kept open.** An agent that stops short needing an answer, or found work that 131 takes as needing one, from a stage other than its kind's last or from a last stage that is its kind's own fix round (record 0011's amendment of 2026-11-06), is kept open from its stop-short for its `keep_open`, where the terminal its place runs in hands a running agent words, as a tell needs (53, 95, 190). Under the printed command, which types into no running agent (C.5), no agent is kept open, and every answer is a retry, as before.

**The answer reaches the open session.** The answer is what it was: a retry naming the stage with the answer as its words (42, 49). A tell to the place is an answer too, since the stage waits on nothing else. While the agent is kept open, flume hands it the question and the answer, as a retry's prompt carries them, the way a tell is handed: once the agent waits, received or sent once more, never into a question, within the tell's bound (53, 54). It records them on the stop-short they answer, with the act that gave them and when they were sent and received, where the questions change records a keyed retry's answer, so an answer is taken once whichever way it goes. The stage carries on in that session under the same start, begun again. No agent starts and none is ended, and no start is counted (48, 49, 50, 55). An answer the agent does not take is refused with nothing typed, and the stage still waits on it. A retry with no words is no answer: it ends the agent once quiet and starts a fresh one with the question, as before. A caller that wants a fresh agent to take the answer within the hour stops the agent first (51).

**The agent stops being kept open.** It stops once its time has passed since the stop-short; once the agent is read stopped or its own record says its session ended (96, 134), as a restart of the host or its terminal leaves it; or once its account is spent, by its own record or the host's documents (146). A settle reads it so and records when and why; no process of flume's waits for the time (120; core.13), and under a terminal that keeps watch the watch settles the place within its bound (122). Once the agent is quiet, flume ends it, never forced (31), freeing its room and its account (70, 145). The stage keeps waiting on its answer, under the same move and since, so a place that stalled does not stall again (130). A later answer is a retry, whose fresh agent's prompt carries the question with the answer (42, 49), and a tell is refused, naming that retry (53). The fresh agent carries no conversation of the one ended: clause 179 carries a conversation only to a start that 146 makes, and the user ruled a fresh agent here.

**What carries on.** A stage carried on after its answer is a running stage. The stop check sends its agent back as it would any (110). Its quiet limit and its working past the limit run from the answer's receipt as from a beginning (44, 118). Its next stop-short is an end of its own, keyed apart from the one answered, recorded and delivered as the first was (43), and its agent is kept open again from it. A wait on the stage answers an end read after the answer, never the stop-short it answered. A fresh start flume makes on its own of a stage, after a spent account its program cannot carry a conversation from or after a start begun without its settings, carries after flume's own words the words the earlier start was given and each answer it took, so no answer is lost with its agent (146, 161).

**Status.** For a stage ended short needing an answer, status says whether its agent is kept open and until when, or when and why it stopped being so and whether it is ended. It gives each answer the stage's current start took in its session, with its question, and `keep_open` with the agent's other settings and its layer (56, 130). The stop-short's notice says until when its agent is kept open.

**Why not the others.** Option 1 frees the room at once, but throws away every asking agent's context and cache, even when the answer comes a minute later. Option 2 holds room and an account's allowance for as long as a question waits, a weekend's for one asked on a Friday. Past the cache's life the session's next request reads its whole context in afresh anyway, so keeping it buys little that a fresh agent with the question and the answer would not.

### Consequences

- Records 0003 and 0011 carry an amendment line naming this record.
- Rule core.27 is amended, and core.27's and core.30's details with it.
- The statement gains clause 198, the agent kept open, and scenarios S104 and S105, and section 3, clauses 4, 32, 42, 49, 50, 53 to 56, 95, 118, 130, 146, 158, 161, C.2, C.4's send-back row, C.5, C.7 and S4 are amended, as design 13 of the OpenSpec change `an-answer-reaches-the-open-session-within-an-hour` words them and the blueprints branch above holds them (core.19).
- That OpenSpec change builds it once the change `an-agents-effort-is-a-setting-of-flumes-layers` has landed, on the layers and tables that change and `an-agents-program-settings-come-from-flumes-layers` build.
- The change `every-question-a-place-asks-reaches-someone-who-answers-it` gives each end a key and takes a retry's answer once by it; a stop-short after an answer in its session is an end of its own under its own key, and a tell that answers takes that key too.
- A stage for review (163; record 0024) keeps its own tell and retry, apart from this.

## Amendment, 2026-11-06: a conversation's agent lost with its host while kept open is carried at its answer's retry

Status: proposed by the code stage of the change `an-answer-reaches-the-open-session-within-an-hour` on 2026-11-06, the choice its own, for the user's word. Its statement amendment, of 49, 179 and 198, is on agentplot/blueprints, branch `design/flume-an-asking-conversation-lost-is-carried` at 6529c04, cut from main at a4eeec3, committed and not pushed.

Deciders: the words the code stage was started again with: "Reconcile with #130's lost-conversation retry and the settles change, since an agent kept open for its answer meets both". Record 0052 leaves "a conversation let go after its stop-short's hour (198)" to this record (its option 3).
Sources: `agentplot/blueprints:design/flume/requirements.md` on main at a4eeec3, clauses 45, 49, 77, 109, 179, 198 and 207, and S113 to S117; record 0052; rules core.10 and core.27; `crates/flume-core/src/acts.rs` (`Engine::retry`), `crates/flume-core/src/end.rs` (`read_end`, `end_foreign`) and `crates/flume-core/src/start.rs` (`StartReason::Resumed`, `StartReason::Lost`).

### Context and problem statement

A conversation's agent (109) may stop short needing an answer and be kept open. Within its time its terminal or its host may lose it: its pane gone, or brought back bare by the terminal's restore with no word that it started under its start's identity (77). Its stage said something, so it is not reported lost (45), and 49 carries a conversation only for a stage reported lost. The record above has a later answer start a fresh agent. That agent would read nothing of the conversation, and the retry would still have to wait for a bare agent a person may be talking in (77, 207).

### Considered options

1. **Carried as a conversation reported lost is,** at its answer's retry, the question and the answer the retry's words.
2. **A fresh agent with the question and the answer,** as for any other stage.
3. **Every conversation's agent let go carried,** by its time and by its caller's stop too.

### Decision outcome

Option 1. A conversation's agent kept open and let go because it was read stopped, its own record said its session ended, or the agent in its pane is not its start's is let go as its agent ended. One its terminal brought back bare is left in its pane, sent nothing and ended by no settle (77; core.10). The answer's retry of that stage goes on with the lost agent's conversation as 49 says for a conversation reported lost. It ends no agent, waits while a process runs the conversation (207), takes the account the lost start ran under first, and resumes the session with flume's words and then the question and the answer. Where it cannot carry, the stage starts afresh with flume's words before them. A conversation's agent let go by its time, its account spent or its caller's stop carries nothing, and its answer starts a fresh agent as the record above says. A caller's stop is recorded as its own reason, so the two are told apart.

Option 2 drops what the person said to the bare agent and all the conversation held, which record 0052 brings back whole. Option 3 overrides a caller who stopped a conversation on purpose, and the user ruled a fresh agent past the time.

### Consequences

- The statement's 49, 179 and 198 are amended on the branch above (core.19).
- core.27's details gain the case.
- The OpenSpec change `an-answer-reaches-the-open-session-within-an-hour` builds it, its design 14 and the stage-runs spec's "A conversation's asking agent lost is carried at its answer".
