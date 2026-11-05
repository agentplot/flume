---
number: 20
title: A host's work runs in named streams, each with its own bound, priority and pool of accounts, and an intent keeps to the account its prompt cache is on
status: accepted
date: 2026-10-29
decision-makers:
- Chuck Swanberg
---

# A host's work runs in named streams, each with its own bound, priority and pool of accounts, and an intent keeps to the account its prompt cache is on

Status: accepted by the user, 2026-10-29, in their design session, on how one host runs work for several purposes under several Claude accounts without one queue and one account per kind. Amended 2026-10-29 by the user, approving its plan: accounts exist only in pools and give an agent its whole environment. Amended 2026-11-02, the first of the two sections below, accepted by the user the same day on their review of the change that builds it: an agent stopped at its account's usage limit goes on with its own conversation on the account its pool chooses, where its program can carry it there. Amended again 2026-11-02, the second section below, accepted by the user the same day on their Pending You card req_f861af4d, "Fix both": an intent keeps an account for each program its agents run, and a stage whose pool holds no account that can run it is refused, at its launch or a later start, rather than left waiting.

Deciders: the user, who chose named streams with their own limits, priorities and account pools under the host's limit, the two strategies, the pause honoured by every flume process, affinity for the prompt cache, the deferral of the weekly allowance, that a kind stays portable, naming nothing a host owns, and that accounts exist only in pools, a stream or the host naming the pool and no other layer giving an environment or choosing an account; that the host's tables give no program, the layers' order without the environment, the affinity's key, an account's own bound, the order of the queue across streams and what happens to a stage whose account is spent are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 6e5aae2, clauses 2, 4, 7, 8, 32, 56, 70, 143 to 147, 151, 153, C.4, C.7, I6 and S53 to S56, and section 10's question on the weekly allowance, as amended with this record, and at 87a7458, clauses 29, 32, 143, 144, C.2, C.7 and S44, as amended with it; records 0010, 0016, 0017; rules core.26, core.30, core.32; `crates/flume-core/src/room.rs` (`Queue`, `must_wait`), `crates/flume-core/src/settings.rs` (`layers`), `crates/flume/src/host/config.rs` (`HostConfig`); the StopFailure hook flume's Claude Code plugin already holds (`crates/flume/src/profiles/agent/claude/hook.rs`), and Claude Code 2.1.296's `rate_limit` error and its "You've hit your limit · resets …" words.

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

**What the host gives a stream, and what it keeps.** A stream gives its places a bound, a priority and a pool, and nothing else: its pool comes before `[host] pool`, the pool for the whole host. The host's bound, its terminal and the runs it wakes for roles (132) stay the host's alone: one machine, one herdr, one waker. A role's run names its stream with `stream` in its `[roles]` table.

**The environment is the account's, and the layers give the rest.** An account is a program and its environment, and the account is the only source of an agent's environment: the host user's login environment with the account's variables over it, each `~` and `$VAR` read against that login environment (29). No launch, host, repository or kind layer gives a variable or chooses an account, and a stage with no pool runs under the login environment alone. Record 0017's layers keep their order for everything else:

1. the launch's settings;
2. the host's, for the repository and kind, its model and instructions;
3. the host's, for the kind, its model and instructions;
4. the repository's, for a kind it does not declare;
5. the kind's declaration;
6. flume's default.

The host's two tables give a model and instructions alone, so a host runs `change` on opus and `fix` on sonnet whichever account each runs under. They give no program: the host chooses a program by the accounts it pools, and a program named in its tables could only disagree with them. The program is the launch's, the repository's, the kind's or flume's default, the program the work is written for, and an account whose program is not the stage's is not usable for it. A stream gives no settings: it is a share of the host's room and a pool, and a stream's word on a kind's model would be a third host table for what the host's word for the kind already says (32; rule core.30).

