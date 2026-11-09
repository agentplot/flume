# end-delivery Specification

## Purpose
How whoever a place's work is for learns how it ended with no polling and
nothing lost to a restart: each end delivered once into its recipient's
pane and recorded, pending until then, waited on by place and collected
by `flume inbox` (43, 46, 119, 120, 122; record 0009; core.25).

## Requirements

### Requirement: The settle delivers each end once to each recipient

The settle that reads a stage's end, a stage lost, never begun or
stuck, a submission's outcome or a stall SHALL record it on the place
as an end, with what flume read, and record its delivery to each of its
recipients, as "An end's recipients" gives them; and SHALL hand it to
each recipient once, as a notice through the terminal into the
recipient's own pane, recorded as a hand-over with to whom and when,
the end pending until the recipient confirms it (7, 43, 46, 95).

#### Scenario: A delivery and its record

- **WHEN** a fix's code stage delivers at a head and the settle reads it, with the session `ops` waiting in the fake terminal
- **THEN** the fake terminal records one notice to `ops` naming the place and the end, and status gives the end handed to `ops` by notice with when, still pending for `ops`

#### Scenario: Settled twice

- **WHEN** the same place is settled again after the end was handed over
- **THEN** the fake terminal records no second notice to `ops` (74)

#### Scenario: An agent killed

- **WHEN** a code stage's agent is killed with nothing delivered and the watch's settle reads the stage lost
- **THEN** each recipient gets one notice that the stage is lost, and the end carries what flume read of it (43, 45, S28)

#### Scenario: A landing and a failure

- **WHEN** the merge queue's inbox gives the place's submission failed, and later landed
- **THEN** each outcome is an end handed once to each recipient, the failure with the merge queue's words and the landing with the commit it landed as (43, 133)

### Requirement: A notice is never sent into a question or left typed

A notice SHALL name the place, the end, the command that reads it and
the delivery's key. It SHALL be sent only while the terminal reads the
recipient asking no question, neither asking nor unknown, and SHALL
count handed over on the terminal's answer sent, with no reading back,
delivering nothing; a send refused, or a recipient not found, SHALL
record no hand-over (43, 95, core.11).

#### Scenario: A recipient asking

- **WHEN** an end is read while the recipient's session is asking a person a question
- **THEN** no notice is sent, no hand-over is recorded, and the first settle that reads the session not asking sends it

#### Scenario: A recipient gone

- **WHEN** the fake terminal finds no session by the recipient's name
- **THEN** no hand-over is recorded, the end stays pending, and the settle goes on with the place's other steps

### Requirement: The printed command delivers nothing

Where the terminal has no pane to deliver to, no notice SHALL be sent,
every end SHALL stay pending until its recipient confirms it, and status
SHALL give it (43, C.5).

#### Scenario: Under the printed command

- **WHEN** a stage ends under a fake terminal that refuses send
- **THEN** status gives the end pending for each recipient, handed to none, and the fake terminal records no send

### Requirement: A wait is by place

`flume wait` SHALL name a place and its stage or its submission, and the
recipient it waits as. It SHALL answer at once with an end of that stage
or submission pending for the recipient, recording a hand-over by the
wait, and confirming it where `--confirm` is given; answer one the
recipient confirmed as a read, handing nothing over; and otherwise
follow the host's state until such an end, or exit still waiting when
its timeout passes (43).

#### Scenario: A restarted recipient waits again

- **WHEN** a recipient's session restarts while waiting on a place's submission, the submission lands, and the recipient runs the same wait
- **THEN** the wait answers at once with the landing and its commit, and the end reads handed to that recipient by the wait and still pending for it

#### Scenario: A wait that confirms

- **WHEN** the recipient runs the wait again with `--confirm`
- **THEN** it answers with the landing, and the end reads confirmed by that recipient, its hand-over and its confirmation recorded together (weir 103)

#### Scenario: The timeout

- **WHEN** a wait with a timeout of one minute finds nothing pending and the place's stage still runs after a minute
- **THEN** the wait exits still waiting, with its own exit, and records no hand-over

### Requirement: A wait drives no settle while a caller keeps watch

While a caller keeps watch over the host, a wait SHALL settle nothing
and read only the host's state. While none does, it SHALL settle every
place before each reading, as every acting command does, whatever
terminal the host binds (119, 122, 208; core.13).

#### Scenario: Under a watch

- **WHEN** a wait follows a place while a caller keeps watch
- **THEN** the wait makes no call to any seam but the store, and the place moves on only by the caller's settles

