# seam-fakes Specification

## Purpose
One scriptable fake per seam, and for git's history and the host, so
the engine and every scenario of the suite run without a tool, and each
fake is held to its seam's contract as a profile is (106).

## Requirements

### Requirement: One scriptable fake per seam

There SHALL be one fake for each of the seven seams, and one each for
git's history and the host. A test SHALL set what each fake answers,
including each refusal and each unknown its seam may give, and SHALL
read back every call the core made of it, in order.

#### Scenario: A scripted refusal

- **WHEN** a test scripts the fake place to refuse `remove` naming a teardown step
- **THEN** the core's tidy keeps the place naming that step (66), and the fake's log shows the one `remove` call

#### Scenario: An unknown answer

- **WHEN** a test scripts the fake agent to answer own-work as unknown
- **THEN** the core reads the agent as not quiet and status names what it cannot read (31)

### Requirement: Each fake passes its seam's scenarios

Each fake SHALL pass every scenario of its seam in the suite, the same
scenarios a profile unit runs against its real tool, so a fake never
admits what a profile may not (106).

#### Scenario: The fake place's guarantee

- **WHEN** the place scenarios run against the fake place
- **THEN** make refuses an existing branch and an existing location and touches neither, and every operation is all or nothing (83)

#### Scenario: The fake store's compare-and-set

- **WHEN** the store scenarios run against the fake store
- **THEN** a write at a version that has moved is refused, and a first write for a place that has held state is refused (103)

### Requirement: The fake clock moves only when told

The fake host's clock SHALL move only when a test advances it or the
core waits on it, so a bound of 60 seconds, ten minutes or a stage's
quiet limit runs in a test without real time passing.

#### Scenario: A quiet limit passes

- **WHEN** a test advances the fake clock past a stage's limit with its agent quiet and nothing delivered
- **THEN** the next settle reports the stage stuck, with no real wait (44)

### Requirement: The fake terminal holds recipients' sessions

The fake terminal SHALL hold sessions by name, each with a settable
state, SHALL record every text sent to each in order, and SHALL answer a
name it holds no session for as not found, so the suite can script a
recipient away, asking, or restarted (43, 95).

#### Scenario: A recipient scripted away and back

- **WHEN** a test removes the session `ops` from the fake terminal, settles a place whose end is for `ops`, then adds `ops` back and settles again
- **THEN** the fake records no notice to `ops` from the first settle and one from the second

### Requirement: The fakes script streams, pools, spent accounts and unsettled starts

The fake host SHALL script streams, pools and their accounts; the fake
agent a usage limit with or without a reset and a start that began
without its settings; and the fake store the host's own documents; each
passing its seam's scenarios, and no fake SHALL reach a real account or
configuration directory (106).

#### Scenario: A scripted usage limit

- **WHEN** a test scripts the fake agent's start to stop at a usage limit resetting at 15:00
- **THEN** the core records its account spent until 15:00 and the fake's log shows the `spent` reading (146, 151)

### Requirement: The fake agent scripts a map of modes

The fake agent SHALL script its map of flume's four modes and the own
values it names, whether it reads the mode back, in its word that it
started or at its start's first prompt, and a start that began in
another mode than its record names, each passing the agent seam's
scenarios; its command SHALL pass the value mapped, so a test
reads what the profile was given (106, 161, 169).

#### Scenario: A scripted start in another mode

- **WHEN** a test scripts the fake agent to map `full` to `all` and its code start to begin in `ask-each`
- **THEN** the core records the start's mode `all`, reads it started without its settings naming both modes, and the fake's log shows the command passing `all` (161, 169)

#### Scenario: A scripted start naming its mode at its first prompt

- **WHEN** a test scripts the fake agent to name its mode at its start's first prompt, to map `full` to `all`, and its code start to begin in `ask-each`
- **THEN** its word that it started is taken, its first prompt is sent once and not received, and the core reads the start started without its settings while started, naming both modes (161)

### Requirement: The fake agent scripts carry and a carried start

The fake agent SHALL script, for each account by its environment,
whether it carries a stopped start's conversation, with the session or
the words; the session a start's word names; and a carried start that
ends before its word that it started; its log naming each `carry` it
answers (106, 179).

#### Scenario: A scripted carry