**Account pools.** `[pools.<pool>]` has `strategy` and `accounts`, in order. An account is a `name`, a `program` and an `env`, its values' `~` and `$VAR` read against the login environment, `CLAUDE_CONFIG_DIR = "~/.claude-work-1"` for Claude Code (C.4), and may state `agents`, how many run on it at once. Accounts exist only in pools. A stage's pool is its stream's, else `[host] pool`; with none, its agents run under no account, in the login environment alone (144). An account is usable while it is not spent and has room under its own bound (145). **fill-first** takes the first usable account in order, so the first account's allowance is spent before the second is touched; **least-busy** takes the usable account with the fewest agents running, equals in order. "Until it hits its limit" means either limit: the account's own bound, which fill-first fills, and its usage limit, which pauses it. With nothing usable the stage waits for room, and status gives the soonest reset in the pool. The account is chosen under the room lock (`Queue` in `room.rs`), so two starts never share an account's last room.

**A usage limit.** Claude Code ends a turn at a usage limit with the StopFailure hook's `error` of `rate_limit` and its words, "You've hit your limit · resets …", which flume's plugin already records as the agent's last words (record 0016). The agent seam gains `spent` (151): whether the agent's own record says its account reached a usage limit, and the reset time it names. flume records the account spent until then, or for an hour where the words name no time, in a document of the host's state, `accounts/<pool>/<account>.json` under `$XDG_STATE_HOME/flume/`, written by compare-and-set as a place's is (146, 153). Every flume process on the host reads it, so no start anywhere takes the account before its reset, and nothing asks Claude or its maker whether the limit has reset. The spent agent is ended once quiet, and its stage starts afresh on a usable account, or waits for one, with flume's own words saying the earlier agent stopped at its account's limit and what it left is in the place. No caller is asked and the kind's bound of goings-back is not counted, since nothing about the work failed (112). Running agents on a spent account are never ended for it (31); each moves when its own turn fails.

**Affinity.** The prompt cache belongs to an account, so an intent's agents stay on one account. The key is the **intent**: a name the launch gives, with its brief (record 0021), or, where the launch gives none, the place's repository and base. A bolt in crew's terms is one base (record 0018), so a caller that gives no intent still keeps a bolt's units together; a caller that runs one intent across bases, or several on one base, says so by name. The base alone would merge unrelated launches on `main`, and a bolt name alone is crew's word, which other callers do not have. flume keeps each intent's affinity, its stream and account, in `intents/<digest>.json` beside the accounts' documents (147). Every start of an agent serving the intent takes that account while it is usable; a later place of the intent runs in its stream unless its launch names another. When the account is spent or full the start goes where the strategy chooses, the affinity moves with it and does not move back, and status says the intent moved, from which account to which, why, and that its agents start without the earlier account's cache. An affinity goes an hour after the last place of its intent is gone, the cache's longest lifetime.

**Callers attach by name.** crew's teams and Boomdrive will attach to streams. The stream's name is the whole interface: a caller passes it at the launch and reads it in status, and knows nothing else of a stream.

**Kinds stay shareable.** A kind is portable content: its stages, prompts, `repeat` and what it needs. It names nothing one host owns. A kind's declaration and a repository's settings carry no account, no pool, no stream and no environment, and a kind names its program by name, never by a location on a host (32, 144). Where a kind needs something of an account it says it in host-neutral terms, the agent program it supports and the model its settings name, and the host's configuration binds its kinds to its own pools and streams. An account whose program is not the stage's is not usable for it. A shared kind therefore carries no account or stream, and runs on any host that binds it.

**Deferred.** How the most capable model's allowance is spent against an account's weekly limit overall, rather than one usage limit at a time, is not decided now. It stands as a question in the statement's section 10. Until it is decided an account is spent only when its agent's own record says so.

**Why not the others.** Option 1 is today's: one account per kind, a limit that stops work until someone edits the host's file, and no way to keep work and personal from sharing room. Round-robin moves an intent from account to account on every start, throwing away the cache that affinity keeps, and spends every account at once where fill-first keeps the later ones whole. Weighted shares need flume to count use over time and judge fairness, a plan across places that clause 70 keeps from flume. A bound and a priority are facts the host states and flume keeps to.

