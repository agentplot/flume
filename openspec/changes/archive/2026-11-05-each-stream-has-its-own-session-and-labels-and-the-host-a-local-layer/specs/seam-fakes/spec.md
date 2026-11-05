## ADDED Requirements

### Requirement: The fakes script sessions, labels and the layers

The fake host SHALL script each stream's session and labels' shares,
the labels its configuration gives kinds, and its configuration's
values with their layers; the fake kinds SHALL script a kind's label;
and the fake terminal SHALL log where each agent was asked to open;
none SHALL read a configuration file of the host's (106).

#### Scenario: A scripted session

- **WHEN** a test scripts the fake host's `work` with the session `office` and a fix is launched on `work`
- **THEN** the fake terminal's log names the fix's agent opened in `office` (107)

#### Scenario: A scripted label

- **WHEN** a test scripts the fake kinds' `deploy` with the label `operation`
- **THEN** a deploy place's label is `operation` from `kind` where the fake host gives none (180)