- **WHEN** a test scripts the fake agent to carry to work-2 with the session `s-1`, and a start stops at its account's limit with work-2 chosen next
- **THEN** the core starts the stage resuming `s-1`, and the fake's log shows the `carry` reading for work-2 (146, 179)

#### Scenario: A scripted carried start that ends first

- **WHEN** a test scripts the fake agent's carried start to end before it says it started
- **THEN** the core starts the stage afresh carrying nothing, and the fake's log shows the second start's command with no session (146, S79)

### Requirement: The fakes script sessions, labels and the layers

The fake host SHALL script each stream's session and labels' shares,
the labels its configuration gives kinds, and its configuration's
values with their layers; the fake kinds SHALL script a kind's label;
and the fake terminal SHALL log where each agent was asked to open;
none SHALL read a configuration file of the host's (106).

#### Scenario: A scripted session

- **WHEN** a test scripts the fake host's `work` with the session `office` and a fix is launched on `work`
- **THEN** the fake terminal's log names the fix's agent opened in `office` (107)

#### Scenario: A scripted label

- **WHEN** a test scripts the fake kinds' `deploy` with the label `operation`
- **THEN** a deploy place's label is `operation` from `kind` where the fake host gives none (180)

### Requirement: The fake terminal scripts attach by view

The fake terminal SHALL answer attach with a command of its own for the
session it is asked for a window, and shown for its app, or as scripted
for either view, logging each asking with its view (106, 187).

#### Scenario: A scripted attach

- **WHEN** a test scripts the fake terminal to refuse attach for its app with `no app`, and asks it to attach `work` for a window and then for its app
- **THEN** it answers `fake-terminal attach work` and then the refusal, its log naming `attach work for terminal` and `attach work for app` (187)

### Requirement: The fake window scripts a show

A fake window SHALL log each show with its command and title and
answer opened, printed or refused as scripted, keeping the windows it
opened, and reading no display (106, 188).

#### Scenario: A scripted window

- **WHEN** a test scripts the fake window to answer printed with the line `attach work` and asks the core to open `work`
- **THEN** the core's answer gives that line, and the fake window's log names one show under the stream's title (188)

### Requirement: The fake host and resolver script a stream's display and named terminals

The fake host SHALL script a stream's title, multiplexer, view and
terminal app and the host's own, and the fake resolver SHALL hold
terminals and windows by name, each with a log of its own (106, 143,
190).

#### Scenario: A scripted title

- **WHEN** a test scripts the fake host's `work` with the title `Work Agents`
- **THEN** status gives `work`'s title `Work Agents` from `stream` (143)

#### Scenario: A scripted display

- **WHEN** a test scripts the fake host's own view as `app` and its `trial` with the multiplexer `other`, and asks the fake host for each stream's display
- **THEN** `trial` gives `other` from `stream`, every stream the view `app` from `host`, and every other stream the multiplexer `fake` and the terminal app `fake-window` from `neither` (143, 190)

### Requirement: The fake kinds scripts sources, packages and locks

The fake kinds SHALL script flume's own kinds as packages with hashes; a
source's packages by identity, each with its hash, or a package's or a
listing's refusal; the repository's lock at each revision; a lock on
disk; and a lock written, which it records; the fake host SHALL script
the sources declared; each passing its seam's scenarios, and none
reaching a real configuration directory (106).

#### Scenario: A scripted package edited after a launch

- **WHEN** a test scripts acme's review@1 at one hash, the host's lock records it, a place is launched, and the test then scripts review@1 at another hash
- **THEN** the next launch of `acme/review@1` is refused naming both hashes, the first place's held kind is unchanged, and the fake's log shows one `read_package` call for each launch (21, 156)

### Requirement: The fake terminal scripts runs

The fake terminal SHALL answer runs as scripted for each part asked,
and otherwise that the part does not run, its own part named `own`,
logging each asking as one `runs <part> <part>…`, the parts in the
order asked, and starting nothing (106, 201).

#### Scenario: A scripted runs

- **WHEN** a test scripts the fake terminal to read `work` running, and asks it runs of `work` and then of `office`
- **THEN** it answers `work` running and `office` not running, its log naming `runs work` and `runs office` and no open (201)

#### Scenario: Several parts in one runs

- **WHEN** a test scripts the fake terminal to read `work` running, and asks it runs of `work`, `office` and its own part in one call
- **THEN** it answers `work` running, `office` not running and `own` not running, in that order, its log naming the one `runs work office own` (201)

