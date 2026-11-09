#!/bin/sh
# The scripts' cases (the change flume-publishes-its-own-flake-and-module,
# design 11), which nix/check.sh test runs beside the evaluated ones: each
# runs nix/check.sh or nix/modules/activate.sh by sh in a scratch folder of
# its own. It prints each failure as one line, nix: <case>: <what>, and
# nothing where every case holds.
set -eu

nix=$(cd "$(dirname "$0")/.." && pwd)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
cases=0

# A case's folder.
fresh() {
  cases=$((cases + 1))
  dir=$scratch/$cases
  mkdir -p "$dir"
}

# A failure of the case `$1`, saying `$2`, where the test after them fails.
expect() {
  named=$1
  what=$2
  shift 2
  if ! "$@"; then
    echo "nix: $named: $what"
  fi
}

# `$1` holds `$2`.
says() {
  case $1 in *"$2"*) ;; *) return 1 ;; esac
}

named="No inputs given"
fresh
status=0
(cd "$nix/.." && env -u FLUME_FLAKE_INPUTS sh nix/check.sh test) \
  >"$dir/out" 2>"$dir/err" || status=$?
err=$(cat "$dir/err")
expect "$named" "check.sh test with FLUME_FLAKE_INPUTS unset exits $status" \
  [ "$status" -ne 0 ]
expect "$named" "check.sh test with FLUME_FLAKE_INPUTS unset says \"$err\", not that the devenv shell sets it" \
  says "$err" "FLUME_FLAKE_INPUTS is not set: the devenv shell sets it"

# `nix eval` of nix/tests/fails.nix's `$1` as nix/check.sh runs it: its
# status and its error.
fails() {
  status=0
  (cd "$nix/.." && nix eval --raw --offline --read-only --impure \
    --store 'dummy://?store=/flume-check/store' \
    --extra-experimental-features nix-command \
    --file nix/tests/fails.nix "$1") >"$dir/out" 2>"$dir/err" || status=$?
  err=$(cat "$dir/err")
}

named="A mistyped declared key"
fresh
fails mistyped
expect "$named" 'settings.streams.work.agents = "five" evaluates' [ "$status" -ne 0 ]
expect "$named" "its failure names no services.flume.settings.streams.work.agents: $err" \
  says "$err" "services.flume.settings.streams.work.agents"

named="A path value"
fresh
fails path
expect "$named" "settings holding a path evaluate" [ "$status" -ne 0 ]
expect "$named" "its failure names no services.flume.settings.kinds.change.agents.programs.scribe.file: $err" \
  says "$err" "services.flume.settings.kinds.change.agents.programs.scribe.file"
expect "$named" "its failure names not the values the file takes: $err" \
  says "$err" "strings, booleans, integers, floats, lists and tables"

named="No kinds_dir"
fresh
fails noKindsDir
expect "$named" 'a source acme giving roles alone evaluates' [ "$status" -ne 0 ]
expect "$named" "its failure names no services.flume.sources.acme.kinds_dir: $err" \
  says "$err" "services.flume.sources.acme.kinds_dir"

named="A kinds_dir under no home"
fresh
fails kindsDirUnderNoHome
expect "$named" 'a source acme whose kinds_dir is kinds/acme evaluates' [ "$status" -ne 0 ]
expect "$named" "its failure names no services.flume.sources.acme.kinds_dir: $err" \
  says "$err" "services.flume.sources.acme.kinds_dir"

named="A path value in a source"
fresh
fails sourcePath
expect "$named" "a source holding a path evaluates" [ "$status" -ne 0 ]
expect "$named" "its failure names no services.flume.sources.acme.shelf: $err" \
  says "$err" "services.flume.sources.acme.shelf"
expect "$named" "its failure names not the values the file takes: $err" \
  says "$err" "strings, booleans, integers, floats, lists and tables"

# The activation's link, in a case's folder: its home, the module's file
# in a store of its own, and where the link goes.
activate=$nix/modules/activate.sh
home_of() {
  home=$dir/home
  file=$dir/store/flume-config.toml
  folder=$home/.config/flume
  target=$folder/config.toml
  aside=$target.before-nix
  mkdir -p "$home" "$dir/store"
  echo '# the module' >"$file"
}

# activate.sh with `$@` in the case's home: its status, output and error.
activation() {
  status=0
  HOME=$home sh "$activate" "$@" >"$dir/out" 2>"$dir/err" || status=$?
  out=$(cat "$dir/out")
  err=$(cat "$dir/err")
}

linked() {
  [ -L "$target" ] && [ "$(readlink "$target")" = "$file" ]
}
holds() {
  [ "$(cat "$1" 2>/dev/null)" = "$2" ]
}
missing() {
  [ ! -e "$1" ] && [ ! -L "$1" ]
}
# `$1` names `$2`, then `$3`.
names() {
  case $1 in *"$2"*"$3"*) ;; *) return 1 ;; esac
}
# The names of the folder `$1`'s entries, sorted, on one line.
listed() {
  (cd "$1" && find . -mindepth 1 -maxdepth 1 | sed 's|^\./||' | sort | paste -s -d ' ' -)
}
# The folder `$1` holds exactly `$2`.
entries() {
  [ "$(listed "$1")" = "$2" ]
}

