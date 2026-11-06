## ADDED Requirements

### Requirement: The list leaves out a place that landed or was dropped unless every place is asked

The core SHALL leave out of the list a place that landed or was
dropped, whether it is kept, being tidied or tidied, and SHALL give its
line only where every place is asked, in the words the list gives it; a
place its caller declared done, a live place and a place being made
SHALL be listed either way (62, 63, 200).

#### Scenario: Each phase of a place whose work is over

- **WHEN** the list is asked of the repository `/r` holding a fix named live whose code agent works, and fixes copied from it whose phases say: landed, with nothing keeping it; dropped, kept holding a file neither committed nor ignored; landed and being tidied; dropped and tidied, its document kept for an end pending; and done
- **THEN** it gives under `/r` the lines `fix--done · done` and `fix--live · code · working` alone; and, asked for every place, the lines `fix--done · done`, `fix--dropped · dropped · kept holding work`, `fix--going · landed · being tidied`, `fix--landed · landed`, `fix--live · code · working` and `fix--tidied · dropped · tidied`, in that order (62, 63, 200)

## MODIFIED Requirements

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

### Requirement: The list reads only what flume keeps

The list SHALL read each place's held state, each live agent's asking
mark from its own record, and each stream's terminal once by runs, and
nothing else: no place's worktree, head, deliverable, change or
submission, no terminal's state, screen or clues of a place's agent, no
repository's binding; and asking it SHALL settle, start, send and
write nothing, and start no session's server, whether every place is
asked or not (57, 58, 200, 201; core.52).

#### Scenario: Fifty places in three repositories

- **WHEN** the list is asked over the fakes of fifty places in three repositories: places whose agents work, one whose agent asks a person, one held, one waiting for room, one submitted, one landed and tidied whose document stays for an end pending, and one landed whose teardown failed at `sluice release` with forty lines of words; the streams `work` and `office` declared, the fake terminal reading `work` running and `office` not
- **THEN** it gives forty-eight places' lines under their repositories, the two landed left out, each waiting place's with whom it waits on, and the lines of `default`, `work` and `office`; the fake change, merge queue and history log no read, the fake terminal logs three runs and nothing else, no session is started, and every document's version is the one it had; and, asked for every place, it gives fifty places' lines, the tidied one's reading `landed · tidied` and the failed teardown's with `kept by sluice release` alone, the store read once and written never (57, 62, 63, 66, 70, 130, 200, 201, S108)
