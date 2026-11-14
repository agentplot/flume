//! The usage windows Claude Code reports (171, 184, 211, C.4): the
//! engine hands its own sessions' plugins the windows each response
//! reports, in its `session.measure` event, and flume's plugin module
//! (`hooks/usage.ts`) passes each report whose `rateLimits` moved and
//! holds a window to `flume hook usage` as `{"rateLimits": […]}`. The hook
//! reads the report here, keeps it beside the start's record with the
//! time it ran ([`Record::keep_windows`]), and has the core take it as the
//! account's reading; the profile's `windows` reads it back. Every field
//! is checked against the engine types the pinned Claude Code 2.1.296
//! writes, `plugins/claude/engine-types/claude-code/index.d.ts`:
//! `SessionRateLimit`'s `kind`, `percentUsed`, from 0 to 100 and past it
//! for an exceeded spend limit, and `resetsAt`, ISO 8601.

use super::record::{KeptWindows, Record};
use flume_seams::{rfc3339_read, Identity, UsageWindow, Windows};
use serde_json::Value;

/// The word `flume hook` takes for it, which the plugin's module runs,
/// not a hook event of Claude Code's.
pub const EVENT: &str = "usage";

/// The windows of the engine's report `input`, `{"rateLimits": […]}`:
/// each window named by its `kind`, a non-empty string, its use its
/// `percentUsed`, a number of 0 or more, in hundredths, a use past 100 read
/// as 100, and its reset its `resetsAt` where given, read by RFC 3339.
/// None where the input gives no list, an empty one, or a window in no
/// such form: a report with a window left out could read an account under
/// its threshold, and an empty one would replace a reading kept with
/// nothing (184, 211, C.4; record 0062's amendment).
pub fn report(input: &str) -> Option<Vec<UsageWindow>> {
    let input: Value = serde_json::from_str(input).ok()?;
    let given = input.get("rateLimits")?.as_array()?;
    if given.is_empty() {
        return None;
    }
    given.iter().map(window).collect()
}

/// The engine's report `input` kept beside the start's `record`, taken
/// at `taken_at` by the host's clock, as `flume hook usage` keeps it: false
/// where it is no report [`report`] reads, the file could not be written,
/// or a report taken later stands there (184, 211).
pub fn keep(record: &Record, input: &str, taken_at: u64) -> bool {
    let Some(windows) = report(input) else {
        return false;
    };
    record
        .keep_windows(&KeptWindows { taken_at, windows })
        .unwrap_or(false)
}

/// One window of the engine's report, none where it is in no form
/// [`report`] reads.
fn window(given: &Value) -> Option<UsageWindow> {
    let name = given
        .get("kind")?
        .as_str()
        .filter(|kind| !kind.is_empty())?;
    let percent = given
        .get("percentUsed")?
        .as_f64()
        .filter(|percent| *percent >= 0.0)?;
    let resets_at = match given.get("resetsAt") {
        None | Some(Value::Null) => None,
        Some(written) => Some(rfc3339_read(written.as_str()?)?),
    };
    Some(UsageWindow {
        name: name.to_string(),
        used: (percent.min(100.0) * 100.0).round() as u32,
        resets_at,
    })
}

