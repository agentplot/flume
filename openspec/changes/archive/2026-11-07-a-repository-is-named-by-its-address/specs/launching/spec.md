## ADDED Requirements

### Requirement: A launch reads its repository's address from its clone's origin

A launch SHALL read its repository's address from the origin of the
clone it runs in or is given, before anything is made: the path of an
origin URL that names a host, scp-like or by `ssh`, `git+ssh`,
`ssh+git`, `git`, `http` or `https`, a trailing `/` and `.git`
dropped, where it is exactly `<owner>/<name>`, and none otherwise (143,
C.7; record 0055).

#### Scenario: Each form of a forge origin

- **WHEN** the address is read from `git@github.com:acme/acme.git`, `https://github.com/acme/acme`, `https://github.com/acme/acme.git/`, `ssh://git@github.com:22/acme/acme.git`, `git://git.example.org/acme/acme.git` and `https://user:token@git.example.org/acme/acme.git`
- **THEN** each gives `acme/acme` (143)

#### Scenario: No forge origin

- **WHEN** the address is read from `/srv/git/notes.git`, `../notes`, `file:///srv/git/notes.git`, `codecommit::us-east-1://notes`, `https://github.com/acme`, `https://gitlab.com/group/sub/name.git` and `git@host:~me/notes.git`
- **THEN** each gives none, and its repository is named by its folder (143)

### Requirement: A place keeps its address and its starts take its tables

A place SHALL keep the address its launch read, and its stream, its
starts' settings and its label SHALL take the host's tables for that
address as the configuration stands at each, a place held with no
address taking its folder's (8, 32, 143, 180).

#### Scenario: Its starts take the address's tables

- **WHEN** a place launched in a repository whose address is `acme/acme` starts its stage, the host giving `[repos."acme/acme".kinds.chore.agents] model = "haiku"` and `[repos."acme/acme".kinds.chore] label = "construction"`
- **THEN** its agent starts with the model `haiku` from `host-repository`, and the place's label is `construction` from `host-repository-kind` (32, 180)

#### Scenario: A place held with no address

- **WHEN** a place whose held record has no address, launched before flume kept one in a repository whose origin is now `git@github.com:acme/acme.git`, starts its stage, the host giving its folder's `[repos."<folder>".kinds.chore.agents] model = "opus"` and `[repos."acme/acme".kinds.chore.agents] model = "haiku"`
- **THEN** its agent starts with the model `opus` from `host-repository` (32)

#### Scenario: A launch over git

- **WHEN** a launch runs in a scratch clone whose origin is `git@github.com:acme/acme.git`, fetched from a bare remote on the host by `url.<bare>.insteadOf`, the host declaring `work` and giving `[repos."acme/acme"] stream = "work"`
- **THEN** the place runs in `work` from `host-repository` and `flume status --json` gives its address `acme/acme` (56, 143)

### Requirement: Status gives a place's address and nothing holds its origin's URL

Status SHALL give a place's address beside its repository, and no part
of the origin's URL but the address SHALL be kept, given or written,
since a URL may carry a credential (56; core.30).

#### Scenario: The address kept and given, the URL never

- **WHEN** chore@1 is launched in a repository whose origin is `https://user:token@github.com/acme/acme.git`, the host declaring `work` and giving `[repos."acme/acme"] stream = "work"`, naming no stream and no intent with an affinity
- **THEN** the place runs in `work` from `host-repository`, status gives its address `acme/acme` beside its repository, and neither its held state nor status holds `token` (56, 143; core.30)

### Requirement: flume finds no clone by its address

A command SHALL take its repository by the folder it runs in or is
given, and flume SHALL find no clone by an address and ask no other
tool where a repository is (143; record 0055).

#### Scenario: No clone found by its address

- **WHEN** `flume launch --repository acme/acme` runs in a folder holding nothing at `acme/acme`
- **THEN** the launch is refused naming the folder it was given, and nothing is made (4; record 0055)