### Consequences

- Clauses 2, 4, 7, 8, 32, 56, 70, C.4, C.7 and I6 of flume's statement are amended, and 143 to 147, 151, 153 and S53 to S56 added, at blueprints 6e5aae2, with section 10's question on the weekly allowance. Rule core.26 orders by streams' priorities too; core.30 counts six layers and takes the environment from the account alone; core.32 is drawn from this record.
- Status gains each place's stream and account with where each came from, each move with its reason, each stream's queue and each pool's accounts.
- Nothing is built until the user approves a plan; `_open.md` names the wait.
- Amended 2026-10-29: the user, approving the plan, ruled out an environment or a program in the host's tables for a kind and for a repository and kind, and a pool named for a kind: "I don't think we want the old per-kind config." Accounts exist only in pools, a stream or the host names the pool, and the account gives an agent its whole environment; the host's tables give a model and instructions. The sections above say what stands; clauses 29, 32, 143, 144, C.2, C.7 and S44 are amended at blueprints 87a7458, core.30 and core.32 with them, and record 0017 carries a line saying so.
- Amended 2026-11-03 by three records of their own, each proposed on the user's cards: a stream's agents run in a terminal session of the stream's own (record 0031); the host's configuration has a managed layer and a local one over it (record 0032); and a stream shares its agents among its kinds' labels (record 0033).
- Amended 2026-11-05 by record 0043, proposed: a stream's agents take the settings the host's configuration gives for the stream, `[agents.streams.<stream>]`, beside `[agents]` for every agent. The reason this record gave for none, that a stream's word on a kind's model would be a third host table for what the host's word for the kind already says, holds no longer: what an agent shows and does differs by the share of the host it runs in, which no kind table can say. Clause 143 is amended at blueprints bc72003, core.30's details with it.

## Amendment, 2026-11-02: a spent account's agent keeps its conversation

Status: accepted by the user on 2026-11-02, on their review of the change `a-spent-seats-agent-resumes-on-another-in-its-pool`'s proposal and design (Plannotator review pn-5e50a4); proposed the same day on their card.

Deciders: the user, who asked that an agent whose account runs out move to another account of its pool and go on with its own conversation there, a planning or research agent in a long talk with its person, or a stage half way through its work, continuing where it stopped; that where its program cannot carry the conversation the stage start afresh as before, status saying which; that flume never copy credentials; and that the rest of this record stand: running agents never ended for their account, nothing asking an agent program whether a limit reset, no going-back counted. What a host provides, how flume tells, the part of the agent seam, which start carries a conversation, and what follows a carried start that does not begin are the writing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 7beb5fa, clauses 28, 30, 31, 33, 35, 43, 56, 88, 109, 112, 145 to 147, 151, 161, 170, 171, 174, C.4 and S55, as amended on main at 72410b2, which adds 179, S78 and S79; records 0016, 0024, 0026; rules core.10, core.30, core.32; `crates/flume-core/src/spent.rs` (`read_spent`), `crates/flume-core/src/start.rs` (`claim`, `StartReason::Spent`), `crates/flume-core/src/deliver.rs` (`never_begun`), `crates/flume/src/profiles/agent/claude/` (`command` in `mod.rs`, `Started` in `record.rs`, `transcript.rs`); Claude Code 2.1.296's `-r, --resume [value]`, "Resume a conversation by session ID", its `--fork-session`, "create a new session ID instead of reusing the original", a session's transcript at `<configuration home>/projects/<project folder>/<session>.jsonl`, and its refusal "No conversation found with session ID: ".

### Context and problem statement

