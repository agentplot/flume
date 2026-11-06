//! Claude Code's own settings (195, 196, C.4; design 6, 9): the keys of
//! 2.1.296's settings another of flume's settings, the account or a
//! person owns, each by its path with its owner, which the core refuses
//! where a table's `settings` gives one and leaves out of a named file;
//! and the file the command lists the agent's resolved keys in, one JSON
//! object at `own-settings.json` beside the start's record, which
//! `--settings` names and the release lays over the user's, the
//! project's and the local settings. Every other key is taken as written,
//! `attribution` among them: nothing of flume's is added to a value or
//! read in it. Every fragment of the release this rests on is in
//! [`fragments`], which a test of the check-tools layer finds byte for
//! byte in the release the tree pins.

use super::record::Record;
use flume_seams::{Identity, Reserved};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// The option that names the file the resolved keys are written in.
pub const FLAG: &str = "--settings";

/// The file's name, in the start's record's folder.
pub const FILE: &str = "own-settings.json";

/// The account's environment, which is an agent's whole environment (29,
/// 144; core.30).
const ACCOUNT: &str = "the account's `env`";

/// The account's sign-in, which its configuration home holds (144).
const SIGN_IN: &str = "the account's sign-in";

/// A person's acceptance of a warning before a mode that widens what an
/// agent may do, which flume never gives (34; core.12).
const PERSON: &str = "a person, who alone accepts the warning it records";

/// Each key 2.1.296's settings hold that the profile reserves, by its
/// path, with its owner and its declaration in the release's settings
/// schema past its name, through the opening of its description: the
/// account's environment, flume's model and mode, the account's
/// credential helpers and sign-in, the hooks of flume's plugin, which the
/// start's ack, the guard and the stop check are read from, and the two
/// acceptances of a person's (35, 144, 162, 172; design 6).
const RESERVED: [(&str, &str, &str); 13] = [
    (
        "env",
        ACCOUNT,
        "()=>Ae(()=>Lg()).describe(\"Environment variables to set for Claude Code sessions\")",
    ),
    (
        "model",
        "flume's `model`",
        "()=>o().optional().describe(\"Override the default model used by Claude Code\")",
    ),
    (
        "permissions.defaultMode",
        "flume's `mode`",
        "pa(rh,U([...CH,...Ho(e)])).optional().describe(\"Default permission mode when Claude Code needs access",
    ),
    (
        "apiKeyHelper",
        SIGN_IN,
        "()=>o().optional().describe(\"Path to a script that outputs authentication values\")",
    ),
    (
        "awsAuthRefresh",
        SIGN_IN,
        "()=>o().optional().describe(\"Path to a script that refreshes AWS authentication\")",
    ),
    (
        "awsCredentialExport",
        SIGN_IN,
        "()=>o().optional().describe(\"Path to a script that exports AWS credentials\")",
    ),
    (
        "gcpAuthRefresh",
        SIGN_IN,
        "()=>o().optional().describe(\"Command to refresh GCP authentication",
    ),
    (
        "otelHeadersHelper",
        SIGN_IN,
        "()=>o().optional().describe(\"Path to a script that outputs OpenTelemetry headers\")",
    ),
    (
        "forceLoginMethod",
        SIGN_IN,
        "()=>U([\"claudeai\",\"console\",\"gateway\"]).optional().catch(void 0).describe('Force a specific login method",
    ),
    (
        "forceLoginOrgUUID",
        SIGN_IN,
        "()=>Ue([o(),A(o())]).optional().describe(\"Organization UUID to require for OAuth login.",
    ),
    (
        "disableAllHooks",
        "flume's plugin",
        "()=>H().optional().describe(\"Disable all hooks and statusLine execution",
    ),
    (
        "skipDangerousModePermissionPrompt",
        PERSON,
        "()=>H().optional().describe(\"Whether the user has accepted the bypass permissions mode dialog\")",
    ),
    (
        "skipAutoPermissionPrompt",
        PERSON,
        "H().optional().describe(\"Whether the user has accepted the auto mode opt-in dialog\")",
    ),
];

/// The keys the profile reserves, by their paths, each with its owner
/// (195, 196; design 6).
pub fn reserved() -> Vec<Reserved> {
    RESERVED
        .iter()
        .map(|(key, owner, _)| Reserved {
            key: key.to_string(),
            owner: owner.to_string(),
        })
        .collect()
}

/// The file the start `identity` names in `folder` is given its resolved
/// keys by: `<folder>/claude/<identity>/own-settings.json`, beside its
/// record (128).
pub fn file(folder: &str, identity: &Identity) -> PathBuf {
    Record::at(folder, identity).dir().join(FILE)
}

/// The file's text: `own`, the agent's resolved keys, as one JSON object,
/// its keys in order and each value as the layers wrote it.
pub fn text(own: &BTreeMap<String, Value>) -> String {
    let mut text = serde_json::to_string(own).expect("own settings as JSON");
    text.push('\n');
    text
}