#### Scenario: Attended under any terminal

- **WHEN** a wait follows a place while no caller keeps watch
- **THEN** the wait settles every place before each reading (119, 208)

### Requirement: The inbox collects what is pending

`flume inbox` SHALL give every end pending for a recipient, of every
place on the host, with what 43 names, its key and its delivery's key,
whether expired, and SHALL record a hand-over of each to that recipient,
confirming each where `--confirm` is given. Status, a wait's read and an
inbox for another recipient SHALL confirm nothing (43, core.25).

#### Scenario: Collected after a restart

- **WHEN** two ends of two places are pending for `ops` while its session is gone, and its restarted session runs `flume inbox` as `ops`
- **THEN** the answer holds both ends whole, each with its key and its delivery's key, each handed to `ops` by the inbox, and a second `flume inbox` gives both again

#### Scenario: Collected and confirmed

- **WHEN** `ops` runs `flume inbox --confirm`
- **THEN** the answer holds both ends, each confirmed by this call, and a third `flume inbox` gives nothing

### Requirement: A lost conversation's end says flume left an agent in its pane

The end of a conversation's stage read lost while an agent the
terminal brought back stands in its pane SHALL say that flume left that
agent in its pane, in the end held on the place and in its notice to
each recipient (43, 77).

#### Scenario: A lost talk with a bare agent

- **WHEN** after a rebuild the fake terminal brings a talk's agent back bare, with the session `ops` waiting, and the settle reads talk lost
- **THEN** the end gives `left_in_pane`, and the fake terminal records one notice to `ops` naming talk lost and that flume left the agent in its pane (43, 77, S113)

#### Scenario: A lost talk with no agent

- **WHEN** a talk's agent is killed and the settle reads talk lost
- **THEN** the end gives no `left_in_pane`, and the notice says nothing of an agent left (43, 45)

### Requirement: An end for a standing address goes into the agent found by its name

At each settle that reads an end pending for a standing address and
handed to it by nothing, the terminal of its address's stream SHALL be
asked for the session's agents, and the end sent through it as a notice
to the one agent of the entry's name while it asks no question; none,
more than one, or a part not read SHALL leave it handed to none; nothing
of where the agent was SHALL be kept (43, 132, 190, 193, C.7; core.11,
core.28).

#### Scenario: A standing agent waiting

- **WHEN** a fix names `madswan/planner` as its recipient, the entry gives `agent = "madswan-planner"` and `session = "madswan"`, the fake terminal holds that agent waiting in `madswan`, and the fix's code stage delivers
- **THEN** the agent receives one notice naming the place and the end, the end is recorded handed to `madswan/planner` by notice and pending for it, and no place is made (43, 132)

#### Scenario: A standing agent asking

- **WHEN** the standing agent is asking when the end is read, and later reads waiting
- **THEN** no notice is sent while it asks, the end is handed to none, and the first settle that reads it waiting sends it once (43; core.11)

#### Scenario: No agent of the name

- **WHEN** the fake terminal holds no agent named `madswan-planner` in `madswan` when the end is read
- **THEN** the end stays handed to none, nothing is claimed, and the settle goes on with the place's other steps (132)

#### Scenario: Two agents of the name

- **WHEN** the fake terminal holds two agents named `madswan-planner` in `madswan`
- **THEN** neither receives a notice, and the end stays handed to none (132)

#### Scenario: An agent revived under its name

- **WHEN** the standing agent takes one end, is removed, and another agent named `madswan-planner` is added in `madswan` under a new session key before the next end is read
- **THEN** the next notice reaches the new agent, and the removed one receives no second notice (132)

#### Scenario: The session from the entry's stream

- **WHEN** the entry gives `w/planner` `agent = "w-planner"` and no session, the stream `w` names `session = "w-main"`, and the fake terminal holds `w-planner` in `w-main`
- **THEN** the notice reaches it, asked of `w`'s terminal in `w-main` (143, 190, 193)

#### Scenario: The default stream's own session

- **WHEN** the entry gives `default/planner` only `agent = "w-planner"`, the default stream names no session, and the fake terminal holds `w-planner` in its own part
- **THEN** the notice reaches it, asked of the terminal's own part (107, 143, 193)

#### Scenario: A stream the host does not declare

- **WHEN** the entry gives `gone/planner` an agent, and the host declares no stream `gone`
- **THEN** no terminal is asked, and the end stays handed to none (143, 193)

#### Scenario: The inbox collects for a standing address

