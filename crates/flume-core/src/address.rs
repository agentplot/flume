//! A repository's address, `<owner>/<name>` as its clone's `origin` gives
//! it on its forge, by which the host's configuration names it (143,
//! C.7; record 0055). The URL is read here and dropped: only the address
//! goes on, since a URL may carry a user and a token (56; core.30).

/// The schemes of a URL that names a host (design 1).
const SCHEMES: [&str; 6] = ["ssh", "git+ssh", "ssh+git", "git", "http", "https"];

/// The address an origin's URL gives, `<owner>/<name>`, or none.
///
/// The URL names a host when it is scp-like, `[<user>@]<host>:<path>`
/// with no `://` and no `/` before the first `:`, or
/// `<scheme>://[<user>[:<password>]@]<host>[:<port>]/<path>` by one of
/// [`SCHEMES`]. The address is its path with a trailing `/` and then one
/// trailing `.git` dropped, where that is exactly two segments, neither
/// empty, `.` or `..` nor beginning with `~`, its case kept. A path on
/// the host, a `file://` URL, any other scheme, a remote helper's
/// `<transport>::<address>` and a path of one segment or of three or
/// more give none (143).
pub fn of_origin(url: &str) -> Option<String> {
    let path = match url.split_once("://") {
        Some((scheme, rest)) => {
            if !SCHEMES.contains(&scheme) {
                return None;
            }
            let (authority, path) = rest.split_once('/')?;
            let host = authority
                .rsplit_once('@')
                .map_or(authority, |(_, host)| host);
            if without_port(host).is_empty() {
                return None;
            }
            path
        }
        None => {
            let (host, path) = scp_like(url)?;
            if host.is_empty() {
                return None;
            }
            path
        }
    };
    let path = path.strip_suffix('/').unwrap_or(path);
    let path = path.strip_suffix(".git").unwrap_or(path);
    let (owner, name) = path.split_once('/')?;
    [owner, name]
        .iter()
        .all(|segment| is_segment(segment))
        .then(|| format!("{owner}/{name}"))
}

/// An scp-like URL's host and path: `[<user>@]<host>:<path>`, a host in
/// brackets holding its own `:`, with no `/` before the `:` that ends the
/// host.
fn scp_like(url: &str) -> Option<(&str, &str)> {
    let (user, after_user) = match url.split_once('@') {
        Some((user, rest)) if !user.contains(['/', ':']) => (Some(user), rest),
        _ => (None, url),
    };
    let (host, path) = match after_user.strip_prefix('[') {
        Some(bracketed) => {
            let (host, rest) = bracketed.split_once(']')?;
            (host, rest.strip_prefix(':')?)
        }
        None => after_user.split_once(':')?,
    };
    if host.contains('/') || user.is_some_and(str::is_empty) {
        return None;
    }
    Some((host, path))
}

/// A URL's host with its `:<port>` dropped, a host in brackets kept whole.
fn without_port(host: &str) -> &str {
    match host.strip_prefix('[') {
        Some(bracketed) => bracketed.split_once(']').map_or("", |(host, _)| host),
        None => host.split_once(':').map_or(host, |(host, _)| host),
    }
}

/// One segment of an address: not empty, `.` or `..`, not beginning with
/// `~`, and holding no `/`.
fn is_segment(segment: &str) -> bool {
    !segment.is_empty()
        && segment != "."
        && segment != ".."
        && !segment.starts_with('~')
        && !segment.contains('/')
}

#[cfg(test)]
mod tests {
    use super::of_origin;

    #[test]
    fn an_origin_on_a_forge_gives_its_address() {
        // The launching spec's "Each form of a forge origin" (143).
        for url in [
            "git@github.com:acme/acme.git",
            "https://github.com/acme/acme",
            "https://github.com/acme/acme.git/",
            "ssh://git@github.com:22/acme/acme.git",
            "git://git.example.org/acme/acme.git",
            "https://user:token@git.example.org/acme/acme.git",
            "git+ssh://git@github.com/acme/acme.git",
            "ssh+git://github.com/acme/acme",
            "http://git.example.org:8080/acme/acme.git",
            "github.com:acme/acme",
            "git@[::1]:acme/acme.git",
            "ssh://git@[::1]:22/acme/acme.git",
        ] {
            assert_eq!(of_origin(url).as_deref(), Some("acme/acme"), "{url}");
        }
        assert_eq!(
            of_origin("git@github.com:Acme/Acme.git").as_deref(),
            Some("Acme/Acme"),
            "its case kept"
        );
        assert_eq!(
            of_origin("https://github.com/acme/acme.git.git").as_deref(),
            Some("acme/acme.git"),
            "one trailing .git dropped"
        );
    }

    #[test]
    fn an_origin_on_no_forge_gives_none() {
        // The launching spec's "No forge origin" (143).
        for url in [
            "/srv/git/notes.git",
            "../notes",
            "file:///srv/git/notes.git",
            "codecommit::us-east-1://notes",
            "https://github.com/acme",
            "https://gitlab.com/group/sub/name.git",
            "git@host:~me/notes.git",
            "",
            "notes",
            "./acme/acme",
            "ftp://git.example.org/acme/acme.git",
            "https:///acme/acme.git",
            "https://user@/acme/acme.git",
            ":acme/acme.git",
            "@host:acme/acme.git",
            "git@host:./acme",
            "git@host:../acme",
            "git@host:acme/~acme",
            "https://github.com//acme/acme",
            "https://github.com/acme/",
            "https://github.com/acme/acme//",
            "https://github.com",
            "./host:acme/acme",
        ] {
            assert_eq!(of_origin(url), None, "{url}");
        }
    }
}
