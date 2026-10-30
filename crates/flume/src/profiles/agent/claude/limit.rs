//! A usage limit read from the words Claude Code 2.1.296 writes for one
//! (151, 146, C.4): the failure's `error` is `rate_limit` and its words
//! open as one of the usage-limit messages that release itself
//! recognizes, its own list of their openings; the reset is the words'
//! ` · resets <when> (<zone>)` part, wherever it falls among their ` · `
//! parts, `when` as that release's reset formatter writes it in en-US and
//! `zone` the IANA name it resolved. Every fragment of the release this
//! reading rests on is in [`fragments`], which a test of the check-tools
//! layer finds byte for byte in the release the tree pins.

use chrono::{Datelike, NaiveDate, NaiveTime, TimeZone};
use chrono_tz::Tz;

/// The failure's `error` a usage limit carries; other failures, a model
/// under high load among them, carry it too, and the words tell them
/// apart.
pub const RATE_LIMIT: &str = "rate_limit";

/// The openings of the usage-limit messages, as 2.1.296's own list holds
/// them, in its order.
pub const OPENINGS: [&str; 12] = [
    "You've hit your",
    "You've reached your",
    "You're out of usage credits",
    "Your org is out of usage \u{b7} add funds to continue",
    "Your org is out of usage \u{b7} contact your admin",
    "Your seat type doesn't include usage credits",
    "Your seat type doesn't include usage",
    "Your usage allocation has been disabled by your admin",
    "Your group's usage limit is set to $0",
    "Fable 5 requires usage credits",
    "You're out of extra usage",
    "Your seat type doesn't include extra usage",
];

/// What separates the parts of the words.
const PART: &str = " \u{b7} ";

/// The part naming the reset opens so.
const RESETS: &str = "resets ";

/// The source fragments of 2.1.296 the reading rests on, each named and
/// as the release's program holds it: its list of the openings, the
/// template of the "You've hit your" messages, the reset's part and the
/// zone its reset formatter puts after the time.
pub fn fragments() -> Vec<(&'static str, String)> {
    let openings: Vec<String> = OPENINGS
        .iter()
        .map(|opening| format!("\"{}\"", opening.replace('\u{b7}', "\\xB7")))
        .collect();
    vec![
        (
            "the list of the usage-limit messages' openings",
            format!("var yCr=[{}]", openings.join(",")),
        ),
        (
            "the template of the \"You've hit your\" messages",
            "`You've hit your ${e}${r}${h}`".to_string(),
        ),
        (
            "the reset's part of the words",
            "` \\xB7 resets ${y}`".to_string(),
        ),
        (
            "the zone after the reset's time",
            "` (${s??zIs()})`".to_string(),
        ),
    ]
}

/// Whether a failure of `error` with `words` is a usage limit.
pub fn usage_limit(error: &str, words: &str) -> bool {
    error == RATE_LIMIT && OPENINGS.iter().any(|opening| words.starts_with(opening))
}

/// The reset the words name by the host's clock, read in the zone they
/// name from `at`, when the hook ran; none where they name none, or a
/// zone `chrono-tz` does not know (146).
pub fn reset(words: &str, at: u64) -> Option<u64> {
    let part = words
        .split(PART)
        .find_map(|part| part.strip_prefix(RESETS))?;
    let (when, zone) = part.strip_suffix(')')?.rsplit_once(" (")?;
    let zone: Tz = zone.parse().ok()?;
    let ran = zone.timestamp_opt(i64::try_from(at).ok()?, 0).single()?;
    let reset = match when.rsplit_once(", ") {
        None => {
            // A time of day, within a day of when the words were written:
            // the first such time in the zone after the hook ran. The words
            // name no seconds, so a time in the minute the hook ran is that
            // minute's.
            let time = time_of_day(when)?;
            let today = local(zone, ran.date_naive(), time)?;
            if today + 59 >= ran.timestamp() {
                today
            } else {
                local(zone, ran.date_naive().succ_opt()?, time)?
            }
        }
        Some((day, time)) => {
            let time = time_of_day(time)?;
            let (day, year) = match day.rsplit_once(", ") {
                Some((day, year)) if year.len() == 4 => (day, year.parse().ok()?),
                Some(_) => return None,
                None => (day, ran.year()),
            };
            let (month, day) = day.split_once(' ')?;
            let month = MONTHS.iter().position(|name| *name == month)?;
            let date =
                NaiveDate::from_ymd_opt(year, u32::try_from(month).ok()? + 1, day.parse().ok()?)?;
            local(zone, date, time)?
        }
    };
    u64::try_from(reset).ok()
}

/// The months as en-US writes them short.
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// A time of day as en-US writes it with a 12-hour clock, the minutes
/// left out on the hour: `3pm`, `3:30pm`, `12am`.
fn time_of_day(text: &str) -> Option<NaiveTime> {
    let (clock, afternoon) = match (text.strip_suffix("am"), text.strip_suffix("pm")) {
        (Some(clock), _) => (clock, false),
        (_, Some(clock)) => (clock, true),
        _ => return None,
    };
    let (hour, minute) = match clock.split_once(':') {
        Some((hour, minute)) if minute.len() == 2 => (hour, minute.parse().ok()?),
        Some(_) => return None,
        None => (clock, 0),
    };
    let hour: u32 = hour.parse().ok()?;
    if !(1..=12).contains(&hour) {
        return None;
    }
    NaiveTime::from_hms_opt(hour % 12 + if afternoon { 12 } else { 0 }, minute, 0)
}

