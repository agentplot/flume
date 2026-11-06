# How each multiplexer splits into a backend and the ways a person views it

Read on deneb on 2026-11-05. Nothing was installed or changed.

| | Backend (where agents run) | Viewed in a terminal | Viewed in a native app | Window title |
|---|---|---|---|---|
| **herdr 0.9.5** (installed) | One server per session, socket `~/.config/herdr/sessions/<name>/herdr.sock` | `herdr session attach <name>` | None for this backend | Herdr sets the terminal's title (`[ui] window_title`, default `{hostname}: {workspace}`); Ghostty's `--title` overrides it |
| **Herdr app** | **herdr-core** (`hc`): a different backend from herdr 0.9.5, with its own socket and sessions | `hc attach <terminal>`, one terminal at a time | `herdr-app do space.select` / `terminal.show` on the open window | Unconfirmed |
| **Rex** (Rex Beta 0.1.0, installed) | Rex Server inside the app: `rex server run`, socket `~/Library/Application Support/rex/server.sock`. The `rex` CLI (not on PATH) has `new`, `ls`, `window new`, `send`, `run`, `capture` | **None**: no attach command | `rex do session.select session_id=<id>` (Remote Control must be on in Settings) | `rex window rename`, `session.rename`; default unconfirmed |
| **cmux** | Two: cmux.app holds its own terminals, driven over `/tmp/cmux.sock` (`cmux select-workspace` …); cmux-tui is a separate server (`cmux server start --session N`) | `cmux attach --session N` (cmux-tui) | AppleScript (`new window`, `select tab`) or `cmux select-workspace`; no session link | AppleScript window name is read-only; workspaces can be renamed |
| **TUIOS** (0.7.0 from nixpkgs) | A daemon (`tuios new` starts it), socket `$XDG_RUNTIME_DIR/tuios/tuios.sock` | `tuios attach <name>` | None (a separate `tuios-web` view in the browser) | Leaves the title alone by default |

## Terminal apps

- **Ghostty 1.4.0:** `open -na Ghostty.app --args --title="X" -e <cmd>`. `--title` forces the title, whatever the multiplexer sets. AppleScript can also open a window with a command and set its tab's name.
- **macOS has no default-terminal setting.** The nearest thing is the app that opens `.command` files: Terminal.app on deneb. Terminal.app can be told by AppleScript to run a command and to set a custom title. A `.command` file can't carry a title.

## What follows for Flume

- **A backend with a terminal client** (herdr 0.9.5, cmux-tui, TUIOS): to show a session, open a terminal window that runs the attach command, titled for the stream.
- **A backend shown only by its app** (Rex; herdr-core through the Herdr app): to show a session, tell the running app to show it. The app owns the window.
- **cmux.app holds its own terminals**, driven by AppleScript or `cmux select-workspace`.
- **The Herdr app is not a second view of today's herdr sessions.** It runs on herdr-core, so switching to it means switching backends.
- **Not confirmed:**
  - the Herdr app and `hc attach` on one session at once
  - cmux.app showing a local cmux-tui session
  - session links for Rex and the Herdr app
  - the default window titles of Rex and cmux
