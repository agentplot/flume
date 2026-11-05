## ADDED Requirements

### Requirement: The fakes script an attach, a window and a title

The fake terminal SHALL answer attach with a command of its own for the
session it is asked, or as scripted, shown or refused, and log each
asking; a fake window SHALL log each show with its command and title
and answer opened, printed or refused as scripted, reading no display;
and the fake host SHALL script a stream's title (106, 143, 187, 188).

#### Scenario: A scripted window

- **WHEN** a test scripts the fake window to answer printed with the line `attach work` and asks the core to open `work`
- **THEN** the core's answer gives that line, and the fake window's log names one show under the stream's title (188)

#### Scenario: A scripted title

- **WHEN** a test scripts the fake host's `work` with the title `Work Herdr`
- **THEN** status gives `work`'s title `Work Herdr` from `stream` (143)
