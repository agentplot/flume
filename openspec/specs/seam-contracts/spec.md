# seam-contracts Specification

## Purpose
The seven seams of Part B as one interface each, with the types they
pass, so every profile of Part C codes against them and the core reaches
no tool but through them (core.1, core.2).

## Requirements

### Requirement: One interface per seam

There SHALL be exactly one interface for each of the eight seams of
Part B: the place (78 to 83), the kinds (84, 85), the change (86, 87,
127), the agent (88 to 93, 115, 116, 121, 134, 135), the terminal (94
to 98, 122, 136 to 139, 187), the merge queue (99, 101, 117, 125, 126,
133), the store (102 to 104, 128) and the window (188, 189). Each SHALL
offer the operations its section names and no operation of the core's,
and no interface SHALL name a tool (core.1, core.2).

#### Scenario: Each seam's operations

- **WHEN** each seam's scenarios run against its fake
- **THEN** each operation its section of Part B names is exercised, and each scenario cites the clause of its operation

#### Scenario: No tool named outside a profile

- **WHEN** the seams', core's, fakes', suite's and commands' sources are searched for `wt`, `worktrunk`, `openspec`, `claude`, `herdr`, `sluice`, `ghostty` and `osascript` as words
- **THEN** none is found outside a `profiles` directory

### Requirement: The merge queue's submit holds no wait

The merge queue's submit SHALL answer once the merge queue holds the
submission, with no wait on its verdict, and its inbox SHALL give each
pending outcome of the place once (117, 133).

#### Scenario: Submit holds no wait

- **WHEN** the fake merge queue holds a submission undecided
- **THEN** submit answers with the submission at once, and a later inbox gives its outcome once and then nothing (117, 133)

### Requirement: What the agent and terminal seams read

The agent seam SHALL say whether the agent's record shows its session
ended (134) and whether the agent has said it started under its start's
identity (135); the terminal seam SHALL name the session a command runs
in, for a launch's default recipient (7), and give what it shows of an
agent (136).

#### Scenario: Started and screen

- **WHEN** the fake agent holds no started word for a start, and the fake terminal shows lines for its agent
- **THEN** `started` reads no and `screen` gives those lines (135, 136)

### Requirement: The terminal seam's open, tidy and show

The terminal seam's open SHALL take where the agent runs, beside a
pane, in a session by its name or in the terminal's own, and SHALL
answer, beside opened and refused, a start held at a question
its program asks before it starts and a program that did not come up
within its bound, each with the screen (94, 107); it SHALL tidy what it
made for a place gone (138) and show a place's clues (139); and every
terminal operation SHALL answer within a bound the profile states (98).

#### Scenario: Held, tidied and shown

- **WHEN** the fake terminal holds a start at a question, makes a place's tab in its base's workspace, and has a person's pane in that workspace
- **THEN** open answers held with the screen, tidy closes the place's tab and keeps the workspace naming the person's pane, and show takes the place's clues (94, 138, 139)

#### Scenario: Where an agent opens

- **WHEN** the fake terminal is asked to open one agent beside the pane `operator`, one in the session `work` and one in its own
- **THEN** its log names each where it was asked, and the open's answer is the same for each (94, 107)

### Requirement: The base, a change's location and a place's folder

The merge queue seam SHALL answer the revision the base stands at where
its profile says the base lives, after that profile's fetch, or a
refusal naming why, or that the merge queue lands no such base, which
the core then reads at origin (126; 9, C.1, core.7); the change seam
where a change lies in a place, for a prompt's change part (127); and
the store the place's own folder beside its held state, for a report
kept there (128, 123).

#### Scenario: The base where the merge queue says it lives

- **WHEN** the fake merge queue says the base lives at a revision other than the local base's
- **THEN** the place is cut at that revision

#### Scenario: No base revision readable

- **WHEN** the fake merge queue refuses to give the base's revision, naming a failed fetch
- **THEN** the launch is refused naming the fetch, and nothing is made

#### Scenario: A base the merge queue lands not

- **WHEN** the fake merge queue is asked for a base it lands nothing on
- **THEN** it answers that it lands no such base, and the place's own base at a revision (126)

### Requirement: The place seam says where a place is made

