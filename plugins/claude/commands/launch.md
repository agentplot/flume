---
description: Launch a place for a brief with flume, choosing its kind and name with the operator
argument-hint: <brief>
allowed-tools: Bash(flume launch:*), Bash(flume status:*)
---

Launch one place for this brief with flume:

$ARGUMENTS

1. Choose the kind with the operator. flume's own kinds are `fix` (one
   stage that commits a fix), `chore` (one stage that commits a chore)
   and `change` (an OpenSpec change written, coded and verified); the
   repository's `.config/flume.toml` may declare more. Say which you would
   choose and why, and take the operator's word.
2. Choose a name with the operator: a few words in kebab case saying what
   the place will do, such as `the-parser-reads-tabs`.
3. Run `flume launch --kind <kind> --name <name> --brief "<brief>" --json`
   in the repository, the brief whole. Pass `--base <branch>` only where the
   operator names a base other than the repository's default branch.
4. Report what the answer says: the place's location and branch, its first
   stage and whether that stage started, waits for room or is held, and,
   where the answer is a refusal, its reason in its own words.
5. Say how to follow the place: `flume status <kind>--<name>`, and that
   flume submits the place once its last stage delivers. Do not submit,
   merge or push the place's branch yourself.
