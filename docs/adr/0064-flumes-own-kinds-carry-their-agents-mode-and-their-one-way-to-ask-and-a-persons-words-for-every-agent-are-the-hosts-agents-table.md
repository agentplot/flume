---
number: 64
title: flume's own kinds carry their agents' mode and their one way to ask, and a person's words for every agent are the host's agents table
status: accepted
date: 2026-11-09
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# flume's own kinds carry their agents' mode and their one way to ask, and a person's words for every agent are the host's agents table

Status: accepted. The direction is the user's, in agentplot-design's pane on 2026-11-09, reading the `kinds` table deneb's Boomdrive configuration gives flume, each of change, chore and fix with `mode = "full"` and the instructions "You are a construction agent: you never post a Pending You card or a Plannotator Inbox message [...] Where you would ask the user, run flume needs with your words", and fix with `label = "operation"`: "why is this in swancloud configs? the idea is that the are part of the recipe from crew/boomdrive". What the kinds say, and where a person's own words go, is agentplot-design's own choice, cheap to change.

Deciders: the user, 2026-11-09, above; agentplot-design, for the form.
Sources: `agentplot/blueprints:design/principles.md` 14 ("The Boomdrive set derives or defaults every value the recipe knows: a tool's kinds and their words [...] A host writes only what is its own, and only through a tool's own settings"); `design/flume/requirements.md` 22, 28, 32, 148, 158, 162, 169, 180 and C.7, and its section on flume's own kinds; agentplot/boomdrive's record 0034, point 3 (capture.md is the person's own); this repository's own kinds under `crates/flume/src/profiles/kinds/weir/own/`, whose stages give their agents no settings (`agents = [{}]`) and whose prompts end "If you cannot go on without an answer, or you find work this stage cannot deliver past, stop short: {{needs}}"; afterthought/swancloud's `lib/flume-config.nix:65-115`, which writes deneb's `kinds` table.

## Context and problem statement

flume's own kinds, change, chore and fix, give their agents no mode, so the account's own configuration of Claude Code decides how far an agent may act (162). An agent that asks a person's leave before each command waits in a place no person watches. So every host that runs them unattended gives each kind `mode = "full"` in its own `kinds` table.

The kinds' prompts tell an agent to stop short when it needs an answer. They do not say that stopping short is its only way to ask. A person's Claude Code may load plugins and hooks that tell every session to put questions on a card or in an inbox, and an agent of a construction kind that follows them asks someone who is not its caller, and never stops short. deneb's host gives each kind words against it, naming the person's own tools.

deneb's table also gives every kind the person's own process, a file every agent is to read first, and gives fix its label, `operation`. A label is the host's word, which flume's own kinds never declare (180).

## Considered options

1. **flume's own kinds carry what every host gives them.** Their stages give their agents the mode `full` and say, in their standing instructions, that stopping short is their one way to ask. A person's own words for every agent are the host's table for every agent. A label stays the host's, and the Boomdrive set gives it.
2. The Boomdrive set gives the kinds' mode and words. A host without the set would still write them, and flume's own kinds would still need every host's words to run unattended.
3. Nothing moves. Every host keeps writing the same table.

## Decision outcome

Option 1.

- **flume's own kinds give `full`.** Each stage of change, chore and fix gives its agents the mode `full`, in the kind's layer, the last of 32's: such a stage commits in its own place, unattended, and flume answers no question an agent in `ask` or `edits` puts (169). A host that wants its agents to reach less states the mode in its own layers, which come before the kind's (162). plan's plan stage starts in its program's planning mode whatever a layer gives (163).
- **Their one way to ask is to stop short.** Each stage's standing instructions say that an agent that needs an answer stops short with its words (28), and never asks through anything else, a card, an inbox, a message or a question in its pane, whatever a hook, a plugin, a file or a person's instructions given to every agent tell it. The words name no tool of anyone's.
- **A person's own words for every agent are the host's `[agents]` table.** A person who wants every agent to read their own process, as deneb's file names, gives it as the host's `instructions` for every agent (32), through flume's module, never in a kind's table. flume's own kinds hold none of a person's words.
- **A label stays the host's.** flume's own kinds declare none (180). The Boomdrive set gives each the label its drives' streams name: fix's `operation` among them.
- **A host writes no `kinds` table for flume's own kinds.** One that does still lays over them (32), and is taken.

Not taken: options 2 and 3. A kind's words travel with the kind (148), and a host without the set would write them still.

### Consequences

- Good: a host runs flume's own kinds unattended with no table of its own, and a construction agent asks only its caller, whatever its account's plugins say.
- Bad: a host that relied on its account's own mode for these kinds, narrower than `full`, now states it in its own layers.
- Bad: a new version of each of flume's own kinds, since a version's content never changes once released (157). change@4, chore@2 and fix@2 carry it, and plan's next version once plan is built.
- The statement's 22, its section on flume's own kinds, 162 and C.7 say it, at agentplot/blueprints.
- The Boomdrive set gives fix its `operation` label and the other kinds theirs, and deneb's file drops its `kinds` table and gives its process as `[agents] instructions`. That is Boomdrive's and swancloud's to follow. A kind of another source, research's, carries its own mode and words where it is declared.
