# Design

## Context

flume's main at 201086c finds a kind in three declarations. `resolve_kind`
in `crates/flume-core/src/launch.rs` looks a launch's name up in the
launch's `--kinds` file, then in the repository's `.config/flume.toml`
at the base's revision, then among flume's own kinds. `named_version`
splits `<name>@<version>` in the core. The kinds seam
(`crates/flume-seams/src/kinds.rs`) offers `read_kinds`,
`read_declaration`, `flumes_own` and `read_binding`, and a kind's
`Declared` is `launch`, `repository` or `flume`. flume's own kinds are
already laid out one directory per version,
`crates/flume/src/profiles/kinds/weir/own/<name>@<version>/kind.toml`
with its prompts beside it, built into the binary. A place's held state
keeps the whole kind it launched with, prompts included
(`PlaceRecord::kind`), so a later change to a declaration never reaches
it (21). A launch naming `crew/design@1` today is refused as resolving
to no declaration.

Record 0022, accepted on 2026-10-29, is stated at blueprints 57a1ba9
(154 to 160, S59 to S63; weir 88 to 93). It makes a kind a package named
`<source>/<name>@<version>`, with versions side by side, and runs a kind
from outside only where the repository's lock or the host's records its
content's hash. It puts the host's packages at a fixed directory,
`$XDG_DATA_HOME/flume/kinds/<source>/<name>@<version>/` (C.7), and leaves
open how they get there (159). Nothing has built it yet. weir-kinds at
flume's pin, 0b6eb5a, and at weir's main, f21b6b3, reads a declaration
and nothing of packages, identities, hashes or locks.

The user ruled on 2026-11-01, reviewing the Boomdrive runner's design,
that kinds are flume's concern. The runner's configuration holds
`kinds = { crew = "/nix/store/…-crew/plugin/flume/kinds.toml" }`, and it
passes that path as `--kinds` to each role's launch. The user asked
instead for:

1. the host's configuration to name kind sources;
2. a launch to name a kind by its identity;
3. a design that fits a marketplace configuring each host with no
   person; and
4. `--kinds <file>` kept for a one-off launch.

The cloud's domain model, ruling T3 of 2026-10-31, has a kind built
into flume, declared in a repository, or a package from a marketplace,
with the cloud copying its identity and hash.

This change builds record 0022 with the host's sources in place of its
fixed packages directory. It needed clauses the statement lacked, and
it amends 154's bare name. The user approved this design on 2026-11-05,
"Start its code", and *The statement as the user ruled* names each
clause as the statement states it.

The rules this design relies on:

- **The seams.** core.1 and core.2: the core names no tool, and only
  the kinds profile names weir-kinds. core.3: a source binds no
  profile.
- **The format.** core.4: a package, an identity, a hash and a lock are
  read and written by weir-kinds alone. core.41: a statechart kind in a
  package is refused as one elsewhere is.
- **Launches and acts.** core.8: every refusal comes before anything is
  made. core.17: `flume kinds lock` is safe to repeat.
- **Status.** core.14: status's JSON form only gains fields. core.15:
  status reads a source where it lies when asked.
- **The repository.** core.16: nothing of flume's goes into a
  repository's content but the lock its caller asks for.
- **Settings.** core.30: a source names a location on the host, so only
  the host declares one.
- **The host's configuration.** The statement's C.7 and section 9: the
  configuration has a managed layer, `config.toml`, and a local one
  over it, `config.local.toml`, which sets only what runs the host's
  work from hour to hour. A source is the managed layer's.
- **Kinds from outside.** core.34 stands as written: nothing from
  outside runs unlocked, flume fetches nothing, and it writes no lock at
  a launch or a settle. Its details gain the sources directory and the
  bare name.
- **The site.** core.35: the site and `/launch` speak to any host's
  operator.
- **The gate.** core.21 and core.22: weir-kinds' pin moves by a chore of
  its own. No new moon project.

The one rules page is `docs/architecture/core.md`. This change touches
no area another page would own. Where flume has no rule, the kit
standard's code profile binds (kit:specs.3, kit:testing.5).

## Goals / Non-Goals

**Goals:**

- A host says where kinds come from, beyond flume's own and its
  repositories', in files a program can write, replace and remove with
  no person and no edit to a file a person keeps (brief 1, 3).
- A launch, and a role, names any kind on the host by its identity, and
  is refused naming what is missing when the host does not hold it
  (brief 2; 4, 154).
- A caller's bare name means tomorrow what it means today, whatever
  sources a program adds to the host (brief 2).
- Several versions of a kind stand side by side, and a place runs the
  kind its launch read whatever happens to its source after (brief 3;
  21).
- Nothing from outside runs unless a lock records its content, and a
  program can lock exactly the content a marketplace published (155,
  156; core.34).
- Status says, for every place, which kind it runs and where that kind
  came from (brief; 19, 56).

