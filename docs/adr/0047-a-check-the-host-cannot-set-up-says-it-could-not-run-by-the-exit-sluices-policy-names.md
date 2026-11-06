---
number: 47
title: A check the host cannot set up says it could not run, by the exit sluice's policy names, and never fails for it
status: proposed
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# A check the host cannot set up says it could not run, by the exit sluice's policy names, and never fails for it

Status: proposed on 2026-11-05 by the code stage of the change `flume-opens-a-streams-multiplexer`, started again on the words "failed on main: flume:tools". The direction is sluice's statement and the user's words in sluice's record 0074, quoted below. Using exit 77 and leaving the tests that fit unchanged are the stage's choices.

Deciders: the code stage, on sluice's clauses 19, 35 and 114 and its record 0074; the user, at the change's review.
Sources: `agentplot/blueprints:design/sluice/requirements.md` on main at cd8fd73, clauses 19, 35, 114, 115 and 116; sluice's records 0031 and 0074, and its `profiles/checks.rs` and `profiles/moon.rs` at v0.67.0; sluice's record of submission #108 and the results it keeps under flume:tools' key 9b568c1; this change's design and tasks; the user's rulings of 2026-11-03 and 2026-11-05 on the named-session tests, quoted in `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`'s design 2; rules core.21 and core.22; `crates/flume/tests/profiles/tools/main.rs`, `terminal_herdr.rs` (`SessionHome::new`) and `open.rs` (`Host::new`).

## Context and problem statement

sluice failed submission #108 on flume:tools. Its own run of the check passed, 93 of 93. One key held both that pass and an earlier fail, so sluice read it as flaky, and main's policy reads a flaky key as `fail` (19). The fail came from `sluice exec main` in this change's own place, which an older sluice had made. The place kept its scratch at the repository's `sluice/.tmp/<key>`, 73 bytes on this host. sluice gives every place made since 0.66.0 a short scratch (35), but one made before keeps its own until it is released. Under that scratch, a named herdr session's `herdr-client.sock` lies past macOS's 103 bytes. The sandbox refuses `/tmp`, so the ten tests that start a named session had no room for their socket. They failed naming their bytes, as this change's tasks asked.

That fail was the host's and not the code's. sluice keeps a host's fail out of a key in two ways only: the check says it could not run, by an exit the policy's `[tests] not_run` names (114), or the operator sets the run aside (116). No session sets a run aside. The user's words in sluice's record 0074: "A check brings up what it needs (containers, Postgres, a stack) and waits for it within its own limit. If it couldn't set up, it says so by the policy's not_run exit."

## Considered options

1. **The check ends the run with exit 77, which `[tests] not_run` names, naming what the host lacked.**
2. **The check fails naming its bytes, as before, and the operator sets each such run aside.**
3. **The check passes without its sessions where the host has no room.**

## Decision outcome

Option 1. Where no folder the run may write leaves a named session's socket room, `SessionHome::new` and the open's `Host::new` write why to stderr and end the test binary with exit 77. `.config/sluice.toml`'s `[tests] not_run` names 77, so sluice records no outcome under the check's key and runs it again (114). After as many runs in a row as the deployment states, it reads the candidate as not passed and says why (115). A test whose sockets fit runs as before. sluice reads `[tests]` at the branch's revision (56), so the exit counts from the landing that brings this record to main.

Option 2 leaves a host's fail under the key until the operator acts. Any old place's run of the checks then fails the next landing that shares the key, as #108 shows. Option 3 records a pass for a test that started nothing. The user ruled on 2026-11-03 that "a test run by hand is no verification". A test that does not run verifies even less.

Exiting from one test ends the whole binary. sluice ends a run's process group when the run ends, so a server another test started goes with it. No named session starts on a host with no room, since every test checks before it starts one.

### Consequences

- Good: the gate's keys hold the code's outcomes alone, and a place an older sluice made fails no later landing.
- Good: the bytes are still named, in the run's output and in sluice's reason for the run.
- Bad: a host that never has room keeps the check unjudged until 115 reads it not passed, which is slower than a fail.
- Bad: a run of `cargo test` by hand on such a host ends with 77 and no list of the tests that passed.
