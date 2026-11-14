# flume relays a place's terminal

## Why

A person may open a launch's terminal from its card in Boomdrive's
cloud, and the runner carries that person's screen and keys between
the browser and the place's live agent. As built, the runner runs
herdr's command line itself, which stands against 107: flume opens,
sends to and closes every agent's terminal, and no caller does. It also
ties the runner to one multiplexer. The user chose, in
boomdrivedev/boomdrive record 0004 (accepted on 2026-11-07 on their
card): "a combo of #1 and #3 might make the most sense. Remember that
Herdr should be a seam for multiplexers. We will want to support TuiOS
shortly." So flume is asked for a command that relays a place's
terminal through its own terminal seam, screen out and keys in, and
counts the relayed person as a person at the keyboard, never as a
caller's send.

The statement now says how: 107, 221 and 222, with C.5's herdr relay
row and the printed command's refusal, on boomdrivedev/blueprints main
since 41bd0a2, merged on the user's word of 2026-11-14. This unit was
queued on the bolt boomdrives-runner-reaches-a-terminal-through-flume
(afterthought/crew-state `plan.rec` at agentplot/main 20688ef1).

## What Changes

- **`flume relay <place>` (221).** A caller that carries a person's
  screen and keys runs it for as long as it holds the relay open. It
  writes what the terminal shows of the place's live agent to standard
  output, one JSON line each time that changes, and types each line of
  the person's input it reads on standard input, `{"text": …}` or
  `{"keys": [ … ]}`, as the person typed it. It ends when the caller
  closes its input, or once the agent's terminal goes, and its last line
  is its answer. A place with no live agent, and a terminal that offers
  no relay, are refused in their own words. The relay settles, writes
  and keeps nothing, and status names no relay.
- **The terminal seam's relay (222).** The seam gains relay: given an
  agent, a handle that reads what the terminal shows of it as a
  person's view would, within a bound the profile states, and types a
  person's input with nothing cleared first and no wait for receipt; or
  a refusal where the terminal offers none. The seam names a person's
  input, text or keys by flume's own key names, so no caller names a
  multiplexer's.
- **The core's relay.** One loop in the core gives the first screen at
  once and each changed screen at the profile's bound, types each input
  as it comes, reads the agent's state once a second and after a read
  that fails, and ends at the caller's close or once the agent is read
  stopped. It reaches the terminal the agent's start recorded (190).
- **herdr's relay (C.5's relay row).** `pane read <pane> --source
  visible --format ansi` every 250 milliseconds at most, `pane
  send-text` for text and `pane send-keys` for keys, the six keys
  herdr 0.9.5 names none of, `delete`, `insert`, `home`, `end`,
  `pageup` and `pagedown`, refused, named, and the keys after them
  typed, each call bounded
  at one second, the pane found again where it moved.
- **The printed command and Rex refuse relay.** The printed command
  holds no part of a terminal a person can be shown (C.5). The
  statement gives Rex no relay row, so flume's Rex profile offers none.
- **The fakes and the suite.** The fake terminal scripts a relay; the
  terminal seam's scenarios and new engine scenarios exercise it.
- **Record 0070 and rule core.57**, proposed, say how flume relays and
  what it keeps of a relay, which is nothing; core.11 says a relay's
  keys are no send.
- **The site** gives `flume relay` in its command reference, and its
  rules page mirrors core.11 and core.57.

## Capabilities

### New Capabilities

- `place-relay`: how a caller relays a place's live agent's terminal
  through flume, what goes out and in, when it ends, what refuses it,
  and that flume keeps and writes nothing of it.

### Modified Capabilities

- `command-line`: `flume relay` and the JSON lines it reads and writes.
- `seam-contracts`: the terminal seam's relay and the types of a
  person's input and a relay's read.
- `seam-fakes`: the fake terminal scripts a relay.
- `terminal-profiles`: herdr's relay, and the printed command's and
  Rex's refusals.

## Impact

- `crates/flume-seams/src/terminal.rs`: `Key`, `Typed`, `RelayScreen`,
  the `Relay` handle and `Terminal::relay`; `round_trip.rs`.
- `crates/flume-core/src/relay.rs` (new), `answer.rs`, `lib.rs`.
- `crates/flume-fakes/src/terminal.rs`.
- `crates/flume-conformance/src/seams/terminal.rs`,
  `src/fake_harness.rs`, `src/engine/relay.rs` (new),
  `src/engine/mod.rs`.
- `crates/flume/src/commands/relay.rs` (new), `commands/mod.rs`,
  `commands/print.rs`; `crates/flume/tests/command_line.rs`.
- `crates/flume/src/profiles/terminal/herdr/mod.rs`, `printed/mod.rs`,
  `rex/mod.rs`; `crates/flume/tests/profiles/terminal_printed.rs`,
  `tests/profiles/tools/terminal_herdr.rs`, `terminal_rex.rs`,
  `walkthroughs.rs`.
- `docs/adr/0070-…` (new, proposed), `docs/architecture/core.md`,
  `site/src/content/docs/architecture/rules.md`,
  `site/src/content/docs/reference/commands.md`.
- Boomdrive's runner can drop its own herdr relay for `flume relay`;
  that change is the runner's own (boomdrive record 0004).
