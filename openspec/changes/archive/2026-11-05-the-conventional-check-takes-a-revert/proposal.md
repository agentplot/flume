# The conventional check takes a commit git revert wrote

## Why

CI's `conventional` job has failed on main at every push since 045361c
(#109), so nothing has been released since v0.10.0. The job runs
`convco check` from the newest v* tag to the head, and b2fb0a6 on main is
`Revert "docs(openspec): archive flume-opens-a-streams-multiplexer ..."`,
the message `git revert` writes, which convco refuses: "first line
doesn't match `<type>[optional scope]: <description>`". main cannot be
rewritten, so the check must take it, and the commit-msg hook with it,
so a session and CI hold a commit to one rule (core.22).

## What Changes

- **CI's conventional check skips a commit `git revert` wrote.** It runs
  `convco check --ignore-reverts`, which skips a commit whose message
  starts `Revert "`, in convco 0.6.3, the hook's, and 0.7.2, CI's alike.
- **The commit-msg hook takes the same commit.** convco's `--from-stdin`,
  how the hook reads a message before its commit exists, skips no revert
  in either release, so the hook's entry is a script of flume's that
  passes a message whose first line starts `Revert "` and hands every
  other to `convco check --from-stdin --config .versionrc`. git runs the
  hook on a revert committed after a conflict or `--no-commit`, never on
  one `git revert` commits itself.
- **git-cliff's notes and bump are unchanged by a revert.** A revert is
  not conventional, so `filter_unconventional` leaves it out of both, and
  the commit it reverts stays listed. Over v0.10.0 to main the bumped
  version is v0.11.0 and the notes are the same with b2fb0a6 skipped and
  without.

Not taken: a revert written as `revert: ...`, which `git revert` does not
write and which main's b2fb0a6 is not; `ignoreMessagePattern` in
`.versionrc`, which only 0.7.2 reads and neither applies to a message on
stdin; and a git-cliff group naming reverts, which would count a revert
toward the bump, so a range holding only a revert would release a patch.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `the-gate`: the commit hooks take a commit `git revert` wrote as CI's
  conventional check does, and `root:hooks` drives one.

## Impact

- `.github/workflows/ci.yml`: the conventional job's check.
- `devenv.nix`: the convco hook's entry, `conventional`.
- `check-hooks.sh` and the root project's `hooks` task: a revert taken
  and a message with no type refused, in the scratch clone.
- `.versionrc` and `cliff.toml`: what each does with a revert, said.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool.
