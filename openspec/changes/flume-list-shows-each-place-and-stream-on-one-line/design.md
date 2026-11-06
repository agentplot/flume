# Design

## Context

A person asks what flume holds and what they can open. flume answers
that today only through status. With no place named, status reads every
fact of every place where it lives (58). It reads the change at each
head through the change profile, which archives the head and runs
OpenSpec over it. It reads the merge queue's record of each branch, and
for each live agent its state, screen and words. Then it reads the
host's pools, sources and terminals. On deneb on 2026-11-05, flume 0.9.1
held 42 documents among 54 place folders:

| asked | time | printed |
|---|---|---|
| `flume status` | 75 s | 984 lines |
| `flume status --json` | 81 s | 474 KB |
| `flume status --line` | 2.6 s | 42 lines |

The status the user ran printed each agent's last words whole and each
readiness step. It printed sluice's refusal to read an old policy 42
times across eleven places. Four landed places printed their phase as
JSON with their teardown's error inside it: `sluice release` failed to
read `~/.config/sluice/config.toml`, which holds a `[[caches]]` table
the pinned sluice no longer takes. `--line` is fast only beside status.
It still reads the change at each live place's head. Its lines name no
repository, though the host's places came from fifteen. And no command
lists the streams, which are what `flume open` takes (186).

The statement's amendment, on blueprints' branch
`design/flume-a-list-of-places-and-streams` at be85a32, adds the list
(200), the terminal seam's runs (201), herdr's and the printed
command's runs (C.5), `list` among the commands (section 9) and S108.
Record 0049, proposed, decides it, and core.52 holds it.

This change touches the area core.md binds; docs/architecture has no
area page. It relies on core.1 and core.2 (runs named only in a
profile, through one interface), core.13 (no process started), core.14
(one core), core.15 and core.16 (no fact kept, nothing written),
core.17 (safe to repeat), core.29 (herdr bounded), core.35 (the site's
words), core.48 (nothing opened) and core.52, which this change draws.

## Goals / Non-Goals

**Goals:**

- `flume list` answers within a second on a host with fifty places,
  whatever their changes, submissions and agents hold (200, S108).
- One short line per place, grouped under its repository, in the words
  the place's line already uses (56, 139).
- One line per stream the host declares, with its session and whether
  that runs (107, 186, 201).
- Nothing settled, started, sent or written (57; core.13, core.16).

**Non-Goals:**

- Changing status. It reads every fact where it lives (58), and its
  answer only gains fields (61). Its every-place form stays as it is;
  the list is the answer a person reads at a glance.
- Changing `status --line`, the place line or the plugins' surfaces.
  The Claude Code plugin's MCP server keeps its one tool, `status`.
- Filters. The list gives every place and every stream; a person names
  one place to `flume status`.
- Mending the host's sluice configuration or sluice's reading of old
  policies. Those are the host's and sluice's, and this change's
  construct stage told its caller of them.

## Decisions

### 1. The act and its answer

`crates/flume-core/src/list.rs` adds `Engine::list(&self) -> Answer`:

```rust
/// Answer::List: every place on the host and every stream it
/// declares, a line each (200).
List {
    places: Vec<Listed>,
    streams: Vec<StreamListed>,
},

pub struct Listed {
    pub place: PlaceId,
    pub repository: String,
    /// `<kind>--<name>`.
    pub name: String,
    /// `<kind>@<version>`.
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    pub state: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waits_on: Option<WaitsOn>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kept: Option<String>,
    pub line: String,
}

#[serde(rename_all = "snake_case")]
pub enum WaitsOn { Caller, Host, MergeQueue }

pub struct StreamListed {
    pub stream: String,
    pub session: SessionHeld,
    pub multiplexer: ProfileHeld,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<SessionRuns>,
    /// Why no runs was asked: the stream's multiplexer names no profile
    /// flume carries (190).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncarried: Option<String>,
    pub line: String,
}
```

Its JSON is `{"list":{"places":[…],"streams":[…]}}`, as every answer is
tagged (61). `Listed` gives the place's location as `place`, so a
program names it to any command from anywhere. The list refuses
nothing: a host with no places gives none, and a stream that cannot be
read says so on its own line.

