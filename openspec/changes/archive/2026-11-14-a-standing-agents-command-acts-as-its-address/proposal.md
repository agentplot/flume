# A standing agent's command acts as its address

## Why

A standing agent is handed its address's ends as notices (132), and then
cannot confirm them. On 2026-11-14 boomdrive-ops ran `flume confirm` in
its standing pane with no recipient named. flume read the command as the
terminal's session, `herdr:w2:p3@…/boomdrive/herdr.sock`, which is no
reader of the landing's end owed to `boomdrive/ops`, so the end stayed
pending (signal `2026-11-14-boomdrive-ops-bca216e0/01` on
afterthought/crew-state's `boomdrive/main`). `recipient_of` takes a
command's recipient from `--recipient`, then `$FLUME_RECIPIENT`, then the
address of the role's run `$FLUME_PLACE` names, and else the terminal's
session; a standing agent is none of the first three. A place the agent
launches takes its pane as recipient, which a revive changes.

Record 0013's amendment of 2026-11-14 decides it, and rule core.28 takes
it. Clause 132 says it on boomdrivedev/blueprints' branch
`design/a-standing-agent-acts-as-its-address` at d008a1d, until the user
pushes it (core.19).

## What Changes

- **A standing agent's command acts as its address (7, 132).** Where a
  command names no recipient, `$FLUME_RECIPIENT` names none and it runs
  in no role's run, flume reads which standing agent the command runs
  in: the one agent an address's delivery would reach, found in its
  stream's terminal and session as a delivery finds it, whose pane is
  the one that terminal names as the command's session. `flume
  confirm`, `flume inbox` and `flume wait` then act as that address, and
  a retry's or a tell's answer is recorded as given by it. Anywhere
  else the command acts as the terminal's session, as now.
- **A launch from a standing agent's pane takes its address (7).** A
  launch naming no recipient, with `$FLUME_RECIPIENT` unset, takes the
  address the standing agent it runs in holds, before the terminal's
  session; status gives it as from that standing agent, with the source
  that declares it.
- **One address, or the command names which (132).** A pane whose
  agent its sources declare standing for more than one address acts as
  none: `flume confirm`, `flume inbox`, `flume wait` and a launch are
  refused, naming the agent and each address, and asking for
  `--recipient`. A launch so refused makes nothing (core.8).
- **What the command names comes first.** `--recipient`,
  `$FLUME_RECIPIENT` and a role's run keep their place ahead of this
  reading (core.28).
- **Nothing is read where nothing could match.** The terminal is asked
  for its agents only where the sources declare a standing agent and a
  terminal names the session the command runs in, each part once per
  command, and nothing is kept (core.15).
- **The terminals' caller panes.** Under herdr and Rex, the session a
  command names for its pane is the session name the profile gives the
  agent in that pane, checked against the pinned releases (C.5;
  core.29).
- **Unchanged.** The notice still names `flume confirm --recipient
  <address>`; SessionStart in a session flume did not start still
  collects as the session, never as an address (C.4); a launch from a
  role's run takes what it takes now.
- **Record 0013** gains the construct's reading of 2026-11-14, and
  **rule core.28's** details what it leaves open.
- **The site** says what a standing agent's own commands act as, and
  mirrors core.28 as amended.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `role-waking`: a standing agent's command acts as the address it
  holds, and an agent standing for more than one address names which.
- `launching`: a launch from a standing agent's pane takes its address,
  from that standing agent.
- `place-status`: a recipient a standing agent's launch gave, with the
  agent and its source.
- `seam-contracts`: the type of a recipient taken from a standing agent.
- `terminal-profiles`: the caller's pane named as its agent is given,
  under herdr and Rex.

## Impact

- `crates/flume-seams`: `RecipientsFrom::Standing { agent, source }`
  and its round trip.
- `crates/flume-core`: `standing.rs` reads the standing agent a command
  runs in; `Engine::recipient` (`wait.rs`) answers the address it holds
  or the refusal; `launch.rs`'s `recipients_of` takes it before the
  session.
- `crates/flume`: `commands/mod.rs`'s `recipient_of` and the
  `--recipient` help of `wait`, `inbox`, `confirm` and `launch`;
  `print.rs`'s words for a recipient from a standing agent.
- `crates/flume-conformance`: role-waking's and launching's scenarios.
- `crates/flume/tests/profiles/tools/`: `terminal_herdr.rs`,
  `terminal_rex.rs` and a walkthrough under herdr.
- `site/`: `guides/roles-and-wake.md`, `reference/commands.md`,
  `reference/state.md`, `architecture/rules.md`.
- `docs/adr/0013-…`, `docs/architecture/core.md`'s core.28 details.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency.
