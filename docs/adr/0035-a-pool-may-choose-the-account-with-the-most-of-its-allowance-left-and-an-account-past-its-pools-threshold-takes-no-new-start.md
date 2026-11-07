---
number: 35
title: A pool may choose the account with the most of its allowance left, by a reading the host writes, and an account past its pool's threshold takes no new start
status: accepted
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# A pool may choose the account with the most of its allowance left, by a reading the host writes, and an account past its pool's threshold takes no new start

Status: accepted by the user on 2026-11-05 in their Plannotator Inbox, on the change `a-pool-chooses-the-seat-with-the-most-quota-left`, whose design, holding this decision and the statement's clauses, was attached. Asked "Choose a seat by the quota it has left: approve both designs, so their code starts?", they answered "Approve both: start their code". They approved the clauses as the design wrote them, and agentplot/blueprints main states them at 6188a10 (core.19).

Amended in part by record 0055, accepted on 2026-11-06: an account's allowance is read by flume itself from its agent program's own source, and no host writes a reading for it; how flume reads it, and what of the `usage` file stays, is flume's design still to make, until which the reading is read as decided here.

Deciders: the user, on 2026-11-05 in the Plannotator Inbox, deciding what to do with claude-swap (github.com/realiti4/claude-swap): "Take its ideas into the shim and Flume's pools; don't install it". Of the four ideas they chose, as they were put to them, two are flume's pools': "Choose by quota left, not by agents running. Flume's pool and `claude-account use` prefer the seat with the most 5-hour and 7-day quota left, read without renewing anyone's login." and "A seat near its limit takes no new work. Past a threshold, say 90% of either window, new sessions and agents go to another seat. Running ones finish where they are, and no seat's login ever moves." A seat in their words is an account in flume's. The writing agent's: that the choice is a third strategy beside fill-first and least-busy; how an account's windows combine into what it has left; the ties; that the threshold holds under every strategy, ninety percent where a pool states none; where flume reads the reading from, and its form; how long a reading is fresh, and what a stale reading, a window past its reset and a missing reading mean; that flume records nothing of a reading and that a reading never changes a spent account's pause; and what status gives.
Sources: the brief of the change `a-pool-chooses-the-seat-with-the-most-quota-left`, which carries the user's words and the four ideas as they were put to them; a close reading of claude-swap 0.29.0 (tag `v0.29.0`, commit `480d194`), made for a design that weighed installing it and read here for claude-swap's behavior alone; `agentplot/blueprints:design/flume/requirements.md` on main at 4ec26f7, clauses 29, 31, 32, 56, 58, 70, 144 to 147, 151, 153, 165 to 167, C.7, the vocabulary's pool and usable, section 10's question on the weekly allowance, and S54 to S56; records 0020 and its two amendments, 0025, and 0030 (proposed); rules core.15, core.26, core.30, core.32, core.38; `crates/flume-core/src/accounts.rs` (`AccountRoom::short`, `PoolRoom::strategy`, `Usable`), `crates/flume-core/src/room.rs` (`Engine::queue`), `crates/flume-seams/src/host.rs` (`Strategy`, `Account`, `Pool`, `Host`), `crates/flume/src/host/config.rs` (`pools_declared`); the user's approval in the Plannotator Inbox on 2026-11-05, thread msg_01M6MS7EZB1YS8RBR7MYEDZ9SF, message msg_01M6PA6PEZW5WCX96DW1BP7N9R; `agentplot/blueprints:design/flume/requirements.md` on main at 6188a10, which states 184, 185 and S86 to S88 and the amended 56, 144, 145, 147, C.7, the vocabulary's pool and usable, reading and past its threshold, and section 10's question on the weekly allowance.

## Context and problem statement

A pool chooses an account fill-first or least-busy (record 0020). Neither reads how much of an account's allowance is left. least-busy counts flume's agents running on each account. That count leaves out the sessions a person runs under the account, the work its agents did in the hours before, and how far through its week the account is. An account is set aside only once an agent's own record says it reached its usage limit (146). So the last agent started on an account near its limit is the one that stops mid-turn, and its stage moves.

A host can know more. The maker of an agent program reports, for each signed-in account, how much of each usage window it has used and when the window resets: Claude's are a 5-hour window and a 7-day one. claude-swap reads that report, and when an account runs low it switches a folder's login to another account's. It keeps its own copy of each login and renews the idle copies. Each renewal rotates the login's refresh token, so the account's own folder's login dies at its next renewal, and every session and agent on it stops with `invalid_grant`. flume never touches a login (146), and its accounts are folders a person signed in once.

So flume's part is to choose among its accounts by what each has left, without asking the maker itself and without renewing any login. The host reads the windows its own way and writes what it read where flume can read it.

## Considered options

For the choice:

1. Keep fill-first and least-busy alone.
2. A third strategy, most-left: the usable account with the most left by the host's reading of it.
3. least-busy reading the host's reading where its accounts give one, and agents running where they do not.
4. Most left per agent: what an account has left divided by its agents running and one.

For an account near its limit:

1. Nothing new: an account is set aside only once spent.
2. A pool's threshold, past which an account takes no new start under every strategy, until the window that holds it resets.
3. As 2, under most-left alone.
4. As 2, the threshold a preference: an account past it taken only where no other is usable.

