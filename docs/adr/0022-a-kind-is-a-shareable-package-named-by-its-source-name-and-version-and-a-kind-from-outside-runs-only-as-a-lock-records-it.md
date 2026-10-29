---
number: 22
title: A kind is a shareable package, named by its source, name and version, and a kind from outside runs only as a lock records it
status: accepted
date: 2026-10-29
decision-makers:
- Chuck Swanberg
---

# A kind is a shareable package, named by its source, name and version, and a kind from outside runs only as a lock records it

Status: accepted by the user, 2026-10-29, in their design session, on how kinds are to be shared from repository to repository and host to host, through a marketplace later, without flume running anything its host has not chosen.

Deciders: the user, who chose identity by source, name and version, versions installed side by side, a lock with a content hash per kind, locking as the act of trust, nothing from outside run unlocked, kinds that name nothing a host owns, prompts and parts that travel with their kind, and flume's change@1 and change@2 both kept; the names of the sources, the version a launch that names none takes, the lock's file and shape, the host's lock beside the repository's, the hash's shape, that a kind stands alone for now, where packages lie on a host, and that the weir work is this record's own build, are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 57a1ba9, section 3, clauses 4, 19 to 22, 154 to 160, C.2, C.7, section 9 and S59 to S63, and `design/weir/requirements.md` section 3, clauses 58, 88 to 93, C.4, section 9, S10, S17 and S18, both as amended with this record; records 0004, 0017, 0019, 0020, 0021; rules core.4, core.16, core.30; weir-kinds at a214b95 (`read_kinds`, `Declaration::kinds` keyed by name), `crates/flume-core/src/launch.rs` (`resolve_kind`), `crates/flume/src/profiles/kinds/weir/` (`flume.toml`, `OWN_PROMPTS`, `flumes_own`), `crates/flume-seams/src/kinds.rs` (`Declared`).

## Context and problem statement

A kind is named by its name and nothing else. weir-kinds returns a declaration's kinds keyed by name, and `resolve_kind` looks a launch's name up in the launch's declaration, then the repository's, then flume's own (19), each holding one kind per name. A kind's version is a number its declaration states (23), shown in status and decided by nothing.

That is enough while every kind is flume's own or a repository's. It is not enough for the next step, kinds shared from one repository and host to another, and later through a marketplace:

- Two publishers will each have a `review`. A name alone cannot tell them apart.
- A repository that runs a shared kind needs to know it runs the same text tomorrow as the text it reviewed today. A name, or a name and a version number, promises nothing about the bytes.
- A kind's prompts drive agents that run with the host's accounts, its tools and its repository. A kind fetched from elsewhere and run on its name is a stranger's instructions run under the host's credentials.
- Record 0019's change@2 had to replace change@1 outright, because one declaration holds one kind per name. A repository that wanted change@1 had to copy it into its own `.config/flume.toml`.

Records 0017 and 0020 already said a kind is portable: it names no account, pool, stream or environment. Record 0021 made a stage's standing instructions the kind's portable part. Nothing yet said what a kind is as a thing that travels.

## Considered options

1. Name-only kinds, as today: a shared kind is copied in under a name, and its version is a number nobody checks.
2. Vendoring: each repository copies the kinds it uses into its own tree, with no identity beyond the copy, and edits them as it likes.
3. Kinds as plugins of the agent program, such as Claude Code plugins, installed and versioned by the program's own plugin manager.
4. A kind is a package with an identity, its source, name and version; versions of one kind sit side by side; a repository or a host locks the kinds it runs from outside, each with its content's hash; nothing from outside runs unlocked.

## Decision outcome

Option 4.

**Identity.** A kind is named `<source>/<name>@<version>`: `flume/change@2`, `acme/review@1` (154; weir 88, C.4). A source is where kinds come from. flume's own kinds are the source `flume`. A repository's own declaration, `.config/flume.toml` at the base's revision, is the source `repo`. A launch's declaration, `flume launch --kinds <file>`, which sounding uses for its elaboration kinds, is the source `launch`. Every other source names kinds from outside, and none may take those three names. Within a source a name and a version name exactly one kind.

**Versions side by side.** A source may hold several versions of one name. flume's own holds change@1 and change@2, and a host may hold acme/review@1 and acme/review@2. The repository's declaration and a launch's still hold one version per name, since the format keys a declaration's kinds by name; versions side by side live in packages, one kind each.