The place seam SHALL answer where a place of a repository, named
`<kind>--<name>`, is made, the same at every ask, its own, and outside
the repository's directory, or why it cannot say; a launch SHALL make
its place there, its id that location, and a launch whose location
cannot be found SHALL be refused naming why; a place made where the
seam no longer gives SHALL be read, submitted and tidied where it
stands (2, 3, 8, 76, 78, C.1; record 0027).

#### Scenario: The launch makes its place where the seam says

- **WHEN** a fix is launched over the fake place, which gives `/host/data/flume/places/<an id of the repository>/<kind>--<name>`
- **THEN** the place is made at that location, and its held state's id is that location

#### Scenario: A location outside the repository's directory

- **WHEN** the place seam is asked where a place of a repository goes
- **THEN** the location lies outside the repository's directory

#### Scenario: A place made where the seam no longer gives

- **WHEN** a fix is launched over the fake place giving `<repository>/places/fix--one`, the fake then gives its data directory's, a fix of the same name is launched, and the first's code delivers
- **THEN** the second launch is refused naming `fix/one`, and the first place is submitted, lands and is removed at `<repository>/places/fix--one`

### Requirement: The types the seams pass

The seams SHALL pass one type for each of: a launch with its priority,
its recipients and each hold's words (2, 3, 7, 8, 70, 130); a place and
its held state, its priority and each stage's wait for room among it
(62, 70, 73 to 76, 102 to 104); a kind with its stages as read-kinds
returns them (18, 23); a deliverable in one of the six forms (24); and a
prompt and its parts (27, 28). Each SHALL serialize to JSON and back
unchanged.

#### Scenario: A type round-trips

- **WHEN** a value of each type is written to JSON and read back
- **THEN** it equals the value written

#### Scenario: A kind with two agents or two deliverables

- **WHEN** a kind's stage carries two agents or two deliverables as read-kinds returns them
- **THEN** the type keeps both, so the core can refuse the kind (4, 23)

### Requirement: The types of an agent, the terminal and a submission

The seams SHALL pass one type for each of: an agent's identity, its
start's secret and a stop-short that needs an answer or found work (29,
30, 33, 41, 92); an agent's command with the program's folder-trust
question, its answer and the program's kind (34, 88, C.4); the
terminal's clues and what its tidy closed and kept (138, 139); and a
submission with its outcomes, landed, failed, in conflict or withdrawn
(99, 111 to 113, 117, 125, 133). Each SHALL serialize to JSON and back
unchanged.

#### Scenario: An agent's command and a withdrawn submission round-trip

- **WHEN** an agent's command carrying its folder-trust question, its answer and its kind, and a submission withdrawn, are each written to JSON and read back
- **THEN** each equals the value written (34, 88, 125)

### Requirement: Git's history and the host's facts beside the seams

The core SHALL read git's history and the host's facts through two
interfaces of their own, which are no seam of Part B (section 9): a
branch's head, whether a head holds commits another does not, the
newest commit at a head touching a path, whether a branch name is
valid; and the host's clock, a new secret, a file's write time, whether
a command of flume's is still live, the bound on agents the host's
configuration states, and a host-wide lock over the room (70).

#### Scenario: A commit since the stage started

- **WHEN** the fake history holds a head that reaches a commit the stage's start head does not
- **THEN** the history answers that the head holds a commit since the start (25)

#### Scenario: An agent reset to an older commit

- **WHEN** the head is moved back to an ancestor of the start head
- **THEN** the history answers that the head holds no commit since the start

#### Scenario: The fake host's bound

- **WHEN** a test sets the fake host's bound to two
- **THEN** the core reads a bound of two, and none where the test sets none

### Requirement: The terminal reaches a recipient's session by its name

The terminal seam SHALL send a text to, and read the state of, a
recipient's session named as the terminal names sessions, the same
guarantee as a send to an agent: delivered as input or refused, never
left typed; and a recipient the terminal cannot find SHALL be answered
as not found, never guessed (43, 95, 96, 98).

#### Scenario: A session by its name

- **WHEN** the terminal scenarios send a notice to a recipient's session by its name, then to a name no session holds
- **THEN** the first is sent and the session's state is read, and the second is answered not found with nothing sent

### Requirement: An agent opened under a session name

The terminal seam's open SHALL take the session name the agent is to
carry, a role run's address where the place is one, and a send or a
state read by that name SHALL reach that agent (94, 107, 132).

