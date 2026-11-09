# flume's managed files, config.toml rendered from services.flume.settings
# and each source's from services.flume.sources, at evaluation (the change
# flume-publishes-its-own-flake-and-module, design 7, as Boomdrive's
# nix/modules/toml.nix): so the gate compares the text a module writes
# without building it, and one option gives one text on either platform. Top-level keys come before tables, each sorted by name,
# and a key is bare where TOML allows. A key whose value is null is unset,
# and left out, so flume's own default holds (design 6). An empty table is
# written as its header, since `[repos."<repository>"]` alone names a
# repository flume reads. Beside Boomdrive's, it takes floats, written with
# a point or an exponent so they never read as integers, and any list: a
# non-empty list of tables only as `[[...]]` sections, any other inline, a
# table in it as an inline table, as a program's own settings hold them
# (195). Any other value, a path or a derivation among them, fails, naming
# its path.
#
# `render option value` is the text of the file rendered from the option
# whose path is `option`, `services.flume.sources.acme` say: its first line
# names the option, and it fails with the first of the value's refusals,
# each naming the path under the option of a value the file cannot hold
# (the change flumes-module-writes-the-hosts-sources, design 3). `text` and
# `refusals` are `services.flume.settings`'s, config.toml's.
{ lib }:
let
  inherit (builtins)
    attrNames
    filter
    isAttrs
    isBool
    isFloat
    isInt
    isList
    isString
    typeOf
    ;
  inherit (lib)
    concatLists
    concatMap
    concatMapStringsSep
    concatStringsSep
    filterAttrs
    imap0
    mapAttrsToList
    optionals
    ;

  header = option: "# Written by flume's module from ${option}; a change goes there.";

  # TOML's basic string: `"`, `\` and every control character escaped.
  hex = n: lib.fixedWidthString 4 "0" (lib.toHexString n);
  controls = lib.range 1 31 ++ [ 127 ];
  short = {
    "8" = "\\b";
    "9" = "\\t";
    "10" = "\\n";
    "12" = "\\f";
    "13" = "\\r";
  };
  string =
    text:
    ''"${
      builtins.replaceStrings
        (
          [
            "\\"
            ''"''
          ]
          ++ map (n: builtins.fromJSON ''"\u${hex n}"'') controls
        )
        (
          [
            "\\\\"
            ''\"''
          ]
          ++ map (n: short.${toString n} or "\\u${hex n}") controls
        )
        text
    }"'';

  # A key as TOML writes it: bare where it can be, else quoted.
  key = name: if builtins.match "[A-Za-z0-9_-]+" name != null then name else string name;
  headerOf = concatMapStringsSep "." key;

  # A float as JSON writes it, which holds a point or an exponent; one that
  # holds neither is given a point.
  float =
    value:
    let
      written = builtins.toJSON value;
    in
    if builtins.match ".*[.eEn].*" written != null then written else "${written}.0";

  # A table, never a derivation or anything else that stands for a path.
  isTable = value: isAttrs value && !(value ? outPath);
  isTables = value: isList value && value != [ ] && builtins.all isTable value;
  isScalar = value: isString value || isBool value || isInt value || isFloat value;

  at = named: name: if named == "" then name else "${named}.${key name}";

  # What `value` is, as a failure names it.
  what =
    value:
    if isAttrs value then
      "a derivation"
    else if value == null then
      "null"
    else
      "a ${typeOf value}";

  refusal =
    option: named: value:
    "${option}.${named} is ${what value}: flume's file takes strings, booleans, integers, floats, lists and tables";

  # Every value of `value` at `named` under `option` the file cannot hold,
  # by its path: a null is left out of a table, and refused in a list.
  refusalsOf =
    option: named: value:
    if isScalar value then
      [ ]
    else if isTable value then
      concatLists (
        mapAttrsToList (name: inner: refusalsOf option (at named name) inner) (
          filterAttrs (_: inner: inner != null) value
        )
      )
    else if isList value then
      concatLists (imap0 (i: refusalsOf option "${named}[${toString i}]") value)
    else
      [ (refusal option named value) ];

  # A value written on its key's line, or in an inline list or table.
  inline =
    value:
    if isString value then
      string value
    else if isBool value then
      lib.boolToString value
    else if isInt value then
      toString value
    else if isFloat value then
      float value
    else if isList value then
      "[${concatMapStringsSep ", " inline value}]"
    else
      let
        set = filterAttrs (_: inner: inner != null) value;
      in
      if set == { } then
        "{}"
      else
        "{ ${concatStringsSep ", " (mapAttrsToList (name: inner: "${key name} = ${inline inner}") set)} }";

  # A table's lines: its keys', then its tables', each sorted by name.
  # `path` is its keys from the top.
  table =
    path: attrs:
    let
      set = filterAttrs (_: value: value != null) attrs;
      nested = name: isTable set.${name} || isTables set.${name};
      sectionOf =
        name:
        let
          value = set.${name};
        in
        if isTable value then
          section (path ++ [ name ]) value
        else
          concatMap (element (path ++ [ name ])) value;
    in
    {
      keys = map (name: "${key name} = ${inline set.${name}}") (
        filter (name: !nested name) (attrNames set)
      );
      sections = concatMap sectionOf (filter nested (attrNames set));
    };

  # `[a.b]`, written where it holds a key or nothing at all, so an empty
  # table is its header and a table holding only tables is named by theirs.
  section =
    path: attrs:
    let
      inner = table path attrs;
    in
    optionals (inner.keys != [ ] || inner.sections == [ ]) (
      [
        ""
        "[${headerOf path}]"
      ]
      ++ inner.keys
    )
    ++ inner.sections;

  # `[[a.b]]`, one for each table of the list.
  element =
    path: attrs:
    let
      inner = table path attrs;
    in
    [
      ""
      "[[${headerOf path}]]"
    ]
    ++ inner.keys
    ++ inner.sections;

  settings = "services.flume.settings";

  render =
    option: value:
    let
      refused = refusalsOf option "" value;
      top = table [ ] value;
    in
    if refused != [ ] then
      throw (builtins.head refused)
    else
      concatStringsSep "\n" ([ (header option) ] ++ top.keys ++ top.sections) + "\n";
in
{
  inherit render;

  refusals = refusalsOf settings "";
  text = render settings;
}
