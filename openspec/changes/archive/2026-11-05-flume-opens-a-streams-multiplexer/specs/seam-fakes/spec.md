## ADDED Requirements

### Requirement: The fake terminal scripts attach by view

The fake terminal SHALL answer attach with a command of its own for the
session it is asked for a window, and shown for its app, or as scripted
for either view, logging each asking with its view (106, 187).

#### Scenario: A scripted attach

- **WHEN** a test scripts the fake terminal to refuse attach for its app with `no app`, and asks it to attach `work` for a window and then for its app
- **THEN** it answers `fake-terminal attach work` and then the refusal, its log naming `attach work for terminal` and `attach work for app` (187)

### Requirement: The fake window scripts a show

A fake window SHALL log each show with its command and title and
answer opened, printed or refused as scripted, keeping the windows it
opened, and reading no display (106, 188).

#### Scenario: A scripted window

- **WHEN** a test scripts the fake window to answer printed with the line `attach work` and asks the core to open `work`
- **THEN** the core's answer gives that line, and the fake window's log names one show under the stream's title (188)

### Requirement: The fake host and resolver script a stream's display and named terminals

The fake host SHALL script a stream's title, multiplexer, view and
terminal app and the host's own, and the fake resolver SHALL hold
terminals and windows by name, each with a log of its own (106, 143,
190).

#### Scenario: A scripted title

- **WHEN** a test scripts the fake host's `work` with the title `Work Agents`
- **THEN** status gives `work`'s title `Work Agents` from `stream` (143)

#### Scenario: A scripted display

- **WHEN** a test scripts the fake host's own view as `app` and its `trial` with the multiplexer `other`, and asks the fake host for each stream's display
- **THEN** `trial` gives `other` from `stream`, every stream the view `app` from `host`, and every other stream the multiplexer `fake` and the terminal app `fake-window` from `neither` (143, 190)