/// The source fragments of 2.1.296 the way rests on, each named and as
/// the release's program holds it: the option of one value that names a
/// settings file or JSON, its sources' order with the flag's after the
/// user's, the project's and the local settings, the merge that joins a
/// source's lists with those beneath it and takes the later source's
/// value otherwise, each reserved key as its settings schema declares it,
/// and the `attribution` schema's commit and pull request texts, each
/// saying an empty string hides attribution. `attribution` is no reserved
/// key: its texts pin the docs' worked example, not a key the profile
/// treats apart (design 9, 16).
pub fn fragments() -> Vec<(&'static str, String)> {
    let mut fragments = vec![
        (
            "the option that names the settings",
            format!("\"{FLAG} <file-or-json>\",\"Path to a settings JSON file or a JSON string to load additional settings from\""),
        ),
        (
            "the sources' order, the flag's after the user's, the project's and the local",
            "return[\"userSettings\",\"projectSettings\",\"localSettings\",\"flagSettings\",\"policySettings\"]".into(),
        ),
        (
            "the merge joining a source's lists with those beneath it",
            "if(Array.isArray(e)&&Array.isArray(n)){if(s===\"fallbackModel\")return n;return D([...e,...n])}".into(),
        ),
    ];
    fragments.extend(RESERVED.iter().map(|(key, _, schema)| {
        let name = key.rsplit('.').next().expect("a key");
        (
            "a reserved key as the settings schema declares it",
            format!("{name}:{schema}"),
        )
    }));
    fragments.extend([
        (
            "the attribution of a commit",
            "Attribution text for git commits, including any trailers. Empty string hides attribution.".to_string(),
        ),
        (
            "the attribution of a pull request",
            "Attribution text for pull request descriptions. Empty string hides attribution.".into(),
        ),
    ]);
    fragments
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn the_fragments_are_written_as_the_release_writes_them() {
        let fragments: Vec<String> = fragments().into_iter().map(|(_, text)| text).collect();
        assert_eq!(
            fragments,
            [
                r#""--settings <file-or-json>","Path to a settings JSON file or a JSON string to load additional settings from""#,
                r#"return["userSettings","projectSettings","localSettings","flagSettings","policySettings"]"#,
                r#"if(Array.isArray(e)&&Array.isArray(n)){if(s==="fallbackModel")return n;return D([...e,...n])}"#,
                r#"env:()=>Ae(()=>Lg()).describe("Environment variables to set for Claude Code sessions")"#,
                r#"model:()=>o().optional().describe("Override the default model used by Claude Code")"#,
                r#"defaultMode:pa(rh,U([...CH,...Ho(e)])).optional().describe("Default permission mode when Claude Code needs access"#,
                r#"apiKeyHelper:()=>o().optional().describe("Path to a script that outputs authentication values")"#,
                r#"awsAuthRefresh:()=>o().optional().describe("Path to a script that refreshes AWS authentication")"#,
                r#"awsCredentialExport:()=>o().optional().describe("Path to a script that exports AWS credentials")"#,
                r#"gcpAuthRefresh:()=>o().optional().describe("Command to refresh GCP authentication"#,
                r#"otelHeadersHelper:()=>o().optional().describe("Path to a script that outputs OpenTelemetry headers")"#,
                r#"forceLoginMethod:()=>U(["claudeai","console","gateway"]).optional().catch(void 0).describe('Force a specific login method"#,
                r#"forceLoginOrgUUID:()=>Ue([o(),A(o())]).optional().describe("Organization UUID to require for OAuth login."#,
                r#"disableAllHooks:()=>H().optional().describe("Disable all hooks and statusLine execution"#,
                r#"skipDangerousModePermissionPrompt:()=>H().optional().describe("Whether the user has accepted the bypass permissions mode dialog")"#,
                r#"skipAutoPermissionPrompt:H().optional().describe("Whether the user has accepted the auto mode opt-in dialog")"#,
                "Attribution text for git commits, including any trailers. Empty string hides attribution.",
                "Attribution text for pull request descriptions. Empty string hides attribution.",
            ]
        );
    }

    #[test]
    fn the_file_is_beside_the_starts_record() {
        let identity = Identity {
            repository: "/repository".into(),
            place: "fix--one".into(),
            stage: "code".into(),
            start: 2,
        };
        assert_eq!(
            file("/state/places/fix--one", &identity),
            Record::at("/state/places/fix--one", &identity)
                .dir()
                .join("own-settings.json")
        );
    }

    #[test]
    fn the_text_is_one_json_object_of_the_keys_as_given() {
        let own = BTreeMap::from([
            ("tui".to_string(), json!("fullscreen")),
            (
                "attribution".to_string(),
                json!({ "commit": "", "pr": "", "sessionUrl": false }),
            ),
        ]);
        assert_eq!(
            text(&own),
            "{\"attribution\":{\"commit\":\"\",\"pr\":\"\",\"sessionUrl\":false},\"tui\":\"fullscreen\"}\n"
        );
    }
}
