## ADDED Requirements

### Requirement: sluice forecasts a base place's branch

The sluice profile SHALL answer forecast by `sluice submit <base>
--check --head <branch> --json` run in the repository, with every
multiplexer's session stripped and `GIT_TERMINAL_PROMPT=0`: exit 0 with
`conflict` null as clear, exit 2 with `conflict` as in conflict, and any
other answer as could not read with its `error.message` or its errors.
It SHALL pass `--check` to no submit, and the forecast SHALL leave no
entry in sluice's records (203, 210, C.6; sluice 64; core.6).

#### Scenario: A base place's branch forecast over a scratch repository

- **WHEN** the pinned sluice lands `main` and `train/*` of a scratch repository with a stand-in origin, flume opens `train/a` from `main`, a fix lands a change to `README.md` on `train/a`, and flume forecasts `train/a` onto `main`; then a change to `README.md` lands on `main` and flume forecasts again
- **THEN** the first forecast answers clear with `main`'s revision; the second answers in conflict on `README.md` with `main`'s new revision and no entry ahead; and `sluice status main --all --json` gives no submission of `train/a` after either (203, 210; sluice 64)

### Requirement: sluice's forecast is read as a conflicting submission's words are

The head and the base's revision SHALL be the answer's `head` and
`onto`, the files contested its `conflict.paths`, and each entry ahead
it meets one of `conflict.with[]` named by its `id` and where it was
submitted from. The words SHALL be sluice's text form followed by the
files contested and the base's revision, as a conflicting submission's
words carry them (112, 210, C.6; sluice 65).

#### Scenario: A conflict's words

- **WHEN** the profile reads an answer with `conflict` naming `README.md` and no entry ahead, `head` `aaa111` and `onto` `c0ffee`
- **THEN** it gives the conflict with `README.md`, head `aaa111`, the base's revision `c0ffee` and no entry ahead, its words naming `README.md` and `c0ffee` in the lines a conflicting submission's words carry (112, 210)
