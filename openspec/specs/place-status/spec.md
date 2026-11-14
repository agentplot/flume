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
its caller, the move it waits on and since when, a hold's words among
it, and, for an answer, whether its agent is kept open for it and until
when, or when and why it was let go and whether it is ended (56, 130,
198).

#### Scenario: A place waiting on an answer

- **WHEN** status is asked of a place whose construct ended short needing an answer at a time t, its agent kept open
- **THEN** it gives the move waited on as a retry of construct, since t, the question whole, and its agent kept open until t plus its `keep_open`, in the JSON form and the person's (56, 130)

#### Scenario: Its agent let go

- **WHEN** status is asked of that place once its agent was let go by its time and ended
- **THEN** it gives the same move since t, and its agent let go with when, `time`, and when it was ended (130, 198, S104)

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
recipients; each end with its delivery's key and expiry, its
hand-overs, and per recipient whether it is pending, expired or
delivered, and whether its name is known; for a place waiting on its
caller, the move, since when, a hold's words and whether it is stalled;
a role run's address; each stage's made lines; and the ends pending for
an address the host names no role for (43, 56, 129, 130, 132; weir 104
to 106).

#### Scenario: A stalled place with a pending end

- **WHEN** status is asked of a place stalled on a retry of construct whose end was handed to `ops` by notice and confirmed by it, and is pending for `madswan/planner`
- **THEN** the answer gives the move, since when, stalled, the end's hand-over to `ops` with when, its confirmation by `ops` with when, and pending for `madswan/planner`, a name never confirmed under

#### Scenario: The fields kept

- **WHEN** the suite's field list is read after this change
- **THEN** it holds the recipients, the ends with their deliveries and `pending_for`, each end's `delivery`, `expires_at`, `owed` and `confirmed`, the waiting move, the hold's words, stalled, the role address, the made lines and the host's `expired`, and an answer lacking one fails the suite (61)

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
time it stops being spent, the words in which no reset was read while
it is spent for the hour 146 gives, its reading, and, while it is past
its threshold, the window that holds it and until when; and never an
account's environment's value or a reading's path (56, 144 to 146,
185).

#### Scenario: A pool with a spent account

- **WHEN** status is asked while work-1 runs two agents under a bound of four and work-2 is spent until 15:00
- **THEN** the pool `work` gives fill-first, work-1 with two running and a bound of four, and work-2 spent until 15:00 (56, 146)

#### Scenario: An account spent for an hour

- **WHEN** status is asked while work-3 is spent for an hour because no reset was read in "You've hit your weekly limit · resets Nov 8, 12am (UTC)"
- **THEN** work-3 gives its spent time and `spent_unread` with those words, and the person's form gives them under its line (56, 146)

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

Status SHALL give, with the current agent of a start after a spent one
or of a lost conversation's retry, the start whose conversation it
carries with whether that start stopped at its account's limit or was
lost, and whether the agent carries its conversation or, where it does
not, why; with a stage's move, whether its conversation went with it;
and each carried start that did not begin, with why; and never a
configuration home's path (49, 56, 146, 179; core.30).

#### Scenario: A carried agent

- **WHEN** status is asked of a place whose code start 2 carries the conversation of start 1 under work-2
- **THEN** the agent's conversation gives start 1, spent, and carried, and the move of code from work-1 to work-2 gives carried, in the JSON and the person's form (56, 146, S78)

#### Scenario: A conversation that stayed

- **WHEN** status is asked of a place whose code start 2 started afresh because work-2's `CLAUDE_CONFIG_DIR` does not reach start 1's conversation
- **THEN** the agent's conversation gives start 1, not carried, and why, naming `CLAUDE_CONFIG_DIR` and no path, and the move gives not carried (56, S55; core.30)

#### Scenario: A carried start that did not begin

- **WHEN** status is asked of a place whose carried start 2 ended before it said it started and whose start 3 started afresh
- **THEN** the place lists start 2 as carried and not begun with why, and the agent of start 3 gives start 1, not carried, and why (56, S79)

