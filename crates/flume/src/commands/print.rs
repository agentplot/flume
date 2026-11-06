//! The core's answer printed for a person (61): every fact the JSON holds
//! that a person reads, in lines, and nothing the answer does not hold.
//! A word of the answer's own, a state or a layer, is printed as the JSON
//! spells it.

use flume_core::{
    group_name, percentage, AgentStatus, Answer, HostStatus, PlaceStatus, PoolStatus, Queued,
    ReadingStatus, Reserves, StageStatus, StreamOpened, StreamStatus, Tidied, TitleFrom,
    WindowOutcome, Woke,
};
use flume_seams::{
    AccountFrom, Act, BaseFrom, ConfigLayer, ConfigValue, Conversation, HoldOn, HostConfiguration,
    IntentKey, LabelHeld, LocalLayer, Made, NamedBy, ProfileHeld, Readiness, SessionFrom,
    SessionHeld, StreamHeld, SubmissionRead, UncarriedStart, UnsettledStart, WaitingOn, Waits,
    Watch,
};
use serde_json::{json, Value};

/// The answer as a person reads it.
pub fn person(answer: &Answer) -> String {
    let mut lines = vec![];
    match answer {
        Answer::Status { places, host } => {
            if places.is_empty() {
                lines.push("no place stands on this host".to_string());
            }
            for place in places {
                lines.extend(place_lines(place));
            }
            lines.extend(host_lines(host));
        }
        Answer::Launched { place } => {
            lines.push(format!("launched {}", place.place));
            lines.extend(place_lines(place));
        }
        Answer::Prompt { place, stage, text } => {
            lines.push(format!("the prompt {place}'s {stage} would be sent:"));
            lines.push(text.clone());
        }
        Answer::Acted {
            act,
            place,
            status,
            tidy,
        } => {
            lines.push(format!("{} {place}", word(json!(act))));
            if let Some(status) = status {
                lines.extend(place_lines(status));
            }
            if let Some(tidy) = tidy {
                lines.extend(tidy_lines(tidy));
            }
        }
        Answer::Settled { places, tidy } => {
            if places.is_empty() {
                lines.push("settled no place".to_string());
            }
            for place in places {
                lines.push(format!("settled {place}"));
            }
            lines.extend(tidy_lines(tidy));
        }
        Answer::Tidied { tidy } => {
            lines.extend(tidy_lines(tidy));
            if lines.is_empty() {
                lines.push("nothing to tidy".to_string());
            }
        }
        Answer::StopCheck { words } => lines.push(match words {
            Some(words) => words.clone(),
            None => "the agent may go quiet".to_string(),
        }),
        Answer::Refused { refusal } => lines.push(format!("refused: {}", refusal.reason)),
        Answer::Waited {
            place,
            recipient,
            end,
            delivered,
        } => lines.push(match end {
            None => format!("still waiting on {place} as {recipient}"),
            Some(end) => {
                let how = if *delivered {
                    "delivered by this wait"
                } else {
                    "read; delivered before"
                };
                format!(
                    "{place} {} for {recipient}: {how}",
                    flume_core::what_ended(&end.end.of)
                )
            }
        }),
        Answer::Inbox { recipient, ends } => {
            if ends.is_empty() {
                lines.push(format!("nothing pending for {recipient}"));
            }
            for collected in ends {
                lines.push(format!(
                    "{}--{} {}; read: flume status {}",
                    collected.kind,
                    collected.name,
                    flume_core::what_ended(&collected.end.end.of),
                    collected.place
                ));
            }
        }
        Answer::Woken {
            address,
            woke,
            place,
        } => {
            lines.push(match woke {
                Woke::Launched => format!("woke {address}: launched {}", place.place),
                Woke::Live => format!("{address}'s run {} is live; nothing made", place.place),
                Woke::Waiting => format!("{address}'s run {} waits; nothing made", place.place),
            });
            lines.extend(place_lines(place));
        }
        Answer::Lines { lines: places } => {
            lines.extend(places.iter().map(|line| line.full.clone()));
        }
        Answer::Opened(opened) => lines.extend(opened_lines(opened)),
        Answer::List { places, streams } => {
            if places.is_empty() {
                lines.push("no places".to_string());
            }
            let mut repository = None;
            for place in places {
                if repository != Some(&place.repository) {
                    repository = Some(&place.repository);
                    lines.push(place.repository.clone());
                }
                lines.push(format!("  {}", place.line));
            }
            lines.push("streams".to_string());
            lines.extend(streams.iter().map(|stream| format!("  {}", stream.line)));
        }
    }
    lines.join("\n")
}

/// An open's lines (186): what showed the stream's session, with its
/// title, its session, its multiplexer, its view and its terminal app
/// each with where it came from, and the command by its program, its
/// arguments and its variables' names; the printed window's line on a
/// line of its own, `run: <line>`, each variable by its name alone
/// (core.30).
fn opened_lines(opened: &StreamOpened) -> Vec<String> {
    let multiplexer = &opened.multiplexer.profile;
    let (session, its) = match &opened.session.name {
        Some(name) => {
            let from = session_from(opened.session.from);
            (
                format!("the session {name} ({from})"),
                format!("{multiplexer}'s session {name} ({from})"),
            )
        }
        None => (
            "its own session".to_string(),
            format!("{multiplexer}'s own session"),
        ),
    };
    let title = opened
        .title
        .as_ref()
        .map(|title| format!(" titled \"{}\" ({})", title.title, title_from(title.from)))
        .unwrap_or_default();
    let app = opened
        .terminal_app
        .as_ref()
        .map_or("no", |app| app.profile.as_str());
    let mut lines = vec![match &opened.window {
        WindowOutcome::Opened => format!("opened a {app} window{title} on {its}"),
        WindowOutcome::Printed { .. } => {
            format!("the terminal app {app} opens no window; run this line to show {its}{title}:")
        }
        WindowOutcome::App => format!("{multiplexer}'s app showed {session}"),
    }];
    let mut facts = vec![
        format!("stream {}", opened.stream),
        format!("multiplexer {}", profile_words(&opened.multiplexer)),
        format!(
            "view {} ({})",
            opened.view.view.name(),
            named_by(opened.view.from)
        ),
    ];
    if let Some(app) = &opened.terminal_app {
        facts.push(format!("terminal app {}", profile_words(app)));
    }
    lines.push(format!("  {}", facts.join(", ")));
    if let Some(command) = &opened.command {
        let mut words = vec![command.program.clone()];
        words.extend(command.arguments.iter().cloned());
        let needs = match command.environment.as_slice() {
            [] => String::new(),
            names => format!(", needing {}", names.join(", ")),
        };
        lines.push(format!("  its command {}{needs}", words.join(" ")));
    }
    if let WindowOutcome::Printed { line } = &opened.window {
        lines.push(format!("run: {line}"));
    }
    lines
}

/// A word of the answer's own as its JSON spells it.
fn word(value: Value) -> String {
    match value {
        Value::String(word) => word,
        Value::Object(map) if map.len() == 1 => {
            let (name, inner) = map.into_iter().next().expect("one entry");
            match inner {
                Value::Object(fields) if fields.is_empty() => name,
                inner => format!("{name} {inner}"),
            }
        }
        other => other.to_string(),
    }
}