- **WHEN** an end is pending for `madswan/planner`, whose agent is away, and `flume inbox --recipient madswan/planner` runs
- **THEN** it collects the end as for any recipient, handed to the address by the inbox and pending for it, and the next settle sends no notice of it (43)

#### Scenario: A standing agent under herdr through the binary

- **WHEN** under the pinned herdr and the stand-in, a fix names `w/planner` as its recipient, a scratch source's roles file gives it `agent = "w-planner"` and `session = "w"`, the host declares the stream `w`, and a stand-in runs in `w` under the herdr name `w-planner`
- **THEN** the fix's end reaches that pane as one notice, and no place of a planner is made (43, 132, 193, C.5)

### Requirement: A stop-short's end says until when its agent is kept open

The end of a stage ended short needing an answer SHALL carry, where its
agent is kept open for the answer, the time it is kept open until, in
the JSON form and in the notice's words (43, 198).

#### Scenario: The notice of a question

- **WHEN** a code agent stops short needing an answer at a time t, on a terminal that hands a running agent words
- **THEN** the end delivered to each recipient gives its agent kept open until t plus its `keep_open`, and the notice's words say so (43, 198)

### Requirement: A wait after an answer waits on the next end

A wait on a stage SHALL answer only an end of its current start read
after the last answer that start took in its session, so a wait after
an answer never answers the stop-short it answered (43, 198).

#### Scenario: Waiting again after an answer

- **WHEN** a caller's wait on code delivered its stop-short, the caller answered it in its session, and the caller waits on code again
- **THEN** the wait does not answer the stop-short as a read, and answers code's next end once it is read (43)

### Requirement: An end's recipients

An end's recipients SHALL be its place's, as its launch took them. A
stop-short's that named a role SHALL be that address alone, not the
place's. A stall's SHALL be the place's, the address the stop-short it
waits on named, of whichever stream, and the answerer behind the
place's own stream, each once. A drop's or a done's SHALL be the
place's that are no role's address (7, 41, 43, 130, 193).

#### Scenario: A stop-short that named a role

- **WHEN** a construct of a place in `east`, whose recipient is `east/operator`, stops short naming `ops`
- **THEN** the end stands pending for `east/ops` alone, and none of it for `east/operator` (41, S93)

#### Scenario: A stall of a question for another stream

- **WHEN** a construct of a place in `east`, whose recipient is `east/planner`, stopped short naming `west/design`, `west/operator` stands behind `west`, and the place stalls
- **THEN** the stall is delivered to `east/planner`, `west/design` and `east/operator`, once each, and not to `west/operator` (130, 193)

### Requirement: The inbox gives what waits on its recipient

`flume inbox` SHALL also give, as `waiting`, every place that waits on
its caller and that the recipient is a recipient of, was named by the
stop-short the place waits on, or was given a stall of as the answerer,
pending or not, with its kind, name, stream and brief's first line,
the move and since when, a stop-short's words, needs, key and the
address it named, and a hold's words, since and bound. It SHALL confirm
nothing for them (43, 130, 193; core.25).

#### Scenario: A question collected, then a restart

- **WHEN** `ops` collected a construct's stop-short by `flume inbox --confirm`, did nothing with it, and its restarted session runs `flume inbox` again
- **THEN** `ends` is empty, and `waiting` gives the place with the retry of construct as its move, since the stop-short, the agent's words and the key `stage/construct/1/ended-short`

#### Scenario: A question addressed to a role

- **WHEN** a place whose recipient is `east/planner` waits on a stop-short that named `east/ops`, and `east/ops`'s run runs `flume inbox`
- **THEN** `waiting` gives that place, naming `east/ops` as the stop-short's addressee

#### Scenario: The answerer's waiting places

- **WHEN** a pane-launched place stalled and its stall was handed to the answerer `east/operator`, and the answerer's run runs `flume inbox`
- **THEN** `waiting` gives that place, though `east/operator` is not among its recipients (193)

#### Scenario: A held question

- **WHEN** a recipient holds the waiting move with words and a bound of one hour
- **THEN** `waiting` gives the place with the hold's words, since when and its bound, and the next `flume inbox` gives it the same

### Requirement: A notice names the place's brief and the end's key

A notice SHALL stay one line, the end's message as its delivery
recorded it, and SHALL name the place, the first line of its brief cut
at 60 characters, the end, its key, the command that reads it whole,
`flume inbox` for an end the place waits on a move for and `flume
status <place>` for any other, and `flume confirm` with the delivery's
key (43; core.11, core.25).