**Non-Goals:**

- **Fetching.** flume fetches nothing at a launch, a settle or `flume
  kinds lock` (core.34). How a package reaches a source's directory, a
  marketplace's registry, publishing and discovery all belong to the
  program that manages the host (159).
- **A catalogue in flume.** A `flume kinds` listing beyond the host's
  part of status, and an install, update or search command.
- **A kind that refers to another.** Composition stays out, as record
  0022 says (157).
- **Statechart kinds.** A statechart kind in a source's package is
  refused at its launch as one in a repository is (core.41). Records
  0028 and 0029 are built after crew's rewrite.
- **Sources by repository.** A source declared for one repository on
  the host alone. A launch names its source, and a lock gates what
  runs, so a narrower declaration would add a layer and no safety.
- **The runner's and crew's changes.** crew lays out its release's kinds
  as packages, and the runner gives the host a source file and names
  identities. Each is built in its own repository.
- **The standing prompt.** Record 0021 waits on a plan of its own, and
  so does its effect on flume's own kinds' versions. See *Risks*.

## Decisions

### 1. A source is one file in the host's sources directory

The host declares a source by a file of its configuration:

```
$XDG_CONFIG_HOME/flume/sources/<source>.toml
```

```toml
# ~/.config/flume/sources/acme.toml
kinds_dir = "~/.local/share/flume/kinds/acme"
```

- **The name.** The file's name, less `.toml`, is the source's name. A
  name is stated in one place, so a file cannot say one name while
  lying under another.
- **`kinds_dir`.** Its one key is the directory that holds the source's
  packages. The value is an absolute path, or one whose leading `~` is
  the host user's home, as a repository's key in `[repos]` is read (C.7).
  The key ends in `_dir` because it names a directory. It says `kinds`,
  flume's word, rather than `packages`, which other tools on a host use
  for their own things.
- **Read at each launch.** flume reads the directory as it stands at
  each launch, at each role's wake, at `flume kinds lock` and at each
  status, and writes nothing in it or in a source's `kinds_dir`.
- **Entries flume reads.** It reads every regular file whose name ends
  in `.toml` and begins with no `.`, and ignores every other entry. So a
  program writes `.acme.toml.tmp` and renames it over `acme.toml`, and
  no reader ever sees half a file. A link is read as what it leads to,
  so a host whose files a store manages links them in; a source's file is
  configuration, never hashed, so weir's refusal of a link in a package
  does not reach it.
- **A missing directory.** It is a host with no sources, as a missing
  `config.toml` is a host with no bound. A `sources` that cannot be
  listed, a file of that name among them, fails the configuration as a
  `config.toml` that cannot be read does: it names no source a fault
  could be kept with, and reading it as no sources would hide every
  source the host declares.
- **The managed layer.** The sources directory is of the host's managed
  layer, written by whoever manages the host, as its accounts are
  (C.7). The local layer, `config.local.toml`, declares no source and
  names no `kinds_dir`, since a source names a location on the host.

**Why a directory of files, and not a table of `config.toml`.** A
program that manages the host owns its files whole. It never parses,
edits or reformats a file a person keeps, and two programs, or a
program and a person, never write one file. Adding, replacing and
removing a source are each one rename or one delete, so the host is
never between two states. A source named both in `config.toml` and in a
file would need a rule for which wins, and one place for a fact needs
none. A person writes a source's file by hand as easily as a program
does.

**Layering.** The brief asks for sources to be layered as flume's other
settings are where that makes sense. It makes sense in three places:

- **A bare name.** It resolves over the declarations as it does today:
  the launch's, then the repository's, then flume's own (decision 4).
- **A version.** The locks pick it, the repository's over the host's
  (decision 5).
- **Settings.** A kind's settings stay given by its name in the
  repository's and the host's tables, and join whichever source and
  version the launch resolves to (158).

The sources themselves are declared at one layer, the host's. A
source's `kinds_dir` is a location on the host, which no repository,
kind or launch may name (158; core.30). A launch's one-off source is
already its own layer, `--kinds <file>`.

**A source's fault stays its own.** Each of these refuses every launch
that names the source, naming the file and the fault, and no other
launch (4):

- a file that is not TOML;
- one that gives no `kinds_dir`, a `kinds_dir` that is neither absolute
  nor under `~`, or a key flume does not know;
- one whose name is not a key weir's identity allows;
- one named `flume.toml`, `repo.toml` or `launch.toml`, since those
  sources are flume's own, the repository's and a launch's (154; weir
  C.4).

`flume/change@1` still resolves to flume's own whatever the sources
directory holds. Status's host part names each fault (decision 9). A
`kinds_dir` that does not exist is no fault of the file's, since a
program may write the file before its packages. A launch naming that
source is refused, naming the directory as missing.

