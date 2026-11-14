## MODIFIED Requirements

### Requirement: Rex starts the agent by a script of the start's own

The Rex profile SHALL start the block, with no shell and not as a
login, so nothing runs between Rex and it, by a script of the start's
own, mode 0700, that removes itself and execs `/usr/bin/env -i` with
Rex's terminal variables, the login environment, the agent's
environment and flume's own, `PATH` led by a folder of the start's own
linking the program's kind to the program, then that link and the
arguments, so no variable stands in any process's arguments (29, 32,
94, C.5; core.30).

#### Scenario: No variable in any process's arguments

- **WHEN** an agent is given the variable `A_SECRET` and opens under Rex
- **THEN** the program sees `A_SECRET` and Rex's `REX_BLOCK`, no process under the block or of the server holds its value among its arguments, and the start's script is gone

#### Scenario: Nothing between Rex and the script

- **WHEN** the profile makes the block for an agent's start
- **THEN** its options give `"shell": "none"` and `"login": false`, so no `/usr/bin/login`, which a sandbox refuses to start, runs between Rex and the start's script (94, C.5)

### Requirement: Rex runs the program the settings name

Once Rex reads the program ready, the Rex profile SHALL read the host's
process table for the block's program, the block's child, which SHALL
be the program the settings name by its path or, for a program a
script starts, the script's path among its arguments; anything else
SHALL close the block and refuse the open naming what ran (32, C.5).

#### Scenario: A program that runs another in its place

- **WHEN** the program the settings name execs another program that reports Program Status
- **THEN** the block is closed and the open refused naming what ran
