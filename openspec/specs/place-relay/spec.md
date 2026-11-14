# place-relay Specification

## Purpose
How a caller that carries a person's screen and keys relays a place's
live agent's terminal through flume: what the terminal shows goes out
as it changes, the person's input goes in as typed, a person's at the
keyboard and never a caller's send; the relay ends at the caller's
close or once the agent's terminal goes, is refused for a place with no
live agent or a terminal that offers none, and flume writes, settles
and keeps nothing of it (107, 120, 190, 221, 222; boomdrivedev/boomdrive
record 0004).

## Requirements

### Requirement: A relay reaches the place's live agent through the terminal its start recorded

The core SHALL relay the agent of the place's current stage whose start
is live, read once from the place's held state with no lock taken,
through the terminal that start recorded, the host's own for a start
that recorded none, and through no other terminal (107, 190, 221;
core.49).

#### Scenario: The current stage's agent

- **WHEN** a fix place named one has its code agent live under the fake terminal, and the core is asked to relay it with a link that closes at its first tick
- **THEN** the link is given the agent's screen first, and the answer is relayed, naming the place and the agent of code's first start, ended by the caller (221)

#### Scenario: A start recorded in another terminal

- **WHEN** the place's code start recorded the terminal `other`, a fake terminal the resolver carries, while the host's own is another fake terminal
- **THEN** the relay reads and types through `other`, and the host's own terminal is asked no relay (190, 221)

### Requirement: A relay of a place with no live agent, or of a terminal that offers none, is refused

The core SHALL refuse a relay of a place no live start holds an agent
of, naming the place, that it has no live agent and its state in its
line's words, asking no terminal; and SHALL refuse a relay the agent's
terminal refuses, in that terminal's words (4, 221, 222).

#### Scenario: A place waiting for room

- **WHEN** the core is asked to relay a place whose first stage waits for room
- **THEN** it is refused naming the place and that it has no live agent, and no terminal is asked to relay (4, 70, 221)

#### Scenario: A terminal that offers no relay

- **WHEN** the fake terminal is scripted to refuse relay with `no relay here`, and the core is asked to relay the place's live agent
- **THEN** it is refused with `no relay here`, and the link is given no line (4, 222)

### Requirement: A relay gives the screen at once and again only when it changes

The core SHALL give the caller what the terminal shows of the agent at
once, then read it again at each tick of the bound the terminal's relay
states and give it only where it differs from the last given; a read
the terminal does not answer SHALL be given once as unread, until a read
answers again, and the agent's state read at once (96, 136, 221, 222).

#### Scenario: A screen given once while it stands

- **WHEN** the fake terminal shows `one` at the first three reads and `two` at the fourth, and the link ticks four times then closes
- **THEN** the link is given the screens `one` and `two`, two lines and no more (221, 222)

#### Scenario: A read the terminal does not answer

- **WHEN** the fake terminal's read is unread with `slow` at the second and third reads and answers `one` at the fourth
- **THEN** the link is given `one`, then `unread` with `slow` once, and no second screen, and the agent's state is read after the second read (96, 98)

### Requirement: A relay types the person's input as typed

The core SHALL type each input the link gives, text or keys, through the
terminal's relay as it comes, with nothing cleared first and no receipt
waited on, never through the terminal's send; an input the terminal
refuses SHALL be given back as untyped, in the terminal's words, and the
relay SHALL go on (95, 107, 221, 222).

#### Scenario: Text onto what was left typed, then enter

- **WHEN** the agent holds `draft` typed, and the link gives the text ` more` and then the key `enter`
- **THEN** the agent took `draft more` as submitted, the terminal's send was asked nothing, and no ctrl+c went before the text (95, 107, 221)

#### Scenario: An input the terminal refuses

- **WHEN** the fake terminal refuses the next input with `no such key`, and the link gives the key `f12` and then the text `yes`
- **THEN** the link is given `untyped` with `no such key`, and `yes` is typed after it (222)

### Requirement: A relay ends at its caller's close or once the agent's terminal goes

The core SHALL end a relay as the caller's when the link closes or
takes no more lines, and as gone when a read answers that the terminal
holds the agent no more, or when the agent's state, read at the first
tick of each second of the host's clock, reads stopped; its answer SHALL
name the place, the agent and which ended it (96, 120, 221).

#### Scenario: The caller closes

- **WHEN** the link closes after one tick
- **THEN** the answer is relayed, ended by the caller (221)

#### Scenario: The agent stops while its screen still reads

- **WHEN** the fake terminal reads the agent stopped once the host's clock moves a second, its screen still answering, and the link ticks, the clock moved, and ticks again
- **THEN** the relay ends gone at that tick, its answer naming the agent (96, 221)

#### Scenario: The agent closed

- **WHEN** the agent is closed while the relay runs
- **THEN** the next read answers gone, and the relay ends gone (221)

### Requirement: A relay writes, settles and keeps nothing

A relay SHALL write no held state, no host document and no file, record
no act, settle nothing and keep nothing it carried but the last screen
given, held to compare with; status and the list SHALL name no relay
(56, 120, 200, 221; core.16, core.57).

#### Scenario: Nothing written

- **WHEN** a relay types `secret-typed` and is given a screen holding `secret-screen`, then its link closes
- **THEN** the place's held state is at the version it was before, it holds no new act, the place's status answers as it did before the relay, and no document the fake store holds has `secret-typed` or `secret-screen` (56, 221)

#### Scenario: Through the binary under herdr

- **WHEN** a fix is launched on a host whose terminal is herdr, with the stand-in agent program, `flume relay fix--one` runs once the fix's agent has said it started, the text `relay-marker` is written to its input and then the input is closed
- **THEN** its first line is a screen of the agent's pane, a later line is a screen holding `relay-marker`, its last line is the relayed answer ended by the caller, it exits 0, the place's held state is at the version it was before, and no file under the host's state directory holds `relay-marker` (107, 221, 222, C.5)