#### Scenario: A lost talk carried

- **WHEN** status is asked of a place whose talk start 2 carries the conversation of start 1, lost, under work-1
- **THEN** the agent's conversation gives start 1, lost, and carried, and the person's form reads that it carries the conversation of start 1, lost (49, 56, S113)

### Requirement: Status gives where a place's base was read

Status SHALL give, for each place, where its launch read the base's
revision: `merge-queue`, `origin`, `no-origin` or `origin-lacks`, as
`base_from` in the JSON form and beside the base in the person's form,
and none for a place launched before flume said (56, C.1).

#### Scenario: Where the base was read

- **WHEN** status is asked of a place cut at origin's base, and of one cut at the host's own where origin holds none
- **THEN** the JSON form gives `base_from` `origin` and `origin-lacks`, and the person's form `from main (at origin)` and `from trial (the host's own, origin holding none)` (56, C.1)

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

### Requirement: Status gives where a place's kind came from

Status SHALL give a place's kind's `identity` and `kind_from`: its
source, and for a host's source the source's file and the kind's
directory, for the repository's the declaration's file and the base's
revision, for a launch's the declaration's file, and for a package its
hash and the lock that recorded it; `declared` SHALL take `host` for a
host's source, each as the launch read it (19, 21, 56).

#### Scenario: A kind of a host's source

- **WHEN** status is asked of a place of acme/review@1, locked in the host's lock
- **THEN** it gives the identity `acme/review@1`, `declared` `host`, and `kind_from` with the source `acme`, the source's file, the kind's directory, its hash and the host's lock's file, and the person's form gives them on the kind's line (19, 56)

#### Scenario: The repository's, a launch's and flume's own

- **WHEN** status is asked of places of the repository's bump@2, a launch's `--kinds` kind and flume's own change@2
- **THEN** the first gives `repo/bump@2` with `.config/flume.toml` and the base's revision, the second `launch/<name>@<version>` with the file's path, and the third `flume/change@2` with its hash and no lock (19, 56)

#### Scenario: A place launched before this change

- **WHEN** status is asked of a place whose held state was written with no `kind_from`
- **THEN** it gives the source from `declared`, and no identity or hash, and the place settles as before (21, core.14)

### Requirement: Status gives the host's sources

The host's part of status SHALL give each source the host declares, its
file, its `kinds_dir`, the identity of every package that directory
holds, and why the file or the directory cannot be read where either
cannot, each read when asked (56, 159, 191; core.15).

#### Scenario: A source and a fault

- **WHEN** the host's sources directory holds `acme.toml`, whose `kinds_dir` holds `review@1/` and `review@2/`, and `bad.toml` with no `kinds_dir`
- **THEN** status's host part gives `acme` with its file, its `kinds_dir`, acme/review@1 and acme/review@2, and `bad` with the fault naming `kinds_dir` (S96, 56, 159, 191)

### Requirement: Status gives an agent's own settings

Status SHALL give the current agent's own settings, its program's entry
alone, each key with its value as its layer or its file gave it, its
layer and its file where it came from one, as
`programs.<program>.settings.<key>`; each file a layer named for the
program, with the keys left out of it and their owners; how its profile
passed them; and that they were not read back. An agent given none
SHALL be given no own settings, in the JSON form and the person's (56,
195).

#### Scenario: Keys from three layers

- **WHEN** status is asked of a fix in `work` whose agent took `line` from the host's `[agents.streams.work.programs.agent.settings]`, `tips` from the file `[agents.programs.agent]` names and `style` from the repository
- **THEN** its own settings give `programs.agent.settings.line` from `host-stream`, `programs.agent.settings.tips` from `host` with the file, and `programs.agent.settings.style` from `repository`, how they were passed, and that they were not read back (S100, 56)

#### Scenario: Keys left out of a file

- **WHEN** the host's file gave `env` and `perms.mode`, which the fake agent's profile reserves
- **THEN** the agent's own settings name the file with `env` and `perms.mode` left out and their owners, and no value of either (S100, 195)

#### Scenario: Another program's entry

- **WHEN** the host also gives `[agents.programs.other.settings] line = "b"`
- **THEN** the agent's own settings name nothing of `other` (195)

### Requirement: Status gives an account taken for its conversation

Status SHALL give the account of a start that took its lost start's
account for the conversation it carries as from the conversation,
beside the affinity and the strategy, in the JSON and the person's form
(56, 145).

#### Scenario: The conversation's account

- **WHEN** status is asked of a place whose talk start 2 took work-2, the account its lost start 1 ran under, while the intent keeps work-1
- **THEN** the agent's account reads work-2 from the conversation, and the person's form says it is its conversation's (56, 145, S113)

### Requirement: Status names each process running a waiting conversation

Status SHALL give, with a place whose stage waits to carry a
conversation a process runs, each process as live last read it, by its
id and when it started, or what could not be read, and since when the
stage waits; and, with a conversation's stage reported lost whose last
start's pane holds an agent the terminal brought back, that flume left
it there (49, 56, 77, 207; core.30).

#### Scenario: A talk waiting on its session

- **WHEN** status is asked of a place whose lost talk waits while the fake agent reads its session live as process 4242
- **THEN** it gives process 4242 with when it started and since when talk waits, in the JSON and the person's form (49, 56, 207, S117)

#### Scenario: A bare agent left

- **WHEN** status is asked of a place whose talk reads lost while the fake terminal holds its agent brought back bare
- **THEN** the agent gives `left_in_pane`, and the person's form says the terminal brought it back and flume left it in its pane (56, 77, S113)

### Requirement: Status says where each standing address lives

The host's status SHALL give every address a source gives a standing
agent, pending or not, with its agent, stream, session and where that
came from, the stream's terminal, and what the terminal found at the
ask: one agent with its state, none, more than one, or why it could not
read; each terminal and session read once per status, and nothing kept
(56, 132, 190, 193; core.14, core.15).

#### Scenario: An agent found

- **WHEN** the host's status is asked while `madswan/planner`'s standing agent waits in `madswan` and two ends are pending for it
- **THEN** the address gives `madswan-planner`, the stream `madswan` from its address, the session `madswan` from `role`, the agent found waiting, and the two pending ends (56, 132)

#### Scenario: None of the name

- **WHEN** the fake terminal holds no `madswan-planner` in `madswan`
- **THEN** the address's standing says none found, and its pending ends stand beneath (132)

#### Scenario: More than one

- **WHEN** the fake terminal holds two `madswan-planner` in `madswan`
- **THEN** the address's standing says two found (132)

#### Scenario: A part not read

- **WHEN** the fake terminal refuses to read `madswan`'s agents with its words
- **THEN** the address's standing says it could not read, with those words (132)

#### Scenario: Nothing pending

- **WHEN** a source gives `madswan/ops` a standing agent and no end is pending for it
- **THEN** the host's status still gives `madswan/ops` with its standing (56)

#### Scenario: Two addresses in one session

- **WHEN** `madswan/planner` and `madswan/ops` each name an agent in `madswan`, and the host's status is asked
- **THEN** the fake terminal is asked for `madswan`'s agents once (core.15)

#### Scenario: The fields kept

- **WHEN** the suite's field list is read after this change
- **THEN** it holds each address's standing with its agent, stream, session and its `from`, terminal and what was found, and the wake's held answer, and an answer lacking one fails the suite (61; core.14)

### Requirement: Status gives an agent's effort

Status SHALL give the current agent's effort as its layer wrote it,
with the layer it came from, the value its profile passed, and that it
was not read back; an agent whose settings give no effort SHALL be given
none. Where the agent's account's environment names the variable its
program reads over the effort, status SHALL name that variable as
outranking it, never its value, in the JSON form and the person's (56,
197; core.30).

#### Scenario: An effort from the launch

- **WHEN** status is asked of a place whose agent took `xhigh` from `--agent effort=xhigh`
- **THEN** the agent's settings give the effort `xhigh` from `launch`, passed and not read back, in the JSON and the person's form (S103, 56)

#### Scenario: An account's variable that outranks it

- **WHEN** status is asked of a place whose agent took `high` from `host-kind` and whose account's `env` names the variable the fake names as read over its effort
- **THEN** the agent's effort names that variable as outranking it, and no value of it (56, 197)

#### Scenario: No effort

- **WHEN** status is asked of a place whose agent's settings give no effort
- **THEN** the agent's settings give no effort, and the JSON form holds no `effort` for it (197)

### Requirement: Status gives each answer a session took

Status SHALL give, with each stage, every answer its current start took
in its session, in order, with its question, the act it came by, and
when it was sent and received, in the JSON form and the person's (42,
56).

#### Scenario: Two answers in one session

- **WHEN** status is asked of a fix whose code took one answer by a retry and one by a tell in its first start's session
- **THEN** code gives both in order, each with its question, its answer, `retry` or `tell`, and when it was sent and received (42, 56)

### Requirement: Status gives keep_open with its layer

Status SHALL give an agent's `keep_open` with its settings, as written,
with the layer it came from, flume's own one hour named `default`
(32, 56, 198).

#### Scenario: From the host's table for every agent

- **WHEN** the host gives `[agents] keep_open = "20m"` and no other layer gives one
- **THEN** status gives the agent's `keep_open` `20m` from `host` (32, 56)

### Requirement: Status gives the host's watch

Status SHALL give, for the host, the watch a caller keeps: its bound in
the host's `watch`, a bound or none, when its newest settle began as
`watch_settled`, and, while flume runs attended after a watch lapsed,
when it lapsed as `watch_lapsed`. The person's form SHALL say whether a
caller keeps watch, with its bound and when it last settled, or since
when none has (56, 61, 119, 208; core.14).

#### Scenario: A watched host and an unwatched one

- **WHEN** a caller settled every place with a bound of two minutes one minute ago, and status is asked; then five minutes pass with no such settle, and status is asked again
- **THEN** the first gives `watch` with 120 seconds and `watch_settled`, and the person's form prints `a caller keeps watch: every place settles every 2m, last at <time>`; the second gives `watch_lapsed` four minutes after that settle began, and prints `no caller keeps watch since <time>: every place settles at each command of flume's that acts` (119, 208, S118)

