---
number: 20
title: A host's work runs in named streams, each with its own bound, priority and pool of accounts, and an intent keeps to the account its prompt cache is on
status: accepted
date: 2026-10-29
decision-makers:
- Chuck Swanberg
---

# A host's work runs in named streams, each with its own bound, priority and pool of accounts, and an intent keeps to the account its prompt cache is on

Status: accepted by the user, 2026-10-29, in their design session, on how one host runs work for several purposes under several Claude accounts without one queue and one account per kind.

Deciders: the user, who chose named streams with their own limits, priorities and account pools under the host's limit, the two strategies, the pause honoured by every flume process, affinity for the prompt cache, the deferral of the weekly allowance, and that a kind stays portable, naming nothing a host owns; where the stream sits among record 0017's layers, the affinity's key, an account's own bound, the order of the queue across streams and what happens to a stage whose account is spent are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 6e5aae2, clauses 2, 4, 7, 8, 32, 56, 70, 143 to 147, 151, 153, C.4, C.7, I6 and S53 to S56, and section 10's question on the weekly allowance, as amended with this record; records 0010, 0016, 0017; rules core.26, core.30, core.32; `crates/flume-core/src/room.rs` (`Queue`, `must_wait`), `crates/flume-core/src/settings.rs` (`layers`), `crates/flume/src/host/config.rs` (`HostConfig`); the StopFailure hook flume's Claude Code plugin already holds (`crates/flume/src/profiles/agent/claude/hook.rs`), and Claude Code 2.1.296's `rate_limit` error and its "You've hit your limit · resets …" words.

## Context and problem statement

Record 0010 gave a host one queue under one bound, `[host] agents`, ordered by the callers' priorities alone. Record 0017 let the host's configuration choose the account an agent runs under, by an `env` such as `CLAUDE_CONFIG_DIR` for a kind, or for a repository and kind. Between them a host can run work under one account per kind, and nothing more.

