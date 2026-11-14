---
number: 70
title: A caller relays a place's terminal through flume, a person's keys in and its screen out
status: proposed
date: 2026-11-14
decision-makers:
- Chuck Swanberg
- boomdrive-design
- the change flume-relays-a-places-terminal's construction agent
---

# A caller relays a place's terminal through flume, a person's keys in and its screen out

Status: proposed. The user's choice is boomdrivedev/boomdrive's record 0004, accepted by the user on 2026-11-07 on their card, and its statement is flume's 107, 221 and the printed command's paragraph, merged to boomdrivedev/blueprints' main at `41bd0a2` on the user's word of 2026-11-14. 222 and C.5's relay row are read as boomdrivedev/blueprints `b54a9b4` amends them, on its branch `design/a-relay-refuses-a-key-it-cannot-type`, which waits on the user's push. The choices below that are not the user's are boomdrive-design's ruling of 2026-11-14 and the construction agent's own, made on the rules written and cheap to change.

Deciders: the user, in boomdrivedev/boomdrive's record 0004. Their words: "a combo of #1 and #3 might make the most sense. Remember that Herdr should be a seam for multiplexers. We will want to support TuiOS shortly." boomdrive-design, on the keys herdr 0.9.5 cannot type, ruling on 2026-11-14 on the code stage's stop-short. The construction agent of the change `flume-relays-a-places-terminal`, on the rest.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` 4, 56, 95, 96, 98, 107, 120, 136, 190, 221 and the printed command's paragraph at `41bd0a2`, and 222 and C.5's relay row at `b54a9b4`; boomdrivedev/boomdrive's record 0004 and its terminal-relay spec; the change `flume-relays-a-places-terminal`, its design sections 2 to 12; herdr 0.9.5's `pane send-keys`, which refuses `delete`, `insert`, `home`, `end`, `pageup` and `pagedown`, bare and led, with `invalid_key`, read on deneb on 2026-11-14; records 0018, 0065 and 0068; rules core.1, core.11, core.13 to core.17, core.29, core.35 and core.49.

## Context and problem statement

A person may open a launch's terminal from its card in Boomdrive's cloud, and the runner carries that person's screen and keys between the browser and the place's live agent. As built, the runner ran herdr's command line itself, which stands against 107: flume opens, sends to and closes every agent's terminal, and no caller does. It also tied the runner to one multiplexer. The user chose that the runner reach a terminal only through a command of flume's, through flume's terminal seam, and that a relayed hand count as a person's (boomdrive record 0004). The statement now says how (107, 221, 222, C.5). This record holds how flume builds it.

## Considered options

1. **The lines.** JSON lines both ways, whatever `--json` says: `{"screen": …}`, `{"unread": …}` and `{"untyped": …}` out, the answer last; `{"text": …}` and `{"keys": [ … ]}` in. Or raw bytes out and in, or `--json` choosing between them.
2. **The keys.** flume's own key names, `enter`, `tab`, `esc`, `backspace`, `delete`, `insert`, `home`, `end`, `pageup`, `pagedown`, `up`, `down`, `left`, `right`, `space`, `f1` to `f12` and any one printable character, each led by any of `ctrl+`, `alt+` and `shift+` in that order. Or each terminal's names passed through.
3. **The seam.** A handle the core drives, its bound, a read and a type one call each. Or one call that runs the whole relay inside each profile.
4. **The agent's state.** Read once a second of the host's clock, and after a read the terminal did not answer. Or beside every read.
5. **herdr's bound.** One second for each relay call. Or the profile's ten.
6. **Keys herdr 0.9.5 cannot type**, `delete`, `insert`, `home`, `end`, `pageup` and `pagedown`, bare or led. (a) Drop them from flume's names. (b) Keep them, and have herdr's relay refuse each, named, with nothing sent for it and the keys around it typed. (c) Type them through `pane send-text` as the escape sequences an xterm sends.
7. **Rex.** Refuse relay. Or build one from Rex's `capture`.
8. **A settle.** None. Or a settle first, as an act takes one.

## Decision outcome

1. **JSON lines both ways.** A caller tells a screen from an end and from a refusal, and the form only gains fields, as every answer does (core.14). No person reads a relay's frames, so `--json` chooses nothing.
2. **flume's key names.** The caller names no multiplexer's keys, as the user ruled, so the runner stays free of herdr's spelling and TuiOS's to come. The seam carries a key as `Key`, read and written by its name, and each profile maps it to its terminal's own (core.1).
3. **A handle the core drives.** The loop, its ends and its reading of state are written once, in the core, over the fakes. A loop in each profile would put them in every profile.
4. **State once a second.** A read every 250 milliseconds with a state read beside it would double herdr's calls for an end a second later. A read the terminal does not answer reads the state at once, so a gone agent ends the relay without waiting.
5. **One second.** herdr answers in milliseconds, and a read that waits ten seconds holds the person's screen. A read past it is unread, and an input past it untyped, naming the bound (98; core.29).
6. **(b), boomdrive-design's ruling.** The six stay flume's names, which a later herdr may type. Under herdr 0.9.5 each is refused, named as a key herdr 0.9.5 cannot type, with nothing sent for it, and the keys around it are typed, each run of them one `pane send-keys`. The input is given back as untyped with those words (222). Not (a): the names are flume's, not herdr's. Not (c): `pane send-text` may paste an escape sequence as text rather than type it as a key, and C.5 sends keys by `pane send-keys`.
7. **Rex refuses.** C.5 gives Rex no relay row, so flume's Rex profile refuses relay before any call. A relay of Rex is the statement's to add first (core.19).
8. **No settle.** The relay is no act (221). It settles, writes and keeps nothing, and a relay of a place whose held state still reads its agent live after the agent went finds it gone at its terminal at once.

The relay reaches the current stage's live agent through the terminal its start recorded (190; core.49). It writes no held state, host document, file, log line or telemetry, records no act, and keeps nothing it carried but the last screen given, to compare the next with. What herdr is given to type travels as its arguments, and a refusal names `pane send-text`'s text as `<typed>`, so no words of flume's repeat what a person typed. The printed command refuses relay, since it holds no part of a terminal a person can be shown (C.5).

### Consequences

- Rule core.57 says how flume relays and what it keeps of a relay, which is nothing. Rule core.11's details gain that a relay's text and keys, typed as the person typed them, are not ruled out.
- Good: the runner drops its own herdr relay for `flume relay`, and TuiOS needs only a profile's relay.
- Bad: under herdr 0.9.5 a person's Delete, Insert, Home, End, Page Up and Page Down do not reach the agent. The caller is told which key did not go in, and a herdr that comes to type them shows in the tools layer, which still reads the pinned herdr refusing each.
- The suite's clause list stays at `31f9f1c`, as record 0065's construct left it: moving it to main's revision brings 211 to 224 and S121 to S131 with it, each to be cited or named unbuilt, and is a change of its own. The relay's scenarios cite the clauses the list holds and name 221 and 222 in their words.
- Elsewhere: Boomdrive's runner relays through `flume relay` and drops its own herdr calls, a change of its own (boomdrive record 0004).
