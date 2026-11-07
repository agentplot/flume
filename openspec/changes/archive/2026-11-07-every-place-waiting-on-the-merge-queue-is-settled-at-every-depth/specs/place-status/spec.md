## ADDED Requirements

### Requirement: Status gives the host's watch

Status SHALL give, for the host, the watch a caller keeps: its bound in
the host's `watch`, a bound or none, when its newest settle began as
`watch_settled`, and, while flume runs attended after a watch lapsed,
when it lapsed as `watch_lapsed`. The person's form SHALL say whether a
caller keeps watch, with its bound and when it last settled, or since
when none has (56, 61, 119, 208; core.14).

#### Scenario: A watched host and an unwatched one

- **WHEN** a caller settled every place with a bound of two minutes one minute ago, and status is asked; then five minutes pass with no such settle, and status is asked again
- **THEN** the first gives `watch` with 120 seconds and `watch_settled`, and the person's form prints `a caller keeps watch: every place settles every 2m, last at <time>`; the second gives `watch_lapsed` four minutes after that settle began, and prints `no caller keeps watch since <time>: every place settles at each command of flume's that acts` (119, 208, S118)

### Requirement: Status gives no terminal a watch

Each terminal's answer in status SHALL keep its `watch` field, given
none for every terminal, and nothing of a terminal SHALL decide the
host's `watch` (61, 122, 208; core.14).

#### Scenario: The terminals' answers in status

- **WHEN** a caller keeps watch, the host binds the fake terminal and `trial` names `other`, and status is asked
- **THEN** each terminal's answer gives `watch` none, and the host's `watch` gives the caller's bound (61, 122, 208)
