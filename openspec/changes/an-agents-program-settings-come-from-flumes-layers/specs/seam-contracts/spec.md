## ADDED Requirements

### Requirement: The agent seam takes a program's own settings

The agent seam SHALL name the keys of its program's settings another
setting or the account owns, each with its owner, none by default; and
SHALL say whether its profile takes its program's keys as resolved,
refusing every non-empty entry by default. Its command SHALL be given
the agent's resolved keys alone and the start's folder, and SHALL pass
the keys, listing each file it is to be started with under that folder,
and none where they are empty (88, 175, 176).

#### Scenario: A profile that keeps the seam's defaults

- **WHEN** an agent profile that keeps the seam's defaults is asked its reserved keys, and to take `{ line = "a" }` and then an empty entry
- **THEN** it names no reserved key, refuses the first, saying its profile passes no settings of its program's own, and takes the second (170, 175)

#### Scenario: The fake's way over the seam's scenarios

- **WHEN** the agent seam's scenarios run against the fake agent scripted to take own settings by a file and to reserve `env`
- **THEN** it names `env` with its owner, takes `{ line = "a" }`, and its command lists one file under the start's folder holding the entry (176)

### Requirement: The core writes a command's files before the terminal opens it

The core SHALL write each file an agent's command lists, written beside
and renamed over, readable by the host user alone, before the terminal
opens the agent, and SHALL read a start whose command lists a file
outside the start's folder as not opened, naming the file (16, 17,
176).

#### Scenario: A file written before the open

- **WHEN** a stage starts whose command lists one file under its start's folder
- **THEN** the file holds the command's text when the fake terminal opens the agent, and only the host's user may read it (176)

#### Scenario: A file outside the folder

- **WHEN** a command lists a file outside its start's folder
- **THEN** the start is not opened, naming the file, and nothing is written (176)

### Requirement: The types of own settings

The seams SHALL pass `programs`, a table of programs each giving a
named file, with its path as written and as read, its keys and the keys
left out of it, and `settings`, a table of JSON values, in an agent's
settings; the layers `host-stream` and `host` beside the others; a
profile's reserved keys; and a command's files.
Each SHALL serialize to JSON and back unchanged, and a held state
written before them SHALL read with none (32, 175, 176).

#### Scenario: A held state with own settings

- **WHEN** a held state whose launch gives `programs` for two programs, one with a named file whose `env` was left out, and whose start took a key from `host-stream`, is written and read
- **THEN** it reads back unchanged, holding `env` by name alone, and the same state written with no `programs` reads with none (175)

### Requirement: The seams carry a repository's settings for every kind

The kinds read SHALL carry a repository's settings for every kind
apart from its settings for each kind, and a place's held state SHALL
keep them as the launch read them; each SHALL serialize to JSON and
back unchanged, and one written before them SHALL read with none (21,
32, 175, C.2).

#### Scenario: A held state with a repository's settings for every kind

- **WHEN** a kinds read and a held state whose repository gave `attribution` for every kind are written and read
- **THEN** each reads back unchanged, and the same written with none reads with none (175)
