## MODIFIED Requirements

### Requirement: A hold after the launch carries its caller's words

A caller SHALL be able to put a place's next stage, the next repetition
of a repeating stage that runs, or its submission, on hold with words
saying what for; flume SHALL record the hold with its words and since
when, and a retry naming that stage or the submission SHALL end it. A
hold on another stage already started, or on a place whose work is over,
SHALL be refused naming why (8, 47, 49, 130, 142).

#### Scenario: A hold for a card's answer

- **WHEN** a caller holds a change's code stage with the words "waiting on the user's card about the schema" while construct works
- **THEN** construct's delivery starts no code stage, status gives code on hold with those words and since when, and the caller's retry of code starts it

#### Scenario: A hold on the submission

- **WHEN** a caller holds a fix's submission after the launch and its code stage delivers
- **THEN** the place is not submitted until a retry names the submission

#### Scenario: A hold on a running stage

- **WHEN** a caller holds a stage that does not repeat while its agent is working
- **THEN** the hold is refused naming the stage, and nothing changes

#### Scenario: A hold on a running repeating stage

- **WHEN** a caller holds a repeating code stage with words while its repetition on group 2 works
- **THEN** that repetition goes on, and once it delivers no next repetition starts; status gives code on hold with the words, naming group 3, and a retry of code starts group 3 (142)
