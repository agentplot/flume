## ADDED Requirements

### Requirement: A notice's hand-over waits on its send's answer

A notice's hand-over SHALL be recorded only once the terminal answered
its send sent. A settle that reads another's claim of a notice whose
send is still unanswered SHALL record nothing of it, and a send refused
SHALL take its claim back, leaving the end to the next delivery's
notice (43, 75; weir 103; core.25).

#### Scenario: A notice refused while another settle reads its claim

- **WHEN** a settle claims a notice to `ops`, another settle runs while its send is in flight, and the terminal then refuses the send, nothing typed
- **THEN** neither settle records a hand-over of the refused send, the claim is taken back, and the delivery that follows, the settle's own later pass or the next settle, sends the notice to `ops` once, its one hand-over recorded by that send (43, 75)