#### Scenario: A role run's agent

- **WHEN** an agent is opened under the session name `madswan/planner`
- **THEN** a send to the recipient `madswan/planner` reaches that agent, and a second open under that name while it is live is refused (98)

### Requirement: The agent seam reads a spent account and a start without its settings

The agent seam SHALL say whether the agent's own record shows its
account reached a usage limit, with the reset it names, or unknown
(151), whether its agent began without the settings its start
recorded, naming each variable that differs, whether its program did,
and, where its profile reads the mode back, the mode asked and the mode
begun (135, 161, 162), and which variables its program keeps from every
process it starts, none by default (161).

#### Scenario: Spent and without its settings over the fake

- **WHEN** the fake agent scripts a usage limit with a reset for one start and a missing variable for another
- **THEN** `spent` reads yes with that reset for the first and `started_without` names the variable for the second (135, 151, 161)

#### Scenario: Begun without its settings and no turn taken

- **WHEN** an agent's word at its start shows a variable missing and it has taken no turn
- **THEN** `started_without` names the variable, and `own_work` and `asking` read no (31, 91, 161)

#### Scenario: Begun in another mode over the fake

- **WHEN** the fake agent scripts a start that began in `ask-each` where its record named `all`
- **THEN** `started_without` names the mode asked `all` and the mode begun `ask-each`, and no variable (135, 161, 162)

### Requirement: The store keeps the host's own documents

The store seam SHALL read and write the host's own documents, each
account's spent time and each intent's affinity, by key, compare-and-set
on a version as a place's held state is, and reached by every flume
process on the host (102 to 104, 153).

#### Scenario: A host document written at a moved version

- **WHEN** two writers write one key at the version both read
- **THEN** one write lands and the other is refused as moved (103, 153)

### Requirement: The types of streams, pools and a start's record

The seams SHALL pass one type each for a stream, a pool, its strategy
and its accounts, an account chosen and where from, a move between
accounts, an intent's naming the program whose account moved, an
intent's affinity with an account for each program, and a start's record
of what its agent runs under, held as the start's `start_record` and
holding digests and no value; each SHALL serialize to JSON and back
unchanged (143 to 147, 153, 161).

#### Scenario: A held state with a stream, an account and a record

- **WHEN** a held state whose place has a stream with `from`, an intent, and a start with an account and a start's record is written and read
- **THEN** it reads back unchanged, and the record holds each variable's digest and no value (143 to 147, 161)

#### Scenario: An affinity of two programs

- **WHEN** an affinity holding an account for `agent` and one for `other`, and an intent's move for `agent`, is written and read
- **THEN** it reads back unchanged, and it once left the move's `from` for `agent` and never for `other` (147, 153)

### Requirement: The agent seam maps a mode and says whether it reads one back

The agent seam SHALL map an agent's entry of a mode, one of flume's
four or a value of its program's own, to the value of its program's
that it passes, or refuse it with words naming the mode and the
program, refusing every entry by default; SHALL say whether its
program's word at its start, its word that it started or at its start's
first prompt, reads back the mode it began in, not by default; and its
command SHALL pass the value mapped, and none where
the settings give no mode (88, 161, 169).

#### Scenario: A profile that maps no mode

- **WHEN** an agent profile that keeps the seam's defaults is asked to map `edits`
- **THEN** it refuses with words naming `edits`, and says it reads no mode back (169)

#### Scenario: The fake's map over the seam's scenarios

- **WHEN** the agent seam's scenarios run against the fake agent scripted with a map of the four and the own value `careful`
- **THEN** each of the four and `careful` maps as scripted, any other own value is refused naming it, and the fake's command passes the value mapped (88, 169)

### Requirement: The types of a mode

The seams SHALL pass one type each for flume's four modes, a mode as a
layer gives it, one of the four or a table of programs, an agent's
entry of it, a start's record of its mode, and a mode a start began in
beside the mode asked; each SHALL serialize to JSON and back unchanged,
a held state written before the mode SHALL read with none, and a bare
word that is not one of the four SHALL be refused naming it (161, 162,
169).

#### Scenario: A held state with a mode

- **WHEN** a held state whose launch gives a table of programs, whose kind gives `edits`, and whose start records a mode and began in another is written and read
- **THEN** it reads back unchanged, and the same state written without any mode reads with none (161, 162)

