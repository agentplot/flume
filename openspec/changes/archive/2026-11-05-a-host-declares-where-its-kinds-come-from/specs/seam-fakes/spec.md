## ADDED Requirements

### Requirement: The fake kinds scripts sources, packages and locks

The fake kinds SHALL script flume's own kinds as packages with hashes; a
source's packages by identity, each with its hash, or a package's or a
listing's refusal; the repository's lock at each revision; a lock on
disk; and a lock written, which it records; the fake host SHALL script
the sources declared; each passing its seam's scenarios, and none
reaching a real configuration directory (106).

#### Scenario: A scripted package edited after a launch

- **WHEN** a test scripts acme's review@1 at one hash, the host's lock records it, a place is launched, and the test then scripts review@1 at another hash
- **THEN** the next launch of `acme/review@1` is refused naming both hashes, the first place's held kind is unchanged, and the fake's log shows one `read_package` call for each launch (21, 156)
