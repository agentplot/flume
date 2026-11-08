# flume's git crates: each git package of Cargo.lock, by
# `<name>-<version>`, to the revision its hash was taken at and the hash of
# fetchgit's tree there, which the package fetches it by (the change
# flume-publishes-its-own-flake-and-module, design 2). A revision moves with
# its hash, in one commit: nix:test fails where an entry's revision is not
# Cargo.lock's, or a git package has no entry. A hash is taken by building
# `fetchgit { url; rev; sha256 = lib.fakeHash; }` and reading the hash nix
# got.
{
  "weir-capture-0.0.0" = {
    rev = "5562af52cb7e065e7de7109135dfd7eecb1d61c5";
    hash = "sha256-2DFTZKoYQiYKggsaZTqWCWtcJpux7x9prTEaomLXk9k=";
  };
  "weir-kinds-0.0.0" = {
    rev = "5562af52cb7e065e7de7109135dfd7eecb1d61c5";
    hash = "sha256-2DFTZKoYQiYKggsaZTqWCWtcJpux7x9prTEaomLXk9k=";
  };
}