#### Scenario: A bare word of a program's own

- **WHEN** a mode is read from the JSON word `"auto"`
- **THEN** it is refused naming `auto` and saying a program's own value goes under its program's name (169)

### Requirement: The agent seam carries a conversation or says why not

The agent seam SHALL answer, given the start whose conversation a start
carries, one whose agent stopped at its account's usage limit or a
conversation's start reported lost, and the environment of the account
the next start takes, an empty one under no pool, whether its program
can carry that agent's conversation there, with the session to resume,
or why not, or that it offers no carry, the default; and its command
SHALL take the session a start resumes (49, 88, 179).

#### Scenario: A profile that offers no carry

- **WHEN** the seam's scenarios run against a profile that implements no carry
- **THEN** carry answers that it is not offered, for any start and account (179)

#### Scenario: The fake's carry over the seam's scenarios

- **WHEN** the seam's scenarios run against the fake agent scripted to carry to one account and not to another
- **THEN** carry answers yes with the scripted session for the first, and no with the scripted words for the second (179)

### Requirement: The types of a carried start

The seams SHALL pass one type each for a stage's wait to carry a
conversation, naming the start it carries and its cause, spent or
lost; a start's conversation with that cause; a start carried that did
not begin; the session in a start's record; and an unsettled start's
other session. Each SHALL serialize to JSON and back unchanged, and
held state written before them SHALL read with none, and a wait or a
conversation written before with no cause SHALL read as spent (49, 146,
161, 179).

#### Scenario: A held state with a carried start

- **WHEN** a held state whose stage waits after a spent agent, whose start carries a conversation with its record's session, whose move says carried and which lists a start carried that did not begin is written and read
- **THEN** it reads back unchanged (146, 161, 179)

#### Scenario: A held state written before

- **WHEN** a held state written before this change is read
- **THEN** it reads with no wait after a spent agent, no conversation, no session and no start carried that did not begin (146)

#### Scenario: A lost conversation's wait and conversation

- **WHEN** a held state whose talk waits to carry a lost start's conversation, and whose talk start 2 carries it, is written and read, and one whose wait and conversation name no cause is read
- **THEN** the first reads back unchanged with the cause lost, and the second reads with the cause spent (49, 179)

### Requirement: An affinity written before reads with its one account

An affinity written before SHALL read with its one account and its pool,
and its intent's moves with no program, and SHALL never be written so
(147, 153).

#### Scenario: An affinity written before

- **WHEN** the JSON of an affinity with one `pool` and `account` and an intent's move with no program is read, and written again
- **THEN** it reads with that pool and account, no account for any program and the move with no program, and is written with no `pool` or `account` key once they are taken away (147, 153)

### Requirement: The host's facts give sessions, labels and the layers

The host's facts SHALL give, as its configuration stands when asked,
each stream's `session`, the sessions it names for its kinds and their
stages, and its labels' shares with their `reserve` and `keep_free`,
each table with any other key it holds; the label the configuration
gives a repository's kind or a kind, with where from; and every value
of the configuration with its layer, whether its local layer was taken,
and each key passed over with its pin (143, 180, 181, 199, C.7).

#### Scenario: The fake host's sessions, labels and layers

- **WHEN** a test scripts the fake host's `work` with the session `office` and `operation`'s reserve of two keeping one free, `[kinds.research] label = "inception"`, and `streams.work.agents` from the local layer
- **THEN** the core reads `office` for `work`, `operation`'s share, `inception` from `host-kind` for research in any repository, and `streams.work.agents` from `local` (143, 180, 181, C.7)

#### Scenario: The fake host's sessions for its kinds and stages

- **WHEN** a test scripts the fake host's `work` naming `work-design` for research and for release's review, and `work-plan` for plan with the other key `label`
- **THEN** the core reads `work-design` for research, `work-design` for release's review, and `work-plan` for plan with `label` named as its table's other key (199, C.7)

### Requirement: The types of a session, a label and the layers

The seams SHALL pass one type each for where an agent opens, a session
and where it came from, `launch`, `stream-stage`, `stream-kind`,
`stream`, `stream-name` or `own`, a stream's sessions for its kinds and
stages, a label, where it came from and its share of its stream, and
the configuration's values with their layers; each SHALL serialize to
JSON and back unchanged, and a held state, a kind, a stream or settings
written before them SHALL read with none (107, 143, 180, 181, 199, C.7).

