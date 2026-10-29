#!/bin/sh
# The plugin's tests (tests/run.sh) under the system's sh and under dash,
# the /bin/sh of Debian and Ubuntu, which herdr runs the plugin's commands
# with there; dash is the one devenv.nix pins (core.22).
set -u
here=$(cd "$(dirname "$0")" && pwd -P)
bad=0
echo "== under the system's sh"
sh "$here/run.sh" || bad=1
echo "== under dash"
if command -v dash >/dev/null 2>&1; then
  FLUME_HERDR_SH=dash sh "$here/run.sh" || bad=1
else
  echo "dash is not on PATH: run the tests in the devenv shell, which pins it"
  bad=1
fi
exit "$bad"
