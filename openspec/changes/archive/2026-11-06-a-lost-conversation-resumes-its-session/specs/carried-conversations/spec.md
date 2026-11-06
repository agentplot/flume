## RENAMED Requirements

- FROM: `### Requirement: Only the start after a spent agent carries a conversation`
- TO: `### Requirement: Only a start after a spent agent or a lost conversation carries one`

- FROM: `### Requirement: A carried start's prompt is flume's words alone`
- TO: `### Requirement: A carried start's prompt is flume's words and the retry's`

## MODIFIED Requirements

### Requirement: Only a start after a spent agent or a lost conversation carries one

No start but the one that follows an agent stopped at its account's
usage limit, and the one a caller's retry makes of a conversation's
stage reported lost, SHALL carry a conversation, and each SHALL carry
only that agent's. A retry of any other stage, a fix round, a start
after one begun without its settings and every start of another stage
SHALL carry none (30, 49, 146).

#### Scenario: A retry carries nothing

- **WHEN** a caller retries a stage whose agent stopped at its account's limit, before its stage starts again
- **THEN** the start the retry makes resumes no conversation, and its prompt is the stage's with the retry's words (30, 49)

#### Scenario: The next stage carries nothing

- **WHEN** a carried code start delivers and verify starts
- **THEN** verify's agent resumes no conversation (30)

#### Scenario: A lost code stage's retry carries nothing

- **WHEN** the host restarts while a code stage's agent works, the fake terminal brings the agent back bare, and once flume ends it the caller retries code with words
- **THEN** code's fresh agent resumes no conversation, its prompt is code's with the retry's words, and the fake agent's log shows no `carry` (39, 45, 49, S116)

#### Scenario: A stopped conversation's retry carries nothing

- **WHEN** a caller stops a talk stage, a conversation, and then retries it
- **THEN** talk's fresh agent resumes no conversation (49, 51)

### Requirement: A carried start's prompt is flume's words and the retry's

A start that carries a conversation SHALL be sent flume's own words,
then the retry's words where a lost conversation's retry gives any,
then the way to stop short, with nothing of the stage's prompt. After a
spent agent the words SHALL say that its last turn stopped at its
account's usage limit and it goes on under an account of its pool;
after a lost conversation, that its session was lost and goes on here,
and anything it had running then has stopped (28, 49, 146).

#### Scenario: The prompt of a carried start

- **WHEN** a carried code start's agent says it started and waits
- **THEN** its prompt is flume's words and the way to stop short, and holds nothing of the code stage's template (28, 146)

#### Scenario: The prompt of a lost conversation's carried start

- **WHEN** a lost talk's carried start, retried with the words "we were on the second option", says it started and waits
- **THEN** its prompt is flume's words that its session was lost and goes on here, then "we were on the second option", then the way to stop short, and holds nothing of talk's template (28, 49)

### Requirement: A carried start that does not begin starts the stage afresh

A carried start read never begun, or whose agent came up otherwise than
its record, SHALL not be delivered: its agent SHALL be ended once quiet
and the stage started afresh at once, carrying nothing, counting no
bound and waiting on no caller, its words those its cause gives, a spent
agent's or a lost conversation's with the retry's after them, and status
saying why (35, 49, 112, 146, 161).

#### Scenario: No such conversation

- **WHEN** a carried code start's program finds no such conversation and ends before it says it started
- **THEN** no end is delivered, code starts afresh at once with flume's words, status says the conversation could not be carried and why, and the kind's bound is not counted (S79)

#### Scenario: Another session

- **WHEN** a carried start's agent says it started in another session than its record names
- **THEN** its started ack is not taken, its agent is ended once quiet, and the stage starts afresh carrying nothing (161, 179)

#### Scenario: The fresh start after it begins otherwise

- **WHEN** the fresh start after a carried start that ended before it said it started comes up without its account's `CLAUDE_CONFIG_DIR`
- **THEN** it is ended once quiet and the stage starts afresh once more, and a third start that comes up so is never begun (161)

### Requirement: Claude Code carries over a shared projects folder

The Claude Code profile SHALL answer that it can carry where the start
whose conversation it carries, one stopped at its account's limit or a
conversation's start reported lost, has a started ack whose transcript
is the same file by device and inode as the chosen account's
`<home>/projects/<folder>/<file>`, `<home>` its `CLAUDE_CONFIG_DIR`, else
`~/.claude`, and resume it by `--resume <session>` (49, 179, C.4).

#### Scenario: The second account's projects a link to the first's

- **WHEN** the stopped start's transcript is in work-1's home, and work-2's `projects` is a symbolic link to work-1's
- **THEN** carry answers yes with the ack's session, and the next start's command holds `--resume` and that session (179, C.4)

#### Scenario: Two homes of their own

- **WHEN** work-2's `projects` is a folder of its own holding no such transcript
- **THEN** carry answers no, naming `CLAUDE_CONFIG_DIR` and no path (179; core.30)

#### Scenario: A start that never said it started

- **WHEN** the stopped start has no started ack
- **THEN** carry answers no, saying its conversation is not known (179)

#### Scenario: A lost conversation on its own account

- **WHEN** a lost start's transcript is in work-1's home, which shares `projects` with no other, and work-1 is the account asked
- **THEN** carry answers yes with the ack's session (49, 145, 179, C.4)