The decision above starts a spent agent's stage afresh: a new agent reads what the earlier one committed and left in the place, and nothing of its conversation. A stage half way through its work loses whatever its agent had worked out and not yet written down, and reads again what the earlier agent had already read. A conversation (109), an agent planning or researching with its person over an hour, loses the talk itself, and the person starts it again.

Claude Code keeps a session's conversation as its transcript in its configuration home, `<home>/projects/<project folder>/<session>.jsonl`. 2.1.296 resumes a session by `--resume <session>`, looks for it under its own configuration home's `projects` alone, and goes on with it in the same transcript under the same session. An account is a configuration home (C.4), so an agent under another account looks in another `projects` folder, and finds the conversation only where the host made the two one folder.

### Considered options

1. Keep the fresh start.
2. Resume the stopped agent's own conversation on the account the pool chooses, where its program can carry it there: the host makes the conversation reachable from the accounts that should carry it, flume reads whether it is, and a stage whose conversation cannot be carried starts afresh as before.
3. As 2, flume copying the transcript into the chosen account's configuration home first.
4. As 2, resuming from the transcript's path in the earlier account's configuration home, which 2.1.296's `--resume` also takes.
5. Wait for the spent account's reset and resume there.

### Decision outcome

Option 2.

1. **A part of the agent seam, carry (179).** Given the start whose agent stopped at its account's usage limit and the environment of the account chosen for the next start, the agent's profile answers whether its program can carry that agent's conversation to an agent under that account, with the session the next start resumes; or why it cannot; or that its program gives no way to. The answer is read from what the host provides, and flume never makes it so. The common profile offers none (171), and status names the part degraded (170, 174).
2. **Which start carries.** Only the start 146 makes after its stage's agent stopped at its account's limit carries a conversation, and only that agent's, whether it starts at once or after waiting for a usable account (145). A retry, a fix round, a start after one begun without its settings (161) and every start of a later stage carry none (30). The account is chosen as 145 chooses, the intent's affinity first, and carry is asked of the account chosen, under the room lock. A pool of one account carries its agent's conversation across its own reset.
3. **A carried start is a start.** Its agent is one fresh process under the start's own identity and secret (30, 33), with the settings and the standing prompt resolved at that start (32, 148). It is the stage's only on its own word that it started under its start's record (35, 161), which names the session it resumes: an agent that came up in another session is not the stage's. Its prompt is flume's words alone, saying its last turn stopped at its account's usage limit and that it goes on under an account of its pool, and the way to stop short (28), since its conversation already holds the stage's prompt. It counts toward no bound of the kind's (112) and waits on no caller's word.
4. **How Claude Code carries.** The profile answers yes where the earlier start's transcript, as its started ack named it, is the same file, by device and inode, as `<home>/projects/<the transcript's folder>/<the transcript's file>`, `<home>` being the chosen account's `CLAUDE_CONFIG_DIR`, else `~/.claude` of the login environment. The folder's name is the one Claude Code itself gave, read from the ack, so flume computes no name of Claude Code's. The next start's command adds `--resume <session>`, and its SessionStart hook compares the session it runs with the one its record names. Each reading the profile makes of a start's transcript, own work, last words, asking's answer and a slash command's receipt, reads only what follows the transcript's length when that start's first started ack came. Otherwise the earlier agent's background tasks, never finished because it was ended, would read as the new agent's own work, and it would never be quiet (31, 91).
5. **What a host provides.** For Claude Code, the accounts that should carry conversations between them share one `projects` folder: each configuration home's `projects` is one folder, as a symbolic link from one to the other, or from each to a folder of the host's, makes it. Each home keeps its own credentials. flume copies, links and writes nothing in a configuration home, and only reads whether the transcript is there. A host that shares nothing gets the fresh start, and status says the conversation stayed behind and why, naming `CLAUDE_CONFIG_DIR` and never its value (core.30).
6. **A carried start that does not begin.** A carried start never begun (35), as when its program finds no such conversation and ends, or whose agent is not the stage's (161), is not delivered: flume ends its agent once quiet and starts the stage afresh at once, carrying nothing, with 146's words, counting toward no bound and waiting on no caller. Status says the conversation could not be carried and why. The fresh start after it is read as any start is, and one that begins without its settings is never begun as 161 says.
7. **Status.** With the agent, for a start after a spent one: that it carries the conversation of the stage's start before it, or why it does not. With the stage's move between accounts: whether its conversation went with it. The earlier account's prompt cache is lost either way, and a carried agent's first request reads its whole conversation uncached, which status says as it says it of a fresh start (56).
8. **What stands.** Agents running on a spent account are never ended for it (31). Nothing asks an agent program or its maker whether a limit has reset (146). The account's pause, the affinity and its moves are as the sections above decide.

