//! The place profile over plain git worktree under the pinned sluice
//! (C.1, C.6; design 9): a fresh clone of a moon repository whose
//! `.prototools` pins a moon nothing installed, readied through the
//! resolver's data and the profile by the pinned sluice's equip, which
//! installs the pins, checks moon's version and runs its setup, each
//! step named as sluice names it (S89); and the same clone with no pins,
//! whose `moon --version` fails as deneb's place did. proto and moon are
//! stand-ins of the test's own, and no moon of the host's is on the
//! steps' path.

use crate::common::*;
use flume::profiles::place::git_worktree::{GitWorktree, Login};
use flume::profiles::place_data;
use flume_seams::*;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// The version of moon the scratch repository pins.
const MOON: &str = "2.6.1";

/// The toolchain's script: proto's home in the clone's common git
/// directory, and on `PATH` the stand-in for proto and each tool proto
/// installed there, the system's tools after them and no moon of the
/// host's, read as the environment is sourced, installing nothing.
const ENV: &str = r#"common=$(git rev-parse --path-format=absolute --git-common-dir)
cat <<EOF
PROTO_HOME='$common/proto'
export PROTO_HOME
PATH='$PWD/tools/bin:$common/proto/bin:/usr/bin:/bin'
export PATH
EOF
"#;

/// The stand-in for proto: its install puts a stand-in for the moon
/// `.prototools` pins in `$PROTO_HOME/bin`, which answers its version and
/// passes its setup.
const PROTO: &str = r#"#!/bin/sh
if [ "$*" != "install --config-mode upwards" ]; then
  echo "proto: not the install: $*" >&2
  exit 2
fi
version=$(sed -n 's/^moon = "\(.*\)"$/\1/p' .prototools)
mkdir -p "$PROTO_HOME/bin"
cat > "$PROTO_HOME/bin/moon" <<EOF
#!/bin/sh
case "\$1" in
--version) echo "moon $version" ;;
setup) echo "moon $version set up" ;;
*) echo "moon: not run here: \$*" >&2; exit 2 ;;
esac
EOF
chmod +x "$PROTO_HOME/bin/moon"
echo "installed moon $version"
"#;

/// A moon repository on main, pinning moon in `.prototools` where
/// `pinned`, whose sluice policy names the moon runner and a toolchain
/// that puts the stand-in for proto on `PATH` and installs nothing,
/// pushed to a bare remote: a fresh clone of it.
fn fresh_clone(label: &str, pinned: bool) -> Repository {
    let origin = Repository::new(label);
    origin.write(
        ".config/sluice.toml",
        "[tests]\nrunner = \"moon\"\ntoolchain = [\"sh\", \"tools/env.sh\"]\n\n[branches.main]\nremote = \"origin\"\n",
    );
    origin.write("tools/env.sh", ENV);
    origin.write("tools/bin/proto", PROTO);
    std::fs::set_permissions(
        origin.dir.join("tools/bin/proto"),
        std::fs::Permissions::from_mode(0o755),
    )
    .expect("the stand-in for proto made executable");
    if pinned {
        origin.write(".prototools", &format!("moon = \"{MOON}\"\n"));
    }
    origin.commit("moon's policy, proto's pins");
    let remote = origin.bare_remote();
    Repository::clone_of(&remote, Scratch::new(&format!("{label}-clone")))
}

/// The test's `PATH`, the pinned sluice on it, with every folder that
/// holds a moon left out.
fn path_without_moon() -> String {
    std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .filter(|dir| !dir.is_empty() && !Path::new(dir).join("moon").exists())
        .collect::<Vec<_>>()
        .join(":")
}

/// A place of the clone `repository` made and readied at its launch's
/// revision through the resolver's data and the git worktree profile,
/// its steps run under the pinned sluice: the readiness, and the
/// clone's common git directory.
fn readied(repository: &Repository, name: &str) -> (Readiness, String) {
    let launched = Revision::new(repository.head());
    let data = place_data(&repository.name(), Some("sluice"), "main", Some(&launched))
        .expect("the place's data");
    let home = repository.scratch.path("home");
    std::fs::create_dir_all(&home).expect("a scratch home");
    let profile = GitWorktree::with_login(
        data,
        Login::given(&[
            ("HOME", &home.to_string_lossy()),
            ("PATH", &path_without_moon()),
            ("LANG", "C"),
        ]),
    )
    .with_own(&repository.scratch.path("data/flume"));
    let place = PlaceRef {
        repository: repository.name(),
        location: profile
            .location(&repository.name(), &format!("change--{name}"))
            .expect("the place's location"),
        branch: format!("change/{name}"),
        base: "main".into(),
    };
    assert_eq!(
        profile.make(&place.repository, &launched, &place.branch, &place.location),
        MakeOutcome::Made
    );
    let common = git(
        &repository.dir,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    );
    (profile.ready(&place, &launched), common)
}

/// The steps a readiness ran, each with whether it passed.
fn steps(readiness: &Readiness) -> Vec<(&str, bool)> {
    readiness
        .steps
        .iter()
        .map(|step| (step.step.as_str(), step.passed))
        .collect()
}

#[test]
fn a_fresh_clones_pins_are_installed_by_the_pinned_sluice() {
    let repository = fresh_clone("pinned", true);
    let (readiness, common) = readied(&repository, "pinned");
    assert_eq!(
        steps(&readiness),
        vec![
            ("toolchain", true),
            ("proto install", true),
            ("moon --version", true),
            ("moon setup", true),
        ],
        "{readiness:?}"
    );
    let installed = readiness
        .steps
        .iter()
        .find(|step| step.step == "proto install")
        .expect("the install");
    assert!(
        installed.words.contains(&format!("installed moon {MOON}")),
        "{}",
        installed.words
    );
    assert!(
        Path::new(&common).join("proto/bin/moon").is_file(),
        "the stand-in for moon is not installed under {common}/proto"
    );
}

#[test]
fn without_the_install_moon_is_not_there_to_answer_its_version() {
    let repository = fresh_clone("unpinned", false);
    let (readiness, common) = readied(&repository, "unpinned");
    assert_eq!(
        steps(&readiness),
        vec![("toolchain", true), ("moon --version", false)],
        "{readiness:?}"
    );
    assert!(!Path::new(&common).join("proto/bin/moon").exists());
}
