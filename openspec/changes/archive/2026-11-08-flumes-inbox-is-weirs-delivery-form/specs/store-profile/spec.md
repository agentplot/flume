## ADDED Requirements

### Requirement: The host's deliveries document in the state directory

The store profile SHALL keep the host's deliveries at `deliveries.json`
under `$XDG_STATE_HOME/flume/`, with its version, written whole by
compare-and-set under its own lock, readable by the host's user alone
and reached by every flume process on the host, its records as weir's
JSON spells them (C.7; weir 112; core.16).

#### Scenario: Seen by another process

- **WHEN** one engine over a scratch state directory records an end's delivery, and a second engine built over the same directory reads the host's deliveries
- **THEN** the second reads the delivery as written, and the file's mode is 0600 (C.7)

### Requirement: The host's span for an end

The host's configuration SHALL read `[host] ends_expire` as `keep_open`
is written, `<n>`, `<n>s`, `<n>m` or `<n>h`, from the managed layer
alone, giving a week where it is not given, refusing `0` and any other
form naming the key, and status SHALL give it with its layer (43, 56,
C.7; core.45; weir 104).

#### Scenario: A span stated

- **WHEN** the managed layer gives `[host] ends_expire = "48h"`
- **THEN** the host's span reads 172800 seconds from the managed layer, and an end read then expires 48 hours after it was read

#### Scenario: No span stated

- **WHEN** neither layer gives `ends_expire`
- **THEN** the host's span reads 604800 seconds, flume's own

#### Scenario: A span refused

- **WHEN** the managed layer gives `[host] ends_expire = "0"`, or `"2d"`
- **THEN** the configuration is refused naming `host.ends_expire`, as any `[host]` key out of its form is (4, C.7)

#### Scenario: A span in the local layer

- **WHEN** only the local layer gives `[host] ends_expire = "1h"`
- **THEN** the local layer is not taken, status naming the key, and the span reads a week (C.7; core.45)