#### Scenario: A lost conversation whose transcript is gone

- **WHEN** a lost start's started ack names a transcript that no longer exists
- **THEN** carry answers no, saying its transcript cannot be read, naming `CLAUDE_CONFIG_DIR` and no path (49, 179, S115; core.30)

## ADDED Requirements

### Requirement: A retry of a lost conversation carries its conversation

A caller's retry of a stage declared a conversation and reported lost
SHALL start the stage carrying the lost start's conversation, where its
profile answers that its program can carry it to the account the start
takes. Its start SHALL be one fresh agent under its own identity, taken
as the stage's only on its own word that it started in the session its
record names (30, 45, 49, 161, 179).

#### Scenario: A talk lost with its host comes back on its own account

- **WHEN** a research place's talk, a conversation, runs under work-1, the core is rebuilt over the same fake store and the fake terminal holds the agent no longer, and the caller retries talk with the fake agent scripted to carry to work-1 with the session `s-1`
- **THEN** talk starts under work-1 resuming `s-1`, its start's record names `s-1`, its started ack is taken in `s-1`, status gives the agent's conversation from start 1, lost and carried, and the place's starts of talk count two (45, 48, 49, 145, 161, 179, S113)

#### Scenario: A lost talk waits for room and carries when it starts

- **WHEN** a lost talk is retried while the host is at its bound, and an agent of another place ends later
- **THEN** talk waits for room, and the start that takes the room carries `s-1` (49, 70)

#### Scenario: A lost conversation's carried start that does not begin

- **WHEN** a lost talk's carried start, retried with words, ends before it says it started because the fake agent finds no such conversation
- **THEN** no end is delivered, talk starts afresh at once with the stage's prompt holding flume's words that its earlier agent was lost and its conversation could not be carried, then the retry's words, and status says the conversation could not be carried and why (35, 49, 146, S79)

### Requirement: A lost conversation starts afresh where it cannot be carried

Where the profile answers that its program cannot carry a lost
conversation to the account the start takes, or offers no carry, the
stage SHALL start afresh with its prompt as for any retry, flume's words
that its earlier agent was lost and its conversation could not be
carried, and that what it committed and left is in the place, standing
before the retry's words as its words. Status SHALL say why, naming any
variable by its name alone (49, 56, 179; core.30).

#### Scenario: Its transcript gone

- **WHEN** a lost talk is retried, and the fake agent answers for work-1 that its conversation's transcript cannot be read
- **THEN** talk starts afresh under work-1 with flume's words before the retry's, and status gives the conversation from start 1, lost, not carried, with the profile's words (49, 179, S115)

#### Scenario: Another account that does not reach it

- **WHEN** a lost talk's start ran under work-1, work-1 is spent until 15:00 at the retry, and the fake agent answers that work-2 does not reach the conversation
- **THEN** talk starts afresh under work-2 with flume's words before the retry's, status says the conversation stayed and why, naming `CLAUDE_CONFIG_DIR` and no path, and the move of talk from work-1 to work-2 gives not carried (49, 145, 179, S114)

#### Scenario: A lost start that never acked

- **WHEN** a lost talk's start never said it started, and the caller retries it
- **THEN** talk starts afresh, and status says its conversation is not known (35, 49, 179)

#### Scenario: A host with no pool

- **WHEN** a lost talk's start ran under no account, and the fake agent answers for the login environment that it reaches the conversation
- **THEN** talk starts resuming its session under no account, and the fake agent's log shows the `carry` reading for an empty environment (49, 179)

### Requirement: A carried start opens only while no process runs its conversation

A start that carries a conversation, after a spent agent or a lost one,
and a lost conversation's fresh start, SHALL open only once the agent
profile's live answers that no process runs the conversation. While it
answers that one does, or cannot read it, the stage SHALL wait for room
with each process or what could not be read, flume adopting, ending,
signalling and sending nothing to any process, and each settle SHALL ask
again (30, 49, 70, 207).

#### Scenario: A lost talk whose session a person runs by hand

- **WHEN** a lost talk's session is scripted live in the fake agent as process 4242, the caller retries talk, and the place is settled twice
- **THEN** no agent opens, talk waits for room naming process 4242, the place is not stalled, and the fakes' log shows no close, send or open for the place (49, 207, S117)

#### Scenario: The session ends

- **WHEN** the fake agent then reads that session live no more, and any place on the host is settled
- **THEN** talk starts carrying the conversation at that settle (49, 70, 207, S113)

#### Scenario: A session that cannot be read

- **WHEN** a lost talk's session is scripted unknown, naming `CLAUDE_CONFIG_DIR`, and the caller retries talk
- **THEN** talk waits, and status gives what could not be read, naming `CLAUDE_CONFIG_DIR` and no path (207; core.30)

#### Scenario: A spent agent's session resumed by hand

- **WHEN** a code agent stops at work-1's limit and is ended, and before work-2's start a person resumes its session by hand, scripted live
- **THEN** code's carried start does not open while the session is live, and starts at the first settle after it ends (30, 146, 207)

#### Scenario: A lost talk that cannot be carried, its session still live

- **WHEN** a lost talk's transcript cannot be read and its session is scripted live
- **THEN** talk's fresh start waits as the carried one would, and starts afresh once the session ends (49, 207)
