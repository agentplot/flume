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
set -eu

usage() {
  echo "usage: activate.sh link <file>" >&2
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

case ${1:-} in
link)
  [ $# -eq 2 ] || usage
  link "$2"
  ;;
*) usage ;;
esac
