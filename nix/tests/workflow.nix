# The-gate spec's CI build of the flake's package (the change
# flume-publishes-its-own-flake-and-module, design 11), held by the
# workflow's text: .github/workflows/ci.yml read line by line, as yamlfmt
# lays it out. GitHub runs it; the gate holds what it says.
t:
let
  inherit (t) lib;
  inherit (lib.lists) findFirstIndex;

  lines = lib.splitString "\n" (builtins.readFile ../../.github/workflows/ci.yml);

  # The lines after the line `head`, up to the next at its own indent or
  # less, less the empty ones.
  blockAfter =
    indent: head: from:
    let
      at = findFirstIndex (l: l == "${indent}${head}") null from;
      after = lib.drop (at + 1) from;
      end = findFirstIndex (
        l: l != "" && builtins.match "${indent} .*" l == null
      ) (builtins.length after) after;
    in
    if at == null then [ ] else builtins.filter (l: l != "") (lib.take end after);

  on = blockAfter "" "on:" lines;
  jobs = blockAfter "" "jobs:" lines;
  flake = blockAfter "  " "flake:" jobs;
  has = line: from: builtins.elem line from;
in
{
  "The workflow's build job" =
    t.expect (flake != [ ]) "the workflow holds no job flake"
    ++ t.expect (has "        os: [ubuntu-latest, macos-latest]" flake) "the job flake runs on no ubuntu-latest and macos-latest"
    ++ t.expect (has "    runs-on: \${{ matrix.os }}" flake) "the job flake runs on no matrix.os"
    ++ t.expect (has "      fail-fast: false" flake) "the job flake's strategy names no fail-fast: false"
    ++ t.expect (has "      - run: devenv shell -- moon run nix:build" flake) "the job flake runs no devenv shell -- moon run nix:build"
    ++ t.expect (
      has "  push:" on && has "    branches: [main]" on && has "  pull_request:" on
    ) "the workflow runs on no push to main and pull request: ${builtins.toJSON on}"
    ++ t.expect (!lib.any (lib.hasPrefix "    if:") flake) "the job flake runs on a condition";
}
