# place-status Specification

## Purpose
Status of one place or every place on the host, each fact read where it
lives when asked, and every answer of flume's in one JSON form a program
reads, from one core, that only ever gains fields (43, 56 to 62, 69).

## Requirements

### Requirement: Status gives every fact of a place

Status SHALL give, for one place or every place, what clause 56 lists of
it: the place's repository, kind at its version with its declaration,
name, branch, base, mark, priority and recipients; each stage's state,
end and start count; whether the deliverable exists; the submission, how
often flume submitted and how often the place went back; the change's
tasks; refused acts; and a refused stage's part, the host's part being
the host queue's (56, 70).

#### Scenario: A stage's end with what it delivered

- **WHEN** status is asked of a place whose code stage delivered
- **THEN** the stage's end carries the head it delivered at, and a stage that ended short carries the agent's words and its kind (43, 56)

### Requirement: Status gives a place's current agent

Status SHALL give a place's current agent with its identity, its
settings each with the layer it came from and its variables each from
its account, each value as written but a variable's, which status names
with the value `<set>` and never gives, its state and whether it works
past its limit, and the command to run under a terminal that prints it,
which names the file holding its environment and holds no value of it
(32, 56; records 0017, 0020).

#### Scenario: Each setting with its layer

- **WHEN** status is asked of a change whose code agent took its model from the repository and `CLAUDE_CONFIG_DIR = "~/.claude-work"` from its account
- **THEN** the agent's settings give the model from `repository` and the variable `CLAUDE_CONFIG_DIR` from `account` as `<set>` (32, 56, S44)

#### Scenario: A token in no answer and no record

- **WHEN** the stage's account gives `CLAUDE_CODE_OAUTH_TOKEN`, and the launch, status as text and as JSON, and a settle are asked
- **THEN** no answer holds the token's value, and it is in no file of flume's state but the printed command's environment file, which the host's user alone may read (32, 56)

### Requirement: Status says what a waiting place waits on

Status SHALL give a stop-short's words and whether it needs an answer or
found work, or a failed submission's words, and, for a place waiting on
its caller, the move it waits on and since when, a hold's words among it
(56, 130).

#### Scenario: A place waiting on an answer

- **WHEN** status is asked of a place whose construct ended short needing an answer
- **THEN** it gives the move waited on as a retry of construct, since when, and the question whole (56, 130)

### Requirement: Status reads where each fact lives and writes nothing

Status SHALL read every fact of a deliverable, a change or a submission
from its seam when asked, at one head per place, SHALL keep none of
them between asks, and SHALL start no agent, send nothing and write
nothing (57, 58).

#### Scenario: A change ticked between two asks

- **WHEN** status is asked, the fake change then ticks a task at a new head, and status is asked again
- **THEN** the second answer gives the new count, and neither ask wrote to the fake store or called the fake terminal's open or send

### Requirement: One answer from one core

Every act's answer SHALL come from the one core, the same for the same
question about the same places whoever asks, and no surface SHALL
compute a stage's end, a status, a submission or a tidy of its own (60,
69).

#### Scenario: Asked three times

- **WHEN** status of one place is asked three times over the same fakes
- **THEN** the three answers are equal and none starts or tells an agent (S14)

### Requirement: One JSON form that only gains fields

Every answer SHALL serialize to one JSON form, the same through every
surface, and the suite SHALL hold each answer's field list so that an
answer losing a field fails the suite (61).

#### Scenario: A field removed

- **WHEN** an answer's form drops a field the suite's list holds
- **THEN** the suite fails naming the answer and the field

#### Scenario: A refusal is an answer

- **WHEN** a launch is refused
- **THEN** its answer is the same JSON form with the refusal and its reason, not an error of another shape (2, 61)

### Requirement: Status gives what waits, what was made and what is pending

Status SHALL give, beside clause 56's other facts, each place's
recipients; each end with whom it was delivered to and when, and for
whom it stands pending; for a place waiting on its caller, the move,
since when, a hold's words and whether it is stalled; a role run's
address; each stage's made lines as written; and the ends pending for
an address the host names no role for (43, 56, 129, 130, 132).

#### Scenario: A stalled place with a pending end

