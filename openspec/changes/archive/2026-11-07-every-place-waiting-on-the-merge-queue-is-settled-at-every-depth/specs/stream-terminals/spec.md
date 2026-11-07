## REMOVED Requirements

### Requirement: flume runs attended while any terminal it binds keeps no watch

**Reason**: Whether flume runs attended is read from the host's watch,
which a caller keeps (119, 208), and no terminal keeps one or settles
(122).

**Migration**: The requirement "No terminal decides whether flume runs
attended" below holds the terminals' part, and settling's "A caller
keeps watch over the host" the rest.

## ADDED Requirements

### Requirement: No terminal decides whether flume runs attended

The terminals the host binds, its own and each declared stream's that a
profile carries, SHALL settle nothing and decide nothing of whether
flume runs attended: while no caller keeps watch, every command that
acts on places SHALL settle every place first, and a wait SHALL drive
its settles, under every terminal alike (119, 122, 190, 208).

#### Scenario: One stream under another terminal

- **WHEN** a caller keeps watch, the host's terminal is `fake`, `trial`'s is `other`, and a caller drops a place on `work`
- **THEN** the drop settles nothing first, and `trial`'s places move at the caller's settles (190, 208, S91)

#### Scenario: Two terminals and no watch

- **WHEN** no caller keeps watch, the host's terminal is `fake` and `trial`'s is `other`
- **THEN** a caller's drop settles every place first, under either terminal (119, 208)