This differs from a fault in `config.toml`, which refuses every launch
and start. A source a program dropped in must not stop the host's other
work.

### 2. A source holds packages, one version each, side by side

A source's packages lie at `<kinds_dir>/<name>@<version>/`, each a
package as weir 89 has it: its `kind.toml` at its root, declaring
exactly one kind, with every `prompt` and `standing` path inside the
directory. The kinds profile reads one through weir-kinds' package
reader, given the directory and the identity it lies at (weir 92).

The reader answers with the kind and its content hash, `sha256:` over
each file's path within the package, its length and its bytes (weir
90). The same package hashes the same in a nix store path, under
`$XDG_DATA_HOME` or anywhere else.

Each of these refuses the launch, naming the package and the field
(157, 158; S63):

- a package whose name or version is not the identity it lies at;
- one that declares no kind, or more than one;
- one that names a part outside itself;
- one whose agents tables give an `env`, or a program by a path.

Any number of versions of one name stand side by side in one
`kinds_dir`. The source's listing, read-source, answers every
`<name>@<version>` directory holding a `kind.toml`, by identity. It
ignores entries of any other shape.

**Why packages, and not a file of declarations.** crew ships its kinds
today as one declaration file of several kinds sharing a prompt. A
source could have pointed at such a file, but three facts rule it out:

- weir keys a declaration's kinds by name, so a file holds one version
  of each and cannot stand versions side by side (weir 88).
- weir's content hash is a package's, so a kind inside a shared file
  has no hash a lock could record, short of flume hashing it itself,
  which core.4 rules out.
- A declaration's prompts lie wherever its file says. A package's lie
  inside it, so its hash covers its whole meaning (157).

crew's release lays its kinds out as packages, a `turn.md` in each. That
is crew's change, and the runner keeps `--kinds` until it lands.

### 3. A launch names a kind by its identity

The core never parses or spells an identity. `flume launch --kind` and a
role's `kind` are read through the kinds seam's `read_named`, which asks
weir-kinds (weir C.4; core.4). It answers the source, the name and the
version, the source and the version each optional, or why the text names
no kind.

| written | searched | version taken |
|---|---|---|
| `review` | the launch's declaration, else the repository's, else flume's own (decision 4) | the declaration's one, or for flume's own as decision 5 picks |
| `review@2` | the same | 2, refused where the declaration that resolves the name holds another |
| `acme/review` | the source `acme` the host declares, alone | as decision 5 picks |
| `acme/review@2` | the source `acme`, alone | 2 |
| `flume/change`, `flume/change@1` | flume's own alone | as decision 5 picks, or the one named |
| `repo/bump`, `repo/bump@2` | the repository's declaration alone | its one, refused where another is named |
| `launch/x`, `launch/x@1` | the launch's `--kinds` file alone, refused where the launch carries none | its one, refused where another is named |

Each of these refuses the launch, naming what is missing, and makes
nothing (4, 154; core.8):

- **A source the host does not declare.** "the kind acme/review@2 names
  the source acme, which the host does not declare: no file
  `<sources>/acme.toml`; the host declares crew".
- **A version the source does not hold.** "the source acme holds no
  review@3: it holds review@1 and review@2 in <kinds_dir>".
- **A name the source does not hold.** "the source acme holds no review:
  it holds lint@1".

In this design, "a host's source" means a source the host declares, and
"a kind of a host's source" one of its packages. The place's location
and branch are made
from the kind's name and its declaration's prefix, as today (C.1, C.2).
So a place of `acme/review` named `one` is `review--one`. A second
launch of a `review` from another source with the name `one`, in the
same repository, is refused because that location stands (3).

### 4. A bare name keeps today's meaning

A kind named with no source resolves among the launch's declaration,
the repository's own and flume's own alone. It never resolves to a kind
of a host's source, locked or not.

Where none of the three holds the name, the refusal names each identity
the host's sources hold under it: "the kind review resolves to no
declaration: the launch carries none, the repository declares no
review, flume holds none; the host's sources hold acme/review@1 and
beta/review@3, which a launch names by its source".

**Why a bare name never reaches a host's source.** The brief says a
bare kind name keeps today's meaning, and a host managed by a program
is why. Were a bare name to reach "the one other source that holds the
name, flume's own or one a lock records", as record 0022 had it, a
program that installs and locks a source holding `fix` would leave
every caller's `flume launch --kind fix` refused as ambiguous between
flume's own and the new source. Every caller would then have to change
because the host changed.

