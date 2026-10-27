---
number: 7
title: A stage records the addresses of what it made, and flume carries them as written
status: accepted
date: 2026-10-27
decision-makers:
- Chuck Swanberg
---

# A stage records the addresses of what it made, and flume carries them as written

Status: accepted by the user on 2026-10-27, on their card ("Approved"), after a revision the same day on their note.

Deciders: the user, answering swancloud's proposal 369 on 2026-10-26 ("I want to make sure we think about this design more in terms of how Flume ties in. Landing a bolt is one thing but we could possibly have another Flume kind where maybe it was running or created a spike. We want a card that has the link to that spike or maybe there's a preview server that's not done through sluice, although I'm not sure we would want that, really"), relayed by swancloud-design; and the user again on the record's first card (2026-10-27): "How does this work? What process is driving this loose preview? Is it Flume or is it Crew? What if the preview needs to be started? I'm just thinking about a card that sits for a long time. Is there some concept of not just being handed something and hoping that the service stays running, but being able to toggle the server on and off? I don't want to overcomplicate it." The shape, and the answer on a thing that must run, are madswan-design's; the user's yes is asked on the record's card.
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 23, 24, 42 to 44, 62, 110, 123, 128, 129, C.2, C.7; records 0003, 0005, 0006; rules core.9, core.14, core.15, core.16; sluice record 0045 and its previews guide (`sluice preview`, `sluice release`, `sluice previews`).

## Context and problem statement

A stage ends on its deliverable, one of six forms read where it lives (24; record 0003), and whoever waits on it learns the end with the head or the report (43). Some stages make a thing a person will want to open: a spike on a branch, a preview a server serves, a page, a report kept elsewhere. The user wants the card that asks them about such work to carry the link. Today nothing in flume's model holds an address: the caller would have to read the report for one, or build one itself from the branch's name, which is a guess about another tool's layout.

Where does a made thing's address live, who writes it, and how does it reach the caller?

A thing that runs raises a second question. A card may sit for days. A server an agent started in its place is nobody's once the stage ends: nothing holds it, nothing restarts it, and the host's next restart ends it, so a card pointing at it holds a dead link with no way back. The user asked who drives such a preview, what happens when it needs starting, and whether it can be toggled on and off rather than hoped alive.

## Considered options

1. The kind declares what each stage makes, by name; the run records each thing's address in a file in the place's own folder, one line per thing; flume reads the lines at the stage's end and hands them to the caller and to status as written, making none of its own; a declared thing without an address is the deliverable not yet made (clause 129).
2. The addresses in the report's frontmatter, read by flume from the deliverable.
3. The caller reads the report, or builds the address from the branch's name and the forge's layout.

For a thing that must run:

1. The stage records the live address of a server it started, and the card points at it.
2. The stage records what brings it up, the branch and revision; the merge queue's preview is brought up and released by whoever shows the card.
3. flume keeps the server alive for the card, by a daemon of its own.

## Decision outcome

Option 1 for the address; for a thing that must run, option 2.

**The run knows the address; flume carries it.** A spike's address is the forge's, a page's is wherever it was published: each is known to the agent that made it, at the moment it made it, and to nothing else. flume carries the lines as written and checks, builds or rewrites none, as status reads every fact where it lives and computes none of its own (rules core.14, core.15). The caller's card carries them as flume gave them.

**Declared, so the card is honest.** The kind names what each stage makes, as it names the deliverable (23, C.2). A stage that was to make a preview and recorded no address has not finished: the stop check sends the agent back naming the thing (44, 110), as it does while the deliverable is missing. A thing the run made and did not declare is carried too, since an address is cheap to carry and dear to lose.

**In the place's folder, gone with the place.** The lines live at `made.jsonl` in the place's own folder beside its held state (128), where a stage already writes for flume what a declaration puts under `{{state}}`, and `{{made}}` renders the path into the prompt. The folder goes when the place is tidied (62, 123; record 0006), so a caller that wants the addresses after keeps them, as it keeps a `{{state}}` report; flume keeps no record of them between asks (rule core.15).

**A thing that must run is carried as what brings it up, never as a loose server.** A server is alive only while something holds it, and a stage holds nothing once it ends (62, 123; record 0006): a line naming such a server's address is a loose process a card points at for days, dead on the host's next restart, restarted by nobody. The merge queue already owns a running copy of a revision: `sluice preview <branch> <revision>` makes one from a landed revision, hands back its addresses, holds it until `sluice release`, gives the same preview back when asked twice, and frees one whose stack died (sluice record 0045). So a made thing that must run is recorded as the branch and revision that bring it up, and a stage starts no server that outlives its place. Whoever shows the card brings the preview up when the person opens it and releases it when they are done: the toggle the user asked for is sluice's on and off, pressed by the caller, and flume adds no daemon and no address of its own. Option 1 is the loose server; option 3 makes flume keep a stack it did not build, against core.9, with a second owner of ports and teardown beside sluice's.

**A landing makes nothing by landing.** What whoever waits on a submission learns is the commit it landed as (43). A preview of a landed revision is the merge queue's to make, asked of it at that commit by whoever wants one; no stage records it, since the stage ended before the landing.

Option 2 ties addresses to a report deliverable, which a `commits` or `tasks` stage has none of, and puts flume's reading inside a document whose shape is the agent's. Option 3 is the guess the user did not want: a link built by the caller from another tool's layout breaks when that layout moves, and a report's prose is no place to read an address from.

### Consequences

- The kind format gains `makes`, a list of names on a stage; `deliverables` is unchanged.
- `flume status --json` and the stage's end carry `made`, a list as written: each a name and kind with, for a durable thing, its address, and for a thing that must run, the branch and revision that bring it up.
- A stage starts no server that outlives its place; a kind that declares a running thing declares a revision to bring up, not a server to keep.
- Bringing a preview up from a card and releasing it is the caller's to build, on sluice's `preview` and `release`; flume carries the branch and revision and nothing more.
- The prompt gains `{{made}}`, filled only for a stage that declares what it makes.
- Rule core.23 is drawn from this record.