### Why not the others

Option 1 is what the user asked to change. Option 3 has flume write into a configuration home, which is the host's and its program's: a copy of a file in a format flume does not own, two copies of one conversation that then differ, and one step from copying the home's other files, its credentials among them. Option 4 has the session go on writing in the earlier account's home while its other state is in the chosen one's, and carries conversations between accounts on a host that never chose to share them. Sharing `projects` is one choice a host makes once and can see. Option 5 holds the stage for the hours until a reset, which a pool exists to avoid; a pool of one account does it anyway, and option 2 carries the conversation there too.

### Consequences

- The statement's 30, 56, 88, 146, 161 and 171, C.4's command and started rows, its paragraph on accounts and its common profile's table, and S55 are amended, and 179, C.4's carry row, S78 and S79 added, on blueprints main at 72410b2, accepted with this amendment (core.19). 179, S78 and S79 follow the statechart kinds' 175 to 178, S76 and S77 (records 0028 and 0029).
- Rule core.32 and core.10's details are amended with it. `_open.md` names the build until it lands.
- Accounts that share `projects` share everything Claude Code keeps of a project there, what their agents remember of each project among it. The site's agent-settings guide says so beside how a host shares the folder.
- The change `a-spent-seats-agent-resumes-on-another-in-its-pool` builds it.

## Amendment, 2026-11-02: an intent keeps an account per program, and a pool that cannot run a stage refuses it

Status: accepted by the user on 2026-11-02, on their Pending You card req_f861af4d, "Fix both".

Deciders: the user, who chose to fix both: that an intent keep an account for each program its agents run, and that a stage whose pool holds no account that can run it be refused rather than left waiting. The affinity's document's shape, how a document written before reads, the move's program, and where the refusal is read, at the launch, at a later start and at the settle of a waiting stage, are the writing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` on main at a15d927, clauses 4, 32, 145 and 147 and the definition of affinity as amended there, and S80 and S81 added; records 0017 and 0026; rules core.26, core.30, core.32; `crates/flume-core/src/accounts.rs` (`Engine::choose_account`, `Engine::affinity_after`, `PoolRoom::usable`), `crates/flume-core/src/room.rs` (`Queue::wants`, `Engine::start_waiting`), `crates/flume-core/src/launch.rs` (`make_place`), `crates/flume-seams/src/store.rs` (`Affinity`).

### Context and problem statement

The decision above keeps an intent's affinity as one account: its stream and the account its first agent started on. A place's stages may run different programs, code on Claude Code and review on Codex, and one account runs one program (144). A review on Codex finds the Claude Code account not usable for its program and takes the strategy's choice, so no account is kept for Codex and the review loses its prompt cache from start to start. Where the Claude Code account is at its bound when the review starts, flume reads it full and records the intent moved to the Codex account; the next code start finds that account not usable for Claude Code and moves the intent back. Each move says the cache was lost, when nothing of the kind happened.

And a stage whose pool holds no account of its program waits for room (145). `PoolRoom::usable` answers that no account runs the program, the queue gives the stage as wanting one, and every settle passes over it. No reset ends that wait, and status points at the pool as though one would. A launch already refuses a program no agent profile binds, and a stage at a later start a setting its profile cannot honour (4, 32, 169), so the launch could have said so.