Under this decision, adding a source never changes what a bare name
runs. A kind from outside is named by its source, which says to every
reader where it came from. The machines that launch from a source, the
runner and the cloud, already name kinds by their whole identity (the
domain model's unit). *The statement as the user ruled* gives 154's
words, S60's and S62's.

### 5. Locks: nothing from outside runs unlocked

core.34 stands. A kind of a host's source runs only where a lock
records its identity:

- the repository's lock, `.config/flume.lock` at the base's revision,
  read with its kinds (20); or
- the host's lock, `$XDG_CONFIG_HOME/flume/flume.lock`, read as it
  stands at the launch (155).

Both are read through weir-kinds' lock reader (weir 91, 93). A missing
lock records nothing. At each launch of a kind a lock records, flume
reads its content as the host holds it, or as flume carries it for its
own, hashes it through weir-kinds, and checks it against every lock
that records the identity (156). Each of these refuses the launch and
makes nothing (4):

| case | the refusal names |
|---|---|
| no lock records a kind from outside (S60) | the identity, and both ways to record it: `flume kinds lock <identity>` committed on the base, or `flume kinds lock --host <identity>` |
| the content's hash is not the lock's (S61) | the kind, the lock's file and both hashes |
| the two locks give one identity two hashes | both locks and both hashes |
| a lock cannot be read | the lock's file and its fault |

A lock that cannot be read refuses only the launches it bears on: each
launch that resolves to flume's own or to a host's source. A launch of
the repository's own kind, or of a launch's, reads no lock, since
neither needs one (155).

**The version a launch takes when it names none** (154; S59):

1. the newest version of that kind the repository's lock records; else
2. the newest the host's lock records; else
3. the newest its source holds.

For flume's own, this is how a repository that records `flume/change@1`
keeps launching change@1 with a bare `change`. For a host's source,
step 3 picks a version that no lock records, and the launch is refused
naming it as unlocked. So a version a program installed but has not yet
locked never runs, and a launch meanwhile takes the newest the locks
record.

Locks record flume's own kinds only to pick a version. A launch of a
version of flume's own that no lock records runs unchecked, since flume
carries it (155). A version that a lock records is checked against the
hash flume carries.

### 6. `flume kinds lock`

```
flume kinds lock [--host] [--remove] [--hash <hash>] <identity>…
```

- **Records each identity.** Each must be named whole,
  `<source>/<name>@<version>`, and each is recorded at its content's
  hash. A host's source's kind is read as the host holds it, and
  flume's own as flume carries it.
- **Where it writes.** The lock it writes is `.config/flume.lock` of the
  working copy it runs in, or with `--host` the host's lock. The working
  copy is the one whose top git gives from the directory it runs in, or
  from `--repository`'s, and outside a working copy it is refused,
  naming `--host`. It records each identity beside any other version of
  the same kind the lock holds, and replaces an entry of the same
  identity.
- **`--remove`** takes each named entry out, reading no package. So a
  program can unlock a version whose directory it already removed.
  `--remove` and `--hash` are not given together.
- **`--hash <hash>`**, with one identity named, records it only where
  its content hashes to `<hash>`. Otherwise it refuses, naming both
  hashes, and leaves the lock as it was. This is how a program records
  exactly the content a marketplace published, and not whatever lies on
  disk when it runs. C.2 states it (*The statement as the user ruled*).
- **What it refuses.** An identity of `repo` or `launch`, since a lock
  never records those (155). An identity no source holds, named as a
  launch's refusal names it. A lock that cannot be read, which it never
  overwrites.
- **How it writes.** It writes the lock whole, through weir-kinds' lock
  writer, beside the lock and renamed over it. It holds an advisory lock
  kept under flume's state directory, `$XDG_STATE_HOME/flume/kinds-lock.lock`,
  while it reads and writes, so two writers on the host never lose each
  other's entry and nothing is left in the working copy but the lock.
  The host has one such lock, whichever lock of kinds is written, rather
  than one per lock: the command takes it before the core names the
  lock's file, and a write takes milliseconds. Run twice, it leaves the
  same lock (core.17).
- **What it never does.** It fetches nothing and commits nothing. A
  repository's lock reaches its places once its caller commits it on
  the base (155; core.16).
- **Its answer.** It answers through the core, as every command does
  (core.14). The JSON form is `{"locked": {"lock_file", "recorded":
  [{"kind", "hash"}], "removed": [<identity>]}}`, and the person's form
  is one line per entry.

### 7. flume's own kinds are packages, each with a fixed hash

The kinds profile's `OWN` table already holds each version's
declaration and its prompts by their paths within the version's
directory. `flumes_own` gives each as a package: its identity, which
weir-kinds spells, its kind, and its content hash, which weir-kinds
computes over those paths and bytes (156, 157).

This is the first release of flume that reads locks. From it on, a
version of flume's own kinds keeps its content in every later release
(156). A test in `crates/flume/tests/profiles/built_in.rs` holds each
version's hash as this change records it, and fails, naming the version,
where a hash moves. A change to what change@2 holds is then change@3,
and the test says so before the gate does.

### 8. A place keeps the kind its launch read

`PlaceRecord` gains `kind_from`, written once at the launch:

| field | what it holds |
|---|---|
| `identity` | the identity as weir-kinds spelled it, with its parts |
| `source` | the source's name |
| `source_file`, `kind_dir` | for a host's source: its file and the package's directory as read |
| `declaration_file` | for the repository's: `.config/flume.toml` |
| `revision` | for the repository's: the base's revision |
| `declaration_file` | for a launch's: the file's path |
| `hash` | the content's hash, for any package |
| `lock_file` | the lock that recorded it: the repository's where it does, else the host's; none where neither does |

`Declared` gains `host`.

The place's `kind` already holds its whole content, so none of these
reaches a place already launched (21):

- a source's file renamed over, or naming another `kinds_dir`;
- a package edited, or its directory removed;
- a lock moved, or its entry removed.

Held state written before this change reads with no `kind_from`. Its
status gives no identity or hash, and gives the source from `declared`:
`flume`, `repo` or `launch` (core.14).

### 9. Status

A place's status gains:

- **`identity`**, as `acme/review@2`.
- **`kind_from`**, as decision 8 has it, each field present only where
  it applies.
- **`host`** as a value of `declared`, which keeps its other values.

The host's part gains `sources`, read from the sources directory when
asked (core.15). Each entry gives `name`, `source_file`, `kinds_dir`,
`kinds`, every identity the directory holds, and `cannot_read`, the
fault where the file or the directory cannot be read.

The person's form gives the identity on the place's kind line, followed
by `from the source acme (<source_file>) at <kind_dir>`, or `from the
repository at <revision>`, or `from <declaration_file>`, or `flume's
own`. Then come the hash, and `locked in <lock_file>` where a lock
recorded it. A place whose held state keeps no `kind_from` has no kind
line, its first line giving the source from `declared`. The host's part
gives each source on a line of its own: `source acme (<source_file>) at
<kinds_dir>:` and the identities it holds, `holds no kind`, or `cannot
be read:` and the fault. Every field is only added (core.14).

### 10. The seams

The kinds seam gains these operations, each answering the same for the
same input and changing nothing (85, 160):

- **`read_named(text)`**: the parts of a kind as a launch writes it, or
  why the text names no kind.
- **`read_source(source, kinds_dir)`**: every identity the directory of
  the source `source` holds, or why it could not be read. The source's
  name is given beside its directory, since an identity holds its source
  and the directory does not name it.
- **`read_package(source, kinds_dir, name, version)`**: the package's
  kind, identity and hash, or why it could not be read (157, 158).
- **`read_lock_at(repository, revision)`**: the repository's lock at the
  revision.
- **`read_lock_file(path)`**: a lock on disk, the host's or a working
  copy's.
- **`write_lock(path, entries)`**: a lock written whole.
- **`identity_of(source, name, version)`**: the identity of a kind no
  package gives one, the repository's or a launch's, as weir-kinds
  spells it, so the core spells none (core.4).
- **`declaration_path()`** and **`lock_path()`**: where a repository's
  declaration and its lock lie in its tree, `.config/flume.toml` and
  `.config/flume.lock`, which `kind_from` and a refusal name.

`flumes_own` answers packages, each with its identity and hash. A
package read from a source's directory carries that directory as read,
which `kind_from`'s `kind_dir` gives.

The types are `KindIdentity` (`source`, `name`, `version` and
`spelled`, weir's spelling), `KindNamed`, `Packaged`, `PackageRead`,
`SourceRead`, `LockEntry` (`kind`, `hash`), `LockRead` (none, read, or
could not be read) and `KindFrom`. A kind's identity is `KindIdentity`
because the seams already pass an agent's identity as `Identity` (33),
which keeps its name.

The host's sources are configuration, read beside `config.toml` by
`crates/flume/src/host/config.rs` and given to the core by
`Host::sources()`, by name. Each entry, a `HostSource`, is its file and
its `kinds_dir`, or its fault. `Host::sources_dir()` names the directory,
so a refusal of a source the host does not declare names the file it
lacks, and `Host::kinds_lock()` names the host's lock, beside
`config.toml`. Status's fields are the core's answer's,
`PlaceStatus` and the host part's `SourceStatus` in `flume-core`, beside
the fields they join.

In the core, `resolve_kind` takes the named parts and searches as
decision 3 has it. It checks the locks as decision 5 has them and
returns `kind_from` beside the kind. `named_version` goes, since weir
parses. `Engine::lock_kinds` holds `flume kinds lock`'s act. A role's
wake resolves its `kind` through the same path (132).

The fake kinds scripts:

- flume's own as packages with hashes;
- sources, each a set of packages by identity, or a fault;
- a package's refusal;
- the repository's lock at each revision;
- the host's lock;
- a lock written.

The fake host scripts the sources declared. The kinds seam's scenarios
in `crates/flume-conformance/src/seams/kinds.rs` gain these operations,
generic over the trait, below the seam's marker.

### 11. Roles

A role's `kind` in `[roles."<group>/<role>"]` is read as a launch's
`--kind` is, so `kind = "acme/turn@1"` or `kind = "acme/turn"` wakes a
kind of a host's source under the same locks (132, C.2, C.7). A bare
`kind` keeps its meaning (decision 4). A role's run has no `--kinds`
file. So the runner can drop its `kinds` table once crew's kinds lie in
a source, and its role records name `crew/<role>@<version>`.

### 12. A program configuring the host with no person

This is the brief's third ask, followed through. A program that manages
the host, such as a marketplace's agent, does the following. Each step
is a whole-file write, a rename, a delete or a `flume` command, and
none needs a person or touches a running place.

**Install acme/review@2:**

1. Write the package to a directory it owns, such as
   `$XDG_DATA_HOME/flume/kinds/acme/review@2/`, host data under the
   storage rule (section 9).
2. Write `.acme.toml.tmp` in the sources directory and rename it to
   `acme.toml`.
3. Run `flume kinds lock --host --hash sha256:… acme/review@2`, the hash
   being the one the marketplace published.

**Update to review@3.** Add `review@3/` beside `review@2/` and lock it.
A launch of `acme/review` then takes review@3, the host lock's newest,
unless the repository's lock records another. Places on review@2 run on
(21).

**Retire review@2.** Run `flume kinds lock --host --remove
acme/review@2`, then remove the directory. Places on review@2 run on.

**Remove the source.** Remove its lock entries, then its file.

**Read back.** `flume status --json` gives `status.host.sources`:
every source, the identities each holds, and every fault.

Between any two steps the host only refuses launches that name what is
half-installed. An unlocked version is refused as unlocked, and a
locked one whose directory is gone as not held. It never runs content
the lock did not record.

### 13. How each part is proved

Every proof is a check of flume's own, named task by task (kit:testing.5).

- **Seams** (`seams:test`). The round trip of a held state with
  `kind_from`, and of status with `identity`, `kind_from` and the host's
  `sources`. The kinds seam's scenarios over the fake.
- **The kinds profile** (`flume:test`, in
  `crates/flume/tests/profiles/kinds_weir.rs`), over scratch
  directories:
  - a package read, and its hash the same in two places;
  - S63's path outside and `env`;
  - a name that is not the identity it lies at;
  - two versions side by side;
  - the repository's lock at a revision;
  - a lock written and read back.
- **flume's own kinds' hashes**, pinned in `built_in.rs`.
- **The sources directory** (`crates/flume/tests/host.rs`), over a
  scratch `XDG_CONFIG_HOME`:
  - a source read;
  - a temporary and a non-TOML entry ignored;
  - each fault of decision 1 confined to its source.
- **The core** (`core:test`, `conformance:test`). A new
  `crates/flume-conformance/src/engine/kind_sources.rs` holds each
  scenario of the kind-sources spec over the fakes, citing 154 to 160,
  S59 to S63, 191, 192 and S95 to S97. `launching.rs` gains the
  launching spec's scenarios, `place_status.rs` the place-status
  spec's, and `role_waking.rs` the role-waking spec's.
- **The command line** (`flume:test`, in
  `crates/flume/tests/commands.rs`). The binary over a scratch
  repository and a scratch host:
  - a launch by identity, refused unlocked;
  - `flume kinds lock --host`, after which the launch is made;
  - `flume kinds lock` in the working copy, with nothing committed;
  - `--hash` refused with the lock unchanged;
  - two writers at once, both recorded;
  - `--remove`.
- **A walkthrough** (`flume:tools`, in
  `crates/flume/tests/profiles/tools/walkthroughs.rs`), under the
  printed command with the stand-in. A scratch host's source `acme`
  holds `review@1`, a one-stage kind that commits and lands, and the
  host locks it.
  - A launch of `acme/review` lands, and status names `acme/review@1`
    from its file and directory with its hash, locked in the host's
    lock.
  - `review@2` is then added and locked, and the source's file is
    renamed over by one naming a directory that holds `review@2`
    alone. A second launch of `acme/review` runs review@2.
  - A place of review@1 launched before the rename runs on to its
    landing.

### 14. The site and `/launch`

- **The kinds guide** gains *Kinds from a source on the host*:
  - the identity;
  - the sources directory and a source's file;
  - packages side by side;
  - locking and `flume kinds lock`, `--hash` among it;
  - the hash checked at each launch;
  - a bare name keeping its meaning;
  - a place keeping its kind;
  - decision 12's steps for a program that manages the host.
- **The configuration reference** gains the sources directory, a
  source's file and `kinds_dir`, and both locks.
- **The commands reference** gains `--kind <identity>` and `flume kinds
  lock`.
- **The state reference** gains `identity`, `kind_from`, `declared`'s
  `host` and the host's `sources`.
- **The roles guide** gains a role's `kind` as an identity.
- **The plugin's `/launch`** says that a kind of a host's source is
  named `<source>/<name>`.

All of it speaks to any host's operator. It names no host, team or
program of ours, and calls a program that manages the host just that
(core.35).

## The statement as the user ruled

The user ruled each item below on 2026-11-05, approving this design.
The statement states them on agentplot/blueprints' main at ff616eb
(core.19). The new clauses are 191 and 192, and the new scenarios S95
to S97. Parts A and B name a source's declaration, and C.7 names its
file, since the statement's Parts A and B name no file.

1. **191, new in A.3, the host's sources** (decision 1):

   > The host's configuration declares each source of the kinds from
   > outside it holds, in its managed layer (C.7). Each source has a
   > declaration of its own, named by the source, which names where the
   > source's packages lie. A person, or a program that manages the
   > host, writes a source's declaration whole, replaces it whole and
   > removes it. None of these touches another source's declaration or
   > anything else of the configuration. flume reads the sources as they
   > stand at each launch, each wake of a role, each lock its caller asks
   > for and each status. It writes nothing in them or where a source's
   > packages lie. A source's declaration may be unreadable, or take the
   > name of flume's own source, the repository's or a launch's. Such a
   > declaration refuses each launch that names its source, naming the
   > declaration and why, and no other launch (4); status names it (56).

2. **154, 4 and 19 amended** (decision 4). 154, from "Naming no source"
   to its sentence's end:

   > Naming no source, it takes the kind of that name from the launch's
   > declaration, else the repository's own, else flume's own, and never
   > a kind of a source the host declares, which a launch names by its
   > source; where none of the three holds the name, the launch is
   > refused naming each kind of that name the host's sources hold (4).

   4 refuses "a launch naming a version its kind's source does not
   hold, naming it (154), and one naming a source the host does not
   declare, or whose declaration cannot be read, naming the declaration
   and why (191)", and refuses no launch for a name two sources hold.
   19 orders a name "the launch's over the repository's own over
   flume's own", and "a launch names a kind of a source the host
   declares by its source (154)".

   S60:

   > The host's source acme holds review@1, and a repository declares
   > no `review` and locks nothing. A launch of `acme/review` is refused
   > naming acme/review@1 as recorded in no lock, and makes nothing; a
   > launch of `review` is refused naming acme/review@1 as the host's.
   > Once `flume kinds lock acme/review@1` is run in the repository and
   > its lock committed on the base, the launch of `acme/review` runs
   > it, and status names `acme/review@1` with its hash (4, 154, 155,
   > 159).

   S61's next launch is of `acme/review`. S62:

   > The host's sources hold acme/review@1 and beta/review@3, both
   > locked. A launch of `review` is refused naming both as the host's
   > sources'; a launch of `beta/review` runs beta/review@3 (4, 154).

   core.34 keeps its words, since it orders a name as 154 does. Its
   details rule out a bare name resolved to a kind of a host's source.

3. **159 and C.7 amended** (decisions 1 and 2). 159:

   > The host holds the packages of the kinds from outside it may run
   > where its sources say they lie (191), each at its name and version,
   > several versions of one name side by side.

   Its last two sentences stay. C.7's bullet on packages:

   > The sources the host declares are files of its managed layer,
   > `$XDG_CONFIG_HOME/flume/sources/<source>.toml`. Each gives
   > `kinds_dir`, an absolute path or one under `~`: the directory
   > holding the source's packages, each at `<name>@<version>/` with its
   > `kind.toml` at its root (157, 159, 191). flume reads each such file
   > whose name begins with no `.`, ignores every other entry, and
   > writes none; the local layer declares no source.
   > `$XDG_DATA_HOME/flume/kinds/<source>/` is host data where a program
   > that fetches a source's packages may put them. Several versions of
   > one kind lie side by side, and flume reads them and never writes
   > them. The host's lock is `$XDG_CONFIG_HOME/flume/flume.lock`, of
   > the host's configuration (155, C.2).

4. **C.2 amended** (decisions 3 and 6). `flume launch --kind` takes "a
   kind of a source the host declares by its source, `acme/review` or
   `acme/review@1`", and `flume kinds lock` gains:

   > `--hash <hash>`, naming one kind, records it only where its content
   > hashes so, and otherwise refuses naming both hashes and leaves the
   > lock as it was.

5. **19's last sentence, and 56's list, amended** (decisions 8 and 9).
   19:

   > Status names the identity of the kind each place runs and where it
   > came from: its source, with the source's declaration and where the
   > kind lies, or the declaration that holds it. For a package, status
   > names the hash of its content and the lock that recorded it too
   > (155, 191).

   56's host part gains "each source the host declares, the kinds it
   holds by identity, or why it cannot be read (191)".

6. **B.2 amended** (decision 10). 160 is given "the host's sources" for
   "the packages the host holds", and 192 is added beside it:

   > **read-source**: given where a source's packages lie, the identity
   > of every package it holds; or that it could not be read, with the
   > reason (159, 191). The same place always gives the same answer, and
   > reading changes nothing.

7. **S95 to S97 added**:

   > **S95.** The host's source acme holds review@1 and review@2, both
   > in the host's lock. A launch of `acme/review` runs review@2. A
   > program renames over `acme.toml` a file naming a directory that
   > holds review@2 and review@3, and locks review@3 by `flume kinds
   > lock --host --hash`. The next launch of `acme/review` runs review@3,
   > one of `acme/review@1` is refused naming review@1 as one acme does
   > not hold, and the place of review@2 runs on as launched (21, 154,
   > 155, 159, 191).

   > **S96.** A program writes `bad.toml` into the host's sources with
   > no `kinds_dir`. A launch of `bad/x@1` is refused naming the file
   > and `kinds_dir`; a launch of `fix` and one of `acme/review@1` go on
   > as before, and status names the fault (4, 56, 191).

   > **S97.** A host's source holds `fix@1`, locked. A launch of `fix`
   > runs flume's own fix@1, and a launch of `<source>/fix` runs the
   > source's (154, 191).

Record 0040 holds what the user ruled here, accepted, and core.34's
details come with it in one commit (task 1.1).

## What this change rests on

1. **weir-kinds reads packages, identities, hashes and locks, and writes
   a lock.** weir's statement holds 88 to 93 and C.4 since blueprints
   57a1ba9. weir-kinds at 0b6eb5a builds none of it. flume may read none
   of it with its own code (core.4). From weir-kinds, this change needs:
   - an identity read from `<source>/<name>@<version>`, its source and
     version each optional, and spelled back (C.4);
   - read-package, given a directory and its identity, with 89's
     refusals (92);
   - the content hash over a package's files, read from disk or given
     as paths and bytes for a tool's own built-in kinds (90);
   - read-lock (93);
   - a lock written in C.4's form, which 93 does not name and this
     change asks of weir.

   That is a change in agentplot/weir, landed before group 2 is built,
   and task 2.1 moves flume's pin to it. weir's main at f21b6b3 holds
   none of it, so the build stops short at 2.1, naming it, until it
   lands.
2. **The statement as the user ruled** (*The statement as the user
   ruled*), on blueprints' main at ff616eb, which task 8.1's clause
   list reads. That revision holds no S92 to S94, so the list skips
   them. It holds 180 to 190 and S82 to S91, built by changes that
   landed on flume's main after this branch last took main; the list
   names each unbuilt by its change, as main's own list named them
   before they landed, and the merge that brings them takes main's
   list for them. blueprints' main at 2287669 adds 195 to 197, S92 and
   S100 to S103, which no change has built; reading it would name
   more unbuilt and build nothing of this change's, so 8.1 reads
   ff616eb.
3. **Records 0021, 0024's plan stage, 0025 and 0026** are not needed.
   See *Risks* for 0021.

## Risks / Trade-offs

- **flume's own kinds' content is fixed from this release.** Record
  0021's split of flume's own prompts into standing instructions, built
  after this change, then gives each kind it touches a new version, as
  that record foresaw: change@3, fix@2, chore@2. A repository that locks
  `flume/change@2` keeps today's text. Building record 0021 first would
  spare the versions. The user decides which lands first. This design
  works either way, and the pinned hashes are recorded at whatever
  `own/` holds when the change is built.
- **Hashing at each launch** reads every file of the kind's package once.
  Packages are a declaration and a few prompts, read in milliseconds.
  Status reads a source's listing, never its packages, so it stays as
  fast as today.
- **A bare name never reaches a host's source.** A caller who wants
  acme's `review` types `acme/review`. The refusal of a bare `review`
  names it, so the cost is one look. The gain is that a program never
  breaks a caller's launch by installing something.
- **A source's fault is said, not shouted.** A malformed source file
  refuses only the launches naming it. An operator who never names that
  source learns of it from status's host part, and from nothing else.
- **Two sources' kinds of one name share a place's location** in one
  repository under one place name, and the second launch is refused as
  the place stands (3). The place's name is the caller's to vary.
- **Settings by name join every source's kind of that name** (158).
  `[kinds.review.agents]` gives acme's review and beta's review alike.
  That is 158 as stated. A host that needs them apart names a
  repository and kind instead.
- **A program that edits a package in place** without a new version
  sees its next launch refused with both hashes, and a place already
  running unchanged. This is 156's guarantee. The docs say a change to
  a package is a new version.
- **The runner and crew wait on two changes of their own** before they
  stop carrying a kinds file by path. Until then `--kinds` works as
  today.