#### Scenario: A stop-short's notice

- **WHEN** a change whose brief begins "Every question a place asks reaches someone who answers it" ends short needing an answer at construct, and the settle hands it to a pane
- **THEN** the fake terminal records one line naming the place, "Every question a place asks reaches someone who answers it" cut at 60 characters, "construct needs an answer", `stage/construct/1/ended-short`, `flume inbox` and `flume confirm` with the end's delivery key (S94)

#### Scenario: A delivery's notice

- **WHEN** the same place's construct delivers instead
- **THEN** the notice names `flume status`, the delivery's key and `flume confirm` with its delivery key

### Requirement: An inbox naming a place

An inbox that names a place SHALL give and hand over only the ends of
the documents `flume status` reads under that name, the place's own,
else those kept for it once it was tidied, or the one document
`<place>@<at>` names, and SHALL give only that place's entry of the
places that wait on the recipient. It SHALL be refused, naming the
place and handing nothing over, where no document stands under that
name (43, 62, core.25).

#### Scenario: An inbox naming a place

- **WHEN** two fixes launched with the recipients `ops` and `boomdrive` have landed and been tidied, no pane holding `boomdrive`, and `boomdrive` runs `flume inbox --confirm` naming the first
- **THEN** the answer holds the first's ends alone, its code's end and its landing, each confirmed by `boomdrive`; the second's stay pending for `boomdrive` and its document stays (43, 62)

#### Scenario: A name under which nothing stands

- **WHEN** an inbox names a place under which no document stands on the host
- **THEN** it is refused naming the place, and nothing is handed over (43)

### Requirement: A recipient no pane holds collects its ends

A recipient that no pane holds SHALL be sent no notice, and each end
pending for it SHALL stay pending, keeping its tidied place's document,
until it confirms the end, by `flume confirm` or by `flume inbox` or a
wait as itself with `--confirm` (43, 62).

#### Scenario: A landing for a recipient no pane holds

- **WHEN** a fix launched with the recipients `ops` and `boomdrive` lands while `ops` waits in the fake terminal and no pane holds `boomdrive`
- **THEN** `ops` gets one notice of the landing and no notice goes to `boomdrive`; the place is tidied, its document staying with code's end and the landing pending for `ops`, handed to it by notice, and for `boomdrive`, handed to none; and once `ops` confirms them and `boomdrive` confirms them by `flume inbox --confirm` naming the place, the next settle removes the document (43, 62)

### Requirement: A drop or a done is an end no notice carries

A drop, and a done of a place whose kind does not land, SHALL record an
end of the place, keyed `over/dropped` or `over/done`, in the write that
makes its work over. No notice of it SHALL be sent and no role SHALL be
woken for it: its delivery's recipients SHALL be the place's that are no
role's address, and it SHALL have no delivery where every recipient is
one (43, 52, 114, 132).

#### Scenario: A drop for a role and a reader

- **WHEN** the caller drops a live fix launched with the recipients `east/planner`, which the scratch source `acme` declares with a run, and `boomdrive`
- **THEN** status of the tidied document gives the end `over/dropped` pending for `boomdrive` alone; no notice is sent and no run of `east/planner` is launched; and the document stays until `boomdrive` confirms the end (43, 52, 62)

#### Scenario: A drop for a role alone

- **WHEN** the caller drops a live fix launched with the recipient `east/planner` alone, which `acme` declares with a run
- **THEN** the end `over/dropped` is recorded with no delivery, no notice is sent, no run of `east/planner` is launched, and the tidy keeps no document of the place (43, 52, 62, 132)

#### Scenario: A drop for a session in a pane

- **WHEN** the caller drops a live fix launched with the recipient `ops` while `ops` waits in the fake terminal, every earlier end confirmed
- **THEN** `ops` gets no notice of the drop, the end `over/dropped` stays pending for `ops`, and the document stays until `ops` confirms it (43, 52, 62)

#### Scenario: A done

- **WHEN** a place of a kind that does not land, launched with the recipient `boomdrive`, is declared done
- **THEN** the end `over/done` stands pending for `boomdrive`, and the tidied document stays until it is confirmed (43, 114)

### Requirement: A tidied place's document stays while an end is pending for a recipient

A place whose work is over SHALL be tidied as always, its worktree and
branch removed (66, 67), and while an end of it stands pending for a
recipient its document alone SHALL stay, naming the recipients it waits
on, until every end is delivered to each; a drop of it SHALL be refused
then, naming each. A launch of the same name SHALL not be refused for
it (3, 43, 62; weir 107, I9).

