## ADDED Requirements

### Requirement: A base place keeps the kind of its fixes as its launch resolved it

Where a base place's kind names the kind its fixes run as, its launch
SHALL resolve that name as a launch's kind name resolves, at the base's
revision the launch read, and keep the identity it resolves to, with
where it came from, on the place, never resolving it again. A base
place whose kind names none SHALL be launched as before (19, 20, 21,
23, 154, 203; core.34).

#### Scenario: A base place whose kind names mend

- **WHEN** the repository declares `train`, landing with no stages and naming `mend` the kind of its fixes, and `mend`, landing with one code stage delivering commits, and `train--spring` is launched on `main`
- **THEN** the launch is taken, and the place holds `repo/mend@1` as the kind its fixes run as, from the repository's declaration (21, 23, 154)

#### Scenario: A declaration changed after the launch

- **WHEN** `train--spring` is launched with `mend` at version 1, the repository then declares `mend` at version 2, and a forecast conflict of `train/spring` is read
- **THEN** the fix's launch names `repo/mend@1` and is refused naming it, since the repository no longer holds it, and the base place waits on its caller with the refusal's words (21, 203)

### Requirement: A base place's launch is refused for a fix kind flume could not launch

A base place's launch SHALL be refused whole, naming its kind's fix
field and why, before the merge queue opens anything, where the name
resolves to no kind, to a kind that does not land, to a kind with no
stages, to a kind flume does not run, or to a kind that reads a change
while the repository binds no change profile (4, 23, 203; core.8).

#### Scenario: A fix kind that resolves to nothing

- **WHEN** `train` names `mend` the kind of its fixes, no declaration holds a kind `mend`, and `train--spring` is launched
- **THEN** the launch is refused naming `train`'s fix kind and that no kind `mend` resolves, the fake merge queue logs no open, and the store holds no document of the place (4, 154; core.8)

#### Scenario: A fix kind with no stages

- **WHEN** `train` names `train` itself the kind of its fixes, and `train--spring` is launched
- **THEN** the launch is refused naming `train`'s fix kind and that a kind with no stages runs no fix, and nothing is made (4, 23)

### Requirement: A settle forecasts a base place waiting on its caller

A settle of a live base place whose kind names a fix SHALL ask the
merge queue seam's forecast of its branch against its base, once, while
no place whose base is its branch is being made or live and its
submission is not open. It SHALL ask none otherwise, none for a base
place whose kind names no fix, and none at a settle of another place
(46, 202, 203, 208, 210; core.13).

#### Scenario: No forecast while a change works on its branch

- **WHEN** a change on `train/spring` is live and a settle of every place runs
- **THEN** the fake merge queue logs no forecast, and the base place waits on its places (203)

#### Scenario: No forecast while its submission is open

- **WHEN** the caller retried `train--spring`'s submission and the fake merge queue holds it under test, and a settle runs
- **THEN** the fake merge queue logs no forecast (111, 203)

#### Scenario: No forecast for a kind that names no fix

- **WHEN** `train` names no fix kind, `train--spring` waits on its caller, and a settle runs
- **THEN** the fake merge queue logs no forecast (203)

### Requirement: A base place holds the newest forecast a settle read

A settle SHALL hold the forecast it read on the base place, with when it
read it, only where its outcome, its head or the base's revision differs
from the one held. A forecast that could not be read SHALL launch
nothing and fail no settle, and the next settle SHALL ask again (17,
203, 210; core.17).

#### Scenario: Forecast while it waits on its caller

- **WHEN** `train--spring`, whose kind names `mend`, has no place on its branch and no submission, and three settles of every place run
- **THEN** the fake merge queue logs `forecast train/spring onto main` once at each settle, and the forecast `train--spring` holds is clear at `main`'s revision, read at the first settle, the two after it writing no forecast (203, 210)

#### Scenario: A forecast that could not be read

