## MODIFIED Requirements

### Requirement: One interface per seam

There SHALL be exactly one interface for each of the eight seams of
Part B: the place (78 to 83), the kinds (84, 85), the change (86, 87,
127), the agent (88 to 93, 115, 116, 121, 134, 135), the terminal (94
to 98, 122, 136 to 139, 187), the merge queue (99, 101, 117, 125, 126,
133), the store (102 to 104, 128) and the window (188, 189). Each SHALL
offer the operations its section names and no operation of the core's,
and no interface SHALL name a tool (core.1, core.2).

#### Scenario: Each seam's operations

- **WHEN** each seam's scenarios run against its fake
- **THEN** each operation its section of Part B names is exercised, and each scenario cites the clause of its operation

#### Scenario: No tool named outside a profile

- **WHEN** the seams', core's, fakes', suite's and commands' sources are searched for `wt`, `worktrunk`, `openspec`, `claude`, `herdr`, `sluice` and `ghostty` as words
- **THEN** none is found outside a `profiles` directory

## ADDED Requirements

### Requirement: The terminal seam's attach

The terminal seam SHALL offer attach: given a session by its name or
the terminal's own, and a title, the command that shows it to a person,
its program by its full path, its arguments and the variables it needs
with their values, the session started first where it does not run; or
that the terminal showed it in a window of its own; or a refusal (187).

#### Scenario: Attach over the fake

- **WHEN** the fake terminal is asked to attach `work` under `Work Herdr`, then its own session, then is scripted to refuse
- **THEN** it answers a command for `work`, then one for its own session, then the refusal, and logs each asking with its title (187)

### Requirement: The window seam's show

The window seam SHALL offer show: given a command and a title, a window
running the command under that title, opened; the line a person runs,
printed, where the host shows no window; or a refusal in the window
program's words; within a bound the profile states, opening at most one
window at each asking (188, 189).

#### Scenario: Show over the fake

- **WHEN** the fake window is asked to show a command under `Work Herdr` twice, then is scripted to answer printed and then refused
- **THEN** it answers opened twice, logging two shows with the command and the title, then the printed line, then the refusal (188, 189)

### Requirement: The types of a view and a window

The seams SHALL pass one type each for a view's command, attach's
answer, show's answer and a stream's title, none where a stream gives
none; each SHALL serialize to JSON and back unchanged, a stream written
before reading with no title (143, 187, 188).

#### Scenario: A view, a window and a stream round-trip

- **WHEN** a command needing `XDG_CONFIG_HOME`, each of attach's and show's answers, and a stream with the title `Work Herdr` are written to JSON and read back, and a stream written with no `title` key is read
- **THEN** each equals the value written, and the last stream reads with no title (143, 187, 188)
