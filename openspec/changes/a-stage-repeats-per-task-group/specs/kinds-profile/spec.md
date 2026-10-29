## ADDED Requirements

### Requirement: A stage's repeat read through weir's crate

The kinds profile SHALL read a stage's `repeat` through weir-kinds alone
and map `per-task-group` onto the stage as repeating; a declaration weir
refuses for `repeat`, any other value or the field on a stage that does
not deliver `tasks`, SHALL be answered as could not be read, naming the
field (23, 84, C.2; weir 53, 57; core.4).

#### Scenario: A repeating stage read

- **WHEN** a scratch repository's declaration gives a stage delivering `tasks` the field `repeat = "per-task-group"`
- **THEN** the kind is read with that stage repeating and every other stage not

#### Scenario: Repeat on a stage of commits

- **WHEN** a declaration gives `repeat = "per-task-group"` to a stage delivering `commits`
- **THEN** the launch is refused, naming the stage's `repeat` field, and nothing is made (4)

### Requirement: change@2's code prompt puts what differs last

change@2's code prompt SHALL hold every part the same for each
repetition of a place, its instructions, the place, branch, base and
change and the way to stop short, before its group, the notes' path and
its words, which come last, and SHALL tell the agent to read and add to
the notes and never commit them (141, C.2; core.31).

#### Scenario: Two repetitions' prompts

- **WHEN** change@2's code prompt is rendered for groups 1 and 2 of one place, the second with a retry's words
- **THEN** the two texts are equal up to the first character of the group, and differ only after it

## MODIFIED Requirements

### Requirement: flume's own three kinds and their prompts

flume's own declaration and its prompts SHALL be built into the binary
and read like any other: fix@1, chore@1 and change@2 as C.2 gives them,
change@2's code stage repeating per task group, each prompt naming its
deliverable and the way to stop short, and none asking for a submit, a
push or a rebase (22, 28, 71, 140, C.2).

#### Scenario: The kinds built in

- **WHEN** flume's own declaration is read
- **THEN** it holds fix@1 on `fix/`, chore@1 on `chore/`, change@2 on `change/` with construct, code repeating per task group, and verify, all landing, each last stage's `on_fail` `code`

#### Scenario: No prompt asks to submit

- **WHEN** every prompt of flume's own kinds is rendered for a place
- **THEN** each holds `flume needs` and none holds a submit, a push or a rebase, nor a merge but the base's in a conflict's words (28, 71, 112)

#### Scenario: A place launched at change@1

- **WHEN** a place launched at change@1 is settled by a flume whose own change is at version 2
- **THEN** it keeps change@1's stages, its code stage not repeating, and status names change@1 (21)

### Requirement: What flume's own prompts ask

change@2's verify prompt SHALL commit `verify.md` in the change's
directory and archive the change, or stop short having found work; its
code prompt sent with words SHALL restore an archived change and record
the fix as a ticked task; every code prompt sent back with a conflict's
words SHALL have its agent `git merge` the base's revision named, resolve
the files contested and commit, first, and never rebase (22, 112, C.2).

#### Scenario: A code prompt sent back with a conflict

- **WHEN** fix@1's, chore@1's and change@2's code prompts are rendered with a conflict's words naming the files contested and the base's revision
- **THEN** each tells the agent to `git merge` that revision into the place's branch, resolve those files and commit, before anything else, and never to rebase (112)