- **WHEN** the fake merge queue answers `train/spring`'s forecast could not be read, `the reader's document is torn`, and two settles run
- **THEN** each settle asks the forecast, nothing is launched, the settles fail nothing, and status gives the forecast unread with those words (203, 210)

### Requirement: A conflict with the base, or a failed submission, launches the fix

A forecast in conflict with the base itself, naming no entry ahead, and
a failed or conflicting submission of a base place whose kind names a
fix, SHALL launch its fix inside the settle that reads it, with no
caller's word, and start no agent in the base place. A forecast in
conflict only with entries queued ahead SHALL launch nothing, and the
next settle SHALL forecast again (1, 46, 112, 203, I6; core.28).

#### Scenario: A failed submission launches mend

- **WHEN** the caller retried `train--spring`'s submission and the fake merge queue's record fails it naming the check `test`
- **THEN** the settle that reads the failure delivers it once to `ops` and launches `mend--spring-1` with a brief whose first line asks it to make `train/spring` pass `main`'s checks that failed, followed by the record's words naming `test`; no agent starts in `train--spring` (43, 112, 203)

#### Scenario: A conflicting submission launches mend

- **WHEN** the fake merge queue's record finds `train--spring`'s submission in conflict on `README.md` against `main` at `c0ffee`
- **THEN** the settle that reads it launches `mend--spring-1`, its brief's first line asking it to merge `c0ffee` in resolving `README.md`, and its words naming both (112, 203)

#### Scenario: A conflict with an entry queued ahead

- **WHEN** the fake merge queue forecasts `train/spring` in conflict on `README.md` with the entry `#7` queued ahead of it, and a settle runs, then `#7` lands, the fake forecasts `train/spring` in conflict with `main` on `README.md`, and another settle runs
- **THEN** the first settle launches nothing and status gives the forecast in conflict with `#7`; the second launches `mend--spring-1` (203; sluice 65)

### Requirement: A base place's fix is launched as its base place's

A fix SHALL be a place of the identity its base place resolved, on the
base place's branch, named by the base place's name and the fix's
number from 1 joined by `-`, in its stream, at its priority, serving the
intent its launch named, with its recipients and stall bound, and no
settings, holds, mark or points of its own. It SHALL hold the base
place, its number and its cause, and run as any place does (9, 15, 43,
70, 143, 147, 165, 202, 203).

#### Scenario: A forecast conflict launches mend

- **WHEN** `train--spring` waits on its caller with its recipient `ops` at priority 4 in the stream `east`, the fake merge queue forecasts `train/spring` in conflict with `main` at `c0ffee` on `README.md`, naming no entry ahead, and a settle of every place runs
- **THEN** that settle launches `mend--spring-1` on the base `train/spring`, its branch `mend/spring-1`, in `east` at priority 4, its recipients `ops` from its base place; the fix holds `train--spring`, fix 1 and the forecast as its cause; and no caller's command ran (203, S120)

### Requirement: A fix's brief carries the merge queue's words

A fix's brief SHALL be one line saying what the fix is for, a blank
line, and the merge queue's words whole. For a conflict, forecast or
found, the line SHALL ask the fix to merge the base's revision the words
name into the branch and commit it, resolving the files contested,
before anything else; for a failure, to make the branch pass the checks
that failed (43, 112, 203; core.5, core.36).

#### Scenario: The brief of a forecast conflict's fix

- **WHEN** a forecast conflict of `train/spring` with `main` at `c0ffee` on `README.md` launches `mend--spring-1`
- **THEN** its brief's first line asks it to merge `main` at `c0ffee` into `train/spring` and commit it, resolving `README.md`, before anything else, and the merge queue's words follow, naming `README.md` and `c0ffee` (112, 203)

### Requirement: A base place's fixes run one at a time

While a fix of a base place is live, a settle SHALL launch no second fix
for it and ask no forecast. Once the fix lands, the base place SHALL be
forecast at its next settle, and SHALL wait on its caller where that
forecast is clear, its submission still its caller's retry (111, 203;
core.6).

#### Scenario: The fix lands and the forecast is clear

- **WHEN** `mend--spring-1` runs, lands on `train/spring`, and is tidied, the fake merge queue then forecasts `train/spring` clear, and the caller retries `train--spring`'s submission, which lands
- **THEN** no forecast was asked while `mend--spring-1` was live, the settle after its landing asked one, `train--spring` waited on its caller once it was clear, and the retry submitted `train/spring` named, which landed (111, 203, S120)

### Requirement: Each forecast and each failure launches one fix at most