- **WHEN** status is asked of a place stalled on a retry of construct whose end is delivered to `ops` and pending for `madswan/planner`
- **THEN** the answer gives the move, since when, stalled, the end delivered to `ops` with when, and pending for `madswan/planner`

#### Scenario: The fields kept

- **WHEN** the suite's field list is read after this change
- **THEN** it holds the recipients, the ends with their deliveries, the waiting move, the hold's words, stalled, the role address and the made lines, and an answer lacking one fails the suite (61)

### Requirement: Status gives a repeating stage's group

Status SHALL give a repeating stage with its current group's place among
the change's groups at the head and how many there are, with the group's
number and title, read from the change when asked and never from the
notes; the person's form SHALL print it as `code 3/7` (56, 58, 142).

#### Scenario: Status mid-stage

- **WHEN** status is asked while a repeating code stage's repetition works on the second of three groups
- **THEN** the JSON form gives the group's place 2 of 3 with its number and title, and the person's form prints `code 2/3`

#### Scenario: A group added mid-stage

- **WHEN** a repetition adds a fourth group to a change of three and status is asked
- **THEN** status gives the count as 4, read at the head

### Requirement: Status gives a place's stream, intent, account and moves

Status SHALL give a place's stream and intent, each with where it came
from, its current agent's account with whether it came from the
intent's affinity for the agent's program or the strategy, each move of
a stage or the intent between accounts with from, to, why, when and
that the cache was lost, an intent's naming the program whose account
moved, and each start that began without its settings, naming its
variables alone (56, 143, 145 to 147, 161).

#### Scenario: A stage that moved

- **WHEN** status is asked of a place whose code stage moved from work-1 to work-2 when work-1 was spent
- **THEN** the agent's account reads work-2 from the strategy, and the place's moves give code, work-1, work-2, spent, when, and the cache lost, in the JSON and the person's form (56, 146)

#### Scenario: An intent that moved for one program

- **WHEN** status is asked of a place whose code started on agent-2 after the intent moved for `agent` from agent-1, spent, and whose review started on other-1 from the affinity
- **THEN** the place's moves give the intent's move with the program `agent`, and the person's form reads that the intent moved for `agent` from agent-1 to agent-2; the review's agent reads other-1 from the affinity, and its line says from the affinity for `other` (56, 147, S80)

#### Scenario: A start without its settings

- **WHEN** status is asked of a place whose code start began without `CLAUDE_CONFIG_DIR`
- **THEN** it names the start and `CLAUDE_CONFIG_DIR` as missing, and no value (56, 161)

### Requirement: Status gives the host's pools

Status SHALL give, for the host, each pool with its strategy and its
threshold, and each account with its agents running, its bound, the
time it stops being spent, its reading, and, while it is past its
threshold, the window that holds it and until when; and never an
account's environment's value or a reading's path (56, 144 to 146,
185).

#### Scenario: A pool with a spent account

- **WHEN** status is asked while work-1 runs two agents under a bound of four and work-2 is spent until 15:00
- **THEN** the pool `work` gives fill-first, work-1 with two running and a bound of four, and work-2 spent until 15:00 (56, 146)

#### Scenario: A pool with readings

- **WHEN** status is asked of a most-left pool stating no threshold, whose work-1 reads `5-hour` 42% used resetting at 17:00, read two minutes ago, whose work-2's reading is twenty minutes old, and whose work-3's file is missing
- **THEN** the pool gives most-left and a threshold of 90, work-1 its window with 42, 17:00 and fresh, work-2 its reading stale, and work-3 no reading with why, in the JSON form and the person's, and neither gives a path (56, 184, 185)

#### Scenario: An account past its threshold

- **WHEN** status is asked while work-1's reading gives `7-day` 95% used, resetting Monday at 08:00
- **THEN** work-1 gives `past` with `7-day` and Monday at 08:00, and its window `past` (56, 185)

### Requirement: Status gives an agent's mode

Status SHALL give the current agent's mode as its layer wrote it, with
the layer it came from, the program whose entry of a table of programs
the agent took, the value its profile passed, and whether its profile
read the mode back; an agent whose settings give no mode SHALL be given
none. A start that began in another mode, and a never-begun end of
one, SHALL name the mode asked and the mode it began in, in the JSON
form and the person's (56, 161, 162, 169).

#### Scenario: A mode from the host's table

