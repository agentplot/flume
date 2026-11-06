## REMOVED Requirements

### Requirement: flume runs unattended only while the watch is kept

**Reason**: Whether flume runs attended is read from the host's watch,
which a caller keeps (119, 208), never from a terminal or its plugin.

**Migration**: "The herdr terminal nudges while the watch is kept"
below gives what the plugin's lease still decides.

### Requirement: The watch settles on its bound and at each state change

**Reason**: The watch keeps no bound: a caller keeps the host's watch
(208), and the plugin's watch nudges (122).

**Migration**: "The watch nudges at each state change and keeps no
bound" below.

### Requirement: The watch reads a dead agent within its bound

**Reason**: The watch keeps no bound to read within (122, 208).

**Migration**: "The watch reads a dead agent at its pane's change"
below.

## ADDED Requirements

### Requirement: The herdr terminal nudges while the watch is kept

The herdr terminal SHALL answer that it nudges only while the plugin's
watch holds a fresh lease, and that it nudges nothing otherwise; neither
answer SHALL decide whether flume runs attended (119, 122, 208, C.5).

#### Scenario: The plugin not running

- **WHEN** the herdr terminal profile is bound, the plugin's watch lease is stale, and a caller keeps watch
- **THEN** the terminal answers that it nudges nothing, and flume still runs unattended

### Requirement: The watch nudges at each state change and keeps no bound

The watch SHALL run `flume settle` whenever herdr reads a flume agent's
state change, or an agent coming up or going in one of the server's
panes, and at no other time; it SHALL keep no bound of its own, and
SHALL run nothing of flume's but `flume settle` and `flume status` (46,
120, 122, C.5).

#### Scenario: A state change

- **WHEN** a stand-in herdr reports a flume agent going from working to idle
- **THEN** the watch runs `flume settle` at that tick

#### Scenario: One listing a tick

- **WHEN** a tick reads the states of five flume panes
- **THEN** the stand-in herdr is asked one listing, and no call per pane (core.29)

#### Scenario: Nothing changes

- **WHEN** no agent's state changes for ten minutes of ticks, and the stand-in flume's status names one submitted place with no agent
- **THEN** the watch runs no `flume settle`, and writes `flume status --json` for the sidebar at its ticks as before (122, 208)

### Requirement: The watch reads a dead agent at its pane's change

An agent that crashed or was killed SHALL be read by the settle the
watch runs at its pane's change, with no caller asking (45, 122, S28).

#### Scenario: A pane killed

- **WHEN** a stand-in herdr's pane holding a flume agent is closed by hand
- **THEN** at the next tick the watch runs `flume settle`, and the stand-in flume records that settle
