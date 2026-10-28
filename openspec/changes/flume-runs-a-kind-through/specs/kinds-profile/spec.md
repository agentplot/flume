## ADDED Requirements

### Requirement: Kinds read through weir's crate

The kinds profile SHALL read every declaration, a repository's
`.config/flume.toml` at the base's revision, a launch's `--kinds` file
and flume's own, through `weir-kinds` at a pinned revision, with no
reader of flume's own, mapping the format's fields onto the seam's
types and ignoring every section it does not know; a declaration weir
refuses SHALL be answered as could not be read, naming every field that
fails (19, 20, 23, 84, 85, core.4).

#### Scenario: Seam scenarios over the profile

- **WHEN** the kinds seam's scenarios run against the profile over scratch repositories
- **THEN** each passes, the same revision giving the same kinds and a launch's file read the same way (85)

#### Scenario: A kind at version 1, then 2

- **WHEN** a scratch repository commits a kind at version 1, then version 2, and a place is launched after each
- **THEN** the first place keeps version 1's stages and status names each place's kind at its version (S13)

#### Scenario: An elaboration section

- **WHEN** a launch's declaration holds a kind with an elaboration section
- **THEN** the kind is read and the section ignored (23, C.2)

### Requirement: The binding from the default branch

The profile SHALL read the `[profiles]` table at the default branch's
current revision, `origin/HEAD` after one fetch where the repository has
a remote and the local default branch otherwise, and nothing from
another tool's files; a seam the table leaves out SHALL be the place
profile `git-worktree` or unbound (20, 105, C.2, core.3).

#### Scenario: No table

- **WHEN** a scratch repository has no `.config/flume.toml` and fix is launched
- **THEN** the launch is refused naming the kind and the merge queue it needs (S33)

#### Scenario: A worktrunk binding

- **WHEN** a repository's table names `place = "worktrunk"`
- **THEN** the launch is refused naming the place profiles there are, and nothing is made (C.1, core.3)

### Requirement: flume's own three kinds and their prompts

flume's own declaration and its prompts SHALL be built into the binary
and read like any other: fix@1, chore@1 and change@1 as C.2 gives them,
each prompt naming its deliverable, the way to stop short and no submit,
merge or push; change@1's verify prompt committing `verify.md` in the
change's directory and archiving the change, or stopping short having
found work; its code prompt, sent with words, restoring an archived
change and recording the fix as a ticked task (22, 28, C.2).

#### Scenario: The kinds built in

- **WHEN** flume's own declaration is read
- **THEN** it holds fix@1 on `fix/`, chore@1 on `chore/`, change@1 on `change/` with construct, code and verify, all landing, each last stage's `on_fail` `code`

#### Scenario: No prompt asks to submit

- **WHEN** every prompt of flume's own kinds is rendered for a place
- **THEN** each holds `flume needs` and none holds a submit, a merge or a push (28, 71)
