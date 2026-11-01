# A host declares where its kinds come from

## Why

flume finds a kind in three places today: its own, the repository's
`.config/flume.toml`, and the file a launch names by `--kinds <file>`.
A kind that any other repository or tool ships reaches a place only
when every caller carries its file by path. The flywheel runner does
exactly that: its own configuration holds the path of crew's kinds file
in crew's release, and it passes that path to every launch of a role.

The user ruled on 2026-11-01, reviewing the runner's design, that kinds
are flume's concern, not the runner's or a control plane's. The cloud's
domain model ruled the same way on 2026-10-31 (its T3): a kind is built
into flume, declared in a repository's configuration, or a package
distributed through a marketplace, and the cloud copies its identity
and hash. The long-term picture is a marketplace in the cloud that
configures each host's flume as the host needs, with no person at the
host.

Record 0022, accepted on 2026-10-29, already says what a kind from
outside is: a package named `<source>/<name>@<version>`, with versions
side by side, and run only where a lock records its content's hash
(154 to 160; core.34). It left open how a host comes to hold a source,
and it waits on a build plan (`docs/architecture/_open.md`). This change
is that plan, with the host's configuration declaring its sources.

## What Changes

- **The host's sources.** The host's configuration gains a directory,
  `$XDG_CONFIG_HOME/flume/sources/`, holding one file per source. The
  file `<source>.toml` declares the source of that name, and its one
  key, `kinds_dir`, names the directory that holds the source's
  packages. A person, or a program that manages the host, adds a source
  by writing its file, updates it by renaming a new file over it, and
  removes it by deleting the file. flume reads the directory as it
  stands at each launch and writes nothing in it. A source's file that
  cannot be read refuses only the launches that name its source.
- **Packages side by side.** A source's packages lie at
  `<kinds_dir>/<name>@<version>/kind.toml`, one kind each, read through
  weir's crate with its content hash (157; weir 89, 90, 92). Any number
  of versions of one kind stand side by side.
- **A launch names a kind by its identity.** `flume launch --kind
  acme/review@2`, or `acme/review` to let the locks pick the version.
  `flume`, `repo` and `launch` name flume's own kinds, the repository's
  and the launch's declaration. Every other source is one the host
  declares. An identity that no source holds refuses the launch, naming
  it and what that source does hold (4, 154).
- **A bare name keeps today's meaning.** `fix`, `change@1` and a
  repository's own `bump` resolve as they do today: the launch's
  declaration, else the repository's, else flume's own. A host's
  sources never change what a bare name runs, so a source a program
  drops in cannot make a caller's launch of `fix` ambiguous. Where no
  declaration holds a bare name, the refusal names the identities the
  host's sources hold under it.
- **Locks.** A kind from a host's source runs only where the
  repository's lock or the host's records it, as core.34 already
  requires. The repository's lock is `.config/flume.lock` at the base's
  revision, and the host's is `$XDG_CONFIG_HOME/flume/flume.lock`. The
  content is hashed at each launch, and the launch is refused where the
  hash differs. A launch that names no version takes the newest version
  the repository's lock records, else the host's lock's, else the newest
  its source holds (154 to 156).
- **`flume kinds lock`.** `flume kinds lock <identity>…` records each
  kind at its content's hash. It writes the working copy's
  `.config/flume.lock`, or the host's lock with `--host`. `--remove`
  takes an entry out. `--hash <hash>` records one identity only where
  its content hashes to that value, so a program can lock exactly the
  content a marketplace published. It fetches nothing and commits
  nothing (155).
- **flume's own kinds are packages, each with a fixed hash.** This is
  the first release of flume that reads locks. From it on, each version
  of flume's own kinds keeps its content, and a check fails where a
  version's hash moves (156).
- **A place keeps its kind.** A place holds the identity, the whole
  kind and where it came from, as its launch read them. Renaming a
  source's file over, editing or removing a package, or moving a lock
  changes no place already launched (21).
- **Status says where each kind came from.** A place's status gains
  `identity`, as `acme/review@2`, and `kind_from`: the source, the
  source's file and the package's directory, or the declaration's file
  and revision, with the hash and the lock that recorded it. `declared`
  gains the value `host`. The host's part gains `sources`: each source
  declared, its file, its `kinds_dir`, the kinds it holds, or why it
  cannot be read.
- **Roles.** A role's `kind` in the host's configuration may be an
  identity, as a launch's may (C.2, C.7).
- **`--kinds <file>` stays** for a one-off launch: the source `launch`.
- **Docs.** The site's kinds guide, the configuration, commands and
  state references, the roles guide and the Claude Code plugin's
  `/launch`, all in the site's neutral voice (core.35).

## Capabilities

### New Capabilities

- `kind-sources`: the host's sources, one file each in its
  configuration's sources directory; the packages a source holds side
  by side; a launch naming a kind by its identity, and a bare name
  keeping its meaning; the two locks and the hash checked at each
  launch; the version a launch takes when it names none; flume's own
  kinds hashed; `flume kinds lock`; a place keeping the kind its launch
  read.

### Modified Capabilities

- `launching`: a launch's kind is a name, a name and version, or an
  identity, and the version it takes when it names none comes from the
  locks.
- `kinds-profile`: packages, identities, content hashes and locks are
  read, and locks written, through weir's crate alone.
- `place-status`: a place's kind's identity and where it came from, and
  the host's sources.
- `role-waking`: a role's kind may be an identity.
- `seam-contracts`: the kinds seam's operations for a source, a package
  and a lock, and the types they pass.
- `seam-fakes`: the fake kinds scripts sources, packages and locks.
- `clause-coverage`: record 0022's clauses and the host's sources
  built.

## Impact

- **agentplot/weir.** weir-kinds gains the package reader, an
  identity's reading and spelling, the content hash, and the lock's
  reading and writing (weir 88 to 93, C.4). This is weir's own change,
  landed before this one is built, and flume's pin then moves to it
  (core.4, core.22). Design, *What this change rests on*.
- **agentplot/blueprints.** The statement lacks the host's sources and
  holds a bare name that reaches a locked kind from outside. Design,
  *For the user's ruling*, names each clause to add or amend, with
  words for each (core.19).
- **docs/adr and docs/architecture.** Record 0030 holds the decisions
  the user rules here. core.34's details name the sources directory and
  a bare name. `_open.md`'s item on record 0022 leaves once this is
  built.
- **crates/flume-seams.** The kinds seam's new operations and types;
  `Host::sources`; a place record's `kind_from`; status fields, only
  added (core.14).
- **crates/flume-core.** `launch.rs` resolves an identity and checks the
  locks; `wake.rs` resolves a role's kind the same way; `status.rs`
  gives the new fields; a new `kinds_lock.rs` holds `flume kinds lock`'s
  act.
- **crates/flume.** The kinds profile reads packages, identities and
  locks through weir-kinds and hashes flume's own; `host/config.rs`
  reads the sources directory; `flume kinds lock`; the person's form of
  status; the walkthroughs.
- **crates/flume-fakes and crates/flume-conformance.** The fake kinds'
  sources, packages and locks; the kinds seam's scenarios; a new engine
  file of scenarios; the clause list.
- **site/ and plugins/claude.** The guides, the references and
  `/launch`.
- **Elsewhere, not built here.** crew's release lays out its kinds as
  packages, one directory per version. The runner drops its `kinds`
  table and gives the host a source file in its place, its roles naming
  `crew/<role>@<version>`. Each is a change in its own repository.
- No new crate, plugin or site part, so no new moon project. No new
  dependency beyond weir-kinds' move.
