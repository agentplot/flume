# role-waking Specification

## Purpose
A role with no session standing still hears its ends: a recipient named
by a role's address, a run of the role launched by the settle that finds
an end pending for it or by a carrier's wake, and never two runs of one
address at once (3, 43, 132, C.7, S38; record 0013; core.28).

## Requirements

### Requirement: A source declares each role's run

flume SHALL read the roles a host wakes from its sources alone, each
from the file a source's declaration names as `roles`, as it stands at
each launch, wake, start, stall and status, and SHALL write nothing in
it and choose nothing of a run of its own. A host's configuration that
gives a `[roles]` table SHALL be unreadable, naming it and that a
source declares a role (132, 191, 193, C.7).

#### Scenario: A role configured

- **WHEN** the scratch host declares the stream `east`, the scratch source `acme`'s roles file declares `east/planner` with a repository, the kind `turn` and priority 5, and `acme/turn@1` is locked
- **THEN** a wake of `east/planner` launches a place of `acme/turn@1` in that repository at priority 5, cut from the repository's default branch, with no brief

#### Scenario: A base and a brief

- **WHEN** the entry gives `east/planner` the base `next` and a brief
- **THEN** the woken place is cut from `next` and its stages that ask for the brief get it whole

#### Scenario: Read as it stands

- **WHEN** `acme`'s program rewrites its roles file, renamed over the first, to declare `east/ops` beside `east/planner`
- **THEN** the next wake of `east/ops` launches its run, with no command of flume's run between, and flume writes nothing in the file

#### Scenario: An address with no kind

- **WHEN** an end is pending for `east/ops` and no source declares it
- **THEN** nothing is launched, and status names the end pending for `east/ops` and that the address wakes nothing

#### Scenario: The roles in the host's configuration

- **WHEN** the scratch host's `config.toml` gives `[roles."east/planner"]`
- **THEN** every command of flume's says the host's configuration is unreadable, naming `[roles]` and that a source declares a role

### Requirement: A role's run is a place named per address

A role's run SHALL be a place of its declared kind and repository,
named by the entry's `name`, else `<stream>-<role>`, its agent's
session named by the address, so at most one run of an address is live
at once and two streams' runs of one role are two places. A run an
earlier flume named by the role alone SHALL be found by its role (3,
132, 193).

#### Scenario: The run's session

- **WHEN** a wake launches `east/planner`'s run, whose entry names no name, and its first stage starts
- **THEN** the place is `turn--east-planner`, the fake terminal opens the agent under the session name `east/planner`, and status names the place's role address

#### Scenario: Two streams' planners from one declaration

- **WHEN** ends come at once for `east/planner` and `west/planner`, which `acme`'s one roles file declares in one repository and the kind `turn`, named `east` and `west`
- **THEN** two places are launched, `turn--east` and `turn--west`, and each run's `flume inbox` gives its own address's ends (S99)

#### Scenario: A run an earlier flume named

- **WHEN** a run of `east/planner` stands as `turn--planner`, made by an earlier flume, and an end comes for the address
- **THEN** no run is launched, and the end is delivered into that run's pane

### Requirement: A delivery pending for an address with no run wakes the role

A settle that finds an end or a message pending for an address, handed
to it by nothing, with no run live or waiting for room SHALL launch the
address's run, inside that settle, and a message's own act SHALL do the
same; deliveries for an address whose run is live SHALL be handed to
that run's session at its next wait, as to any recipient; a delivery a
run was handed and did not confirm SHALL wake no run again (43, 132,
214, core.13, core.28).

#### Scenario: Three ends, one run

- **WHEN** a unit's place names `madswan/planner` as its recipient, its verify stops short needing an answer while no planner run is live, and two more ends for the address come before the run's agent starts
- **THEN** exactly one planner place is launched, and the run's `flume inbox` gives all three ends (S38)

#### Scenario: Two settles at once

- **WHEN** two settles each find an end pending for `madswan/planner` with no run and launch at once
- **THEN** one run is made, the other launch is refused naming it, and neither settle fails (3, 75)

#### Scenario: A run that did not confirm