A forecast whose head and base's revision are a fix's cause, or a
failure whose submission is one, SHALL launch nothing more. A fix SHALL
be written onto its base place by compare-and-set before it is launched;
a settle that finds a fix written with no place standing SHALL launch
it, and a launch refused because the fix's place now stands SHALL take
that place as the fix (3, 17, 74, 75, 203).

#### Scenario: Two settles at once

- **WHEN** two settles of every place run side by side and each reads the same forecast conflict of `train/spring`
- **THEN** one fix, `mend--spring-1`, is launched, and `train--spring` holds one fix (17, 75)

#### Scenario: A launch that died between its writing and its making

- **WHEN** a settle wrote fix 1 onto `train--spring` and its command died before it launched, and a settle runs
- **THEN** that settle launches `mend--spring-1`, and `train--spring` holds one fix (17, 74)

### Requirement: A dropped fix leaves its base place waiting on its caller

Once a fix is dropped, its base place SHALL wait on its caller with
words naming the fix and that it was dropped, and SHALL launch no fix
again until its branch's head or its base's revision moves (130, 203).

#### Scenario: A fix dropped

- **WHEN** the caller drops `mend--spring-1`, and three settles run while the fake merge queue forecasts the same head of `train/spring` in conflict with `main` at `c0ffee`
- **THEN** no second fix is launched, and `train--spring` waits on its caller with words naming `mend--spring-1` and that it was dropped; when `main` moves to `d00d` and the forecast conflicts there, the next settle launches `mend--spring-2` (130, 203)

### Requirement: A base place's fixes count toward its kind's bound

Each fix of a base place, by forecast and by failure together, SHALL
count toward its kind's bound, flume's default for a stage's where the
kind gives none, and the count SHALL never be reset. A conflict or a
failure read with the bound spent SHALL launch nothing: the base place
SHALL wait on its caller with the merge queue's words and the bound
spent, stalling and delivered as any wait on its caller is (23, 43,
58, 130, 131, 203).

#### Scenario: The bound spent

- **WHEN** `train` names `mend` with a bound of 1, `mend--spring-1` has landed, the fake merge queue forecasts `train/spring` in conflict with `main` at a new revision, a settle runs, and six minutes pass
- **THEN** no fix is launched, `train--spring` waits on its caller with the merge queue's words and that its kind's 1 fix was launched, as many as it allows, it is stalled, and `ops` is delivered the stall once (43, 130, 203)

### Requirement: A fix whose launch is refused counts and leaves its base place waiting

A fix whose launch is refused SHALL count toward its kind's bound and be
held refused with the refusal's words, and its base place SHALL wait on
its caller with them (3, 4, 203).

#### Scenario: A refused launch

- **WHEN** the branch `mend/spring-1` already stands, and a forecast conflict of `train/spring` is read
- **THEN** the launch of `mend--spring-1` is refused naming the branch, `train--spring` holds fix 1 refused with those words and waits on its caller with them, and the next settle, reading the same forecast, launches nothing more (3, 203)

## MODIFIED Requirements

### Requirement: A base place's failed submission waits on its caller

A failed or conflicting submission of a base place SHALL start no stage
and no agent in it. Where its kind names no fix, the place SHALL wait on
its caller with the merge queue's words, delivered to its recipients,
and a fix launched on its branch SHALL be taken while it so waits;
where its kind names one, the fix SHALL be launched as a conflict with
the base launches it (43, 112, 203, 204).

#### Scenario: A failed submission, a fix, a retry

- **WHEN** `train` names no fix kind, the fake merge queue's record fails `train--spring`'s submission naming the check `test`, the caller launches a fix with `--base train/spring` and those words as its brief, the fix lands on `train/spring`, and the caller retries `train--spring`'s submission, which lands
- **THEN** no agent started in `train--spring`, its status gave the words while it waited, its recipient was delivered the failure once, and its work is over once it lands (112, 203, S112)

#### Scenario: A conflicting submission

- **WHEN** `train` names no fix kind, and the fake merge queue's record finds `train--spring`'s submission in conflict on `README.md` against `main` at `c0ffee`
- **THEN** the place waits on its caller, its words naming `README.md` and `c0ffee`, and no stage starts (112, 203, S112)