/// A place's facts under its heading, its one line first (56, 60).
fn place_lines(place: &PlaceStatus) -> Vec<String> {
    let mut lines = vec![
        format!(
            "{}: {}@{} ({}), {}",
            place.place,
            place.kind,
            place.version,
            word(json!(place.declared)),
            word(json!(place.phase))
        ),
        format!("  {}", place.line.full),
    ];
    let mut facts = vec![format!(
        "branch {} from {}",
        place.branch,
        base_words(&place.base, place.base_from)
    )];
    if let Some(mark) = &place.mark {
        facts.push(format!("mark {mark}"));
    }
    if place.priority != 0 {
        facts.push(format!("priority {}", place.priority));
    }
    if let Some(head) = &place.head {
        facts.push(format!("head {head}"));
    }
    lines.push(format!("  {}", facts.join(", ")));
    lines.push(stream_line(
        &place.stream,
        place.label.as_ref(),
        &place.intent,
    ));
    lines.extend(readiness_lines(&place.readiness));
    for stage in &place.stages {
        lines.extend(stage_lines(stage));
    }
    if let Some(agent) = &place.agent {
        lines.push(agent_line(agent));
        lines.extend(where_line(agent));
        lines.push(terminal_line(agent));
        for (setting, given) in &agent.settings.from {
            lines.push(format!(
                "    {setting} = {} (from {})",
                given.value,
                word(json!(given.layer))
            ));
        }
        if let Some(conversation) = &agent.conversation {
            lines.push(conversation_line(conversation));
        }
        if let Some(printed) = &agent.printed {
            lines.push(format!("    run: {printed}"));
        }
        if let Some(words) = &agent.last_words {
            lines.push(format!("    last words: {words}"));
        }
        for note in &agent.notes {
            lines.push(format!("    note: {note}"));
        }
    }
    for moved in &place.moves {
        lines.push(format!("  moved: {}", moved.words()));
    }
    for unsettled in &place.unsettled {
        lines.push(unsettled_line(unsettled));
    }
    for uncarried in &place.uncarried {
        lines.push(uncarried_line(uncarried));
    }
    if let Some(delivered) = place.delivered {
        lines.push(format!(
            "  the deliverable {}",
            if delivered { "exists" } else { "is missing" }
        ));
    }
    if let Some(tasks) = &place.tasks {
        let ticked = tasks.iter().filter(|task| task.ticked).count();
        lines.push(format!("  tasks: {ticked} of {} ticked", tasks.len()));
    }
    if let Some(submission) = &place.submission {
        lines.push(format!(
            "  submission: {}, submitted {} times, sent back {} times",
            word(json!(submission.read)),
            submission.submits,
            submission.send_backs
        ));
        if let Some(request) = &submission.request {
            lines.push(match submission.read {
                SubmissionRead::None => {
                    format!("    pull request #{request}, not yet taken in by the merge queue")
                }
                _ => format!("    from pull request #{request}"),
            });
        }
        if let Some(waiting) = &submission.waiting {
            lines.push(format!("    waiting: {waiting}"));
        }
    }
    if let Some(waiting) = &place.waiting_on {
        lines.push(waiting_line(waiting, place.stalled));
    }
    for hold in &place.holds {
        let what = match &hold.on {
            HoldOn::Stage { stage } => stage.as_str(),
            HoldOn::Submission => "the submission",
        };
        let mut line = format!("  {what} on hold since {}", hold.since);
        if let Some(words) = &hold.words {
            line.push_str(&format!(": {words}"));
        }
        lines.push(line);
    }
    if let Some(role) = &place.role {
        lines.push(format!("  the run of {role}"));
    }
    if !place.recipients.is_empty() {
        lines.push(format!("  recipients: {}", place.recipients.join(", ")));
    }
    for end in &place.ends {
        let mut line = format!("  ended: {}", flume_core::what_ended(&end.end.of));
        let delivered: Vec<&str> = end
            .end
            .deliveries
            .iter()
            .map(|delivery| delivery.recipient.as_str())
            .collect();
        if !delivered.is_empty() {
            line.push_str(&format!("; delivered to {}", delivered.join(", ")));
        }
        if !end.pending_for.is_empty() {
            line.push_str(&format!("; pending for {}", end.pending_for.join(", ")));
        }
        lines.push(line);
    }
    for (name, list) in [
        ("refused act", &place.refused_acts),
        ("cannot read", &place.cannot_read),
        ("failure", &place.failures),
    ] {
        for item in list {
            lines.push(format!("  {name}: {item}"));
        }
    }
    lines
}

/// What a place waits on its caller for, since when, and its words; a
/// place waiting on its readiness without them, since its readiness
/// lines give the failed step and the last of its words (12, 56); a
/// place waiting on a person to answer its agent's dialog with the
/// dialog's lines beneath, as the terminal draws them (34, 35).
fn waiting_line(waiting: &WaitingOn, stalled: bool) -> String {
    let mut line = match waiting.act {
        Act::Answer => format!(
            "  waits on a person to answer the dialog its agent shows{} ({})",
            waiting
                .stage
                .as_ref()
                .map(|stage| format!(" in {stage}"))
                .unwrap_or_default(),
            word(json!(waiting.why))
        ),
        _ => format!(
            "  waits on its caller to {}{} ({})",
            word(json!(waiting.act)),
            waiting
                .stage
                .as_ref()
                .map(|stage| format!(" {stage}"))
                .unwrap_or_default(),
            word(json!(waiting.why))
        ),
    };
    if let Some(since) = waiting.since {
        line.push_str(&format!(" since {since}"));
    }
    if stalled {
        line.push_str(", stalled");
    }
    match &waiting.words {
        Some(words) if waiting.act == Act::Answer => {
            line.push(':');
            for drawn in plain(words).lines() {
                line.push_str(&format!("\n    {drawn}"));
            }
        }
        Some(words) if waiting.why != Waits::Unready => line.push_str(&format!(": {words}")),
        _ => {}
    }
    line
}

/// The last lines of a failed readiness step's words a person is shown.
const FAILED_LINES: usize = 10;

/// Each readiness step's outcome in one line, and under a failed step
/// the last lines of its words as a terminal leaves them to read; the
/// JSON keeps every word (12, 79).
fn readiness_lines(readiness: &Readiness) -> Vec<String> {
    let mut lines = vec![];
    for step in &readiness.steps {
        if step.passed {
            lines.push(format!("  readiness {}: passed", step.step));
            continue;
        }
        lines.push(format!("  readiness {}: failed", step.step));
        let words = plain(&step.words);
        let printed: Vec<&str> = words
            .lines()
            .filter_map(|line| {
                line.split('\r')
                    .map(str::trim_end)
                    .rfind(|part| !part.is_empty())
            })
            .collect();
        for line in &printed[printed.len().saturating_sub(FAILED_LINES)..] {
            lines.push(format!("    {line}"));
        }
    }
    lines
}

/// Words with every escape sequence and every control character but a
/// line's end, a carriage return and a tab taken out, as a terminal
/// would draw them.
fn plain(words: &str) -> String {
    let mut plain = String::with_capacity(words.len());
    let mut chars = words.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\u{1b}' => match chars.next() {
                // A control sequence, up to its final byte.
                Some('[') => {
                    for c in chars.by_ref() {
                        if ('\u{40}'..='\u{7e}').contains(&c) {
                            break;
                        }
                    }
                }
                // A string, up to its bell or its string terminator.
                Some(']' | 'P' | 'X' | '^' | '_') => {
                    while let Some(c) = chars.next() {
                        if c == '\u{7}' {
                            break;
                        }
                        if c == '\u{1b}' {
                            chars.next_if_eq(&'\\');
                            break;
                        }
                    }
                }
                // Intermediate bytes, then a final one.
                Some(c) if ('\u{20}'..='\u{2f}').contains(&c) => {
                    for c in chars.by_ref() {
                        if !('\u{20}'..='\u{2f}').contains(&c) {
                            break;
                        }
                    }
                }
                _ => {}
            },
            '\n' | '\r' | '\t' => plain.push(c),
            c if c.is_control() => {}
            c => plain.push(c),
        }
    }
    plain
}

/// A place's stream, its label where it has one, each with where it came
/// from, and its intent (56, 143, 147, 180).
/// The base a place was cut from, with where the launch read its revision
/// (56, C.1).
fn base_words(base: &str, from: Option<BaseFrom>) -> String {
    let from = match from {
        None => return base.to_string(),
        Some(BaseFrom::MergeQueue) => "where its merge queue lands it",
        Some(BaseFrom::Origin) => "at origin",
        Some(BaseFrom::NoOrigin) => "the host's own, the repository having no origin",
        Some(BaseFrom::OriginLacks) => "the host's own, origin holding none",
    };
    format!("{base} ({from})")
}

fn stream_line(stream: &StreamHeld, label: Option<&LabelHeld>, intent: &IntentKey) -> String {
    let label = label
        .map(|label| format!(", label {} (from {})", label.name, word(json!(label.from))))
        .unwrap_or_default();
    format!(
        "  stream {} (from {}){label}, {}",
        stream.name,
        word(json!(stream.from)),
        intent_words(intent)
    )
}

/// The intent a place serves, as a person reads it (147).
fn intent_words(intent: &IntentKey) -> String {
    match intent {
        IntentKey::Launch { name, .. } => format!("intent {name} (from the launch)"),
        IntentKey::Base {
            repository, base, ..
        } => format!("intent of {repository} on {base}"),
    }
}

/// The current agent's line: its identity, program, account and state,
/// the account from the intent's affinity for the agent's program or the
/// strategy, and its mode as written with its layer, the entry it took of
/// a table and the value passed, where a layer gave one (56, 145, 147,
/// 169).
fn agent_line(agent: &AgentStatus) -> String {
    let mut line = format!("  agent {} ({})", agent.identity, agent.program);
    if let Some(account) = &agent.account {
        let from = match account.from {
            AccountFrom::Affinity => format!("the affinity for {}", agent.program),
            AccountFrom::Strategy => "the strategy".to_string(),
        };
        line.push_str(&format!(
            " on the account {} of {} (from {from})",
            account.name, account.pool,
        ));
    }
    line.push_str(&format!(": {}", word(json!(agent.state))));
    if agent.past_limit {
        line.push_str(", past its limit");
    }
    if let (Some(mode), Some(given)) = (&agent.mode, agent.settings.from.get("mode")) {
        line.push_str(&format!(
            ", in the mode {} (from {})",
            given.value,
            word(json!(given.layer))
        ));
        if let Some(entry) = &mode.entry {
            line.push_str(&format!(", its entry for {entry}"));
        }
        line.push_str(&format!(" passed as {}", mode.passed));
        if !mode.read_back {
            line.push_str(", not read back");
        }
    }
    line
}

/// Where the agent opened: the session with where it came from, or the
/// pane it opened beside; none for a start written before (56, 107).
fn where_line(agent: &AgentStatus) -> Option<String> {
    match (&agent.session, &agent.settings.settings.beside) {
        (Some(session), _) => Some(format!("    in {}", session_words(session))),
        (None, Some(pane)) => Some(format!("    beside the pane {pane}")),
        (None, None) => None,
    }
}

/// The terminal the agent opened in, with where it came from; the host's
/// for a start written before, which records none (56, 190).
fn terminal_line(agent: &AgentStatus) -> String {
    match &agent.terminal {
        Some(terminal) => format!(
            "    its terminal {}, {}",
            terminal.profile,
            named_by(terminal.from)
        ),
        None => "    its terminal the host's".into(),
    }
}

