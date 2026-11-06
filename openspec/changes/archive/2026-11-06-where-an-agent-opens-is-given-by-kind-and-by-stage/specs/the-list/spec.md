## MODIFIED Requirements

### Requirement: The list reads only what flume keeps

The list SHALL read each place's held state, each live agent's asking
mark from its own record, and each stream's terminal once by runs, for
every session the stream gives together, and nothing else: no place's
worktree, head, deliverable, change or submission, no terminal's state,
screen or clues of a place's agent, no repository's binding; and asking
it SHALL settle, start, send and write nothing, and start no session's
server (57, 58, 199, 200, 201; core.52).

#### Scenario: Fifty places in three repositories

- **WHEN** the list is asked over the fakes of fifty places in three repositories: places whose agents work, one whose agent asks a person, one held, one waiting for room, one submitted, one landed and tidied whose document stays for an end pending, and one landed whose teardown failed at `sluice release` with forty lines of words; the streams `work` and `office` declared, the fake terminal reading `work` running and `office` not
- **THEN** it gives fifty places' lines under their repositories, each waiting place's with whom it waits on and the failed teardown's with `kept by sluice release` alone, and the lines of `default`, `work` and `office`; the fake change, merge queue and history log no read, the fake terminal logs three runs and nothing else, no session is started, and every document's version is the one it had (57, 62, 66, 70, 130, 200, 201, S108)

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

## ADDED Requirements

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
