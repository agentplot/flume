---
number: 62
title: flume reads each account's usage from the windows its agent program reports, and no host writes a reading
status: accepted
date: 2026-11-09
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# flume reads each account's usage from the windows its agent program reports, and no host writes a reading

Status: accepted. The direction is the user's, in agentplot-design's pane on 2026-11-09, reading deneb's Boomdrive configuration at an account's `usage = "~/.local/state/swancloud/usage/readings/${name}.json";`: "We should not be passing Flume some custom SwanCloud JSON. Flume is supposed to be figuring out Claude Code usage on its own. This coupling cannot exist". It settles what record 0055 left open. How flume reads the windows is agentplot-design's own choice, cheap to change.

Amends records 0035 and 0055 where an account's reading is a file the host writes.

Deciders: the user, 2026-11-09, above, and on 2026-11-06 in record 0055 ("flume reads each account's usage itself"); agentplot-design, for the form.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` clauses 144 to 146, 152, 153, 171, 174, 184, 185, C.4 and C.7, and `design/principles.md` 14 ("A tool that needs what another program knows reads that program's own source, never a file a host makes for it"); records 0026, 0035, 0045 and 0055; the engine declarations the pinned Claude Code 2.1.296 writes, `plugins/claude/engine-types/claude-code/index.d.ts`: the `session.measure` event, "Fires when the engine measures the session and a unit moved: after each main-thread turn, and when a rate-limit window moves a whole point", its `rateLimits` ("The rate-limit windows the last response reported, each with its `percentUsed`"), and `SessionRateLimit`'s `kind` (`five_hour`, `seven_day`), `percentUsed` (0 to 100) and `resetsAt` (ISO 8601); Claude Code's status line reference, `rate_limits.five_hour.used_percentage` and `resets_at`, "Percentage of the 5-hour or 7-day rate limit consumed, from 0 to 100" (https://code.claude.com/docs/en/statusline); flume's plugin module `plugins/claude/hooks/line.ts`; afterthought/swancloud's `packages/swancloud-usage`, which reads each account's sign-in from the login keychain and asks `GET https://api.anthropic.com/api/oauth/usage` every five minutes to write the file flume reads today.

## Context and problem statement

A pool's `most-left` strategy and its threshold read each account's reading: for each usage window, how much is used and when it resets (184, 185; record 0035). Today the reading is a file the host writes and names in the account's `usage`. On deneb a program of the fleet's writes it, by reading each account's sign-in and asking the maker's usage endpoint. So flume's account form names a file of one host's making, and a host without that program has no reading. Record 0055 ruled that flume reads the usage itself, and left how open, weighed against 146's rule that flume reads no sign-in and fetches nothing.

Claude Code now gives its own sessions' plugins the windows its responses report. Each response from a subscription carries them, and the engine passes them to a plugin's module in the `session.measure` event, and answers them at no cost from `$.session.usage()`. flume's plugin is already loaded into every Claude Code agent flume starts, and its module already runs in each of those sessions (`hooks/line.ts`).

## Considered options

1. **The windows the agent program reports to flume's plugin.** In each session flume started, flume's plugin module passes the windows `session.measure` gives to `flume hook usage`, which records them as the reading of that start's account.
2. **The maker's usage report, asked with the account's sign-in.** What deneb's program does now, moved into flume. flume would read a sign-in and fetch, which 146 forbids, and would hold a token.
3. **The status line command.** flume names its own `statusLine` for its agents, reads `rate_limits` from its input and runs the status line the agent would have had. It takes a key a host or a person sets for their own line, and has flume run their command.
4. **Tokens counted from each agent's transcript against a limit.** The maker alone knows an account's limit, so the count would be a guess.

## Decision outcome

Option 1.

- **The reading is the agent program's own.** In every session of Claude Code that flume started, flume's plugin module, on the engine's `session.measure` whose `rateLimits` moved, and on its first, passes the windows to `flume hook usage` on its input: each window's kind, its share used, from 0 to 100, and when it resets. The hook records them, with the time it took them, as the reading of the account that start ran under, in the host's own document of that account, `accounts/<pool>/<account>.json`, beside its spent time, by compare-and-set (153). A later reading of the same account replaces an earlier one; one taken earlier than the one kept is dropped. flume asks nothing of the program or its maker, reads no sign-in and fetches nothing (146).
- **An account flume runs no agent on keeps its last reading.** It ages as a reading does: fresh for fifteen minutes from when it was taken, a window whose reset has passed read as nothing used, and an account past its threshold held until that window resets, however old the reading (184, 185). An account no session of flume's ever reported for has no reading, and a `most-left` pool chooses among such accounts as `least-busy` does (145).
- **The windows are a part of the agent seam, `windows` (211).** Given an agent, the usage windows its program last reported for its account, each with its share used and its reset, or unknown. Claude Code's profile builds it from the engine's event. A program whose profile gives no such part is degraded there (171, 174): its accounts have no reading, which status says, and it refuses no start.
- **No host writes a reading.** The account's `usage` key leaves the account form. A host file that still names it is refused at that account, naming the key, as any word the form does not hold, until the file is mended (no alias is kept).
- **Status gives each account's reading** with when it was taken and from which agent's session, and whether it is fresh (56).

Not taken:

- **Option 2.** flume would read a sign-in and fetch, which 146 forbids, and would hold a credential.
- **Option 3.** It takes a setting that is the host's and the person's (195), and has flume run a command of theirs as part of its own.
- **Option 4.** A guess at a limit only the maker knows.
- **Keeping the host's file as a second source.** It is the coupling the user ruled out.

### Consequences

- Good: a host writes nothing for flume to know an account's usage, and the reading is the maker's own figure, as each response reports it.
- Good: a reading is as new as the account's last response in a session flume started, where the host's program read it every five minutes.
- Bad: an account with no agent of flume's running keeps an aging reading, and one flume never ran an agent on has none. A person's own sessions on the account count toward its windows but report nothing to flume.
- Bad: a reading needs the engine's event, from the Claude Code release flume's profile pins (C.4).
- The statement gains 211, and its 153, 171, 184, 185, C.4, C.7 and the vocabulary's reading say it, at boomdrivedev/blueprints. core.32 says it here.
- deneb's `packages/swancloud-usage` stops writing readings for flume, and the Boomdrive set names no `usage`. That is swancloud's and Boomdrive's to follow.