/// Where a stream's multiplexer, view or terminal app came from (186,
/// 190).
fn named_by(from: NamedBy) -> &'static str {
    match from {
        NamedBy::Stream => "the stream's",
        NamedBy::Host => "the host's",
        NamedBy::Neither => "the default",
    }
}

/// A profile with where it came from: `printed (the host's)` (186, 190).
fn profile_words(held: &ProfileHeld) -> String {
    format!("{} ({})", held.profile, named_by(held.from))
}

/// Where a stream's title came from (143, 186).
fn title_from(from: TitleFrom) -> &'static str {
    match from {
        TitleFrom::Stream => "the stream's title",
        TitleFrom::StreamName => "the stream's name",
    }
}

/// A stream's display: its multiplexer, view, terminal app and title,
/// each with where it came from (56, 143, 186, 190).
fn display_line(stream: &StreamStatus) -> String {
    format!(
        "    multiplexer {}, view {} ({}), terminal app {}, title \"{}\" ({})",
        profile_words(&stream.multiplexer),
        stream.view.view.name(),
        named_by(stream.view.from),
        profile_words(&stream.terminal_app),
        stream.title.title,
        title_from(stream.title.from)
    )
}

/// A session with where it came from: `the session work, the stream's
/// name`, or the terminal's own (56, 107, 143).
fn session_words(session: &SessionHeld) -> String {
    let Some(name) = &session.name else {
        return "the terminal's own session".into();
    };
    format!("the session {name}, {}", session_from(session.from))
}

/// Where a session's name came from (56, 107, 143).
fn session_from(from: SessionFrom) -> &'static str {
    match from {
        SessionFrom::Launch => "the launch's",
        SessionFrom::Stream => "the stream's",
        SessionFrom::StreamName => "the stream's name",
        SessionFrom::Own => "the terminal's own",
    }
}

/// A pool's lines: its strategy and threshold, then each account's
/// program, agents running, bound and spent time, its reading under it,
/// and what holds it past its threshold; never a reading's path (56, 144
/// to 146, 184, 185).
fn pool_lines(pool: &PoolStatus) -> Vec<String> {
    let mut lines = vec![format!(
        "  pool {}, {}, threshold {}%:",
        pool.name,
        pool.strategy.name(),
        pool.threshold
    )];
    for account in &pool.accounts {
        let mut line = match account.bound {
            Some(bound) => format!(
                "    {} ({}): {} agents running of {bound}",
                account.name, account.program, account.running
            ),
            None => format!(
                "    {} ({}): {} agents running, no bound of its own",
                account.name, account.program, account.running
            ),
        };
        if let Some(until) = account.spent_until {
            line.push_str(&format!(", spent until {until}"));
        }
        lines.push(line);
        if let Some(reading) = &account.reading {
            lines.push(format!("      {}", reading_line(reading)));
        }
        if let Some(past) = &account.past {
            lines.push(format!(
                "      past its threshold in {} until {}",
                past.window, past.until
            ));
        }
    }
    lines
}

/// An account's reading as a person reads it: each window's use and
/// reset, then when the host read it, stale where it is not fresh; or why
/// there is none (184).
fn reading_line(reading: &ReadingStatus) -> String {
    match reading {
        ReadingStatus::Read {
            read_at,
            fresh,
            windows,
        } => {
            let mut parts: Vec<String> = windows
                .iter()
                .map(|window| {
                    let used = format!("{} {}%", window.name, percentage(window.used));
                    match window.resets_at {
                        Some(resets_at) => format!("{used} (resets {resets_at})"),
                        None => used,
                    }
                })
                .collect();
            parts.push(if *fresh {
                format!("read at {read_at}")
            } else {
                format!("stale, read at {read_at}")
            });
            parts.join(" · ")
        }
        ReadingStatus::Unread { unread } => format!("no reading: {unread}"),
    }
}

/// A stage's line, a repeating one named with its group's place among
/// the change's groups, `code 2/3`, and the group it names; then each
/// line its end carries of what it made (61, 129, 142).
fn stage_lines(stage: &StageStatus) -> Vec<String> {
    let mut line = format!(
        "  stage {}: {}, started {} times",
        stage.label(),
        word(json!(stage.state)),
        stage.starts
    );
    if let Some(group) = &stage.group {
        line.push_str(&format!(
            ", on {}",
            group_name(group.number.as_deref(), group.title.as_deref())
        ));
    }
    if let Some(end) = &stage.end {
        if let Some(head) = &end.head {
            line.push_str(&format!(", ended at {head}"));
        }
        if let Some(words) = &end.words {
            line.push_str(&format!(", saying: {words}"));
        }
    }
    if let Some(part) = &stage.refused_part {
        line.push_str(&format!(", refused for want of {part}"));
    }
    if let Some(setting) = &stage.refused_setting {
        line.push_str(&format!(", refused: {setting}"));
    }
    let mut lines = vec![line];
    for made in stage.end.iter().flat_map(|end| &end.made) {
        let line = match made {
            Made::Thing(thing) => json!(thing).to_string(),
            Made::Text(text) => text.clone(),
        };
        lines.push(format!("    made: {line}"));
    }
    lines
}

/// The host's facts, what its terminal's configuration lacks for flume's
/// plugin under the line naming the watch (72, 122).
fn host_lines(host: &HostStatus) -> Vec<String> {
    let mut lines = vec![match (host.bound, host.room) {
        (Some(bound), Some(room)) => format!(
            "host: {} agents running of {bound}, room for {room}",
            host.running
        ),
        _ => format!("host: {} agents running, no bound", host.running),
    }];
    for queued in &host.queue {
        lines.push(format!("  {}", queued_line(queued)));
    }
    for stream in &host.streams {
        let room = match (stream.bound, stream.room) {
            (Some(bound), Some(room)) => format!(
                "{} agents running of {bound}, room for {room}",
                stream.running
            ),
            _ => format!("{} agents running, no bound of its own", stream.running),
        };
        lines.push(format!(
            "  stream {}, priority {}: {room}",
            stream.name, stream.priority
        ));
        lines.push(format!(
            "    its agents open in {}",
            session_words(&stream.session)
        ));
        lines.push(display_line(stream));
        lines.extend(label_lines(stream));
        for queued in &stream.queue {
            lines.push(format!("    {}", queued_line(queued)));
        }
    }
    for pool in &host.pools {
        lines.extend(pool_lines(pool));
    }
    lines.extend(configuration_lines(&host.configuration));
    for kept in &host.terminal_kept {
        lines.push(format!("  the terminal keeps {kept}"));
    }
    for terminal in host.terminals.iter().skip(1) {
        let name = terminal.profile.as_deref().unwrap_or("the host's own");
        for kept in &terminal.kept {
            lines.push(format!("  the terminal {name} keeps {kept}"));
        }
        lines.push(match terminal.watch {
            Watch::Bound { seconds } => {
                format!("  the terminal {name} keeps watch every {seconds} seconds")
            }
            Watch::None => format!(
                "  the terminal {name} keeps no watch, so every place settles at each command of flume's that acts"
            ),
        });
    }
    for address in &host.addresses {
        let run = match (&address.run, &address.role) {
            (Some(run), _) => format!("its run {run}"),
            (None, Some(role)) => format!("no run; it wakes {} in {}", role.kind, role.repository),
            (None, None) => "no role configured".to_string(),
        };
        lines.push(format!(
            "  {} ends pending for {}, {run}",
            address.pending.len(),
            address.address
        ));
        for end in &address.pending {
            lines.push(format!("    {}: {}", end.place, end.what));
        }
        if let Some(unwoken) = &address.unwoken {
            lines.push(format!("    not woken: {unwoken}"));
        }
    }
    lines.push(match host.watch {
        Watch::Bound { seconds } => format!(
            "  the terminal keeps watch: every place settles every {seconds} seconds and when an agent's state changes"
        ),
        Watch::None => "  the terminal keeps no watch: places move on when a command of flume's acts or an agent goes quiet".into(),
    });
    for setup in &host.terminal_setup {
        lines.push(format!("    {setup}"));
    }
    lines
}

/// A start whose agent began without its settings, each variable by its
/// name alone, and a mode that differed by the mode asked and the mode
/// begun (56, 161, 162).
fn unsettled_line(unsettled: &UnsettledStart) -> String {
    format!(
        "  without its settings: {}'s start {} began at {} {}, and was not taken as its stage's agent",
        unsettled.stage,
        unsettled.start,
        unsettled.at,
        unsettled.unsettled.words()
    )
}

/// Whether the current agent carries the conversation of its stage's
/// start whose agent stopped at its account's usage limit, or why not,
/// naming no path (56, 146, 179; core.30).
fn conversation_line(conversation: &Conversation) -> String {
    match &conversation.why {
        None if conversation.carried => {
            format!(
                "    carries the conversation of start {}",
                conversation.from
            )
        }
        why => format!(
            "    starts afresh after start {}: {}",
            conversation.from,
            why.as_deref().unwrap_or("its conversation stayed behind")
        ),
    }
}

/// A start that carried its stage's conversation and did not begin, with
/// what happened (56, 146, S79).
fn uncarried_line(uncarried: &UncarriedStart) -> String {
    format!(
        "  carried and not begun: {}'s start {} at {}: {}, and its stage started afresh",
        uncarried.stage, uncarried.start, uncarried.at, uncarried.words
    )
}

