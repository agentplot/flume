# flume's own shapes of a credential in Nix (the change
# flume-publishes-its-own-flake-and-module, design 8): flume-core's
# `shaped_as_credential` (crates/flume-core/src/own.rs), as core.50 applies
# it, so a string of services.flume.settings shaped as one never reaches
# the store, which every user reads. A string is shaped as a credential
# where it holds an issuer's prefix opening a run of sixteen or more of a
# token's characters, a private key's opening, a JSON web token, or a run
# of 32 or more mixing upper case, lower case and digits apart from any
# variable it names, or where its key is named as a secret's and it names
# no variable. Both are held to one table of cases,
# crates/flume-core/tests/credential-shapes.json (nix/tests/credential.nix).
#
# The text is read as its runs of a token's characters, letters, digits,
# `_` and `-`, each with the one character after it: a run opens at the
# text's start or after a character that is no token's and no `.`.
{ lib }:
let
  inherit (builtins)
    elemAt
    genList
    head
    length
    stringLength
    substring
    ;
  inherit (lib) any hasPrefix hasSuffix;

  # The issuers' prefixes a token opens with, own.rs's PREFIXES.
  prefixes = [
    "sk-"
    "ghp_"
    "gho_"
    "ghu_"
    "ghs_"
    "ghr_"
    "github_pat_"
    "glpat-"
    "xoxb-"
    "xoxp-"
    "xoxa-"
    "xapp-"
    "AKIA"
    "ASIA"
    "AIza"
    "ya29."
    "npm_"
    "pypi-"
    "hf_"
    "sk_live_"
    "rk_live_"
  ];

  # The endings of a key's name, in lower case with `_` and `-` taken out,
  # whose value is a secret, own.rs's SECRET_NAMES.
  secretNames = [
    "token"
    "secret"
    "password"
    "passwd"
    "apikey"
    "privatekey"
    "accesskey"
    "clientsecret"
    "authorization"
    "credentials"
  ];

  characters = lib.stringToCharacters;
  is = class: c: builtins.match class c != null;
  named = is "[A-Za-z0-9_]";
  opensName = is "[A-Za-z_]";

  # A variable's name, as an account's `env` reads one (29).
  isName = name: name != "" && opensName (substring 0 1 name) && lib.all named (characters name);

  shaped =
    name: text:
    let
      # Each run of `text`, with the character after it, "" after the last.
      parts = builtins.split "([^A-Za-z0-9_-])" text;
      count = (length parts + 1) / 2;
      run = k: elemAt parts (2 * k);
      after = k: if 2 * k + 1 < length parts then head (elemAt parts (2 * k + 1)) else "";
      runs = genList (k: k) count;

      # Whether run `k` stands on its own, not after a `.`.
      opens = k: k == 0 || after (k - 1) != ".";

      prefixed = any (
        k:
        opens k
        && any (
          prefix:
          if hasSuffix "." prefix then
            run k == lib.removeSuffix "." prefix
            && after k == "."
            && k + 1 < count
            && stringLength (run (k + 1)) >= 16
          else
            hasPrefix prefix (run k) && stringLength (run k) - stringLength prefix >= 16
        ) prefixes
      ) runs;

      privateKey = lib.hasInfix "-----BEGIN" text && lib.hasInfix "PRIVATE KEY-----" text;

      webToken = any (
        k:
        opens k
        && hasPrefix "eyJ" (run k)
        && after k == "."
        && k + 2 < count
        && run (k + 1) != ""
        && after (k + 1) == "."
        && run (k + 2) != ""
      ) runs;

      # A `$` after run `k` naming a variable: `${NAME}`, which is all of
      # run `k + 2`, or `$NAME`, which opens run `k + 1` up to its first
      # `-`.
      braced =
        k:
        after k == "$"
        && run (k + 1) == ""
        && after (k + 1) == "{"
        && after (k + 2) == "}"
        && isName (run (k + 2));
      bare = k: after k == "$" && k + 1 < count && opensName (substring 0 1 (run (k + 1)));
      variable = k: braced k || bare k;

      # Each run with every variable put out of it.
      apart =
        k:
        if k >= 2 && braced (k - 2) then
          ""
        else if k >= 1 && bare (k - 1) then
          let
            whole = run k;
          in
          substring (stringLength (head (lib.splitString "-" whole))) (stringLength whole) whole
        else
          run k;
      mixed = any (
        k:
        let
          chars = characters (apart k);
        in
        length chars >= 32 && any (is "[A-Z]") chars && any (is "[a-z]") chars && any (is "[0-9]") chars
      ) runs;

      stripped = builtins.replaceStrings [ "_" "-" ] [ "" "" ] (lib.toLower name);
      secretNamed =
        any (secret: hasSuffix secret stripped) secretNames && text != "" && !(any variable runs);
    in
    prefixed || privateKey || webToken || mixed || secretNamed;
in
{
  inherit prefixes secretNames shaped;
}