/// The windows the program of `agent` last reported, as `flume hook
/// usage` kept them in its start's record in `folder`, with the session
/// the start's started ack names; none yet where the record is readable
/// and holds none; unknown where the folder does not stand or the file
/// kept cannot be read (128, 211).
pub fn windows(folder: &str, agent: &Identity) -> Windows {
    let record = Record::at(folder, agent);
    if !record.readable() {
        return Windows::Unknown;
    }
    match record.windows() {
        Ok(Some(KeptWindows { taken_at, windows })) => Windows::Reported {
            taken_at,
            windows,
            session: record
                .started()
                .map(|started| started.session_id)
                .filter(|session| !session.is_empty()),
        },
        Ok(None) => Windows::NoneYet,
        Err(_) => Windows::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::agent::claude::record::Started;

    /// The agent-profile spec's scenario "The engine's form read" (211,
    /// C.4).
    #[test]
    fn the_engines_form_read() {
        let input = r#"{"rateLimits": [
            {"kind": "five_hour", "percentUsed": 23.5, "resetsAt": "2026-11-05T17:00:00.000Z"},
            {"kind": "seven_day", "percentUsed": 7},
            {"kind": "spend_limit", "percentUsed": 112}
        ]}"#;
        assert_eq!(
            report(input),
            Some(vec![
                UsageWindow {
                    name: "five_hour".into(),
                    used: 2_350,
                    resets_at: Some(1_793_898_000),
                },
                UsageWindow {
                    name: "seven_day".into(),
                    used: 700,
                    resets_at: None,
                },
                UsageWindow {
                    name: "spend_limit".into(),
                    used: 10_000,
                    resets_at: None,
                },
            ])
        );
        assert_eq!(
            report(
                r#"{"rateLimits": [{"kind": "five_hour", "percentUsed": 0, "resetsAt": null}]}"#
            ),
            Some(vec![UsageWindow {
                name: "five_hour".into(),
                used: 0,
                resets_at: None,
            }]),
            "a window of no use, its reset left null"
        );
        assert_eq!(
            report(
                r#"{"rateLimits": [{"kind": "five_hour", "percentUsed": 42, "resetsAt": "2026-11-05T12:00:00-05:00"}]}"#
            )
            .map(|windows| windows[0].resets_at),
            Some(Some(1_793_898_000)),
            "a reset with its offset"
        );
    }

    /// Each window in no form the profile reads, and a report with no
    /// list, keeps nothing (184, 211).
    #[test]
    fn a_window_in_no_such_form_is_no_report() {
        let good = r#"{"kind": "seven_day", "percentUsed": 18}"#;
        for bad in [
            r#"{"percentUsed": 42}"#,
            r#"{"kind": "", "percentUsed": 42}"#,
            r#"{"kind": 5, "percentUsed": 42}"#,
            r#"{"kind": "five_hour"}"#,
            r#"{"kind": "five_hour", "percentUsed": "42%"}"#,
            r#"{"kind": "five_hour", "percentUsed": -1}"#,
            r#"{"kind": "five_hour", "percentUsed": 42, "resetsAt": "at five"}"#,
            r#"{"kind": "five_hour", "percentUsed": 42, "resetsAt": 1793898000}"#,
            r#""five_hour""#,
        ] {
            let input = format!(r#"{{"rateLimits": [{good}, {bad}]}}"#);
            assert_eq!(report(&input), None, "{input}");
        }
        for input in [
            "",
            "half a {",
            "{}",
            r#"{"rateLimits": null}"#,
            r#"{"rateLimits": {"kind": "five_hour", "percentUsed": 42}}"#,
            r#"{"rateLimits": []}"#,
        ] {
            assert_eq!(report(input), None, "{input}");
        }
    }

    fn scratch() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "flume-windows-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |since| since.as_nanos())
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn agent() -> Identity {
        Identity {
            repository: "r".into(),
            place: "fix--one".into(),
            stage: "code".into(),
            start: 1,
        }
    }

    /// The windows kept beside the start's record and read back with the
    /// session its started ack names, a later report standing, none yet
    /// before any, and unknown in a folder that does not stand or where
    /// the file cannot be read (128, 211).
    #[test]
    fn the_windows_kept_and_read_back() {
        let dir = scratch();
        let folder = dir.to_string_lossy().to_string();
        let record = Record::at(&folder, &agent());
        assert_eq!(windows(&folder, &agent()), Windows::NoneYet);
        let window = |used| UsageWindow {
            name: "five_hour".into(),
            used,
            resets_at: None,
        };
        let kept = |taken_at, used| KeptWindows {
            taken_at,
            windows: vec![window(used)],
        };
        assert!(record.keep_windows(&kept(1_000, 4_200)).unwrap());
        assert_eq!(
            windows(&folder, &agent()),
            Windows::Reported {
                taken_at: 1_000,
                windows: vec![window(4_200)],
                session: None,
            }
        );
        record
            .ack_started(&Started {
                session_id: "s-1".into(),
                ..Started::default()
            })
            .unwrap();
        assert!(record.keep_windows(&kept(1_180, 5_000)).unwrap());
        assert!(!record.keep_windows(&kept(1_060, 4_600)).unwrap());
        assert_eq!(
            windows(&folder, &agent()),
            Windows::Reported {
                taken_at: 1_180,
                windows: vec![window(5_000)],
                session: Some("s-1".into()),
            }
        );
        assert_eq!(
            windows(&format!("{folder}-of-another-place"), &agent()),
            Windows::Unknown
        );
        std::fs::write(record.dir().join("windows.json"), "half a {").unwrap();
        assert_eq!(windows(&folder, &agent()), Windows::Unknown);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