named="A fresh home"
fresh
home_of
activation link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "link says \"$out\", not that it linked $target" \
  [ "$out" = "flume: linked $target to $file" ]
expect "$named" "$folder holds $(listed "$folder"), not config.toml alone" \
  entries "$folder" config.toml

named="A file aside already"
fresh
home_of
mkdir -p "$folder"
echo '# by hand' >"$target"
echo '# by hand before' >"$aside"
activation link "$file"
expect "$named" "link exits $status, not 1" [ "$status" -eq 1 ]
expect "$named" "link changed $target" holds "$target" '# by hand'
expect "$named" "link changed $aside" holds "$aside" '# by hand before'
expect "$named" "link's refusal \"$err\" names not both paths" \
  names "$err" "$target" "$aside"
expect "$named" "link says \"$out\"" [ -z "$out" ]

named="A link of an earlier activation"
fresh
home_of
mkdir -p "$folder"
ln -s "$dir/store/earlier-config.toml" "$target"
activation link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "link says \"$out\", not that it linked $target" \
  [ "$out" = "flume: linked $target to $file" ]
expect "$named" "link moved the earlier link aside" missing "$aside"

named="A hand-written file there"
fresh
home_of
mkdir -p "$folder/sources"
echo '# by hand' >"$target"
echo '[streams.work]' >"$folder/config.local.toml"
echo 'kinds_dir = "/srv/kinds"' >"$folder/sources/acme.toml"
echo '# the host lock' >"$folder/flume.lock"
before=$(cd "$folder" && cat config.local.toml sources/acme.toml flume.lock | cksum)
activation link "$file"
after=$(cd "$folder" && cat config.local.toml sources/acme.toml flume.lock | cksum)
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "$aside does not hold the hand-written file" holds "$aside" '# by hand'
expect "$named" "link says \"$out\", not both paths" \
  [ "$out" = "flume: moved $target to $aside
flume: linked $target to $file" ]
expect "$named" "config.local.toml, sources/ or flume.lock changed" [ "$before" = "$after" ]
expect "$named" "$folder holds $(listed "$folder")" \
  entries "$folder" "config.local.toml config.toml config.toml.before-nix flume.lock sources"
expect "$named" "sources/ holds $(listed "$folder/sources")" \
  entries "$folder/sources" acme.toml

fresh
home_of
mkdir -p "$target"
echo 'kept' >"$target/inside"
activation link "$file"
expect "$named" "link of a folder exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file, where a folder was" linked
expect "$named" "$aside does not hold the folder" holds "$aside/inside" kept

named="An unknown system"
fresh
fails unknownSystem
expect "$named" "services.flume.package on riscv64-linux evaluates" [ "$status" -ne 0 ]
expect "$named" "its failure names not the systems the flake gives: $err" \
  says "$err" "gives no package for riscv64-linux: it gives aarch64-darwin, aarch64-linux, x86_64-linux"

# The activation's sources, in a case's folder: its home, where the
# sources go, and each module file in a store of the case's own,
# `<n>-flume-source-<source>.toml` (the change
# flumes-module-writes-the-hosts-sources, design 5).
sources_home() {
  home=$dir/home
  folder=$home/.config/flume
  sources=$folder/sources
  mkdir -p "$home" "$dir/store"
}
# The module's file `$1`-flume-source-`$2`.toml, made in the case's store.
store_file() {
  made=$dir/store/$1-flume-source-$2.toml
  echo "# the module's $2" >"$made"
  echo "$made"
}
# `$1` is a link to `$2`.
links() {
  [ -L "$1" ] && [ "$(readlink "$1")" = "$2" ]
}
# The folder `$1` holds no name beginning with `.`.
no_dot() {
  [ -z "$(cd "$1" && find . -mindepth 1 -maxdepth 1 -name '.*')" ]
}

named="Sources in a fresh home"
fresh
sources_home
acme=$(store_file 1 acme)
tools=$(store_file 1 tools)
activation sources acme "$acme" tools "$tools"
expect "$named" "sources exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$sources/acme.toml is no link to $acme" links "$sources/acme.toml" "$acme"
expect "$named" "$sources/tools.toml is no link to $tools" links "$sources/tools.toml" "$tools"
expect "$named" "$sources holds $(listed "$sources"), not acme.toml and tools.toml alone" \
  entries "$sources" "acme.toml tools.toml"
expect "$named" "sources says \"$out\", not that it linked each" \
  [ "$out" = "flume: linked $sources/acme.toml to $acme
flume: linked $sources/tools.toml to $tools" ]

named="A source's link of an earlier activation"
fresh
sources_home
earlier=$(store_file 1 acme)
acme=$(store_file 2 acme)
mkdir -p "$sources"
ln -s "$earlier" "$sources/acme.toml"
activation sources acme "$acme"
expect "$named" "sources exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$sources/acme.toml is no link to $acme" links "$sources/acme.toml" "$acme"
expect "$named" "sources moved the earlier link aside" missing "$sources/acme.toml.before-nix"
expect "$named" "$sources holds a name beginning with .: $(listed "$sources")" no_dot "$sources"
expect "$named" "sources says \"$out\", not that it linked acme" \
  [ "$out" = "flume: linked $sources/acme.toml to $acme" ]

named="A hand-written file of a declared source"
fresh
sources_home
acme=$(store_file 1 acme)
mkdir -p "$sources"
echo 'kinds_dir = "/srv/kinds/acme"' >"$sources/acme.toml"
activation sources acme "$acme"
expect "$named" "sources exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$sources/acme.toml is no link to $acme" links "$sources/acme.toml" "$acme"
expect "$named" "$sources/acme.toml.before-nix does not hold the hand-written file" \
  holds "$sources/acme.toml.before-nix" 'kinds_dir = "/srv/kinds/acme"'
expect "$named" "sources says \"$out\", not both paths" \
  [ "$out" = "flume: moved $sources/acme.toml to $sources/acme.toml.before-nix
flume: linked $sources/acme.toml to $acme" ]

named="A source's file aside already"
fresh
sources_home
acme=$(store_file 1 acme)
tools=$(store_file 1 tools)
mkdir -p "$sources"
echo '# by hand' >"$sources/acme.toml"
echo '# by hand before' >"$sources/acme.toml.before-nix"
activation sources acme "$acme" tools "$tools"
expect "$named" "sources exits $status, not non-zero" [ "$status" -ne 0 ]
expect "$named" "sources changed $sources/acme.toml" holds "$sources/acme.toml" '# by hand'
expect "$named" "sources changed $sources/acme.toml.before-nix" \
  holds "$sources/acme.toml.before-nix" '# by hand before'
expect "$named" "sources' refusal \"$err\" names not both paths" \
  names "$err" "$sources/acme.toml" "$sources/acme.toml.before-nix"
expect "$named" "$sources/tools.toml is no link to $tools" links "$sources/tools.toml" "$tools"

named="A sources that is no folder"
fresh
sources_home
acme=$(store_file 1 acme)
mkdir -p "$folder"
echo '# not a folder' >"$sources"
activation sources acme "$acme"
expect "$named" "sources acme exits $status, not non-zero" [ "$status" -ne 0 ]
expect "$named" "sources acme's refusal \"$err\" names not $sources" says "$err" "$sources"
activation sources
expect "$named" "sources with no source exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "sources with no source says \"$out$err\"" [ -z "$out$err" ]
expect "$named" "$sources is not as it was" holds "$sources" '# not a folder'

named="No source in a fresh home"
fresh
sources_home
activation sources
expect "$named" "sources exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "sources says \"$out$err\"" [ -z "$out$err" ]
expect "$named" "sources made $folder" missing "$folder"

named="A source no longer declared"
fresh
sources_home
gone=$(store_file 1 acme)
tools=$(store_file 2 tools)
mkdir -p "$sources" "$dir/theirs"
ln -s "$gone" "$sources/acme.toml"
echo 'kinds_dir = "/srv/kinds/mine"' >"$sources/mine.toml"
echo 'kinds_dir = "/srv/kinds/theirs"' >"$dir/theirs/theirs.toml"
ln -s "$dir/theirs/theirs.toml" "$sources/theirs.toml"
echo '# moved aside' >"$sources/old.toml.before-nix"
activation sources tools "$tools"
expect "$named" "sources exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "sources left $sources/acme.toml" missing "$sources/acme.toml"
expect "$named" "sources says \"$out\", not that it removed acme.toml" \
  says "$out" "removed $sources/acme.toml"
expect "$named" "$sources/tools.toml is no link to $tools" links "$sources/tools.toml" "$tools"
expect "$named" "sources changed mine.toml" holds "$sources/mine.toml" 'kinds_dir = "/srv/kinds/mine"'
expect "$named" "sources changed theirs.toml" links "$sources/theirs.toml" "$dir/theirs/theirs.toml"
expect "$named" "sources changed old.toml.before-nix" holds "$sources/old.toml.before-nix" '# moved aside'
expect "$named" "$sources holds $(listed "$sources")" \
  entries "$sources" "mine.toml old.toml.before-nix theirs.toml tools.toml"

named="The sources beside the rest of the folder"
fresh
sources_home
acme=$(store_file 1 acme)
mkdir -p "$folder"
echo '# by hand' >"$folder/config.toml"
echo '[streams.work]' >"$folder/config.local.toml"
echo '# the host lock' >"$folder/flume.lock"
before=$(cd "$folder" && cat config.toml config.local.toml flume.lock | cksum)
activation sources acme "$acme"
after=$(cd "$folder" && cat config.toml config.local.toml flume.lock | cksum)
expect "$named" "sources exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$sources/acme.toml is no link to $acme" links "$sources/acme.toml" "$acme"
expect "$named" "config.toml, config.local.toml or flume.lock changed" [ "$before" = "$after" ]
expect "$named" "$folder holds $(listed "$folder")" \
  entries "$folder" "config.local.toml config.toml flume.lock sources"
