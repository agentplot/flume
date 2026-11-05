## ADDED Requirements

### Requirement: Status gives every place's line

Status SHALL give, for every place it gives, the place's line with each of its parts, its full form and its short form, made from the facts status read, in its JSON form, and the person's form SHALL print the line's full form under the place's heading (56, 61; core.14).

#### Scenario: A line in the JSON and the person's form

- **WHEN** status is asked of a fix place whose code agent works
- **THEN** its JSON gives `line` with `name`, `kind`, `stage`, `run`, `tasks`, `state`, `full` and `short`, and the person's form prints `fix--<name> · fix@1 · code · run 1 · working` under the place's heading (56, 61)

#### Scenario: The field list holds the line

- **WHEN** the place's answer's form drops `line` or one of its fields
- **THEN** the suite fails naming the answer and the field (61)

### Requirement: Status gives what the host's terminal lacks for flume's plugin

Status SHALL give, for the host, what the terminal's own configuration lacks for flume's plugin, each in the terminal profile's words, and the person's form SHALL print each under the line naming the watch; flume SHALL say it to no person but when asked (72, 122).

#### Scenario: A popup with no key

- **WHEN** the fake terminal says its configuration binds no key to flume's popup, and status is asked of one place
- **THEN** the host's `terminal_setup` gives those words, the person's form prints them under the watch's line, and nothing was sent to any pane (72)
