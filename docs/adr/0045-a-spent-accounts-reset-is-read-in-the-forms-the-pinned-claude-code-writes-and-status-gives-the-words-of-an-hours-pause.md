---
number: 45
title: A spent account's reset is read in the forms the pinned Claude Code writes, a date among them, and status gives the words of an hour's pause
status: proposed
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# A spent account's reset is read in the forms the pinned Claude Code writes, a date among them, and status gives the words of an hour's pause

Status: proposed on 2026-11-05 by the code stage of the fix `a-spent-accounts-reset-is-read-with-its-date`. The direction is the orchestrator's, in its brief's words below. Reading the budget's `your <limit> resets` part, no longer reading a date joined by a comma, and where status gives the words are the stage's choices, made on the statement and these rules.

Deciders: the orchestrator's brief, for the direction; the code stage, for the rest.
Sources: the orchestrator's brief of the fix `a-spent-accounts-reset-is-read-with-its-date`, 2026-11-05; `agentplot/blueprints:design/flume/requirements.md` at ebfbc0d, clauses 56, 146, 151 and C.4, as amended with this record on the branch `design/flume-a-spent-accounts-reset-is-read-with-its-date` at 83af642, and clause 185 as record 0035 wrote it on `design/flume-a-pool-chooses-by-the-allowance-left` at 6188a10; records 0020 and 0035; rules core.14, core.22 and core.32; Claude Code 2.1.296, the release `devenv.nix` pins, its darwin-arm64 and linux-arm64 programs; `crates/flume/src/profiles/agent/claude/limit.rs`, `crates/flume-core/src/spent.rs` and `crates/flume-core/src/room.rs`.

## Context and problem statement

On deneb, 2026-11-05 about 20:40Z, four agents on one account stopped with Claude Code's words "You've hit your weekly limit · resets Nov 8 at 12am (America/New_York) (rate_limit)". flume recorded the account spent until 1793914903, about 21:41Z the same day: an hour, as clause 146 gives where the words name no reset. The words name one, Nov 8 at 00:00 in New York, 05:00Z. An hour later the pool read the account usable, with no agents, the first choice of least-busy, and the next start there would fail the same way. Nothing in status said why the pause was an hour.

The Claude Code profile read a date only as `Nov 2, 3:30pm`, joined by a comma, a form C.4 named and nobody had read from the release.

The orchestrator's brief, 2026-11-05: "a reset that names a date, `resets <Mon> <d> at <time> (<zone>)`, is read as that date and time in that zone, as well as today's `resets 5pm (<zone>)`. Read the forms the pinned Claude Code release writes from its binary; don't infer them. A form flume cannot read is still an hour, and status says the words it could not read."

## What 2.1.296 writes

Read from the program the pinned release's `claude` wrapper runs:

- Its limit messages put the reset in one of two parts. Most carry `` ` \xB7 resets ${y}` ``. A limit hit on a budget, a team's shared budget or a monthly spend cap, carries `` ` \xB7 your ${R.limitName} resets ${R.resetTime}` ``, the plan limit's reset, after where to raise the budget.
- Both times come from one formatter, called with the zone shown. It writes a time of day where the reset is within 24 hours, and a date and a time past that, `i=(o.getTime()-c.getTime())/3600000;if(n||i>24){`. Its date's options are `{month:"short",day:"numeric",hour:"numeric",minute:` the minutes only where they are not zero`,hour12:true,timeZone:s}`. They add `year:"numeric"` only where the reset's year in the zone is not the formatter's clock's. It writes them with `toLocaleString("en-US", …)`, lowercasing ` AM` or ` PM`, and puts `` ` (${s??zIs()})` `` after them.
- How `toLocaleString` joins a date to its time is ICU's, not the program's. The release carries ICU 78 in its Linux build (`icudt78l`), and links the system's `libicucore`, 78.1 on the host this was read on, in its macOS build. The formatter's options run under that ICU, through the system's JavaScriptCore, write `Nov 8 at 12am (America/New_York)` for deneb's reset, `Nov 8 at 3:30pm`, and `Jan 3, 2027 at 9am (Europe/London)` across a year's end. The spaces are plain spaces. A time within a day is `10pm (UTC)` or `1:30am (UTC)`. deneb's words match the first byte for byte.