/// A stage waiting for room, with its stream, its label where it has
/// one, its priority and the room it is short of (56, 70, 143, 180).
fn queued_line(queued: &Queued) -> String {
    let label = queued
        .label
        .as_ref()
        .map(|label| format!(" as {label}"))
        .unwrap_or_default();
    format!(
        "waiting for room: {}'s {} in {}{label}, priority {}, short of {}",
        queued.place,
        queued.stage,
        queued.stream,
        queued.priority,
        queued.wants.words()
    )
}

/// A stream's labels, each with its reserve, what it keeps free, its
/// agents running and its stages waiting; its reserves passed over where
/// it states no bound; and what it refuses (56, 181, 182).
fn label_lines(stream: &StreamStatus) -> Vec<String> {
    let mut lines: Vec<String> = stream
        .labels
        .iter()
        .map(|label| {
            format!(
                "    label {}: a reserve of {}, keeping {} free, {} agents running, {} waiting",
                label.name, label.reserve, label.keep_free, label.running, label.waiting
            )
        })
        .collect();
    if stream.reserves == Reserves::PassedOver {
        lines.push("    its labels' reserves are passed over: it states no bound".into());
    }
    let view = stream.view_refused.as_deref();
    let stages = match (stream.refused.as_deref(), view) {
        (Some(refused), Some(view)) => refused
            .strip_suffix(view)
            .map(|rest| rest.trim_end_matches("; "))
            .filter(|rest| !rest.is_empty()),
        (refused, _) => refused,
    };
    if let Some(stages) = stages {
        lines.push(format!("    it starts no stage: {stages}"));
    }
    if let Some(view) = view {
        lines.push(format!("    it opens no view: {view}"));
    }
    lines
}

/// The host's local layer: taken, with each value it set and each key
/// passed over with its pin, or not taken, naming each key or its fault;
/// nothing where the host has no local file. A value is as status gives
/// it, an account's variable `<set>` (56, C.7; core.30, core.45).
fn configuration_lines(configuration: &HostConfiguration) -> Vec<String> {
    match &configuration.local {
        LocalLayer::NoFile => vec![],
        LocalLayer::Taken => {
            let set: Vec<&ConfigValue> = configuration
                .values
                .iter()
                .filter(|value| value.layer == ConfigLayer::Local)
                .collect();
            let mut lines = vec![if set.is_empty() {
                "  the host's local layer is taken, setting nothing".to_string()
            } else {
                "  the host's local layer is taken, setting:".to_string()
            }];
            for value in set {
                lines.push(format!("    {} = {}", value.key, value.value));
            }
            for passed in &configuration.passed_over {
                lines.push(format!(
                    "    {} passed over: the managed layer pins {}",
                    passed.key, passed.pin
                ));
            }
            lines
        }
        LocalLayer::NotTaken { keys, fault } => {
            let mut lines = vec![];
            if !keys.is_empty() {
                lines.push(format!(
                    "  the host's local layer is not taken: it sets {}, which the local layer may not set",
                    keys.join(", ")
                ));
            }
            if let Some(fault) = fault {
                lines.push(format!("  the host's local layer is not taken: {fault}"));
            }
            lines
        }
    }
}