### Considered options

For the affinity:

1. Keep one account, a start on another program taking the strategy's choice and moving nothing.
2. An account per program the intent's agents run, a start taking the one for its stage's program.
3. An account per program and stage.

For a pool that holds no account of a stage's program:

1. Wait, status naming the pool, as today.
2. Refuse the launch, and the stage at a later start, naming the stage, the pool and the program.
3. Refuse the launch alone.

### Decision outcome

For the affinity, option 2; for the pool, option 2.

1. **An account per program (147).** An intent's affinity is its stream and, for each program its agents run, the pool and account the first of them on that program started on. Every later start of an agent serving the intent takes the account of its affinity for the stage's program while that account is usable, and otherwise goes where the strategy chooses. A start on one program leaves the intent's account for every other program as it was, and is no move of the intent. A spent or full account moves the affinity for its own program alone, never back, and status says the intent moved, for which program, from which account to which and why, and that its agents on that program start without the earlier account's cache (56). The key is the program as the stage's layers resolve it and the account's `program` names it, the string that makes the account usable for the stage (144). Each program keeps its own pool, since an intent's places may run in streams of different pools (143).
2. **A document written before.** A document written before holds one account and its pool and names no program; only the host's configuration does. flume reads it as the affinity for the program the pool the host's configuration declares gives that account, and gives that program to each of its moves recorded with none. Where the pool no longer declares the account, the account is dropped and the stream kept, since no stage could take it. The next write puts the document in the new shape, and a read that changes nothing writes nothing (core.17).
3. **A pool that cannot run a stage refuses it (4, 32, 145).** A stage waits for room only while its pool holds an account that can run it, spent or without room. Where it holds none, no wait would end. The launch is refused, naming the stage, the pool and the program, with nothing made (core.8). At a later start, read against the host's configuration as it stands, the stage is refused the same way, starts no agent and waits on its caller's retry (27, 32). A waiting stage whose pool loses its last account of its program is refused at the next settle, whatever room the host has. The queue gives no stage as wanting a program any more, since none waits on one.
4. **What stands.** The strategies, the spent pause, the carry, the queue's order and the hour an affinity outlives its intent's last place are as the sections above decide. A stage whose layers name no program (173) takes the halves of 145 and 147 the statement gives it, the pool's accounts whose program can run it and the affinity for the program the intent's last agent ran, when record 0026's change builds 173.

### Why not the others

The affinity's option 1 is today's: the second program never keeps an account, and an account full at the wrong moment moves the intent to an account that cannot run its first program, a move status reports as a cache lost. Option 3 splits one program's cache between its stages, when the cache belongs to the account and an intent's stages on one program share what it holds, the same brief and the same tree. The pool's option 1 is a wait that cannot end, said as a wait for a reset. Option 3 leaves a host that drops an account after the launch with that same wait; the host's configuration is read at each start (32), and a stage refused then is retried once the pool is mended.

### Consequences

- The statement's 4, 32, 145, 147 and the definition of affinity are amended, and S80 and S81 added, on blueprints main at a15d927, accepted with this amendment (core.19). S80 and S81 follow the carry's S78 and S79.
- Rule core.32 is amended with it, its rule and its details, and core.26's and core.30's details: a stage refused for its pool is not refused for want of room, and a pool that cannot run a stage is found at the launch where the launch can find it.
- A host that removes a program's last account from a pool while a stage of that program waits sees the stage refused, and retries it once the account is back.
- An affinity holds an entry for each program its intent ran, for an hour after its last place.
- The halves of 145 and 147 for a stage whose layers name no program wait on record 0026's change, which builds 173: until it lands every stage resolves a program, flume's default naming Claude Code's, and the binary runs no second program, so S80 is held by the engine's scenario alone until then.
- `_open.md` names the build until it lands. The change `an-intents-affinity-is-kept-per-program` builds it.
