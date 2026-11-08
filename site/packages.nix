# The packages bun.lock locks, as bun's install cache holds them, for
# site:install, which installs from them with no network in the gate's
# sandbox (record 0059). Each package is fetched by the integrity the lock
# gives it, as the devenv shell builds its inputs outside the sandbox, so a
# lock that moves fetches what it names and no hash is kept here; a package
# whose os or cpu leaves out the host's is left out, as bun leaves it out.
# The bun devenv.nix pins writes the cache itself, installing the lock from
# the registry's packages served on the build's own loopback, so its cache
# is laid out as that bun lays it out. A GitHub package, which bun fetches
# from GitHub's API rather than a registry, is put where bun looks for it,
# `@GH@<tag>@@@1` with its tag in `.bun-tag`.
{
  pkgs,
  lib,
  lock ? ./bun.lock,
  manifest ? ./package.json,
}:
let
  inherit (pkgs.stdenv) hostPlatform;

  # bun.lock is JSON with trailing commas, which fromJSON refuses.
  text = lib.concatMapStrings (part: if builtins.isList part then builtins.head part else part) (
    builtins.split ",([[:space:]]*[]}])" (builtins.readFile lock)
  );
  locked = builtins.fromJSON text;

  os =
    if hostPlatform.isDarwin then
      "darwin"
    else if hostPlatform.isLinux then
      "linux"
    else
      throw "site/packages.nix: no os of bun's for ${hostPlatform.system}";
  cpu =
    if hostPlatform.isAarch64 then
      "arm64"
    else if hostPlatform.isx86_64 then
      "x64"
    else
      throw "site/packages.nix: no cpu of bun's for ${hostPlatform.system}";

  # A field npm's way: absent is every platform, a list of negations every
  # one it does not name, and otherwise only those it names.
  fits =
    want: meta: field:
    let
      named = lib.toList meta.${field};
    in
    !(meta ? ${field})
    || (
      if lib.all (lib.hasPrefix "!") named then !(lib.elem "!${want}" named) else lib.elem want named
    );

  # `<name>@<spec>`, the name scoped or not.
  split =
    ident:
    let
      at = lib.lists.findFirstIndex (c: c == "@") null (lib.reverseList (lib.stringToCharacters ident));
      cut = lib.stringLength ident - 1 - at;
    in
    {
      name = builtins.substring 0 cut ident;
      spec = builtins.substring (cut + 1) (-1) ident;
    };

  # A package of the lock: a registry's `[ident, registry, meta,
  # integrity]`, or a GitHub one's `[ident, meta, tag, integrity]`.
  entryOf =
    tuple:
    let
      ident = builtins.elemAt tuple 0;
      inherit (split ident) name spec;
      github = lib.hasPrefix "github:" spec;
      integrity = builtins.elemAt tuple 3;
    in
    if github then
      let
        tag = builtins.elemAt tuple 2;
        at = lib.splitString "#" (lib.removePrefix "github:" spec);
      in
      {
        inherit ident tag;
        meta = builtins.elemAt tuple 1;
        src = pkgs.fetchurl {
          name = "${tag}.tgz";
          url = "https://api.github.com/repos/${builtins.head at}/tarball/${lib.last at}";
          hash = integrity;
        };
      }
    else
      let
        file = "${lib.last (lib.splitString "/" name)}-${spec}.tgz";
      in
      {
        inherit ident;
        tag = null;
        meta = builtins.elemAt tuple 2;
        path = "${name}/-/${file}";
        src = pkgs.fetchurl {
          url = "https://registry.npmjs.org/${name}/-/${file}";
          hash = integrity;
        };
      };

  # One entry a package, however many places of the tree hold it.
  entries = lib.filter (e: fits os e.meta "os" && fits cpu e.meta "cpu") (
    lib.attrValues (
      lib.listToAttrs (
        map (
          tuple:
          let
            e = entryOf tuple;
          in
          lib.nameValuePair e.ident e
        ) (lib.attrValues locked.packages)
      )
    )
  );
  registry = lib.filter (e: e.tag == null) entries;
  github = lib.filter (e: e.tag != null) entries;

  # The registry: each tarball at the path bun asks for it, on a port the
  # kernel gives, written to the file its second argument names.
  serve = pkgs.writeText "flume-site-registry.js" ''
    const root = process.argv[2];
    const server = Bun.serve({
      hostname: "127.0.0.1",
      port: 0,
      async fetch(request) {
        const path = decodeURIComponent(new URL(request.url).pathname);
        const file = Bun.file(root + path);
        if (path.includes("..") || !(await file.exists())) {
          console.error(`registry: no ''${path}`);
          return new Response("not found", { status: 404 });
        }
        return new Response(file);
      },
    });
    await Bun.write(process.argv[3], String(server.port));
  '';
in
pkgs.runCommand "flume-site-packages"
  {
    nativeBuildInputs = [
      pkgs.bun
      pkgs.gnutar
    ];
  }
  ''
    export HOME=$TMPDIR/home
    mkdir -p "$HOME" registry work "$out"
    ${lib.concatMapStrings (e: ''
      mkdir -p "registry/${dirOf e.path}"
      ln -s ${e.src} "registry/${e.path}"
    '') registry}
    ${lib.concatMapStrings (e: ''
      mkdir -p "$out/@GH@${e.tag}@@@1"
      tar -xzf ${e.src} --strip-components=1 --no-same-owner -C "$out/@GH@${e.tag}@@@1"
      printf %s '${e.tag}' > "$out/@GH@${e.tag}@@@1/.bun-tag"
    '') github}
    cp ${manifest} work/package.json
    cp ${lock} work/bun.lock

    bun ${serve} "$PWD/registry" "$PWD/port" &
    server=$!
    trap 'kill $server' EXIT
    for _ in $(seq 100); do [ -s port ] && break; sleep 0.1; done
    [ -s port ] || { echo "site/packages.nix: the registry did not start" >&2; exit 1; }

    cd work
    bun install --frozen-lockfile --ignore-scripts --no-progress \
      --registry "http://127.0.0.1:$(cat ../port)/" --cache-dir "$out"
    # The links bun keeps beside its cache's folders, `<name>/<version>@@@1`,
    # point into the cache by its absolute path, and bun reads none of them
    # to install a lock: each goes, and its folder once it holds no other.
    find "$out" -type l -name '*@@@*' -lname "$out/*" > links
    while read -r link; do
      rm "$link"
      rmdir "$(dirname "$link")" 2>/dev/null || true
    done < links
    # bun names a package's folder by its registry's host where that is
    # not npm's, `<name>@<version>@@127.0.0.1@@@1`: taken out, each is
    # named as a package of npm's registry, where the lock says it is.
    find "$out" -maxdepth 2 -type d -name '*@@127.0.0.1@@@*' > folders
    while read -r folder; do
      mv "$folder" "$(printf %s "$folder" | sed 's/@@127\.0\.0\.1@@@/@@@/')"
    done < folders
  ''
