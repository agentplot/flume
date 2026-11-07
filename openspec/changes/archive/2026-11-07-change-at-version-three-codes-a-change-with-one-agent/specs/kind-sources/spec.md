## MODIFIED Requirements

### Requirement: flume's own kinds are packages whose content never changes

Each version of flume's own kinds SHALL be a package with its content
hash, as weir's crate computes it over its declaration and prompts, and
a check SHALL fail naming the version where a version's hash is not the
one this release records, so a change to what a version holds is a new
version (156, 157).

#### Scenario: The hashes recorded

- **WHEN** the check reads flume's own kinds
- **THEN** fix@1, chore@1, change@1, change@2 and change@3 each hash to the value the check records, the first four to the values recorded before change@3 (156)

#### Scenario: A lock of flume's own at another hash

- **WHEN** a repository's lock records `flume/change@1` at a hash that is not flume's, and a launch names `change`
- **THEN** it is refused naming flume/change@1 and both hashes, and makes nothing (156)
