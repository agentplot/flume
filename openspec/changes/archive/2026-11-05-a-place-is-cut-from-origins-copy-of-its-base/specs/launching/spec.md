## ADDED Requirements

### Requirement: A place is cut where its base's current revision lives

A launch SHALL cut its place at the base's current revision: where a
merge queue lands the base, where the merge queue's profile says it
lives; otherwise at `origin/<base>` after flume's own fetch; and at the
host's own branch only where the repository has no `origin` or origin
holds no such branch. A fetch that fails otherwise SHALL refuse the
launch, nothing made. The place SHALL keep where its base was read (9,
56, 126, C.1, S92).

#### Scenario: A base no merge queue lands, its local copy lagging origin

- **WHEN** a fix is launched in a clone bound to no merge queue whose own `main` lags origin's by two commits, or under a merge queue that lands no `main`
- **THEN** the place is cut at origin's `main` after flume's fetch, the clone's own `main` is left where it was, and status gives `base_from` `origin` (9, 126, C.1)

#### Scenario: A base origin lacks

- **WHEN** a fix is launched on `trial`, a branch the clone holds and origin does not, or in a repository with no `origin`
- **THEN** the place is cut at the host's own branch, and status gives `base_from` `origin-lacks` or `no-origin` (9, 56, C.1)

#### Scenario: A fetch from origin that fails

- **WHEN** a fix is launched on `main` with origin out of reach
- **THEN** the launch is refused naming the fetch of `main` from origin, and nothing is made (2, 9)

#### Scenario: A base the merge queue lands

- **WHEN** a fix is launched in a clone whose merge queue lands `main` at a revision other than origin's
- **THEN** the place is cut where the merge queue says, origin's `main` never read for it, and status gives `base_from` `merge-queue` (9, 126)