#### Scenario: A held state with a session and a label

- **WHEN** a held state whose kind declares a label, whose launch's settings name a session, and whose live start holds its session and label, is written and read
- **THEN** it reads back unchanged, and one written before reads with no label and no session (107, 180)

#### Scenario: A start's session from its stream's word for its stage

- **WHEN** a held state whose live start holds the session `work-review` from `stream-stage`, and a stream naming `work-design` for research and `work-review` for release's review, are written and read
- **THEN** each reads back unchanged, and a stream written before reads with no session for any kind or stage (199)

### Requirement: The terminal seam gives what its configuration lacks for flume's plugin

The terminal seam SHALL offer `setup`, what the terminal's own configuration lacks for flume's plugin there, each in words a person acts on, none by default, and the fake terminal SHALL script it (122).

#### Scenario: A terminal with no plugin

- **WHEN** a terminal profile that has no plugin, as the printed command, is asked its setup
- **THEN** it gives none

#### Scenario: The fake's scripted setup

- **WHEN** the fake terminal is scripted with one missing entry and status is asked
- **THEN** status gives that entry, and the fake recorded no open, send or close (57)

### Requirement: The host gives an account's reading

The host's facts SHALL give the reading in the file at a path, or why
there is none, and the core SHALL ask it with the account's `usage`
expanded against the login environment, a `$VAR` that environment lacks
being no reading whose words name the variable (29, 184).

#### Scenario: The fake host's reading

- **WHEN** a test sets the fake host's reading for `work-1` and a refusal for `work-2`
- **THEN** the core reads the reading for `work-1`, the refusal's words for `work-2`, and that the file is missing for `work-3` (184)

### Requirement: The types of a reading and a move past a threshold

The seams SHALL pass one type each for a reading and its windows, a
pool's threshold, an account's `usage`, the strategy `most-left`, and a
move whose `why` is `threshold` with its window and end; each SHALL
serialize to JSON and back unchanged, and a move recorded before SHALL
read with no window and no end (147, 184, 185; core.14).

#### Scenario: A pool and a reading

- **WHEN** a most-left pool with a threshold of 85 and an account naming `usage`, and a reading of two windows, one with no reset, are written and read
- **THEN** each reads back unchanged, the strategy spelled `most-left` (184, 185)

#### Scenario: A move past a threshold

- **WHEN** an intent's move with `why` `threshold`, `window` `5-hour` and `until` 17:00 is written and read, and a move written before with neither
- **THEN** the first reads back unchanged and the second with no window and no end (147, 185)

### Requirement: The terminal seam's attach by view

The terminal seam SHALL offer attach: given a session by its name or
the terminal's own, and a view, for a window the command that shows it,
its program by its full path, its arguments and the variables it needs
with their values, the session started first where it does not run; for
the terminal's own app, the app told to show it; or a refusal where the
terminal holds no such part or offers no such view (187).

#### Scenario: Attach over the fake

- **WHEN** the fake terminal is asked to attach `work` for a window, then its own session for a window, then `work` for its app, then is scripted to refuse its app
- **THEN** it answers a command for `work`, one for its own session, shown, then the refusal, and logs each asking with its view (187)

### Requirement: The window seam's show

The window seam SHALL offer show: given a command and a title, a window
running the command under that title, opened; the line a person runs,
printed, where no window program is named; or a refusal in the window
program's words; within a bound the profile states, opening at most one
window at each asking (188, 189).

#### Scenario: Show over the fake

- **WHEN** the fake window is asked to show a command under `Work Agents` twice, then is scripted to answer printed and then refused
- **THEN** it answers opened twice, logging two shows with the command and the title, then the printed line, then the refusal (188, 189)

### Requirement: The resolver gives a terminal and a window by name

The resolver SHALL give a terminal profile and a window profile by
name, none for a name it carries none for, and list the names it
carries, the one the host's multiplexer names being the terminal the
core is built with (105, 190).

#### Scenario: Named terminals and windows

- **WHEN** the fake resolver is given a terminal `other` and a window `other-window`, and asked for `fake`, `other`, `rex`, `fake-window` and `kitty`
- **THEN** it gives the core's own terminal, `other`, none, the fake window and none, and lists `fake` and `other`, and `fake-window` and `other-window` (105, 190)

