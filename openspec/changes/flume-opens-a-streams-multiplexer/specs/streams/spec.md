## ADDED Requirements

### Requirement: A stream declares its window's title

`[streams.<stream>]` SHALL take `title`, the title of a window that
shows its agents, the stream's name where it gives none, read from the
managed layer alone; and status SHALL give each stream's title with
where it came from, `stream` or `stream-name` (56, 143, 186, C.7).

#### Scenario: A title and none

- **WHEN** the host's configuration gives `[streams.work]` with `title = "Work Herdr"` and `[streams.personal]` with none
- **THEN** status gives `work`'s title `Work Herdr` from `stream` and `personal`'s `personal` from `stream-name`, in the JSON form and the person's (56, 143)

#### Scenario: A title that is no text

- **WHEN** `[streams.work]` gives `title = 3`
- **THEN** the configuration is refused naming `streams.work`'s `title` (C.7)

#### Scenario: A title set by the local layer

- **WHEN** the host's `config.local.toml` sets `streams.work.title`
- **THEN** the local layer is not taken, status naming `streams.work.title`, and `work`'s title is the managed layer's (C.7; record 0032)
