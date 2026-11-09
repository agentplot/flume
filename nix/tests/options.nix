# The host-module spec's settings (the change
# flume-publishes-its-own-flake-and-module, design 5, 6 and 8), over
# options.nix evaluated alone beside a system's `assertions`: every key
# renders its file, only the keys flume reads are declared, an undeclared
# key passes as written, two modules' tables merge into one file, and a
# string shaped as a credential fails, naming its path and never its value.
# A source of `sources` passes an undeclared key to its file as written,
# and a token in it fails the same way (the change
# flumes-module-writes-the-hosts-sources, design 1 and 4).
# A mistyped key's failure is read by its words in nix/tests/scripts.sh.
t:
let
  inherit (t) lib;
  toml = import ../modules/toml.nix { inherit lib; };
  pkgs = t.inputs.nixpkgs.legacyPackages.x86_64-linux;
  every = import ./every-key.nix;

  # options.nix enabled with each of `given`'s modules beside a system's
  # `assertions`, and each of `modules`.
  evaluate =
    modules:
    lib.evalModules {
      modules = [
        (import ../modules/options.nix { inherit (t.outputs) packages; })
        {
          options.assertions = lib.mkOption {
            type = lib.types.listOf lib.types.unspecified;
            default = [ ];
          };
        }
        {
          _module.args = { inherit pkgs; };
          services.flume.enable = true;
        }
      ]
      ++ modules;
    };
  evaluated = given: evaluate (map (settings: { services.flume = { inherit settings; }; }) given);
  failures =
    given: map (a: a.message) (lib.filter (a: !a.assertion) (evaluated given).config.assertions);

  # The same with `sources` set, and the text of its source `name`.
  sourced = sources: evaluate [ { services.flume = { inherit sources; }; } ];
  sourceFailures =
    sources: map (a: a.message) (lib.filter (a: !a.assertion) (sourced sources).config.assertions);
  sourceText =
    sources: name:
    toml.render "services.flume.sources.${name}"
      (sourced sources).config.services.flume.sources.${name};
  text = given: toml.text (evaluated given).config.services.flume.settings;
  header = "# Written by flume's module from services.flume.settings; a change goes there.";

  # The options `settings` declares, and those a table of it declares.
  declared = (evaluated [ ]).options.services.flume.settings.type.getSubOptions [ ];
  within = option: option.type.getSubOptions [ ];
  hostKeys = within declared.host;
  streamKeys = within declared.streams;

  undeclared = [
    {
      host.terminal = "printed";
      streams.work.points = 8;
    }
  ];
  merged = [
    { repos."acme/acme".stream = "work"; }
    { streams.work.agents = 5; }
  ];

  token = "sk-ant-oat01-a1B2c3D4e5F6g7H8i9J0a1B2c3D4e5F6g7H8i9J0";
  withEnv = value: [ { pools.work.accounts = [ { env.CLAUDE_CODE_OAUTH_TOKEN = value; } ]; } ];
  tokenAt = "services.flume.settings.pools.work.accounts[0].env.CLAUDE_CODE_OAUTH_TOKEN";
  refused = toString (failures (withEnv token));

  shelved.acme = {
    kinds_dir = "/opt/kinds/acme";
    shelf = "upper";
  };
  tokened.acme = {
    kinds_dir = "/opt/kinds/acme";
    inherit token;
  };
  tokenRefused = toString (sourceFailures tokened);
in
{
  "Every key renders its file through the options" =
    t.expect (failures [ every ] == [ ]) "every-key's settings fail: ${toString (failures [ every ])}"
    ++
      t.expect (text [ every ] == builtins.readFile ./files/every-key.toml)
        "every-key's settings through the options render ${
          builtins.toJSON (text [ every ])
        }, not nix/tests/files/every-key.toml";

  "The default settings" = t.expect (
    text [ ] == builtins.readFile ./files/empty.toml
  ) "the default settings render ${builtins.toJSON (text [ ])}, not nix/tests/files/empty.toml";

  "A mistyped declared key" = t.expect (
    !(builtins.tryEval (text [ { streams.work.agents = "five"; } ])).success
  ) "settings.streams.work.agents = \"five\" evaluates";

  "Only the keys flume reads are declared" =
    t.expect (hostKeys ? multiplexer) "host declares no multiplexer"
    ++ t.expect (!(hostKeys ? terminal)) "host declares terminal, the word written before multiplexer"
    ++ t.expect (!(streamKeys ? points)) "a stream declares points, which flume does not read"
    ++ t.expect (!(streamKeys ? pace)) "a stream declares pace, which flume does not read";

  "A key the module does not declare" =
    t.expect (failures undeclared == [ ]) "the undeclared keys fail: ${toString (failures undeclared)}"
    ++
      t.expect (builtins.fromTOML (text undeclared) == lib.head undeclared)
        "host.terminal and a stream's points render ${builtins.toJSON (text undeclared)}, not as written alone";

  "Two modules' settings merge" = t.expect (
    text merged == lib.concatStringsSep "\n" [
      header
      ""
      ''[repos."acme/acme"]''
      ''stream = "work"''
      ""
      "[streams.work]"
      "agents = 5"
      ""
    ]
  ) "two modules' repos and streams render ${builtins.toJSON (text merged)}";

  "A token in an account's env" =
    t.expect (refused != "") "a token in an account's env evaluates"
    ++ t.expect (lib.hasInfix tokenAt refused) "the failure names no ${tokenAt}: ${refused}"
    ++ t.expect (!lib.hasInfix "a1B2c3D4" refused) "the failure holds the token's text"
    ++ t.expect (lib.hasInfix "`$NAME`" refused) "the failure does not say to name a variable as `$NAME`";

  "A variable named instead" =
    t.expect (
      failures (withEnv "$AGENT_TOKEN") == [ ]
    ) "$AGENT_TOKEN fails: ${toString (failures (withEnv "$AGENT_TOKEN"))}"
    ++ t.expect (lib.hasInfix ''CLAUDE_CODE_OAUTH_TOKEN = "$AGENT_TOKEN"'' (text (withEnv "$AGENT_TOKEN"))) "the file does not hold $AGENT_TOKEN: ${text (withEnv "$AGENT_TOKEN")}";

  "A key the module does not declare in a source" =
    t.expect (
      sourceFailures shelved == [ ]
    ) "the source's undeclared key fails: ${toString (sourceFailures shelved)}"
    ++
      t.expect (builtins.fromTOML (sourceText shelved "acme") == shelved.acme)
        "the source acme's kinds_dir and shelf render ${builtins.toJSON (sourceText shelved "acme")}, not as written alone"
    ++ t.expect (lib.hasPrefix "# Written by flume's module from services.flume.sources.acme; a change goes there.\n" (sourceText shelved "acme")) "the source acme's file does not name its option first: ${builtins.toJSON (sourceText shelved "acme")}";

  "A token in a source's file" =
    t.expect (tokenRefused != "") "a token in a source's file evaluates"
    ++ t.expect (lib.hasInfix "services.flume.sources.acme.token" tokenRefused) "the failure names no services.flume.sources.acme.token: ${tokenRefused}"
    ++ t.expect (!lib.hasInfix "a1B2c3D4" tokenRefused) "the failure holds the token's text";
}
