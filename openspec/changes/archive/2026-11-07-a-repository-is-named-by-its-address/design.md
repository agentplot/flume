# Design

## Context

The host's configuration gives a repository its stream, its kinds'
streams and labels and its agents' settings in `[repos."<repository>"]`
and the tables under it: `.kinds.<kind>` with `stream` and `label`,
`.kinds.<kind>.agents` with its stages', and `.agents` (32, 143, 180,
C.7). flume reads the key as a folder, a leading `~` the home, and
matches it against the launch's repository, the canonical folder that
holds the clone's git directory or the bare repository beside its
worktrees (`crates/flume/src/commands/place.rs`, `repository`).

Three reads of the host's tables take the repository, each through the
host's facts (`crates/flume-seams/src/host.rs`, trait `Host`):

- `stream_for(repository, kind)`, once, at the launch
  (`crates/flume-core/src/launch.rs`, `stream_of`; 8, 143);
- `settings(repository, kind, stream)`, at each start
  (`crates/flume-core/src/settings.rs`, `layers`; 32);
- `label_for(repository, kind)`, at each start and each ranking of the
  queue (`crates/flume-core/src/room.rs`, `label_of`; 180, 181).

`crates/flume/src/host/config.rs` implements all three over one
`HostConfig`, its maps keyed by the folder `home()` reads from each
key. The local layer finds a repository's table in the managed one by
the same reading (`as_read`, `set_at`), so `~/Code/acme` and
`/Users/x/Code/acme` are one table.

Record 0055, accepted by the user on 2026-11-06 ("yes 55"), decides
that the configuration names a repository by its address, read from the
clone, the folder kept where a repository has no forge origin, and that
flume finds no clone by its address. The statement carries it in 143
and C.7, at agentplot/blueprints main 9c060ab and later.

### What the family does

Read on 2026-11-07 at each tool's main.

- **sluice** reads `<owner>/<name>` from a remote's URL in its GitHub
  profile (`crates/sluice/src/profiles/github/app.rs`,
  `Repo::from_url`): `https://github.com/`, `ssh://git@github.com/` and
  `git@github.com:`, a trailing `/` and `.git` dropped, exactly two
  segments, any other host none. It needs github.com because it calls
  GitHub's API.
- **Boomdrive** names every repository it keeps by `owner/name`
  (`crates/boomdrive-control/src/want.rs`, `Launch.repository`;
  `crates/boomdrive-crew/src/config.rs`, `StateRepository`), two
  segments, and runs flume in the clone it keeps.
- **crew** writes `owner/name` for a state repository in its teams file.

So the family's address is `<owner>/<name>`, two segments, and flume's
reading gives, for every URL sluice reads, the address sluice reads
(design 1). flume's is wider, any host, because 143 names the
repository's forge and not GitHub. Neither calls the other: sluice's
reading belongs to its GitHub profile and answers its API, and flume's
is the core's reading of a git fact (core.1 leaves git to the core).

## Goals / Non-Goals

**Goals:**

- Every table of the host's configuration that names a repository finds
  it by its address, else by its folder (143, C.7).
- A host's file written by folder reads as it did.
- flume finds no clone by its address and asks no other tool where a
  repository is.

**Non-Goals:**

- flume's own Nix module, and flume's own reading of each account's
  usage, the other two points of record 0055.
- A command that takes an address to find a clone, or a setting for
  where clones are kept.
- A role's `repository`, which names the folder a role's run is
  launched in, as a launch's `--repository` does (C.7, 193).

## Decisions

### 1. The address is read from origin's URL, exactly `<owner>/<name>`

A repository's address is read from its clone's `remote.origin.url` as
git's configuration holds it, by `git config --get remote.origin.url`
in the repository (record 0055 names the key). The URL names a host
when it is:

- scp-like, `[<user>@]<host>:<path>`, with no `://` and no `/` before
  the first `:`; or
- `<scheme>://[<user>[:<password>]@]<host>[:<port>]/<path>`, its scheme
  `ssh`, `git+ssh`, `ssh+git`, `git`, `http` or `https`.

The address is that path with a trailing `/` and then one trailing
`.git` dropped, and a leading `/` dropped in the second form, where it
is exactly two segments, `<owner>/<name>`, neither empty, `.` or `..`,
nor beginning with `~`. Anything else gives no address: a path on the
host, absolute or relative, a `file://` URL, any other scheme or a
remote helper's `<transport>::<address>`, a path of one segment or of
three or more, and a repository with no `origin`. A repository with no
address is named by its folder (143).