### Requirement: Status gives no terminal a watch

Each terminal's answer in status SHALL keep its `watch` field, given
none for every terminal, and nothing of a terminal SHALL decide the
host's `watch` (61, 122, 208; core.14).

#### Scenario: The terminals' answers in status

- **WHEN** a caller keeps watch, the host binds the fake terminal and `trial` names `other`, and status is asked
- **THEN** each terminal's answer gives `watch` none, and the host's `watch` gives the caller's bound (61, 122, 208)

### Requirement: Status gives who answers a place

Status SHALL give each place's recipients, each with where it came
from, a recipient a standing agent's launch gave with that agent and
the source that declares it; the role a stop-short named; each end's
key, and a stop-short's answer with who gave it and when; and a hold on
the waiting move with its words, since when, its bound or that it
lapsed (56, 130, 132, 193).

#### Scenario: Recipients and their sources

- **WHEN** one place names `ops` at its launch and another took its stream's answerer `east/operator`
- **THEN** status gives the first `ops` from the launch and the second `east/operator` from the stream's answerer, declared by the source `acme`, in the JSON form and the person's

#### Scenario: A recipient from a standing agent's launch

- **WHEN** a place was launched naming no recipient from the pane of `east-planner`, which `acme` gives `east/planner`
- **THEN** status gives `east/planner` from the standing agent `east-planner`, declared by `acme`: in the JSON form `recipients_from` with `from` `standing`, `agent` and `source`, and in the person's as `east/planner, from the standing agent east-planner that launched it, declared by acme` (56, 132)

