## ADDED Requirements

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