- **WHEN** status is asked of a fix whose agent took `edits` from the host's `[kinds.fix.agents]`
- **THEN** the agent's settings give the mode `edits` from `host-kind`, the value its profile passed and that it was read back, in the JSON and the person's form (S66, 56)

#### Scenario: A table's entry

- **WHEN** status is asked of a place whose agent took the `agent` entry of `{ agent = "careful", other = "edits" }` from `launch`
- **THEN** the agent's mode reads the table as written from `launch`, the entry `agent` and the value `careful` passed (56, 169)

#### Scenario: A mode not read back

- **WHEN** status is asked of a place whose agent's profile reads no mode back
- **THEN** the agent's mode says it was not read back (169)

#### Scenario: A start in another mode

- **WHEN** status is asked of a place whose code start began in `ask-each` where its record named `all`
- **THEN** it names the start, the mode asked `all` and the mode begun `ask-each`, and no variable (56, 161)

### Requirement: Status gives a start's conversation

Status SHALL give, with the current agent of a start after a spent one,
the stopped start's number and whether the agent carries its
conversation or, where it does not, why; with a stage's move, whether
its conversation went with it; and each carried start that did not
begin, with why; and never a configuration home's path (56, 146, 179;
core.30).

#### Scenario: A carried agent

- **WHEN** status is asked of a place whose code start 2 carries the conversation of start 1 under work-2
- **THEN** the agent's conversation gives start 1 and carried, and the move of code from work-1 to work-2 gives carried, in the JSON and the person's form (56, 146, S78)

#### Scenario: A conversation that stayed

- **WHEN** status is asked of a place whose code start 2 started afresh because work-2's `CLAUDE_CONFIG_DIR` does not reach start 1's conversation
- **THEN** the agent's conversation gives start 1, not carried, and why, naming `CLAUDE_CONFIG_DIR` and no path, and the move gives not carried (56, S55; core.30)

#### Scenario: A carried start that did not begin

- **WHEN** status is asked of a place whose carried start 2 ended before it said it started and whose start 3 started afresh
- **THEN** the place lists start 2 as carried and not begun with why, and the agent of start 3 gives start 1, not carried, and why (56, S79)

### Requirement: Status gives every place's line

Status SHALL give, for every place it gives, the place's line with each of its parts, its full form and its short form, made from the facts status read, in its JSON form, and the person's form SHALL print the line's full form under the place's heading (56, 61; core.14).

#### Scenario: A line in the JSON and the person's form

- **WHEN** status is asked of a fix place whose code agent works
- **THEN** its JSON gives `line` with `name`, `kind`, `stage`, `run`, `tasks`, `state`, `full` and `short`, and the person's form prints `fix--<name> · fix@1 · code · run 1 · working` under the place's heading (56, 61)

#### Scenario: The field list holds the line

- **WHEN** the place's answer's form drops `line` or one of its fields
- **THEN** the suite fails naming the answer and the field (61)

### Requirement: Status gives what the host's terminal lacks for flume's plugin

Status SHALL give, for the host, what the terminal's own configuration lacks for flume's plugin, each in the terminal profile's words, and the person's form SHALL print each under the line naming the watch; flume SHALL say it to no person but when asked (72, 122).

#### Scenario: A popup with no key

- **WHEN** the fake terminal says its configuration binds no key to flume's popup, and status is asked of one place
- **THEN** the host's `terminal_setup` gives those words, the person's form prints them under the watch's line, and nothing was sent to any pane (72)

### Requirement: Status says a move off an account past its threshold

A move of a stage or an intent off an account past its threshold SHALL
give `why` `threshold`, with the window that held the account and when
that window resets; a move recorded before SHALL read with neither (56,
147, 185; core.14).

#### Scenario: The intent's move past a threshold

- **WHEN** an intent's affinity for `agent` moves from work-1, past its threshold in `5-hour` until 17:00, to work-2
- **THEN** the place's moves give the intent's move for `agent` from work-1 to work-2 with `why` `threshold`, `window` `5-hour` and `until` 17:00, and the person's form says work-1 was past its threshold in `5-hour` until 17:00 (56, 147, 185)

#### Scenario: A move recorded before

- **WHEN** a held state holds a move with `why` `spent` and no `window` or `until`, as an earlier release wrote it
- **THEN** status gives it as written, with no window and no end (56; core.14)
