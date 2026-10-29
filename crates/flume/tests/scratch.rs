//! The scratch-repository helper isolates every git call (core.22).

mod common;

use common::*;

#[test]
fn a_scratch_repository_reads_no_configuration_of_the_host() {
    let repository = Repository::new("isolated");
    let helper = std::process::Command::new("git")
        .arg("-C")
        .arg(&repository.dir)
        .args(["config", "--get-all", "credential.helper"])
        .output()
        .expect("git runs");
    assert_eq!(
        String::from_utf8_lossy(&helper.stdout).trim(),
        "",
        "a credential helper is configured"
    );
    let origins = repository.git(&["config", "--list", "--show-origin"]);
    for line in origins.lines() {
        assert!(
            line.starts_with("file:.git/config") || line.starts_with("command line:"),
            "configuration from outside the scratch repository: {line}"
        );
    }
    assert_eq!(repository.git(&["branch", "--show-current"]), "main");
}

#[test]
fn a_clone_names_its_bare_remote_as_origin() {
    let repository = Repository::new("remote");
    let remote = repository.bare_remote();
    let clone = Repository::clone_of(&remote, Scratch::new("clone"));
    assert_eq!(clone.head(), repository.head());
    assert_eq!(
        clone.git(&["rev-parse", "refs/remotes/origin/HEAD"]),
        repository.head()
    );
}
