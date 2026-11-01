## ADDED Requirements

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
cannot, each read when asked (56, 159; core.15).

#### Scenario: A source and a fault

- **WHEN** the host's sources directory holds `acme.toml`, whose `kinds_dir` holds `review@1/` and `review@2/`, and `bad.toml` with no `kinds_dir`
- **THEN** status's host part gives `acme` with its file, its `kinds_dir`, acme/review@1 and acme/review@2, and `bad` with the fault naming `kinds_dir` (56, 159)
