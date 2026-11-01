## ADDED Requirements

### Requirement: A stop-short's end says until when its session is open

The end of a stage ended short needing an answer SHALL carry, where its
agent's session is open for the answer, the time the session closes, in
the JSON form and in the notice's words (41, 43).

#### Scenario: The notice of a question

- **WHEN** a code agent stops short needing an answer at a time t, on a terminal that hands a running agent words
- **THEN** the end delivered to each recipient gives its session open until t plus its `keep_open`, and the notice's words say so (41, 43)

### Requirement: A wait after an answer waits on the next end

A wait on a stage SHALL answer only an end of its current start read
after the last answer that start took in its session, so a wait after
an answer never answers the stop-short it answered (43).

#### Scenario: Waiting again after an answer

- **WHEN** a caller's wait on code delivered its stop-short, the caller answered it in its session, and the caller waits on code again
- **THEN** the wait does not answer the stop-short as a read, and answers code's next end once it is read (43)