#### Scenario: An answered question

- **WHEN** a construct's stop-short in `east`, naming `ops`, was answered by a retry naming its key from the run of `east/ops`
- **THEN** status gives the end's key, `east/ops` as its addressee, and the answer's words with `east/ops` and when

### Requirement: Status gives the roles a host wakes

Status SHALL give, for each stream, its answerer with the source that
declares it; each source's roles, or why they cannot be read; and for
each address an end stands pending for, its run, or that it is carried
or wakes nothing and why (56, 132, 191, 193).

#### Scenario: An address that wakes nothing

- **WHEN** an end stands pending for `east/planner`, which no source declares, on a place whose stream's answerer is `east/operator`
- **THEN** status gives the address with no run and that no source declares it, and the stream's answerer `east/operator`

### Requirement: Status gives a base place's fixes

Status SHALL give a base place whose kind names a fix its `fix_kind`,
with where it came from; its `fix_limit`; its `forecast`, the newest a
settle held, with when, its head, the base's revision, and clear, in
conflict with its files and entries ahead, or unread with why; and its
`fixes`, each with its number, place, cause, launch time and end: live,
landed with its commit, dropped, or refused with its words. Status SHALL
form no forecast of its own (56, 58, 203; core.15).

#### Scenario: A base place whose first fix is live

