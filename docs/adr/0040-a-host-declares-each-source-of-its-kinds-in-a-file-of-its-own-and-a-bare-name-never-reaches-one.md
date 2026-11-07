---
number: 40
title: A host declares each source of its kinds in a file of its own, and a bare name never reaches one
status: accepted
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# A host declares each source of its kinds in a file of its own, and a bare name never reaches one

Status: accepted by the user, 2026-11-05, in the Plannotator Inbox, approving the design of the change `a-host-declares-where-its-kinds-come-from` and each item of its ruling: "Start its code".

Amended 2026-11-06 by record 0039, its design approved by the user the same day: a source's declaration may name `roles`, the file in which the source declares the roles it answers, read as its packages are, as it stands at each launch, wake, start, stall and status, and never written by flume.

Deciders: the user, who ruled on 2026-11-01, reviewing the Boomdrive runner's design, that kinds are flume's concern and not the runner's or a control plane's, and asked for the host's configuration to name kind sources, a launch to name a kind by its identity, a design a marketplace can configure each host by with no person at it, and `--kinds <file>` kept for a one-off launch; and who approved the design that answers them. The file per source, its one key `kinds_dir`, the sources being the managed layer's, packages and never a file of declarations, a bare name's reach, a fault confined to the launches it bears on, `--hash`, status's fields and the clause numbers are the designing agent's, approved with the design.
Sources: `agentplot/blueprints:design/flume/requirements.md` at main ff616eb, clauses 4, 19, 21, 56, 154, 155, 156, 159, 160, 191, 192, C.2, C.7, section 9 and S59 to S62 and S95 to S97, as amended with this record; record 0022, whose build this is; records 0017 and 0020; rules core.4, core.14, core.16, core.30, core.34; the change's design, `openspec/changes/a-host-declares-where-its-kinds-come-from/design.md`, decisions 1 to 12; `crates/flume-core/src/launch.rs` (`resolve_kind`, `named_version`) and `crates/flume-seams/src/kinds.rs` (`Declared`) at main 201086c; weir-kinds at 0b6eb5a, which reads a declaration and nothing of packages, identities, hashes or locks.

## Context and problem statement

Record 0022 made a kind a package named `<source>/<name>@<version>`, with versions side by side, run from outside only as a lock records its content's hash. It put a host's packages at one fixed directory, `$XDG_DATA_HOME/flume/kinds/<source>/<name>@<version>/`, and left how a source comes to be there outside the statement (159). It also had a bare name reach "the one other source that holds the name, flume's own or one a lock records", refusing a name two such sources hold (154, S62).

Two things followed that record 0022 did not decide:

- **Nothing on the host says what a source is.** A source was whatever directory lay under the fixed path. A program that manages the host, such as a marketplace's agent, had nothing it owned to write: no declaration to add, replace or remove whole, and no place to say where a source's packages lie, a store path among them.
- **A program could break a caller by installing something.** A host's program that installs and locks a source holding `fix` would leave every caller's `flume launch --kind fix` refused as ambiguous between flume's own and the new source.

Meanwhile the Boomdrive runner carries crew's kinds file by path in its own configuration and passes it as `--kinds` to every launch of a role. The user ruled that kinds are flume's concern. The cloud's domain model ruled the same on 2026-10-31 (its T3): a kind is built into flume, declared in a repository, or a package from a marketplace, and the cloud copies its identity and hash.

## Considered options

1. Record 0022 as written: packages under the fixed host data directory, a source being whatever directory lies there, and a bare name reaching a locked kind from outside.
2. A `[sources]` table in `config.toml`, each source naming a directory of packages or a declaration file of several kinds.
3. One file per source in `$XDG_CONFIG_HOME/flume/sources/`, of the host's managed layer, each naming the directory of the source's packages; a bare name never reaching a host's source.

## Decision outcome

Option 3.

**The host's sources.** A source is declared by `$XDG_CONFIG_HOME/flume/sources/<source>.toml`, whose one key, `kinds_dir`, names the directory holding the source's packages, an absolute path or one under `~` (191, C.7). The file's name is the source's, so a file cannot say one name while lying under another. flume reads each regular `.toml` file whose name begins with no `.` and ignores every other entry, so a program writes `.acme.toml.tmp` and renames it over `acme.toml`, and no reader sees half a file. flume reads the directory as it stands at each launch, wake, lock and status, and writes nothing in it or in a `kinds_dir`. The sources are the managed layer's, written by whoever manages the host, as its accounts are; the local layer, `config.local.toml`, declares none, since a source names a location on the host (C.7; core.30). A missing directory is a host with no sources.

