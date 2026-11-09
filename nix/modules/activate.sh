#!/bin/sh
# The modules' activation, run as the user whose file it is (the change
# flume-publishes-its-own-flake-and-module, design 7, 9 and 10, as
# Boomdrive's nix/modules/activate.sh link):
#
#   activate.sh link <file>
#
# links ~/.config/flume/config.toml, flume's managed file, to the module's
# file, making ~/.config/flume/ where it is missing. A link there is
# replaced. A file or folder there that is no link is moved once, to
# config.toml.before-nix, and both paths are named. Where
# config.toml.before-nix is there already, it changes neither, names both
# and fails: flume reads the file left in place. It writes nothing else of
# flume's, not config.local.toml, sources/ or flume.lock, and nothing of
# another tool's (C.7; core.45). It says a line only where it moved or
# linked something.
#
#   activate.sh sources [<source> <file>]...
#
# links ~/.config/flume/sources/<source>.toml to its file for each source
# the module declares, and removes the module's own link of each source it
# declares no more (the change flumes-module-writes-the-hosts-sources,
# design 5). Each link is made beside its file, under
# .<source>.toml.<pid>, and renamed over it, so a reader finds the old file
# or the new one and never no source (191). A link there is replaced. A
# file or folder there that is no link is moved once, to
# <source>.toml.before-nix, and both paths are named; where that is there
# already, it changes neither, names both, links the other sources and
# fails. A link removed is one the module made: <name>.toml leading to a
# file whose name ends in -flume-source-<name>.toml, the store name the
# module gives a source's file. It writes and removes nothing else: not a
# person's own source or another program's, a file moved aside, a
# temporary name, a source's roles file or its packages, nor anything
# outside sources/ (191, C.7; core.45). It makes sources/ where it is
# missing and a source is declared, and never removes it. Where sources is
# there and is no folder, it links and removes nothing in it, and fails
# naming it where a source is declared. It says a line only where it
# moved, linked or removed something, or failed.
set -eu

usage() {
  echo "usage: activate.sh link <file>" >&2
  echo "       activate.sh sources [<source> <file>]..." >&2
  exit 2
}

link() {
  file=$1
  folder=$HOME/.config/flume
  target=$folder/config.toml
  aside=$target.before-nix

  if [ -L "$target" ] && [ "$(readlink "$target")" = "$file" ]; then
    return
  fi
  if [ -e "$target" ] && [ ! -L "$target" ]; then
    if [ -e "$aside" ] || [ -L "$aside" ]; then
      echo "flume: $target is no link and $aside is there already: left both, linked nothing; move one aside to link $file" >&2
      exit 1
    fi
    mv "$target" "$aside"
    echo "flume: moved $target to $aside"
  fi
  mkdir -p "$folder"
  ln -sfn "$file" "$target"
  echo "flume: linked $target to $file"
}

# Whether `$1` is among the sources `$2 $3 ...` names, given as source and
# file in turn.
declared() {
  name=$1
  shift
  while [ $# -gt 0 ]; do
    [ "$1" = "$name" ] && return 0
    shift 2
  done
  return 1
}

sources() {
  dir=$HOME/.config/flume/sources

  if [ -L "$dir" ] || { [ -e "$dir" ] && [ ! -d "$dir" ]; }; then
    if [ $# -gt 0 ]; then
      echo "flume: $dir is no folder: linked and removed nothing in it; move it aside to link the sources declared" >&2
      exit 1
    fi
    return
  fi
  if [ $# -gt 0 ]; then
    mkdir -p "$dir"
  elif [ ! -d "$dir" ]; then
    return
  fi

  # Each source and its file in turn, put back after the rest, so "$@" is
  # as given once every one is linked.
  failed=0
  rest=$#
  while [ "$rest" -gt 0 ]; do
    source=$1
    file=$2
    shift 2
    set -- "$@" "$source" "$file"
    rest=$((rest - 2))

    target=$dir/$source.toml
    aside=$target.before-nix
    if [ -L "$target" ] && [ "$(readlink "$target")" = "$file" ]; then
      continue
    fi
    if [ -e "$target" ] && [ ! -L "$target" ]; then
      if [ -e "$aside" ] || [ -L "$aside" ]; then
        echo "flume: $target is no link and $aside is there already: left both, linked nothing; move one aside to link $file" >&2
        failed=1
        continue
      fi
      mv "$target" "$aside"
      echo "flume: moved $target to $aside"
    fi
    # A rename over a link to a folder would move the new link into it.
    if [ -L "$target" ] && [ -d "$target" ]; then
      rm "$target"
    fi
    temporary=$dir/.$source.toml.$$
    rm -f "$temporary"
    ln -s "$file" "$temporary"
    mv -f "$temporary" "$target"
    echo "flume: linked $target to $file"
  done

  for entry in "$dir"/*.toml; do
    [ -L "$entry" ] || continue
    name=${entry##*/}
    name=${name%.toml}
    if declared "$name" "$@"; then
      continue
    fi
    leads=$(readlink "$entry")
    case $leads in
    *"-flume-source-$name.toml")
      rm "$entry"
      echo "flume: removed $entry, the module's link to $leads, since no source $name is declared"
      ;;
    esac
  done

  [ "$failed" -eq 0 ] || exit 1
}

case ${1:-} in
link)
  [ $# -eq 2 ] || usage
  link "$2"
  ;;
sources)
  shift
  [ $(($# % 2)) -eq 0 ] || usage
  sources "$@"
  ;;
*) usage ;;
esac
