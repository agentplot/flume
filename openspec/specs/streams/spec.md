# streams Specification

## Purpose
The host's named streams of work: each with its own bound, priority and
pool, a place's stream resolved once at its launch from the launch, its
intent or the host's configuration, and the queue taking stages by the
stream's priority before the place's.

## Requirements

### Requirement: A host declares its streams

The host's configuration SHALL declare streams by name, each with its
bound on agents running, its priority, zero where it gives none, and
its pool. The stream `default` SHALL always stand, held by the host's
bound alone unless a `[streams.default]` table states its own. The
host's bound SHALL cap every stream's agents together (70, 143, C.7).

#### Scenario: Two streams declared

- **WHEN** the host's configuration gives `[host] agents = 8`, `[streams.work]` with `agents = 5`, `priority = 10` and `pool = "work"`, and `[streams.personal]` with `agents = 3`
- **THEN** the host reads three streams, `work`, `personal` and `default`, `personal` at priority zero and `default` bounded by the host's eight alone (143, C.7)

#### Scenario: No streams

- **WHEN** the host's configuration declares no stream
- **THEN** every place runs in `default`, bounded by `[host] agents` alone (70)

### Requirement: A place's stream is chosen once, at its launch

A place's stream SHALL be the launch's, else its intent's affinity's,
else the one the host names for its repository and kind, for its
repository, or for its kind, in that order, else `default`; it SHALL
be fixed at the launch and kept with where it came from. A launch
naming a stream the host does not declare SHALL be refused naming it,
leaving nothing made (4, 8, 143, 147).

#### Scenario: Each source in its order

- **WHEN** the host names `stream = "work"` for `[repos."/r"]` and `stream = "personal"` for `[kinds.chore]`, and chore@1 is launched in `/r` naming no stream and no intent with an affinity
- **THEN** the place runs in `work`, from `host-repository`, since the repository's word comes before the kind's (143)

#### Scenario: The launch's stream

- **WHEN** the same launch names `--stream personal`
- **THEN** the place runs in `personal`, from `launch` (143)

#### Scenario: An undeclared stream

- **WHEN** a launch names `--stream nightly` and the host declares no `nightly`
- **THEN** the launch is refused naming `nightly`, and nothing is made (4)

#### Scenario: The stream fixed at the launch

- **WHEN** the host's configuration moves `/r` to `personal` while a place of `/r` runs in `work`
- **THEN** the place's later stages still run in `work`, and a place launched after runs in `personal` (8, 143)

#### Scenario: A later place of an intent

- **WHEN** an intent's affinity stands in `personal` and a launch of the intent in `/r`, whose host's word is `work`, names no stream
- **THEN** the place runs in `personal`, from `intent` (143, 147)

### Requirement: Status gives each place's stream and each stream's queue

Status SHALL give each place's stream and where it came from, and for
each stream of the host its agents running, its bound, its room and the
stages waiting in it in the order they would start (56, 143).

#### Scenario: A stream's part of status

- **WHEN** status is asked of a host running three agents in `work`, bounded at four, with one place waiting in it
- **THEN** the host's `streams` give `work` with three running, a bound of four, a room of one, and the waiting place, and the place's own status names `work` and where it came from (56)
