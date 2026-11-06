# the-list Specification

## Purpose
What `flume list` gives: a line for each place on the host, under its
repository, saying its stage, its state in its line's words, whom it
waits on and what keeps it, and a line for each stream the host
declares, saying whether its session runs; read from flume's held
state, each live agent's own record and each stream's terminal once by
its runs alone, settling and writing nothing, its JSON form holding the
same, and a place's every fact left to status (56, 58, 62, 200, 201).

## Requirements

### Requirement: The list gives a line for each place under its repository

The core SHALL answer the list with a line for each place on the host
that it lists, a place that landed or was dropped only where every
place is asked and a tidied place whose document stays among them then,
under the place's repository: `<kind>--<name> · <stage> · <state>`, the
stage the place's line names without a repeating stage's group, and the
state in the words the place's line gives it; a part the place has none
of left out (56, 62, 63, 139, 200).

#### Scenario: A working fix and a construct on hold

- **WHEN** the list is asked of a host holding a fix place named one whose code agent works and a change place named two whose construct stage is on hold, both of the repository `/r`
- **THEN** it gives under `/r` the lines `change--two · construct · held · waits on its caller` and `fix--one · code · working` in that order (47, 56, 130, 200)

#### Scenario: A repeating stage and a place being made

- **WHEN** the list is asked of a change@2 place whose code stage repeats at its second task group, and of a place a launch is still making
- **THEN** the first's line is `change--parser · code · working`, with no group, run or tasks, and the second's names no stage and reads `being made` (140, 200)

#### Scenario: Places of two repositories

- **WHEN** the list is asked of a host holding places of the repositories `/b` and `/a`
- **THEN** it gives `/a`'s places first and then `/b`'s, each repository's by `<kind>--<name>` (200)

### Requirement: The list says whom a waiting place waits on

A place's line SHALL add `waits on its caller` where its next move is
its caller's, stalled or not; `waits on the host` where a stage waits
for room and nothing waits on its caller; and `waits on the merge
queue` where its state is `submitted`; and nothing for an agent's own
question in its pane (70, 90, 111, 130, 200).

#### Scenario: The caller, the host and the merge queue

- **WHEN** the list is asked of a place stalled on a stop-short that needs an answer, a place whose first stage waits for room on the host, a place submitted, and a place whose agent's asking mark stands
- **THEN** their lines end `stalled · waits on its caller`, `waiting for room · waits on the host`, `submitted · waits on the merge queue` and `asking` (42, 70, 90, 111, 130, 200)

### Requirement: The list says in a word what keeps a place whose work is over

A place's line SHALL add, for a place whose work is over and that
still stands, `kept by <step>` for the step its teardown failed at,
`kept holding work`, `kept holding a lock` or `kept holding work and a
lock` for what it holds, `kept while its agent runs`, or `kept unread`;
`being tidied` for a place removed whose branch and document are still
to go; `tidied` for a tidied place's document kept for its ends; and
nothing for a place whose tidy has not yet run (62, 63, 64 to 66, 68,
200).

#### Scenario: A teardown that failed

- **WHEN** the list is asked of a chore place that landed, whose teardown failed at `sluice release` with the words of a configuration file that could not be read, forty lines of them
- **THEN** it gives no line of the chore; asked for every place, its line is `chore--one · landed · kept by sluice release`, and neither the line nor the JSON holds any of the teardown's words (63, 66, 200)

#### Scenario: Holds, a live agent and a tidied document

- **WHEN** the list is asked of a dropped place holding a file neither committed nor ignored, a done place whose agent is live, and a landed place tidied whose document stays for an end pending
- **THEN** it gives the done place's line alone, ending `done · kept while its agent runs`; and, asked for every place, their lines end `dropped · kept holding work`, `done · kept while its agent runs` and `landed · tidied` (62, 63, 64, 65, 200)

### Requirement: The list gives a line for each stream the host declares

The core SHALL give a line for each stream the host declares, `default`
among them, in status's order: `<stream> · session <name> · <runs>`,
the session resolved as an open resolves it, the terminal's own named
as its runs names it, and the runs part `runs`, `not running`, `not
read`, or `no profile for <multiplexer>` where the stream's multiplexer
names no profile flume carries, which asks no terminal; and
`<stream> · no session` where the terminal holds no part (107, 143,
186, 190, 200, 201).

#### Scenario: Three streams over the fake terminal

- **WHEN** the fake host declares `work`, naming no session, and `office`, naming the session `desk`, and the fake terminal is scripted to read `work` running and `desk` not, and its own unread
- **THEN** the list gives `work · session work · runs`, `office · session desk · not running` and `default · its terminal's own session · not read`, and the fake terminal logs one runs for each (107, 186, 201)

#### Scenario: A multiplexer flume carries no profile for

- **WHEN** the fake host's `trial` names the multiplexer `rex`, which the resolver carries no terminal for
- **THEN** `trial`'s line is `trial · session trial · no profile for rex`, the other streams' lines stand, the list is not refused, and no terminal is asked of `trial` (190, 200)