#### Scenario: A landing with its recipient away

- **WHEN** a place lands while its one recipient's session is gone
- **THEN** its worktree and branch are removed, status gives the tidied place pending for that recipient, and once the recipient confirms its ends the next settle removes the document and lets go of their deliveries

#### Scenario: Dropped while pending

- **WHEN** the caller drops a tidied place whose end stands pending for `boomdrive`
- **THEN** the drop is refused naming `boomdrive` and `flume confirm`, and the document and its pending ends stay (62; weir I9)

#### Scenario: Launched again

- **WHEN** a place of the same repository, kind and name is launched while the tidied document stays
- **THEN** the launch makes its place, and the tidied document's ends stay pending as they were

#### Scenario: Kept for a recipient no pane holds

- **WHEN** a place launched with the recipients `ops` and `boomdrive` lands, `ops` has its notices and confirms them, and no pane holds `boomdrive`
- **THEN** status gives the tidied place pending for `boomdrive` (62)

### Requirement: A hand-over delivers nothing

An end SHALL stay pending for a recipient until that recipient confirms
it, naming its delivery's key: a notice, a wait, the inbox, a session's
start, a timeout and an expiry SHALL confirm nothing. Each hand-over
SHALL be recorded with to whom, when and by what, and a notice SHALL go
to a recipient only while nothing has handed it the end (43; weir 102,
103, I9; core.25).

#### Scenario: Handed and stopped before acting

- **WHEN** a fix's code delivers, its notice is sent to `ops`, and `ops` is then cleared without confirming it
- **THEN** status gives the end handed to `ops` by notice and pending for it, no second notice is sent at later settles, and `ops`'s next `flume inbox` gives it (S122)

#### Scenario: A notice claimed and not recorded

- **WHEN** a notice's send to `ops` is answered sent and the deliveries document's write of its hand-over fails, and the place is settled again
- **THEN** no second notice is sent, and the settle records the hand-over at the claim's time

### Requirement: A recipient confirms an end by its key

`flume confirm` SHALL confirm each delivery key it is given as the
recipient it names, recording when; a key it confirmed before SHALL
write nothing and say when it first did; a key no end is under, or a
recipient not among its end's, SHALL be refused in weir's words naming
the key and the field, confirming none of the keys after it (43; weir
102, 106).

#### Scenario: Confirmed twice

- **WHEN** `ops` confirms a code end's delivery key, then confirms it again
- **THEN** the first answer gives it confirmed at that time and the end delivered to `ops`; the second writes nothing and gives the first time (weir 102)

#### Scenario: A key of another recipient's

- **WHEN** `ops` confirms a key whose end's recipients are `boomdrive` alone, and then a key no end is under
- **THEN** each is refused naming its key, the first naming `reader` and the second `key`, and nothing is written (weir 102)

### Requirement: An end pending past the host's span is expired

An end pending for a recipient once the host's span has passed since it
was read SHALL be expired for that recipient: still pending, shown so
with its recipient by status, the inbox and a wait, never dropped, and
delivered by its recipient's confirmation alone; a recipient's name no
confirmation was ever made under SHALL be shown so (43, 56; weir 104,
106).

#### Scenario: A misspelt recipient

- **WHEN** a fix launched naming the recipient `bomdrive` lands, nothing collects its ends, and the fake host's clock moves past the week the host's span gives
- **THEN** status gives each end expired for `bomdrive`, a name never confirmed under, the host's status lists them, the tidied document stays, and a drop of it is refused naming `bomdrive` (S123)

#### Scenario: Confirmed once expired

- **WHEN** the operator runs `flume confirm --recipient bomdrive` with the ends' delivery keys
- **THEN** they are delivered, `bomdrive` reads known, and the next settle removes the document (weir 104, 106, 107)

### Requirement: An end written before reads as delivered where it was

An end whose held state holds deliveries of the old form SHALL be
recorded as a delivery at the first settle, wait or inbox that reads it,
each old delivery a hand-over by its way confirmed at its time, its
other recipients owed it; and no end so read SHALL be sent again to a
recipient it was delivered to (43, 61; core.17).

#### Scenario: A live place upgraded

- **WHEN** a live place's document holds code's end delivered to `ops` by notice and pending for `boomdrive`, written in the old form, and the place is settled
- **THEN** code's end reads delivered to `ops` at its old time and pending for `boomdrive`, no notice goes to `ops`, and settling again writes nothing more
