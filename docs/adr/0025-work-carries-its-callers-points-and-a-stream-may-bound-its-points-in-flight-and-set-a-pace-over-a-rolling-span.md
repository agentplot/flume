---
number: 25
title: Work carries its caller's points, and a stream may bound its points in flight and set a pace over a rolling span
status: accepted
date: 2026-10-30
decision-makers:
- Chuck Swanberg
---

# Work carries its caller's points, and a stream may bound its points in flight and set a pace over a rolling span

Status: accepted by the user on 2026-10-30, on flume-sluice-build's card (req_bb65e1d78dd14c8d859f, "Approved", no note).

Deciders: the user, on flume-sluice-build's card req_8f361bf504424a5b975b, 2026-10-30, who set an optional `--points` on `flume launch`, a Fibonacci number the caller sets and flume never estimates, "I think of it like story points, and it should be on a Fibonacci scale... This would help serve the long-term goals of Flywheel in terms of understanding how much work is in the queue and would also let us load balance according to that complexity score"; that points measure the agent's work only, test and merge time being machine time sluice measures and flume reports beside them; points queued and in flight per stream and per host in status; a stream's bound on points in flight beside its agents bound, which stays because accounts and usage limits count sessions running at once; and a stream's pace, a rate over time that is no priority, for "a side project... I want it to be moving along but I don't necessarily want to saturate available capacity. I want it to move at a defined rate because I'm okay with it taking 3 weeks instead of 1 week"; with tokens reported per stream and never limited; and, on the card, 2026-10-30, of the pace and its spans: "yes. pace is good. per hour, day and week as options". Whether points belong to a place or to each stage, when points are in flight, how a bound and a pace hold a place larger than either, a rolling span over a calendar's, and what work with no points counts for, are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 9c7490c, clauses 4, 7, 8, 56, 70, 99, 143, 152, 153, 165 to 168, C.4, C.6, C.7, S69 and S70, and section 3's points, pace, room, in flight and waiting for room, as amended with this record; records 0010, 0020, 0023, 0024; rules core.26, core.32; `crates/flume-core/src/room.rs` (`Queue`, `must_wait`); `sluice status <id> --json` (sluice 0.55.0), its `submitted`, and sluice's run records' `started` and `ended`.

## Context and problem statement

A host's queue counts agents (record 0010) and orders by priority, the stream's then the place's (record 0020). It says nothing of how much work stands behind each place: a one-line fix and a week's change each take one agent's room, and nothing tells the user, or a caller balancing work across hosts, how much is queued or how fast a stream is spending.

Two things follow from the user's words. A caller wants to say how big a piece of work is, as a team says a story's points, so the queue can be read and balanced by size. And a stream that matters less, a side project, should move steadily rather than take all the room it is allowed whenever room is free: a rate, not a rank.

## Considered options

1. Points on each stage, set by the caller per stage.
2. Points on the place, set once at the launch for the unit of work; a stream's points bound on its places in flight, beside its agents bound; a pace over a rolling span; a launch with no points refused on a stream that bounds or paces them.
3. As 2, with the pace over a calendar's span, a day from midnight or a week from Monday.
4. As 2, with work given no points counted as 1.
5. As 2, with work given no points counted as nothing against the bound and the pace.

## Decision outcome

Option 2.

**Points belong to the place.** The user said "each stage, and/or the unit of work". The unit: a caller estimates a piece of work, as a story is estimated, and the stages are the kind's way of doing it, which a caller did not write and cannot weigh one from another; change@2's code stage repeats per task group (140), so a stage's share is not even known at the launch. One number per place is also what the queue needs to say how much work stands in it. So `flume launch --points <n>` gives a number of the Fibonacci scale, 1, 2, 3, 5, 8, 13, 21 and on; any other refuses the launch naming it (4). flume never estimates, changes or judges points. They are fixed at the launch with the brief (8), since a re-estimate is a new launch's to make. A role's run takes its points from `points` in its `[roles]` table (C.7).

**What points measure.** The agents' work alone. The time a submission spends in the merge queue, tested and landed, is the machine's: status gives it beside the points, from submission to outcome and how long its checks ran, as the merge queue's records give them, read where they live (58, 99, 168), never added in.