The places come by repository, then by `<kind>--<name>`, then by the
order the store gives their documents, so a place tidied twice keeps
both its documents in order. The streams come in status's order, by
priority and then by name (`room.rs`'s `host_status`).

### 2. A place's line

`<kind>--<name> · <stage> · <state>`, then ` · waits on …` and
` · kept …` where they hold, joined by ` · ` as a place's line is
(`line.rs`'s `SEPARATOR`). A part the place has none of is left out.

- **The stage** is the one the place's line names, `stage_at` (made
  `pub(crate)`), as `label` gives it with no group: a live place's
  current stage, else the first it reached without a start. A
  repeating stage's group is left out, since it is read from the change
  at the head. A place that is not live names no stage.
- **The state** is `state_words`, made `pub(crate)`, given the held
  state, the stage and the asking mark, so the list and the line give a
  place one word (56, 139; core.14). It is `being made`, `landed`,
  `dropped`, `done`, `stalled`, `held`, `waiting on an answer`,
  `asking`, `working`, `waiting for room`, `submitted` and the rest
  that the place-line spec lists.
- **Whom it waits on** comes from the held state alone. It is `waits on
  its caller` where the place's `waiting` stands, stalled or not, since
  every move 130 names is the caller's. It is `waits on the host` where
  a stage waits for room and nothing waits on the caller (70). And it
  is `waits on the merge queue` where the state is `submitted` (111).
  `asking` from the agent's own mark adds nothing, since the agent's
  own question waits on whoever answers its pane (90).
- **What keeps a place whose work is over** (64 to 66, 68), from its
  phase, in this order:

  | phase | words |
  |---|---|
  | over, kept by `Teardown { step, .. }` | `kept by <step>` |
  | over, kept by `Holds` with a change or a file | `kept holding work` |
  | over, kept by `Holds` with a lock alone | `kept holding a lock` |
  | over, kept by `Holds` with both | `kept holding work and a lock` |
  | over, kept by `AgentLive` | `kept while its agent runs` |
  | over, kept by `Unreadable` | `kept unread` |
  | gone | `being tidied` |
  | tidied | `tidied` |
  | over, nothing kept yet | nothing: the next tidy removes it |

  The step is the teardown's own name for it, such as `sluice
  release`. The teardown's words and what the holds name are left to
  status.

So a landed place whose teardown failed reads
`chore--tailrace-is-renamed-timber · landed · kept by sluice release`,
and a held construct reads `change--boom-is-designed-in-its-own-records
· construct · held · waits on its caller`.

### 3. What the list reads

`Engine::list` reads `store.read_every()`, each document's state as it
stands, with no lock taken and no version moved. For each live start it
asks the agent profile its program names for `asking`, from the
start's folder and identity, as `line_of` does. That read is the
agent's own record and, where a mark stands, the transcript it names
(90). Nothing else of a place is read. Neither `bound` nor `facts` is
called, so no repository's binding is resolved. No head is read from
git's history, no change or deliverable read, no merge queue asked. No
`state`, `screen`, `shown_at` or `kept` is asked of a terminal.
`place_clues`, `read_agent` and `host_status` are not called. Each of
those reads where a fact lives, and the list needs none (200; core.52).

What the list says is what the last settle left (57). A place reads
stalled, or landed, once a settle has read it so. The herdr plugin's
watch settles within its bound of thirty seconds.

### 4. A stream's line

For each stream `host.streams()` gives, `default` always among them,
the list resolves:

- its session as `open_stream` resolves it, `StreamRoom { session:
  declared.session, .. }.session(name)`: the stream's own `session`,
  else its name, else the terminal's own for `default` (107, 143);
- its multiplexer from `host.display_for(name)` (190), and the terminal
  by that profile's name from the resolver;
- where the resolver carries no such terminal, `uncarried` as
  `terminals.rs`'s `uncarried` words it, and no runs asked;
- otherwise `terminal.runs(session.name.as_deref())`, asked once for the
  stream.

Its line is `<stream> · session <name> · <runs>`. The name is the
session's, or for the terminal's own the name runs gives, else `its
terminal's own session`. The runs part is `runs`, `not running`, `no
session` for `NoPart`, `not read` for `Unread`, or `no profile for
<multiplexer>` where it is uncarried. Where the terminal holds no part,
the session part is left out, since no session runs there: under the
printed command a stream's line reads `work · no session`. The JSON's
`session` keeps what the stream resolves, and its `runs` carries the
terminal's words for `NoPart` and `Unread`.

### 5. The terminal seam's runs

`crates/flume-seams/src/terminal.rs` gains:

```rust
/// runs: whether a part of the terminal runs now, with the name the
/// terminal knows it by; or that the terminal holds no part, or could
/// not read it, in its words (201).
#[serde(rename_all = "snake_case")]
pub enum SessionRuns {
    Runs { name: String },
    NotRunning { name: String },
    NoPart { reason: String },
    Unread { reason: String },
}

/// Whether the part named `session`, the terminal's own where none,
/// runs now, starting nothing, within a bound the profile states (98,
/// 201).
fn runs(&self, session: Option<&str>) -> SessionRuns;
```

It has no default, so every terminal states its own answer (core.2).

### 6. herdr's and the printed command's runs

herdr's runs reads `herdr session list --json` through the profile's
`run`, bounded at one second (C.5; core.29), with the caller's
`HERDR_*` stripped as every call is. It finds the session by `name`,
the profile's own `Server::Session` name, `flume`, where none is given:

- listed with `running` true: `Runs { name }`;
- listed not running, or not listed: `NotRunning { name }`;
- a listing that times out, exits non-zero or does not parse:
  `Unread { reason }` in herdr's words.

A profile whose own server is held by its socket alone, as tests build
one, finds its own part by `socket_path` in the same listing, and reads
`Unread` naming the socket where none lists it. runs never calls
`socket_of` or `start_session`, which start a server where none runs.
One listing for each stream's line is a call of milliseconds; on deneb
`herdr session list --json` took 8 ms. The profile keeps no listing
between calls.

The printed command's runs answers `NoPart` with the words its attach
gives, that it holds no session, its agents running in the commands it
printed (187, 201).

### 7. The person's form

`print.rs` prints `Answer::List` as each repository's path on a line of
its own, the path as the place's document holds it, each of its
places' lines under it indented by two spaces, then `streams`, then
each stream's line indented the same:

```console
$ flume list
/Users/chuck/Code/github_agentplot/flume
  change--flume-list-shows-each-place-and-stream-on-one-line · construct · working
  chore--tailrace-is-renamed-timber · landed · kept by sluice release
  fix--a-spent-accounts-reset-is-read-with-its-date · code · waiting for room · waits on the host
/Users/chuck/Code/github_agentplot/boomdrive
  change--boom-is-designed-in-its-own-records · construct · held · waits on its caller
streams
  madswan · session madswan · runs
  swancloud · session swancloud · not running
  default · session flume · runs
```

A host with no places prints `no places` before `streams`. The lines
are the answer's own `line` fields, so the print computes nothing
(core.14). `--json` prints the answer as every command does.

### 8. The command

`flume list` takes no argument. It needs no repository, as `open` needs
none, and the global `--repository` changes nothing it answers. It
builds the core as every command does and asks `Engine::list`. It
settles nothing first, as status does not (the commands reference's
"Settling first"), and always exits 0. Its help says it gives a line
for each place and each stream, reading only what flume keeps, and
that `flume status <place>` gives a place's every fact.

### 9. Beside the changes in flight

`where-an-agent-opens-is-given-by-kind-and-by-stage`, constructed and
held, has a stream name a session for a kind and for one stage of a
kind (199), which `flume open <stream> --session <name>` shows. Its
sessions are the stream's too, and the list is what a person reads to
know what they can open. Whichever of the two changes lands second
gives each session a stream names for a kind or a stage a line of its
own under the stream's: `<stream> · session <name> for <kind>[.<stage>]
· <runs>`, by one runs each. This change's task 3.2 builds it where that
change has landed on main when the group is built. That change's build
adds it where this one landed first.

`every-question-a-place-asks-reaches-someone-who-answers-it` makes a
role's address `<stream>/<role>` and has a role's run read the inbox's
list for its address. That list is `flume inbox`'s, not this one, and
neither reads the other.

### 10. The fakes

The fake terminal answers runs from a script, `runs_as(part,
SessionRuns)`, and otherwise `NotRunning` for the part named, its own
named `own` as its attach names it. It logs `runs <part>`, starting
nothing. The engine's scenarios read the fakes' logs to hold what the
list does not ask: the fake change, merge queue and history log no
read, and the fake terminal logs nothing but `runs` lines.

### 11. How it is held

- **seams:test.** `round_trip.rs` writes each `SessionRuns` to JSON and
  back unchanged.
- **fakes:test.** `fakes.rs` scripts the fake terminal's runs for `work`
  and asks `work`, `office` and its own.
- **conformance:test.** `seams/terminal.rs` holds runs' scenario over
  the fake (201). `engine/list.rs` holds the-list spec's scenarios over
  the fakes, citing 200, and S108's, named `s108_the_list_of_fifty_places`,
  over fifty documents in three repositories. `engine/answers.rs`'s
  field list holds `list`'s fields (61).
- **core:test.** `list.rs`'s unit tests hold the kept words' order and
  the places' order.
- **flume:test.** `print.rs`'s tests hold the person's form. The
  command-line spec's scenarios run in `tests/commands.rs` over a
  scratch host. One runs `flume list` outside any repository, with
  `PATH` set to an empty scratch folder, so any tool the list ran would
  fail. Another writes fifty places' documents, made from a launched
  place's under fifty names in three repositories, and holds the answer
  within a second. `tests/profiles/terminal_printed.rs` holds the
  printed command's runs.
- **flume:tools.** `tests/profiles/tools/terminal_herdr.rs` holds
  herdr's runs against the pinned herdr under a scratch session home:
  a session not listed reads not running and stays unlisted, one
  started reads running, and the own session reads by the name `flume`.
  A stand-in herdr of the test's own that sleeps past the bound reads
  `Unread` within two seconds.

### 12. The site

`site/src/content/docs/reference/commands.md` gains `flume list` in its
table, under "Settling first" among the commands that only read, and a
section of its own with the example of 7. `reference/state.md` gives
the answer's fields. The words name herdr and sluice only as products
flume integrates with (core.35).

### 13. The clause list

`crates/flume-conformance/src/clauses.rs` reads the revision of
blueprints main that lands record 0049's amendment, 200 in Part A, 201
in Part B and S108 among the scenarios, each cited by this change's
scenarios. Until the amendment lands on main, the list's revision is
main's, and group 4 waits for it.

## Risks / Trade-offs

- [The list shows what the last settle left] → A place under a terminal
  that keeps no watch moves on only at flume's commands, as the
  commands reference says. The list does not settle, since a settle
  reads every fact where it lives and would cost the seconds the list
  saves.
- [A stage's group and a change's tasks are missing] → `status --line`
  and `flume status <place>` give both. The list names the stage, which
  is what a person scans for.
- [Reading the asking mark reads a transcript] → Only where a mark
  stands, and only the transcript the agent's own started ack named,
  as every line already does at each render of a pane.
- [herdr's listing is one call per stream] → A host declares a few
  streams. Each call is bounded at one second, so a herdr that does not
  answer costs a second per stream it serves. The list stays within
  its second only while herdr answers, and says `not read` where it
  does not.
- [Another operation on the terminal seam] → Every terminal profile
  answers it, the printed command with one line.

## Migration Plan

Nothing migrates. `Answer` gains a variant and the terminal seam an
operation; every answer and document reads as before (61). A host's
operator runs `flume list` once the release carrying it is installed.

## Open Questions

None for the build. Record 0049 waits on the user's yes, and the
statement's amendment on its landing on blueprints main, before groups
2 to 5 are built.
