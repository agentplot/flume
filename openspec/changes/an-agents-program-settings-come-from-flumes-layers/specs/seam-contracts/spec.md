## ADDED Requirements

### Requirement: The agent seam takes a program's own settings

The agent seam SHALL say whether its profile takes an entry of its
program's own settings, refusing each reserved key with its owner, and
every entry that holds a key by default; its command SHALL be given the
agent's entry alone and the start's folder, and SHALL pass the entry,
listing each file it is to be started with under that folder, and none
where the entry holds no key (88, 175, 176).

#### Scenario: A profile that keeps the seam's defaults

- **WHEN** an agent profile that keeps the seam's defaults is asked to take `{ line = "a" }`, and then an empty entry
- **THEN** it refuses the first, saying its profile passes no settings of its program's own, and takes the second (170, 175)

#### Scenario: The fake's way over the seam's scenarios

- **WHEN** the agent seam's scenarios run against the fake agent scripted to take own settings by a file and to reserve `env`
- **THEN** it takes `{ line = "a" }`, refuses `env` naming its owner, and its command lists one file under the start's folder holding the entry (176)

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

The seams SHALL pass `programs`, a table of programs each a table of
JSON values, in an agent's settings, its layer `host-stream` or `host`
beside the others, and a command's files; each SHALL serialize to JSON
and back unchanged, and a held state written before them SHALL read
with none (32, 175, 176).

#### Scenario: A held state with own settings

- **WHEN** a held state whose launch gives `programs` for two programs, and whose start took a key from `host-stream`, is written and read
- **THEN** it reads back unchanged, and the same state written with no `programs` reads with none (175)
