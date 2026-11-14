# services.flume, the options both modules share (the change
# flume-publishes-its-own-flake-and-module, design 5, 6 and 8, as
# Boomdrive's nix/modules/options.nix), given the flake's own `packages`:
# flume's package, the flake's own for the host's system, and `settings`,
# flume's managed file under the names the file gives each key (143, C.7;
# record 0055). `settings` is free-form TOML (RFC 42) with the tables and
# keys declared that C.7 types and flume reads at the module's revision;
# a stream's `points` and `pace` and `host.terminal` are not, and pass as
# written, and a label's `reserve` and `keep_free`, the words its table gave
# before `owed` and `reserved`, pass undeclared for flume to refuse (181,
# 182; record 0063). Each number's option says what it counts, its range
# and what holds where it is unset (record 0063). Each declared key and table is null unless set and left out of
# the file, so flume's own default holds; a key the module does not
# declare passes to the file, and flume judges it. Each table of named
# entries merges across modules. `sources` is the host's sources of kinds,
# each the file `sources/<source>.toml` beside config.toml under the names
# that file gives each key (191, C.7; the change
# flumes-module-writes-the-hosts-sources, design 1): `kinds_dir`, required,
# and `roles`, each an absolute path or one under `~`, and any other key
# free-form, for flume to judge. A string shaped as a credential by
# flume's own shapes, in `settings` or in a source, fails evaluation,
# since the files are in the store, which every user reads (core.50;
# design 4). A source's name flume would refuse or pass over fails
# evaluation too, since it is the file's name (design 2).
{ packages }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (lib) mkOption types;
  cfg = config.services.flume;
  toml = (pkgs.formats.toml { }).type;
  credential = import ./credential.nix { inherit lib; };

  inherit (pkgs.stdenv.hostPlatform) system;
  set =
    packages.${system}
      or (throw "the flume flake gives no package for ${system}: it gives ${lib.concatStringsSep ", " (builtins.attrNames packages)}");

  unset =
    type: description:
    mkOption {
      type = types.nullOr type;
      default = null;
      inherit description;
    };
  # A table of the file: its declared keys, and any other key free-form.
  table =
    options:
    types.submodule {
      freeformType = toml;
      inherit options;
    };
  # A table of named tables, `[streams.<stream>]` and the like.
  tables = options: types.attrsOf (table options);

  unsigned = types.ints.unsigned;
  positive = types.ints.positive;
  # A span as flume reads `ends_expire`: `<n>`, `<n>s`, `<n>m` or `<n>h`,
  # `<n>` from 1 (43, C.7).
  span = types.strMatching "0*[1-9][0-9]*[smh]?";
  # A path on the host as a source's file names one: absolute, or under
  # `~` (C.7).
  located = types.strMatching "/.*|~|~/.*";
  view = types.enum [
    "terminal"
    "app"
  ];
  agents = unset toml "An agents table, free-form whole: flume reads and judges its keys at each start (32, 195 to 198). Its `keep_open` is how long an agent that stops short is kept open for an answer, `<n>` or `<n>s` seconds, `<n>m` minutes or `<n>h` hours, `0` keeping none; flume's one hour where every layer leaves it unset (198).";

  stream = {
    agents = unset positive "The stream's agents running at once, within the host's bound, a whole number from 1; the host's bound alone where unset (70, 143).";
    priority = unset types.int "The stream's priority among streams, a whole number, negative allowed, a stream of higher priority starting first; flume's 0 where unset (143).";
    pool = unset types.str "The pool the stream's agents start under; the host's where unset.";
    session = unset types.str "The terminal session the stream's agents open in; the stream's name where unset.";
    multiplexer = unset types.str "The terminal profile the stream binds; the host's where unset.";
    terminal_app = unset types.str "The window profile the stream binds; the host's where unset.";
    title = unset types.str "The title of the stream's window; the stream's name where unset.";
    view = unset view "How the stream's agents are shown, `terminal` or `app`; the host's where unset.";
    labels = unset (tables {
      owed = unset unsigned "The stream's agents the label's places are first in line for, lent while it has nothing waiting, a whole number of agents from 0; flume's 0 where unset (181).";
      reserved = unset unsigned "Of the agents the label is owed, those held for it alone, never lent, a whole number of agents from 0 and no more than `owed`; flume's 0 where unset (181).";
    }) "Each label's share of the stream's agents, `[streams.<stream>.labels.<label>]` (180, 181).";
    kinds = unset (tables {
      session = unset types.str "The session the kind's agents open in.";
      stages = unset (tables {
        session = unset types.str "The session the stage's agents open in.";
      }) "The session each stage of the kind opens in.";
    }) "The session the stream names for a kind, `[streams.<stream>.kinds.<kind>]` (199).";
  };

  account = {
    name = unset types.str "The account's name, one of its pool's.";
    program = unset types.str "The agent program the account runs.";
    env = unset (types.attrsOf types.str) "The account's environment, each value as written, a variable of the host user's login environment named by `$NAME` (29). It holds no credential.";
    agents = unset positive "The agents running under the account at once, a whole number from 1; no bound of its own where unset (144).";
  };

  pool = {
    strategy = unset (types.enum [
      "fill-first"
      "least-busy"
      "most-left"
    ]) "How the pool chooses an account for a start.";
    threshold = unset (types.ints.between 1 100) "A whole percent, from 1 to 100, of any of an account's usage windows, past which the account takes no new start until that window resets; flume's 90 where unset (185).";
    accounts = unset (types.listOf (table account)) "The pool's accounts, in order.";
  };

  kind = {
    stream = unset types.str "The stream the kind's places run in.";
    label = unset types.str "The label the kind's places carry (180).";
    inherit agents;
  };

  # Each string of `value`, at `named`, read with the name of the key whose
  # value it is, a list's items with the list's, by its path where flume's
  # shapes read it as a credential.
  key = name: if builtins.match "[A-Za-z0-9_-]+" name != null then name else builtins.toJSON name;
  credentials =
    named: name: value:
    if builtins.isString value then
      lib.optional (credential.shaped name value) named
    else if builtins.isAttrs value && !(value ? outPath) then
      lib.concatLists (lib.mapAttrsToList (inner: credentials "${named}.${key inner}" inner) value)
    else if builtins.isList value then
      lib.concatLists (lib.imap0 (i: credentials "${named}[${toString i}]" name) value)
    else
      [ ];
  found =
    credentials "services.flume.settings" "" cfg.settings
    ++ lib.concatLists (
      lib.mapAttrsToList (name: credentials "services.flume.sources.${key name}" "") cfg.sources
    );

  # Why flume takes no source of the name `name`, or null where it takes
  # it: weir's name (weir C.4, `weir_kinds::check_name`), and none of the
  # sources of flume's own kinds, the repository's and a launch's
  # (`source_fault`; 154, 191).
  reserved = {
    flume = "flume's own kinds";
    repo = "the repository's kinds";
    launch = "a launch's kinds";
  };
  nameFault =
    name:
    if builtins.match "[A-Za-z0-9_-]+" name == null then
      "is no source's name: a source's name is one or more ASCII letters, digits, `_` and `-` (weir C.4), since it is the name of its file in ~/.config/flume/sources/, which flume reads and the activation links"
    else if reserved ? ${name} then
      "is the source of ${reserved.${name}}, which no host declares (154, 191): name the source otherwise"
    else
      null;
