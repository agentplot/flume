# Design

The statement is read at boomdrivedev/blueprints main 41bd0a2: 24, 25,
49, 112, 129, 131, 140, 146, S52 and S131; record 0065 decides it.

## 1. The count is kept, since a stage keeps only its last start

flume holds one start per stage, `StageHeld::start`, so the head, the
time and the length of `made.jsonl` an earlier start began at are gone
once the next start is written. The point a deliverable counts from is
therefore kept in held state, as `flume_seams::Counted`:

- `start`, the number of the start that began it;
- `at`, when that start began, which a report's write time is read
  against (25);
- `head`, the branch's head that start began at, which a commit since,
  the newest commit touching the change and a committed report are
  read against (24, 25);
- `made_from`, how many lines `made.jsonl` held then (129).

`StageHeld::counts` is the stage's count, the first of its starts since
it last delivered. `StartHeld::counts` is the count a start reads its
deliverable from where it began at an earlier start, and is none where
the start counts from itself. `StartHeld::counted()` gives the one or
the start's own `n`, `started_at`, `start_head` and `made_from`. Both
fields are left out when none, and read as none in a document written
before (core.16).

The count is copied onto each start so that a reading after the stage
let go of its count, status's of a delivered stage among them, still
reads the deliverable from where the start's count began.

The path not taken: keeping every start of a stage. That grows the
held state by a start's whole record at each retry, to keep four
fields of one of them.

## 2. Which starts carry the count

At a start's claim, where it writes the `StartHeld`, under the room
lock:

- where the stage has no count, the start begins one: the stage's
  `counts` is the start's own point, and the start's `counts` is none;
- otherwise, for a start of a repeating stage given a group, see 3;
- otherwise the start's `counts` is the stage's.

Nothing asks why the stage starts. A retry of a stage that stopped
short, was stuck, lost, stopped or never begun, the start after a spent
account (146), a lost conversation's retry (49) and a stage refused at
a start and started again all find the stage's count and carry it, and
so does any later start of a stage that has not delivered.

## 3. A repetition counts from the first start given its group

A start of a repeating stage given a group carries the count of the
stage's last start, `prev.counted()`, where that start was given the
same group, by number and title, as 140 reads a group at the head;
otherwise it counts from itself. A start with no group, every task
ticked at its head, takes the stage's count, since it delivers as the
stage does (140). So a retry after a repetition that ticked the last
group and stopped short starts with no group and delivers on what that
repetition committed; and a repetition after one that delivered its
group is given the next group and counts from itself.

The stage's count is not let go between repetitions: a repetition's
delivery is no delivery of its stage (140).

## 4. What lets the count go

- **The stage's delivery.** `ended` with `Delivered` or `Finished`
  sets the stage's `counts` to none, once its end has read its lines
  (5). Any other end, `Repeating` among them, keeps it.
- **A fix round.** `fix_round` sets the `counts` of the stage it sends
  the place back to to none, for a failed submission and for a finding
  alike (112, 131). That stage has delivered before a failed
  submission, and has none then; the stage it names may not have
  delivered since a retry skipped it, and 25 starts a fix round afresh
  whatever came before.

A retry of a stage that has delivered starts afresh because that stage
holds no count, so the retry needs no rule of its own.

## 5. What reads the count, and what stays the start's own

From `StartHeld::counted()`:

- `delivered`: `holds_since`, `newest_touching` and `in_commit_since`
  take the count's `head`, and a report's write time is compared with
  its `at` (24, 25);
- `made_since` and `unmade`: the lines after the count's `made_from`
  (129).

The stage's end, at `ended` and in `read_out`, carries the lines after
the stage's count's `made_from`, so a repetition's end and the stage's
carry every line since the stage started, as 129 and 140 ask. An end
that stopped short may carry lines the next end carries again; each
carries what its stage made since it started.

The start's own: the commits a stuck, lost or never-begun end names
(43), `first_quiet`, `last_head` and `last_head_move` (44, 118), when it
waited and every timing status gives. They describe what this start's
agent did.

The stop check's and the prompt's words, "made since the stage
started", are unchanged, and true under 25. flume's own kinds' prompts
are released and unchanged (core.34): an agent that finds its work
committed ends its turn, and the stage delivers.

The path not taken: a status field naming the start the count began
at. 25 asks nothing of status, and the end's head shows what it
delivered at.

## 6. A going-back past a stage leaves its count

A fix round before a stage, and a retry of a stage before it, set it
`NotReached` and run it again in order (49, 112). Its count stays: 25
names the first of its starts since it last delivered, and only a fix
round of the stage itself and its own delivery let it go. A stage that
delivered before the going-back holds no count and starts afresh. One
that did not, a last stage that found work among them, counts from its
first start since, so what the stages before it made since then counts
toward its deliverable. Its fresh agent still runs and checks the work,
and where it judges the work short it stops short again; record 0065
accepts that an agent that wrongly finds its work done ends its stage
delivered, the later stages and the merge queue checking it.

The path not taken: starting every stage a going-back resets afresh.
That reads 25 as saying more than it does, and would be a change to the
statement first (core.19).

## 7. A document written before

A stage whose held state holds no `counts` begins its count at its
next start, as flume counted it before. Its `made_from`, which a
repeating stage between repetitions kept, is no longer written; where a
document written before holds it, the stage's end carries its lines
from it, as before, and lets it go.

The path not taken: deriving a count from the stage's last start where
its state shows no delivery. The document holds no start before it, so
that would guess at where the stage started.

## 8. The suite cites what its list holds

The suite's clause list reads the statement at 31f9f1c, which holds 25,
49, 112, 131, 140, 146 and S52 but not S131, so a scenario that cites
S131 would fail coverage as citing what the statement does not hold.
Each scenario here cites the clauses the list holds and names S131 in
its words. Moving the list to main's revision, with clauses 211 to 224
and S121 to S131, is its own change.

## 9. The rule and the site

Rule core.9's details gain what 25 now rules out and allows, citing 25,
S52, S131 and record 0065. The site's guide to kinds says when a stage
started for its deliverable.
