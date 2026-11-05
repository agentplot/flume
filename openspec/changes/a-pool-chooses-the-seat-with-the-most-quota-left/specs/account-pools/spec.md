## MODIFIED Requirements

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
else the strategy's: fill-first's first usable account, least-busy's
usable one running fewest of flume's agents, or most-left's (184),
equals in order. Usable SHALL mean not spent, not past its threshold,
under its own bound and of the stage's program. With none usable the
stage SHALL wait for room while its pool holds an account of its
program (145, 147, 185).

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

## ADDED Requirements

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