For where the reading comes from:

1. flume asks the maker's usage report itself, with each account's sign-in.
2. The agent program's own record of its account's use.
3. A command the host's configuration names, which flume runs at each choice.
4. A file per account, named by `usage` in the account's entry of the host's configuration, which the host writes in flume's form.
5. A file per account at a path flume fixes under its own state directory.

## Decision outcome

For the choice, option 2; for an account near its limit, option 2; for the reading, option 4.

1. **most-left (145).** A pool's `strategy` may be `most-left`. What an account has left is the least that any window of its fresh reading leaves, 100 less the window's `used`, so the window that would stop it first decides. The usable account with the most left is chosen, then, after every account with a fresh reading, each with none; equals in either by the fewest of flume's agents running, then in the pool's order. A most-left pool none of whose accounts has a fresh reading chooses as least-busy does. The intent's affinity for the stage's program still comes first while it is usable (147).
2. **A threshold (185).** A pool's `threshold` is a percentage, ninety where the pool states none. An account whose reading gives a window used to the threshold or past it is past its threshold until that window resets, however old the reading, since a window's use only grows until it resets. Where the window names no reset, it is past its threshold only while the reading is fresh. An account past its threshold is not usable: it takes no new start under any strategy, the affinity's among them, and the affinity moves off it as off a spent or full account, the move saying which window and until when. Its agents running go on where they are and are never ended or moved for it (31); one whose turn stops at the limit moves as 146 says. flume changes nothing of the account's login.
3. **The reading (184).** An account may name `usage`, the path of a file of the host's, read as an `env` value is. The host writes the file whole, a new file renamed over the old. flume reads it at each choice and when status is asked, and never writes, keeps or renews anything of it. Its form is one JSON document: `read_at`, when the host read the allowance, and `windows`, each with a `name`, its `used` from 0 to 100, and an optional `resets_at`; each time is RFC 3339's with its offset. flume ignores every key it does not know. A reading is fresh for fifteen minutes from its `read_at`, by the host's clock. A window whose reset has passed reads as nothing used. A file that is missing, or not in the form, is no reading: status says why, and it refuses nothing.
4. **What stands.** A spent account (146) is set aside until its reset whatever its reading says. A reading never shortens or lengthens that pause, since the agent's own record is the program's word and a reading may lag. flume records nothing of a reading in its own documents (153): it is read where it lives, as status reads a deliverable's facts where they live (58). least-busy, fill-first, the affinity, the carry (179) and the queue's order stand. A host that names no reading runs as before.
5. **Points and pace.** A stream's points bound and its pace (166, 167) hold its first starts by its callers' estimates. An account's reading decides which account any start takes. Neither reads the other: flume turns no points into a share of an allowance and no reading into points (core.38), and plans nothing across places (70). A stage either holds waits for room, and status says which.
6. **Status (56).** For each pool, its strategy and threshold. For each account, its reading: each window's name, use and reset, how old the reading is and whether it is fresh, or why there is none; and, while the account is past its threshold, which window holds it and until when. The pool's soonest time an account stops being spent counts the end of a threshold too.

### Why not the others

The choice's option 1 leaves what the host knows unused. least-busy's count of agents is no measure of an allowance: a person's own sessions and the hours before are not in it. Option 3 changes what least-busy means on a host that adds a reading, so one name chooses two ways; a strategy's name should say how it chooses on every host. Option 4 turns the user's "not by agents running" back into agents running, and guesses that every agent spends alike.

For an account near its limit, option 1 is today's: an account's limit is found by the agent it stops. Option 3 lets a fill-first or least-busy pool keep starting work on an account a reading shows nearly spent. Option 4 lets a pool whose accounts are all past the threshold start more work on them, and that is the work the threshold keeps from stopping mid-turn. A host that wants every last share spent sets the threshold to 100.

For the reading, option 1 has flume read each account's sign-in and ask the maker, which flume does not do (146): it fetches nothing, reads no credential, and the asking can itself renew a login. Option 2 speaks only for an account an agent runs on, and only once that agent asked; it cannot rank an idle account, which is the one the choice most needs to see. Option 3 runs a host's command under the room lock at every start, at a cost and with a reach flume cannot bound: option 1's fetch, made by another program. Option 5 keys the reading by flume's pool and account names, so the host's reader must know flume's configuration. It also puts a writer that is not flume in flume's own directory, whose documents flume writes by compare-and-set (153).

### Consequences

- The statement gains 184, 185 and S86 to S88, and 56, 144, 145, 147, C.7, the vocabulary's pool and usable, and section 10's question on the weekly allowance are amended, as the change's design writes them, once the user approves (core.19). Rule core.32 is amended citing them, and core.30's detail on `~` and `$VAR` gains an account's `usage` beside its `env`, in the build's first group. agentplot/blueprints main states them at 6188a10.
- A host that wants the choice runs a reader of its own. The reader writes each account's file at least every five minutes, and the host's managed configuration names the files and `strategy = "most-left"` (C.7). The change's design gives the form.
- An account past its threshold leaves the share above it unspent until its window resets. A host that wants it spent sets the threshold higher.
- The change `a-pool-chooses-the-seat-with-the-most-quota-left` builds it.