fn tidy_lines(tidy: &Tidied) -> Vec<String> {
    let mut lines = vec![];
    for place in &tidy.removed {
        lines.push(format!("removed {place}"));
    }
    for kept in &tidy.kept {
        lines.push(format!("kept {}: {}", kept.place, word(json!(kept.why))));
    }
    for branch in &tidy.branches_kept {
        lines.push(format!(
            "kept the branch {} of {}, {} commits not landed",
            branch.branch,
            branch.place,
            branch.commits.len()
        ));
    }
    for place in &tidy.not_over {
        lines.push(format!("{place}'s work is not over"));
    }
    for terminal in &tidy.terminal {
        let within = match &terminal.profile {
            Some(profile) => format!("in the terminal {profile}"),
            None => "in the host's own terminal".into(),
        };
        for closed in &terminal.closed {
            lines.push(format!("closed {closed} of {} {within}", terminal.place));
        }
        for kept in &terminal.kept {
            lines.push(format!("kept {kept} of {} {within}", terminal.place));
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use flume_core::{
        AccountStatus, AgentSettings, ModeStatus, MoveStatus, Past, PlaceLine, RoomWanted,
        StreamStatus, TitleFrom, TitleHeld, WindowStatus,
    };
    use flume_seams::{
        AccountFrom, AccountHeld, AccountMove, AgentState, Declared, Given, Layer, ModeBegan,
        MoveOn, MoveWhy, NamedBy, Phase, PlaceId, ProfileHeld, Reading, ReadyStep, Strategy,
        StreamFrom, StreamHeld, Unsettled, View, ViewHeld,
    };
    use std::collections::BTreeMap;

    #[test]
    fn an_agents_account() {
        let agent = AgentStatus {
            identity: "flume:r:fix--one:code:2".into(),
            stage: "code".into(),
            program: "agent".into(),
            settings: AgentSettings::default(),
            account: Some(AccountHeld {
                pool: "work".into(),
                name: "work-2".into(),
                from: AccountFrom::Strategy,
            }),
            state: AgentState::Working,
            started: Reading::Yes,
            asking: Reading::No,
            own_work: Reading::No,
            past_limit: false,
            screen: None,
            dialog: None,
            printed: None,
            last_words: None,
            notes: vec![],
            mode: None,
            conversation: None,
            session: None,
            terminal: None,
        };
        assert_eq!(
            agent_line(&agent),
            "  agent flume:r:fix--one:code:2 (agent) on the account work-2 of work (from the strategy): working"
        );
        let kept = AgentStatus {
            program: "other".into(),
            account: Some(AccountHeld {
                pool: "work".into(),
                name: "other-1".into(),
                from: AccountFrom::Affinity,
            }),
            ..agent.clone()
        };
        assert_eq!(
            agent_line(&kept),
            "  agent flume:r:fix--one:code:2 (other) on the account other-1 of work (from the affinity for other): working"
        );
        let none = AgentStatus {
            account: None,
            ..agent
        };
        assert_eq!(
            agent_line(&none),
            "  agent flume:r:fix--one:code:2 (agent): working"
        );
    }

    #[test]
    fn each_agents_session() {
        let agent = |session: Option<SessionHeld>, beside: Option<&str>| AgentStatus {
            settings: AgentSettings {
                settings: flume_seams::Settings {
                    beside: beside.map(str::to_string),
                    ..flume_seams::Settings::default()
                },
                ..AgentSettings::default()
            },
            session,
            ..in_mode(
                "edits",
                Layer::Launch,
                ModeStatus {
                    passed: "edits".into(),
                    entry: None,
                    read_back: true,
                },
            )
        };
        let named = |name: &str, from| {
            Some(SessionHeld {
                name: Some(name.into()),
                from,
            })
        };
        let own = Some(SessionHeld {
            name: None,
            from: SessionFrom::Own,
        });
        for (session, beside, line) in [
            (
                named("review", SessionFrom::Launch),
                None,
                Some("    in the session review, the launch's"),
            ),
            (
                named("office", SessionFrom::Stream),
                None,
                Some("    in the session office, the stream's"),
            ),
            (
                named("work", SessionFrom::StreamName),
                None,
                Some("    in the session work, the stream's name"),
            ),
            (own, None, Some("    in the terminal's own session")),
            (None, Some("operator"), Some("    beside the pane operator")),
            (None, None, None),
        ] {
            assert_eq!(
                where_line(&agent(session, beside)).as_deref(),
                line,
                "{line:?}"
            );
        }
    }

    /// A profile `name` from `from`.
    fn held(name: &str, from: NamedBy) -> ProfileHeld {
        ProfileHeld {
            profile: name.into(),
            from,
        }
    }

    /// The open of `work` under the multiplexer `mux`, its window as
    /// `window` came to, in `view`.
    fn opened(window: WindowOutcome, view: View) -> StreamOpened {
        let terminal = view == View::Terminal;
        StreamOpened {
            stream: "work".into(),
            multiplexer: held("mux", NamedBy::Host),
            session: SessionHeld {
                name: Some("work".into()),
                from: SessionFrom::StreamName,
            },
            view: ViewHeld {
                view,
                from: if terminal {
                    NamedBy::Neither
                } else {
                    NamedBy::Stream
                },
            },
            title: terminal.then(|| TitleHeld {
                title: "Work Agents".into(),
                from: TitleFrom::Stream,
            }),
            terminal_app: terminal.then(|| held("app-window", NamedBy::Stream)),
            command: terminal.then(|| flume_core::ViewCommandStatus {
                program: "/bin/mux".into(),
                arguments: vec!["session".into(), "attach".into(), "work".into()],
                environment: vec!["XDG_CONFIG_HOME".into()],
            }),
            window,
        }
    }

    #[test]
    fn the_opened_printed_and_app_lines() {
        // 186, 188; core.30: what showed the session, each setting with
        // where it came from, the command's variables by name alone, and
        // the printed window's line on a line of its own.
        assert_eq!(
            opened_lines(&opened(WindowOutcome::Opened, View::Terminal)),
            [
                "opened a app-window window titled \"Work Agents\" (the stream's title) on mux's session work (the stream's name)",
                "  stream work, multiplexer mux (the host's), view terminal (the default), terminal app app-window (the stream's)",
                "  its command /bin/mux session attach work, needing XDG_CONFIG_HOME",
            ]
        );
        let line = "XDG_CONFIG_HOME=<set> /bin/mux session attach work";
        let printed = person(&Answer::Opened(opened(
            WindowOutcome::Printed { line: line.into() },
            View::Terminal,
        )));
        assert_eq!(
            printed.lines().last(),
            Some(format!("run: {line}").as_str())
        );
        assert!(
            printed.starts_with("the terminal app app-window opens no window; run this line to show mux's session work (the stream's name) titled \"Work Agents\" (the stream's title):"),
            "{printed}"
        );
        assert_eq!(
            opened_lines(&opened(WindowOutcome::App, View::App)),
            [
                "mux's app showed the session work (the stream's name)",
                "  stream work, multiplexer mux (the host's), view app (the stream's)",
            ]
        );
    }

    #[test]
    fn a_streams_display_and_a_terminal_app_refused() {
        // 56, 186, 188, 190: the display with where each came from; a
        // terminal app's refusal said of the view alone, beside a label's
        // fault that starts no stage.
        let stream = |refused: Option<&str>, view_refused: Option<&str>| StreamStatus {
            name: "office".into(),
            session: SessionHeld {
                name: Some("office".into()),
                from: SessionFrom::StreamName,
            },
            multiplexer: held("mux", NamedBy::Host),
            view: ViewHeld {
                view: View::Terminal,
                from: NamedBy::Host,
            },
            terminal_app: held("kitty", NamedBy::Stream),
            title: TitleHeld {
                title: "Office Agents".into(),
                from: TitleFrom::Stream,
            },
            priority: 0,
            running: 0,
            bound: None,
            room: None,
            queue: vec![],
            labels: vec![],
            reserves: Reserves::None,
            refused: refused.map(str::to_string),
            view_refused: view_refused.map(str::to_string),
        };
        let kitty = "the stream office names the terminal app kitty, and no window profile is named kitty; the window profiles there are: printed";
        let fault = "the label ops of the stream office holds pool, which no label takes";
        assert_eq!(
            display_line(&stream(None, None)),
            "    multiplexer mux (the host's), view terminal (the host's), terminal app kitty (the stream's), title \"Office Agents\" (the stream's title)"
        );
        assert_eq!(
            label_lines(&stream(Some(kitty), Some(kitty))),
            [format!("    it opens no view: {kitty}")]
        );
        assert_eq!(
            label_lines(&stream(Some(&format!("{fault}; {kitty}")), Some(kitty))),
            [
                format!("    it starts no stage: {fault}"),
                format!("    it opens no view: {kitty}"),
            ]
        );
        assert_eq!(
            label_lines(&stream(Some(fault), None)),
            [format!("    it starts no stage: {fault}")]
        );
    }

    #[test]
    fn each_starts_terminal() {
        // 56, 190: the terminal with where it came from, and the host's
        // for a start written before.
        let agent = |terminal| AgentStatus {
            terminal,
            ..in_mode(
                "edits",
                Layer::Launch,
                ModeStatus {
                    passed: "edits".into(),
                    entry: None,
                    read_back: true,
                },
            )
        };
        assert_eq!(
            terminal_line(&agent(Some(held("mux", NamedBy::Stream)))),
            "    its terminal mux, the stream's"
        );
        assert_eq!(
            terminal_line(&agent(Some(held("printed", NamedBy::Neither)))),
            "    its terminal printed, the default"
        );
        assert_eq!(terminal_line(&agent(None)), "    its terminal the host's");
    }

    #[test]
    fn a_command_that_needs_a_variable() {
        // 186, core.30: the stream-views scenario's answer in the
        // person's form, the variable by its name alone, its value
        // nowhere, and the printed line on a line of its own.
        use flume_conformance::engine::open::{a_variables_open, VARIABLES_LINE, VARIABLES_VALUE};
        let printed = person(&a_variables_open());
        assert!(
            printed
                .contains("  its command /bin/fake-terminal attach work, needing XDG_CONFIG_HOME"),
            "{printed}"
        );
        assert!(!printed.contains(VARIABLES_VALUE), "{printed}");
        assert_eq!(
            printed.lines().last(),
            Some(format!("run: {VARIABLES_LINE}").as_str()),
            "{printed}"
        );
    }

    #[test]
    fn each_terminal_a_tidy_closed_in() {
        // 137, 190: what each terminal closed and kept of a place, named
        // by its profile, the host's own where no profile gives it.
        let place = flume_seams::PlaceId::new("/r/places/pair--one");
        let tidy = Tidied {
            terminal: vec![
                flume_core::TerminalTidied {
                    place: place.clone(),
                    profile: Some("other".into()),
                    closed: vec!["the tab pair--one".into()],
                    kept: vec![],
                },
                flume_core::TerminalTidied {
                    place,
                    profile: None,
                    closed: vec![],
                    kept: vec!["the workspace r main, held by the pane w1:p3".into()],
                },
            ],
            ..Tidied::default()
        };
        assert_eq!(
            tidy_lines(&tidy),
            vec![
                "closed the tab pair--one of /r/places/pair--one in the terminal other",
                "kept the workspace r main, held by the pane w1:p3 of /r/places/pair--one in the host's own terminal",
            ]
        );
    }

    /// An agent at work under `mode`, as `layer` wrote it, its start's
    /// mode as `status` gives it.
    fn in_mode(written: &str, layer: Layer, status: ModeStatus) -> AgentStatus {
        AgentStatus {
            identity: "flume:r:fix--one:code:1".into(),
            stage: "code".into(),
            program: "agent".into(),
            settings: AgentSettings {
                from: BTreeMap::from([(
                    "mode".to_string(),
                    Given {
                        layer,
                        value: written.into(),
                    },
                )]),
                ..AgentSettings::default()
            },
            account: None,
            state: AgentState::Working,
            started: Reading::Yes,
            asking: Reading::No,
            own_work: Reading::No,
            past_limit: false,
            screen: None,
            dialog: None,
            printed: None,
            last_words: None,
            notes: vec![],
            mode: Some(status),
            conversation: None,
            session: None,
            terminal: None,
        }
    }

    #[test]
    fn a_mode_from_the_hosts_table() {
        let agent = in_mode(
            "edits",
            Layer::HostKind,
            ModeStatus {
                passed: "edit-files".into(),
                entry: None,
                read_back: true,
            },
        );
        assert_eq!(
            agent_line(&agent),
            "  agent flume:r:fix--one:code:1 (agent): working, in the mode edits (from host-kind) passed as edit-files"
        );
    }

    #[test]
    fn a_tables_entry() {
        let agent = in_mode(
            r#"{ agent = "careful", other = "edits" }"#,
            Layer::Launch,
            ModeStatus {
                passed: "careful".into(),
                entry: Some("agent".into()),
                read_back: true,
            },
        );
        assert_eq!(
            agent_line(&agent),
            r#"  agent flume:r:fix--one:code:1 (agent): working, in the mode { agent = "careful", other = "edits" } (from launch), its entry for agent passed as careful"#
        );
    }

    #[test]
    fn a_mode_not_read_back() {
        let agent = in_mode(
            "edits",
            Layer::Kind,
            ModeStatus {
                passed: "edit-files".into(),
                entry: None,
                read_back: false,
            },
        );
        assert_eq!(
            agent_line(&agent),
            "  agent flume:r:fix--one:code:1 (agent): working, in the mode edits (from kind) passed as edit-files, not read back"
        );
    }

    #[test]
    fn a_start_without_its_settings() {
        let unsettled = UnsettledStart {
            stage: "code".into(),
            start: 1,
            at: 54_000,
            unsettled: Unsettled {
                missing: vec!["AGENT_HOME".into()],
                ..Unsettled::default()
            },
        };
        assert_eq!(
            unsettled_line(&unsettled),
            "  without its settings: code's start 1 began at 54000 without AGENT_HOME, and was not taken as its stage's agent"
        );
    }

    #[test]
    fn a_start_in_another_mode() {
        // S67: a start that asked `all` and began in `ask-each`, named on
        // its own line with both modes and no variable (56, 161, 162).
        let unsettled = UnsettledStart {
            stage: "code".into(),
            start: 2,
            at: 54_000,
            unsettled: Unsettled {
                mode: Some(ModeBegan {
                    asked: "all".into(),
                    began: Some("ask-each".into()),
                }),
                ..Unsettled::default()
            },
        };
        assert_eq!(
            unsettled_line(&unsettled),
            "  without its settings: code's start 2 began at 54000 in the mode `ask-each`, where its start asked `all`, and was not taken as its stage's agent"
        );
    }

    #[test]
    fn a_carried_conversation() {
        assert_eq!(
            conversation_line(&Conversation {
                from: 1,
                carried: true,
                why: None,
            }),
            "    carries the conversation of start 1"
        );
    }

    #[test]
    fn a_conversation_that_stayed() {
        assert_eq!(
            conversation_line(&Conversation {
                from: 1,
                carried: false,
                why: Some("the chosen account's configuration home (`CLAUDE_CONFIG_DIR`) does not reach its conversation: its `projects` folder is not the stopped account's".into()),
            }),
            "    starts afresh after start 1: the chosen account's configuration home (`CLAUDE_CONFIG_DIR`) does not reach its conversation: its `projects` folder is not the stopped account's"
        );
    }

    #[test]
    fn a_carried_start_not_begun() {
        let uncarried = UncarriedStart {
            stage: "code".into(),
            start: 2,
            at: 54_000,
            words: "the start that carried its conversation ended before it said it started".into(),
        };
        assert_eq!(
            uncarried_line(&uncarried),
            "  carried and not begun: code's start 2 at 54000: the start that carried its conversation ended before it said it started, and its stage started afresh"
        );
    }

    #[test]
    fn a_start_in_another_conversation() {
        let unsettled = UnsettledStart {
            stage: "code".into(),
            start: 2,
            at: 54_000,
            unsettled: Unsettled {
                session: true,
                ..Unsettled::default()
            },
        };
        assert_eq!(
            unsettled_line(&unsettled),
            "  without its settings: code's start 2 began at 54000 in another conversation than the one its start resumes, and was not taken as its stage's agent"
        );
    }

    #[test]
    fn a_move_with_its_conversation_and_without() {
        let moved = |carried| {
            MoveStatus::of(&AccountMove {
                on: MoveOn::Stage {
                    stage: "code".into(),
                },
                from: "work-1".into(),
                to: "work-2".into(),
                why: MoveWhy::Spent,
                at: 54_000,
                carried: Some(carried),
                window: None,
                until: None,
            })
        };
        assert_eq!(
            moved(true).words(),
            "code moved from work-1 to work-2 at 54000, work-1 spent, its conversation with it, its agents after starting without work-1's cache"
        );
        assert_eq!(
            moved(false).words(),
            "code moved from work-1 to work-2 at 54000, work-1 spent, its conversation stayed behind, its agents after starting without work-1's cache"
        );
        assert_eq!(
            serde_json::to_value(moved(true)).unwrap()["carried"],
            json!(true)
        );
    }

    #[test]
    fn a_move() {
        let moved = MoveStatus::of(&AccountMove {
            on: MoveOn::Stage {
                stage: "code".into(),
            },
            from: "work-1".into(),
            to: "work-2".into(),
            why: MoveWhy::Spent,
            at: 54_000,
            carried: None,
            window: None,
            until: None,
        });
        assert_eq!(
            moved.words(),
            "code moved from work-1 to work-2 at 54000, work-1 spent, its agents after starting without work-1's cache"
        );
        assert_eq!(
            serde_json::to_value(&moved).unwrap(),
            json!({"on": "stage", "stage": "code", "from": "work-1", "to": "work-2", "why": "spent", "at": 54000, "cache": "lost"})
        );
    }

    #[test]
    fn an_intents_move_for_its_program_and_one_recorded_before() {
        let moved = |program: Option<&str>| {
            MoveStatus::of(&AccountMove {
                on: MoveOn::Intent {
                    program: program.map(str::to_string),
                },
                from: "agent-1".into(),
                to: "agent-2".into(),
                why: MoveWhy::Spent,
                at: 54_000,
                carried: None,
                window: None,
                until: None,
            })
        };
        assert_eq!(
            moved(Some("agent")).words(),
            "the intent moved for agent from agent-1 to agent-2 at 54000, agent-1 spent, its agents after starting without agent-1's cache"
        );
        assert_eq!(
            serde_json::to_value(moved(Some("agent"))).unwrap(),
            json!({"on": "intent", "program": "agent", "from": "agent-1", "to": "agent-2", "why": "spent", "at": 54000, "cache": "lost"})
        );
        assert_eq!(
            moved(None).words(),
            "the intent moved from agent-1 to agent-2 at 54000, agent-1 spent, its agents after starting without agent-1's cache"
        );
        assert_eq!(
            serde_json::to_value(moved(None)).unwrap(),
            json!({"on": "intent", "from": "agent-1", "to": "agent-2", "why": "spent", "at": 54000, "cache": "lost"})
        );
    }

    #[test]
    fn the_hosts_pools() {
        let host = HostStatus {
            pools: vec![PoolStatus {
                name: "work".into(),
                strategy: Strategy::FillFirst,
                threshold: 90,
                accounts: vec![
                    AccountStatus {
                        name: "work-1".into(),
                        program: "agent".into(),
                        running: 2,
                        bound: Some(4),
                        spent_until: None,
                        reading: None,
                        past: None,
                    },
                    AccountStatus {
                        name: "work-2".into(),
                        program: "agent".into(),
                        running: 0,
                        bound: None,
                        spent_until: Some(54_000),
                        reading: None,
                        past: None,
                    },
                ],
            }],
            ..HostStatus::default()
        };
        assert_eq!(
            host_lines(&host),
            vec![
                "host: 0 agents running, no bound".to_string(),
                "  pool work, fill-first, threshold 90%:".to_string(),
                "    work-1 (agent): 2 agents running of 4".to_string(),
                "    work-2 (agent): 0 agents running, no bound of its own, spent until 54000"
                    .to_string(),
                "  the terminal keeps no watch: places move on when a command of flume's acts or an agent goes quiet".to_string(),
            ]
        );
    }

    /// An unbounded account of `agent`, running none, with `reading` and
    /// `past`.
    fn read(name: &str, reading: Option<ReadingStatus>, past: Option<Past>) -> AccountStatus {
        AccountStatus {
            name: name.into(),
            program: "agent".into(),
            running: 0,
            bound: None,
            spent_until: None,
            reading,
            past,
        }
    }

    #[test]
    fn a_pools_threshold_and_its_accounts_readings() {
        let window = |name: &str, used, resets_at, past| WindowStatus {
            name: name.into(),
            used,
            resets_at,
            past,
        };
        let pool = PoolStatus {
            name: "work".into(),
            strategy: Strategy::MostLeft,
            threshold: 80,
            accounts: vec![
                read(
                    "work-1",
                    Some(ReadingStatus::Read {
                        read_at: 53_880,
                        fresh: true,
                        windows: vec![
                            window("5-hour", 4_200, Some(61_200), false),
                            window("7-day", 7_150, None, false),
                        ],
                    }),
                    None,
                ),
                read(
                    "work-2",
                    Some(ReadingStatus::Read {
                        read_at: 52_800,
                        fresh: false,
                        windows: vec![window("5-hour", 1_000, Some(61_200), false)],
                    }),
                    None,
                ),
                read(
                    "work-3",
                    Some(ReadingStatus::Unread {
                        unread: "the file is missing".into(),
                    }),
                    None,
                ),
                read(
                    "work-4",
                    Some(ReadingStatus::Read {
                        read_at: 54_000,
                        fresh: true,
                        windows: vec![window("7-day", 9_500, Some(313_200), true)],
                    }),
                    Some(Past {
                        window: "7-day".into(),
                        until: 313_200,
                    }),
                ),
                read("work-5", None, None),
            ],
        };
        assert_eq!(
            pool_lines(&pool),
            vec![
                "  pool work, most-left, threshold 80%:",
                "    work-1 (agent): 0 agents running, no bound of its own",
                "      5-hour 42% (resets 61200) · 7-day 71.5% · read at 53880",
                "    work-2 (agent): 0 agents running, no bound of its own",
                "      5-hour 10% (resets 61200) · stale, read at 52800",
                "    work-3 (agent): 0 agents running, no bound of its own",
                "      no reading: the file is missing",
                "    work-4 (agent): 0 agents running, no bound of its own",
                "      7-day 95% (resets 313200) · read at 54000",
                "      past its threshold in 7-day until 313200",
                "    work-5 (agent): 0 agents running, no bound of its own",
            ]
        );
    }

    #[test]
    fn a_move_off_an_account_past_its_threshold() {
        let moved = MoveStatus::of(&AccountMove {
            on: MoveOn::Intent {
                program: Some("agent".into()),
            },
            from: "work-1".into(),
            to: "work-2".into(),
            why: MoveWhy::Threshold,
            at: 54_000,
            carried: None,
            window: Some("5-hour".into()),
            until: Some(61_200),
        });
        assert_eq!(
            moved.words(),
            "the intent moved for agent from work-1 to work-2 at 54000, work-1 past its threshold in 5-hour until 61200, its agents after starting without work-1's cache"
        );
        assert_eq!(
            serde_json::to_value(&moved).unwrap(),
            json!({"on": "intent", "program": "agent", "from": "work-1", "to": "work-2", "why": "threshold", "at": 54000, "window": "5-hour", "until": 61200, "cache": "lost"})
        );
    }

    #[test]
    fn a_pool_waited_on_until_an_account_is_back() {
        assert_eq!(
            RoomWanted::Pool {
                pool: "work".into(),
                until: Some(61_200),
            }
            .words(),
            "a usable account of the pool work, the soonest back at 61200"
        );
    }

    #[test]
    fn a_refusal_and_an_empty_host() {
        assert_eq!(person(&Answer::refused("no")), "refused: no");
        let status = Answer::Status {
            places: vec![],
            host: HostStatus::default(),
        };
        assert_eq!(
            person(&status),
            "no place stands on this host\nhost: 0 agents running, no bound\n  the terminal keeps no watch: places move on when a command of flume's acts or an agent goes quiet"
        );
    }

    #[test]
    fn a_host_under_a_watch() {
        let host = HostStatus {
            watch: Watch::Bound { seconds: 30 },
            ..HostStatus::default()
        };
        assert_eq!(
            host_lines(&host)[1],
            "  the terminal keeps watch: every place settles every 30 seconds and when an agent's state changes"
        );
    }

    #[test]
    fn a_missing_key_under_the_watch() {
        // 72, 122: what the terminal lacks, under the line naming the watch.
        let key = "the terminal's /t/config.toml binds no key to flume's places".to_string();
        let host = HostStatus {
            watch: Watch::Bound { seconds: 30 },
            terminal_setup: vec![key.clone()],
            ..HostStatus::default()
        };
        let lines = host_lines(&host);
        assert_eq!(
            lines[lines.len() - 2..],
            [
                "  the terminal keeps watch: every place settles every 30 seconds and when an agent's state changes".to_string(),
                format!("    {key}"),
            ]
        );
    }

    fn a_line(name: &str, short: &str) -> PlaceLine {
        PlaceLine {
            name: name.into(),
            kind: "fix@1".into(),
            stage: Some("code".into()),
            run: Some(1),
            tasks: None,
            state: "working".into(),
            full: format!("{name} · fix@1 · {short}"),
            short: short.into(),
        }
    }

    #[test]
    fn a_line_each() {
        // 60, 61: each line's full form on a line of its own.
        let lines = Answer::Lines {
            lines: vec![
                a_line("fix--one", "code · run 1 · working"),
                a_line("fix--two", "code · run 2 · stuck"),
            ],
        };
        assert_eq!(
            person(&lines),
            "fix--one · fix@1 · code · run 1 · working\nfix--two · fix@1 · code · run 2 · stuck"
        );
    }

    #[test]
    fn a_places_heading_with_its_line() {
        // 56, 60: the place's line under its heading.
        let place = PlaceStatus {
            place: PlaceId::new("/d/places/1/fix--one"),
            repository: "/r".into(),
            kind: "fix".into(),
            version: 1,
            declared: Declared::Flume,
            name: "one".into(),
            branch: "fix/one".into(),
            base: "main".into(),
            base_from: None,
            mark: None,
            lands: true,
            phase: Phase::Live,
            readiness: Readiness::default(),
            stages: vec![],
            current: Some("code".into()),
            agent: None,
            head: None,
            delivered: None,
            tasks: None,
            submission: None,
            refused_acts: vec![],
            cannot_read: vec![],
            failures: vec![],
            recipients: vec![],
            hold_words: BTreeMap::new(),
            waiting_on: None,
            stalled: false,
            holds: vec![],
            priority: 0,
            stream: StreamHeld {
                name: "default".into(),
                from: StreamFrom::Default,
            },
            label: None,
            intent: IntentKey::Launch {
                name: "i".into(),
                digest: "d".into(),
            },
            moves: vec![],
            unsettled: vec![],
            uncarried: vec![],
            ends: vec![],
            role: None,
            clues: None,
            line: a_line("fix--one", "code · run 1 · working"),
        };
        assert_eq!(
            place_lines(&place)[..2],
            [
                "/d/places/1/fix--one: fix@1 (flume), live".to_string(),
                "  fix--one · fix@1 · code · run 1 · working".to_string(),
            ]
        );
    }

    #[test]
    fn a_repeating_stage_with_its_groups_place() {
        let stage = StageStatus {
            name: "code".into(),
            state: flume_seams::StageState::Begun,
            starts: 2,
            end: None,
            refused_part: None,
            refused_setting: None,
            conversation: false,
            group: Some(flume_core::GroupStatus {
                place: 2,
                of: 3,
                number: Some("2".into()),
                title: Some("Two".into()),
            }),
        };
        assert_eq!(
            stage_lines(&stage),
            vec!["  stage code 2/3: begun, started 2 times, on group 2. Two".to_string()]
        );
        let once = StageStatus {
            group: None,
            ..stage
        };
        assert_eq!(
            stage_lines(&once),
            vec!["  stage code: begun, started 2 times".to_string()]
        );
    }

    #[test]
    fn the_hosts_streams() {
        let queued = |place: &str, stream: &str, wants| Queued {
            place: flume_seams::PlaceId::new(place),
            stage: "code".into(),
            stream: stream.into(),
            label: None,
            priority: 0,
            since: Some(5),
            wants,
        };
        let work = queued(
            "/r/places/fix--w5",
            "work",
            RoomWanted::Stream {
                stream: "work".into(),
                bound: 4,
            },
        );
        let personal = queued("/r/places/fix--p3", "personal", RoomWanted::Host);
        let host = HostStatus {
            running: 6,
            bound: Some(6),
            room: Some(0),
            queue: vec![work.clone(), personal.clone()],
            streams: vec![
                StreamStatus {
                    name: "work".into(),
                    multiplexer: ProfileHeld {
                        profile: "printed".into(),
                        from: NamedBy::Host,
                    },
                    view: ViewHeld {
                        view: View::Terminal,
                        from: NamedBy::Neither,
                    },
                    terminal_app: ProfileHeld {
                        profile: "printed".into(),
                        from: NamedBy::Neither,
                    },
                    title: TitleHeld {
                        title: "work".into(),
                        from: TitleFrom::StreamName,
                    },
                    session: SessionHeld {
                        name: Some("office".into()),
                        from: SessionFrom::Stream,
                    },
                    priority: 10,
                    running: 4,
                    bound: Some(4),
                    room: Some(0),
                    queue: vec![work],
                    labels: vec![],
                    reserves: Reserves::None,
                    refused: None,
                    view_refused: None,
                },
                StreamStatus {
                    name: "personal".into(),
                    multiplexer: ProfileHeld {
                        profile: "printed".into(),
                        from: NamedBy::Host,
                    },
                    view: ViewHeld {
                        view: View::Terminal,
                        from: NamedBy::Neither,
                    },
                    terminal_app: ProfileHeld {
                        profile: "printed".into(),
                        from: NamedBy::Neither,
                    },
                    title: TitleHeld {
                        title: "personal".into(),
                        from: TitleFrom::StreamName,
                    },
                    session: SessionHeld {
                        name: Some("personal".into()),
                        from: SessionFrom::StreamName,
                    },
                    priority: 0,
                    running: 2,
                    bound: Some(3),
                    room: Some(1),
                    queue: vec![personal],
                    labels: vec![],
                    reserves: Reserves::None,
                    refused: None,
                    view_refused: None,
                },
                StreamStatus {
                    name: "default".into(),
                    multiplexer: ProfileHeld {
                        profile: "printed".into(),
                        from: NamedBy::Host,
                    },
                    view: ViewHeld {
                        view: View::Terminal,
                        from: NamedBy::Neither,
                    },
                    terminal_app: ProfileHeld {
                        profile: "printed".into(),
                        from: NamedBy::Neither,
                    },
                    title: TitleHeld {
                        title: "default".into(),
                        from: TitleFrom::StreamName,
                    },
                    session: SessionHeld {
                        name: None,
                        from: SessionFrom::Own,
                    },
                    priority: 0,
                    running: 0,
                    bound: None,
                    room: None,
                    queue: vec![],
                    labels: vec![],
                    reserves: Reserves::None,
                    refused: None,
                    view_refused: None,
                },
            ],
            ..HostStatus::default()
        };
        let w5 = "waiting for room: /r/places/fix--w5's code in work, priority 0, \
                  short of the stream work's room, at its bound of 4";
        let p3 = "waiting for room: /r/places/fix--p3's code in personal, priority 0, \
                  short of the host's room";
        assert_eq!(
            host_lines(&host),
            vec![
                "host: 6 agents running of 6, room for 0".to_string(),
                format!("  {w5}"),
                format!("  {p3}"),
                "  stream work, priority 10: 4 agents running of 4, room for 0".to_string(),
                "    its agents open in the session office, the stream's".to_string(),
                "    multiplexer printed (the host's), view terminal (the default), terminal app printed (the default), title \"work\" (the stream's name)".to_string(),
                format!("    {w5}"),
                "  stream personal, priority 0: 2 agents running of 3, room for 1".to_string(),
                "    its agents open in the session personal, the stream's name".to_string(),
                "    multiplexer printed (the host's), view terminal (the default), terminal app printed (the default), title \"personal\" (the stream's name)".to_string(),
                format!("    {p3}"),
                "  stream default, priority 0: 0 agents running, no bound of its own".to_string(),
                "    its agents open in the terminal's own session".to_string(),
                "    multiplexer printed (the host's), view terminal (the default), terminal app printed (the default), title \"default\" (the stream's name)".to_string(),
                "  the terminal keeps no watch: places move on when a command of flume's acts or an agent goes quiet".to_string(),
            ]
        );
    }

    #[test]
    fn a_places_base_with_where_it_was_read() {
        let lines: Vec<String> = [
            None,
            Some(BaseFrom::MergeQueue),
            Some(BaseFrom::Origin),
            Some(BaseFrom::NoOrigin),
            Some(BaseFrom::OriginLacks),
        ]
        .into_iter()
        .map(|from| base_words("main", from))
        .collect();
        assert_eq!(
            lines,
            vec![
                "main",
                "main (where its merge queue lands it)",
                "main (at origin)",
                "main (the host's own, the repository having no origin)",
                "main (the host's own, origin holding none)",
            ]
        );
    }

    #[test]
    fn a_places_label() {
        let stream = StreamHeld {
            name: "work".into(),
            from: flume_seams::StreamFrom::HostRepository,
        };
        let intent = IntentKey::Launch {
            name: "ship".into(),
            digest: "d1".into(),
        };
        let label = LabelHeld {
            name: "construction".into(),
            from: flume_seams::LabelFrom::HostRepositoryKind,
        };
        assert_eq!(
            stream_line(&stream, Some(&label), &intent),
            "  stream work (from host-repository), label construction (from host-repository-kind), intent ship (from the launch)"
        );
        assert_eq!(
            stream_line(&stream, None, &intent),
            "  stream work (from host-repository), intent ship (from the launch)"
        );
    }

    #[test]
    fn a_streams_labels() {
        let research = Queued {
            place: flume_seams::PlaceId::new("/r/places/research--one"),
            stage: "work".into(),
            stream: "work".into(),
            label: None,
            priority: 0,
            since: Some(5),
            wants: RoomWanted::KeptFree {
                stream: "work".into(),
                labels: vec!["operation".into()],
            },
        };
        let deploy = Queued {
            place: flume_seams::PlaceId::new("/r/places/deploy--two"),
            label: Some("operation".into()),
            wants: RoomWanted::Stream {
                stream: "work".into(),
                bound: 5,
            },
            ..research.clone()
        };
        let stream = |name: &str, bound, reserves, refused: Option<&str>| StreamStatus {
            name: name.into(),
            multiplexer: ProfileHeld {
                profile: "printed".into(),
                from: NamedBy::Host,
            },
            view: ViewHeld {
                view: View::Terminal,
                from: NamedBy::Neither,
            },
            terminal_app: ProfileHeld {
                profile: "printed".into(),
                from: NamedBy::Neither,
            },
            title: TitleHeld {
                title: name.into(),
                from: TitleFrom::StreamName,
            },
            session: SessionHeld {
                name: Some(name.into()),
                from: SessionFrom::StreamName,
            },
            priority: 0,
            running: 5,
            bound,
            room: bound.map(|_| 0),
            queue: vec![],
            labels: vec![flume_core::LabelStatus {
                name: "operation".into(),
                reserve: 2,
                keep_free: 1,
                running: 1,
                waiting: 1,
            }],
            reserves,
            refused: refused.map(str::to_string),
            view_refused: None,
        };
        let host = HostStatus {
            streams: vec![
                StreamStatus {
                    queue: vec![research, deploy],
                    ..stream("work", Some(5), Reserves::Held, None)
                },
                stream("personal", None, Reserves::PassedOver, None),
                stream(
                    "nightly",
                    Some(1),
                    Reserves::Held,
                    Some("the label operation of the stream nightly keeps 2 free of a reserve of 1, and a reserve is at least what it keeps free"),
                ),
            ],
            ..HostStatus::default()
        };
        let operation =
            "    label operation: a reserve of 2, keeping 1 free, 1 agents running, 1 waiting";
        assert_eq!(
            host_lines(&host)[1..host_lines(&host).len() - 1],
            [
                "  stream work, priority 0: 5 agents running of 5, room for 0".to_string(),
                "    its agents open in the session work, the stream's name".to_string(),
                "    multiplexer printed (the host's), view terminal (the default), terminal app printed (the default), title \"work\" (the stream's name)".to_string(),
                operation.to_string(),
                "    waiting for room: /r/places/research--one's work in work, priority 0, short of the stream work's room, the rest kept free for operation".to_string(),
                "    waiting for room: /r/places/deploy--two's work in work as operation, priority 0, short of the stream work's room, at its bound of 5".to_string(),
                "  stream personal, priority 0: 5 agents running, no bound of its own".to_string(),
                "    its agents open in the session personal, the stream's name".to_string(),
                "    multiplexer printed (the host's), view terminal (the default), terminal app printed (the default), title \"personal\" (the stream's name)".to_string(),
                operation.to_string(),
                "    its labels' reserves are passed over: it states no bound".to_string(),
                "  stream nightly, priority 0: 5 agents running of 1, room for 0".to_string(),
                "    its agents open in the session nightly, the stream's name".to_string(),
                "    multiplexer printed (the host's), view terminal (the default), terminal app printed (the default), title \"nightly\" (the stream's name)".to_string(),
                operation.to_string(),
                "    it starts no stage: the label operation of the stream nightly keeps 2 free of a reserve of 1, and a reserve is at least what it keeps free".to_string(),
            ]
        );
    }

    #[test]
    fn the_hosts_local_layer() {
        let value = |key: &str, value: &str, layer| ConfigValue {
            key: key.into(),
            value: value.into(),
            layer,
        };
        let values = vec![
            value(
                "pools.work.accounts[0].env.TOKEN",
                "<set>",
                ConfigLayer::Managed,
            ),
            value("streams.work.agents", "3", ConfigLayer::Local),
            value("streams.work.pool", "\"work\"", ConfigLayer::Managed),
            value("streams.work.priority", "30", ConfigLayer::Local),
        ];
        let lines = |configuration: HostConfiguration| configuration_lines(&configuration);
        assert_eq!(
            lines(HostConfiguration {
                local: LocalLayer::Taken,
                values: values.clone(),
                passed_over: vec![flume_seams::PinnedKey {
                    key: "streams.work.bound".into(),
                    pin: "streams.work".into(),
                }],
            }),
            vec![
                "  the host's local layer is taken, setting:".to_string(),
                "    streams.work.agents = 3".to_string(),
                "    streams.work.priority = 30".to_string(),
                "    streams.work.bound passed over: the managed layer pins streams.work"
                    .to_string(),
            ]
        );
        assert_eq!(
            lines(HostConfiguration {
                local: LocalLayer::Taken,
                values: values[..1].to_vec(),
                passed_over: vec![],
            }),
            vec!["  the host's local layer is taken, setting nothing".to_string()]
        );
        assert_eq!(
            lines(HostConfiguration {
                local: LocalLayer::NotTaken {
                    keys: vec!["pools.work".into(), "streams.work.pool".into()],
                    fault: None,
                },
                values: values.clone(),
                passed_over: vec![],
            }),
            vec!["  the host's local layer is not taken: it sets pools.work, streams.work.pool, which the local layer may not set".to_string()]
        );
        assert_eq!(
            lines(HostConfiguration {
                local: LocalLayer::NotTaken {
                    keys: vec![],
                    fault: Some("expected `=` at line 2".into()),
                },
                values: values.clone(),
                passed_over: vec![],
            }),
            vec!["  the host's local layer is not taken: expected `=` at line 2".to_string()]
        );
        assert_eq!(
            lines(HostConfiguration::default()),
            Vec::<String>::new(),
            "a host with no local file"
        );
    }

    #[test]
    fn readiness_gives_each_steps_outcome_and_a_failed_steps_last_lines() {
        let step = |step: &str, passed, words: &str| ReadyStep {
            step: step.into(),
            passed,
            words: words.into(),
        };
        let built: String = (1..=30)
            .map(|line| format!("\u{1b}[1;32mbuilding {line}\u{1b}[0m\n"))
            .collect();
        let failed = format!(
            "{built}\u{1b}]8;;https://example.org\u{7}a link\u{1b}]8;;\u{7}\n\
             fetching 50%\rfetching 100%\r\n\n\
             \u{1b}[31merror:\u{1b}[0m the lockfile is stale  \n"
        );
        let readiness = Readiness {
            steps: vec![
                step("toolchain", true, &built),
                step("moon setup", false, &failed),
            ],
        };
        let mut expected = vec![
            "  readiness toolchain: passed".to_string(),
            "  readiness moon setup: failed".to_string(),
        ];
        expected.extend((24..=30).map(|line| format!("    building {line}")));
        expected.extend([
            "    a link".to_string(),
            "    fetching 100%".to_string(),
            "    error: the lockfile is stale".to_string(),
        ]);
        assert_eq!(readiness_lines(&readiness), expected);
        let silent = Readiness {
            steps: vec![step("toolchain", false, "\u{1b}[0m\n\n")],
        };
        assert_eq!(
            readiness_lines(&silent),
            vec!["  readiness toolchain: failed".to_string()]
        );
    }

    #[test]
    fn a_wait_on_readiness_leaves_its_words_to_the_readiness_lines() {
        let held = WaitingOn {
            act: flume_seams::Act::Retry,
            stage: Some("code".into()),
            why: Waits::OnHold,
            since: Some(54_000),
            words: Some("Wait until the release is cut.".into()),
        };
        assert_eq!(
            waiting_line(&held, false),
            "  waits on its caller to retry code (on_hold) since 54000: Wait until the release is cut."
        );
        let unready = WaitingOn {
            stage: Some("construct".into()),
            why: Waits::Unready,
            words: Some("moon setup: \u{1b}[1mbuilding 1\u{1b}[0m\nerror: stale".into()),
            ..held
        };
        assert_eq!(
            waiting_line(&unready, true),
            "  waits on its caller to retry construct (unready) since 54000, stalled"
        );
    }

    #[test]
    fn a_wait_on_a_dialog_gives_its_lines_beneath() {
        let dialog = WaitingOn {
            act: Act::Answer,
            stage: Some("talk".into()),
            why: Waits::Dialog,
            since: Some(54_000),
            words: Some(
                "New MCP server found in .mcp.json: docs\n\u{1b}[1m\u{276f} 1. Use this and all future MCP servers\u{1b}[0m"
                    .into(),
            ),
        };
        assert_eq!(
            waiting_line(&dialog, false),
            "  waits on a person to answer the dialog its agent shows in talk (dialog) since 54000:\n    New MCP server found in .mcp.json: docs\n    \u{276f} 1. Use this and all future MCP servers"
        );
    }

    #[test]
    fn words_as_a_terminal_draws_them() {
        assert_eq!(
            plain("\u{1b}(B\u{1b}7a\u{8}\u{1b}]0;title\u{1b}\\b\u{1b}[?25l\tc\r\n"),
            "ab\tc\r\n"
        );
    }

    #[test]
    fn a_word_as_the_json_spells_it() {
        assert_eq!(word(json!("on_hold")), "on_hold");
        assert_eq!(word(json!({"live": {}})), "live");
        assert_eq!(
            word(json!({"over": {"over": "done"}})),
            r#"over {"over":"done"}"#
        );
    }
}