/// The moment `date` at `time` in `zone`, the earlier where the zone's
/// clock passes it twice.
fn local(zone: Tz, date: NaiveDate, time: NaiveTime) -> Option<i64> {
    zone.from_local_datetime(&date.and_time(time))
        .earliest()
        .map(|moment| moment.timestamp())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The moment `year-month-day hour:minute` in `zone`.
    fn at(zone: &str, year: i32, month: u32, day: u32, hour: u32, minute: u32) -> u64 {
        let zone: Tz = zone.parse().expect("a zone");
        let moment = zone
            .with_ymd_and_hms(year, month, day, hour, minute, 0)
            .single()
            .expect("a moment");
        u64::try_from(moment.timestamp()).expect("after 1970")
    }

    /// The words 2.1.296 builds as `You've hit your ${limit}${" · resets "
    /// + when}${" · progress saved"?}`.
    fn hit(limit: &str, reset: Option<&str>, saved: bool) -> String {
        let reset = reset
            .map(|when| format!(" \u{b7} resets {when}"))
            .unwrap_or_default();
        let saved = if saved { " \u{b7} progress saved" } else { "" };
        format!("You've hit your {limit}{reset}{saved}")
    }

    #[test]
    fn a_session_limit_with_a_time_of_day() {
        let words = hit("session limit", Some("3pm (America/New_York)"), false);
        let ran = at("America/New_York", 2026, 10, 29, 13, 10);
        assert!(usage_limit(RATE_LIMIT, &words));
        assert_eq!(
            reset(&words, ran),
            Some(at("America/New_York", 2026, 10, 29, 15, 0))
        );
    }

    #[test]
    fn a_weekly_limit_with_a_date() {
        let words = hit("weekly limit", Some("Nov 2, 3:30pm (UTC)"), true);
        let ran = at("UTC", 2026, 10, 29, 9, 0);
        assert!(usage_limit(RATE_LIMIT, &words));
        assert_eq!(reset(&words, ran), Some(at("UTC", 2026, 11, 2, 15, 30)));
    }

    #[test]
    fn a_reset_past_midnight() {
        let words = hit("session limit", Some("1am (UTC)"), true);
        let ran = at("UTC", 2026, 10, 29, 23, 40);
        assert_eq!(reset(&words, ran), Some(at("UTC", 2026, 10, 30, 1, 0)));
    }

    #[test]
    fn a_reset_in_the_minute_the_hook_ran() {
        let words = hit("session limit", Some("3pm (UTC)"), false);
        let ran = at("UTC", 2026, 10, 29, 15, 0) + 20;
        assert_eq!(reset(&words, ran), Some(at("UTC", 2026, 10, 29, 15, 0)));
    }

    #[test]
    fn no_reset_in_the_words() {
        let words = hit("limit", None, false);
        assert!(usage_limit(RATE_LIMIT, &words));
        assert_eq!(reset(&words, at("UTC", 2026, 10, 29, 9, 0)), None);
    }

    #[test]
    fn a_model_under_high_load() {
        for words in [
            "Opus is experiencing high load, please use /model to switch to Sonnet",
            "Server is temporarily limiting requests (not your usage limit)",
            "No response requested.",
        ] {
            assert!(!usage_limit(RATE_LIMIT, words), "{words}");
        }
        assert!(!usage_limit(
            "authentication_failed",
            &hit("session limit", None, false)
        ));
    }

    #[test]
    fn a_monthly_spend_limit() {
        // `at(V, `${Z}${W}`)`: the limit, then where to raise it, and no
        // reset of its own.
        let words = "You've hit your monthly spend limit \u{b7} raise it at https://claude.ai/settings/usage?from=cc_cli_limit_message";
        assert!(usage_limit(RATE_LIMIT, words));
        assert_eq!(reset(words, at("UTC", 2026, 10, 29, 9, 0)), None);
    }

    #[test]
    fn a_date_in_another_year() {
        let words = hit(
            "weekly limit",
            Some("Jan 3, 2027, 9am (Europe/London)"),
            true,
        );
        let ran = at("Europe/London", 2026, 12, 30, 18, 0);
        assert_eq!(
            reset(&words, ran),
            Some(at("Europe/London", 2027, 1, 3, 9, 0))
        );
    }

    #[test]
    fn a_zone_the_crate_does_not_know() {
        let words = hit("session limit", Some("3pm (Mars/Olympus_Mons)"), false);
        assert!(usage_limit(RATE_LIMIT, &words));
        assert_eq!(reset(&words, at("UTC", 2026, 10, 29, 9, 0)), None);
    }

    #[test]
    fn every_opening_reads_a_usage_limit() {
        for opening in OPENINGS {
            assert!(usage_limit(RATE_LIMIT, opening), "{opening}");
        }
        assert!(usage_limit(
            RATE_LIMIT,
            "You're out of usage credits \u{b7} resets 3pm (UTC)"
        ));
    }

    #[test]
    fn the_openings_fragment_is_the_releases_list() {
        let (_, list) = &fragments()[0];
        assert!(list.starts_with("var yCr=[\"You've hit your\",\"You've reached your\","));
        assert!(list.contains("\"Your org is out of usage \\xB7 add funds to continue\""));
        assert!(list.ends_with("\"Your seat type doesn't include extra usage\"]"));
    }
}