**What a launch gets.** A launch names its kind as `change`, `change@1` or `flume/change@1`, and so does a role's `kind` in the host's configuration.

- A name with no source resolves as clause 19 has always ordered it: the launch's declaration, else the repository's own, else the one other source that holds the name, whether flume's own or one a lock records. Where two such sources hold it the launch is refused naming both, so the caller names the source. flume never guesses between two publishers.
- A launch with no version takes the version the repository's lock records for that kind, the newest of them where it records several; else the host's lock's, the same way; else the newest its source holds.

So a repository that records `flume/change@1` keeps launching change@1 when a later flume adds change@3. A repository that records nothing takes flume's newest, which is today's behaviour, and crew's launches of `change` keep working unchanged. The lock is where a repository pins a version. A caller that wants another version for one place names it.

**Settings stay keyed by name.** Record 0017's `[kinds.<kind>.agents]`, in a repository's file and in the host's configuration, and record 0020's `[kinds.<kind>]` stream and pool, give settings to a kind by its name. They join whichever source and version the launch's name resolves to (158, C.2). A setting written for `change` holds for change@1 and change@2 alike, and nothing in those tables changes.

**The lock.** A repository's lock is `.config/flume.lock`, at the base's revision, read with its kinds (20). The host's is `$XDG_CONFIG_HOME/flume/flume.lock`, in the host's configuration (C.7). Both are TOML in the kind format's lock form, which is weir's (weir 91, C.4), an array of entries each holding an identity and a hash:

```toml
[[kinds]]
kind = "flume/change@1"
hash = "sha256:9c1f…"

[[kinds]]
kind = "acme/review@1"
hash = "sha256:4e07…"
```

A lock records each identity at most once, and may record several versions of one kind. `flume kinds lock <identity>…` writes it: it records each kind named at the hash of its content as the host holds it, or as flume carries it for its own, into the working copy it runs in, or into the host's lock with `--host`. It reads only what the host already holds, fetches nothing and commits nothing. A repository's lock reaches its places once its caller commits it on the base, like any other change to the repository's declaration (20). flume writes a lock at no launch and no settle (155). This record names the command and its effect; its flags beyond these are the build's.

Two locks, because two parties may trust. The repository's lock is the repository's maintainers' choice and travels with the repository to every host that runs its places. The host's lock is the operator's choice for that machine: a kind run in many repositories, or by a role the host wakes (132), with no repository's lock in the way. A launch may run what either records. Where both record one identity with different hashes the launch is refused naming both, since one identity must be one content.

**The hash.** A kind's content is its package, its declaration and every part the declaration names. Its hash is `sha256:` over each file's path within the package, its length and its bytes, in path order, so the same package hashes the same on every machine wherever it lies (weir 90, C.4). weir-capture already holds the family's sha256 content hash, so the hash is weir's, computed once for every reader. At each launch of a locked kind flume reads its content as the host holds it and refuses the launch where the hash differs, naming the kind and both hashes (156). A place already launched keeps the identity and the content it launched with (21): its held state already carries its whole kind, prompts included, so an edit to a package, a new version installed or a lock moved never touches a place in flight. flume's own kinds are hashed too, so a repository that locks `flume/change@1` gets the content it locked. From the first release of flume that reads locks, a version of flume's own kinds keeps its content in every later release, and a change to what it holds is a new version.

**Where packages lie.** A host holds the packages of the kinds from outside it may run, as host data under `$XDG_DATA_HOME/flume/kinds/<source>/<name>@<version>/`, each with its `kind.toml` at its root (159, C.7). Several versions of one kind lie side by side there, and flume reads them and never writes them. How a package reaches that directory, copied there by a person or fetched later from a marketplace, is outside this record (159). The marketplace's registry, publishing and discovery are left to a later record. What this record fixes is what such a fetch must produce: a package at its identity, which runs nothing until a lock records it.

**Locking is the act of trust.** A shared kind's prompts drive agents with the host's accounts, its tools and its repository. So nothing from outside runs unlocked (155). Recording a kind in a lock says that this content, to the byte, may drive agents here, and the hash makes that statement hold until someone locks again. Three sources need no lock because they are already trusted on other grounds:

- flume's own kinds are trusted with flume, whose installation is the operator's act.
- A repository's own kinds are trusted with the repository's revision, which the repository's own review and its merge queue admitted, as its code was.
- A launch's declaration is trusted with its caller, which already acts on the host with the host's accounts.

