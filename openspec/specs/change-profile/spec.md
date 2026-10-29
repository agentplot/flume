# change-profile Specification

## Purpose
The change seam, Part B.3, through OpenSpec: a change's location,
whether it exists, whether its planning is complete, its tasks ticked
and whether it is archived, read from its files for OpenSpec's
spec-driven schema and through OpenSpec's status for any other, writing
nothing to the place.

## Requirements

### Requirement: A change read through OpenSpec's files

The change profile SHALL read a change at a revision from its files for
OpenSpec's spec-driven schema, and through `openspec status --change
<name> --json` in a checkout of the revision for any other: whether it
exists, whether its planning is complete, its tasks each ticked or not,
and whether it is archived; reading SHALL write nothing to the place
(86, 87, C.3).

#### Scenario: Seam scenarios over the profile

- **WHEN** the change seam's scenarios run against the profile over a scratch repository's changes
- **THEN** each passes, the same revision giving the same answer

#### Scenario: Agreement with OpenSpec's own reader

- **WHEN** a scratch change is read at each of five revisions, from created to archived
- **THEN** existence, planning complete and the tasks agree with what the pinned `openspec status` gives at the same revision (59)

### Requirement: Where the change lies

The profile SHALL locate a change by its name at `openspec/changes/<name>`
in the place (127).

#### Scenario: The change part of a prompt

- **WHEN** a code stage's prompt asks for `{{change}}`
- **THEN** it renders `openspec/changes/<name>`