### Requirement: The fake agent scripts its own settings

The fake agent SHALL script whether it takes own settings, the keys it
reserves with their owners, nested keys by their path among them, and
its way, a file under the start's folder its command names by `--own
<file>`, each passing the agent seam's scenarios; its log SHALL name
the keys it was given, so a test reads what the core passed (106, 195,
196).

#### Scenario: A scripted way and a reserved key

- **WHEN** a test scripts the fake agent to take own settings by a file and to reserve `env`, and a fix's layers give `line` and `tips` inline and `env` by a named file
- **THEN** the core writes the file the fake's command lists before the open, the fake's log names `line` and `tips` and not `env`, and a launch giving `env` inline is refused naming the fake's owner (195, 196)

### Requirement: The fakes bring an agent back bare

The fake terminal and the fake agent SHALL together bring a start's
agent back bare, as a terminal's restore does: its pane holding an
agent again, waiting, whose started ack reads no, at a test's word, by
one helper that logs `bring back bare <identity>`; and the fake terminal
SHALL read it working, waiting or stopped as the test scripts (77, 106).

#### Scenario: A bare agent brought back

- **WHEN** a test brings a talk start's agent back bare and settles the place
- **THEN** the core reads the start's agent waiting with its started ack no, talk reads lost, and the fakes' log shows `bring back bare` for that identity (77)

### Requirement: The fake agent scripts a live session

The fake agent SHALL script, per session, the processes live answers
that run it, or that it cannot read it with words, answering none
otherwise; its log SHALL name each `live` it answers (106, 207).

#### Scenario: A scripted live session that ends

- **WHEN** a test scripts the session `s-1` live as process 4242, asks live, then scripts it ended and asks again
- **THEN** live answers runs with process 4242, then none, and the fake's log shows both readings (207)

### Requirement: The fake host scripts standing agents

The fake host SHALL give each address a test gives a standing agent, as
the test sets it at the time of each call, and none for an address a
test takes it from (132).

#### Scenario: An entry set and taken away

- **WHEN** a test gives `madswan/planner` a standing agent, reads the host's standing entries, takes it away and reads them again
- **THEN** the first read holds it and the second does not

### Requirement: The fake terminal scripts named agents in a part

The fake terminal SHALL hold agents a test adds by name in a part, each
under a session key of its own that its sends and state reach, two of
one name among them, give them to the seam's operation, record what is
sent to each, and answer a part a test refuses as not read (132).

#### Scenario: An agent added, removed and refused

- **WHEN** a test adds `w-planner` waiting in `w`, sends to its key, removes it, then refuses `w` with words
- **THEN** the first read lists it waiting, its key holds the notice, the second read lists none, and the third answers not read with those words

#### Scenario: Two of one name

- **WHEN** a test adds `w-planner` twice in `w`
- **THEN** the part's agents list two named `w-planner`, each under its own key

### Requirement: The fake agent scripts the efforts it takes

The fake agent SHALL script the words it takes as an effort and the
variable it names as read over them, each passing the agent seam's
scenarios; its command SHALL pass `--effort <word>` where the settings
give one, and its log SHALL name each `effort` it is asked, so a test
reads what the profile was given (106, 197).

#### Scenario: A scripted effort taken and passed

- **WHEN** a test scripts the fake agent to take `low` and `high`, and a fix launches with `--agent effort=high`
- **THEN** the fake's log shows `effort` asked of `high` at the launch and at the start, and the start's command passing `--effort high` (88, 197)

### Requirement: The fake terminal scripts whether it hands words

The fake terminal SHALL script whether it can type into a running
agent, and the fake agent SHALL script a session that ends after a
stop-short, so the engine's scenarios read each (53, 96, 134, 198).

#### Scenario: A fake terminal that hands no words

- **WHEN** the fake terminal is scripted to hand no words to a running agent
- **THEN** it says it cannot, and a send to it is refused with nothing typed (53)

#### Scenario: A fake agent's session ended after its stop-short

- **WHEN** the fake agent is scripted to end its session after a stop-short
- **THEN** the fake reads the agent stopped and its session ended from then on (96, 134)

### Requirement: The fake host runs another process while a command sleeps

The fake host SHALL take what another process does while a command of
flume's sleeps on its clock, and run it once, at the next sleep, after
the clock moves, so the engine's scenarios place a settle an agent's
quiet started inside an act's wait (121, 198).

#### Scenario: Another process at the next sleep

- **WHEN** a test gives the fake host what another process does, advances its clock, then a command sleeps twice
- **THEN** it runs once, at the first sleep, after that sleep moved the clock, and not before

### Requirement: The fakes script a base place's branch

The fake merge queue SHALL answer open and close as scripted, by
default making the branch at its base's revision and removing it, and
log each as `open <branch> from <base>` and `close <branch> into
<base>`; a branch it opened SHALL be one it lands on. It SHALL log a
submit by name as `submit <branch> named`, and decide such a
submission in its record alone, its inbox giving nothing for it (106,
133, 202, 205, 206).

#### Scenario: Scripted open and close

- **WHEN** a test scripts the fake merge queue to refuse `open train/x from main` with words, and to keep `train/y` at its close with words
- **THEN** open of `train/x` answers refused with them, close of `train/y` answers kept with them, and the log names each ask (205, 206)

#### Scenario: A submission by name decided in the record

- **WHEN** a test submits `train/z` by name and scripts the fake merge queue to land it
- **THEN** read-submission of `train/z` gives it landed, and the inbox gives nothing for any place (133, 203)

### Requirement: The fakes keep the host's watch and settle nothing

The fake store SHALL hold the host's watch as any host document, so a
test makes flume watched by settling with a bound and lets the watch
lapse by the fake clock; the fake terminal SHALL settle nothing and
answer nothing of a watch, and SHALL log the clues show gives it, the
line's short form among them (106, 122, 139, 208).

#### Scenario: A watch the clock lets lapse

- **WHEN** a test settles every place with a bound of thirty seconds and moves the fake clock past a minute
- **THEN** the engine reads attended, and reads unattended again after the test's next settle with a bound (119, 208)

#### Scenario: Show's clues in the log

- **WHEN** a settle shows a place's clues to the fake terminal
- **THEN** its log names the place, its stage and state, its line's short form and its base's count (139)

### Requirement: The fake merge queue scripts a forecast

The fake merge queue SHALL answer forecast as a test scripts it for a
repository and a branch, clear at the base's revision where nothing is
scripted, a scripted answer standing at every ask until scripted again,
and log each ask as `forecast <branch> onto <base>`; a forecast SHALL
add no submission to its record (106, 210).

#### Scenario: Scripted, then cleared

- **WHEN** a test asks the fake merge queue to forecast `train/x` onto `main`, then scripts it in conflict on `README.md` with `#7` ahead and asks twice, then scripts it clear and asks again
- **THEN** the answers are clear at `main`'s revision, in conflict on `README.md` with `#7` twice, then clear; the log names four forecasts; and the record holds no submission of `train/x` (210)

