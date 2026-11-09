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
    rev = "52b95528a495100e34fca39109d563d8bf75673a";
    hash = "sha256-29h7paqrZ2AONJ/Xs2KBTF+9qYJWJMoJBZDE+OIxqoc=";
  };
  "weir-delivery-0.0.0" = {
    rev = "52b95528a495100e34fca39109d563d8bf75673a";
    hash = "sha256-29h7paqrZ2AONJ/Xs2KBTF+9qYJWJMoJBZDE+OIxqoc=";
  };
  "weir-kinds-0.0.0" = {
    rev = "52b95528a495100e34fca39109d563d8bf75673a";
    hash = "sha256-29h7paqrZ2AONJ/Xs2KBTF+9qYJWJMoJBZDE+OIxqoc=";
  };
  "weir-pack-0.0.0" = {
    rev = "52b95528a495100e34fca39109d563d8bf75673a";
    hash = "sha256-29h7paqrZ2AONJ/Xs2KBTF+9qYJWJMoJBZDE+OIxqoc=";
  };
}