### Requirement: The types of a view, a window and a stream's terminal

The seams SHALL pass one type each for a view's command, attach's
answer, show's answer, a view, where a stream's multiplexer, view and
terminal app came from, and a start's terminal; each SHALL serialize to
JSON and back unchanged, a stream written before reading with no
multiplexer, view, terminal app or title, and a start written before
with no terminal (143, 187, 188, 190).

#### Scenario: A view, a window and a stream round-trip

- **WHEN** a command needing `XDG_CONFIG_HOME`, each of attach's and show's answers, each view, a stream's display from each of `stream`, `host` and `neither`, a stream with all four keys, and a start with its terminal are written to JSON and read back, and a stream and a start written with none of those keys are read
- **THEN** each equals the value written, the stream written before reads with none of its four keys, and the start with no terminal (143, 187, 188, 190)

### Requirement: The kinds seam reads sources, packages and locks

The kinds seam SHALL offer read-named, a kind's parts as a launch writes
it; read-source, the identities a source's directory holds; read-package,
a package's kind, identity and hash; the repository's lock at a revision;
a lock on disk; and a lock written whole; flume's own SHALL be packages
with their hashes; the same input SHALL give the same answer, and no read
SHALL change anything (85, 154 to 157, 160, 192).

#### Scenario: Each operation over the fake

- **WHEN** the kinds seam's scenarios run against the fake kinds
- **THEN** each new operation is exercised twice with one input, each answering the same both times and leaving the fake's scripted packages and locks as they were, and each scenario cites its clause (85, 160)

#### Scenario: A kind's text read by the seam

- **WHEN** launches name `review`, `acme/review` and `acme/review@2` over the fakes
- **THEN** the fake kinds' log shows `read_named` given each text whole, and each place's identity is the spelling the fake gave, the core parsing and spelling none (core.4)

### Requirement: The types of a source, a package and a lock

The seams SHALL pass one type for each of: a kind's identity with its
parts and weir's spelling; a kind as named, its source and version each
optional; a package read or why not; a source the host declares or its
fault; a lock's entry and a lock read, none, or why not; and where a
place's kind came from, beside the place's `declared`, which gains
`host`. Each SHALL serialize to JSON and back unchanged (19, 154, 155).

#### Scenario: A held state with where its kind came from

- **WHEN** a held state whose `kind_from` names a host's source, its file, its kind's directory, its hash and the host's lock is written to JSON and read back, and one with no `kind_from` is read
- **THEN** the first equals the value written, and the second reads with none (21, core.14)

### Requirement: The host gives its sources

The host's facts SHALL give the core the sources the host declares, each
by its name with its file and its `kinds_dir`, or with why its file
cannot be read, as they stand when asked; the core SHALL read a source's
packages only through the kinds seam (159, C.7; core.2).

#### Scenario: Sources scripted on the fake host

- **WHEN** a test scripts the fake host with the source `acme` and the faulty source `bad`
- **THEN** the core's resolution of `acme/review@1` reads acme's `kinds_dir` through the fake kinds, and one of `bad/x@1` is refused naming bad's fault with no call to the fake kinds (4, 159)

### Requirement: The terminal seam's runs

The terminal seam SHALL offer runs: given one or more parts, each by
its name or the terminal's own, read together in one reading of the
terminal, for each in the order asked, that it runs or does not, with
the name the terminal knows it by; or that the terminal holds no part a
person can be shown, or that it could not read the part, in the
terminal's words; starting nothing, within a bound the profile states.
No terminal SHALL take a default answer for it (98, 200, 201; core.2).

#### Scenario: Runs over the fake

- **WHEN** the fake terminal is scripted to read `work` running and `desk` unread with `no listing`, and is asked runs of `work`, `desk`, `office` and its own part, one at a time
- **THEN** it answers `work` running, `desk` unread with `no listing`, `office` not running and its own part not running as `own`, logs `runs work`, `runs desk`, `runs office` and `runs own`, and opens nothing (201)

#### Scenario: Runs of several parts over the fake

- **WHEN** the same fake terminal is asked runs of `work`, `desk`, `office` and its own part in one call
- **THEN** it answers each as it answered alone, in that order, and logs the one `runs work desk office own` (201)

### Requirement: The type of a session's running

