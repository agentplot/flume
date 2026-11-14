## ADDED Requirements

### Requirement: A stream naming Rex runs its places in Rex

A stream whose `multiplexer` is `rex` SHALL have its agents opened, sent
prompts, read and closed in the Rex server of its session, a server of
flume's own, its places tidied there, and its launches and starts taken
as any stream's are (94 to 98, 137, 190, C.5).

#### Scenario: A fix on a stream under Rex

- **WHEN** the host's `[streams.trial]` gives `multiplexer = "rex"`, and a fix is launched on `trial` through the binary with the stand-in for Claude Code
- **THEN** its agent opens in a block of the Rex server at `<state>/terminal/rex/trial.sock`, in the Rex session `<repository> main` and the window `fix--<name>`, takes its prompt, delivers, and the fix lands; status gives the agent's terminal `rex` from `stream`, and once the place is tidied its window and its Rex session are gone and the server still runs (190, S91)

### Requirement: A stream whose multiplexer flume carries no profile for is refused alone

A stream whose `multiplexer` names a terminal profile the resolver does
not carry SHALL refuse a launch on it whole and each start of a place
on it, naming the stream, the name and the profiles carried, and
nothing else on the host; status SHALL give its refusal with the stream
(4, 190; core.8).

#### Scenario: A stream naming cmux

- **WHEN** the fake host's `trial` names the multiplexer `cmux`, and a fix is launched on `trial` and one on `work`
- **THEN** the trial fix is refused naming `trial`, `cmux` and the profiles carried, nothing made, the work fix opens its agent, and status gives `trial`'s refusal (4, 190, S91)

#### Scenario: A stream changed to cmux while it waits

- **WHEN** a place on `trial` waits for room, and `trial` is then set to the multiplexer `cmux`
- **THEN** its start is refused naming `cmux` as a setting is, and nothing opens (4, 190)

## REMOVED Requirements

### Requirement: A stream whose multiplexer names no profile is refused alone

**Reason**: Replaced by "A stream whose multiplexer flume carries no profile for is refused alone": flume carries a profile for Rex, so its scenarios name `cmux`, as S91 does.

**Migration**: None: a stream naming a multiplexer flume carries no profile for is refused as before.