A lock records flume's own kinds only to pick a version. A package found on the host and recorded in no lock refuses the launch naming its identity (S60), so a kind dropped on a machine, or fetched by mistake, never runs because its name matched.

**Host-neutral.** This confirms records 0017 and 0020. A kind names nothing a host owns, whichever source declares it: no environment, account, pool, stream or location on the host (158; core.30). It may name the agent program it supports, by name, and a model by the class its program knows, such as `opus` or `sonnet`. The host binds both through its own layers of clause 32, since the host's settings for a kind come before the kind's own. A kind's declaration giving an `env`, or a program by a path, refuses the launch naming the field. Before this record that held for a repository's file. Now it holds for a package's and flume's own too.

**Self-contained, and standing alone.** A kind's prompts and standing instructions travel inside its package. No path in it leads outside the package: no `..`, no absolute path, no link out (157; weir 89). A kind refers to no other kind either. No stage extends, includes or hands over to another kind's, so flume's change@1 and change@2 each carry their own construct and verify, the same text twice. A kind stands alone for now, for four reasons:

- A hash that covers a kind's whole meaning is only possible when nothing outside the package bears on it. A kind that extended another would mean something different whenever the other moved, and its lock would have to record the other at its hash as well.
- Extension brings resolution with it. Two kinds that extend two versions of one base, a base's source missing, and whose prompts win where both speak each need a rule and a refusal.
- Trust would chain: locking one kind would silently trust whatever it extends.
- The cost of standing alone is duplicated prompt text, which is cheap, and which record 0021's standing instructions keep in a few files per kind.

A later record can add composition with lock entries for each part once shared kinds show what they need. Nothing here rules it out except a reference written today.

**flume's own kinds as packages.** flume's built-in kinds become one package per version, each a `kind.toml` and its prompts, built into the binary and read as any package is: fix@1, chore@1, change@1 and change@2 (22, C.2). change@1 is today's change, its code stage one agent for the whole change. change@2 is record 0019's. Both are installed, and a repository picks one by its lock, a launch by naming it.

**This supersedes part of record 0019.** Record 0019's versioning said change@1 and change@2 could not both be flume's own, since weir keys kinds by name and the launch resolves a name. So flume's own `change` moved to version 2 outright, and a repository wanting change@1 declared it itself. Under this record they sit side by side as two packages. A launch of `change` with no lock takes change@2, the newest, as record 0019 intended. A repository that locks `flume/change@1` keeps it, and a launch may name `change@1`. Record 0019 is amended to match, and so is its open change.

**Why not the others.** Option 1 can tell neither two publishers nor two contents apart, and runs whatever bytes sit under a name. Option 2 gives every repository its own fork of every kind it uses, with nothing to say which upstream version a copy came from, whether it was edited, or that an update exists. It also writes a host's choice into every repository rather than the repository's choice into a lock. Option 3 ties a kind to one agent program and its plugin manager, while a kind is the family's format and flume runs any program an agent profile binds (core.1). It also puts trust in that program's installer rather than in a lock flume checks, and it would put a kind's stages, which the program knows nothing of, into a package format made for the program's own extensions.

### Consequences

- flume's statement amends section 3, clauses 4, 19 to 22, C.2, C.7 and section 9, and adds 154 to 160 and S59 to S63; weir's statement amends section 3, 58, C.4, section 9 and S10, and adds 88 to 93, S17 and S18, at blueprints 57a1ba9. Rule core.34 is drawn from this record. core.4's details name the package, the identity, the hash and the lock as weir's, and core.16's allow `flume kinds lock` its file.
- weir-kinds gains the package reader, the identity and its spelling, the content hash and the lock's reading and writing (weir 88 to 93). That work belongs to this record's own build, not to record 0019's change. Record 0019's change was planned and approved without it, its change@2 needs none of it, and this record waits on a plan. Its group 5 keeps change@1 and change@2 side by side in a form that needs no weir change: each of flume's own versions is a declaration of its own, read through weir-kinds as it reads today, and a launch may name a version. Sources, locks and hashes come with this record's build.
- Record 0021's split of flume's own prompts into standing files changes those kinds' content. Built before the first release that reads locks, it moves no version, as that record says. Built after, each kind it touches takes a new version.
- Nothing of this record is built until the user approves a plan; `_open.md` names the wait.