in
{
  options.services.flume = {
    enable = lib.mkEnableOption "flume, installed with its managed file written from `settings`";

    package = mkOption {
      type = types.nullOr types.package;
      default = set.flume;
      defaultText = lib.literalExpression "flume.packages.\${pkgs.stdenv.hostPlatform.system}.flume";
      description = "flume, put in the system profile; `null` installs none, and the file is still written.";
    };

    settings = mkOption {
      type = table {
        host = unset (table {
          agents = unset positive "The agents of flume's places running at once on the host, every caller's and stream's together, a whole number from 1; no bound where unset (70).";
          ends_expire = unset span "How long an end stays pending for a recipient before status shows it expired, `<n>` or `<n>s` seconds, `<n>m` minutes or `<n>h` hours, `<n>` from 1, at least a second; flume's `168h` where unset (43, C.7).";
          multiplexer = unset types.str "The terminal profile the host binds; the printed command where unset (190).";
          pool = unset types.str "The host's own pool (144).";
          terminal_app = unset types.str "The window profile the host binds; the printed window where unset (188).";
          view = unset view "How the host's streams are shown, `terminal` or `app` (186).";
          pin = unset (types.listOf types.str) "The keys the local layer may not set, each a dotted key (C.7).";
        }) "`[host]`.";
        streams = unset (tables stream) "The host's streams, `[streams.<stream>]` (143).";
        pools = unset (tables pool) "The host's pools, `[pools.<pool>]` (144).";
        kinds = unset (tables kind) "Each kind's stream, label and agents in every repository, `[kinds.<kind>]`.";
        repos =
          unset
            (tables {
              stream = unset types.str "The stream the repository's places run in.";
              inherit agents;
              kinds = unset (tables kind) "Each kind's stream, label and agents in the repository.";
            })
            "Each repository's settings, `[repos.\"<repository>\"]`, its key an address, `<owner>/<name>`, or a folder, as written.";
        inherit agents;
      };
      default = { };
      description = "flume's managed file, `~/.config/flume/config.toml`, each key named as the file names it (C.7), rendered at evaluation. It holds no credential: name a variable of the host user's login environment by `$NAME` in its place.";
    };

    sources = mkOption {
      type = types.attrsOf (table {
        kinds_dir = mkOption {
          type = located;
          description = "Where the source's packages lie, an absolute path or one under `~` (191).";
        };
        roles = unset located "The source's roles file, an absolute path or one under `~`, which the source's own tool writes (193).";
      });
      default = { };
      example = lib.literalExpression ''{ acme.kinds_dir = "~/.local/share/flume/kinds/acme"; }'';
      description = "The host's sources of kinds, each the file `~/.config/flume/sources/<source>.toml`, each key named as the file names it (191, C.7), rendered at evaluation and linked by the activation. A source's name is ASCII letters, digits, `_` and `-`, and not `flume`, `repo` or `launch`. It holds no credential. The roles file and the packages it names are the source's own tool's, which the module never writes.";
    };
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = found == [ ];
        message = "${builtins.head found} is shaped as a credential: flume's file is in the store, which every user reads, so it holds none. Name a variable of the host user's login environment in its place, as `$NAME`, which flume reads at each start (29).";
      }
    ]
    ++ lib.mapAttrsToList (name: _: {
      assertion = nameFault name == null;
      message = "services.flume.sources.${key name} ${toString (nameFault name)}.";
    }) cfg.sources;
  };
}
