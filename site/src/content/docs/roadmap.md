---
title: Roadmap
description: What flume does today, and what is decided and not yet built.
---

flume runs on one host, from a launch to its landing and its tidy.
Callers such as a planning tool or a team runner drive it through
`flume launch`, `flume status`, `flume wait` and `flume inbox`, and see
the same answers you do.

## Built

| | macOS | Linux |
|---|---|---|
| Launches into a worktree of their own, made ready by the merge queue's policy | ✓ | ✓ |
| The built-in fix, chore and change, and kinds a repository or a launch declares | ✓ | ✓ |
| A fresh Claude Code agent per stage, its start and prompt acknowledged by its own hooks | ✓ | ✓ |
| Stages ended on their deliverable, a change's read from its OpenSpec files | ✓ | ✓ |
| A kind run through on its own, a failed submission sending the work back | ✓ | ✓ |
| Landing through sluice, and the place tidied after | ✓ | ✓ |
| Holds with words, stops short, retries and stalls | ✓ | ✓ |
| Ends delivered once to each recipient, or kept pending for `flume wait` and `flume inbox` | ✓ | ✓ |
| Role addresses, woken by a pending end or `flume wake` | ✓ | ✓ |
| The host's bound on agents running, with a queue by priority | ✓ | ✓ |
| Agent settings in six layers | ✓ | ✓ |
| Agents in herdr panes, one workspace per base, or a printed command | ✓ | ✓ |
| The herdr plugin's watch, sidebar rows and popup | ✓ | ✓ |
| The Claude Code plugin's `/launch` and status over MCP | ✓ | ✓ |
| Release binaries | Apple silicon | x86-64, ARM64 |

## Decided, not built

Each of these is decided in a record of the repository and stated in
flume's statement, and waits on its build. Until then flume behaves as
the rest of this site says.

- **A stage that repeats per task group**
  ([record 0019](https://github.com/agentplot/flume/blob/main/docs/adr/0019-a-stage-may-repeat-per-task-group-a-fresh-agent-for-each-group-and-change-2s-code-stage-does.md)).
  A stage whose deliverable is `tasks` may say `repeat = "per-task-group"`,
  and runs a fresh agent for each task group of the change, the first
  group with a task unticked each time, rather than one agent for every
  task. The prompt gains `{{group}}`, the group's number, title and
  tasks, and `{{notes}}`, a notes file in the place's own folder that
  each repetition reads first and leaves for the next. Status gives the
  stage as `code 3/7`, and holds, stops and retries name the group.
  flume's own `change` moves to version 2 with its code stage
  repeating; a repository that wants version 1 declares it. It waits on
  the kind format reading `repeat`.

- **Streams and account pools**
  ([record 0020](https://github.com/agentplot/flume/blob/main/docs/adr/0020-a-hosts-work-runs-in-named-streams-each-with-its-own-bound-priority-and-pool-of-accounts-and-an-intent-keeps-to-its-account.md)).
  The host's configuration declares named streams, `[streams.<stream>]`,
  each with its own bound on agents running, its priority and its pool
  of accounts, under the host's bound for them all. A launch, an
  intent, or the host's configuration for its repository and kind puts
  a place in a stream, and a full stream never holds another's room. A
  pool, `[pools.<pool>]`, lists accounts, each a program and its
  environment, chosen first-filled or least-busy. An account that hits
  its usage limit is paused until the reset its words name, for every
  flume process on the host, and its stage starts afresh on another
  account with no caller's word. An intent's agents keep to one account
  while it is usable, so they share its prompt cache. The account's
  variables and the stream's settings become two more layers of an
  agent's settings, eight in all.

- **Prompts in layers**
  ([record 0021](https://github.com/agentplot/flume/blob/main/docs/adr/0021-an-agents-prompt-is-assembled-in-layers-from-the-most-stable-to-the-least-the-stable-ones-in-its-standing-prompt.md)).
  Each agent starts with a standing prompt in the system prompt. It
  holds the repository's goals, from a file `[prompt] goals` names or
  from the launch, then the intent's brief. Then come the stage's
  standing instructions, from a new `standing` field of the stage, the
  same byte for byte for every agent of the stage, and its settings'
  instructions. Its
  first message holds the brief and the place's facts, and last the
  words of this run. Fresh agents then read the stable layers from the
  prompt cache, and status gives how much of each agent's input came
  from it. It waits on the kind format reading `standing`.

- **Kinds as shareable packages**
  ([record 0022](https://github.com/agentplot/flume/blob/main/docs/adr/0022-a-kind-is-a-shareable-package-named-by-its-source-name-and-version-and-a-kind-from-outside-runs-only-as-a-lock-records-it.md)).
  A kind is named `<source>/<name>@<version>`, as in `flume/change@2`,
  and a source may hold several versions side by side. A launch names
  `change`, `change@1` or `flume/change@1`; a name two outside sources
  hold is refused rather than guessed. A kind from any other source than
  flume, the repository and the launch runs only as a lock records it.
  The lock holds the hash of its content, and is the repository's `.config/flume.lock` or the host's
  `flume.lock`, written by a new `flume kinds lock` and by nothing else.
  flume fetches nothing at a launch. Settings given by a kind's name
  hold for every version of it.
