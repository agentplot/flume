# history-and-host Specification

## Purpose
Git's history and the host's facts read for real beside the seams: the
commits of a place since its stage started and the repository's common
git directory; the host's clock, its secrets, the commands of flume's
that live, the bound its configuration states and the lock over its
room.

## Requirements

### Requirement: Git's history read through git

The core's reading of git's history SHALL go through `git` in the
repository: a branch's head, whether a head holds a commit a start does
not, the newest commit at a head touching a path and whether it is since
a start, whether a path is in a commit since a start, and whether a
branch name is valid (24, 25, C.1).

#### Scenario: The history scenarios over git

- **WHEN** the history scenarios run against the reader over scratch repositories
- **THEN** each passes, a head reset to an ancestor of the start holding nothing since it

### Requirement: The default branch read through git

The repository's default branch SHALL be read through `git` as the one
`origin/HEAD` names, set from `origin`'s own `HEAD` where the repository
lacks it, and as the one its common git directory's `HEAD` names only
where it has no `origin`, never a branch that names no commit (20, 132,
C.7).

#### Scenario: The default branch is the remote's

- **WHEN** a repository made by `git init --bare`, its `HEAD` naming `master`, a branch never made, is given `origin`, whose default branch is `main`, and is fetched with no `origin/HEAD` set, and its default branch is read twice, then again with `origin` removed, then again with its `HEAD` naming `main`
- **THEN** the first two reads give `main` and `origin/HEAD` names `origin/main` after the first, the third gives none, and the fourth gives `main` (20, 132, C.7)

### Requirement: The host's facts

The host's facts SHALL be its clock, a sleep, a secret of 32 random
bytes, a file's write time, a command of flume's named by process and
start time with whether it lives, the bound its configuration states,
and a host-wide lock over the room (41, 70, 75).

#### Scenario: Secrets never repeat

- **WHEN** a thousand secrets are taken
- **THEN** no two are equal

#### Scenario: The room lock is exclusive

- **WHEN** two processes take the room lock at once
- **THEN** the second takes it only after the first lets it go

### Requirement: The host's streams and pools

The host's facts SHALL give, as its configuration stands when asked,
its declared streams, the stream it names for a repository and kind
and the pool for a stream, each with the key it came from, and each
pool by name; none SHALL be kept between asks (32, 143, 144, C.7).

#### Scenario: A stream's pool and the host's

- **WHEN** the configuration gives `[streams.work] pool = "work"` and `[host] pool = "personal"`, and declares `personal` with no pool
- **THEN** the host names `work` for `work` from the stream, and `personal` for `personal` and `default` from the host (144)

### Requirement: A branch read at origin through git

The core's reading of git's history SHALL read a branch at `origin`
after `git fetch origin +refs/heads/<branch>:refs/remotes/origin/<branch>`,
and never a tracking ref an earlier fetch left in its place. It SHALL
say origin lacks the branch only where that fetch fails and `git
ls-remote --exit-code` exits 2, say so of a repository with no
`origin`, and answer any other failed fetch with git's words (9, 126,
C.1).

#### Scenario: A branch at origin

- **WHEN** a scratch repository's branch is read at origin with no `origin`, then once a bare remote beside it holds the branch two commits past the repository's own, then for a branch the remote lacks, then with the remote out of reach
- **THEN** the first says no origin, the second gives the remote's head and leaves the repository's own branch where it was, the third says origin lacks it, and the fourth gives git's words (9, C.1)

### Requirement: The answerer behind a stream

The host's facts SHALL give, for a stream, the role its sources declare
to stand behind it, an address of that stream, with the source that
declares it; none where no role stands behind it; and, where two do,
none, naming both. They SHALL read the sources as they stand when asked
and keep nothing between asks (143, 191, 193, C.7).

#### Scenario: Each stream's door

- **WHEN** the scratch host declares the streams `east` and `west`, and the scratch source `acme`'s roles file declares `east/operator` with `answerer = true`, and `west/operator` likewise
- **THEN** the host names `east/operator` for `east` and `west/operator` for `west`, each from `acme`, and none for `default`

#### Scenario: No answerer

- **WHEN** no source declares a role standing behind any stream
- **THEN** the host names none, for every stream

#### Scenario: Two behind one stream

- **WHEN** the sources `acme` and `beta` each declare a role of `east` standing behind it, `east/operator` and `east/ops`
- **THEN** the host names none for `east`, naming both roles and their sources

### Requirement: The roles a host answers with

The host's facts SHALL give every address a source declares with
`answers`, a run's, a standing agent's or a carried one's, each with
its stream and its words, as the sources stand when asked (28, 41, 132,
193).

#### Scenario: The host's roles

- **WHEN** `acme`'s roles file declares runs for `east/planner`, `east/ops` and `west/ops`, each with its `answers`, gives `east/design` a standing agent with its `answers`, carries `north/design` with its `answers`, and declares `east/reviewer` with no `answers`
- **THEN** the host gives `east/design`, `east/ops` and `east/planner` for `east`, `west/ops` for `west` and `north/design` for `north`, each with its words, and not `east/reviewer`

### Requirement: The repository's origin read through git

The repository's origin URL SHALL be read through `git` as
`remote.origin.url` holds it in the repository's configuration, a
folder holding a bare repository beside its worktrees reading the bare
repository's, never rewritten by `url.<base>.insteadOf`; none SHALL be
read where the repository has no `origin` (143, C.7).

#### Scenario: The origin's URL as written

- **WHEN** the history scenarios run against the reader over a scratch repository given the origin `https://user:token@github.com/acme/acme.git`, and again once `origin` is removed
- **THEN** the first read gives that URL as written and the second none (143)

#### Scenario: A rewritten origin read as configured

- **WHEN** a scratch repository's origin is `git@github.com:acme/acme.git` and its configuration rewrites that URL to a bare remote on the host by `url.<bare>.insteadOf`
- **THEN** its origin URL reads `git@github.com:acme/acme.git` (143, C.7)
