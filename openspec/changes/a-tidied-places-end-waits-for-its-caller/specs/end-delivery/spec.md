## ADDED Requirements

### Requirement: The end of a place's work waits for its caller

The end of a place's work, its landing or its drop or its done, SHALL
stand pending for the caller its launch named, besides its recipients,
until the caller collects it by `flume inbox` or a wait as itself. No
notice SHALL be sent to the caller, a notice to a recipient of the
caller's name SHALL not count delivered for the caller, and the
caller's end SHALL wake no role, whatever the caller's name. A place
whose launch named no caller SHALL deliver its ends as before (7, 43,
62, 132).

#### Scenario: A landing collected by its caller

- **WHEN** a fix launched with the recipient `ops` and the caller `boomdrive` lands while `ops` waits in the fake terminal
- **THEN** `ops` gets one notice of the landing and no notice names `boomdrive`; the place is tidied, its document stays with the landing pending for `boomdrive` alone; and once `boomdrive` collects it by `flume inbox`, the next settle removes the document (43, 62)

#### Scenario: A caller that is also a recipient

- **WHEN** a fix launched with the recipient `ops` and the caller `ops` lands while `ops` waits in the fake terminal
- **THEN** `ops` gets one notice of the landing, the tidied document stays with the landing pending for `ops`, and it goes at the settle after `ops` collects it by `flume inbox` (43, 62)

#### Scenario: A wait as the caller collects

- **WHEN** the caller `boomdrive` waits on the submission of its fix, which lands and is tidied
- **THEN** the wait answers with the landing delivered to `boomdrive` by the wait, and the next settle removes the document (43)

#### Scenario: A caller written as an address wakes nothing

- **WHEN** a fix whose caller is `east/ops`, an address the scratch source `acme` declares with a run, lands
- **THEN** no run of `east/ops` is launched for it, no notice is sent, and the landing stays pending for `east/ops` until its `flume inbox` collects it (43, 132)

#### Scenario: No caller named

- **WHEN** a fix launched with the recipient `ops` and no caller lands while `ops` waits in the fake terminal
- **THEN** `ops` gets one notice of the landing, and the tidy keeps no document of the place (43, 62)

### Requirement: A drop or a done is an end for the caller alone

A drop, and a done of a place whose kind does not land, SHALL record an
end of the place, keyed `over/dropped` or `over/done`, in the write that
makes its work over, with when; its one recipient SHALL be the place's
caller, none where the launch named none. No notice of it SHALL be sent
to the place's recipients and no role SHALL be woken for it (43, 52,
114, 132).

#### Scenario: A drop

- **WHEN** the caller drops a live fix launched with the recipient `east/planner`, which the scratch source `acme` declares with a run, and the caller `boomdrive`
- **THEN** status of the tidied document gives the end `over/dropped` pending for `boomdrive` alone; no notice is sent and no run of `east/planner` is launched; and the document stays until `boomdrive` collects the end by `flume inbox` (43, 52, 62)

#### Scenario: A done

- **WHEN** a place of a kind that does not land, launched with the caller `boomdrive`, is declared done
- **THEN** the end `over/done` stands pending for `boomdrive` alone, and the tidied document stays until it is collected (43, 114)

#### Scenario: A drop with no caller

- **WHEN** a fix launched with the recipient `ops` and no caller is dropped while its construct runs
- **THEN** the end `over/dropped` is recorded pending for no one, `ops` gets no notice, and the tidy keeps no document of the place (43, 52, 62)

## MODIFIED Requirements

### Requirement: The inbox collects what is pending

`flume inbox` SHALL give every end pending for a recipient, of every
place on the host, with what 43 names and its key, and SHALL record each
delivered to that recipient. An inbox that names a place SHALL give and
record only the ends of the documents `flume status` reads under that
name, the place's own, else those kept for it once it was tidied, or
the one document `<place>@<at>` names; it SHALL give only that place's
entry of the places that wait on the recipient, and SHALL be refused,
naming the place, where no document stands under that name. Status, a
wait's read and an inbox for another recipient SHALL mark nothing
delivered (43, core.25).

#### Scenario: Collected after a restart

- **WHEN** two ends of two places are pending for `ops` while its session is gone, and its restarted session runs `flume inbox` as `ops`
- **THEN** the answer holds both ends whole, each with its key, both read delivered to `ops`, and a second `flume inbox` gives nothing

#### Scenario: An inbox naming a place

- **WHEN** two fixes launched with the caller `boomdrive` have landed and been tidied, and `boomdrive` runs `flume inbox` naming the first
- **THEN** the answer holds the first's landing alone, delivered to `boomdrive`; the second's landing stays pending for `boomdrive` and its document stays (43, 62)

#### Scenario: A name under which nothing stands

- **WHEN** an inbox names a place under which no document stands on the host
- **THEN** it is refused naming the place, and nothing is marked delivered (43)

### Requirement: A tidied place's document stays while an end is pending

A place whose work is over SHALL be tidied as always, its worktree and
branch removed (66, 67), and while an end of it stands pending for a
recipient or for its caller its document alone SHALL stay, naming those
it waits on, its caller among them, until every end is delivered or its
caller drops it. A launch of the same name SHALL not be refused for it
(3, 43, 62).

#### Scenario: A landing with its recipient away

- **WHEN** a place lands while its one recipient's session is gone
- **THEN** its worktree and branch are removed, status gives the tidied place pending for that recipient, and once the recipient collects by `flume inbox` the next settle removes the document

#### Scenario: Dropped while pending

- **WHEN** the caller drops a tidied place whose end stands pending
- **THEN** the document and its pending ends are gone, and the inbox gives nothing of it

#### Scenario: Launched again

- **WHEN** a place of the same repository, kind and name is launched while the tidied document stays
- **THEN** the launch makes its place, and the tidied document's ends stay pending as they were

#### Scenario: Kept for its caller

- **WHEN** a place launched with the recipient `ops` and the caller `boomdrive` lands, and `ops` has its notice
- **THEN** status gives the tidied place pending for `boomdrive` (62)

### Requirement: An end's recipients

An end's recipients SHALL be its place's, as its launch took them. A
stop-short's that named a role SHALL be that address alone, not the
place's. A stall's SHALL be the place's, the address the stop-short it
waits on named, of whichever stream, and the answerer behind the
place's own stream, each once. A drop's or a done's SHALL be none: it
stands pending for the place's caller alone, as the end of the place's
work stands pending for it besides its recipients (7, 41, 43, 130, 193).

#### Scenario: A stop-short that named a role

- **WHEN** a construct of a place in `east`, whose recipient is `east/operator`, stops short naming `ops`
- **THEN** the end stands pending for `east/ops` alone, and none of it for `east/operator` (41, S93)

#### Scenario: A stall of a question for another stream

- **WHEN** a construct of a place in `east`, whose recipient is `east/planner`, stopped short naming `west/design`, `west/operator` stands behind `west`, and the place stalls
- **THEN** the stall is delivered to `east/planner`, `west/design` and `east/operator`, once each, and not to `west/operator` (130, 193)