The user runs more than that on one machine: work and personal projects, under different accounts, which should not take each other's room; and several accounts of one plan, where one account reaching its usage limit should not stop the work. Today an account that hits its limit leaves its stage's agent quiet with its last words (record 0016's StopFailure reason), and someone changes the host's configuration and retries, as record 0017's consequences say.

A third fact makes the choice of account more than bookkeeping: the prompt cache belongs to an account. An agent that starts on the account its intent's other agents ran on reads what they share from the cache; one that starts on another account pays to read it all again.

And kinds are to be shared from host to host, through a marketplace later. A kind that named an account, a pool or a stream would carry one host's arrangement to every host that took it.

## Considered options

1. Keep record 0017's environment per kind, one account each, and move accounts by hand.
2. Named streams, each with its own bound, priority and pool of accounts under the host's bound; a pool chooses its account fill-first or least-busy; a spent account paused in the host's state; an intent kept to its account.
3. As 2, but the pool chooses round-robin.
4. As 2, but the streams share the host by weighted shares rather than bounds and priorities.

## Decision outcome

Option 2.

**Streams.** The host's configuration, `~/.config/flume/config.toml`, declares streams by name, `[streams.<stream>]`, each with `agents`, its bound on agents running, `priority`, and `pool`. A stream is a name for a share of the host's work, `work`, `personal`, or one per project. `[host] agents` still caps every stream together (70, 143). A place's stream is the launch's, else its intent's (below), else the one the host's configuration names for its repository and kind, for its repository, or for its kind (`stream` in `[repos."<repository>".kinds.<kind>]`, `[repos."<repository>"]`, `[kinds.<kind>]`), else the stream `default`, which holds everything unnamed and is bounded by the host's bound alone unless a `[streams.default]` table states its own. A stream is fixed for a place at its launch (8). A launch naming a stream the host does not declare is refused naming it (4). Status gives the stream and where it came from, and for each stream its agents running, its bound, its room and its queue (56).

**The queue across streams.** Record 0010's queue holds, with two bounds where it had one: a stage starts only while the host is under its bound, its stream is under the stream's, and an account of its pool is usable (70). A settle starts the waiting stages by the stream's priority first, then the place's, then the order they came to wait. A stage whose stream is full, or whose pool has nothing usable, is passed over for the next that may start, so one full stream never holds another's room. A stream's priority is the host's word, a place's the caller's; flume orders by nothing else. So rule core.26 changes: flume orders places by its callers' priorities and the host's streams'.

**What the host gives a stream, and what it keeps.** A setting given for the whole host that a stream gives too is the stream's first: its pool (`[host] pool` for the whole host) and its agents' settings for a kind (`[streams.<stream>.kinds.<kind>.agents]`, beside record 0017's `[kinds.<kind>.agents]`). The host's bound, its terminal and the runs it wakes for roles (132) stay the host's alone: one machine, one herdr, one waker. A role's run names its stream with `stream` in its `[roles]` table.

**Where the stream sits among record 0017's layers.** Two layers are added, and every other keeps its order:

1. the account chosen from the pool, for its program and the variables it names, and for nothing else;
2. the launch's settings;
3. the host's, for the repository and kind;
4. the host's, for the stream and kind;
5. the host's, for the kind;
6. the repository's, for a kind it does not declare;
7. the kind's declaration;
8. flume's default.

A stream spans repositories and is narrower than the host, so its layer goes between the host's for a repository and kind and the host's for a kind: what the host says of one repository holds in whatever stream that repository runs, and what a stream says of a kind holds over what the whole host says of it. The environment still merges variable by variable (32; rule core.30). The account goes over even the launch's, since an account's limit is counted against the account the agent actually ran under; a launch that names an account's variable shows in status as passed over by the account, never silently.

**Account pools.** `[pools.<pool>]` has `strategy` and `accounts`, in order. An account is a `name`, a `program` and an `env`, read as an agents table's is, `CLAUDE_CONFIG_DIR = "~/.claude-work-1"` for Claude Code (C.4), and may state `agents`, how many run on it at once. A stage's pool is the host's for its stream and kind (`[streams.<stream>.kinds.<kind>] pool`), else its stream's, else its kind's for every stream (`[kinds.<kind>] pool`), else `[host] pool`; with none, record 0017's environment stands as it is (144). An account is usable while it is not spent and has room under its own bound (145). **fill-first** takes the first usable account in order, so the first account's allowance is spent before the second is touched; **least-busy** takes the usable account with the fewest agents running, equals in order. "Until it hits its limit" means either limit: the account's own bound, which fill-first fills, and its usage limit, which pauses it. With nothing usable the stage waits for room, and status gives the soonest reset in the pool. The account is chosen under the room lock (`Queue` in `room.rs`), so two starts never share an account's last room.

**A usage limit.** Claude Code ends a turn at a usage limit with the StopFailure hook's `error` of `rate_limit` and its words, "You've hit your limit · resets …", which flume's plugin already records as the agent's last words (record 0016). The agent seam gains `spent` (151): whether the agent's own record says its account reached a usage limit, and the reset time it names. flume records the account spent until then, or for an hour where the words name no time, in a document of the host's state, `accounts/<pool>/<account>.json` under `$XDG_STATE_HOME/flume/`, written by compare-and-set as a place's is (146, 153). Every flume process on the host reads it, so no start anywhere takes the account before its reset, and nothing asks Claude or its maker whether the limit has reset. The spent agent is ended once quiet, and its stage starts afresh on a usable account, or waits for one, with flume's own words saying the earlier agent stopped at its account's limit and what it left is in the place. No caller is asked and the kind's bound of goings-back is not counted, since nothing about the work failed (112). Running agents on a spent account are never ended for it (31); each moves when its own turn fails.

**Affinity.** The prompt cache belongs to an account, so an intent's agents stay on one account. The key is the **intent**: a name the launch gives, with its brief (record 0021), or, where the launch gives none, the place's repository and base. A bolt in crew's terms is one base (record 0018), so a caller that gives no intent still keeps a bolt's units together; a caller that runs one intent across bases, or several on one base, says so by name. The base alone would merge unrelated launches on `main`, and a bolt name alone is crew's word, which other callers do not have. flume keeps each intent's affinity, its stream and account, in `intents/<digest>.json` beside the accounts' documents (147). Every start of an agent serving the intent takes that account while it is usable; a later place of the intent runs in its stream unless its launch names another. When the account is spent or full the start goes where the strategy chooses, the affinity moves with it and does not move back, and status says the intent moved, from which account to which, why, and that its agents start without the earlier account's cache. An affinity goes an hour after the last place of its intent is gone, the cache's longest lifetime.

**Callers attach by name.** crew's teams and Flywheel will attach to streams. The stream's name is the whole interface: a caller passes it at the launch and reads it in status, and knows nothing else of a stream.

**Kinds stay shareable.** A kind is portable content: its stages, prompts, `repeat` and what it needs. It names nothing one host owns. A kind's declaration and a repository's settings carry no account, no pool, no stream and no environment, and a kind names its program by name, never by a location on a host (32, 144). Where a kind needs something of an account it says it in host-neutral terms, the agent program it supports and the model its settings name, and the host's configuration binds its kinds to its own pools and streams. An account whose program is not the stage's is not usable for it. A shared kind therefore carries no account or stream, and runs on any host that binds it.

**Deferred.** How the most capable model's allowance is spent against an account's weekly limit overall, rather than one usage limit at a time, is not decided now. It stands as a question in the statement's section 10. Until it is decided an account is spent only when its agent's own record says so.

**Why not the others.** Option 1 is today's: one account per kind, a limit that stops work until someone edits the host's file, and no way to keep work and personal from sharing room. Round-robin moves an intent from account to account on every start, throwing away the cache that affinity keeps, and spends every account at once where fill-first keeps the later ones whole. Weighted shares need flume to count use over time and judge fairness, a plan across places that clause 70 keeps from flume. A bound and a priority are facts the host states and flume keeps to.

### Consequences

- Clauses 2, 4, 7, 8, 32, 56, 70, C.4, C.7 and I6 of flume's statement are amended, and 143 to 147, 151, 153 and S53 to S56 added, at blueprints 6e5aae2, with section 10's question on the weekly allowance. Rule core.26 orders by streams' priorities too; core.30 counts eight layers with the account and the stream; core.32 is drawn from this record.
- Status gains each place's stream and account with where each came from, each move with its reason, each stream's queue and each pool's accounts.
- Record 0017's environment per kind still stands for a host that declares no pools.
- Nothing is built until the user approves a plan; `_open.md` names the wait.
