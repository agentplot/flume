//! `flume hook status-line` (design 5): the line of the place an agent
//! flume started works in, which the plugin's module (`hooks/flume.ts`)
//! pins under the prompt with `$.ui.status` at the session's events and
//! on its clock. It is the core's line asked alone, its full form, the
//! words the core's alone (core.14); it reads no input, writes nothing,
//! settles nothing and starts no agent (57; core.16), and, like every
//! hook, it always exits 0 and does nothing outside an agent flume
//! started (C.4).

use flume_core::Engine;
use flume_seams::{Identity, PlaceId};

/// The word `flume hook` takes for it, which the plugin's module runs,
/// not a hook event of Claude Code's.
pub const EVENT: &str = "status-line";

/// The full line of the place `place` names, asked alone for the session
/// `agent` names; nothing where either is unset, `agent` is not an
/// identity flume gives, the core cannot be built, or no place it names
/// stands on the host (56, 57, 60).
pub fn line(
    place: Option<&str>,
    agent: Option<&str>,
    build: &dyn Fn() -> Result<Engine, String>,
) -> String {
    let (Some(place), Some(identity)) = (place, agent.and_then(identity)) else {
        return String::new();
    };
    let Ok(core) = build() else {
        return String::new();
    };
    core.line(&PlaceId::new(place), Some(&identity))
        .map(|line| line.full)
        .unwrap_or_default()
}

/// The identity `word` gives as flume spells one,
/// `flume:<repository>:<place>:<stage>:<start>`, read back only where it
/// spells the same again (33).
fn identity(word: &str) -> Option<Identity> {
    let (repository, rest) = word.strip_prefix("flume:")?.split_once(':')?;
    let (rest, start) = rest.rsplit_once(':')?;
    let (place, stage) = rest.rsplit_once(':')?;
    let identity = Identity {
        repository: repository.into(),
        place: place.into(),
        stage: stage.into(),
        start: start.parse().ok()?,
    };
    (identity.to_string() == word).then_some(identity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_identity_read_back_as_flume_spells_it() {
        let identity = Identity {
            repository: "b3e4c15d5ebe".into(),
            place: "fix--one".into(),
            stage: "code".into(),
            start: 2,
        };
        assert_eq!(super::identity(&identity.to_string()), Some(identity));
        for word in [
            "",
            "fix--one",
            "flume:b3e4c15d5ebe:fix--one:code",
            "flume:b3e4c15d5ebe:fix--one:code:two",
            "flume:b3e4c15d5ebe:fix--one:code:+2",
            "other:b3e4c15d5ebe:fix--one:code:2",
        ] {
            assert_eq!(super::identity(word), None, "{word}");
        }
    }

    #[test]
    fn nothing_outside_an_agent_flume_started() {
        let built = std::cell::Cell::new(false);
        let build = || -> Result<Engine, String> {
            built.set(true);
            Err("no core".into())
        };
        let agent = "flume:b3e4c15d5ebe:fix--one:code:1";
        assert_eq!(line(None, None, &build), "");
        assert_eq!(line(Some("/places/fix--one"), None, &build), "");
        assert_eq!(line(None, Some(agent), &build), "");
        assert_eq!(line(Some("/places/fix--one"), Some("an-agent"), &build), "");
        assert!(!built.get(), "the core was built outside an agent");
        assert_eq!(line(Some("/places/fix--one"), Some(agent), &build), "");
        assert!(built.get());
    }
}
