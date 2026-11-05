//! An account's reading as the host keeps it (184, C.7): one JSON
//! document in the file its `usage` names, written whole by something of
//! the host's other than flume, which flume reads and never writes. Its
//! `read_at` and each window's `resets_at` are RFC 3339 times with their
//! offset, each window's `used` a percentage from 0 to 100, kept in
//! hundredths; every key the form does not know is ignored. A file that
//! is missing or not in the form is no reading, its words naming no path
//! (core.30).

use chrono::DateTime;
use flume_seams::{Seconds, Usage, Window};
use serde_json::{Map, Value};
use std::io::ErrorKind;
use std::path::Path;

/// The reading in the file at `path`, or why there is none (184).
pub fn read(path: &Path) -> Result<Usage, String> {
    match std::fs::read(path) {
        Ok(bytes) => parse(&bytes),
        Err(error) if error.kind() == ErrorKind::NotFound => Err("the file is missing".into()),
        Err(error) => Err(format!("the file cannot be read: {error}")),
    }
}

/// The reading `bytes` hold in C.7's form, or why they hold none (184).
pub fn parse(bytes: &[u8]) -> Result<Usage, String> {
    let document: Value =
        serde_json::from_slice(bytes).map_err(|error| format!("the file is not JSON: {error}"))?;
    let document = document
        .as_object()
        .ok_or("the file is not a JSON object".to_string())?;
    let read_at = match document.get("read_at") {
        None => return Err("the reading names no read_at".into()),
        Some(time) => time_of(time, "read_at")?,
    };
    let windows = match document.get("windows") {
        None => return Err("the reading names no windows".into()),
        Some(Value::Array(windows)) => windows,
        Some(_) => return Err("windows is not a list".into()),
    };
    let windows = windows
        .iter()
        .enumerate()
        .map(|(n, window)| {
            let at = format!("windows[{n}]");
            let window = window.as_object().ok_or(format!("{at} is not an object"))?;
            window_of(window, &at)
        })
        .collect::<Result<_, String>>()?;
    Ok(Usage { read_at, windows })
}

/// One window of the list, `at` naming its place in it.
fn window_of(window: &Map<String, Value>, at: &str) -> Result<Window, String> {
    let name = match window.get("name") {
        None => return Err(format!("{at} names no name")),
        Some(Value::String(name)) => name.clone(),
        Some(_) => return Err(format!("{at} name is not a string")),
    };
    let used = match window.get("used") {
        None => return Err(format!("{at} names no used")),
        Some(used) => used
            .as_f64()
            .filter(|used| (0.0..=100.0).contains(used))
            .ok_or(format!("{at} used is not a number from 0 to 100"))?,
    };
    let resets_at = window
        .get("resets_at")
        .map(|time| time_of(time, &format!("{at} resets_at")))
        .transpose()?;
    Ok(Window {
        name,
        used: (used * 100.0).round() as u32,
        resets_at,
    })
}

