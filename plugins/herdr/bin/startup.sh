#!/bin/sh
# shellcheck disable=SC1091,SC2154 # common.sh, sourced, sets root and state.
# herdr's startup of a server, run once and gone (122, C.5; core.13):
# what herdr's configuration lacks for the popup's key and the sidebar's
# rows written into this server's setup.json with the server's socket,
# which flume's herdr profile gives in status while that socket is
# present, and logged, telling no person (56, 72); then a watch an
# earlier release of the plugin left on this server ended, by the pid
# and identity its lease names and never another process holding that
# pid, and its lease removed (record 0016). The plugin keeps no process:
# it runs no settle and reads no status, the popup reading status only
# when a person opens it.
set -u
. "$(dirname "$0")/../lib/common.sh"
. "$root/lib/procs.sh"

log=$state/startup.log

# setup.json: {"socket": <this server's socket>, "lacking": [<words>]}.
lacking "$(herdr_config)" >"$state/setup.lines"
jq -R -s -c --arg socket "${HERDR_SOCKET_PATH:-}" \
  '{socket: $socket, lacking: (split("\n") | map(select(. != "")))}' <"$state/setup.lines" |
  put "$state/setup.json" || say "what herdr's configuration lacks was not written" 2>>"$log"
while IFS= read -r words; do say "$words"; done <"$state/setup.lines" 2>>"$log"
rm -f "$state/setup.lines"

# An earlier release's watch: its lease's first line its pid and when it
# last renewed, its second the pid's identity. Asked to end, it removes
# its own lease; one that has not ended within its tick's bound is
# killed.
lease=$state/watch.lease
if [ -f "$lease" ]; then
  read -r pid _ <"$lease" 2>/dev/null || pid=
  was=$(sed -n 2p "$lease")
  if same_process "$pid" "$was"; then
    kill "$pid" 2>/dev/null
    n=0
    while same_process "$pid" "$was" && [ "$n" -lt 30 ]; do
      sleep 1
      n=$((n + 1))
    done
    same_process "$pid" "$was" && kill -9 "$pid" 2>/dev/null
    say "ended the watch an earlier release left (pid $pid)" 2>>"$log"
  fi
  rm -f "$lease" "$state/watch.lease.tmp."* "$state/status.json" "$state/seen" "$state/reports" "$state/reported"
  rm -rf "$state/lease.lock"
fi
exit 0
