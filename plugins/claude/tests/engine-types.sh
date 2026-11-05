#!/usr/bin/env bash
# The engine types the plugin's module and its tests are typed against
# (engine-types/, tsconfig.json) are the ones the pinned Claude Code
# writes. A session of the pinned release, started on a terminal of its
# own over a scratch mod, with a scratch home, a key that is no one's and
# no traffic, writes its declarations beside the mod as it loads it; no
# prompt is sent, and the session is ended once they are written. Each
# vendored file must be the one it wrote, byte for byte, and a copy of
# them with one line changed must fail the same comparison (C.4; core.22,
# core.39). Only the engine's own module, claude-code, is vendored: the
# tools' and the MCP servers' declarations it writes beside it are the
# session's own tools and servers, not the release's.
#
# To move the pin: change the version and the checksums in devenv.nix,
# start a session of that release with `claude --plugin-dir
# plugins/claude`, and copy its .claude-plugin/types/claude-code over
# engine-types/claude-code.

set -u

plugin=$(cd "$(dirname "$0")/.." && pwd)
vendored=${1:-$plugin/engine-types}

# Under sluice's sandbox only SLUICE_TMP is writable beside the checkout.
scratch=$(mktemp -d "${SLUICE_TMP:-${TMPDIR:-/tmp}}/flume-engine-types.XXXXXX")
scratch=$(cd "$scratch" && pwd -P)
session=
end() {
  [ -n "$session" ] && kill "$session" 2>/dev/null
  rm -rf "$scratch"
}
trap end EXIT

for name in $(env | grep -Eo '^(CLAUDE|ANTHROPIC_)[A-Z_]*=' | tr -d =); do
  unset "$name"
done
export HOME=$scratch/home CLAUDE_CONFIG_DIR=$scratch/home/.claude
export CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1 TERM=xterm-256color
# A key that is no one's and an address nothing answers on, so the session
# reaches no account and sends nothing.
key=sk-ant-flume-engine-types-0000000000
export ANTHROPIC_API_KEY=$key ANTHROPIC_BASE_URL=http://127.0.0.1:9

release=$(claude --version | sed -n '1s/^\([^ ]*\).*/\1/p')
[ -n "$release" ] || {
  echo "plugin-claude: the pinned claude gives no version" >&2
  exit 1
}

# A mod of one module, whose folder the session is told it may trust.
mod=$scratch/mod
mkdir -p "$mod/.claude-plugin" "$mod/hooks" "$CLAUDE_CONFIG_DIR"
printf '{"name": "engine-types", "version": "0.0.0"}\n' >"$mod/.claude-plugin/plugin.json"
printf '{"modules": ["./probe.ts"]}\n' >"$mod/hooks/hooks.json"
printf 'export const register = () => {};\n' >"$mod/hooks/probe.ts"
jq -n --arg mod "$mod" --arg key "${key: -20}" '{
  hasCompletedOnboarding: true,
  customApiKeyResponses: {approved: [$key], rejected: []},
  projects: {($mod): {hasTrustDialogAccepted: true}}
}' >"$CLAUDE_CONFIG_DIR/.claude.json"

# The session on a terminal of its own: util-linux's script, or the BSD
# one macOS ships.
cd "$mod" || exit 1
if script --version >/dev/null 2>&1; then
  script -q -e -c "claude --plugin-dir $(printf %q "$mod")" /dev/null </dev/null >"$scratch/screen" 2>&1 &
else
  script -q /dev/null claude --plugin-dir "$mod" </dev/null >"$scratch/screen" 2>&1 &
fi
session=$!

written=$mod/.claude-plugin/types/claude-code/index.d.ts
size=
for _ in $(seq 1 240); do
  if [ -f "$written" ]; then
    now=$(wc -c <"$written")
    [ "$now" = "$size" ] && break
    size=$now
  fi
  kill -0 "$session" 2>/dev/null || break
  sleep 0.25
done
kill "$session" 2>/dev/null
wait "$session" 2>/dev/null
session=
if [ ! -f "$written" ]; then
  echo "plugin-claude: Claude Code $release wrote no engine types for a mod; its screen:" >&2
  tail -c 2000 "$scratch/screen" | tr -d '\033' >&2
  exit 1
fi

# The first vendored file under $1 that is not the one the release wrote,
# in words; nothing where each is.
difference() {
  local file
  while IFS= read -r file; do
    if ! cmp -s "$1/$file" "$mod/.claude-plugin/types/$file"; then
      echo "plugin-claude: engine-types/$file is not the one Claude Code $release writes: copy it again (tests/engine-types.sh)"
      return
    fi
  done < <(cd "$1" && find . -type f -name '*.d.ts' | sed 's|^\./||' | sort)
}

[ -f "$vendored/claude-code/index.d.ts" ] || {
  echo "plugin-claude: no engine types at $vendored/claude-code/index.d.ts" >&2
  exit 1
}
said=$(difference "$vendored")
[ -z "$said" ] || {
  echo "$said" >&2
  exit 1
}

# The comparison fails a copy of them with one line changed.
copy=$scratch/changed
cp -R "$vendored" "$copy"
sed '2s/^/ /' "$copy/claude-code/index.d.ts" >"$scratch/changed.d.ts"
mv "$scratch/changed.d.ts" "$copy/claude-code/index.d.ts"
[ -n "$(difference "$copy")" ] || {
  echo "plugin-claude: a copy of the engine types with one line changed read as Claude Code $release's" >&2
  exit 1
}

echo "the engine types are the ones Claude Code $release writes"
