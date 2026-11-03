# account-pools Specification

## Purpose
The pools of accounts a host's work runs under: a stage's pool from its
stream or the host, the account chosen by the pool's strategy among those
usable for the stage's program, an intent kept to its account while it is
usable, and a spent account set aside until its reset.

## Requirements

### Requirement: A stage's pool

Accounts SHALL exist only in pools. A stage's pool SHALL be its
stream's, else the host's own; a stage with neither SHALL run under no
account, in the login environment alone. A pool SHALL name its accounts
in order and a strategy, `fill-first` or `least-busy`, and an account a
name, a program, an `env` and an optional bound (29, 144, C.7).

#### Scenario: The stream's pool before the host's

- **WHEN** `[streams.work] pool = "work"` and `[host] pool = "personal"`, and fix@1 places run in `work` and in `default`
- **THEN** the place in `work` takes accounts of `work` and the place in `default` accounts of `personal` (144)

#### Scenario: No pool anywhere

- **WHEN** the host names no pool
- **THEN** every agent starts under no account, its environment the login environment alone and no variable named in its settings (29, 144)

### Requirement: An account is chosen under the room lock

At each start of an agent with a pool flume SHALL choose, under the room
lock, the account of the intent's affinity for the stage's program while
usable, else under fill-first the first usable account in order, under
least-busy the usable one with fewest of flume's agents, equals in
order. Usable SHALL mean not spent, under its own bound and running the
stage's program. With none usable the stage SHALL wait for room while
its pool holds an account of its program (145, 147).

#### Scenario: Fill-first and least-busy

- **WHEN** a stream's pool is fill-first over two accounts, the first bounded at two agents, and three stages start; then the same under least-busy
- **THEN** fill-first starts two under the first account and one under the second, and least-busy alternates first, second, first (S54)

#### Scenario: Two starts and one account's last room

- **WHEN** two settles at once each start a stage on a pool whose only account has one agent's room left
- **THEN** exactly one stage starts on it and the other waits for room (145)

#### Scenario: An account for another program

- **WHEN** a fill-first pool's first account runs `agent` and its second `other`, and a stage whose layers resolve `other` starts
- **THEN** it starts on the second account, the first not usable for it (144, 145)

#### Scenario: Nothing usable

- **WHEN** every account of a stage's pool is spent, the soonest until 15:00
- **THEN** the stage waits for room and status gives 15:00 as the pool's soonest reset (145)

### Requirement: A spent account is paused for every flume process

Where an agent's own record says its account reached a usage limit,
flume SHALL record the account spent in the host's own documents until
the reset the record names, or for an hour where it names none, and no
start of any flume process on the host SHALL take it before then;
flume SHALL ask no agent program or its maker whether a limit has reset
(146, 151, 153).

#### Scenario: A reset at 15:00

- **WHEN** an agent's turn fails with its account's usage limit, the record naming a reset at 15:00, and another flume process starts a stage of another place on the same pool before then
- **THEN** that start passes the account over, and a start after 15:00 may take it again (146, S55)

#### Scenario: No reset named

- **WHEN** the record names a usage limit and no reset
- **THEN** the account is spent for an hour from the reading (146)

### Requirement: A spent agent's stage starts again on another account

An agent stopped at its account's usage limit SHALL be ended once
quiet and its stage started on a usable account of the pool, or left
waiting for one, carrying the agent's conversation where its program
can carry it there and afresh otherwise, with flume's own words; the
start SHALL count toward no bound of the kind's and wait on no caller;
agents still running on that account SHALL not be ended for it (31,
112, 146, 179).

#### Scenario: The stage moves

- **WHEN** a code agent under work-1 stops at its usage limit, and work-2 is usable and does not reach its conversation
- **THEN** once quiet the agent is ended, code starts afresh under work-2 with flume's words, status says code moved from work-1 to work-2 because work-1 was spent, that its conversation stayed and why, and that the fresh agent starts without work-1's cache, and the kind's bound is unchanged (S55)

#### Scenario: The stage moves with its conversation

- **WHEN** a code agent under work-1 stops at its usage limit, and work-2 is usable and reaches its conversation
- **THEN** once quiet the agent is ended, code starts under work-2 resuming that conversation, status says code moved from work-1 to work-2 with its conversation and without work-1's cache, and the kind's bound is unchanged (S78)

#### Scenario: Another agent on the spent account

- **WHEN** a second agent works under work-1 when work-1 is read spent
- **THEN** it works on, and moves only if its own turn fails at the limit (31, 146)

#### Scenario: A stage under no account

