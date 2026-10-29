---
title: Prompt parts
description: Every part a stage's prompt may ask for, what fills it, and what flume adds to every prompt.
sidebar:
  order: 4
---

A stage's prompt is a text file with parts written in double braces,
`{{brief}}`. flume fills every part at each start of the stage, and
`flume prompt <place> <stage>` shows the result without starting
anything.

## The parts

| Part | Filled with | With nothing to fill it |
|---|---|---|
| `{{brief}}` | the launch's brief, whole | the launch is refused when a stage asks for it and the launch gives none |
| `{{name}}` | the work's name, as in `the-parser-reads-tabs` | always filled |
| `{{kind}}` | the kind's name, as in `fix` | always filled |
| `{{stage}}` | the stage's name, as in `code` | always filled |
| `{{branch}}` | the place's branch, as in `fix/the-parser-reads-tabs` | always filled |
| `{{base}}` | the base the place was cut from, as in `main` | always filled |
| `{{place}}` | the place's location | always filled |
| `{{change}}` | the place's change, `openspec/changes/<name>` | refused where the repository binds no change tool |
| `{{report}}` | the path of the stage's report: for `report:<path>`, the path in the place, or in the place's own folder where it starts `{{state}}`; for `committed:<path>`, the path | refused for a stage with neither deliverable |
| `{{words}}` | the words the stage was started with: a retry's question and answer, or a failed submission's or a finding's words | refused; put it inside `{{#words}}…{{/words}}` |
| `{{unready}}` | what readiness came to: the step that failed and what it printed, that it passed, or that the repository declares none | always filled |
| `{{needs}}` | the way to stop short, `flume needs` and `flume needs --found` | always filled |
| `{{made}}` | the path of the place's `made.jsonl` | refused for a stage that declares no `makes` |

A part refused at a start leaves the stage refused, naming the part in
status, and starts no agent. Text in double braces that names no part
is left as written.

## The words section

`{{#words}}…{{/words}}` holds text sent only when the stage starts
with words: a retry's `--words`, a failed or conflicting submission's
words, or found work from the kind's last stage. With no words, the
whole section is left out. flume's own code prompt uses it so:

```markdown
{{#words}}This stage has been sent back to you with these words:

{{words}}

Where the words name files contested and the base's revision the merge
queue tested against, take that revision in before anything else: …

{{/words}}Do the work the brief names, and commit it on {{branch}} in this place.
```

A retry's words reach the agent as the question its stage stopped on
and your answer:

```text
The question: Which README, the root's or the parser's?

The answer: The root's README.
```

## What flume adds

flume adds two things to a prompt that does not place them itself:

- **The way to stop short.** Every prompt carries it. Where the
  template has no `{{needs}}`, flume appends it at the end:

  ```text
  run `flume needs "<words>"` when you cannot go on without an answer,
  saying what you need, or `flume needs --found "<words>"` when you found
  work this stage cannot deliver past, saying what you found; then stop.
  ```

- **A failed readiness.** A kind's first stage, started in a place
  whose readiness failed, carries what `{{unready}}` gives. Where the
  template has no `{{unready}}`, flume appends it.

flume adds nothing between a prompt's parts, and nothing that differs
from start to start beyond what the parts fill.
