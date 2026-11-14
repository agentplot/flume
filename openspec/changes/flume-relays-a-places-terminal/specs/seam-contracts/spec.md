## ADDED Requirements

### Requirement: The terminal seam's relay

The terminal seam SHALL offer relay: given an agent, a handle, or a
refusal in the terminal's words. The handle SHALL state the bound within
which what the terminal shows is read again; read what the terminal
shows of the agent as a person's view would, or that it holds the agent
no more, or its words where it did not answer; and type a person's
input with nothing cleared first and no wait for receipt. Dropping it
SHALL end nothing in the terminal (107, 221, 222).

#### Scenario: The relay among the terminal's scenarios

- **WHEN** the terminal seam's scenarios run against a terminal that relays
- **THEN** the relay of an open agent gives a screen; text typed onto what the agent holds typed is not cleared first and `enter` submits both together; typed text shows on a screen read within ten seconds; and once the agent is closed a read answers that the terminal holds it no more (107, 136, 221, 222)

#### Scenario: The relay refused among the terminal's scenarios

- **WHEN** the terminal seam's scenarios run against a terminal that offers no relay
- **THEN** its relay of an open agent is refused with words (222)

### Requirement: A relay refuses a key its terminal cannot type

A relay's handle SHALL refuse a key its terminal cannot type, naming
it, with nothing typed for it, and SHALL type the keys after it (222).

#### Scenario: A key the terminal cannot type

- **WHEN** a relay is given keys one of which its terminal cannot type
- **THEN** the input is refused naming that key, nothing is typed for it, and the keys before and after it are typed in order (222)

### Requirement: A terminal that builds no relay refuses it

A terminal that builds no relay SHALL refuse every relay with `the
terminal offers no relay`, the seam's default (222).

#### Scenario: A terminal that builds no relay

- **WHEN** a terminal implementing every other operation of the seam is asked to relay an agent
- **THEN** it is refused with `the terminal offers no relay` (222)

### Requirement: A person's input is text or keys

The seams SHALL carry a person's input as `Typed`, text or keys, read
from and written as `{"text": …}` and `{"keys": [ … ]}`, each key by
flume's name (222).

#### Scenario: Input read and written back

- **WHEN** `{"text":"yes"}` and `{"keys":["ctrl+c","enter"]}` are read as `Typed` and written back
- **THEN** each is written as it was read (222)

### Requirement: flume names a person's keys

The seams SHALL carry a key as `Key`, read from and written as flume's
name for it: `enter`, `tab`, `esc`, `backspace`, `delete`, `insert`,
`home`, `end`, `pageup`, `pagedown`, `up`, `down`, `left`, `right`,
`space`, `f1` to `f12`, or one printable character, each led by any of
`ctrl+`, `alt+` and `shift+` in that order, `escape` read as `esc`; any
other name SHALL be refused, naming it and flume's names (222).

#### Scenario: Key names

- **WHEN** `ctrl+c`, `shift+tab`, `escape`, `f12`, `ctrl+alt+x`, `é` and `space` are read as keys, and then `alt+ctrl+x`, `f13` and `hyper+x`
- **THEN** the first seven are read and written back as `ctrl+c`, `shift+tab`, `esc`, `f12`, `ctrl+alt+x`, `é` and `space`, and each of the last three is refused naming it and flume's names (222)