The seams SHALL pass one type for runs' answer, `SessionRuns`, each of
its four answers serializing to JSON and back unchanged (61, 201).

#### Scenario: Each answer round-trips

- **WHEN** `runs`, `not_running`, `no_part` and `unread`, each with its name or its reason, are written to JSON and read back
- **THEN** each equals the value written (201)

### Requirement: The agent seam takes a program's own settings

The agent seam SHALL name the keys of its program's settings another
setting or the account owns, each with its owner, none by default; and
SHALL say whether its profile takes its program's keys as resolved,
refusing every non-empty entry by default. Its command SHALL be given
the agent's resolved keys alone and the start's folder, and SHALL pass
the keys, listing each file it is to be started with under that folder,
and none where they are empty (88, 195, 196).

#### Scenario: A profile that keeps the seam's defaults

- **WHEN** an agent profile that keeps the seam's defaults is asked its reserved keys, and to take `{ line = "a" }` and then an empty entry
- **THEN** it names no reserved key, refuses the first, saying its profile passes no settings of its program's own, and takes the second (170, 195)

#### Scenario: The fake's way over the seam's scenarios

- **WHEN** the agent seam's scenarios run against the fake agent scripted to take own settings by a file and to reserve `env`
- **THEN** it names `env` with its owner, takes `{ line = "a" }`, and its command lists one file under the start's folder holding the entry (196)

### Requirement: The core writes a command's files before the terminal opens it

The core SHALL write each file an agent's command lists, written beside
and renamed over, readable by the host user alone, before the terminal
opens the agent, and SHALL read a start whose command lists a file
outside the start's folder as not opened, naming the file (16, 17,
196).

#### Scenario: A file written before the open

- **WHEN** a stage starts whose command lists one file under its start's folder
- **THEN** the file holds the command's text when the fake terminal opens the agent, and only the host's user may read it (196)

#### Scenario: A file outside the folder

- **WHEN** a command lists a file outside its start's folder
- **THEN** the start is not opened, naming the file, and nothing is written (196)

### Requirement: The types of own settings

The seams SHALL pass `programs`, a table of programs each giving a
named file, with its path as written and as read, its keys and the keys
left out of it, and `settings`, a table of JSON values, in an agent's
settings; the layers `host-stream` and `host` beside the others; a
profile's reserved keys; and a command's files.
Each SHALL serialize to JSON and back unchanged, and a held state
written before them SHALL read with none (32, 195, 196).

#### Scenario: A held state with own settings

- **WHEN** a held state whose launch gives `programs` for two programs, one with a named file whose `env` was left out, and whose start took a key from `host-stream`, is written and read
- **THEN** it reads back unchanged, holding `env` by name alone, and the same state written with no `programs` reads with none (195)

### Requirement: The seams carry a repository's settings for every kind

The kinds read SHALL carry a repository's settings for every kind
apart from its settings for each kind, and a place's held state SHALL
keep them as the launch read them; each SHALL serialize to JSON and
back unchanged, and one written before them SHALL read with none (21,
32, 195, C.2).

#### Scenario: A held state with a repository's settings for every kind

- **WHEN** a kinds read and a held state whose repository gave `attribution` for every kind are written and read
- **THEN** each reads back unchanged, and the same written with none reads with none (195)

### Requirement: The types of an account taken for its conversation and an agent left

The seams SHALL pass an account's choice from the conversation beside
the affinity and the strategy, and a lost end's and a status agent's
`left_in_pane`; each SHALL serialize to JSON and back unchanged, and an
end or a status written before SHALL read with no agent left (43, 56,
77, 145).

#### Scenario: An account from the conversation and an agent left

- **WHEN** a held state whose talk start runs under work-2 chosen from its conversation, and whose earlier talk end gives `left_in_pane`, is written and read
- **THEN** it reads back unchanged (43, 145)

### Requirement: The agent seam reads whether a conversation runs

The agent seam SHALL answer, given the start whose conversation a start
would carry and the environments of the host's accounts for its
program, whether a process runs that conversation, naming each by its
id and when it started, or that none does, or unknown with what could
not be read; the default SHALL answer unknown, saying its profile reads
no running session (171, 207).

#### Scenario: A profile that reads no running session

- **WHEN** the seam's scenarios run against a profile that implements no live
- **THEN** live answers unknown, saying the profile reads no running session (207)

