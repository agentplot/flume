#!/usr/bin/env bash
# The marketplace validates by the pinned Claude Code, and every plugin it
# lists has a manifest. Each plugin's own manifest is validated by its
# project's `validate` task (core.21).

set -u
root=$(cd "$(dirname "$0")" && pwd)
cd "$root" || exit 1

status=0
"$root/plugins/claude.sh" plugin validate --strict . || {
  echo "marketplace: the marketplace failed claude plugin validate --strict" >&2
  status=1
}
while IFS=$'\t' read -r name source; do
  if [ ! -f "$source/.claude-plugin/plugin.json" ]; then
    echo "marketplace: $name: $source has no .claude-plugin/plugin.json" >&2
    status=1
  fi
done < <(jq -r '.plugins[] | [.name, .source] | @tsv' .claude-plugin/marketplace.json)

exit $status
