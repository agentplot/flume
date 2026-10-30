## ADDED Requirements

### Requirement: A stage's pool

A stage's pool SHALL be the host's for its stream and kind, else its
stream's, else its kind's for every stream, else the host's own; a
stage with none SHALL run under no account, as the other layers give
it. A pool SHALL name its accounts in order and a strategy,
`fill-first` or `least-busy`, and an account a name, a program, an
`env` and an optional bound (144, C.7).

#### Scenario: The stream's kind before the stream

- **WHEN** `[streams.work] pool = "work"` and `[streams.work.kinds.chore] pool = "cheap"`, and chore@1 and fix@1 places run in `work`
- **THEN** chore's agents take accounts of `cheap` and fix's of `work` (144)

#### Scenario: No pool anywhere

- **WHEN** the host names no pool and gives `[kinds.change.agents] env = { CLAUDE_CONFIG_DIR = "~/.claude-work" }`
- **THEN** change's agents start under that variable from `host-kind` and no account, as under record 0017 (144)

### Requirement: An account is chosen under the room lock

At each start of an agent with a pool flume SHALL choose, under the room
lock, the intent's affinity account while usable, else under fill-first
the first usable account in order, under least-busy the usable one with
fewest of flume's agents, equals in order. Usable SHALL mean not spent,
under its own bound and running the stage's program. With none usable
the stage SHALL wait for room (145).

#### Scenario: Fill-first and least-busy

- **WHEN** a stream's pool is fill-first over two accounts, the first bounded at two agents, and three stages start; then the same under least-busy
- **THEN** fill-first starts two under the first account and one under the second, and least-busy alternates first, second, first (S54)

#### Scenario: Two starts and one account's last room

- **WHEN** two settles at once each start a stage on a pool whose only account has one agent's room left
- **THEN** exactly one stage starts on it and the other waits for room (145)

#### Scenario: An account for another program

- **WHEN** a pool's only account runs `claude` and a stage's other layers resolve another program
- **THEN** the stage waits for room, and status says no account of the pool runs that program (144, 145)

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

### Requirement: A spent agent's stage starts afresh on another account

An agent stopped at its account's usage limit SHALL be ended once
quiet and its stage started afresh on a usable account of the pool, or
left waiting for one, with flume's own words saying why and that its
work is in the place; the start SHALL count toward no bound of the
kind's and wait on no caller; agents still running on that account
SHALL not be ended for it (31, 112, 146).

#### Scenario: The stage moves

- **WHEN** a code agent under work-1 stops at its usage limit and work-2 is usable
- **THEN** once quiet the agent is ended, code starts afresh under work-2 with flume's words, status says code moved from work-1 to work-2 because work-1 was spent and that the fresh agent starts without work-1's cache, and the kind's bound is unchanged (S55)

#### Scenario: Another agent on the spent account

- **WHEN** a second agent works under work-1 when work-1 is read spent
- **THEN** it works on, and moves only if its own turn fails at the limit (31, 146)

#### Scenario: A stage under no account

- **WHEN** an agent with no account stops at a usage limit
- **THEN** its stage is read as before, stuck with the agent's last words, and no account is recorded spent (43, 146)

### Requirement: An intent keeps to its account

flume SHALL keep each intent's stream and account in the host's own
documents; a start of an agent serving it SHALL take that account while
usable, and otherwise go where the strategy chooses, the affinity moving
with it and never back. An affinity SHALL go an hour after its intent's
last place is gone, and status SHALL say each move and the cache lost
(147, 153).

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
