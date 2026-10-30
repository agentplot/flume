//! A start's record of what its agent is to run under (161; core.10,
//! core.30): the program its settings resolved, its account, and each
//! variable's digest keyed by the start's secret, never a value. The
//! agent program's own word that it started is compared with it, and an
//! agent that began otherwise is not the stage's.

use flume_seams::{AccountHeld, Identity, Secret, StartRecord};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::BTreeMap;

/// The variable naming an agent's start, its identity (33).
pub const AGENT_VARIABLE: &str = "FLUME_AGENT";

/// The variable naming the place an agent works in, its location.
pub const PLACE_VARIABLE: &str = "FLUME_PLACE";

/// The variable naming the stage an agent works.
pub const STAGE_VARIABLE: &str = "FLUME_STAGE";

/// The variables a terminal sets for its own sake, which no start's
/// record holds whatever gives them (record 0018).
pub const TERMINALS_OWN: [&str; 2] = ["PATH", "SHELL"];

/// A variable's digest for a start: HMAC-SHA256 keyed by the start's
/// secret over the variable's name, a zero byte and its value, in hex, so
/// no value is kept and no guessed value is tested without the secret
/// (161; core.30).
pub fn variable_digest(secret: &Secret, name: &str, value: &str) -> String {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.0.as_bytes()).expect("HMAC takes a key of any size");
    mac.update(name.as_bytes());
    mac.update(&[0]);
    mac.update(value.as_bytes());
    mac.finalize()
        .into_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// The record of a start of `program` under `account`, whose environment
/// is `env`, each value read against the login environment, for
/// `identity` in the place at `location`, keyed by `secret`; the
/// variables the program keeps from the processes it starts, `withheld`,
/// left out with the terminal's own (161).
pub(crate) fn start_record(
    program: &str,
    account: Option<&AccountHeld>,
    env: &BTreeMap<String, String>,
    withheld: &[String],
    identity: &Identity,
    location: &str,
    secret: &Secret,
) -> StartRecord {
    let identity_text = identity.to_string();
    let flumes = [
        (AGENT_VARIABLE, identity_text.as_str()),
        (PLACE_VARIABLE, location),
        (STAGE_VARIABLE, identity.stage.as_str()),
    ];
    let env = env
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_str()))
        .filter(|(name, _)| {
            !TERMINALS_OWN.contains(name) && !withheld.iter().any(|kept| kept == name)
        })
        .chain(flumes)
        .map(|(name, value)| (name.to_string(), variable_digest(secret, name, value)))
        .collect();
    StartRecord {
        program: program.to_string(),
        account: account.map(|account| account.name.clone()),
        env,
        mode: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity() -> Identity {
        Identity {
            repository: "r1".into(),
            place: "fix--one".into(),
            stage: "code".into(),
            start: 1,
        }
    }

    #[test]
    fn a_digest_is_keyed_by_the_secret_and_names_its_variable() {
        let one = Secret("one".into());
        let digest = variable_digest(&one, "HOME_DIR", "/s/work-1");
        assert_eq!(digest.len(), 64);
        assert_ne!(
            digest,
            variable_digest(&Secret("two".into()), "HOME_DIR", "/s/work-1")
        );
        assert_ne!(digest, variable_digest(&one, "OTHER_DIR", "/s/work-1"));
        assert_ne!(digest, variable_digest(&one, "HOME_DIR", "/s/work-2"));
        assert_ne!(
            variable_digest(&one, "A", "BC"),
            variable_digest(&one, "AB", "C"),
            "the zero byte parts the name from the value"
        );
    }

    #[test]
    fn the_record_covers_the_environment_and_flumes_variables_and_no_terminals_or_withheld() {
        let secret = Secret("s".into());
        let env = BTreeMap::from([
            ("HOME_DIR".to_string(), "/s/work-1".to_string()),
            ("PATH".to_string(), "/bin".to_string()),
            ("SHELL".to_string(), "/bin/sh".to_string()),
            ("AGENT_TOKEN".to_string(), "t".to_string()),
        ]);
        let record = start_record(
            "agent",
            None,
            &env,
            &["AGENT_TOKEN".to_string()],
            &identity(),
            "/r/places/fix--one",
            &secret,
        );
        assert_eq!(
            record.env.keys().collect::<Vec<_>>(),
            vec!["FLUME_AGENT", "FLUME_PLACE", "FLUME_STAGE", "HOME_DIR"]
        );
        assert_eq!(
            record.env["FLUME_PLACE"],
            variable_digest(&secret, "FLUME_PLACE", "/r/places/fix--one")
        );
        assert_eq!(
            record.env["FLUME_AGENT"],
            variable_digest(&secret, "FLUME_AGENT", "flume:r1:fix--one:code:1")
        );
        assert_eq!(record.program, "agent");
        assert_eq!(record.account, None);
    }
}
