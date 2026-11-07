---
number: 34
title: Flywheel is renamed Boomdrive, its unit of work a drive and tailrace timber, and flume's tree names them so
status: accepted
date: 2026-11-04
decision-makers:
- Chuck Swanberg
---

# Flywheel is renamed Boomdrive, its unit of work a drive and tailrace timber, and flume's tree names them so

Status: accepted by the user on 2026-11-07, on their card, on their words below.

Deciders: the user, renaming Flywheel across every repository at once, in the brief of the chore `flywheel-is-renamed-boomdrive` on 2026-11-04: "the ecosystem, the org, the branding, the cloud and the opensource runner tool are all boomdrive. There is also a "boom" open source tool that does the claim reconciliation."; and, as the brief carries their rulings, that a customer's unit of work becomes a drive, which they approved for ubiquitous language so the interface's words and the code's are the same; that crew's word for a partition's state branch follows the unit's word; that tailrace becomes timber; and that the same pass spells enroll and enrollment, never enrol or enrolment. Which of flume's mentions are live and which are history, and how flume names what the map does not rename, are the renaming agent's.
Sources: the chore's brief of 2026-11-04, its map below; the repositories as GitHub answers for them on 2026-11-04: agentplot/flywheel-cloud as agentplot/boomdrive-cloud, agentplot/flywheel-runner as agentplot/boomdrive, agentplot/flywheel-web as agentplot/boomdrive-web, agentplot/tailrace as agentplot/timber, agentplot/flywheel archived, and agentplot/flywheel-next private, unarchived and under that name, its twelve crates, `flywheel-engine` among them, still so named; `agentplot/blueprints:design/flume/requirements.md` at ebfbc0d, C.2 ("the original Flywheel's flywheel-engine moved into agentplot/weir"); records 0004, 0008, 0020, 0021, 0023, 0025, 0027, 0028 and 0029; rules core.19, core.35 and core.42; `styles/Flume/Headings.yml`; `@agentplot/site-theme` at v0.1.3, its `src/family.ts`, which flume's site takes for its family bar and footer.

## Context and problem statement

The user renamed Flywheel Boomdrive: the product, its brand and ecosystem, the family of open tools on their sites, its cloud and its runner, each repository with its name. The rename runs in every repository at once, and the work paused for it resumes on the new names.

No code of flume's names Flywheel, tailrace or enrol: no crate, command, configuration key, variable or test. The names live in seven records, two rules, the headings' style the site's prose is checked by, and one archived change. Some of those mentions say what Boomdrive does or will do, and some quote a person, cite a repository at a revision, or tell what happened on a day. flume's records also call agentplot/flywheel-next "the original", the product's earlier implementation, whose engine record 0029 moves into weir; the map does not name flywheel-next.

## Considered options

1. Every mention renamed, a person's quoted words, a past day's paths and the archived change among them.
2. Every live mention renamed. A person's quoted words, a past day's paths, the archived change and the names a repository still carries on GitHub stay as they were.
3. The rules renamed and the records kept whole, as history.

## Decision outcome

Option 2.

**The map.** These are the user's names, for every repository:

| Was | Is |
|---|---|
| Flywheel, the product, its brand and ecosystem, and the family of open tools on their sites (sluice, flume, timber, weir and the rest) | **Boomdrive**, one word, capital B in prose; `boomdrive` in code, commands, packages, paths and repository names, and `Boomdrive` where the code's case needs a capital; never Boom Drive, BoomDrive, `boom_drive` or `boom-drive` |
| Flywheel Cloud, the control plane, agentplot/flywheel-cloud | Boomdrive Cloud, agentplot/boomdrive-cloud |
| the runner, agentplot/flywheel-runner, its command `flywheel` and its crates `flywheel` and `flywheel-hosted` | Boomdrive, agentplot/boomdrive, its command `boomdrive` and its crates `boomdrive` and `boomdrive-hosted` |
| agentplot/flywheel-web | agentplot/boomdrive-web |
| tailrace, agentplot/tailrace, its crate, command, site and records | timber, agentplot/timber |
| a customer's unit of work, a Flywheel; its Durable Object class `Flywheel` | a drive, drives; `Drive`, renamed while no object of the class exists, since staging has never been deployed |
| crew's flywheel, a partition's state branch | its drive |
| the runner's mark on its launches, `flywheel:<partition>/<n>` | `boomdrive:<partition>/<n>` |
| agentplot/flywheel, the original plugin | archived; a mention of it as history stays, naming it archived |
| enrol, enrols, enrolled, enrolling, enrolment | enroll, enrolls, enrolled, enrolling, enrollment |

boom is unchanged, and so is anything of a third party's that happens to say flywheel.

**What flume's tree does.**

1. **Live mentions are renamed.** Where a record says what the product does or will do, it says Boomdrive (records 0004, 0020, 0021, 0025, 0028 and 0029). Where a record cites a renamed repository, it names the new one at the same revision: agentplot/boomdrive-cloud's domain model at 2ff6630 (records 0028 and 0029), and timber among the kits whose `.config/sluice.toml` record 0023 read on 2026-10-29. Rule core.35 rules out Boomdrive on a page where it ruled out Flywheel, and its decision is unchanged. The headings' style allows Boomdrive in a heading where it allowed Flywheel.
2. **Record 0029 is retitled.** Its title and its file name say "the original Boomdrive's engine". Its number stays, and a citation names a record by its number.
3. **The original is flywheel-next, named as GitHub names it.** "The original" in flume's records is agentplot/flywheel-next, not the archived agentplot/flywheel, which no record or rule of flume names. The map does not rename flywheel-next, and GitHub answers for it under that name today, so its name and its crates' names, flywheel-engine among them, stay where they name what exists, and the product it was is called Boomdrive: "the original Boomdrive's engine, flywheel-engine". Rule core.42's names stay for the same reason. The chore that renames or archives flywheel-next renames them here.
4. **History keeps its words.** A person's quoted words stay as they said them: the user's "Tailrace" in record 0008, "Flywheel" in record 0025, and "flywheel-engine" and "flywheel-next" in record 0029's quotes of ruling T2 and its ledger item. Record 0027's paths are deneb's on 2026-10-31, the clone's folder then and now, since a host moves its own checkouts. The archived OpenSpec change `2026-10-26-the-core` keeps its text.

**Why not the others.** Option 1 changes what a person said and what a host held on a day, so a quote no longer matches its card and a path no longer names the folder the event happened in. Option 3 leaves the records saying Flywheel will attach to streams and balance across hosts, when the product that will is Boomdrive, and leaves record 0029's citations of agentplot/flywheel-cloud resting on GitHub's redirect.

### Consequences

- Rule core.35 cites this record. Records 0004, 0020, 0021, 0023, 0025, 0028 and 0029 change their names and nothing of their decisions.
- `styles/Flume/Headings.yml` lists `boomdrive` and `Boomdrive` among its exceptions, where it listed `flywheel` and `Flywheel`.
- flume's statement still names "the original Flywheel's flywheel-engine" in C.2, and weir's names Flywheel as a host and a reader. The statements' rename is agentplot/blueprints' own (core.19), and flume's clauses are cited by number, so nothing here waits on it.
- flume's site takes its family bar and footer from `@agentplot/site-theme` at v0.1.3, the release that carries site-theme's rename: its family list names Boomdrive and timber, by the ids `boomdrive` and `timber`, and links boomdrive-web and timber.
