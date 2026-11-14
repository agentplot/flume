## ADDED Requirements

### Requirement: The fake terminal relays an open agent

The fake terminal SHALL relay an open agent every 250 milliseconds: its
screen the one `set_screen` gives the agent followed by what the agent
holds typed, text added to what it holds typed with nothing cleared,
`enter` submitting what it holds typed, and a closed or killed agent
read gone; and SHALL log each relay call, never what was typed (106,
221, 222).

#### Scenario: A relay of the fake

- **WHEN** a test sets an open agent's screen to `ready`, relays it, reads, types the text `hi`, reads, types the key `enter` and reads
- **THEN** the first read gives `ready`, the second holds `hi`, the agent's submitted inputs end with `hi`, and the fake's log names `relay`, each read and `relay text` and `relay keys enter` without `hi` (222)

### Requirement: The fake terminal scripts its relay

The fake terminal SHALL script a refusal of relay, a sequence of
screens an agent's reads give, a read unread with words, and the next
input refused with words (106, 222).

#### Scenario: Scripted refusals

- **WHEN** a test scripts the fake terminal to refuse relay with `no relay here`, then to answer it, its reads with the screens `one` and `two`, its next read unread with `slow` and its next input refused with `no such key`
- **THEN** the first relay is refused with `no relay here`, the second's reads give `unread` with `slow`, `one` and `two` in turn, and its first input is refused with `no such key` and its second typed (222)
