## REMOVED Requirements

### Requirement: flume runs attended while any terminal it binds keeps no watch

**Reason**: Whether flume runs attended is read from the host's watch,
which a caller keeps (119, 207), and no terminal keeps one: a terminal
nudges at most (122).

**Migration**: The requirement "No terminal decides whether flume runs
attended" below holds the terminals' part, and settling's "A caller
keeps watch over the host" the rest.

## ADDED Requirements

### Requirement: No terminal decides whether flume runs attended

The terminals the host binds, its own and each declared stream's that a
profile carries, SHALL decide nothing of whether flume runs attended,
whatever each nudges: while no caller keeps watch, every command that
acts on places SHALL settle every place first, and a wait SHALL drive
its settles, under every terminal alike (119, 122, 190, 207).

#### Scenario: One stream under a terminal with no nudge

- **WHEN** a caller keeps watch, the host's own fake terminal nudges, `trial`'s nudges nothing, and a caller drops a place on `work`
- **THEN** the drop settles nothing first, and `trial`'s places move at the caller's settles (190, 207, S91)

#### Scenario: Every terminal nudging and no watch

- **WHEN** no caller keeps watch, and both the host's fake terminal and `trial`'s nudge
- **THEN** a caller's drop settles every place first (119, 207)
