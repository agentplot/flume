# The host-module spec's file (the change
# flume-publishes-its-own-flake-and-module, design 7), over the renderer
# alone: each settings renders the text of its file under nix/tests/files,
# byte for byte, and a value the file cannot hold fails, naming its path
# and the values the file takes.
t:
let
  inherit (t) lib;
  toml = import ../modules/toml.nix { inherit lib; };
  every = import ./every-key.nix;

  rendersAs =
    name: settings:
    let
      text = toml.text settings;
    in
    t.expect (
      text == builtins.readFile ./files/${name}.toml
    ) "${name}'s settings render ${builtins.toJSON text}, not nix/tests/files/${name}.toml";

  # every-key's settings with a path where a program's file is named.
  path = lib.recursiveUpdate every { kinds.change.agents.programs.scribe.file = ./every-key.nix; };
  pathAt = "services.flume.settings.kinds.change.agents.programs.scribe.file";
  refused = toml.refusals path;
in
{
  "Every key renders its file" = rendersAs "every-key" every;

  "No settings" =
    rendersAs "empty" { }
    ++ t.expect (
      lib.splitString "\n" (toml.text { }) == [
        "# Written by flume's module from services.flume.settings; a change goes there."
        ""
      ]
    ) "the empty settings render more than the first line";

  "A path value" =
    t.expect (
      !(builtins.tryEval (toml.text path)).success
    ) "settings holding a path at ${pathAt} render"
    ++ t.expect (
      builtins.length refused == 1
      && builtins.all (words: lib.hasInfix words (builtins.head refused)) [
        pathAt
        "a path"
        "strings, booleans, integers, floats, lists and tables"
      ]
    ) "settings holding a path at ${pathAt} are refused as ${builtins.toJSON refused}";
}
