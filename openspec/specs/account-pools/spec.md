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
in order, a strategy, `fill-first`, `least-busy` or `most-left`, and a
threshold, 90 where it states none, and an account a name, a program,
an `env`, an optional bound and an optional reading (29, 144, 184, 185,
C.7).

#### Scenario: The stream's pool before the host's

- **WHEN** `[streams.work] pool = "work"` and `[host] pool = "personal"`, and fix@1 places run in `work` and in `default`
- **THEN** the place in `work` takes accounts of `work` and the place in `default` accounts of `personal` (144)

#### Scenario: No pool anywhere

- **WHEN** the host names no pool
- **THEN** every agent starts under no account, its environment the login environment alone and no variable named in its settings (29, 144)

### Requirement: An account is chosen under the room lock

At each start of an agent with a pool flume SHALL choose, under the
room lock, the intent's account for the stage's program while usable,
else the strategy's: fill-first's first usable one, least-busy's
usable one running fewest of flume's agents, or most-left's, equals
in order. Usable SHALL mean not spent, not past its threshold,
under its own bound and of the stage's program. With none usable the
stage SHALL wait for room while its pool holds an account of its
program (145, 147, 184, 185).

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
the reset the record names, or for an hour where it names none its
profile reads, marking that hour so status gives the words, and no
start of any flume process on the host SHALL take it before then;
flume SHALL ask no agent program or its maker whether a limit has reset
(146, 151, 153).

#### Scenario: A reset at 15:00

- **WHEN** an agent's turn fails with its account's usage limit, the record naming a reset at 15:00, and another flume process starts a stage of another place on the same pool before then
- **THEN** that start passes the account over, a start after 15:00 may take it again, and status gives no words for the pause (146, S55)

#### Scenario: No reset named

- **WHEN** the record names a usage limit and no reset its profile reads
- **THEN** the account is spent for an hour from the reading, its document marks the hour, and status gives the agent's words with the account while the hour stands (56, 146)

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

### Requirement: An account's reading

Where an account names `usage`, flume SHALL read the file it names, its
`~` and `$VAR` read against the login environment, at each choice of
an account and when status is asked, and SHALL write, keep and renew
nothing of it. A reading SHALL be fresh for fifteen minutes from its
`read_at`, and a window whose `resets_at` has passed SHALL read as
nothing used. A file missing or not in C.7's form SHALL be no reading
and SHALL refuse nothing (146, 184).

#### Scenario: A fresh reading

- **WHEN** work-1's reading, read two minutes ago, gives its `5-hour` window 40% used and its `7-day` window 80%
- **THEN** status gives both windows with their resets and the reading fresh, and work-1 has 20 left (184)

#### Scenario: A stale reading

- **WHEN** work-1's reading was read twenty minutes ago and none newer is written
- **THEN** status gives it stale, and a most-left pool ranks work-1 with the accounts that have no reading (184)

#### Scenario: A window past its reset

- **WHEN** work-1's reading gives its `5-hour` window 95% used, resetting an hour ago
- **THEN** the window reads as nothing used, and work-1 is not past its threshold (184, 185)

#### Scenario: A reading that cannot be read

- **WHEN** work-1's `usage` names a file that is missing, then one that is not JSON, then one naming `$GONE`, which the login environment lacks
- **THEN** each time work-1 has no reading, status gives why and no path, and a start on the pool is not refused (184)

#### Scenario: Nothing written

- **WHEN** flume chooses an account of a pool whose accounts give readings, and status is asked
- **THEN** every reading's file is as the host wrote it, and no document of the host's own that flume keeps holds a reading (153, 184)

### Requirement: A most-left pool chooses the account with the most left

Under most-left flume SHALL choose the usable account with the most
left, the least that any window of its fresh reading leaves, then each
usable account with no fresh reading, equals in either by fewest of
flume's agents running and then in the pool's order (145, 184).

#### Scenario: Four places of four intents

- **WHEN** a most-left pool holds work-1, reading 40% and 80% used two minutes ago, work-2, reading 60% and 30%, and work-3, naming no reading, and four places of four intents start in turn: the second while work-2's reading is unchanged, the third once it is twenty minutes old, the fourth once work-1 is spent
- **THEN** the first and second start on work-2, the third on work-1, and the fourth on work-3, which runs fewer of flume's agents than work-2 (S86)

#### Scenario: No reading anywhere

- **WHEN** a most-left pool's two accounts give no reading, and three stages of three intents start
- **THEN** they alternate first, second, first, as least-busy's do (145, S54)

#### Scenario: The affinity first

- **WHEN** an intent's affinity for `agent` is work-1, usable, and work-2's reading leaves more
- **THEN** the intent's next start on `agent` stays on work-1, from the affinity (147)

### Requirement: An account past its threshold takes no new start

An account whose reading gives a window used to its pool's threshold
or past it SHALL take no new start under any strategy until that window
resets, however old the reading, or, where the window names no reset,
while the reading is fresh. Its running agents SHALL go on, neither
ended nor moved for it, and an intent's affinity SHALL move off it,
saying which window and until when (31, 147, 185).

#### Scenario: Past ninety percent

- **WHEN** a fill-first pool's work-1 runs two of flume's agents and its reading gives its `5-hour` window 92% used, resetting at 17:00, its pool stating no threshold, and an intent whose affinity is work-1 starts its next stage
- **THEN** the two agents work on and neither is ended or moved, the stage starts on work-2, and status says the intent moved because work-1 was past its threshold in `5-hour` until 17:00; after 17:00, no newer reading written, a place of another intent starts on work-1, and the intent that moved stays on work-2 (S87)

#### Scenario: An old reading holds

- **WHEN** work-1's reading, read three hours ago, gives its `7-day` window 95% used, resetting Monday at 08:00
- **THEN** work-1 takes no start until Monday at 08:00 (185)

#### Scenario: A window with no reset

- **WHEN** work-1's reading gives a window 95% used and no `resets_at`
- **THEN** work-1 takes no start while the reading is fresh, and takes one again once it is fifteen minutes old with none newer (185)

#### Scenario: Under least-busy

- **WHEN** a least-busy pool's work-1 runs no agent and is past its threshold, and work-2 runs two
- **THEN** the start goes to work-2 (145, 185)

#### Scenario: A threshold the pool states

- **WHEN** a pool states `threshold = 100`, and work-1's reading gives a window 99% used; then 100%
- **THEN** work-1 takes the first start and not the second (185)

### Requirement: The soonest time counts a threshold's end

Where no account of a stage's pool is usable, status SHALL give as the
pool's soonest time the earliest at which an account of the stage's
program is neither spent nor past its threshold (145, 185).

#### Scenario: Every account past its threshold or spent

- **WHEN** a most-left pool's work-1 is past its threshold in `7-day` until Monday at 08:00, and work-2 is spent until 15:00 though its reading gives it 50% used
- **THEN** the stage waits for room on the host and not on its caller, status gives 15:00 as the pool's soonest time, and the first settle after 15:00 starts it on work-2 (S88)

### Requirement: A reading leaves a spent pause as it stands

A spent account SHALL stay paused until its reset whatever its reading
says, and flume SHALL record no account spent from a reading (146, 185).

#### Scenario: A reading under the threshold while spent

- **WHEN** work-2 is spent until 15:00, and its reading written at 14:00 gives it 10% used
- **THEN** work-2 takes no start before 15:00 (146, S88)

#### Scenario: A reading at its limit

- **WHEN** work-1's reading gives a window 100% used, and no agent's own record said its account reached a usage limit
- **THEN** no spent time is written for work-1, and work-1 takes no start by its threshold (146, 153, 185)
