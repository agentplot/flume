## ADDED Requirements

### Requirement: The kinds seam reads sources, packages and locks

The kinds seam SHALL offer read-named, a kind's parts as a launch writes
it; read-source, the identities a source's directory holds; read-package,
a package's kind, identity and hash; the repository's lock at a revision;
a lock on disk; and a lock written whole; flume's own SHALL be packages
with their hashes; the same input SHALL give the same answer, and no read
SHALL change anything (85, 154 to 157, 160).

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
