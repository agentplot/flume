# A Rex block runs its script with no login

## Why

bolt/rex-is-flumes-second-terminal's submission to main, #56, failed
`flume:tools`: under sluice's sandbox, every Rex block the profile made
ran nothing, its screen `rex: exec /usr/bin/login: Operation not
permitted`, and every Rex test made its state folder in `/tmp`, which
the sandbox refuses. Past those, each test's Rex server was refused its
start, since the pinned Rex will not start a server where it cannot
write its runtime root, `~/Library/Application Support/rex` on macOS;
and the walkthrough's stand-in could not commit, since a block's
program gets the login environment alone and git, refused a name
lookup, could not guess an author.

The pinned Rex, 0.1.0-preview.14, starts a terminal's command on macOS
under `/usr/bin/login -flp <user> /bin/bash --noprofile --norc -c …`
unless the block's options say `"login": false`, whatever its `shell`
says. `/usr/bin/login` is setuid, and a Seatbelt sandbox refuses to
start a setuid program, so no check on main could ever run a Rex block.
With `"login": false` and `"shell": "none"`, Rex starts the command
itself, the block's child: read on deneb on 2026-11-13 from a server of
the pinned Rex, `sleep` ran as the server's child with `login` false,
and under `/usr/bin/login` and `bash` with it left out.

C.5 already says the block runs the program "with no shell": the login
and the `bash` it runs were between Rex and the start's script against
it, and the login environment the program gets is the one the script
gives, read by flume (29), so the login gave the program nothing.

## What Changes

- **The block runs the start's script with no shell and not as a login
  (94, C.5).** The profile's block options give `"login": false` beside
  `"shell": "none"`, so the block's child is the start's script, which
  becomes the program.
- **The block's program is the block's child (32, C.5).** The profile
  reads the block's child alone in the host's process table, by the
  file it runs or the script's path among its arguments, and no longer
  looks through a `/usr/bin/login` for it.
- **The Rex tools tests find room where a socket fits (core.51).** Each
  test's state folder is made in the temporary folder where its longest
  socket fits a socket's path, else in `/tmp` where the run may write,
  and the test fails naming what lacked room where neither holds it.
- **The Rex tools tests keep Rex's runtime root in their state folder
  (core.29).** The profile takes a runtime root for tests, as herdr's
  takes a session home, and every Rex call, the profile's and the
  test's, runs with `XDG_RUNTIME_DIR` under the test's state folder, so
  no test writes the host user's Rex runtime root.
- **The Rex walkthrough's login environment carries the tests' git
  environment.** The walk's login shell exports the git identity and
  isolation every test process runs under, as a host user's login
  environment and configuration give theirs, since a block's program
  gets nothing of the test process's.
- **The test's own blocks run with no login, and the terminal tests'
  stand-in draws its prompt.** A person's block the tests make takes
  `"login": false` too; the stand-in, which drew nothing a person sees,
  draws a line before it waits, as Claude Code draws its prompt, where
  the screen scenario read only the login's banner before.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `terminal-profiles`: the Rex profile starts its block with no login
  and reads the block's child as its program.

## Impact

- `crates/flume`: the Rex profile's block options, its reading of the
  block's program and its runtime root for tests; the Rex tools tests'
  state folder, runtime root, own blocks and stand-in; the Rex walk's
  login shell; the tests' git environment named once in
  `tests/common`.
- `docs/`: record 0068, still proposed, gains the construction agent's
  choice of `"login": false`.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool. The statement is unchanged: C.5 already says the
  block runs the program with no shell.