### Requirement: The fake store moves a document while it reads every place, and refuses one place's write

The fake store SHALL run an act once inside its next read of every
place, after it lists the documents standing and before it reads each,
so a document the act writes is not listed and one it removes reads as
none; and SHALL refuse the next write of a place it is told, once, as
moved, writing nothing (75, 102, 103).

#### Scenario: A document moved inside a read of every place

- **WHEN** the act armed for the next read of every place writes a place's kept document and removes the place's own
- **THEN** that read holds neither, and the next read holds the kept one

#### Scenario: A place's write refused

- **WHEN** the next write of a place's kept document is refused and a tidy writes it
- **THEN** the write is refused as moved, nothing is written, and the next write of it lands

### Requirement: The fake agent scripts the windows its program reports

The fake agent SHALL keep, for a start a test names, the windows its
program reports with when they were taken, read by `windows` in the
folder the start recorded with the session the test gave the start, the
later of two reports standing, log each `windows` read, and report no
windows where the test says its profile builds none; it SHALL pass the
agent seam's windows scenarios both ways (106, 211).

#### Scenario: A scripted report

- **WHEN** a test scripts a start's program to report `five_hour` 42% used taken at 14:02, then 50% used taken at 14:05
- **THEN** `windows` reads the 14:05 report with the start's session, and the fake's log shows each `windows` read (211)

#### Scenario: A profile scripted to report none

- **WHEN** a test scripts the fake agent's profile to build no windows
- **THEN** `reports_windows` answers not offered, and `windows` reads unknown for every start (171, 174)
