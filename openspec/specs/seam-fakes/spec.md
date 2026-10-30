# seam-fakes Specification

## Purpose
One scriptable fake per seam, and for git's history and the host, so
the engine and every scenario of the suite run without a tool, and each
fake is held to its seam's contract as a profile is (106).

## Requirements

### Requirement: One scriptable fake per seam

There SHALL be one fake for each of the seven seams, and one each for
git's history and the host. A test SHALL set what each fake answers,
including each refusal and each unknown its seam may give, and SHALL
read back every call the core made of it, in order.

#### Scenario: A scripted refusal

- **WHEN** a test scripts the fake place to refuse `remove` naming a teardown step
- **THEN** the core's tidy keeps the place naming that step (66), and the fake's log shows the one `remove` call

#### Scenario: An unknown answer

- **WHEN** a test scripts the fake agent to answer own-work as unknown
- **THEN** the core reads the agent as not quiet and status names what it cannot read (31)

### Requirement: Each fake passes its seam's scenarios

Each fake SHALL pass every scenario of its seam in the suite, the same
scenarios a profile unit runs against its real tool, so a fake never
admits what a profile may not (106).

#### Scenario: The fake place's guarantee

- **WHEN** the place scenarios run against the fake place
- **THEN** make refuses an existing branch and an existing location and touches neither, and every operation is all or nothing (83)

#### Scenario: The fake store's compare-and-set

- **WHEN** the store scenarios run against the fake store
- **THEN** a write at a version that has moved is refused, and a first write for a place that has held state is refused (103)

### Requirement: The fake clock moves only when told

The fake host's clock SHALL move only when a test advances it or the
core waits on it, so a bound of 60 seconds, ten minutes or a stage's
quiet limit runs in a test without real time passing.

#### Scenario: A quiet limit passes

- **WHEN** a test advances the fake clock past a stage's limit with its agent quiet and nothing delivered
- **THEN** the next settle reports the stage stuck, with no real wait (44)

### Requirement: The fake terminal holds recipients' sessions

The fake terminal SHALL hold sessions by name, each with a settable
state, SHALL record every text sent to each in order, and SHALL answer a
name it holds no session for as not found, so the suite can script a
recipient away, asking, or restarted (43, 95).

#### Scenario: A recipient scripted away and back

- **WHEN** a test removes the session `ops` from the fake terminal, settles a place whose end is for `ops`, then adds `ops` back and settles again
- **THEN** the fake records no notice to `ops` from the first settle and one from the second

### Requirement: The fakes script streams, pools, spent accounts and unsettled starts

The fake host SHALL script streams, pools and their accounts; the fake
agent a usage limit with or without a reset and a start that began
without its settings; and the fake store the host's own documents; each
passing its seam's scenarios, and no fake SHALL reach a real account or
configuration directory (106).

#### Scenario: A scripted usage limit

- **WHEN** a test scripts the fake agent's start to stop at a usage limit resetting at 15:00
- **THEN** the core records its account spent until 15:00 and the fake's log shows the `spent` reading (146, 151)
