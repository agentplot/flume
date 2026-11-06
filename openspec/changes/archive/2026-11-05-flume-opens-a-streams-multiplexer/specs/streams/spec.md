## ADDED Requirements

### Requirement: A stream names its multiplexer, its view, its terminal app and its title

`[host]` SHALL take `multiplexer`, `view` and `terminal_app`, and
`[streams.<stream>]` the same three and `title`, a stream's over the
host's. Neither naming a multiplexer or a terminal app SHALL be the
printed command, neither naming a view `terminal`, and a stream naming
no title its name (105, 143, 186, 190, C.7).

#### Scenario: The keys and none

- **WHEN** the host's configuration gives `[host] multiplexer = "herdr"` and `terminal_app = "ghostty"`, `[streams.office]` with `terminal_app = "terminal"` and `title = "Office Herdr"`, and `[streams.personal]` with none
- **THEN** `office` reads `herdr` from `host`, the view `terminal` from `neither`, `terminal` from `stream` and `Office Herdr` from `stream`, and `personal` reads `herdr` and `ghostty` from `host`, the view `terminal` from `neither` and the title `personal` from `stream-name` (143, 186, 190)

### Requirement: The three settings' values

`multiplexer` and `terminal_app` SHALL be names and `title` a text, and
`view` SHALL be `terminal` or `app`, any other value refusing the
configuration; `[host] terminal` SHALL be read as `[host] multiplexer`,
a `[host]` giving both refused naming both (186, 190, C.7).

#### Scenario: The word written before

- **WHEN** the host's configuration gives `[host] terminal = "herdr"`, and then `[host] terminal = "herdr"` and `multiplexer = "printed"` together
- **THEN** the first reads the host's multiplexer `herdr`, and the second is refused naming `terminal` and `multiplexer` (C.7)

#### Scenario: A view no word names

- **WHEN** `[streams.work]` gives `view = "tab"`
- **THEN** the configuration is refused naming `streams.work`'s `view` and its two values (C.7)

#### Scenario: A title that is no text

- **WHEN** `[streams.work]` gives `title = 3`
- **THEN** the configuration is refused naming `streams.work`'s `title` (C.7)

### Requirement: Status gives each stream's display

Status SHALL give each stream's multiplexer, view, terminal app and
title, each with where it came from, and a stream's refusal where its
multiplexer or terminal app names no profile, in the JSON form and the
person's (56, 143, 186, 190; core.14).

#### Scenario: The display in status

- **WHEN** status is asked of a host whose `office` names its terminal app and title and whose `personal` names none
- **THEN** `office` gives its terminal app and title from `stream`, and `personal` the host's multiplexer and terminal app from `host`, the view from `neither` and its name as its title from `stream-name` (56, 143)
