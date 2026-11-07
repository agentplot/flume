## ADDED Requirements

### Requirement: A repository's tables are named by its address, else its folder

The host's configuration SHALL name a repository, in
`[repos."<repository>"]` and every table under it, by its address,
`<owner>/<name>` as its clone's `origin` gives it on its forge, and by
its folder, a leading `~` in the key the home, where its clone has no
forge origin. A key SHALL be matched exactly, with no star (143, 180,
C.7; record 0055).

#### Scenario: The address's table

- **WHEN** the managed layer gives `[repos."acme/acme"] stream = "work"`, `[repos."acme/acme".kinds.fix] label = "operation"`, `[repos."acme/acme".kinds.fix.agents] model = "haiku"` and `[repos."acme/acme".agents] keep_open = "2h"`, and the repository at `/r` has the address `acme/acme`
- **THEN** the host names `work` for a fix in `/r` from `host-repository`, gives it the label `operation` from `host-repository-kind`, and its settings give the kind's model `haiku` and every kind's `keep_open` `2h` (32, 143, 180)

#### Scenario: A repository with no address

- **WHEN** the managed layer gives `[repos."/r"] stream = "work"` and `[repos."acme/acme"] stream = "personal"`, and the repository at `/r` has no address
- **THEN** the host names `work` for it from `host-repository` (143)

#### Scenario: Matched exactly

- **WHEN** the managed layer gives `[repos."Acme/acme"] stream = "work"` and `[repos."acme/*"] stream = "personal"`, and the repository at `/r` has the address `acme/acme`
- **THEN** the host names no stream for it (143)

### Requirement: The address's table stands whole, else the folder's

Where the configuration has a table under a repository's address, that
table SHALL be the repository's, whole, and its folder's SHALL not be
read; otherwise its folder's SHALL be, so a file written by folder
reads as it did (143; record 0055).

#### Scenario: A folder's table where no table names the address

- **WHEN** the managed layer gives `[repos."~/Code/acme"] stream = "work"` and no table under `acme/acme`, and the repository in `~/Code/acme` has the address `acme/acme`
- **THEN** the host names `work` for it from `host-repository`, so a file written by folder reads as it did (143; record 0055)

#### Scenario: The address's table whole over the folder's

- **WHEN** the managed layer gives `[repos."acme/acme"] stream = "work"` and `[repos."/r".kinds.fix] stream = "personal"`, and the repository at `/r` has the address `acme/acme`
- **THEN** the host names `work` for a fix in `/r` from `host-repository`, the folder's table not read (143)

### Requirement: The local layer finds an address's table by its key

The local layer SHALL set a key of a managed table named by an address
under the same key, and a pin SHALL name such a key as it names any
other (C.7; core.45).

#### Scenario: An address's stream set locally

- **WHEN** the managed layer gives `[repos."acme/acme"] stream = "work"` and the local layer gives `[repos."acme/acme"] stream = "personal"`, and then the managed layer also gives `[host] pin = ["repos.\"acme/acme\".stream"]`
- **THEN** the host names `personal` from the local layer for the repository whose address is `acme/acme`, and with the pin `work` from the managed layer, status naming the key passed over (C.7; core.45)