#### Scenario: The fake's live over the seam's scenarios

- **WHEN** the seam's scenarios run against the fake agent scripted live for one session and unknown for another
- **THEN** live answers runs with the scripted processes for the first, unknown with the scripted words for the second, and none for a third (207)

### Requirement: The types of a live session

The seams SHALL pass one type for live's answer and one for a process it
names; a stage's wait SHALL hold the processes or what could not be read
as last read, and a place kept SHALL hold the processes that keep it.
Each SHALL serialize to JSON and back unchanged, and held state written
before SHALL read with none (49, 64, 207).

#### Scenario: A wait on a live session

- **WHEN** a held state whose talk waits on process 4242, and a place kept while its session runs, are written and read
- **THEN** each reads back unchanged (49, 64, 207)

### Requirement: The host gives the addresses standing agents hold

The host SHALL give, as its configuration stands at each call, each
address the configuration gives a standing agent, with the agent's
name, and the `session` and `stream` the entry gives where it gives
them; and SHALL give no run for such an address (132, C.7).

#### Scenario: Standing and run apart

- **WHEN** the fake host is given a standing agent for `madswan/planner` and a run for `madswan/ops`
- **THEN** its standing entries hold `madswan/planner` alone, and its role for `madswan/planner` is none (132)

### Requirement: The terminal seam gives a session's agents by name

The terminal seam SHALL give the agents a part holds, by part name or
its own, each carrying a name, with that name, the session name a send
reaches it by, and its state; a part that does not run holds none and
is not started; a part it cannot read is answered so, in its words
(132, C.5; core.13).

#### Scenario: An agent by its name

- **WHEN** the terminal scenarios start an agent under a name in a part, then ask for that part's agents
- **THEN** the agent is listed by its name with its state, a send to its session name reaches it, and a name no agent carries is not listed (95, 132)

### Requirement: The types of a standing agent

The seams SHALL pass one type each for a standing agent's entry, a
part's named agents, and an address's standing as status gives it with
what was found, and `SessionFrom` SHALL take `role`; each SHALL
serialize to JSON and back unchanged (56, 132; core.14).

#### Scenario: A standing address round-trips

- **WHEN** a standing entry, a part's agents, and an address's standing found one, none, several and unread, with a session from `role`, are written and read
- **THEN** each reads back unchanged, and an address written before this change reads with no standing (core.14)

### Requirement: The terminal says whether it hands a running agent words

The terminal seam SHALL say whether it can type words into a running
agent's input: the herdr profile SHALL say it can, the printed command
SHALL say it cannot, and the core SHALL keep no agent open for an
answer under a terminal that cannot (53, 95, 198, C.5).

#### Scenario: The printed command

- **WHEN** the printed command's profile is asked whether it hands a running agent words
- **THEN** it says it cannot, and its send is refused with nothing typed (53, C.5)

#### Scenario: herdr

- **WHEN** the herdr profile is asked whether it hands a running agent words
- **THEN** it says it can (53, C.5)

### Requirement: The types of an agent kept open

The held state SHALL hold a stage end's `open_until` and `closed`, with
when, why, one of `time`, `stopped`, `agent_ended` and `account_spent`,
when its agent was ended and whether a bare agent was left in its pane,
and a setting's `keep_open` as written, each read with a default so
state written before reads with none (56, 75, 77, 198).

#### Scenario: An agent let go

- **WHEN** held state with an agent let go by `account_spent` and ended is written and read back
- **THEN** it reads back equal, each field as written (75)

### Requirement: The types of an answer taken in a session

The held state SHALL hold an end's `answer`, with its words, the act it
came by, when it was taken and, where it was handed to a session, when
it was sent and received; each start's count of the answers it took in
its session; and an end's count of the answers its start took before
it, each read with a default so state written before reads with none.
An end's key SHALL count the answers only where there are some (42, 43,
75).

#### Scenario: Two answers

- **WHEN** held state with a start whose two ended-short ends each hold an answer, one by `retry` and one by `tell`, is written and read back
- **THEN** it reads back equal, each field as written, and the two ends key apart (43, 75)

#### Scenario: State written before

- **WHEN** held state written before this change is read
- **THEN** it reads with no agent kept open, no close, no answers and no `keep_open`, and each of its ends keys as it did (75)