**Queued and in flight.** A place's points are queued from its launch until its first stage starts. They are in flight from then until its last stage delivers, and again from a fix round's start, a failed submission's or a finding's (131), until the last stage delivers again. Between stages, and while the place waits on its caller, a plan for review among it (record 0024), they stay in flight: the work is begun and not done, and counting them only while an agent runs would let a stream start more work than its bound means. Once the work is over they are neither. Status gives points queued and in flight for the host and for each stream, and each waiting place's points.

**Two bounds.** A stream may state `points` beside `agents`. Its agents bound stays, since accounts and their usage limits count the sessions running at once (record 0020), and a stage starts only while both leave room (70). The points bound holds a place's first start only: a place's first start waits while its stream's points in flight and its own would pass the bound. A place larger than the bound would then never start, so where nothing is in flight it starts alone. Within a stream such a place holds the first starts behind it, so a stream of small places never passes a large one by for ever; it holds no other stream's starts, as a full stream never does (70), and no later stage of a place already started. A fix round puts its place's points in flight again whatever the bound, since work begun is finished before more is started; it waits only for agents' room. A host's own points bound is not stated: the host bound exists because the machine and its accounts count sessions, while points are what a stream spends, and the host's totals are in status.

**Pace.** A stream may state `pace = { points = <n>, per = "hour" | "day" | "week" }`. `per` takes exactly those three, the user's options; any other word is a host configuration flume refuses, naming it. A place's first start charges its points to its stream then, and a first start waits while the points its stream started in the span just past and its own would pass the pace; where none were started in it, a place larger than the pace starts alone. Pace is no priority: priority orders what starts (70), pace caps how fast a stream spends, and a place within its stream's pace still waits its turn. A charge stands once made, the place dropped among it, and no later stage or fix round charges again, since the user's measure is work begun. The charges are kept in the host's own documents, `streams/<stream>.json` (153), so every flume process honours them and a tidied place leaves its charge, kept a week, the longest span a pace names. Status gives each stream's pace, the points started in its span and when the next charge leaves it.

**The span is rolling.** The span is the one ending at each start. A rate the user can rely on, 13 points a day, then holds over every day-long span: a calendar's day lets a stream start 13 points at 23:59 and 13 more at 00:01, twice its pace in two minutes, which is what a side project's pace is there to stop. A rolling span needs no time zone and no first day of the week, and says plainly when room comes back: when the oldest charge in it leaves.

**Work with no points.** A launch that gives no points, while its place's stream states a points bound or a pace, is refused naming the stream (4). Counting it as 1 (option 4) would let unestimated work through a pace a point at a time, so a stream of 13-point work launched without points would run at thirteen times its pace; counting it as nothing (option 5) would let it past both. Either is a guess flume would be making, and flume never estimates. On a stream that states neither, points are optional and only reported. This is no refusal for want of room, which flume never makes (70): every launch that gives points is taken and waits its turn. A role woken onto such a stream needs `points` in its table.

**Tokens are reported, never limited.** The agent seam's usage (152) gains the tokens an agent wrote out; at each agent's end flume adds its tokens to its stream's document, and status gives each stream's tokens over the last day and the last week. Tokens are known only once spent, so flume bounds nothing on them, and nothing on machine time, which is the merge queue's.

**Why not the others.** Option 1 asks the caller to weigh steps it did not design and cannot see, and gives the queue no one number per piece of work. Option 3 doubles a pace across every boundary and needs a zone and a week's start. Options 4 and 5 guess, as above.

### Consequences

- Clauses 4, 7, 8, 56, 70, 99, 143, 152, 153, C.4, C.6 and C.7 of flume's statement are amended, and 165 to 168, S69 and S70 added, with section 3's points, pace, room, in flight and waiting for room, at blueprints 9c7490c. Rule core.26 is amended and core.38 drawn from this record.
- Built after the mode (record 0024), with the rest of flume's first build: `--points`, the streams' `points` and `pace`, the stream documents, the two bounds in `Queue`'s settle, and status.
- What the sluice profile reads for a submission's times, its `submitted` and the times of its decision and its runs, is checked against the pinned sluice at the build; any it does not give is reported unknown.
- A caller that balances across hosts, Flywheel's later, reads points queued and in flight from each host's status; flume balances nothing across hosts (1, 70).
