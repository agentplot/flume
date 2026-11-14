# The host-module spec's settings (the change
# flume-publishes-its-own-flake-and-module, design 5, 6 and 8), over
# options.nix evaluated alone beside a system's `assertions`: every key
# renders its file, only the keys flume reads are declared, an account's
# `usage` not among them since flume refuses it (record 0062), an undeclared
# key passes as written, two modules' tables merge into one file, and a
# string shaped as a credential fails, naming its path and never its value.
# A source of `sources` passes an undeclared key to its file as written,
# and a token in it fails the same way, and a source's name flume would
# refuse fails, naming its option and why (the change
# flumes-module-writes-the-hosts-sources, design 1, 2 and 4).
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
  labelKeys = within streamKeys.labels;
  poolKeys = within declared.pools;
  accountKeys = within poolKeys.accounts;

  # Each number's option, by its name, with the words of its range and
  # `unset`, which its description holds (the host-module spec's "Each
  # number says its unit, range and default"; 70, 143, 181, 185, 198, C.7).
  time = [
    "seconds"
    "minutes"
    "hours"
  ];
  numbers = [
    {
      name = "host.agents";
      option = hostKeys.agents;
      words = [ "from 1" ];
    }
    {
      name = "host.ends_expire";
      option = hostKeys.ends_expire;
      words = time;
    }
    {
      name = "a stream's agents";
      option = streamKeys.agents;
      words = [ "from 1" ];
    }
    {
      name = "a stream's priority";
      option = streamKeys.priority;
      words = [
        "whole number"
        "higher"
      ];
    }
    {
      name = "a label's owed";
      option = labelKeys.owed;
      words = [ "from 0" ];
    }
    {
      name = "a label's reserved";
      option = labelKeys.reserved;
      words = [ "from 0" ];
    }
    {
      name = "a pool's threshold";
      option = poolKeys.threshold;
      words = [ "1 to 100" ];
    }
    {
      name = "an account's agents";
      option = accountKeys.agents;
      words = [ "from 1" ];
    }
    {
      name = "the agents table";
      option = declared.agents;
      words = time;
    }
  ];
  says =
    number:
    lib.concatMap (
      word:
      t.expect (lib.hasInfix word number.option.description) "${number.name}'s description says not ${word}: ${number.option.description}"
    ) (number.words ++ [ "unset" ]);

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

  # Each name flume refuses, by its option as the failure names it, and
  # why.
  refusedNames = {
    flume = {
      option = "services.flume.sources.flume";
      why = "flume's own kinds";
    };
    repo = {
      option = "services.flume.sources.repo";
      why = "the repository's kinds";
    };
    launch = {
      option = "services.flume.sources.launch";
      why = "a launch's kinds";
    };
    ".acme" = {
      option = ''services.flume.sources.".acme"'';
      why = "ASCII letters, digits, `_` and `-`";
    };
    "ac.me" = {
      option = ''services.flume.sources."ac.me"'';
      why = "ASCII letters, digits, `_` and `-`";
    };
    "a/b" = {
      option = ''services.flume.sources."a/b"'';
      why = "ASCII letters, digits, `_` and `-`";
    };
  };
  named = name: { ${name}.kinds_dir = "/opt/kinds"; };
  nameRefused =
    name:
    let
      inherit (refusedNames.${name}) option why;
      said = sourceFailures (named name);
    in
    t.expect (builtins.length said == 1)
      "the source ${builtins.toJSON name} fails ${toString (builtins.length said)} assertions: ${toString said}"
    ++ t.expect (lib.any (lib.hasInfix option) said) "the source ${builtins.toJSON name}'s failure names no ${option}: ${toString said}"
    ++ t.expect (lib.any (lib.hasInfix why) said) "the source ${builtins.toJSON name}'s failure says not why, ${why}: ${toString said}";
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

  "A bound of zero" =
    t.expect (
      !(builtins.tryEval (text [ { host.agents = 0; } ])).success
    ) "settings.host.agents = 0 evaluates"
    ++ t.expect (
      !(builtins.tryEval (text [ { streams.work.agents = 0; } ])).success
    ) "settings.streams.work.agents = 0 evaluates";

  "Each number says its unit, range and default" = lib.concatMap says numbers;

  "Only the keys flume reads are declared" =
    t.expect (hostKeys ? multiplexer) "host declares no multiplexer"
    ++ t.expect (hostKeys ? ends_expire) "host declares no ends_expire"
    ++ t.expect (!(hostKeys ? terminal)) "host declares terminal, the word written before multiplexer"
    ++ t.expect (labelKeys ? owed) "a label declares no owed"
    ++ t.expect (labelKeys ? reserved) "a label declares no reserved"
    ++ t.expect (!(labelKeys ? reserve)) "a label declares reserve, the word written before owed"
    ++ t.expect (
      !(labelKeys ? keep_free)
    ) "a label declares keep_free, the word written before reserved"
    ++ t.expect (!(streamKeys ? points)) "a stream declares points, which flume does not read"
    ++ t.expect (!(streamKeys ? pace)) "a stream declares pace, which flume does not read"
    ++
      t.expect
        (
          lib.attrNames (
            removeAttrs accountKeys [
              "_module"
              "_freeformOptions"
            ]
          ) == [
            "agents"
            "env"
            "name"
            "program"
          ]
        )
        "an account declares ${toString (lib.attrNames accountKeys)}, where flume reads name, program, env and agents and refuses usage";

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

  "Names flume refuses" =
    lib.concatMap nameRefused (builtins.attrNames refusedNames)
    ++ t.expect (
      sourceFailures (named "acme-2_b") == [ ]
    ) "the source acme-2_b fails: ${toString (sourceFailures (named "acme-2_b"))}";
}