So the date forms the pinned release writes are `<Mon> <d> at <time>` and `<Mon> <d>, <yyyy> at <time>`. No form it writes joins a date to its time by a comma.

## How the reading fits a host's reading of an account

Record 0035, in verify when this was written, reads each account's usage from a file the host writes and holds an account past its pool's threshold until the window that holds it resets (184, 185). Clause 185 already says a reading never shortens or lengthens a spent account's pause, and flume records nothing spent from a reading. The code holds an account back while either holds it, and gives the later of the two as when it is back (`AccountRoom::short` and `back`).

So the two never disagree about whether an account takes a start: each holds it on its own, and the account takes a start once neither does. Where a fresh reading gave deneb's account its weekly window at 100%, resetting Nov 8 at 05:00Z, the reading held it past its threshold until then whatever the spent pause said. Where the host names no reading, as on deneb that evening, the spent pause alone holds it, and its time must be the words' own. This record changes only how that time is read from the words, and duplicates nothing of record 0035's.

## Considered options

1. **Read the forms the release writes, both reset parts among them; a form flume cannot read stays an hour, the spent document marking it so, and status giving the words while it stands.**
2. **Read the release's forms and also the comma-joined date C.4 named.**
3. **Read the release's forms, the budget's `your <limit> resets` part left an hour.**
4. **Status giving every spent account's words, read or not.**

## Decision outcome

Option 1.

1. **A reset is read as 2.1.296 writes it.** The part is `resets <when> (<zone>)`, or `your <limit> resets <when> (<zone>)` where the limit hit is a budget, wherever it falls among the words' ` · ` parts. `<when>` with no ` at ` is a time of day, the first such time in the zone no more than a minute before the hook ran. `<when>` with one is a date and a time: `<Mon> <d>`, the year the hook ran in that zone, or `<Mon> <d>, <yyyy>`. Each is read in the zone the words name, the earlier moment where the zone's clock passes it twice. A date joined to its time by a comma is no longer read, since the release never writes one.
2. **Each fragment of the release the reading rests on is pinned.** The budget's part, the formatter's choice of a date past a day off, and its options for each form join the fragments the check-tools test finds byte for byte in the pinned release (core.22). A release that changes them fails there until the reading is made again. The join ICU gives is not in the program and no test pins it. A host whose ICU joins otherwise, such as an older macOS's, falls to the hour, and status gives its words.
3. **A form flume cannot read is still an hour, and is marked.** The spent document gains `unread`, true where flume recorded the hour because the profile read no reset in the words, whether they name none or name one in another form. It is left out otherwise, and a document written before reads false. A reset read later from another agent's words replaces the hour where it ends after it, and clears the mark. The same limit read again leaves the standing pause as it is (146, 153).
4. **Status gives the words while the hour stands (56).** Each account of a pool gives `spent_unread`, its agent's words, while it is spent for an hour so marked. The person's form gives them under the account's line: `spent an hour: no reset read in "<words>"`. The JSON form only gains a field (core.14).

Option 2 reads a form the brief says not to infer, which the release never writes. Option 3 leaves a budget's account to be taken again an hour after it stopped, where the words name when its plan limit resets, the same failure as deneb's. Option 4 gives words a person needs only where the pause is the hour; where a reset was read, the spent time says it.

### Consequences

- `AccountSpent` gains `unread`, `AccountStatus` gains `spent_unread`, and `AccountRoom` carries the words. The spent document and the JSON form only gain fields, read with none where a document lacks them.
- The profile's `spent` keeps its seam: a reset it cannot read is none, as before (151). No other profile reads Claude Code's words.
- Clause 146 and C.4's spent row say so (blueprints 83af642, on the branch `design/flume-a-spent-accounts-reset-is-read-with-its-date`; core.19). 56's pool sentence is left as record 0035 amends it, since 146 names what status gives of an hour's pause. Rule core.32's details are amended with this record.
- An account deneb paused for an hour before this release stays as recorded until its hour passes. The next agent stopped there records the reset its words name.