- **WHEN** the planner run's `flume inbox` is handed a stop-short, the run is dropped before it confirms it, and the place is settled again
- **THEN** no planner run is launched for it, and status gives the stop-short handed to `madswan/planner` by the inbox and pending for it (S122)

#### Scenario: The next run is handed it

- **WHEN** the unit's next end comes for `madswan/planner` with no run live, and the run it wakes runs `flume inbox`
- **THEN** one run is launched, its inbox gives both ends, and once it runs `flume confirm` with both delivery keys both are delivered to the address (S122)

#### Scenario: A message wakes the role

- **WHEN** a caller messages `madswan/planner` while no planner run is live or waiting
- **THEN** the message's own act launches one planner run, and a settle that follows launches none (132, 214)

### Requirement: A carrier may wake an address

`flume wake <address>` SHALL launch the address's run where none is live
or waiting for room, and SHALL do nothing, answering so, where one is
(132).

#### Scenario: A wake while the run waits for room

- **WHEN** crew's card carrier wakes `madswan/planner` while its run waits for room
- **THEN** the wake answers that the run waits, and no second place is made (S38)

### Requirement: A role run collects for its address

`flume inbox` run in a role's run with no recipient named SHALL collect
for the run's address, its ends and its messages, and flume SHALL decide
nothing else of what the run does (43, 69, 72, 132, 214).

#### Scenario: The run starts

- **WHEN** the planner run's agent runs `flume inbox --confirm` with no recipient
- **THEN** it collects every end and every message pending for `madswan/planner`, each handed to that address by the inbox and confirmed by it

### Requirement: A role's run runs in its address's stream

A role's run SHALL be launched in the stream its address names, so the
run is placed and refused as any launch naming that stream is, and its
label SHALL be its kind's as 180 gives it. An entry SHALL name no
stream of its own (132, 143, 180, 193).

#### Scenario: A role in its stream

- **WHEN** `acme`'s file declares `east/planner`, whose kind declares the label `planning`, and an end pending for its address wakes it
- **THEN** the woken place runs in `east`, from its address, with the label `planning` from its kind (143, 180, S99)

#### Scenario: An entry that names a stream

- **WHEN** `acme`'s file gives `east/planner` `stream = "west"`
- **THEN** `east/planner` wakes nothing, and status names the entry and that its address names its stream

### Requirement: A role's kind is its source's own

A role's `kind` SHALL be one of its source's own kinds, named by its
name, bare or under its source's name, and its run SHALL be launched as
`<source>/<kind>`, resolved and locked as a launch's kind is, at the
version the locks record, carrying no declaration of its own. An entry
naming another source's kind, or a kind with a version, SHALL declare
nothing for its address (132, 154, 155, 193, C.7; core.34).

#### Scenario: A role woken to a kind of a host's source

- **WHEN** acme declares `madswan/planner` with `kind = "turn"`, acme holds `turn@1` and `turn@2`, the host's lock records both, and an end pending for the address wakes it
- **THEN** the woken place runs acme/turn@2, and status names `acme/turn@2` with the source's file (132, 154)

#### Scenario: A role's kind unlocked

- **WHEN** acme declares `madswan/planner` with `kind = "turn"`, no lock records acme's `turn`, and `flume wake` names its address
- **THEN** the wake is refused naming acme/turn as a launch of it is refused, and no place is made (4, 132, 155)

#### Scenario: A version in a role's kind

- **WHEN** acme's file gives `east/planner` `kind = "turn@1"`
- **THEN** `east/planner` wakes nothing, and status names the entry and that a role's kind is named by its name, its version the locks' (154)

### Requirement: A standing address wakes no run

A settle that finds an end pending for an address a source gives a
standing agent SHALL launch nothing for it, record nothing of it as
unwoken, and open, start and watch nothing for it, whatever the
terminal finds (132, 193; core.13, core.28).

#### Scenario: Ends pending while the agent is away

- **WHEN** three ends are pending for `madswan/planner`, whose standing agent the fake terminal holds nowhere, and the place is settled twice
- **THEN** no place is made, the fake terminal records no open, and the place's unwoken names nothing for the address (132, S38)

