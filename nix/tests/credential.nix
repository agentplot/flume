# The host-module spec's one table of cases for both implementations of
# flume's shapes of a credential (the change
# flume-publishes-its-own-flake-and-module, design 8): every case of
# crates/flume-core/tests/credential-shapes.json, which flume-core's own
# tests read too, through the module's port, each shaped or passing as the
# table says. The port's prefixes and secret names are own.rs's, read from
# its PREFIXES and SECRET_NAMES.
t:
let
  inherit (t) lib;
  credential = import ../modules/credential.nix { inherit lib; };
  cases = builtins.fromJSON (builtins.readFile ../../crates/flume-core/tests/credential-shapes.json);

  readAs = shaped: if shaped then "shaped as a credential" else "passing";
  holds =
    case:
    let
      read = credential.shaped case.name case.text;
    in
    t.expect (read == case.shaped)
      "the port reads ${case.test}'s ${case.name} = ${builtins.toJSON case.text} as ${readAs read}, where the table says ${readAs case.shaped}";

  # The strings of own.rs's array `name`, one to a line.
  lines = lib.splitString "\n" (builtins.readFile ../../crates/flume-core/src/own.rs);
  arrayOf =
    name:
    let
      from = lib.lists.findFirstIndex (lib.hasPrefix "const ${name}:") null lines;
      rest = if from == null then [ ] else lib.drop (from + 1) lines;
      end = lib.lists.findFirstIndex (line: line == "];") (builtins.length rest) rest;
    in
    map (line: builtins.fromJSON (lib.removeSuffix "," (lib.trim line))) (lib.take end rest);
  sameAs =
    name: given:
    t.expect (
      arrayOf name == given
    ) "the port's ${name} are ${builtins.toJSON given}, not own.rs's ${builtins.toJSON (arrayOf name)}";
in
{
  "One table of cases for both" =
    t.expect (cases != [ ]) "credential-shapes.json holds no case"
    ++ lib.concatMap holds cases
    ++ sameAs "PREFIXES" credential.prefixes
    ++ sameAs "SECRET_NAMES" credential.secretNames;
}
