## MODIFIED Requirements

### Requirement: A stream naming Rex runs its places in Rex

A stream whose `multiplexer` is `rex` SHALL have its agents opened, sent
prompts, read and closed in the Rex server of its session, a server of
flume's own, its places' clues shown on their windows and their bases'
Rex sessions, its places tidied there, and its launches and starts taken
as any stream's are (94 to 98, 137, 139, 190, C.5).

#### Scenario: A fix on a stream under Rex

- **WHEN** the host's `[streams.trial]` gives `multiplexer = "rex"`, and a fix is launched on `trial` through the binary with the stand-in for Claude Code
- **THEN** its agent opens in a block of the Rex server at `<state>/terminal/rex/trial.sock`, in its base's Rex session and the window made for `fix--<name>`; while it works, the window is labelled `fix--<name> · ` and the place's short line, and the Rex session `<repository> main · 1 running · 0 asking · 0 waiting on caller · 0 stalled`; it takes its prompt, delivers, and the fix lands; status gives the agent's terminal `rex` from `stream`, and once the place is tidied its window and its Rex session are gone and the server still runs (139, 190, S91)

## ADDED Requirements

### Requirement: A Rex stream's session is shown in the Rex app

`flume open` of a session of a stream whose `multiplexer` is `rex` and
whose view is `app` SHALL show it through the Rex profile's attach for
the app, and SHALL be refused in the profile's words where it refuses,
naming the socket a person adds under the Rex app's Remote Hosts where
the app is no client of the session's server (186, 187, C.5).

#### Scenario: A Rex stream's session with no app added

- **WHEN** the host's `[streams.trial]` gives `multiplexer = "rex"` and `view = "app"`, and `flume open trial` is run through the binary with no app a client of trial's server
- **THEN** it is refused naming `<state>/terminal/rex/trial.sock` and the Rex app's Remote Hosts, trial's server runs, and no place's held state is written (186, 187, C.5)