/// An RFC 3339 time with its offset, by the host's clock, `key` naming it
/// where it is not one.
fn time_of(time: &Value, key: &str) -> Result<Seconds, String> {
    time.as_str()
        .and_then(|time| DateTime::parse_from_rfc3339(time).ok())
        .and_then(|time| Seconds::try_from(time.timestamp()).ok())
        .ok_or(format!("{key} is not an RFC 3339 time with its offset"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2026-11-05T14:02:11Z and 2026-11-05T17:00:00Z by the host's
    /// clock.
    const READ_AT: Seconds = 1_793_887_331;
    const FIVE_HOUR: Seconds = 1_793_898_000;

    /// The store-profile spec's form read: C.7's example with a `used` of
    /// 71.5, a window with no `resets_at`, and a key `login` at the top
    /// and `kind` in a window, both ignored (184, C.7).
    #[test]
    fn the_form_read() {
        let file = br#"{
            "read_at": "2026-11-05T14:02:11Z",
            "login": "fine",
            "windows": [
                { "name": "5-hour", "used": 42, "resets_at": "2026-11-05T17:00:00Z", "kind": "all" },
                { "name": "7-day", "used": 71.5 }
            ]
        }"#;
        assert_eq!(
            parse(file),
            Ok(Usage {
                read_at: READ_AT,
                windows: vec![
                    Window {
                        name: "5-hour".into(),
                        used: 4_200,
                        resets_at: Some(FIVE_HOUR),
                    },
                    Window {
                        name: "7-day".into(),
                        used: 7_150,
                        resets_at: None,
                    },
                ],
            })
        );
    }

    /// A time with another offset reads as the same moment, and a reading
    /// of no windows is a reading.
    #[test]
    fn an_offset_and_no_windows() {
        assert_eq!(
            parse(br#"{ "read_at": "2026-11-05T15:02:11+01:00", "windows": [] }"#),
            Ok(Usage {
                read_at: READ_AT,
                windows: vec![],
            })
        );
    }

    /// The store-profile spec's missing file: its words name no path
    /// (184).
    #[test]
    fn a_missing_file() {
        let path = std::env::temp_dir().join(format!(
            "flume-usage-{}-missing/work-1.json",
            std::process::id()
        ));
        let read = read(&path);
        assert_eq!(read, Err("the file is missing".to_string()));
    }

    /// The store-profile spec's document not in the form: each refusal
    /// names the key, or the window by its place in the list (184, C.7).
    #[test]
    fn a_document_not_in_the_form() {
        let refused = |file: &str| parse(file.as_bytes()).expect_err(file);
        assert!(
            refused("not json").starts_with("the file is not JSON: "),
            "{}",
            refused("not json")
        );
        for (file, words) in [
            ("[1, 2]", "the file is not a JSON object"),
            (r#"{ "windows": [] }"#, "the reading names no read_at"),
            (
                r#"{ "read_at": "2026-11-05T14:02:11", "windows": [] }"#,
                "read_at is not an RFC 3339 time with its offset",
            ),
            (
                r#"{ "read_at": 1793887331, "windows": [] }"#,
                "read_at is not an RFC 3339 time with its offset",
            ),
            (
                r#"{ "read_at": "2026-11-05T14:02:11Z" }"#,
                "the reading names no windows",
            ),
            (
                r#"{ "read_at": "2026-11-05T14:02:11Z", "windows": {} }"#,
                "windows is not a list",
            ),
            (
                r#"{ "read_at": "2026-11-05T14:02:11Z", "windows": [
                    { "name": "5-hour", "used": 42 },
                    { "name": "7-day", "used": 120 }
                ] }"#,
                "windows[1] used is not a number from 0 to 100",
            ),
            (
                r#"{ "read_at": "2026-11-05T14:02:11Z", "windows": [{ "name": "5-hour", "used": -1 }] }"#,
                "windows[0] used is not a number from 0 to 100",
            ),
            (
                r#"{ "read_at": "2026-11-05T14:02:11Z", "windows": [{ "name": "5-hour", "used": "42%" }] }"#,
                "windows[0] used is not a number from 0 to 100",
            ),
            (
                r#"{ "read_at": "2026-11-05T14:02:11Z", "windows": [{ "name": "5-hour" }] }"#,
                "windows[0] names no used",
            ),
            (
                r#"{ "read_at": "2026-11-05T14:02:11Z", "windows": [{ "used": 42 }] }"#,
                "windows[0] names no name",
            ),
            (
                r#"{ "read_at": "2026-11-05T14:02:11Z", "windows": [{ "name": 5, "used": 42 }] }"#,
                "windows[0] name is not a string",
            ),
            (
                r#"{ "read_at": "2026-11-05T14:02:11Z", "windows": [7] }"#,
                "windows[0] is not an object",
            ),
            (
                r#"{ "read_at": "2026-11-05T14:02:11Z", "windows": [
                    { "name": "5-hour", "used": 42, "resets_at": "at five" }
                ] }"#,
                "windows[0] resets_at is not an RFC 3339 time with its offset",
            ),
        ] {
            assert_eq!(refused(file), words, "{file}");
        }
    }
}