### Requirement: A wake of a standing address launches nothing

`flume wake <address>` of an address a source gives a standing agent
SHALL launch nothing and SHALL answer that a standing agent holds it,
with what status says of the address: its agent, its session, what the
terminal found and its pending ends (132, 193).

#### Scenario: A carrier wakes a standing address

- **WHEN** a carrier runs `flume wake madswan/planner`, whose standing agent the fake terminal holds waiting
- **THEN** the answer is held, naming `madswan-planner`, its session and the agent found waiting, and no place is made (132)

### Requirement: A source's entry decides who holds an address

Where a source's entry gives an address a standing agent, an end for it
SHALL go to that agent and not to a live run of an earlier entry; where
it gives a kind, an end SHALL wake or reach the run and not a standing
agent; each read as the source's roles file stands at the delivery (32,
132, 193).

#### Scenario: A standing agent over a run still live

- **WHEN** `madswan/planner`'s run, woken under an entry with a kind, is live, the entry then gives `agent = "madswan-planner"`, and an end for the address is read
- **THEN** the notice goes to the standing agent, and the run's session receives none (132)

#### Scenario: A standing agent's entry becomes a run's

- **WHEN** `madswan/planner`'s entry gives `agent`, its agent takes one end, the entry then gives a repository and a kind, and another end for the address is read
- **THEN** a run of the kind is woken for the second end, and the standing agent receives no second notice (132)

### Requirement: An address is a role of a stream

A role's address SHALL be `<stream>/<role>`, each part a word of
letters, digits, `.`, `_` and `-`: the role a source declares for a
stream the host declares. An entry whose address names a stream the
host does not declare SHALL declare nothing for it, unless it is
carried, and status SHALL name the address and the stream. A
recipient's name of any other form SHALL be a session's (4, 7, 132,
143, 193).

#### Scenario: A stream the host does not declare

- **WHEN** `acme`'s file declares a run for `south/planner`, and the host declares no stream `south`
- **THEN** a wake of `south/planner` is refused naming `south` as not declared, no place is made, and status names the address and the stream (4, 143)

#### Scenario: A session's name

- **WHEN** a launch names the recipient `planner`
- **THEN** the place's recipient is the session `planner`, and nothing is woken for it

#### Scenario: The wake's words

- **WHEN** `flume wake planner` runs
- **THEN** it is refused naming the form `<stream>/<role>`, and nothing is made

### Requirement: A role's entry gives its run

An entry SHALL give its address's run: a kind of its own source's, its
version the locks'; its repository by its location on the host; its
priority; its base, else the repository's default branch; and an
optional brief, name, intent, `agents`, `answers` and `answerer`. The
run's settings SHALL be taken as a launch's, in their layers. An entry
that gives `points` SHALL declare nothing for its address, since this
flume takes points for no place (7, 32, 70, 147, 155, 165, 193, C.7).

#### Scenario: The run's intent and settings

- **WHEN** the entry gives `east/planner` the intent `east-planner` and, under `agents`, `model = "opus"`
- **THEN** the woken place serves the intent `east-planner`, and its agent starts with the model `opus`, which status gives as from the launch (32, 147)

#### Scenario: Points in a role's entry

- **WHEN** `acme`'s file gives `east/planner` a run's keys and `points = 3`
- **THEN** `east/planner` wakes nothing, and status names the entry and that `points` is no word of a role's run, since this flume takes a place's points from nothing (165)

### Requirement: A source may declare a standing agent for an address

