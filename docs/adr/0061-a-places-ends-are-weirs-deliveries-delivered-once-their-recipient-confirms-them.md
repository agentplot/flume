---
number: 61
title: A place's ends are weir's deliveries, delivered once their recipient confirms them
status: accepted
date: 2026-11-08
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# A place's ends are weir's deliveries, delivered once their recipient confirms them

Status: accepted on 2026-11-09 by agentplot-design. The direction is the user's and accepted, in weir's record 0006. The rules below that weir leaves to its deliverer are the construct stage's own, made on 2026-11-08 in the change `flumes-inbox-is-weirs-delivery-form`, each cheap to change, so no card of the user's is asked for them.

Deciders: the user, in agentplot-design's pane on 2026-11-08, on how flume and sluice hand a reader what it waits for: "it seems like we're maybe inventing multiple similar solutions", "Is it actually tracking what was read?", "Is it done well?", and "So would we move a shared crate under Weir so that it's loose and Flume could use the same inbox system?"; then on agentplot-design's card of the same day: "Runner as one more recipient now; one shared inbox in weir next". weir's record 0006, accepted on that card, fixes the form and its order of work: "Flume, then sluice, move onto the crate, each in a change of its own." The brief of this change carries it: "Flume's ends are owed and delivered by weir's delivery form: a notice, a wait or an inbox hands an end over, it is done only once its reader confirms it, and one nobody collects expires and is shown in status."

Sources: boomdrivedev/weir's record 0006 and its crate `weir-delivery` and pack at weir's main 52b9552; boomdrivedev/blueprints `design/weir/requirements.md` A.14, B.7, C.8, I9 and S22 to S25 at 3c92bcb; `design/flume/requirements.md` as amended for this record on `design/flume-ends-are-weirs-deliveries` at 01b637a, over record 0060's amendment at 479a6ab merged with main at 3c92bcb (vocabulary, 43, 56, 62, 132, C.4, C.7, the runner's row, S121 to S123); this repository's record 0060; `crates/flume-seams/src/held.rs` (`EndHeld`, `Delivery`, `DeliveredBy`) and `crates/flume-core/src/ends.rs`, `wait.rs` and `wake.rs`.

## Context and problem statement

flume records each end of a place once, with a list of its deliveries, to whom, when and by what. An end counts delivered to a recipient the moment it is handed over: a notice once the terminal took it, a wait once it answered, the inbox once it listed it. A recipient that stops before it acts on what it was handed never sees it again: a role's run ended mid-turn, a runner that crashed between its inbox and its report, a session whose agent was cleared. Nothing pending ever expires, so a misspelt recipient keeps a tidied place's document for good, and nothing says so.

weir now holds one form for this, a delivery, and one crate, `weir-delivery`, for every tool that owes a reader a message. A delivery is owed to each reader until that reader confirms it by its key. A hand-over confirms nothing unless the reader says so in the call that hands it over. A delivery expires at a time fixed when it is recorded, and is then shown expired with its reader, never dropped. A name no reader confirmed under is shown as such. A deliverer lets go of a done delivery alone. weir leaves its deliverer how it hands a delivery over, when it hands one again, the span before one expires, who may confirm for a reader, and where it keeps its records.

## Considered options

For where flume keeps the records:

1. **One host document of weir's records (chosen).** Every record of C.8 for the host's ends, in the order added, in one document of the host's own beside the places', written whole by compare-and-set. It is the store weir's `Store` reads and adds to, and passes weir's pack as it is.
2. **Each place's document holds its own records, the known names in host documents.** One write records an end and its delivery together. But a name stays known after its deliveries are let go of (weir 106), so known names would live apart from the deliveries they were confirmed under, and no one store reads back what the pack adds in its order. It fails the pack.

For how a reader confirms:

1. **`flume confirm <key>…`, and `--confirm` on the wait and the inbox (chosen).** A reader confirms what it acted on, by the delivery's key, or says in the call that hands it over that it confirms on receipt (weir 103). A session's start confirms on receipt, since the plugin gives the agent the inbox whole.
2. **The inbox confirms by default.** Callers would keep today's collect-once. It is the hand-over counting as delivery that weir rules out.

For a tidied place's drop while an end is pending:

1. **Refused, naming each recipient it waits on (chosen).** Nothing a reader has not confirmed leaves its store (weir I9). The operator confirms as the recipient, `flume confirm --recipient <name>`, where a recipient will never come.
2. **The drop confirms for each recipient.** It would let the caller confirm under another's name silently.

## Decision outcome

The first option of each.

- **An end is a delivery.** The settle that records an end records its delivery, under the key `<kind>--<name>/<token>/<end key>`, the token one the place's launch took so no two launches of a place share a key. Its readers are the end's recipients as flume gives them now (7, 41, 130, 193). Its message is the end's notice. An end with no recipient, a drop of a place whose recipients are all roles' addresses, has no delivery.
- **Hand-overs.** A notice, a wait, the inbox and a session's start each record a hand-over, by `notice`, `wait`, `inbox` or `session-start`. A notice goes to a recipient once, while nothing has handed the end to it before. A wait and an inbox hand an owed end over each time they answer with it. A notice is claimed on the place before it is sent, as now, and its hand-over is recorded once the terminal answered sent, so no record of weir's is ever taken back.
- **Confirmation.** `flume confirm <key>…` confirms as the recipient the command names, its default as every recipient's is (7, 132). `flume wait --confirm` and `flume inbox --confirm` confirm on receipt. The plugin's SessionStart confirms on receipt. flume takes a recipient's confirmation from the caller naming it, as it takes its inbox, and never confirms for one itself.
- **Expiry.** `[host] ends_expire`, a time as `keep_open` is written, managed layer alone, a week where unset. An end pending past it is expired for that recipient: status, the inbox and a wait show it so, with the recipient, and the host's status lists every one.
- **Known names.** Status gives, for each recipient of an end, whether anything was ever confirmed under its name.
- **Waking.** A role is woken for an end pending for its address that was never handed to it. One a run was handed and did not confirm wakes nothing again, and the next run the address's other ends wake is handed it.
- **Done and let go.** A tidied place's document stays until every end is delivered to each recipient (62). Its drop is refused while one is pending. Once the document goes, flume lets go of its deliveries, and the known names stay.
- **What was written before.** An end written before, with deliveries of the old form, is recorded at the first settle, wait or inbox that reads it, each old delivery a hand-over with a confirmation at its time, since the old form counted it done. The rest of its recipients are owed it.
- **The form only gains fields (61).** An end keeps `deliveries`, now each hand-over, and `pending_for`, now each recipient that has not confirmed it, and gains its delivery's key, its expiry, each recipient owed with whether it is expired and its name known, and each confirmation.

### Consequences

- Good: a recipient that was handed an end and stopped before acting sees it again, at its next inbox or wait, until it confirms it.
- Good: a misspelt or departed recipient shows in status, expired, with its name never confirmed under, in place of holding a document no one sees.
- Good: one form and one crate with sluice once sluice adopts it, and flume's store held to weir's pack.
- Bad: every caller that collects must now confirm. crew's role kinds, the runner and a person's own scripts confirm what they act on, or are handed it again at each inbox.
- Bad: one host document takes every delivery's write, under compare-and-set beside the places'.
- Neutral: the statement is amended for it on `design/flume-ends-are-weirs-deliveries` at 01b637a, not yet on blueprints' main. Rules core.11 and core.25 carry it.
- Neutral: flume's merge-queue profile reads sluice's inbox as today. sluice moving onto the form is sluice's own change, `sluices-inbox-is-weirs-delivery-form`, after which the profile confirms what it collects.
