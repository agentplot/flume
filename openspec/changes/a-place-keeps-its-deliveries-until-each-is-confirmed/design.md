# Design

## 1. Status reads the key from the record

`EndStatus::of` names `delivery` from the end's delivery in what one
read of the host's deliveries holds, `owing`, never from the place's
token alone. An end not yet recorded keeps its `pending_for` as before,
each recipient it is owed to, with no key and no expiry, so a caller
sees what it waits on and confirms nothing flume has not recorded (43;
weir 101, 102).

## 2. The let-go of the gone reads every place twice

A tidy that keeps a place's document for its pending ends writes the
kept one, `<location>@<at>`, and then removes the place's own. A read of
every place lists the documents and then reads each, so one that listed
before the kept document was written and read the place's own after it
was removed holds neither. `let_go_of_the_gone` therefore takes a place
as gone only where a second read of every place, begun once the first
ended, lacks it too: the second lists the kept document, written before
the first ended. The second read is made only where the first left a
done delivery's place out.

The path not taken: dropping the let-go of the gone, leaving a done
delivery whose removal's let-go did not land in the host's deliveries
for good. That keeps nothing a recipient has not confirmed either, but
the document only grows, and record 0061's design 9 asks for the let-go.

## 3. A place's own document stays until the kept one stands

`keep_for_ends` gives why it kept nothing where the kept document could
not be written and none stands under its id. The tidy then removes
nothing of the place's held state, logs why, and the next settle tidies
it again from `Gone`, every step of which is safe to repeat (74). The
path not taken: removing the place's own document anyway, which leaves
its pending ends with no document for status, the inbox or a wait to
read them from.

## 4. The log lines

flume's log, the host's `flume.log`, gets one line per delivery a write
records, `recorded the delivery <key> of <place>, owed to <readers>,
expiring at <time>`, and one per delivery a write lets go of, `let go
of the delivery <key>, confirmed by <readers>: <why>`, `why` naming the
removed document or that no document of its place stands. Each is
written once the write lands, never for a try whose version moved, and
names no message, so no brief and no value reaches the log.