An entry MAY give `agent`, the name the host's terminal knows a standing
agent by, with an optional `session`, `answers` and `answerer`, in
place of `kind` and a run's keys. Its agent SHALL be found in the
session the entry names, else its address's stream's, in that stream's
terminal, and the address's ends SHALL go into it as notices and wake
no run (132, 190, 193, C.5, C.7; record 0039's amendment).

#### Scenario: A standing design agent

- **WHEN** `acme`'s file gives `east/design` `agent = "east-design"`, the scratch terminal holds one agent of that name in `east`'s session, and an end stands pending for `east/design`
- **THEN** the settle delivers the end into `east-design`'s pane as a notice and launches nothing

#### Scenario: A standing door

- **WHEN** `acme`'s file gives `east/operator` `agent = "east-operator"` and `answerer = true`, and a place in `east` with no recipient of its own stalls
- **THEN** the stall goes into `east-operator`'s pane as a notice, and status names `east/operator` as `east`'s answerer from `acme`

### Requirement: A role's crossing to its kind is its source's entry alone

A role SHALL move from a standing agent to a run by its source's entry
trading `agent` for `kind` and a run's keys, the next end for its
address waking a run, with nothing of the host's configuration changed
(132, 193; record 0039's amendment).

#### Scenario: The crossing to its kind

- **WHEN** `acme`'s program rewrites its file so `east/design` gives `kind = "turn"` and a repository in place of the agent, the host's configuration unchanged, and the next end for `east/design` comes
- **THEN** that end wakes a run of `acme/turn@1` in `east`, and nothing is sent to `east-design`

### Requirement: An address carried to another host

An entry MAY give `carried = true`, with an optional `answers` and
nothing else, for an address whose run is on another host, whose stream
the host need not declare. flume SHALL launch nothing for it and SHALL
keep its ends pending (132, 193).

#### Scenario: A carried address

- **WHEN** `acme`'s file gives `north/planner` `carried = true`, the host declares no stream `north`, and an end stands pending for it
- **THEN** nothing is launched, the end stays pending, and status names the address carried

### Requirement: A fault in a source's roles stays its own

A roles file that cannot be read SHALL declare no role. An entry that
names a kind not its source's, an `env`, a pool, an account, a `stream`
or `points`, whose address names a stream the host does not declare and
is not carried, that gives `agent` beside `kind` or a run's keys, or
that gives `carried` beside a run's keys, `agent` or `answerer`, SHALL
declare nothing for its address. Status SHALL name each, and every
other role and launch SHALL go on (3, 4, 56, 132, 165, 191, 193;
core.30).

#### Scenario: A roles file that is not TOML

- **WHEN** `acme`'s roles file holds a line that is no TOML, and the source `beta` declares `west/ops`
- **THEN** no address of `acme`'s wakes, status names `acme`'s file and why, and a wake of `west/ops` launches its run

#### Scenario: Another source's kind

- **WHEN** `acme`'s file gives `east/ops` `kind = "beta/turn"`
- **THEN** `east/ops` wakes nothing, and status names the entry and that its kind is not `acme`'s

#### Scenario: A standing agent beside a kind

- **WHEN** `acme`'s file gives `east/design` both `agent = "east-design"` and `kind = "turn"`
- **THEN** `east/design` wakes nothing and nothing is sent to `east-design`, and status names the entry and that it gives both

### Requirement: What two entries claim is neither's

An address two entries declare, of one source or two, and two addresses
whose runs would be one place, of one repository, kind and name, SHALL
be declared by neither: a wake of either SHALL be refused naming both,
and status SHALL name both (3, 4, 132, 193).

#### Scenario: One address, two sources

- **WHEN** `acme` and `beta` each declare `east/planner`
- **THEN** a wake of `east/planner` is refused naming both sources, and no place is made

#### Scenario: Two addresses, one place

- **WHEN** `acme`'s file declares `east/planner` and `west/planner` in one repository and one kind, both named `planner`
- **THEN** a wake of either is refused naming both addresses, and no place is made

### Requirement: A role's run is over at its delivery

Once a role's run's last stage delivers, the settle that reads it SHALL
declare the run done, with no caller's word, and tidy it, so a later end
pending for the address wakes another run. A run whose kind lands SHALL
instead be submitted as any place is, never declared done, and tidied
once it lands or is dropped (63, 114, 132).

#### Scenario: A turn delivers

- **WHEN** the run of `east/planner` delivers its one stage's report and its agent is quiet
- **THEN** the settle declares it done and tidies the place, and status gives no run of `east/planner` standing (S99)

#### Scenario: The next end

- **WHEN** an end for `east/planner` comes after that run was tidied
- **THEN** the settle that reads it launches a new run (S99)

#### Scenario: A run whose kind lands

- **WHEN** acme's `turn` lands, and the run of `east/planner` delivers its one stage's report, its agent quiet
- **THEN** the settle submits the run through the merge queue and declares nothing done, and once its submission lands the run is tidied and no run of `east/planner` stands (63, 114)

### Requirement: A role's run's stall reaches the answerer behind it

A role's run that waits on its caller, lost, never begun or stopped
short, SHALL be stalled as any place is, and its stall SHALL be
delivered to the answerer behind its stream, and never to the run's own
address (130, 132, 193).

#### Scenario: A planner's run is lost

- **WHEN** the run of `east/planner`, in the stream `east` behind which `east/operator` stands, is reported lost and nobody retries it for five minutes
- **THEN** its stall is delivered to `east/operator`, whose run is woken

#### Scenario: The door's own run

- **WHEN** the run of `east/operator`, the answerer of its own stream, is reported lost and stalls
- **THEN** its stall is delivered to no address, and status names it under the host's addresses

### Requirement: A role's run takes no notice before its prompt

While the current start of an address's run is starting, or started
with its prompt not yet received, a settle SHALL hand nothing into the
run's session, which its agent would take as its prompt, and what came
after the prompt was filled SHALL wait for the run's next wait. A
standing agent of the address SHALL be delivered to as before (37, 38,
43, 132, 214).

#### Scenario: A run not yet given its prompt

- **WHEN** a fix's end wakes `east/planner`'s run, whose agent waits in the session `east/planner` before it says it started, and the fix is settled again
- **THEN** no notice goes into the run's session, and once the agent says it started and its prompt is received the end is recorded handed to `east/planner`, carried in that prompt, and no settle sends it again (37, 38, 43)

#### Scenario: What came after the prompt

- **WHEN** an end comes for `east/planner` after its run's prompt was filled and before it was received
- **THEN** the end is handed to none until the run's agent next waits, and then goes in one prompt alone (214; record 0058)

### Requirement: A role's run's first prompt carries its batch

Each start of an address's run SHALL fill its prompt with the batch
pending at its address when the prompt is filled, after the kind's
template and before the way to stop short, claiming those deliveries
for the address under the start's identity and recording them handed
once its prompt is read received; a start read never begun, or ended
before its prompt was received, SHALL let its claims go and record
nothing (28, 37, 43, 214).

#### Scenario: The first prompt carries the batch

- **WHEN** a message and a fix's end are pending for `east/planner` when its run's start fills its prompt
- **THEN** the prompt holds the kind's template, then the message and the end, each with its confirm, then the way to stop short (28, 214)

#### Scenario: A start that never began

- **WHEN** the run's start carrying a batch is read never begun
- **THEN** its claims are let go, nothing is recorded handed, and the next start's prompt carries the same batch

### Requirement: A role's run takes nothing while it waits on an answer

While a role's run's stage ended short and waits on its answer, a
settle SHALL hand its address's batch to no one, and its take SHALL
answer nothing, so a delivery is never taken for the answer (42, 53,
214).

#### Scenario: A message while the run waits on its answer

- **WHEN** a role's run's code ended short needing an answer, its agent kept open, and a message comes for its address
- **THEN** nothing is typed into its session and the message stays handed to none; once a retry answers the stop-short and the agent goes quiet again, the next settle hands it the message

### Requirement: A launch from a role's run reports to its address

A launch that names no recipient, with `$FLUME_RECIPIENT` unset, run
where `$FLUME_PLACE` names a role's run, SHALL have that run's address
as its recipient, `recipients_from` `role-run`, ahead of the terminal's
session and the answerer behind its stream (7, 132, 214).

#### Scenario: A role's run launches its project

- **WHEN** the run of `east/ops` launches a fix naming no recipient, its environment naming the run's place, and the fix delivers
- **THEN** the fix's recipients are `east/ops` from `role-run`, and its end is pending at `east/ops` with the address's other deliveries (S128)

#### Scenario: Through MCP

- **WHEN** the same run's session calls flume's MCP `launch` for a chore naming no recipient
- **THEN** the chore's recipients are `east/ops` from `role-run` (S128)