**A source holds packages.** A source's packages lie at `<kinds_dir>/<name>@<version>/`, each a package as weir 89 has it, read through weir-kinds with its content hash (157; weir 90, 92). Any number of versions of one name stand side by side. A source never points at a file of declarations: weir keys a declaration's kinds by name, so such a file holds one version of each; weir's hash is a package's, so a kind in a shared file has no hash a lock could record short of flume hashing it, which core.4 rules out; and a declaration's prompts lie wherever its file says, while a package's lie inside it.

**A launch names a kind by its identity, and a bare name keeps its meaning.** `acme/review@2`, or `acme/review` for the locks to pick the version, searches the source `acme` alone; `flume/…`, `repo/…` and `launch/…` search flume's own, the repository's and the launch's declaration alone (154, C.2). A name with no source resolves among the launch's declaration, the repository's own and flume's own, and never to a kind of a source the host declares, locked or not. Where none of the three holds the name, the refusal names every identity of that name the host's sources hold, which a launch names by its source (154, S60, S62, S97). So adding a source never changes what a bare name runs, and a kind from outside is always named by where it came from.

**A fault stays its own.** A source's file that is not TOML, gives no `kinds_dir` or one neither absolute nor under `~`, gives a key flume does not know, or takes the name `flume`, `repo`, `launch` or one weir's identity does not allow, refuses each launch that names its source, naming the file and the fault, and no other launch (4, 191, S96). A `kinds_dir` not yet written refuses such a launch naming the directory. A lock that cannot be read refuses only the launches it bears on, those resolving to flume's own or to a host's source; a launch of the repository's own kind or a launch's reads no lock (155). Unlike a fault in `config.toml`, which refuses every launch, a source a program dropped in never stops the host's other work.

**Locks.** core.34 stands: a kind of a host's source runs only as the repository's lock at the base's revision or the host's lock records it, its content hashed at each launch (155, 156). A launch naming no version takes the newest the repository's lock records, else the host's lock's, else the newest its source holds, so a version installed and not yet locked is refused as unlocked and never runs.

**`flume kinds lock --hash <hash>`.** Naming one identity, it records it only where its content hashes to `<hash>`, and otherwise refuses naming both hashes and leaves the lock as it was (C.2). A program records exactly the content a marketplace published, and not whatever lies on disk when it runs. `flume kinds lock` refuses an identity of `repo` or `launch`, never overwrites a lock it cannot read, and writes the lock whole, beside it and renamed over it, under an advisory lock in flume's state directory.

**Status names where a kind came from.** A place keeps its kind's identity and where it came from as its launch read them, and status gives `identity` and `kind_from`: the source, the source's file and the package's directory, or the declaration's file and revision, with the hash and the lock that recorded it; `declared` gains `host` (19, 21, 56; core.14). The host's part gives `sources`: each source, its file, its `kinds_dir`, the identities it holds, or why it cannot be read (56, 192).

**Why not the others.** Option 1 gives a program nothing it owns to write, ties every source to one directory so a source in a store path must be copied, and lets an install break a caller's bare name. Option 2 makes a program parse and rewrite a file a person keeps, lets two writers race on one file, and leaves the host between two states while it is written; a source named both in the table and elsewhere would need a rule for which wins. Its declaration-file sources cannot stand versions side by side or carry a hash, as above.

### Consequences

- flume's statement adds 191, the host's sources, and 192, read-source; amends 4, 19, 56, 154, 159, 160, C.2 and C.7; rewords S60 to S62; and adds S95 to S97, on blueprints' main at ff616eb.
- core.34's words stand. Its details rule out a bare name resolved to a kind of a host's source, a source's file or anything in a `kinds_dir` written by flume, and a source's fault refusing a launch that does not name its source, and allow a program that manages the host writing a source's file and running `flume kinds lock --host --hash`.
- Record 0022's packages under a fixed host directory, and its bare name reaching a locked kind from outside, give way to this record. `$XDG_DATA_HOME/flume/kinds/<source>/` stays host data where a program that fetches a source's packages may put them (C.7).
- weir-kinds gains the package reader, an identity's reading and spelling, the content hash, and a lock's reading and writing (weir 88 to 93, C.4). That is weir's change, and flume's pin moves to it before the seams are built (core.4, core.22).
- crew lays its release's kinds out as packages, and the Boomdrive runner gives the host a source file and names `crew/<role>@<version>`, dropping its `kinds` table. Each is a change in its own repository; until then `--kinds` works as today.
