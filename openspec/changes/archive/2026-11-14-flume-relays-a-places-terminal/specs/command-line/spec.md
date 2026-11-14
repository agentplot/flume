## ADDED Requirements

### Requirement: relay writes JSON lines

`flume relay <place>` SHALL name its place as status does and relay
that place's live agent through the core, writing one JSON object a
line to standard output whatever `--json` says: `{"screen": …}`,
`{"unread": …}` and `{"untyped": …}` for each line the core gives, and,
last, the core's answer as `--json` gives it, exiting 0 for a relay
ended and 1 for a refusal, which is then the only line (61, 221;
core.14).

#### Scenario: Lines out and in

- **WHEN** `flume relay fix--one` is served over the fakes, the place's code agent live, with the input lines `{"text":"yes"}` and `{"keys":["ctrl+c","enter"]}` and then the input's close
- **THEN** the first line written is `{"screen": …}` with the agent's screen, the fake terminal's relay typed the text `yes` and then the keys `ctrl+c` and `enter`, and the last line is the relayed answer ended by the caller, with exit 0 (221, 222)

#### Scenario: A refusal is the only line

- **WHEN** `flume relay` names a place whose first stage waits for room
- **THEN** the only line written is the refusal's JSON, naming the place, and the exit is 1 (4, 221)

### Requirement: relay reads JSON lines

`flume relay` SHALL read standard input one line at a time on a thread
of the command's own, typing each `{"text": …}` and `{"keys": [ … ]}` as
it comes, passing over an empty line, and writing `{"untyped": …}` for
a line that is neither or names a key flume does not name; it SHALL take
the input's close, or an output no longer read, as the caller's close
(120, 221, 222; core.13).

#### Scenario: Input flume cannot read

- **WHEN** the input holds the line `hello`, the line `{"keys":["hyper+x"]}`, an empty line and then `{"text":"ok"}`
- **THEN** an `untyped` line is written for each of the first two, the second naming `hyper+x` and flume's key names, nothing for the empty line, and `ok` is typed (222)

### Requirement: relay's help says what it speaks

`flume relay --help` SHALL say that it relays the place's live agent's
terminal for a caller carrying a person's screen and keys, that it
writes and reads JSON lines, the input forms `text` and `keys`, and that
it ends when its input closes or the agent's terminal goes; it SHALL
name no caller of flume's (221; core.35).

#### Scenario: The help

- **WHEN** `flume relay --help` runs
- **THEN** it names the input forms `text` and `keys`, says it ends when its input closes, and names no caller of flume's such as Boomdrive (core.35)