- **WHEN** `train--spring`'s kind names `mend` with a bound of 2, a forecast conflict on `README.md` at `c0ffee` launched `mend--spring-1`, which works, and status is asked
- **THEN** `train--spring` gives `fix_kind` `repo/mend@1` from the repository, `fix_limit` 2, its forecast in conflict on `README.md` at `c0ffee` with when it was read, and `fixes` holding fix 1, `mend--spring-1`, its cause the forecast, live; it waits on its places with `on_branch` naming `mend--spring-1`; and the fake merge queue logged no forecast for the status (56, 203)

#### Scenario: A fix landed, in the person's form

- **WHEN** `mend--spring-1` has landed with commit `abc123` and the next forecast was clear, and the person's form of status is asked of `train--spring`
- **THEN** it gives the kind of its fixes with where it came from and `1 of 2 fixes`, `forecast clear` with the base's revision, and fix 1 landed as `abc123`; the JSON gives the same fix with `ended` landed and `commit` `abc123` (56, 61)

### Requirement: Status gives a fix's base place

Status SHALL give a fix flume launched its `fix_of`, naming the base
place, the fix's number and its cause, and its recipients' source as
`base-place`. Every field of a base place's fixes and `fix_of` SHALL be
left out of every place it does not apply to, the JSON form only gaining
fields (56, 61, 203; core.14).

#### Scenario: The fix names its base place

- **WHEN** a forecast conflict launched `mend--spring-1` for `train--spring`, and status is asked of it
- **THEN** it gives `fix_of` naming `train--spring`, fix 1 and the forecast, and its recipients `ops` from `base-place` (43, 56, 203)

#### Scenario: Fields left out of other places

- **WHEN** status is asked of a change no base place launched, and of a base place whose kind names no fix
- **THEN** the change gives no `fix_of`, `fix_kind`, `forecast` or `fixes`, and the base place gives none of them either (61)

### Requirement: Status gives every end expired on the host

Status SHALL give, for the host, each end expired for a recipient, of
every place and every tidied document, with its place, its key, its
delivery's key, the recipient, whether its name is known and when it
expired, reading the host's deliveries once for the ask, and the
person's form SHALL print each (43, 56; weir 104, 106; core.15).

#### Scenario: Two expired ends

- **WHEN** two tidied fixes' landings are pending for `bomdrive` past the host's span, and status is asked of the host
- **THEN** the JSON form's `expired` gives both, each naming `bomdrive`, `known` false, its place and its delivery's key, and the person's form prints a line for each saying `never confirmed`
