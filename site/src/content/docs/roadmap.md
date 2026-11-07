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
| Role addresses, each a role of a stream that a source of the host declares, woken by a pending end or `flume wake`, each run done once it delivers | ✓ | ✓ |
| A question put to the role it is for, of any stream, the answerer behind a stream told of every stall, a hold on the waiting move with a bound, and an answer taken once by its key | ✓ | ✓ |
| A role's address held by an agent that already runs, found by its name in its herdr session at each delivery, its ends delivered into it and no run woken | ✓ | ✓ |
| The host's bound on agents running, with a queue by priority | ✓ | ✓ |
| Agent settings in eight layers, each program's own settings among them, given inline or by a settings file | ✓ | ✓ |
| Streams, each with its own bound and priority, under the host's | ✓ | ✓ |
| Each stream's agents in a herdr session of its own, or the one a launch names | ✓ | ✓ |
| The host's configuration in a managed layer and a local one over it, the managed one pinning keys | ✓ | ✓ |
| Labels sharing a stream's agents, each with a reserve and slots kept free for it | ✓ | ✓ |
| Pools of accounts, a spent account paused and its agent's conversation carried to another where the host shares it, else its stage started afresh | ✓ | ✓ |
| An intent's agents kept to one account while it is usable | ✓ | ✓ |
| A pool choosing the account with the most of its allowance left by the host's reading, and starting nothing on an account past its threshold | ✓ | ✓ |
| Each agent's start checked against its program and environment | ✓ | ✓ |
| Agents in herdr panes, one workspace per base, or a printed command | ✓ | ✓ |
| Each stream's agents under a multiplexer of its own, the host's where it names none | ✓ | ✓ |
| `flume open`: a stream's agents in a window titled as the stream declares, or the line to run | Ghostty, Terminal.app | Ghostty |
| The herdr plugin's sidebar rows and popup | ✓ | ✓ |
| The Claude Code plugin's `/launch` and status over MCP | ✓ | ✓ |
| Release binaries | Apple silicon | x86-64, ARM64 |

A stream's multiplexer is herdr or the printed command. No profile is
written yet for herdr-core, Rex, cmux or TUIOS. Each is a change of its
own, checked against its release, and offers the views its multiplexer
has: a window running its terminal client, its own app, or both
([record 0037](https://github.com/agentplot/flume/blob/main/docs/adr/0037-a-stream-names-its-multiplexer-its-view-and-its-terminal-app-the-hosts-where-it-names-none-and-a-person-opens-its-agents-in-that-view.md)).
Until then a stream naming one refuses its launches and starts, naming
the profiles there are.

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

- **Points, a points bound and a pace**
  ([record 0025](https://github.com/agentplot/flume/blob/main/docs/adr/0025-work-carries-its-callers-points-and-a-stream-may-bound-its-points-in-flight-and-set-a-pace-over-a-rolling-span.md)).
  `flume launch --points <n>` gives a place's points, a number of the
  Fibonacci scale its caller sets and flume never estimates. They are
  in flight from the place's first start until its last stage delivers,
  and again through a fix round. A stream may state `points`, a bound
  on its points in flight beside its bound on agents, and a pace,
  `pace = { points = 8, per = "day" }`, over a rolling hour, day or
  week. Both hold a place's first start alone, and a place larger than
  either starts once its stream has nothing in flight, or started
  nothing in the span. A launch that gives no points on a stream that
  states either is refused, naming the stream. Status gives points
  queued and in flight for the host and each stream, what each stream
  started in its span and when the next charge leaves it, and each
  stream's tokens.

- **A kind's own label.** A kind's declaration may give its places a
  label, `label` in its table, which the host's configuration can
  override. It waits on the kind format's reader taking the field.
  Until then only the host's configuration gives a place a label.

- **An MCP server's labels.** The host's configuration declares each
  MCP server its agents may have, under `[mcp.<server>]`, with the
  labels whose places may have it. At each agent's start flume hands
  the agent the servers its place's label may have. The servers an
  account or a repository lists still load beside them.