### Requirement: The list reads only what flume keeps

The list SHALL read each place's held state, each live agent's asking
mark from its own record, and each stream's terminal once by runs, for
all its sessions together, and nothing else: no place's worktree, head,
deliverable, change or submission, no terminal's state, screen or clues
of a place's agent, no repository's binding; and asking it SHALL
settle, start, send and write nothing, and start no session's server,
whether every place is asked or not (57, 58, 199, 200, 201; core.52).

#### Scenario: Fifty places in three repositories

- **WHEN** the list is asked over the fakes of fifty places in three repositories: places whose agents work, one whose agent asks a person, one held, one waiting for room, one submitted, one landed and tidied whose document stays for an end pending, and one landed whose teardown failed at `sluice release` with forty lines of words; the streams `work` and `office` declared, the fake terminal reading `work` running and `office` not
- **THEN** it gives forty-eight places' lines under their repositories, the two landed left out, each waiting place's with whom it waits on, and the lines of `default`, `work` and `office`; the fake change, merge queue and history log no read, the fake terminal logs three runs and nothing else, no session is started, and every document's version is the one it had; and, asked for every place, it gives fifty places' lines, the tidied one's reading `landed · tidied` and the failed teardown's with `kept by sluice release` alone, the store read once and written never (57, 62, 63, 66, 70, 130, 200, 201, S108)

### Requirement: The list gives none of a place's long words

Neither the list's lines nor its JSON SHALL hold a stop-short's words,
a hold's words, a readiness step's words, a failed submission's words,
a teardown's words, an agent's last words or what a place's holds name;
`flume status` gives each of them of the place (56, 200).

#### Scenario: Every long word left to status

- **WHEN** the list is asked of places whose stop-short says `which design?`, whose hold says `after review`, whose readiness step failed with `moon setup` printing `no toolchain`, whose submission failed with `check failed`, and whose agent's last words are `done for now`
- **THEN** none of those words is in the list's JSON, and status of each place gives them (42, 47, 12, 112, 200)

### Requirement: The list's JSON form

The list's answer SHALL be `{"list":{"places":[…],"streams":[…]}}`,
each place with `place`, `repository`, `name`, `kind`, `stage`,
`state`, `waits_on`, `kept` and `line`, a part it has none of left out,
and each stream with `stream`, `session`, `multiplexer`, `runs` or
`uncarried`, `sessions`, each other session it names with its `name`
and `runs`, left out where it names none, and `line`; written to JSON
and read back unchanged, its form only gaining fields (60, 61, 199,
200).

#### Scenario: The list round-trips and holds its fields

- **WHEN** a list with a place waiting on the host and a place kept by its teardown, and a stream that runs naming a session for a kind that does not run and one uncarried, is written to JSON and read back, and the suite's field list is read
- **THEN** each equals the value written, `waits_on` reads `host`, and the field list holds every field above, the suite failing naming the answer and the field where one is dropped (61)

### Requirement: The list leaves out a place that landed or was dropped unless every place is asked

The core SHALL leave out of the list a place that landed or was
dropped, whether it is kept, being tidied or tidied, and SHALL give its
line only where every place is asked, in the words the list gives it; a
place its caller declared done, a live place and a place being made
SHALL be listed either way (62, 63, 200).

#### Scenario: Each phase of a place whose work is over

- **WHEN** the list is asked of the repository `/r` holding a fix named live whose code agent works, and fixes copied from it whose phases say: landed, with nothing keeping it; dropped, kept holding a file neither committed nor ignored; landed and being tidied; dropped and tidied, its document kept for an end pending; and done
- **THEN** it gives under `/r` the lines `fix--done · done` and `fix--live · code · working` alone; and, asked for every place, the lines `fix--done · done`, `fix--dropped · dropped · kept holding work`, `fix--going · landed · being tidied`, `fix--landed · landed`, `fix--live · code · working` and `fix--tidied · dropped · tidied`, in that order (62, 63, 200)

### Requirement: A stream's line gives each session it names for a kind or a stage

After its own session, a stream's line SHALL give `session <name> ·
<runs>` for each other session the stream names for a kind or a stage,
each once, by name, every session read in the stream's one runs; where
its multiplexer names no profile flume carries, each SHALL be given
with no runs part, before the line's one `no profile for <multiplexer>`
(190, 199, 200, 201).

#### Scenario: A stream's sessions for its kinds and stages

- **WHEN** the fake host's `work` names `work-design` for research and for release's review, and `work` for the kind fix, and the fake terminal is scripted to read `work` running and `work-design` not
- **THEN** `work`'s line is `work · session work · runs · session work-design · not running`, its JSON gives `work-design` not running among its `sessions` and no `work` there, and the fake terminal logs the one runs `runs work work-design` for `work` (199, 200, 201)
