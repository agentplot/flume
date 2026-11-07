# The flake spec's cases on the flake's packages and inputs (the change
# flume-publishes-its-own-flake-and-module, design 2 and 3), for each
# system the flake gives.
t:
let
  inherit (t)
    flake
    lib
    lock
    outputs
    self
    systems
    ;
  gitCrates = import ./git-crates.nix { inherit lib; };
  crates = import ../packages/git-crates.nix;
  file = "nix/packages/git-crates.nix";
  cargoLock = builtins.fromTOML (builtins.readFile ../../Cargo.lock);
  version = (builtins.fromTOML (builtins.readFile ../../Cargo.toml)).workspace.package.version;
  inherit (builtins.fromTOML (builtins.readFile ../../.prototools)) rust;

  each = check: lib.concatMap (system: check system outputs.packages.${system}.flume) systems;

  # weir's revision as git-crates.nix gives it, and one it does not.
  weir = crates."weir-kinds-0.0.0".rev;
  moved = "1111111111111111111111111111111111111111";
  source = url: rev: "git+${url}?rev=${rev}#${rev}";
  weirAt = source "https://github.com/agentplot/weir";
  package = name: src: ''
    [[package]]
    name = "${name}"
    version = "0.0.0"
    source = "${src}"
  '';
  lockOf = packages: builtins.fromTOML (lib.concatStringsSep "\n" ([ "version = 4" ] ++ packages));

  # A lock that moves weir-kinds alone, and one with a git package of no
  # entry beside weir's.
  movedLock = lockOf [
    (package "weir-capture" (weirAt weir))
    (package "weir-kinds" (weirAt moved))
  ];
  extraLock = lockOf [
    (package "weir-capture" (weirAt weir))
    (package "weir-kinds" (weirAt weir))
    (package "another" (source "https://example.com/another" moved))
  ];
  movedFound = gitCrates file movedLock crates;
  extraFound = gitCrates file extraLock crates;

  rootInputs = lock.nodes.${lock.root}.inputs;
  follows = input: lock.nodes.${rootInputs.${input}}.inputs.nixpkgs or null;
  inputNames = [
    "nix-darwin"
    "nixpkgs"
    "rust-overlay"
  ];
in
{
  "The package's version and toolchain are the tree's" = each (
    system: flume:
    let
      name = "packages.${system}.flume";
      rusts = builtins.filter (p: lib.hasPrefix "rust-" (p.name or "")) flume.nativeBuildInputs;
    in
    t.expect (
      toString flume.src == self.outPath
    ) "${name} is made from ${toString flume.src}, not this tree"
    ++ t.expect (flume.version == version) "${name} says ${flume.version}, not Cargo.toml's ${version}"
    ++ t.expect (
      rusts != [ ] && builtins.all (p: p.name == "rust-minimal-${rust}") rusts
    ) "${name}'s Rust is ${toString (map (p: p.name) rusts)}, not .prototools' ${rust}"
    ++
      t.expect ((flume.meta.mainProgram or null) == "flume")
        "${name}'s main program is ${toString (flume.meta.mainProgram or null)}, not flume"
  );

  "Every git crate is fetched by a hash taken at Cargo.lock's revision" =
    gitCrates file cargoLock
      crates;

  "A git crate's revision moves without its hash" = t.expect (
    builtins.length movedFound == 1
    && builtins.all (word: lib.hasInfix word (builtins.head movedFound)) [
      "weir-kinds-0.0.0"
      moved
      weir
      "the hash moves with the revision"
    ]
  ) "a Cargo.lock that moves weir-kinds to ${moved} gives ${builtins.toJSON movedFound}";

  "A git crate with no entry" = t.expect (
    builtins.length extraFound == 1 && lib.hasPrefix "another-0.0.0: " (builtins.head extraFound)
  ) "a Cargo.lock with a git package another of no entry gives ${builtins.toJSON extraFound}";

  "The inputs a taker follows" =
    t.expect (
      builtins.attrNames flake.inputs == inputNames
    ) "flake.nix's inputs are ${toString (builtins.attrNames flake.inputs)}, not ${toString inputNames}"
    ++ t.expect (
      builtins.attrNames rootInputs == inputNames
    ) "flake.lock's inputs are ${toString (builtins.attrNames rootInputs)}, not ${toString inputNames}"
    ++
      lib.concatMap
        (
          input:
          t.expect (
            (flake.inputs.${input}.inputs.nixpkgs.follows or null) == "nixpkgs"
          ) "flake.nix's ${input} does not follow nixpkgs"
          ++ t.expect (follows input == [ "nixpkgs" ]) "flake.lock's ${input} does not follow nixpkgs"
        )
        [
          "nix-darwin"
          "rust-overlay"
        ]
    ++ t.expect (!(flake ? nixConfig)) "flake.nix carries a nixConfig";
}