- **WHEN** an agent with no account stops at a usage limit
- **THEN** its stage is read as before, stuck with the agent's last words, and no account is recorded spent (43, 146)

### Requirement: An intent keeps an account per program

flume SHALL keep each intent's stream and, for each program its agents
run, the account the first of them on that program started on, in the
host's own documents. A start of an agent serving the intent SHALL take
the account for the stage's program while usable, and otherwise go where
the strategy chooses, the affinity moving with it and never back. An
affinity SHALL go an hour after its intent's last place is gone, and
status SHALL say each move and the cache lost (147, 153).

#### Scenario: Three units of one intent

- **WHEN** a caller launches three places of one intent on a least-busy pool, and the first unit's account is spent after all three have started
- **THEN** all three start under the first unit's account while it has room, though another account runs fewer, and the next start goes to another account, the affinity moving and status saying so (S56)

#### Scenario: Never back

- **WHEN** an intent's affinity moved from work-1 to work-2 and work-1's reset passes
- **THEN** the intent's next start stays on work-2 while it is usable (147)

#### Scenario: Dropped an hour after the last place

- **WHEN** an intent's last place is tidied, and a place of the intent is launched 61 minutes later
- **THEN** the affinity is gone and the new place's first start is placed by the strategy, its stream chosen without the intent (147)

#### Scenario: No intent named

- **WHEN** two places on one repository and base are launched naming no intent
- **THEN** they share the intent of that repository and base, and its affinity (147)

### Requirement: An intent moves for one program alone

Where a start of an agent serving an intent goes where the strategy
chooses, the affinity for the stage's program alone SHALL move with it,
and status SHALL say for which program. A start on one program SHALL
leave the intent's account for every other program as it was and SHALL
be no move of the intent (147).

#### Scenario: Two programs of one intent

- **WHEN** a fill-first pool holds agent-1, agent-2, other-1 and other-2, and three places of one intent each run code on `agent` and then review on `other`; then agent-1 is spent
- **THEN** every code starts on agent-1 and every review on other-1, the later ones from the affinity and no move recorded; then the next code starts on agent-2, the intent moving for `agent` alone, and the next review still starts on other-1 (S80)

#### Scenario: A start on another program

- **WHEN** an intent's account for `agent` is agent-1, at its bound, and a review on `other` starts
- **THEN** the review starts on other-1, no move is recorded, and the intent's account for `agent` is still agent-1 (147)

### Requirement: An intent's document written before reads as its account's program's

An intent's document written before, holding one account, SHALL read as
the affinity for that account's program, and with its stream alone where
its pool no longer declares that account (147, 153).

#### Scenario: A document written before

- **WHEN** an intent's document holds one account, agent-2 of the pool `work`, and a move from agent-1 with no program, and a place of the intent starts code on `agent` and then review on `other`
- **THEN** code starts on agent-2 from the affinity, review on the strategy's choice with no move, and the document then holds an account for `agent` and one for `other`, its move naming `agent`, and no lone account (147, 153)

#### Scenario: A document written before whose account is gone

- **WHEN** an intent's document holds one account that its pool no longer declares
- **THEN** it reads with its stream and no account, and the intent's next start takes the strategy's choice (147)

### Requirement: A pool that holds no account of a stage's program refuses it

A launch SHALL be refused, naming the stage, the pool and the program,
with nothing made, where a stage's pool holds no account of the program
its layers resolve. At a later start, read against the host's
configuration as it stands, such a stage SHALL be refused so and start
no agent until a retry, even while it waits for room. A stage whose
pool holds an account of its program, spent or full, SHALL wait for
room (4, 32, 145).

#### Scenario: A launch on a pool of another program's accounts

- **WHEN** a kind's review stage runs `other`, and a place of it is launched on a stream whose pool holds two accounts of `agent`
- **THEN** the launch is refused naming review, the pool and `other`, and nothing is made (4, 145, S81)

#### Scenario: An account dropped before a later start

- **WHEN** a place is launched on a pool holding other-1 for its review on `other`, and the host's configuration drops other-1 from the pool while code works
- **THEN** once code delivers, review is refused naming the pool and `other`, no agent is opened, and status has the place waiting on its caller's retry; a retry once the pool holds other-1 again starts review on it (32, 145, S81)

#### Scenario: A stage waiting when its account is dropped

- **WHEN** review waits for room on a pool whose only account of `other` is spent, the host is at its bound, and the host's configuration drops that account
- **THEN** the next settle refuses review naming the pool and `other`, and opens no agent (32, 145)