Its case is kept as the URL writes it, and the key is matched exactly
(143). The host is not part of the address: the family's addresses
carry none, and the record names `<owner>/<name>`, so two forges'
`acme/acme` are one repository to the configuration, as two clones of
one address on one host are (record 0055's consequences).

The reading is one pure function of the core,
`crates/flume-core/src/address.rs`, `of_origin(url) -> Option<String>`,
unit tested over each form, so every history the core is given reads
the same address.

*Not taken:* a path of three or more segments read whole as the
address, as a GitLab subgroup's `group/sub/name`. The family writes
`owner/name` and nothing else, so such a key would match no runner's
address; the clone is named by its folder instead. *Not taken either:*
a list of known forges' hosts, which would be flume guessing what a
forge is; and `git remote get-url origin`, which rewrites the URL by
`url.<base>.insteadOf`, so the address would turn on a host's rewriting
rules rather than on the origin the clone names.

### 2. The history reads origin's URL, and the core keeps only the address

The history trait (`crates/flume-seams/src/history.rs`), the core's
reading of git beside the seams and no seam of Part B, gains
`origin_url(repository) -> Option<String>`: `remote.origin.url` as
configured, none where the repository has no `origin` or git cannot
say. `GitHistory` (`crates/flume/src/host/history.rs`) runs it through
`Git::at`, so a folder holding a bare repository beside its worktrees
reads the bare repository's configuration, as `common_dir` does.
`FakeHistory` (`crates/flume-fakes/src/git.rs`) answers what its
`FakeGit` was given by `set_origin_url`, none otherwise.

The core passes the URL to `of_origin` and drops it: no held state,
status, refusal, log line or record holds the URL, which may carry a
user and a token (`https://<user>:<token>@<host>/...`), and only the
address goes on (56; core.30's details on values). The history's
scenarios gain one, over git and over the fake: a repository's origin
URL read as written, a credential in it among it, and none once
`origin` is removed.

### 3. The host's tables take a repository's name, its address and its folder

The host's three reads take a repository's name instead of a folder,
one new type of the seams' crate (`crates/flume-seams/src/host.rs`):

```rust
/// A repository as the host's configuration names it (143, C.7).
pub struct RepositoryName {
    /// `<owner>/<name>` as its clone's origin gives it on its forge;
    /// none where it has no forge origin.
    pub address: Option<String>,
    /// The folder that holds it, as a launch names it.
    pub folder: String,
}
```

`Host::stream_for`, `Host::label_for` and `Host::settings` each take
`&RepositoryName`. The engine builds one for a launch from the launch's
folder and the history's address, and for a place from its held state
(design 5). Only these three reads change; every other read of the
repository, the merge queue's, the place's, the history's and the
intent's, still takes the folder, which is where the clone is.

### 4. The address's table first, whole, else the folder's

A repository's table is `[repos."<address>"]` where the configuration
has a table under that key, and `[repos."<folder>"]` otherwise; one
table, never the two merged. `HostConfig` holds every key it read
under `repos` (`repos`, keyed by each table read), so its
`table_for(&RepositoryName)` gives the address where `repos` holds it
and the folder otherwise, and `stream_for`, `label_for` and the
host's `settings` read their maps under that one key. So the order of
143 and 180 is unchanged: a stream for the repository and kind, then
for the repository, then for the kind, each repository word read from
the one table.

The key is read as written where it is an address and with its
leading `~` the home where it is a folder. `home()` changes nothing in
a key that does not begin with `~`, so `acme/acme` is read as written,
and the local layer's `as_read` and `set_at` find the managed layer's
address table by the same key, `[repos."acme/acme"] stream = "personal"`
in `config.local.toml` replacing the managed layer's stream for
`acme/acme` (C.7; core.45). A pin names it as
`"repos.\"acme/acme\".stream"`.

*Not taken:* the two tables merged key by key, the address's over the
folder's. A host moving its file from folders to addresses would then
keep stale folder settings beneath the address's without a word, and
143 names one table per repository. *Not taken either:* refusing a
configuration that names one repository both ways; a host's file
written by folder stays readable while its address tables are added,
which is what record 0055 asks.

`FakeHost` (`crates/flume-fakes/src/host.rs`) follows the same rule
over its maps, the address where any of its repository-keyed maps
holds it, else the folder, so the engine's scenarios show it.

### 5. Read at the launch and kept with the place

A launch reads the address from the clone it runs in or is given,
through `Engine::repository_name`, before `stream_of`, and the place's
held record keeps it, `PlaceRecord.address`
(`crates/flume-seams/src/held.rs`), serialised only where it has one
and read as none from a document written before. Each later read of
the host's tables for the place, its starts' settings and its label,
builds its name from `place.repository` and `place.address`, and takes
the host's tables as the configuration stands then (32, C.7).

Reading it once keeps the queue's ranking from running git for each
waiting place at each settle (`label_of` is called per place per
ranking), and keeps a place's name from changing under it if its
clone's origin is changed mid-flight: its stream was fixed by the
launch's address (8), and its settings and label follow the same one.

A place launched before this change has no address held, and its
starts take its folder's tables, as when it was launched. A host that
moves its file from folders to addresses keeps its folder tables until
those places are gone; every place launched after takes the address's
table.

*Not taken:* reading the address again at each start. It would cost a
git call per waiting place per settle for a fact that does not change
in the life of a place, and would let a place's settings move to
another repository's table mid-flight.

### 6. flume finds no clone by its address

A command still takes a folder: the one it runs in, or
`--repository <path>`, a path read against the working directory as
today. `--repository acme/acme` is the folder `acme/acme` under the
working directory, and refused as any folder that holds no repository
is where none stands there. A role's `repository` stays a location on
the host (C.7, 193). flume asks no other tool where a repository is and
holds no setting for where clones are kept (143; record 0055).

### 7. Status and refusals

A place's status gains `address`, the held one, beside `repository`,
the folder, given where the place has one. The JSON form gains a field
and loses none (56; core.14). A refusal of a launch whose host-named
stream is undeclared names the repository by its address where it has
one and by its folder otherwise (4). Status's configuration view gives
each key as written, an address key among them, with its layer (56,
C.7), unchanged.

### 8. The rule

Rule core.55 joins `docs/architecture/core.md` under "Settling,
surfaces and state", beside core.45, drawn from record 0055:

> **[core.55]** The host's configuration names a repository by its
> address, `<owner>/<name>` as its clone's `origin` gives it on its
> forge, else by its folder: the address read from the clone a command
> runs in or is given, its table taken whole where the configuration
> has one and the folder's otherwise, matched exactly; flume finds no
> clone by its address and asks no other tool where a repository is.

Its details rule out a clone found by an address, a key matched by a
star, case or a prefix, the address's and the folder's tables merged,
an origin's URL kept, logged or given beyond its address, and
`insteadOf` read into the address; and leave a host's folder-keyed file
reading as it did, and a place launched before taking its folder's
tables.

### 9. The site

`site/src/content/docs/reference/configuration.md` and
`site/src/content/docs/guides/agent-settings.md` name a repository's
tables by its address, with an example of each, say when its folder
names it instead and that the address's table comes first, whole, to
any host's operator (core.35).

## Rules and clauses

The design rests on clauses 4, 8, 32, 56, 143, 180 and C.7 and on
record 0055. It relies on core.1 (git is the core's, and the core
names no tool), core.2 and core.3 (no seam or profile changes, and
the `[profiles]` table is not read for this), core.8 (the address is
read before anything is made, and refuses nothing), core.14 (status
gains a field), core.16 and core.45 (flume writes neither layer),
core.30 (no value of a URL is given) and core.15 (the address is a fact
of the place's launch, not a deliverable's, so keeping it is no record
of a deliverable between asks). The one page of `docs/architecture/`
is `core.md`; no area page exists.

C.7's bullet on the host's agents tables still names
`[repos."<repository>".kinds.<kind>.agents]` "by its location on the
host as a launch names it", with a folder in its example. Clause 143's
sentence, "The configuration names a repository by its address", and
record 0055, "Wherever the host's configuration names a repository,
`[repos."<repository>"]` and the tables under it", cover those tables,
and a folder key still matches its clone, so this design reads the
bullet as 143 and the record say and departs from no clause.

## Risks / Trade-offs

- **Two forges, one address.** `github.com/acme/acme` and
  `gitlab.com/acme/acme` take one table. The family's addresses carry
  no host, and the record names `<owner>/<name>`. A host that clones
  both and gives them apart names each by its folder and writes no
  table under the address. → Accepted; status gives each place's
  address, so the operator sees which table a place took.
- **A deeper path is named by its folder.** A GitLab subgroup's clone
  has no address under design 1. → Its folder's table works as today.
- **A folder table shadowed.** Once a host adds `[repos."acme/acme"]`,
  a `[repos."~/Code/acme"]` beside it is no longer read for that clone.
  → Design 4's rule, said in the site's reference.
- **Places launched before.** They keep their folder's tables (design
  5).
