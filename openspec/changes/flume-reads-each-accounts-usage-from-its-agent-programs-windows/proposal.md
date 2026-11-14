# flume reads each account's usage from its agent program's windows

## Why

A pool's `most-left` strategy and its threshold read each account's
reading: how much of each usage window is used and when it resets (145,
184, 185). Today the reading is a file the host writes, named by the
account's `usage`, and on deneb a program of the fleet's writes it by
reading each account's sign-in and asking the maker's usage endpoint.
flume's account form names a file of one host's making, and a host
without that program has no reading. The user ruled on 2026-11-09:
"We should not be passing Flume some custom SwanCloud JSON. Flume is
supposed to be figuring out Claude Code usage on its own. This coupling
cannot exist" (record 0062, accepted; principles 14).

Claude Code hands its own sessions' plugins the windows each response
reports, in the engine's `session.measure` event, and flume's plugin
already runs a module in every session flume starts (`hooks/line.ts`).

## What Changes

- **The windows are a part of the agent seam (211).** Given an agent,
  the usage windows its program last reported for its account, each
  with its share used and its reset, or none yet, or unknown. A program
  whose profile gives none is degraded there: its accounts have no
  reading, which status says, and it refuses no start (171, 174).
- **Claude Code's profile builds it (C.4).** A new module of flume's
  plugin, `hooks/usage.ts`, in a session flume started, passes each
  `session.measure` whose `rateLimits` moved and holds a window to
  `flume hook usage`. The hook keeps the windows beside the start's
  record with the time it ran, and asks the core to take them.
- **The reading is the account's own document's (153, 184, C.7).** The
  core takes a live start's windows, under its secret, as the reading of
  the account the start runs under, in `accounts/<pool>/<account>.json`
  beside its spent time, by compare-and-set: when flume took it, from
  which agent and session, and each window in C.7's form. A reading
  taken earlier than the one kept is dropped. flume asks no program or
  its maker, reads no sign-in and fetches nothing (146).
- **An account keeps its last reading as it ages.** Fresh for fifteen
  minutes, a window past its reset read as nothing used, a window past
  the threshold holding its account until it resets; an account no
  session of flume's reported for has none (184, 185).
- **No host writes a reading.** **BREAKING** The account's `usage` key
  leaves the form: a host file that still names it is refused naming
  the key, as any word an account does not hold, with no alias. The real
  host's reading of a file, the host seam's `usage` and the fake host's
  readings go, and the Nix module no longer declares `usage`.
- **Status gives each account's reading** with when flume took it, from
  which agent's session, whether it is fresh, or why there is none
  (56, 185). Status's JSON form only gains fields (core.14).
- **The clause list adopts 211**, at blueprints 55b89a4, the revision
  that adds it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `account-pools`: an account's reading is the windows its program
  reported in a session flume started, kept in its own document; a
  most-left pool's account no session reported for has none.
- `agent-profile`: Claude Code's windows read from what its plugin kept
  beside the start's record.
- `claude-code-plugin`: the module that passes each report of the
  windows to `flume hook usage`, and the hook that keeps them and has
  the core take them.
- `seam-contracts`: the agent seam's windows; the host gives no reading;
  the reading's types and the account's document of spent time and
  reading.
- `seam-fakes`: the fake agent scripts the windows its program reports;
  the fake host holds no reading.
- `store-profile`: an account's `usage` refused; the account's document
  holds its reading beside its spent time; no file read.
- `place-status`: each account's reading with when it was taken, from
  which agent's session, whether it is fresh, or why there is none.
- `host-module`: an account declares no `usage`.
- `clause-coverage`: the list read at 55b89a4, 211 in Part B.

## Impact

- `plugins/claude`: `hooks/usage.ts` and its tests, `hooks/hooks.json`
  naming it.
- `crates/flume-seams`: `Agent::windows` and `Agent::reports_windows`,
  `Windows`, `AccountReading`, the account's document of spent time and
  reading; `Account::usage`, `Usage` and `Host::usage` go; the workspace's
  chrono for the reading's RFC 3339 times.
- `crates/flume-core`: the take of a live start's windows; the room's
  reading read from the account's document; spent kept beside a reading
  and a reading beside spent; status's reading with its agent and
  session.
- `crates/flume`: the Claude Code profile's `windows`, `flume hook
  usage`, the plugin's files; the configuration refusing `usage`;
  `host/usage.rs` goes; status's person's form.
- `crates/flume-fakes` and `crates/flume-conformance`: the fake agent's
  windows, the agent seam's windows scenario, the account pools'
  scenarios over reported windows, the clause list.
- `nix/`: the module's account options and its tests' files.
- `site/`: agent settings, configuration, state, Claude Code and
  commands pages, and the roadmap.
- `docs/`: record 0062 amended with the construct's choices, still
  accepted. Rule core.32 already says this (record 0062).
- The statement: C.4's `windows` row and the common profile's
  `windows` unknown, on blueprints `design/flume-claude-codes-windows-row`
  at bd79555, cut from main at 81288ba; 153, 171, 184, 185, 211, C.7 and
  the vocabulary's reading say the rest on main.
- No new crate, plugin or site part, so no new moon project; no new
  tool. flume-seams takes the workspace's `chrono`, which flume and
  flume-core already take.
