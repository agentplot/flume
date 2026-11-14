# Tasks

The work is on `fix/rex-is-flumes-second-terminal-1`, cut from
bolt/rex-is-flumes-second-terminal, whose submission to main, #56,
failed `flume:tools` under sluice's sandbox.

## 1. The profile

- [x] 1.1 Give the Rex profile's block options `"login": false` beside `"shell": "none"`, and read the block's child alone as its program, by the file it runs or its arguments (32, 94, C.5). True when `sluice exec bolt/rex-is-flumes-second-terminal -- moon run flume:test` passes, with the profile's unit test reading both options in the block's layout.

## 2. The tests

- [x] 2.1 Make each Rex tools test's state folder in the temporary folder where its longest socket fits, else in `/tmp` where the run may write, failing naming what lacked room where neither holds it (core.51); give the profile a runtime root for tests and run every Rex call of the tests with `XDG_RUNTIME_DIR` under the state folder (core.29); have the Rex walk's login shell export the tests' git environment; run the test's own blocks with `"login": false`; and have the terminal tests' stand-in draw a line before it waits. True when `sluice exec bolt/rex-is-flumes-second-terminal -- moon run flume:tools` passes under sluice's sandbox, every `terminal_rex` test and the walkthrough under Rex among it.

## 3. The record

- [x] 3.1 Add the choice of `"login": false` to record 0067's construction agent's choices, with the path not taken. True when `sluice exec bolt/rex-is-flumes-second-terminal -- moon run root:adrs` passes.

## 4. The gate

- [x] 4.1 Run the checks of the projects this change touched. True when `sluice exec bolt/rex-is-flumes-second-terminal` passes.
