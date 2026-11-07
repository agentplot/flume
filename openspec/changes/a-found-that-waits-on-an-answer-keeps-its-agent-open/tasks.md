# Tasks

The build is the merge commit that took main in on 2026-11-07; this
change carries its spec. The statement is read on agentplot/blueprints
main at b0df70b (42, 131, 198).

## 1. The spec

- [x] 1.1 Modify running-through's "Found work no stage before it can do again needs an answer", its scenario of a fix's code finding work saying code's agent is kept open for the answer, and "The retry that answers a found", the retry handed to the agent kept open in its session and a fresh agent's prompt carrying the findings with the answer only otherwise (42, 48, 49, 131, 198). True when `openspec validate a-found-that-waits-on-an-answer-keeps-its-agent-open --strict` passes.
- [x] 1.2 Hold each scenario in `crates/flume-conformance/src/engine/running_through.rs`: "A fix's code finds work" as `a_fixs_code_finds_work`, checking the move's `open_until`; "The retry that answers a fix's found in its session" as `the_retry_that_answers_a_fixs_found_in_its_session`; and "The retry that answers a fix's found" as `the_retry_that_answers_a_fixs_found`, the clock past `KEEP_OPEN` before the retry; and the agent kept open in `engine/open_sessions.rs` as `found_work_at_the_kinds_own_fix_round` (106). True when `sluice exec main -- moon run conformance:test conformance:coverage` passes.
- [ ] 1.3 Run the gate. True when `sluice exec main` passes and `openspec archive a-found-that-waits-on-an-answer-keeps-its-agent-open --yes` leaves `openspec validate --specs --strict` passing.
