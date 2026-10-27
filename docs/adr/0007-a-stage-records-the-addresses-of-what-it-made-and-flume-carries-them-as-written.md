---
number: 7
title: A stage records the addresses of what it made, and flume carries them as written
status: proposed
date: 2026-10-27
decision-makers:
- Chuck Swanberg
---

# A stage records the addresses of what it made, and flume carries them as written

Status: proposed to the user on 2026-10-27, as `briefs.15` has it; accepted only on their yes.

Deciders: the user, answering swancloud's proposal 369 on 2026-10-26 ("I want to make sure we think about this design more in terms of how Flume ties in. Landing a bolt is one thing but we could possibly have another Flume kind where maybe it was running or created a spike. We want a card that has the link to that spike or maybe there's a preview server that's not done through sluice, although I'm not sure we would want that, really"), relayed by swancloud-design; the shape is madswan-design's, and the user's yes is asked on the record's card.
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 23, 24, 42 to 44, 62, 110, 123, 128, 129, C.2, C.7; records 0003, 0005, 0006; rules core.9, core.14, core.15, core.16.

## Context and problem statement

A stage ends on its deliverable, one of six forms read where it lives (24; record 0003), and whoever waits on it learns the end with the head or the report (43). Some stages make a thing a person will want to open: a spike on a branch, a preview a server serves, a page, a report kept elsewhere. The user wants the card that asks them about such work to carry the link. Today nothing in flume's model holds an address: the caller would have to read the report for one, or build one itself from the branch's name, which is a guess about another tool's layout.

Where does a made thing's address live, who writes it, and how does it reach the caller?

## Considered options

1. The kind declares what each stage makes, by name; the run records each thing's address in a file in the place's own folder, one line per thing; flume reads the lines at the stage's end and hands them to the caller and to status as written, making none of its own; a declared thing without an address is the deliverable not yet made (clause 129).
2. The addresses in the report's frontmatter, read by flume from the deliverable.
3. The caller reads the report, or builds the address from the branch's name and the forge's layout.

## Decision outcome

Option 1.

**The run knows the address; flume carries it.** A preview's address is the preview server's, a spike's is the forge's, a page's is wherever it was published: each is known to the agent that made it, at the moment it made it, and to nothing else. flume carries the lines as written and checks, builds or rewrites none, as status reads every fact where it lives and computes none of its own (rules core.14, core.15). The caller's card carries them as flume gave them.

**Declared, so the card is honest.** The kind names what each stage makes, as it names the deliverable (23, C.2). A stage that was to make a preview and recorded no address has not finished: the stop check sends the agent back naming the thing (44, 110), as it does while the deliverable is missing. A thing the run made and did not declare is carried too, since an address is cheap to carry and dear to lose.

**In the place's folder, gone with the place.** The lines live at `made.jsonl` in the place's own folder beside its held state (128), where a stage already writes for flume what a declaration puts under `{{state}}`, and `{{made}}` renders the path into the prompt. The folder goes when the place is tidied (62, 123; record 0006), so a caller that wants the addresses after keeps them, as it keeps a `{{state}}` report; flume keeps no record of them between asks (rule core.15).

**A landing makes nothing by landing.** What whoever waits on a submission learns is the commit it landed as (43). A preview of a landed revision is the merge queue's to make, asked of it at that commit by whoever wants one; no stage records it, since the stage ended before the landing.

Option 2 ties addresses to a report deliverable, which a `commits` or `tasks` stage has none of, and puts flume's reading inside a document whose shape is the agent's. Option 3 is the guess the user did not want: a link built by the caller from another tool's layout breaks when that layout moves, and a report's prose is no place to read an address from.

### Consequences

- The kind format gains `makes`, a list of names on a stage; `deliverables` is unchanged.
- `flume status --json` and the stage's end carry `made`, a list of name, kind and address as written.
- The prompt gains `{{made}}`, filled only for a stage that declares what it makes.
- Rule core.23 is drawn from this record.
