#!/bin/sh
# Every task of every project carries exactly one layer tag, check-fast,
# check-tools or check-live, or none with a comment above it saying why the
# gate cannot run it (core.21; .moon/workspace.yml names the layers). The
# projects are those .moon/workspace.yml lists, each read from its moon.yml.
set -eu

status=0
count=0
projects=$(awk '
  /^projects:/ { on = 1; next }
  on && /^[^ #]/ { on = 0 }
  on && /^  [A-Za-z0-9_-]+:/ {
    path = $2
    gsub(/["\047]/, "", path)
    print path
  }
' .moon/workspace.yml)
[ -n "$projects" ] || {
  echo "check-layers: .moon/workspace.yml lists no project" >&2
  exit 1
}

for project in $projects; do
  file="$project/moon.yml"
  [ -f "$file" ] || {
    echo "check-layers: $file is missing" >&2
    status=1
    continue
  }
  # One line per task: its name, the number of layer tags it carries, and
  # whether a comment stands directly above it.
  tasks=$(awk '
    function done() {
      if (name != "") print name, layers, commented
      name = ""
    }
    /^tasks:/ { on = 1; next }
    on && /^[^ #]/ { done(); on = 0 }
    !on { next }
    /^  #/ { comment = 1; next }
    /^  [A-Za-z0-9_-]+:/ {
      done()
      name = $1
      sub(/:$/, "", name)
      layers = 0
      commented = comment
      comment = 0
      next
    }
    /^ *$/ { comment = 0; next }
    /^    tags:/ { intags = 1 }
    /^    [a-z]/ && !/^    tags:/ { intags = 0 }
    intags {
      line = $0
      n = gsub(/check-(fast|tools|live)/, "", line)
      layers += n
    }
    END { done() }
  ' "$file")
  while read -r name layers commented; do
    [ -n "$name" ] || continue
    count=$((count + 1))
    if [ "$layers" -gt 1 ]; then
      echo "check-layers: $file: $name carries $layers layer tags" >&2
      status=1
    elif [ "$layers" -eq 0 ] && [ "$commented" -eq 0 ]; then
      echo "check-layers: $file: $name carries no layer tag and no comment saying why the gate cannot run it" >&2
      status=1
    fi
  done <<EOF
$tasks
EOF
done

[ "$status" -ne 0 ] || echo "check-layers: $count tasks, each in one layer or saying why it is in none"
exit "$status"
