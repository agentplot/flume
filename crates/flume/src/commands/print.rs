//! The core's answer printed for a person (61): every fact the JSON holds
//! that a person reads, in lines, and nothing the answer does not hold.
//! A word of the answer's own, a state or a layer, is printed as the JSON
//! spells it.

use flume_core::{
    group_name, percentage, AgentStatus, Answer, HostStatus, OwnStatus, PlaceStatus, PoolStatus,
    Queued, ReadingStatus, Reserves, SessionAnswer, SessionWait, SourceStatus, StageStatus,
    StandingFound, StandingStatus, StreamOpened, StreamStatus, Tidied, TitleFrom, WaitingPlace,
    WindowOutcome, Woke,
};
use flume_seams::{
    AccountFrom, Act, AnswerHeld, BaseFrom, CarryCause, ClosedBy, ConfigLayer, ConfigValue,
    Conversation, EndOf, FixEnded, Forecast, Given, Hold, HoldOn, HostConfiguration, IntentKey,
    Kept, KindFrom, LabelHeld, LiveProcess, LocalLayer, Made, NamedBy, NamedSession, Phase,
    ProfileHeld, Readiness, RecipientsFrom, Seconds, SessionFrom, SessionHeld, StopKind,
    StreamAnswerer, StreamHeld, SubmissionRead, UncarriedStart, UnsettledStart, WaitingOn, Waits,
    Watch, LAUNCH_SOURCE, OWN_SOURCE, REPO_SOURCE,
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
                let confirmed = end
                    .confirmed
                    .iter()
                    .any(|confirmed| confirmed.recipient == *recipient);
                let how = match (*delivered, confirmed) {
                    (true, true) => "handed over and confirmed by this wait".to_string(),
                    (true, false) => format!(
                        "handed over by this wait; pending until {recipient} confirms it{}",
                        confirm_words(end, recipient)
                    ),
                    (false, _) => "read; confirmed before".to_string(),
                };
                format!(
                    "{place} {} for {recipient}: {how}",
                    flume_core::what_ended(&end.end.of)
                )
            }
        }),
        Answer::Inbox {
            recipient,
            ends,
            waiting,
        } => {
            if ends.is_empty() {
                lines.push(format!("nothing pending for {recipient}"));
            }
            for collected in ends {
                let state = match (collected.confirmed, collected.expired) {
                    (true, _) => "; confirmed".to_string(),
                    (false, true) => {
                        format!("; expired{}", confirm_words(&collected.end, recipient))
                    }
                    (false, false) => confirm_words(&collected.end, recipient),
                };
                lines.push(format!(
                    "{}--{} {} [{}]; read: flume status {}{state}",
                    collected.kind,
                    collected.name,
                    flume_core::end_words(&collected.end.end.of),
                    collected.end.key,
                    collected.place
                ));
            }
            for place in waiting {
                lines.extend(waiting_place_lines(recipient, place));
            }
        }
        Answer::Confirmed {
            recipient,
            confirmed,
        } => {
            for key in confirmed {
                lines.push(if key.before {
                    format!("{} confirmed by {recipient} before, at {}", key.key, key.at)
                } else {
                    format!("{} confirmed by {recipient} at {}", key.key, key.at)
                });
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
        Answer::Held {
            address,
            standing,
            pending,
        } => {
            lines.push(format!(
                "{address} is held by {}, so nothing was launched; {}",
                standing_words(standing),
                pending_words(pending.len())
            ));
            for end in pending {
                lines.push(format!("  {}: {}", end.place, end.what));
            }
        }
        Answer::Locked {
            lock_file,
            recorded,
            removed,
        } => {
            for locked in recorded {
                lines.push(format!(
                    "recorded {} at {} in {lock_file}",
                    locked.kind, locked.hash
                ));
            }
            for kind in removed {
                lines.push(format!("removed {kind} from {lock_file}"));
            }
            if lines.is_empty() {
                lines.push(format!("{lock_file} is as it was"));
            }
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
    if let Some(named) = &opened.named {
        lines.push(format!(
            "  the stream names the session {} for {}",
            named.name,
            named_for(named)
        ));
    }
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
    lines.extend(kind_line(place.kind_from.as_ref()));
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
    if let Phase::Over {
        kept: Some(kept), ..
    } = &place.phase
    {
        lines.extend(session_runs_words(kept).map(|words| format!("  kept {words}")));
    }
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
        lines.extend(agent_lines(agent, &place.stream.name, &place.kind));
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
    if place.worktree == Some(false) {
        lines.push(base_place_line(
            place.on_branch.as_deref().unwrap_or_default(),
        ));
    }
    lines.extend(fix_lines(place));
    if let Some(waiting) = &place.waiting_on {
        lines.push(waiting_line(waiting, place.stalled));
        lines.extend(kept_open_line(waiting));
    }
    if let Some(wait) = &place.waits_on_session {
        lines.push(session_wait_line(wait));
    }
    for hold in &place.holds {
        lines.push(hold_line(hold));
    }
    if let Some(role) = &place.role {
        lines.push(format!("  the run of {role}"));
    }
    if !place.recipients.is_empty() {
        let from = place
            .recipients_from
            .as_ref()
            .map(|from| format!(", {}", recipients_from_words(from)))
            .unwrap_or_default();
        lines.push(format!(
            "  recipients: {}{from}",
            place.recipients.join(", ")
        ));
    }
    for end in &place.ends {
        let mut line = format!(
            "  ended {}: {}",
            end.key,
            flume_core::what_ended(&end.end.of)
        );
        line.push_str(&stall_words(&end.end.of));
        if let Some(answer) = &end.end.answer {
            line.push_str(&format!("; {}", answered_words(answer)));
        }
        let mut handed: Vec<&str> = vec![];
        for delivery in &end.end.deliveries {
            if !handed.contains(&delivery.recipient.as_str()) {
                handed.push(&delivery.recipient);
            }
        }
        if !handed.is_empty() {
            line.push_str(&format!("; handed to {}", handed.join(", ")));
        }
        if !end.pending_for.is_empty() {
            line.push_str(&format!("; pending for {}", end.pending_for.join(", ")));
        }
        lines.push(line);
        lines.extend(recipient_lines(end));
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

/// The kind's line: its identity, where it came from, its hash and the
/// lock that recorded it, as the launch read them; none for a place whose
/// held state keeps no `kind_from`, its first line giving the source from
/// `declared` alone (19, 21, 56; design 9).
fn kind_line(from: Option<&KindFrom>) -> Option<String> {
    let from = from?;
    let mut line = format!("  kind {}", from.identity.spelled);
    let at = |words: &str, given: Option<&String>| {
        given
            .map(|given| format!("{words}{given}"))
            .unwrap_or_default()
    };
    match from.source.as_str() {
        OWN_SOURCE => line.push_str(", flume's own"),
        REPO_SOURCE => line.push_str(&format!(
            " from the repository{}",
            at(" at ", from.revision.as_ref().map(|revision| &revision.0))
        )),
        LAUNCH_SOURCE => line.push_str(&match &from.declaration_file {
            Some(file) => format!(" from {file}"),
            None => " from the launch's declaration".to_string(),
        }),
        source => line.push_str(&format!(
            " from the source {source}{}{}",
            from.source_file
                .as_ref()
                .map(|file| format!(" ({file})"))
                .unwrap_or_default(),
            at(" at ", from.kind_dir.as_ref())
        )),
    }
    if let Some(hash) = &from.hash {
        line.push_str(&format!(", its hash {hash}"));
    }
    if let Some(lock) = &from.lock_file {
        line.push_str(&format!(", locked in {lock}"));
    }
    Some(line)
}

/// A source the host declares: its file and its directory, then the
/// kinds it holds by identity, or why the file or the directory cannot be
/// read (56, 191; design 9).
fn source_lines(source: &SourceStatus) -> Vec<String> {
    let mut line = format!("  source {} ({})", source.name, source.source_file);
    if let Some(kinds_dir) = &source.kinds_dir {
        line.push_str(&format!(" at {kinds_dir}"));
    }
    match &source.cannot_read {
        Some(fault) => line.push_str(&format!(": cannot be read: {fault}")),
        None if source.kinds.is_empty() => line.push_str(": holds no kind"),
        None => line.push_str(&format!(": {}", source.kinds.join(", "))),
    }
    let mut lines = vec![line];
    let file = source
        .roles_file
        .as_ref()
        .map(|file| format!(" ({file})"))
        .unwrap_or_default();
    match &source.roles_cannot_read {
        Some(fault) => lines.push(format!("    its roles{file}: cannot be read: {fault}")),
        None if !source.roles.is_empty() => {
            lines.push(format!("    its roles{file}: {}", source.roles.join(", ")))
        }
        None if source.roles_file.is_some() => {
            lines.push(format!("    its roles{file}: declares none"))
        }
        None => {}
    }
    for refused in &source.roles_refused {
        lines.push(format!(
            "    declares nothing for {}: {}",
            refused.address, refused.reason
        ));
    }
    lines
}

/// Where a launch took a place's recipients from, as the person's form
/// gives it (7, 56, 193).
fn recipients_from_words(from: &RecipientsFrom) -> String {
    match from {
        RecipientsFrom::Launch => "from the launch".into(),
        RecipientsFrom::Environment => "from $FLUME_RECIPIENT".into(),
        RecipientsFrom::Session => "from the session that launched it".into(),
        RecipientsFrom::Answerer { source } => {
            format!("from the stream's answerer, declared by {source}")
        }
        RecipientsFrom::BasePlace => "from the base place it fixes".into(),
    }
}

/// Who stands behind a stream, the one role with its source or the roles
/// that each declare to; none where no role does (143, 193).
fn answerer_line(answerer: &StreamAnswerer) -> String {
    match answerer {
        StreamAnswerer::Stands(by) => {
            format!(
                "    its answerer: {}, declared by {}",
                by.address, by.source
            )
        }
        StreamAnswerer::Contested { roles } => format!(
            "    no one answerer: {} each stand behind it",
            roles
                .iter()
                .map(|by| format!("{}, declared by {}", by.address, by.source))
                .collect::<Vec<_>>()
                .join(" and ")
        ),
    }
}

/// A base place's line: it has no worktree, and the places live on its
/// branch, which it waits on while any is (56, 202, 203).
fn base_place_line(on_branch: &[flume_seams::PlaceId]) -> String {
    if on_branch.is_empty() {
        return "  no worktree; no place on its branch is live".into();
    }
    format!(
        "  no worktree; waits on the places on its branch: {}",
        on_branch
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

/// A base place's fixes in the person's form: the kind they run as, with
/// where it came from, and how many of its bound were launched; the
/// newest forecast a settle held; and each fix with its cause and how it
/// ended; and, for a fix, the base place flume launched it for (56, 203).
fn fix_lines(place: &PlaceStatus) -> Vec<String> {
    let mut lines = vec![];
    if let Some(fix_kind) = &place.fix_kind {
        let kind = kind_line(Some(&fix_kind.from))
            .and_then(|line| line.strip_prefix("  kind ").map(str::to_string))
            .unwrap_or_else(|| fix_kind.identity.spelled.clone());
        let bound = place
            .fix_limit
            .map(|limit| format!(", {} of {limit} fixes", place.fixes.len()))
            .unwrap_or_default();
        lines.push(format!("  fix kind {kind}{bound}"));
    }
    if let Some(held) = &place.forecast {
        let forecast = match &held.forecast {
            Forecast::Clear { onto, .. } => format!("forecast clear at {onto}"),
            Forecast::Conflict {
                onto, files, with, ..
            } if with.is_empty() => format!(
                "forecast conflicts on {} with {} at {onto}",
                files.join(", "),
                place.base
            ),
            Forecast::Conflict {
                onto, files, with, ..
            } => format!(
                "forecast conflicts on {} with {} queued ahead, {} at {onto}",
                files.join(", "),
                with.join(", "),
                place.base
            ),
            Forecast::CouldNotRead { reason } => format!("forecast unread: {reason}"),
        };
        lines.push(format!("  {forecast}, read at {}", held.at));
    }
    let kind = place
        .fix_kind
        .as_ref()
        .map(|fix_kind| fix_kind.identity.name.as_str());
    for fix in &place.fixes {
        let name = kind.map_or_else(
            || fix.place.to_string(),
            |kind| format!("{kind}--{}", fix.name),
        );
        let ended = match &fix.ended {
            FixEnded::Live => "live".to_string(),
            FixEnded::Landed { commit } => format!("landed as {commit}"),
            FixEnded::Dropped => "dropped".to_string(),
            FixEnded::Refused { reason } => format!("not launched: {reason}"),
        };
        lines.push(format!(
            "  fix {} {name}, for {}, launched at {}: {ended}",
            fix.number,
            flume_core::cause_words(&fix.cause),
            fix.launched
        ));
    }
    if let Some(of) = &place.fix_of {
        let base = std::path::Path::new(&of.place.to_string())
            .file_name()
            .map_or_else(
                || of.place.to_string(),
                |name| name.to_string_lossy().into_owned(),
            );
        lines.push(format!(
            "  launched by flume for {base} (fix {}, {})",
            of.number,
            flume_core::cause_words(&of.cause)
        ));
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

/// A place the inbox gives as waiting on `recipient`: the place, its
/// brief's first line and its stream, then the move it waits on and since
/// when, the stop-short the move answers by its key, whether it needs an
/// answer or found work and the address it named, its agent kept open or
/// let go, and the hold on the move (43, 130, 193; design 9).
fn waiting_place_lines(recipient: &str, place: &WaitingPlace) -> Vec<String> {
    let brief = place
        .brief
        .as_ref()
        .map(|brief| format!(" ({brief})"))
        .unwrap_or_default();
    let mut lines = vec![
        format!(
            "{}--{}{brief}, in {}, waits on {recipient}; read: flume status {}",
            place.kind, place.name, place.stream, place.place
        ),
        waiting_line(&place.waiting_on, false),
    ];
    if let Some(stop_short) = &place.stop_short {
        let needs = match stop_short.needs {
            Some(StopKind::FoundWork) => "found work",
            _ => "needs an answer",
        };
        let mut line = format!("  its stop-short {} {needs}", stop_short.key);
        if let Some(addressee) = &stop_short.addressee {
            line.push_str(&format!(", asking {addressee}"));
        }
        match &stop_short.words {
            Some(words) if place.waiting_on.words.as_ref() != Some(words) => {
                line.push_str(&format!(": {words}"));
            }
            _ => {}
        }
        lines.push(line);
    }
    lines.extend(kept_open_line(&place.waiting_on));
    lines.extend(place.hold.as_ref().map(hold_line));
    lines
}

/// A hold's line: what it holds, since when and its words; for the move
/// the place waits on, the bound it holds for and when it lapsed, a
/// lapsed hold holding nothing (47, 56, 130).
fn hold_line(hold: &Hold) -> String {
    let what = match &hold.on {
        HoldOn::Stage { stage } => stage.as_str(),
        HoldOn::Submission => "the submission",
        HoldOn::Waiting => "the move it waits on",
    };
    let mut line = format!("  {what} on hold since {}", hold.since);
    if let Some(bound) = hold.bound {
        line.push_str(&format!(", for {bound} seconds"));
    }
    if let Some(lapsed) = hold.lapsed {
        line.push_str(&format!(", lapsed at {lapsed}"));
    }
    if let Some(words) = &hold.words {
        line.push_str(&format!(": {words}"));
    }
    line
}

/// What a stall carries beside the move, for its line: the words of the
/// hold whose lapse stalled it, and the stop-short the place waits on,
/// whether it needs an answer or found work, the role it named and its
/// words; nothing for any other end (41, 56, 130, 193).
fn stall_words(of: &EndOf) -> String {
    let EndOf::Stall {
        stop_short, lapsed, ..
    } = of
    else {
        return String::new();
    };
    let mut words = String::new();
    if let Some(lapsed) = lapsed {
        words.push_str(&format!(", its hold lapsed: {lapsed}"));
    }
    if let Some(stop_short) = stop_short {
        let needs = match stop_short.needs {
            Some(StopKind::FoundWork) => "found work",
            _ => "needs an answer",
        };
        words.push_str(&format!("; its stop-short {needs}"));
        if let Some(addressee) = &stop_short.addressee {
            words.push_str(&format!(", asking {addressee}"));
        }
        if let Some(said) = &stop_short.words {
            words.push_str(&format!(", saying: {said}"));
        }
    }
    words
}

/// For a place waiting on an answer, its agent kept open for it and until
/// when, or when and why it was let go and whether it is ended, left in
/// its pane or to be ended once quiet; none for any other move, and for
/// an answer whose agent was never kept open (130, 198).
fn kept_open_line(waiting: &WaitingOn) -> Option<String> {
    if let Some(until) = waiting.open_until {
        return Some(format!(
            "    its agent kept open for the answer until {until}: retry with --words, or tell it"
        ));
    }
    let closed = waiting.closed?;
    let why = match closed.by {
        ClosedBy::Time => "its time passed",
        ClosedBy::AgentEnded => "its agent ended",
        ClosedBy::AccountSpent => "its account spent",
        ClosedBy::Stopped => "its caller stopped it",
    };
    let ended = match (closed.ended_at, closed.left_in_pane) {
        (Some(at), _) => format!("and ended at {at}"),
        (None, true) => "and left in its pane".to_string(),
        (None, false) => "and ended once quiet".to_string(),
    };
    Some(format!(
        "    its agent let go at {}, {why}, {ended}: retry with --words",
        closed.at
    ))
}

/// A stage waiting for room while a process runs the conversation its
/// start would carry, since when, and each process by its id and start,
/// or what flume could not read (49, 56, 207; core.30).
fn session_wait_line(wait: &SessionWait) -> String {
    let mut line = format!("  {} waits for its conversation to end", wait.stage);
    if let Some(since) = wait.since {
        line.push_str(&format!(" since {since}"));
    }
    format!(
        "{line}: {}",
        live_words(&wait.processes, wait.unread.as_deref())
    )
}

/// Each process that runs a conversation, by its id and when it started,
/// and what flume could not read of whether one does, naming no path
/// (207; core.30).
fn live_words(processes: &[LiveProcess], unread: Option<&str>) -> String {
    let mut words: Vec<String> = processes
        .iter()
        .map(|process| format!("process {}, started {}", process.pid, process.started))
        .collect();
    if let Some(unread) = unread {
        words.push(format!(
            "flume cannot read whether a process runs it: {unread}"
        ));
    }
    words.join("; ")
}

/// A place whose work is over kept while a process runs its last
/// conversation start's session: each process, or what flume could not
/// read; none for a place kept otherwise (64, 207).
fn session_runs_words(kept: &Kept) -> Option<String> {
    match kept {
        Kept::SessionRuns { processes, unread } => Some(format!(
            "while its session runs: {}",
            live_words(processes, unread.as_deref())
        )),
        _ => None,
    }
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

/// The current agent of a place of `kind` in `stream`: its line, where
/// it opened, its terminal, each setting with its layer, its own
/// settings' files and how they were passed, and what its record says
/// (56, 190, 195, 199).
fn agent_lines(agent: &AgentStatus, stream: &str, kind: &str) -> Vec<String> {
    let mut lines = vec![agent_line(agent)];
    lines.extend(where_line(agent, stream, kind));
    lines.push(terminal_line(agent));
    for (setting, given) in &agent.settings.from {
        lines.push(setting_line(setting, given));
    }
    if let Some(own) = &agent.own {
        lines.extend(own_lines(own));
    }
    if let Some(conversation) = &agent.conversation {
        lines.push(conversation_line(conversation));
    }
    if agent.left_in_pane {
        lines.push(LEFT_IN_PANE.to_string());
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
    lines
}

/// The widest line an own setting's value is printed whole on in the
/// person's form; past it a table prints as `{…}` and a list as `[…]`,
/// and the JSON form gives every value whole (56, 195).
const SETTING_LINE: usize = 100;

/// A setting with its value as written, its layer, and the file it came
/// from where it came from one, an own setting's table or list that runs
/// past [`SETTING_LINE`] printed as `{…}` or `[…]` (56, 195).
fn setting_line(setting: &str, given: &Given) -> String {
    let layer = word(json!(given.layer));
    let from = match &given.file {
        Some(file) => format!("from {layer}, the file {file}"),
        None => format!("from {layer}"),
    };
    let line = format!("    {setting} = {} ({from})", given.value);
    let elided = match given.value.chars().next() {
        Some('{') => "{…}",
        Some('[') => "[…]",
        _ => return line,
    };
    if !setting.starts_with("programs.") || line.chars().count() <= SETTING_LINE {
        return line;
    }
    format!("    {setting} = {elided} ({from})")
}

/// An agent's own settings beyond their keys: each file a layer named for
/// its program with the keys left out of it and their owners, how they
/// were passed, and that they were not read back (56, 195).
fn own_lines(own: &OwnStatus) -> Vec<String> {
    let mut lines = vec![];
    for file in &own.files {
        lines.push(format!(
            "    the file {} (from {})",
            file.read,
            word(json!(file.layer))
        ));
        for left in &file.left_out {
            lines.push(format!(
                "      {} left out, owned by {}",
                left.key, left.owner
            ));
        }
    }
    if let Some(passed) = &own.passed {
        lines.push(format!("    own settings passed as {passed}"));
    }
    if !own.read_back {
        lines.push("    own settings not read back".to_string());
    }
    lines
}

/// The current agent's line: its identity, program, account and state,
/// the account from the intent's affinity for the agent's program, the
/// strategy, or the account a lost conversation's start ran under, its
/// conversation's, its mode as written with its layer, the entry it took of
/// a table and the value passed, where a layer gave one, and its effort
/// as written with its layer, the value passed and the account's variable
/// that outranks it, by its name alone, where a layer gave one (56, 145,
/// 147, 169, 197).
fn agent_line(agent: &AgentStatus) -> String {
    let mut line = format!("  agent {} ({})", agent.identity, agent.program);
    if let Some(account) = &agent.account {
        let from = match account.from {
            AccountFrom::Affinity => format!("from the affinity for {}", agent.program),
            AccountFrom::Strategy => "from the strategy".to_string(),
            AccountFrom::Conversation => "its conversation's".to_string(),
        };
        line.push_str(&format!(
            " on the account {} of {} ({from})",
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
    if let Some(effort) = &agent.effort {
        line.push_str(&format!(
            ", at the effort {} (from {}) passed as {}",
            effort.word,
            word(json!(effort.layer)),
            effort.passed
        ));
        if !effort.read_back {
            line.push_str(", not read back");
        }
        if let Some(variable) = &effort.outranked_by {
            line.push_str(&format!("; the account's {variable} outranks it"));
        }
    }
    line
}

/// Where the agent of a place of `kind` in `stream` opened: the session
/// with where it came from, its stream's word for its kind or its stage
/// naming the stream and the kind or the `<kind>.<stage>`, or the pane it
/// opened beside; none for a start written before (56, 107, 199).
fn where_line(agent: &AgentStatus, stream: &str, kind: &str) -> Option<String> {
    let named_for = |session: &SessionHeld| {
        let name = session.name.as_deref()?;
        let named_for = match session.from {
            SessionFrom::StreamKind => format!("kind {kind}"),
            SessionFrom::StreamStage => format!("stage {kind}.{}", agent.stage),
            _ => return None,
        };
        Some(format!(
            "    in the session {name} (stream {stream}, {named_for})"
        ))
    };
    match (&agent.session, &agent.settings.settings.beside) {
        (Some(session), _) => {
            named_for(session).or_else(|| Some(format!("    in {}", session_words(session))))
        }
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
        TitleFrom::Session => "the session's name",
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

/// Each session a stream names for its kinds and their stages, with each
/// kind and `<kind>.<stage>` it names it for and the layer naming it, then
/// each fault of those tables with the refusal it causes the starts that
/// read it (56, 199, C.7).
fn named_lines(stream: &StreamStatus) -> Vec<String> {
    let mut lines: Vec<String> = stream
        .sessions
        .iter()
        .map(|session| {
            format!(
                "    it names the session {} for {}",
                session.name,
                named_for(session)
            )
        })
        .collect();
    for fault in &stream.faults {
        lines.push(format!(
            "    it starts no stage that reads {}: {}",
            fault.table, fault.refused
        ));
    }
    lines
}

/// The kinds and stages a stream names a session for, each with its
/// layer: `the kind research (from managed) and the stage
/// release.review (from local)` (56, 199, C.7).
fn named_for(session: &NamedSession) -> String {
    let kinds = session
        .kinds
        .iter()
        .map(|kind| format!("the kind {} (from {})", kind.name, word(json!(kind.layer))));
    let stages = session.stages.iter().map(|stage| {
        format!(
            "the stage {} (from {})",
            stage.name,
            word(json!(stage.layer))
        )
    });
    let named: Vec<String> = kinds.chain(stages).collect();
    and_list(&named)
}

/// `items` as a person reads a list: `a`, `a and b`, `a, b and c`.
fn and_list(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// Where a standing agent lives and what its stream's terminal found of
/// it: `the standing agent madswan-planner in the session madswan (the
/// entry's), found waiting`; none of its name there, several, or why the
/// terminal could not read (56, 132, 190).
fn standing_words(standing: &StandingStatus) -> String {
    let session = match &standing.session.name {
        Some(name) => {
            let from = match standing.session.from {
                SessionFrom::Role => "the entry's".to_string(),
                SessionFrom::Stream => format!("the stream {}'s", standing.stream),
                SessionFrom::StreamName => format!("the stream {}'s name", standing.stream),
                from => session_from(from).to_string(),
            };
            format!("the session {name} ({from})")
        }
        None => format!(
            "the terminal's own session (the stream {}'s)",
            standing.stream
        ),
    };
    let found = match &standing.found {
        StandingFound::One { state, .. } => format!("found {}", word(json!(state))),
        StandingFound::None => "none of that name there".to_string(),
        StandingFound::Several { count } => {
            format!("{count} of that name there, so none takes a notice")
        }
        StandingFound::Unread { reason } => format!("not read: {reason}"),
    };
    format!(
        "the standing agent {} in {session}, {found}",
        standing.agent
    )
}

/// How many ends stand pending for an address: `nothing pending`, `1 end
/// pending`, `3 ends pending` (56, 132).
fn pending_words(count: usize) -> String {
    match count {
        0 => "nothing pending".to_string(),
        1 => "1 end pending".to_string(),
        count => format!("{count} ends pending"),
    }
}

/// Where a session's name came from (56, 107, 143, 199).
fn session_from(from: SessionFrom) -> &'static str {
    match from {
        SessionFrom::Launch => "the launch's",
        SessionFrom::StreamStage => "the stream's for its stage",
        SessionFrom::StreamKind => "the stream's for its kind",
        SessionFrom::Stream => "the stream's",
        SessionFrom::StreamName => "the stream's name",
        SessionFrom::Own => "the terminal's own",
        SessionFrom::Role => "the entry's",
    }
}

/// A pool's lines: its strategy and threshold, then each account's
/// program, agents running, bound and spent time, under it the words in
/// which no reset was read where that time is the hour, its reading, and
/// what holds it past its threshold; never a reading's path (56, 144 to
/// 146, 184, 185).
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
        if let Some(words) = &account.spent_unread {
            lines.push(format!("      spent an hour: no reset read in \"{words}\""));
        }
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
/// reset, then when flume took it and from which agent, stale where it is
/// not fresh; or why there is none (56, 184, 185).
fn reading_line(reading: &ReadingStatus) -> String {
    match reading {
        ReadingStatus::Read {
            read_at,
            agent,
            fresh,
            windows,
            ..
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
                format!("taken at {read_at} from {agent}")
            } else {
                format!("stale, taken at {read_at} from {agent}")
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
        if let Some(addressee) = &end.addressee {
            line.push_str(&format!(", asking {addressee}"));
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
    for answer in &stage.answers {
        lines.push(answer_line(answer));
    }
    for made in stage.end.iter().flat_map(|end| &end.made) {
        let line = match made {
            Made::Thing(thing) => json!(thing).to_string(),
            Made::Text(text) => text.clone(),
        };
        lines.push(format!("    made: {line}"));
    }
    lines
}

/// An answer a stage's start took in its session: the act it came by,
/// who gave it where it named one, when it was received, and the question
/// then the answer (42, 56).
fn answer_line(answer: &SessionAnswer) -> String {
    let from = answer
        .answer
        .by
        .as_ref()
        .map(|by| format!(" from {by}"))
        .unwrap_or_default();
    let received = answer
        .answer
        .session
        .map(|handed| format!(", received at {}", handed.received_at))
        .unwrap_or_default();
    format!(
        "    answered by {}{from}{received}: asked {}; answered {}",
        word(json!(answer.answer.act)),
        answer.question.as_deref().unwrap_or_default(),
        answer.answer.words
    )
}

/// A stop-short's answer on its end's line: when it was taken, who gave
/// it where it named one, the act it came by, whether it named the key,
/// and its words (42, 56).
fn answered_words(answer: &AnswerHeld) -> String {
    let by = answer
        .by
        .as_ref()
        .map(|by| format!(" by {by}"))
        .unwrap_or_default();
    let keyed = if answer.key.is_some() {
        " naming its key"
    } else {
        ""
    };
    format!(
        "answered at {}{by}, by a {}{keyed}: {}",
        answer.at,
        word(json!(answer.act)),
        answer.words
    )
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
        lines.extend(named_lines(stream));
        lines.push(display_line(stream));
        lines.extend(stream.answerer.as_ref().map(answerer_line));
        lines.extend(label_lines(stream));
        for queued in &stream.queue {
            lines.push(format!("    {}", queued_line(queued)));
        }
    }
    for pool in &host.pools {
        lines.extend(pool_lines(pool));
    }
    for source in &host.sources {
        lines.extend(source_lines(source));
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
    }
    for address in &host.addresses {
        if let Some(standing) = &address.standing {
            lines.push(format!(
                "  {}: {}; {}",
                address.address,
                standing_words(standing),
                pending_words(address.pending.len())
            ));
            for end in &address.pending {
                lines.push(format!("    {}: {}", end.place, end.what));
            }
            continue;
        }
        let run = match (&address.run, &address.role, &address.carried) {
            (Some(run), _, _) => format!("its run {run}"),
            (None, Some(role), _) => {
                format!(
                    "no run; it wakes {} in {}",
                    role.run.kind, role.run.repository
                )
            }
            (None, None, Some(carried)) => format!(
                "no run; {} declares it carried, its run on another host",
                carried.source
            ),
            (None, None, None) => match &address.wakes_nothing {
                Some(why) => format!("no run; it wakes nothing: {why}"),
                None => "no run; no source declares it".to_string(),
            },
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
        if let Some(stall) = &address.stalled {
            lines.push(format!(
                "    its run {} {}, which reached no one",
                stall.place, stall.what
            ));
        }
    }
    lines.push(watch_line(host));
    for setup in &host.terminal_setup {
        lines.push(format!("    {setup}"));
    }
    for end in &host.expired {
        let never = if end.known { "" } else { ", never confirmed" };
        lines.push(format!(
            "  expired: {} {} for {} since {}{never}; confirm: {}",
            end.place,
            end.key,
            end.recipient,
            end.expires_at,
            flume_core::confirm_command(&end.recipient, &[end.delivery.as_str()])
        ));
    }
    lines
}

/// Under an end, each of its recipients: pending, and expired since when
/// where its span passed, or confirmed and when; a name never confirmed
/// under said so (43, 56; weir 104, 106).
fn recipient_lines(end: &flume_core::EndStatus) -> Vec<String> {
    let mut lines = vec![];
    for owed in &end.owed {
        let expired = match (owed.expired, end.expires_at) {
            (true, Some(at)) => format!(", expired since {at}"),
            (true, None) => ", expired".to_string(),
            (false, _) => String::new(),
        };
        let never = if owed.known { "" } else { ", never confirmed" };
        lines.push(format!("    {}: pending{expired}{never}", owed.recipient));
    }
    for confirmed in &end.confirmed {
        lines.push(format!(
            "    {}: confirmed at {}",
            confirmed.recipient, confirmed.at
        ));
    }
    lines
}

/// How `recipient` confirms the end: `; confirm: flume confirm
/// --recipient <recipient> <key>`, none where the end has no delivery
/// (43).
fn confirm_words(end: &flume_core::EndStatus, recipient: &str) -> String {
    end.delivery
        .as_ref()
        .map(|key| {
            format!(
                "; confirm: {}",
                flume_core::confirm_command(recipient, &[key.as_str()])
            )
        })
        .unwrap_or_default()
}

/// Whether a caller keeps watch over the host, with its bound and when
/// its newest settle began, or since when none has (56, 119, 208).
fn watch_line(host: &HostStatus) -> String {
    match (host.watch, host.watch_lapsed) {
        (Watch::Bound { seconds }, _) => format!(
            "  a caller keeps watch: every place settles every {}, last at {}",
            duration(seconds),
            host.watch_settled
                .map_or_else(|| "an unknown time".to_string(), |at| at.to_string())
        ),
        (Watch::None, Some(lapsed)) => format!(
            "  no caller keeps watch since {lapsed}: every place settles at each command of flume's that acts"
        ),
        (Watch::None, None) => {
            "  no caller keeps watch: every place settles at each command of flume's that acts"
                .into()
        }
    }
}

/// A duration in its largest whole unit, as `flume wait --timeout` and
/// `flume settle --every` take one: `2m`, `1h`, `90s`.
fn duration(seconds: Seconds) -> String {
    match seconds {
        0 => "0s".into(),
        _ if seconds.is_multiple_of(3600) => format!("{}h", seconds / 3600),
        _ if seconds.is_multiple_of(60) => format!("{}m", seconds / 60),
        _ => format!("{seconds}s"),
    }
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
/// start whose agent stopped at its account's usage limit or was lost,
/// the latter said, or why not, naming no path (56, 146, 179; core.30).
fn conversation_line(conversation: &Conversation) -> String {
    let lost = match conversation.cause {
        CarryCause::Lost => ", lost",
        CarryCause::Spent => "",
    };
    match &conversation.why {
        None if conversation.carried => {
            format!(
                "    carries the conversation of start {}{lost}",
                conversation.from
            )
        }
        why => format!(
            "    starts afresh after start {}{lost}: {}",
            conversation.from,
            why.as_deref().unwrap_or("its conversation stayed behind")
        ),
    }
}

/// The agent of a conversation's stage read lost that the terminal
/// brought back in its last start's pane, left there (77; core.10).
const LEFT_IN_PANE: &str = "    brought back by the terminal in its start's pane after its stage was lost, and left there: flume sends it nothing, and only its caller's stop, drop or done ends it";

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

/// A stream's labels, each as `label <name>: owed <n>, reserved <m>, <r>
/// agents running, <w> waiting`; `what its labels are owed is passed
/// over: it states no bound` where it states none; and what it refuses
/// (56, 181, 182).
fn label_lines(stream: &StreamStatus) -> Vec<String> {
    let mut lines: Vec<String> = stream
        .labels
        .iter()
        .map(|label| {
            format!(
                "    label {}: owed {}, reserved {}, {} agents running, {} waiting",
                label.name, label.owed, label.reserved, label.running, label.waiting
            )
        })
        .collect();
    if stream.reserves == Reserves::PassedOver {
        lines.push("    what its labels are owed is passed over: it states no bound".into());
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
        lines.push(match session_runs_words(&kept.why) {
            Some(words) => format!("kept {} {words}", kept.place),
            None => format!("kept {}: {}", kept.place, word(json!(kept.why))),
        });
    }
    for branch in &tidy.branches_kept {
        lines.push(match &branch.reason {
            Some(reason) => format!(
                "kept the branch {} of {}, the merge queue keeping it: {reason}",
                branch.branch, branch.place
            ),
            None => format!(
                "kept the branch {} of {}, {} commits not landed",
                branch.branch,
                branch.place,
                branch.commits.len()
            ),
        });
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
    use flume_core::EndStatus;
    use flume_core::{
        AccountStatus, AddressStatus, AgentSettings, EffortStatus, Listed, ModeStatus, MoveStatus,
        OwnFileStatus, OwnStatus, Past, PendingEnd, PlaceLine, RoomWanted, StreamListed,
        StreamStatus, TitleFrom, TitleHeld, WindowStatus,
    };
    use flume_seams::{
        AccountFrom, AccountHeld, AccountMove, AgentState, AnswerHeld, BaseFix, Declared, EndHeld,
        EndOf, FixCause, FixKind, FixOf, ForecastHeld, Given, Handed, Layer, ModeBegan, MoveOn,
        MoveWhy, NamedBy, Phase, PlaceId, ProfileHeld, Reading, ReadyStep, Reserved, SessionClosed,
        SessionRuns, StageEnd, StageState, StopShortWaited, Strategy, StreamFrom, StreamHeld,
        Unsettled, View, ViewHeld,
    };
    use std::collections::BTreeMap;

    #[test]
    fn an_agents_account() {
        let agent = AgentStatus {
            left_in_pane: false,
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
            own: None,
            effort: None,
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
        // 56, 107, 199: where each agent opened, a session its stream
        // names for its kind or its stage naming the stream and the kind
        // or the stage, as design 9 words it.
        let agent = |session: Option<SessionHeld>, beside: Option<&str>| AgentStatus {
            stage: "review".into(),
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
            (
                named("work-design", SessionFrom::StreamKind),
                None,
                Some("    in the session work-design (stream work, kind release)"),
            ),
            (
                named("work-design", SessionFrom::StreamStage),
                None,
                Some("    in the session work-design (stream work, stage release.review)"),
            ),
            (own, None, Some("    in the terminal's own session")),
            (None, Some("operator"), Some("    beside the pane operator")),
            (None, None, None),
        ] {
            assert_eq!(
                where_line(&agent(session, beside), "work", "release").as_deref(),
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
            named: None,
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
    fn the_opened_lines_of_a_session_named_for_a_kind_and_a_stage() {
        // 186, 199: a session the stream names for a kind and a stage,
        // shown under its own name, with the kinds and stages it is
        // named for, each with its layer.
        let named = StreamOpened {
            session: SessionHeld {
                name: Some("work-design".into()),
                from: SessionFrom::StreamKind,
            },
            named: Some(NamedSession {
                name: "work-design".into(),
                kinds: vec![flume_seams::NamedFor {
                    name: "research".into(),
                    layer: ConfigLayer::Managed,
                }],
                stages: vec![flume_seams::NamedFor {
                    name: "release.review".into(),
                    layer: ConfigLayer::Local,
                }],
            }),
            title: Some(TitleHeld {
                title: "work-design".into(),
                from: TitleFrom::Session,
            }),
            ..opened(WindowOutcome::Opened, View::Terminal)
        };
        assert_eq!(
            opened_lines(&named)[..3],
            [
                "opened a app-window window titled \"work-design\" (the session's name) on mux's session work-design (the stream's for its kind)",
                "  stream work, multiplexer mux (the host's), view terminal (the default), terminal app app-window (the stream's)",
                "  the stream names the session work-design for the kind research (from managed) and the stage release.review (from local)",
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
            sessions: vec![],
            faults: vec![],
            priority: 0,
            running: 0,
            bound: None,
            room: None,
            queue: vec![],
            labels: vec![],
            reserves: Reserves::None,
            refused: refused.map(str::to_string),
            view_refused: view_refused.map(str::to_string),
            answerer: None,
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
    fn a_streams_named_sessions_and_its_tables_at_fault() {
        // 56, 199, C.7: each session a stream names for its kinds and
        // stages, under its own, with each kind and stage it names it
        // for and the layer naming it; then each table at fault with the
        // refusal it causes the starts that read it.
        let named_for = |name: &str, layer| flume_seams::NamedFor {
            name: name.into(),
            layer,
        };
        let stream = |sessions, faults| StreamStatus {
            name: "work".into(),
            session: SessionHeld {
                name: Some("work".into()),
                from: SessionFrom::StreamName,
            },
            sessions,
            faults,
            multiplexer: held("mux", NamedBy::Host),
            view: ViewHeld {
                view: View::Terminal,
                from: NamedBy::Neither,
            },
            terminal_app: held("printed", NamedBy::Neither),
            title: TitleHeld {
                title: "work".into(),
                from: TitleFrom::StreamName,
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
            answerer: None,
        };
        let two = stream(
            vec![
                flume_seams::NamedSession {
                    name: "work-design".into(),
                    kinds: vec![
                        named_for("inquiry", ConfigLayer::Managed),
                        named_for("research", ConfigLayer::Managed),
                    ],
                    stages: vec![named_for("release.review", ConfigLayer::Local)],
                },
                flume_seams::NamedSession {
                    name: "work-review".into(),
                    kinds: vec![],
                    stages: vec![named_for("change.verify", ConfigLayer::Managed)],
                },
            ],
            vec![],
        );
        let lines = host_lines(&HostStatus {
            streams: vec![two],
            ..HostStatus::default()
        });
        assert_eq!(
            lines[1..5],
            [
                "  stream work, priority 0: 0 agents running, no bound of its own",
                "    its agents open in the session work, the stream's name",
                "    it names the session work-design for the kind inquiry (from managed), the kind research (from managed) and the stage release.review (from local)",
                "    it names the session work-review for the stage change.verify (from managed)",
            ]
        );
        let other = "streams.work.kinds.research.pool: a stream's table for a kind holds the session its agents open in and its stages' tables alone";
        let theirs = "streams.work.kinds.release.stages.review.session: the session office the stream work names for the stage release.review is the stream personal's, its own session, and a session a stream names for a kind or a stage is that stream's alone";
        let at_fault = stream(
            vec![flume_seams::NamedSession {
                name: "office".into(),
                kinds: vec![],
                stages: vec![named_for("release.review", ConfigLayer::Managed)],
            }],
            vec![
                flume_seams::TableFault {
                    table: "streams.work.kinds.research".into(),
                    refused: other.into(),
                },
                flume_seams::TableFault {
                    table: "streams.work.kinds.release.stages.review".into(),
                    refused: theirs.into(),
                },
            ],
        );
        assert_eq!(
            named_lines(&at_fault),
            [
                "    it names the session office for the stage release.review (from managed)"
                    .to_string(),
                format!("    it starts no stage that reads streams.work.kinds.research: {other}"),
                format!(
                    "    it starts no stage that reads streams.work.kinds.release.stages.review: {theirs}"
                ),
            ]
        );
        assert!(named_lines(&stream(vec![], vec![])).is_empty());
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
            left_in_pane: false,
            identity: "flume:r:fix--one:code:1".into(),
            stage: "code".into(),
            program: "agent".into(),
            settings: AgentSettings {
                from: BTreeMap::from([(
                    "mode".to_string(),
                    Given {
                        layer,
                        value: written.into(),
                        file: None,
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
            own: None,
            effort: None,
        }
    }

    /// An agent whose settings give `from`, each own setting's among them,
    /// and its own settings beyond their keys as `own` gives them: its
    /// lines are its agent's line, its terminal's, then its settings'.
    fn with_own(from: &[(&str, Layer, &str, Option<&str>)], own: Option<OwnStatus>) -> AgentStatus {
        AgentStatus {
            settings: AgentSettings {
                from: from
                    .iter()
                    .map(|(setting, layer, value, file)| {
                        (
                            setting.to_string(),
                            Given {
                                layer: *layer,
                                value: value.to_string(),
                                file: file.map(str::to_string),
                            },
                        )
                    })
                    .collect(),
                ..AgentSettings::default()
            },
            mode: None,
            own,
            ..in_mode(
                "edits",
                Layer::Launch,
                ModeStatus {
                    passed: "edits".into(),
                    entry: None,
                    read_back: true,
                },
            )
        }
    }

    #[test]
    fn an_agent_with_own_settings_from_a_file_and_inline() {
        let agent = with_own(
            &[
                (
                    "programs.agent.settings.line",
                    Layer::HostStream,
                    r#""work""#,
                    None,
                ),
                (
                    "programs.agent.settings.tui",
                    Layer::Host,
                    r#""fullscreen""#,
                    Some("/home/user/agent.json"),
                ),
            ],
            Some(OwnStatus {
                passed: Some("--own /state/fix--one/agent/own.json".into()),
                read_back: false,
                files: vec![OwnFileStatus {
                    layer: Layer::Host,
                    written: "agent.json".into(),
                    read: "/home/user/agent.json".into(),
                    left_out: vec![
                        Reserved {
                            key: "env".into(),
                            owner: "the account".into(),
                        },
                        Reserved {
                            key: "perms.mode".into(),
                            owner: "flume's mode".into(),
                        },
                    ],
                }],
            }),
        );
        assert_eq!(
            agent_lines(&agent, "work", "fix")[2..],
            [
                r#"    programs.agent.settings.line = "work" (from host-stream)"#,
                r#"    programs.agent.settings.tui = "fullscreen" (from host, the file /home/user/agent.json)"#,
                "    the file /home/user/agent.json (from host)",
                "      env left out, owned by the account",
                "      perms.mode left out, owned by flume's mode",
                "    own settings passed as --own /state/fix--one/agent/own.json",
                "    own settings not read back",
            ]
        );
    }

    #[test]
    fn a_long_own_setting_is_elided() {
        let hooks = format!(r#"{{"Stop":[{{"command":"{}"}}]}}"#, "x".repeat(80));
        let list = format!(r#"["{}"]"#, "y".repeat(90));
        let short = r#"{"command":"line"}"#;
        let agent = with_own(
            &[
                ("programs.agent.settings.allow", Layer::Kind, &list, None),
                (
                    "programs.agent.settings.hooks",
                    Layer::Host,
                    &hooks,
                    Some("/home/user/agent.json"),
                ),
                ("programs.agent.settings.status", Layer::Host, short, None),
            ],
            None,
        );
        assert_eq!(
            agent_lines(&agent, "work", "fix")[2..],
            [
                "    programs.agent.settings.allow = […] (from kind)".to_string(),
                "    programs.agent.settings.hooks = {…} (from host, the file /home/user/agent.json)"
                    .to_string(),
                format!("    programs.agent.settings.status = {short} (from host)"),
            ]
        );
    }

    #[test]
    fn an_agent_with_no_own_settings() {
        let agent = with_own(&[("model", Layer::Launch, "large", None)], None);
        assert_eq!(
            agent_lines(&agent, "work", "fix")[2..],
            ["    model = large (from launch)"]
        );
        let none_passed = with_own(
            &[],
            Some(OwnStatus {
                passed: None,
                read_back: false,
                files: vec![OwnFileStatus {
                    layer: Layer::Host,
                    written: "agent.json".into(),
                    read: "/home/user/agent.json".into(),
                    left_out: vec![],
                }],
            }),
        );
        assert_eq!(
            agent_lines(&none_passed, "work", "fix")[2..],
            [
                "    the file /home/user/agent.json (from host)",
                "    own settings not read back",
            ]
        );
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

    /// An agent whose layer `layer` gave the effort `word`, none where it
    /// is none, on an account whose `env` names `CLAUDE_CODE_EFFORT_LEVEL`
    /// where `outranked`, as the core gives it: the variable by its name
    /// alone (56, 197; core.30).
    fn with_effort(word: Option<&str>, layer: Layer, outranked: bool) -> AgentStatus {
        let variable = "CLAUDE_CODE_EFFORT_LEVEL";
        let mut from = BTreeMap::new();
        let mut env = BTreeMap::new();
        if outranked {
            env.insert(variable.to_string(), flume_seams::SET.to_string());
            from.insert(
                format!("env.{variable}"),
                Given {
                    layer: Layer::Account,
                    value: flume_seams::SET.into(),
                    file: None,
                },
            );
        }
        if let Some(word) = word {
            from.insert(
                "effort".to_string(),
                Given {
                    layer,
                    value: word.into(),
                    file: None,
                },
            );
        }
        AgentStatus {
            settings: AgentSettings {
                settings: flume_seams::Settings {
                    effort: word.map(str::to_string),
                    ..flume_seams::Settings::default()
                },
                env,
                from,
            },
            mode: None,
            effort: word.map(|word| EffortStatus {
                word: word.into(),
                layer,
                passed: word.into(),
                read_back: false,
                outranked_by: outranked.then(|| variable.to_string()),
            }),
            ..in_mode(
                "edits",
                Layer::Kind,
                ModeStatus {
                    passed: "edit-files".into(),
                    entry: None,
                    read_back: false,
                },
            )
        }
    }

    #[test]
    fn an_effort_from_the_launch() {
        // S103: the launch's `xhigh`, on the agent's line with its layer,
        // passed and not read back, and among its settings (56, 197).
        let agent = with_effort(Some("xhigh"), Layer::Launch, false);
        assert_eq!(
            agent_line(&agent),
            "  agent flume:r:fix--one:code:1 (agent): working, at the effort xhigh (from launch) passed as xhigh, not read back"
        );
        assert_eq!(
            agent_lines(&agent, "work", "fix")[2..],
            ["    effort = xhigh (from launch)"]
        );
    }

    #[test]
    fn an_effort_the_accounts_variable_outranks() {
        // The account's variable named as outranking the effort, by its
        // name, its value never in the answer (56, 197; core.30).
        let agent = with_effort(Some("high"), Layer::HostKind, true);
        assert_eq!(
            agent_line(&agent),
            "  agent flume:r:fix--one:code:1 (agent): working, at the effort high (from host-kind) passed as high, not read back; the account's CLAUDE_CODE_EFFORT_LEVEL outranks it"
        );
        assert_eq!(
            agent_lines(&agent, "work", "fix")[2..],
            [
                "    effort = high (from host-kind)",
                "    env.CLAUDE_CODE_EFFORT_LEVEL = <set> (from account)",
            ]
        );
    }

    #[test]
    fn no_effort() {
        // No layer gave an effort: the line names none, and the account's
        // variable is a variable like any other (197).
        let agent = with_effort(None, Layer::Launch, true);
        assert_eq!(
            agent_line(&agent),
            "  agent flume:r:fix--one:code:1 (agent): working"
        );
        assert_eq!(
            agent_lines(&agent, "work", "fix")[2..],
            ["    env.CLAUDE_CODE_EFFORT_LEVEL = <set> (from account)"]
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
                cause: flume_seams::CarryCause::Spent,
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
                cause: flume_seams::CarryCause::Spent,
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
                        spent_unread: None,
                        reading: None,
                        past: None,
                    },
                    AccountStatus {
                        name: "work-2".into(),
                        program: "agent".into(),
                        running: 0,
                        bound: None,
                        spent_until: Some(54_000),
                        spent_unread: None,
                        reading: None,
                        past: None,
                    },
                    AccountStatus {
                        name: "work-3".into(),
                        program: "agent".into(),
                        running: 0,
                        bound: None,
                        spent_until: Some(51_000),
                        spent_unread: Some(
                            "You've hit your weekly limit \u{b7} resets Nov 8, 12am (UTC)".into(),
                        ),
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
                "    work-3 (agent): 0 agents running, no bound of its own, spent until 51000"
                    .to_string(),
                "      spent an hour: no reset read in \"You've hit your weekly limit \u{b7} resets Nov 8, 12am (UTC)\""
                    .to_string(),
                "  no caller keeps watch: every place settles at each command of flume's that acts".to_string(),
            ]
        );
    }

    /// Where a kind of `source` came from, `name@version`, with nothing
    /// but its identity and source given.
    fn kind_from(source: &str, name: &str, version: u32) -> KindFrom {
        KindFrom {
            identity: flume_seams::KindIdentity {
                source: source.into(),
                name: name.into(),
                version,
                spelled: format!("{source}/{name}@{version}"),
            },
            source: source.into(),
            source_file: None,
            kind_dir: None,
            declaration_file: None,
            revision: None,
            hash: None,
            lock_file: None,
        }
    }

    #[test]
    fn a_kind_of_a_hosts_source() {
        // 19, 56: acme/review@1, locked in the host's lock, on its line
        // with the source's file, the kind's directory, its hash and the
        // lock.
        let from = KindFrom {
            source_file: Some("/c/flume/sources/acme.toml".into()),
            kind_dir: Some("/k/acme/review@1".into()),
            hash: Some("sha256:ab12".into()),
            lock_file: Some("/c/flume/flume.lock".into()),
            ..kind_from("acme", "review", 1)
        };
        assert_eq!(
            kind_line(Some(&from)).as_deref(),
            Some("  kind acme/review@1 from the source acme (/c/flume/sources/acme.toml) at /k/acme/review@1, its hash sha256:ab12, locked in /c/flume/flume.lock")
        );
    }

    #[test]
    fn the_repositorys_a_launchs_and_flumes_own() {
        // 19, 56: the repository's bump@2 with the base's revision, a
        // launch's kind with its file, and flume's own change@2 with its
        // hash and no lock.
        let repos = KindFrom {
            declaration_file: Some(".config/flume.toml".into()),
            revision: Some(flume_seams::Revision::new("c0ffee")),
            ..kind_from("repo", "bump", 2)
        };
        assert_eq!(
            kind_line(Some(&repos)).as_deref(),
            Some("  kind repo/bump@2 from the repository at c0ffee")
        );
        let launchs = KindFrom {
            declaration_file: Some("/w/kinds.toml".into()),
            ..kind_from("launch", "fix", 1)
        };
        assert_eq!(
            kind_line(Some(&launchs)).as_deref(),
            Some("  kind launch/fix@1 from /w/kinds.toml")
        );
        let own = KindFrom {
            hash: Some("sha256:cd34".into()),
            ..kind_from("flume", "change", 2)
        };
        assert_eq!(
            kind_line(Some(&own)).as_deref(),
            Some("  kind flume/change@2, flume's own, its hash sha256:cd34")
        );
    }

    #[test]
    fn a_place_launched_before_this_change() {
        // 21, core.14: held state with no `kind_from` gives no kind line,
        // the place's first line giving the source from `declared`.
        assert_eq!(kind_line(None), None);
    }

    #[test]
    fn recipients_with_where_they_came_from() {
        // 7, 56, 193: each place's recipients with where its launch took
        // them, the answerer with its source; a place launched before
        // flume kept it gives them alone.
        let line = |recipients: &[&str], from: Option<RecipientsFrom>| {
            place_lines(&PlaceStatus {
                recipients: recipients.iter().map(|name| name.to_string()).collect(),
                recipients_from: from,
                ..a_place()
            })
            .into_iter()
            .find(|line| line.starts_with("  recipients: "))
        };
        assert_eq!(
            line(&["ops"], Some(RecipientsFrom::Launch)).as_deref(),
            Some("  recipients: ops, from the launch")
        );
        assert_eq!(
            line(
                &["east/operator"],
                Some(RecipientsFrom::Answerer {
                    source: "acme".into()
                })
            )
            .as_deref(),
            Some("  recipients: east/operator, from the stream's answerer, declared by acme")
        );
        assert_eq!(
            line(&["conductor"], Some(RecipientsFrom::Environment)).as_deref(),
            Some("  recipients: conductor, from $FLUME_RECIPIENT")
        );
        assert_eq!(
            line(&["pane-7"], Some(RecipientsFrom::Session)).as_deref(),
            Some("  recipients: pane-7, from the session that launched it")
        );
        assert_eq!(line(&["ops"], None).as_deref(), Some("  recipients: ops"));
        assert_eq!(line(&[], None), None);
    }

    #[test]
    fn an_ends_recipients_pending_expired_and_confirmed() {
        // 43, 56; weir 104, 106: under an end, each recipient pending,
        // expired since when and never confirmed under, or confirmed and
        // when.
        let end = EndStatus {
            owed: vec![
                flume_seams::OwedTo {
                    recipient: "bomdrive".into(),
                    expired: true,
                    known: false,
                },
                flume_seams::OwedTo {
                    recipient: "ops".into(),
                    expired: false,
                    known: true,
                },
            ],
            confirmed: vec![flume_seams::ConfirmedBy {
                recipient: "boomdrive".into(),
                at: 131,
            }],
            delivery: Some("fix--one/0a1b2c3d/submission/1/landed".into()),
            expires_at: Some(604_930),
            pending_for: vec!["bomdrive".into(), "ops".into()],
            ..end_status(
                &EndHeld {
                    of: EndOf::Submission {
                        submit: 1,
                        verdict: flume_seams::Verdict::Landed {
                            commit: flume_seams::Revision::new("c9"),
                            place_removed: false,
                            branch_removed: false,
                        },
                    },
                    read_at: 130,
                    deliveries: vec![],
                    answer: None,
                    claim: None,
                    noticing: vec![],
                },
                &["bomdrive", "ops", "boomdrive"],
            )
        };
        let lines = place_lines(&PlaceStatus {
            ends: vec![end],
            ..a_place()
        });
        let at = lines
            .iter()
            .position(|line| line.starts_with("  ended submission/1/landed: "))
            .expect("the landing's line");
        assert_eq!(
            lines[at + 1..at + 4],
            [
                "    bomdrive: pending, expired since 604930, never confirmed".to_string(),
                "    ops: pending".to_string(),
                "    boomdrive: confirmed at 131".to_string(),
            ],
            "{lines:#?}"
        );
    }

    #[test]
    fn the_hosts_expired_ends() {
        // 43, 56; weir 104, 106: the place-status spec's "Two expired
        // ends" in the person's form: a line for each, saying never
        // confirmed, and how its recipient confirms it.
        let expired = |name: &str| flume_core::ExpiredEnd {
            place: PlaceId::new(format!("/d/places/1/fix--{name}")),
            key: "submission/1/landed".parse().unwrap(),
            delivery: format!("fix--{name}/0a1b2c3d/submission/1/landed"),
            recipient: "bomdrive".into(),
            known: false,
            expires_at: 604_930,
        };
        let lines = host_lines(&HostStatus {
            expired: vec![expired("one"), expired("two")],
            ..HostStatus::default()
        });
        for name in ["one", "two"] {
            let line = format!(
                "  expired: /d/places/1/fix--{name} submission/1/landed for bomdrive since 604930, never confirmed; confirm: flume confirm --recipient bomdrive fix--{name}/0a1b2c3d/submission/1/landed"
            );
            assert!(lines.contains(&line), "{line} not in {lines:#?}");
        }
    }

    #[test]
    fn the_inbox_and_a_wait_confirm_as_their_recipient() {
        // 43; weir 102: an end the inbox or a wait hands to `east/ops`
        // says how `east/ops` confirms it, by its name, since the caller's
        // own name, its session's in a pane, is not the one it is owed
        // under.
        let end = EndStatus {
            delivery: Some("fix--one/0a1b2c3d/submission/1/landed".into()),
            ..end_status(
                &EndHeld {
                    of: EndOf::Submission {
                        submit: 1,
                        verdict: flume_seams::Verdict::Landed {
                            commit: flume_seams::Revision::new("c9"),
                            place_removed: false,
                            branch_removed: false,
                        },
                    },
                    read_at: 130,
                    deliveries: vec![],
                    answer: None,
                    claim: None,
                    noticing: vec![],
                },
                &["east/ops"],
            )
        };
        let confirm =
            "confirm: flume confirm --recipient east/ops fix--one/0a1b2c3d/submission/1/landed";
        let inbox = Answer::Inbox {
            recipient: "east/ops".into(),
            ends: vec![flume_core::Collected {
                place: PlaceId::new("/d/places/1/fix--one"),
                kind: "fix".into(),
                name: "one".into(),
                end: end.clone(),
                confirmed: false,
                expired: false,
            }],
            waiting: vec![],
        };
        assert_eq!(
            person(&inbox),
            format!(
                "fix--one submission landed c9 [submission/1/landed]; read: flume status /d/places/1/fix--one; {confirm}"
            )
        );
        let wait = Answer::Waited {
            place: PlaceId::new("/d/places/1/fix--one"),
            recipient: "east/ops".into(),
            end: Some(end),
            delivered: true,
        };
        assert_eq!(
            person(&wait),
            format!(
                "/d/places/1/fix--one submission landed c9 for east/ops: handed over by this wait; pending until east/ops confirms it; {confirm}"
            )
        );
    }

    /// The end `end` as status gives it to a place whose recipients are
    /// `recipients`, no delivery of it recorded (43).
    fn end_status(end: &EndHeld, recipients: &[&str]) -> EndStatus {
        let recipients: Vec<String> = recipients.iter().map(ToString::to_string).collect();
        EndStatus {
            end: end.clone(),
            key: end.of.end_key(),
            pending_for: end.recipients(&recipients),
            open_until: match &end.of {
                EndOf::Stage { end, .. } => end.open_until,
                _ => None,
            },
            confirmed: vec![],
            delivery: None,
            expires_at: None,
            owed: vec![],
        }
    }

    #[test]
    fn a_drops_end_line() {
        // 43, 52, 56; core.14: a drop's end as any end's line, pending for
        // the recipient that is no role's address, and no caller's line.
        let place = PlaceStatus {
            recipients: vec!["east/planner".into(), "boomdrive".into()],
            recipients_from: Some(RecipientsFrom::Launch),
            ends: vec![end_status(
                &EndHeld {
                    of: EndOf::Over {
                        over: flume_seams::Over::Dropped,
                    },
                    read_at: 500,
                    deliveries: vec![],
                    answer: None,
                    claim: None,
                    noticing: vec![],
                },
                &["east/planner", "boomdrive"],
            )],
            ..a_place()
        };
        let lines = place_lines(&place);
        assert!(
            lines
                .iter()
                .any(|line| line == "  ended over/dropped: dropped; pending for boomdrive"),
            "{lines:#?}"
        );
        assert!(
            !lines.iter().any(|line| line.starts_with("  caller: ")),
            "{lines:#?}"
        );
    }

    #[test]
    fn a_streams_answerer() {
        // 143, 193: the one role standing behind a stream with its
        // source, or the roles that each declare to.
        let by = |address: &str, source: &str| flume_seams::DeclaredBy {
            address: address.into(),
            source: source.into(),
        };
        assert_eq!(
            answerer_line(&StreamAnswerer::Stands(by("east/operator", "acme"))),
            "    its answerer: east/operator, declared by acme"
        );
        assert_eq!(
            answerer_line(&StreamAnswerer::Contested {
                roles: vec![by("east/door", "beta"), by("east/operator", "acme")]
            }),
            "    no one answerer: east/door, declared by beta and east/operator, declared by acme each stand behind it"
        );
    }

    #[test]
    fn a_sources_roles() {
        // 56, 191, 193: each source's roles file with the addresses it
        // declares, none, or why it cannot be read, and each entry that
        // declares nothing with why.
        let source = |roles: &[&str], file: Option<&str>, fault: Option<&str>| SourceStatus {
            name: "acme".into(),
            source_file: "/c/flume/sources/acme.toml".into(),
            kinds_dir: Some("/k/acme".into()),
            kinds: vec!["acme/turn@1".into()],
            cannot_read: None,
            roles_file: file.map(str::to_string),
            roles: roles.iter().map(|role| role.to_string()).collect(),
            roles_refused: vec![],
            roles_cannot_read: fault.map(str::to_string),
        };
        let first = "  source acme (/c/flume/sources/acme.toml) at /k/acme: acme/turn@1";
        assert_eq!(
            source_lines(&source(
                &["east/operator", "east/ops"],
                Some("/s/acme/roles.toml"),
                None
            )),
            [
                first,
                "    its roles (/s/acme/roles.toml): east/operator, east/ops"
            ]
        );
        assert_eq!(
            source_lines(&source(&[], Some("/s/acme/roles.toml"), None)),
            [first, "    its roles (/s/acme/roles.toml): declares none"]
        );
        assert_eq!(
            source_lines(&source(&[], None, Some("/s/acme/roles.toml: no such file"))),
            [
                first,
                "    its roles: cannot be read: /s/acme/roles.toml: no such file"
            ]
        );
        assert_eq!(source_lines(&source(&[], None, None)), [first]);
        let refused = SourceStatus {
            roles_refused: vec![flume_seams::RoleRefused {
                address: "east/ops".into(),
                source: "acme".into(),
                reason: "its kind beta/ops is not acme's".into(),
            }],
            ..source(&["east/operator"], Some("/s/acme/roles.toml"), None)
        };
        assert_eq!(
            source_lines(&refused),
            [
                first,
                "    its roles (/s/acme/roles.toml): east/operator",
                "    declares nothing for east/ops: its kind beta/ops is not acme's"
            ]
        );
    }

    #[test]
    fn an_address_carried_or_waking_nothing() {
        // 56, 132, 193: an address with ends pending that a source
        // declares carried, or that no source declares, with why.
        let address = |name: &str, carried, wakes_nothing: Option<&str>| AddressStatus {
            address: name.into(),
            role: None,
            run: None,
            pending: pending(1),
            unwoken: None,
            standing: None,
            carried,
            wakes_nothing: wakes_nothing.map(str::to_string),
            stalled: None,
        };
        let lines = host_lines(&HostStatus {
            addresses: vec![
                address(
                    "north/planner",
                    Some(flume_seams::CarriedRole {
                        source: "acme".into(),
                        answers: None,
                    }),
                    None,
                ),
                address(
                    "east/planner",
                    None,
                    Some("no source of the host declares east/planner"),
                ),
            ],
            ..HostStatus::default()
        });
        assert_eq!(
            lines[1..lines.len() - 1],
            [
                "  1 ends pending for north/planner, no run; acme declares it carried, its run on another host",
                "    /places/fix--one: code delivered",
                "  1 ends pending for east/planner, no run; it wakes nothing: no source of the host declares east/planner",
                "    /places/fix--one: code delivered",
            ]
        );
    }

    #[test]
    fn a_doors_run_whose_stall_reached_no_one() {
        // 130, 132, 193: the run of its stream's own answerer, stalled
        // with no one to reach, named under the host's addresses.
        let run = PlaceId::new("/places/turn--east-operator");
        let lines = host_lines(&HostStatus {
            addresses: vec![AddressStatus {
                address: "east/operator".into(),
                role: None,
                run: Some(run.clone()),
                pending: vec![],
                unwoken: None,
                standing: None,
                carried: None,
                wakes_nothing: None,
                stalled: Some(PendingEnd {
                    place: run,
                    key: "stall/420".parse().unwrap(),
                    what: "stalled: a retry of plan".into(),
                }),
            }],
            ..HostStatus::default()
        });
        assert_eq!(
            lines[1..lines.len() - 1],
            [
                "  0 ends pending for east/operator, its run /places/turn--east-operator",
                "    its run /places/turn--east-operator stalled: a retry of plan, which reached no one",
            ]
        );
    }

    #[test]
    fn the_hosts_sources_with_a_fault() {
        // S96, 56, 191: acme with its file, its directory and the kinds
        // it holds, an empty source, a directory that cannot be read, and
        // bad with the fault naming `kinds_dir`.
        let source = |name: &str, kinds_dir: Option<&str>, kinds: &[&str], fault: Option<&str>| {
            SourceStatus {
                name: name.into(),
                source_file: format!("/c/flume/sources/{name}.toml"),
                kinds_dir: kinds_dir.map(str::to_string),
                kinds: kinds.iter().map(|kind| kind.to_string()).collect(),
                cannot_read: fault.map(str::to_string),
                roles_file: None,
                roles: vec![],
                roles_refused: vec![],
                roles_cannot_read: None,
            }
        };
        let host = HostStatus {
            sources: vec![
                source(
                    "acme",
                    Some("/k/acme"),
                    &["acme/review@1", "acme/review@2"],
                    None,
                ),
                source(
                    "bad",
                    None,
                    &[],
                    Some("gives no `kinds_dir`, the directory holding the source's packages"),
                ),
                source("beta", Some("/k/beta"), &[], None),
                source(
                    "gamma",
                    Some("/k/gamma"),
                    &[],
                    Some("/k/gamma, the directory of the source gamma, does not exist"),
                ),
            ],
            ..HostStatus::default()
        };
        assert_eq!(
            host_lines(&host),
            vec![
                "host: 0 agents running, no bound".to_string(),
                "  source acme (/c/flume/sources/acme.toml) at /k/acme: acme/review@1, acme/review@2".to_string(),
                "  source bad (/c/flume/sources/bad.toml): cannot be read: gives no `kinds_dir`, the directory holding the source's packages".to_string(),
                "  source beta (/c/flume/sources/beta.toml) at /k/beta: holds no kind".to_string(),
                "  source gamma (/c/flume/sources/gamma.toml) at /k/gamma: cannot be read: /k/gamma, the directory of the source gamma, does not exist".to_string(),
                "  no caller keeps watch: every place settles at each command of flume's that acts".to_string(),
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
            spent_unread: None,
            reading,
            past,
        }
    }

    /// A pool's threshold and its accounts' readings, each taken from an
    /// agent, stale where it is not fresh, and an account no session
    /// reported for with why it has none (56, 184, 185).
    #[test]
    fn a_pools_threshold_and_its_accounts_readings() {
        let window = |name: &str, used, resets_at, past| WindowStatus {
            name: name.into(),
            used,
            resets_at,
            past,
        };
        let code = "flume:r:fix--one:code:1";
        let pool = PoolStatus {
            name: "work".into(),
            strategy: Strategy::MostLeft,
            threshold: 80,
            accounts: vec![
                read(
                    "work-1",
                    Some(ReadingStatus::Read {
                        read_at: 53_880,
                        agent: code.into(),
                        session: Some("s-1".into()),
                        fresh: true,
                        windows: vec![
                            window("five_hour", 4_200, Some(61_200), false),
                            window("seven_day", 7_150, None, false),
                        ],
                    }),
                    None,
                ),
                read(
                    "work-2",
                    Some(ReadingStatus::Read {
                        read_at: 52_800,
                        agent: code.into(),
                        session: None,
                        fresh: false,
                        windows: vec![window("five_hour", 1_000, Some(61_200), false)],
                    }),
                    None,
                ),
                read(
                    "work-3",
                    Some(ReadingStatus::Unread {
                        unread: flume_core::NONE_REPORTED.into(),
                    }),
                    None,
                ),
                read(
                    "work-4",
                    Some(ReadingStatus::Read {
                        read_at: 54_000,
                        agent: code.into(),
                        session: Some("s-4".into()),
                        fresh: true,
                        windows: vec![window("seven_day", 9_500, Some(313_200), true)],
                    }),
                    Some(Past {
                        window: "seven_day".into(),
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
                "      five_hour 42% (resets 61200) · seven_day 71.5% · taken at 53880 from flume:r:fix--one:code:1",
                "    work-2 (agent): 0 agents running, no bound of its own",
                "      five_hour 10% (resets 61200) · stale, taken at 52800 from flume:r:fix--one:code:1",
                "    work-3 (agent): 0 agents running, no bound of its own",
                "      no reading: no session flume started has reported its windows",
                "    work-4 (agent): 0 agents running, no bound of its own",
                "      seven_day 95% (resets 313200) · taken at 54000 from flume:r:fix--one:code:1",
                "      past its threshold in seven_day until 313200",
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
            "no place stands on this host\nhost: 0 agents running, no bound\n  no caller keeps watch: every place settles at each command of flume's that acts"
        );
    }

    /// The standing agent `agent` in the session `madswan`, its name from
    /// `from`, sought in the stream `madswan`, and what was found of it.
    fn standing(agent: &str, from: SessionFrom, found: StandingFound) -> StandingStatus {
        StandingStatus {
            agent: agent.into(),
            stream: "madswan".into(),
            session: SessionHeld {
                name: Some("madswan".into()),
                from,
            },
            terminal: Some("fake".into()),
            found,
        }
    }

    /// `count` ends pending, of the fixes `one`, `two` and so on.
    fn pending(count: usize) -> Vec<PendingEnd> {
        ["one", "two", "three"][..count]
            .iter()
            .map(|name| PendingEnd {
                place: PlaceId(format!("/places/fix--{name}")),
                key: "stage/code/1/delivered".parse().unwrap(),
                what: "code delivered".into(),
            })
            .collect()
    }

    /// The host's lines for the one address `address`, held by `standing`
    /// with `count` ends pending, the host's first line and its watch's
    /// left out.
    fn standing_lines(address: &str, standing: StandingStatus, count: usize) -> Vec<String> {
        let lines = host_lines(&HostStatus {
            addresses: vec![AddressStatus {
                address: address.into(),
                role: None,
                run: None,
                pending: pending(count),
                unwoken: None,
                standing: Some(standing),
                carried: None,
                wakes_nothing: None,
                stalled: None,
            }],
            ..HostStatus::default()
        });
        lines[1..lines.len() - 1].to_vec()
    }

    #[test]
    fn a_standing_address_found_waiting() {
        // 56, 132, design 7: the agent, its session with where it came
        // from, what was found and how many ends wait, each beneath.
        assert_eq!(
            standing_lines(
                "madswan/planner",
                standing(
                    "madswan-planner",
                    SessionFrom::Role,
                    StandingFound::One {
                        at: "fake:1@w".into(),
                        state: AgentState::Waiting,
                    },
                ),
                3,
            ),
            [
                "  madswan/planner: the standing agent madswan-planner in the session madswan (the entry's), found waiting; 3 ends pending",
                "    /places/fix--one: code delivered",
                "    /places/fix--two: code delivered",
                "    /places/fix--three: code delivered",
            ]
        );
    }

    #[test]
    fn a_standing_address_with_none_of_its_name() {
        // 132, design 7: the session the stream's name gives it.
        assert_eq!(
            standing_lines(
                "madswan/ops",
                standing("madswan-ops", SessionFrom::StreamName, StandingFound::None),
                1,
            ),
            [
                "  madswan/ops: the standing agent madswan-ops in the session madswan (the stream madswan's name), none of that name there; 1 end pending",
                "    /places/fix--one: code delivered",
            ]
        );
    }

    #[test]
    fn a_standing_address_with_two_of_its_name() {
        // 132: two of one name take no notice.
        assert_eq!(
            standing_lines(
                "madswan/planner",
                standing(
                    "madswan-planner",
                    SessionFrom::Stream,
                    StandingFound::Several { count: 2 },
                ),
                1,
            )[0],
            "  madswan/planner: the standing agent madswan-planner in the session madswan (the stream madswan's), 2 of that name there, so none takes a notice; 1 end pending"
        );
    }

    #[test]
    fn a_standing_address_not_read() {
        // 132, 190: why the terminal could not read, in its words, and the
        // terminal's own session where the default stream names none.
        let mut gone = standing(
            "w-planner",
            SessionFrom::Own,
            StandingFound::Unread {
                reason: "the host declares no stream gone".into(),
            },
        );
        gone.session.name = None;
        gone.stream = "gone".into();
        gone.terminal = None;
        assert_eq!(
            standing_lines("w/planner", gone, 2)[0],
            "  w/planner: the standing agent w-planner in the terminal's own session (the stream gone's), not read: the host declares no stream gone; 2 ends pending"
        );
    }

    #[test]
    fn a_standing_address_with_nothing_pending() {
        // 56, design 7: the entry read and the agent found, with no end
        // beneath.
        assert_eq!(
            standing_lines(
                "madswan/planner",
                standing(
                    "madswan-planner",
                    SessionFrom::Role,
                    StandingFound::One {
                        at: "fake:1@w".into(),
                        state: AgentState::Working,
                    },
                ),
                0,
            ),
            ["  madswan/planner: the standing agent madswan-planner in the session madswan (the entry's), found working; nothing pending"]
        );
    }

    #[test]
    fn a_wake_of_a_standing_address_is_held() {
        // 132, design 5: the agent, its session, what was found, nothing
        // launched, and each end pending beneath.
        let answer = Answer::Held {
            address: "madswan/planner".into(),
            standing: standing(
                "madswan-planner",
                SessionFrom::Role,
                StandingFound::One {
                    at: "fake:1@w".into(),
                    state: AgentState::Asking,
                },
            ),
            pending: pending(2),
        };
        assert_eq!(
            person(&answer),
            "madswan/planner is held by the standing agent madswan-planner in the session madswan (the entry's), found asking, so nothing was launched; 2 ends pending\n  /places/fix--one: code delivered\n  /places/fix--two: code delivered"
        );
    }

    #[test]
    fn a_host_under_a_watch() {
        // The place-status spec's watched host: a caller's bound and when
        // its newest settle began (56, 208).
        let host = HostStatus {
            watch: Watch::Bound { seconds: 120 },
            watch_settled: Some(47_400),
            ..HostStatus::default()
        };
        assert_eq!(
            host_lines(&host)[1],
            "  a caller keeps watch: every place settles every 2m, last at 47400"
        );
    }

    #[test]
    fn a_host_whose_watch_lapsed() {
        // The place-status spec's unwatched host: since when no caller
        // keeps watch, the watch's lapse (119, 208).
        let host = HostStatus {
            watch_settled: Some(47_400),
            watch_lapsed: Some(47_640),
            ..HostStatus::default()
        };
        assert_eq!(
            host_lines(&host)[1],
            "  no caller keeps watch since 47640: every place settles at each command of flume's that acts"
        );
    }

    #[test]
    fn a_duration_in_its_largest_whole_unit() {
        assert_eq!(duration(120), "2m");
        assert_eq!(duration(90), "90s");
        assert_eq!(duration(3600), "1h");
        assert_eq!(duration(30), "30s");
    }

    #[test]
    fn a_missing_key_under_the_watch() {
        // 72, 122: what the terminal lacks, under the line naming the watch.
        let key = "the terminal's /t/config.toml binds no key to flume's places".to_string();
        let host = HostStatus {
            watch: Watch::Bound { seconds: 30 },
            watch_settled: Some(100),
            terminal_setup: vec![key.clone()],
            ..HostStatus::default()
        };
        let lines = host_lines(&host);
        assert_eq!(
            lines[lines.len() - 2..],
            [
                "  a caller keeps watch: every place settles every 30s, last at 100".to_string(),
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

    /// A live place of a fix kind, its current stage code, and nothing
    /// else to say.
    fn a_place() -> PlaceStatus {
        PlaceStatus {
            place: PlaceId::new("/d/places/1/fix--one"),
            repository: "/r".into(),
            address: None,
            kind: "fix".into(),
            version: 1,
            identity: None,
            declared: Declared::Flume,
            kind_from: None,
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
            recipients_from: None,
            hold_words: BTreeMap::new(),
            waiting_on: None,
            stalled: false,
            waits_on_session: None,
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
            waits_on: None,
            worktree: None,
            on_branch: None,
            fix_kind: None,
            fix_limit: None,
            forecast: None,
            fixes: vec![],
            fix_of: None,
        }
    }

    /// A process that runs a lost talk's session, as live read it.
    fn by_hand() -> LiveProcess {
        LiveProcess {
            pid: 4242,
            started: 9000,
        }
    }

    #[test]
    fn a_lost_conversation_carried_and_one_started_afresh() {
        // 56, 179: a lost start's conversation carried says it was lost;
        // one that could not be carried says so with why, naming no path.
        let lost = |carried, why: Option<&str>| Conversation {
            from: 1,
            cause: flume_seams::CarryCause::Lost,
            carried,
            why: why.map(str::to_string),
        };
        assert_eq!(
            conversation_line(&lost(true, None)),
            "    carries the conversation of start 1, lost"
        );
        assert_eq!(
            conversation_line(&lost(false, Some("its conversation's transcript, in the stopped account's configuration home (`CLAUDE_CONFIG_DIR`), cannot be read"))),
            "    starts afresh after start 1, lost: its conversation's transcript, in the stopped account's configuration home (`CLAUDE_CONFIG_DIR`), cannot be read"
        );
    }

    #[test]
    fn an_account_from_its_conversation() {
        // 145: the account a lost conversation's start ran under, taken
        // first, as its conversation's.
        let agent = AgentStatus {
            account: Some(AccountHeld {
                pool: "ops".into(),
                name: "work-1".into(),
                from: AccountFrom::Conversation,
            }),
            mode: None,
            settings: AgentSettings::default(),
            ..in_mode(
                "edits",
                Layer::Kind,
                ModeStatus {
                    passed: "edit-files".into(),
                    entry: None,
                    read_back: true,
                },
            )
        };
        assert_eq!(
            agent_line(&agent),
            "  agent flume:r:fix--one:code:1 (agent) on the account work-1 of ops (its conversation's): working"
        );
    }

    #[test]
    fn a_stage_waiting_on_its_session() {
        // 49, 56, 207: the stage waits for its conversation to end, since
        // when, with each process by its id and start, or what flume could
        // not read.
        let place = PlaceStatus {
            waits_on_session: Some(SessionWait {
                stage: "talk".into(),
                processes: vec![
                    by_hand(),
                    LiveProcess {
                        pid: 4343,
                        started: 9100,
                    },
                ],
                unread: None,
                since: Some(54_000),
            }),
            ..a_place()
        };
        assert!(
            place_lines(&place).contains(
                &"  talk waits for its conversation to end since 54000: process 4242, started 9000; process 4343, started 9100".to_string()
            ),
            "{:#?}",
            place_lines(&place)
        );
        let unread = PlaceStatus {
            waits_on_session: Some(SessionWait {
                stage: "talk".into(),
                processes: vec![],
                unread: Some("the host's processes cannot be listed".into()),
                since: None,
            }),
            ..a_place()
        };
        assert!(place_lines(&unread).contains(
            &"  talk waits for its conversation to end: flume cannot read whether a process runs it: the host's processes cannot be listed".to_string()
        ));
    }

    #[test]
    fn an_agent_left_in_its_pane_and_the_end_that_says_so() {
        // 43, 77: the agent the terminal brought back in a lost talk's
        // pane, left there, on a line of its own beneath it; the lost end
        // in the notice's words, in status and in the inbox.
        let end = EndStatus {
            end: EndHeld {
                of: EndOf::Stage {
                    stage: "talk".into(),
                    start: 1,
                    state: StageState::Lost,
                    quiet_from: None,
                    answered: 0,
                    end: Box::new(StageEnd {
                        left_in_pane: true,
                        ..StageEnd::default()
                    }),
                },
                read_at: 54_000,
                deliveries: vec![],
                answer: None,
                claim: None,
                noticing: vec![],
            },
            key: "stage/talk/1/lost".parse().unwrap(),
            pending_for: vec!["ops".into()],
            open_until: None,
            confirmed: vec![],
            delivery: None,
            expires_at: None,
            owed: vec![],
        };
        let place = PlaceStatus {
            agent: Some(AgentStatus {
                left_in_pane: true,
                ..in_mode(
                    "edits",
                    Layer::Kind,
                    ModeStatus {
                        passed: "edit-files".into(),
                        entry: None,
                        read_back: true,
                    },
                )
            }),
            ends: vec![end.clone()],
            ..a_place()
        };
        let lines = place_lines(&place);
        assert!(
            lines.contains(&"    brought back by the terminal in its start's pane after its stage was lost, and left there: flume sends it nothing, and only its caller's stop, drop or done ends it".to_string()),
            "{lines:#?}"
        );
        assert!(lines.contains(
            &"  ended stage/talk/1/lost: talk lost, and flume left the agent the terminal brought back in its pane; pending for ops".to_string()
        ));
        let inbox = Answer::Inbox {
            recipient: "ops".into(),
            ends: vec![flume_core::Collected {
                place: PlaceId::new("/d/places/1/research--one"),
                kind: "research".into(),
                name: "one".into(),
                end,
                confirmed: false,
                expired: false,
            }],
            waiting: vec![],
        };
        assert_eq!(
            person(&inbox),
            "research--one talk lost, and flume left the agent the terminal brought back in its pane [stage/talk/1/lost]; read: flume status /d/places/1/research--one"
        );
    }

    /// A place waiting on a retry of code for its answer since 1000, the
    /// question asked, its agent kept open until `open_until` or let go
    /// as `closed` records.
    fn asking(open_until: Option<u64>, closed: Option<SessionClosed>) -> PlaceStatus {
        PlaceStatus {
            waiting_on: Some(WaitingOn {
                act: Act::Retry,
                stage: Some("code".into()),
                why: Waits::Answer,
                since: Some(1000),
                words: Some("Which file holds the log?".into()),
                open_until,
                closed,
            }),
            ..a_place()
        }
    }

    /// The lines of `place` from its move's on.
    fn from_the_move(place: &PlaceStatus) -> Vec<String> {
        let lines = place_lines(place);
        let at = lines
            .iter()
            .position(|line| line.starts_with("  waits on its caller"))
            .expect("the move's line");
        lines[at..].iter().take(2).cloned().collect()
    }

    const ASKS: &str =
        "  waits on its caller to retry code (answer) since 1000: Which file holds the log?";

    #[test]
    fn an_agent_kept_open() {
        // 130, 198: the move's line, then until when its agent is kept
        // open for the answer and how to give it.
        assert_eq!(
            from_the_move(&asking(Some(4600), None)),
            vec![
                ASKS.to_string(),
                "    its agent kept open for the answer until 4600: retry with --words, or tell it"
                    .to_string(),
            ]
        );
    }

    #[test]
    fn an_agent_let_go_and_ended() {
        // 130, 198, S104: let go by its time, ended in the same settle.
        let closed = SessionClosed {
            at: 4600,
            by: ClosedBy::Time,
            ended_at: Some(4600),
            left_in_pane: false,
        };
        assert_eq!(
            from_the_move(&asking(None, Some(closed))),
            vec![
                ASKS.to_string(),
                "    its agent let go at 4600, its time passed, and ended at 4600: retry with --words"
                    .to_string(),
            ]
        );
    }

    #[test]
    fn an_agent_let_go_and_to_be_ended_once_quiet() {
        // 31, 146, 198: let go by its account spent while it works, to be
        // ended once a settle reads it quiet.
        let closed = SessionClosed {
            at: 2200,
            by: ClosedBy::AccountSpent,
            ended_at: None,
            left_in_pane: false,
        };
        assert_eq!(
            from_the_move(&asking(None, Some(closed)))[1],
            "    its agent let go at 2200, its account spent, and ended once quiet: retry with --words"
        );
    }

    #[test]
    fn an_agent_stopped_by_its_caller() {
        // 51, 198: a caller's stop lets it go, its agent ended by the stop.
        let closed = SessionClosed {
            at: 1500,
            by: ClosedBy::Stopped,
            ended_at: Some(1500),
            left_in_pane: false,
        };
        assert_eq!(
            from_the_move(&asking(None, Some(closed)))[1],
            "    its agent let go at 1500, its caller stopped it, and ended at 1500: retry with --words"
        );
    }

    #[test]
    fn a_bare_agent_left_in_its_pane() {
        // 77, 198; core.10: a conversation's agent brought back bare in its
        // pane lets it go as ended, and is left there.
        let closed = SessionClosed {
            at: 1800,
            by: ClosedBy::AgentEnded,
            ended_at: None,
            left_in_pane: true,
        };
        assert_eq!(
            from_the_move(&asking(None, Some(closed)))[1],
            "    its agent let go at 1800, its agent ended, and left in its pane: retry with --words"
        );
    }

    #[test]
    fn a_move_with_no_agent_kept_open_gives_no_line_for_one() {
        // 130: a stop-short whose terminal hands a running agent no words
        // gives its move alone.
        let place = asking(None, None);
        let lines = place_lines(&place);
        assert!(lines.contains(&ASKS.to_string()), "{lines:#?}");
        assert!(
            !lines.iter().any(|line| line.contains("its agent")),
            "{lines:#?}"
        );
    }

    #[test]
    fn two_answers() {
        // 42, 56: each answer code's start took in its session on a line of
        // its own beneath the stage, by its act, when it was received, the
        // question and the answer.
        let answer = |question: &str, words: &str, act, at| SessionAnswer {
            question: Some(question.into()),
            answer: AnswerHeld {
                words: words.into(),
                act,
                at,
                session: Some(Handed {
                    sent_at: at,
                    received_at: at + 5,
                }),
                by: None,
                key: None,
            },
        };
        let stage = StageStatus {
            name: "code".into(),
            state: StageState::Begun,
            starts: 1,
            end: None,
            refused_part: None,
            refused_setting: None,
            conversation: false,
            group: None,
            answers: vec![
                answer("Which file holds the log?", "notes.md", Act::Retry, 1200),
                answer("Which test covers it?", "the walkthrough", Act::Tell, 2400),
            ],
        };
        assert_eq!(
            stage_lines(&stage),
            vec![
                "  stage code: begun, started 1 times".to_string(),
                "    answered by retry, received at 1205: asked Which file holds the log?; answered notes.md".to_string(),
                "    answered by tell, received at 2405: asked Which test covers it?; answered the walkthrough".to_string(),
            ]
        );
        // 42: who gave an answer, where it named one.
        let mut from_ops = stage;
        from_ops.answers[1].answer.by = Some("east/ops".into());
        assert_eq!(
            stage_lines(&from_ops)[2],
            "    answered by tell from east/ops, received at 2405: asked Which test covers it?; answered the walkthrough"
        );
    }

    #[test]
    fn a_stop_shorts_end_kept_open_in_the_inbox() {
        // 43, 198: the inbox gives a stop-short's end in the notice's
        // words, until when its agent is kept open among them.
        let end = EndHeld {
            of: EndOf::Stage {
                stage: "code".into(),
                start: 1,
                state: StageState::EndedShort,
                quiet_from: None,
                answered: 0,
                end: Box::new(StageEnd {
                    words: Some("Which file holds the log?".into()),
                    open_until: Some(4600),
                    ..StageEnd::default()
                }),
            },
            read_at: 1000,
            deliveries: vec![],
            answer: None,
            claim: None,
            noticing: vec![],
        };
        let inbox = Answer::Inbox {
            recipient: "ops".into(),
            ends: vec![flume_core::Collected {
                place: PlaceId::new("/d/places/1/fix--one"),
                kind: "fix".into(),
                name: "one".into(),
                end: end_status(&end, &["ops"]),
                confirmed: false,
                expired: false,
            }],
            waiting: vec![],
        };
        assert_eq!(
            person(&inbox),
            "fix--one code needs an answer, its agent kept open for the answer until 4600 [stage/code/1/ended-short]; read: flume status /d/places/1/fix--one"
        );
    }

    #[test]
    fn a_place_waiting_in_the_inbox() {
        // 43, 130, 193, design 9: the inbox gives a place waiting on its
        // recipient with its brief's first line and stream, the move and
        // since when, the stop-short by its key and the role it asked, its
        // agent kept open, and the hold on the move.
        let inbox = Answer::Inbox {
            recipient: "east/ops".into(),
            ends: vec![],
            waiting: vec![WaitingPlace {
                place: PlaceId::new("/d/places/1/change--plan"),
                kind: "change".into(),
                name: "plan".into(),
                stream: "east".into(),
                brief: Some("one of two designs".into()),
                waiting_on: WaitingOn {
                    act: Act::Retry,
                    stage: Some("construct".into()),
                    why: Waits::Answer,
                    since: Some(100),
                    words: Some("Which design does the brief mean?".into()),
                    open_until: Some(3700),
                    closed: None,
                },
                stop_short: Some(StopShortWaited {
                    words: Some("Which design does the brief mean?".into()),
                    needs: Some(StopKind::NeedsAnswer),
                    key: "stage/construct/1/ended-short".parse().unwrap(),
                    addressee: Some("east/ops".into()),
                }),
                hold: Some(Hold {
                    on: HoldOn::Waiting,
                    words: Some("east-planner has it".into()),
                    since: 160,
                    bound: Some(3600),
                    lapsed: None,
                }),
            }],
        };
        assert_eq!(
            person(&inbox),
            [
                "nothing pending for east/ops",
                "change--plan (one of two designs), in east, waits on east/ops; read: flume status /d/places/1/change--plan",
                "  waits on its caller to retry construct (answer) since 100: Which design does the brief mean?",
                "  its stop-short stage/construct/1/ended-short needs an answer, asking east/ops",
                "    its agent kept open for the answer until 3700: retry with --words, or tell it",
                "  the move it waits on on hold since 160, for 3600 seconds: east-planner has it",
            ]
            .join("\n")
        );
    }

    #[test]
    fn a_base_place_says_it_has_no_worktree_and_what_is_on_its_branch() {
        // 56, 202, 203: a base place's status in the person's form; any
        // other place's says nothing of a worktree.
        let base = PlaceStatus {
            worktree: Some(false),
            on_branch: Some(vec![
                PlaceId::new("/d/places/1/change--one"),
                PlaceId::new("/d/places/1/change--two"),
            ]),
            ..a_place()
        };
        assert!(place_lines(&base).contains(
            &"  no worktree; waits on the places on its branch: /d/places/1/change--one, /d/places/1/change--two"
                .to_string()
        ));
        let alone = PlaceStatus {
            on_branch: Some(vec![]),
            ..base
        };
        assert!(place_lines(&alone)
            .contains(&"  no worktree; no place on its branch is live".to_string()));
        assert!(!place_lines(&a_place())
            .iter()
            .any(|line| line.contains("worktree")));
    }

    #[test]
    fn a_fix_landed_in_the_persons_form() {
        // The place-status spec's "A fix landed, in the person's form": a
        // base place whose kind names mend gives the kind of its fixes with
        // where it came from and 1 of 2 fixes, its forecast clear at main's
        // revision, and fix 1 landed as abc123; the JSON the same fix
        // landed with its commit (56, 61, 203).
        let mend = KindFrom {
            declaration_file: Some(".config/flume.toml".into()),
            revision: Some(flume_seams::Revision::new("c1")),
            ..kind_from("repo", "mend", 1)
        };
        let base = PlaceStatus {
            worktree: Some(false),
            on_branch: Some(vec![]),
            fix_kind: Some(FixKind {
                identity: mend.identity.clone(),
                from: mend,
            }),
            fix_limit: Some(2),
            forecast: Some(ForecastHeld {
                at: 300,
                forecast: Forecast::Clear {
                    head: flume_seams::Revision::new("aaa222"),
                    onto: flume_seams::Revision::new("c0ffee"),
                },
            }),
            fixes: vec![BaseFix {
                number: 1,
                name: "spring-1".into(),
                place: PlaceId::new("/d/places/1/mend--spring-1"),
                cause: FixCause::Forecast {
                    head: flume_seams::Revision::new("aaa111"),
                    onto: flume_seams::Revision::new("beef00"),
                },
                launched: 120,
                ended: FixEnded::Landed {
                    commit: flume_seams::Revision::new("abc123"),
                },
            }],
            ..a_place()
        };
        let lines = place_lines(&base);
        for line in [
            "  fix kind repo/mend@1 from the repository at c1, 1 of 2 fixes",
            "  forecast clear at c0ffee, read at 300",
            "  fix 1 mend--spring-1, for a forecast conflict, launched at 120: landed as abc123",
        ] {
            assert!(
                lines.contains(&line.to_string()),
                "{line} not in {lines:#?}"
            );
        }
        let json = serde_json::to_value(&base).expect("status as JSON");
        assert_eq!(json["fixes"][0]["ended"], "landed");
        assert_eq!(json["fixes"][0]["commit"], "abc123");
        let fix = PlaceStatus {
            fix_of: Some(FixOf {
                place: PlaceId::new("/d/places/1/train--spring"),
                number: 1,
                cause: FixCause::Forecast {
                    head: flume_seams::Revision::new("aaa111"),
                    onto: flume_seams::Revision::new("beef00"),
                },
            }),
            ..a_place()
        };
        assert!(place_lines(&fix).contains(
            &"  launched by flume for train--spring (fix 1, a forecast conflict)".to_string()
        ));
        assert!(!place_lines(&a_place())
            .iter()
            .any(|line| line.contains("fix kind") || line.contains("forecast")));
    }

    #[test]
    fn a_base_places_branch_the_merge_queue_kept() {
        // 67, 206: a branch the merge queue keeps at a base place's tidy
        // is named once with its words; a place's with its commits.
        let place = PlaceId::new("/d/places/1/train--spring");
        let tidy = Tidied {
            removed: vec![place.clone()],
            kept: vec![],
            branches_kept: vec![
                flume_core::BranchKept {
                    place: place.clone(),
                    branch: "train/spring".into(),
                    commits: vec![],
                    reason: Some("work main lacks".into()),
                },
                flume_core::BranchKept {
                    place: PlaceId::new("/d/places/1/fix--one"),
                    branch: "fix/one".into(),
                    commits: vec![flume_seams::Revision::new("c4")],
                    reason: None,
                },
            ],
            not_over: vec![],
            terminal: vec![],
        };
        assert_eq!(
            tidy_lines(&tidy),
            [
                "removed /d/places/1/train--spring",
                "kept the branch train/spring of /d/places/1/train--spring, the merge queue keeping it: work main lacks",
                "kept the branch fix/one of /d/places/1/fix--one, 1 commits not landed",
            ]
        );
    }

    #[test]
    fn a_place_kept_while_its_session_runs() {
        // 64, 207: the tidy keeps a place while a process runs its last
        // conversation start's session, naming each process, and the
        // place says so beneath its heading.
        let place = PlaceId::new("/d/places/1/research--one");
        let tidy = Tidied {
            removed: vec![],
            kept: vec![
                flume_core::KeptPlace {
                    place: place.clone(),
                    why: Kept::SessionRuns {
                        processes: vec![by_hand()],
                        unread: None,
                    },
                },
                flume_core::KeptPlace {
                    place: PlaceId::new("/d/places/1/research--two"),
                    why: Kept::SessionRuns {
                        processes: vec![],
                        unread: Some("its agent profile reads no running session".into()),
                    },
                },
            ],
            branches_kept: vec![],
            not_over: vec![],
            terminal: vec![],
        };
        assert_eq!(
            tidy_lines(&tidy),
            [
                "kept /d/places/1/research--one while its session runs: process 4242, started 9000",
                "kept /d/places/1/research--two while its session runs: flume cannot read whether a process runs it: its agent profile reads no running session",
            ]
        );
        let kept = PlaceStatus {
            phase: Phase::Over {
                over: flume_seams::Over::Dropped,
                kept: Some(Kept::SessionRuns {
                    processes: vec![by_hand()],
                    unread: None,
                }),
            },
            ..a_place()
        };
        assert!(place_lines(&kept)
            .contains(&"  kept while its session runs: process 4242, started 9000".to_string()));
    }

    #[test]
    fn a_places_heading_with_its_line() {
        // 56, 60: the place's line under its heading.
        let place = a_place();
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
            answers: vec![],
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
    fn a_hold_on_the_waiting_move_and_its_lapse() {
        // 56, 130: a hold on the move the place waits on, with its words,
        // since when and its bound, and, once lapsed, when it lapsed; in
        // the person's form, and in the JSON form as fields.
        let holding = Hold {
            on: HoldOn::Waiting,
            words: Some("east-planner has it".into()),
            since: 100,
            bound: Some(3600),
            lapsed: None,
        };
        let lapsed = Hold {
            lapsed: Some(3700),
            ..holding.clone()
        };
        let place = PlaceStatus {
            holds: vec![holding.clone()],
            ..a_place()
        };
        assert!(place_lines(&place).contains(
            &"  the move it waits on on hold since 100, for 3600 seconds: east-planner has it"
                .to_string()
        ));
        assert_eq!(
            hold_line(&lapsed),
            "  the move it waits on on hold since 100, for 3600 seconds, lapsed at 3700: east-planner has it"
        );
        assert_eq!(
            hold_line(&Hold {
                bound: None,
                ..holding.clone()
            }),
            "  the move it waits on on hold since 100: east-planner has it"
        );
        let json = serde_json::to_value(PlaceStatus {
            holds: vec![lapsed],
            ..a_place()
        })
        .expect("status as JSON");
        assert_eq!(
            json["holds"],
            json!([{"on": "waiting", "words": "east-planner has it", "since": 100, "bound": 3600, "lapsed": 3700}])
        );
    }

    #[test]
    fn a_stall_with_its_stop_short_and_lapsed_hold() {
        // 41, 56, 130, 193: a stall's line names the hold whose lapse
        // stalled it and the stop-short the place waits on, with the role
        // it named and its words; in the JSON form as fields.
        let stall = EndStatus {
            end: EndHeld {
                of: EndOf::Stall {
                    since: 3700,
                    waiting: "a retry of construct".into(),
                    answerer: Some("east/operator".into()),
                    stop_short: Some(Box::new(flume_seams::StopShortWaited {
                        words: Some("Which design does the brief mean?".into()),
                        needs: Some(StopKind::NeedsAnswer),
                        key: "stage/construct/1/ended-short".parse().unwrap(),
                        addressee: Some("west/design".into()),
                    })),
                    answerer_read: true,
                    lapsed: Some("on the user's card".into()),
                },
                read_at: 3700,
                deliveries: vec![],
                answer: None,
                claim: None,
                noticing: vec![],
            },
            key: "stall/3700".parse().unwrap(),
            pending_for: vec!["east/planner".into()],
            open_until: None,
            confirmed: vec![],
            delivery: None,
            expires_at: None,
            owed: vec![],
        };
        let place = PlaceStatus {
            ends: vec![stall.clone()],
            ..a_place()
        };
        assert!(place_lines(&place).contains(
            &"  ended stall/3700: stalled: a retry of construct, its hold lapsed: on the user's card; its stop-short needs an answer, asking west/design, saying: Which design does the brief mean?; pending for east/planner"
                .to_string()
        ), "{:?}", place_lines(&place));
        let json = serde_json::to_value(&stall).expect("the end as JSON");
        assert_eq!(json["answerer"], json!("east/operator"));
        assert_eq!(json["lapsed"], json!("on the user's card"));
        assert_eq!(
            json["stop_short"],
            json!({
                "words": "Which design does the brief mean?",
                "needs": "needs_answer",
                "key": "stage/construct/1/ended-short",
                "addressee": "west/design"
            })
        );
    }

    #[test]
    fn an_answered_question_with_its_key() {
        // 41, 42, 56: an end's line opens with its key as callers name it,
        // and a stop-short's answer gives when, who gave it, the act and
        // whether it named the key; in the JSON form as fields.
        let answered = EndStatus {
            end: EndHeld {
                of: EndOf::Stage {
                    stage: "construct".into(),
                    start: 1,
                    state: flume_seams::StageState::EndedShort,
                    quiet_from: None,
                    answered: 0,
                    end: Box::new(StageEnd {
                        words: Some("Which queue's settings are live in east?".into()),
                        needs: Some(StopKind::NeedsAnswer),
                        addressee: Some("east/ops".into()),
                        ..StageEnd::default()
                    }),
                },
                read_at: 1000,
                deliveries: vec![],
                answer: Some(AnswerHeld {
                    words: "2.4, as deployed".into(),
                    act: Act::Retry,
                    at: 1600,
                    session: None,
                    by: Some("east/ops".into()),
                    key: Some("stage/construct/1/ended-short".parse().unwrap()),
                }),
                claim: None,
                noticing: vec![],
            },
            key: "stage/construct/1/ended-short".parse().unwrap(),
            pending_for: vec![],
            open_until: None,
            confirmed: vec![],
            delivery: None,
            expires_at: None,
            owed: vec![],
        };
        let place = PlaceStatus {
            ends: vec![answered.clone()],
            ..a_place()
        };
        assert!(
            place_lines(&place).contains(
                &"  ended stage/construct/1/ended-short: construct needs an answer; answered at 1600 by east/ops, by a retry naming its key: 2.4, as deployed"
                    .to_string()
            ),
            "{:?}",
            place_lines(&place)
        );
        let json = serde_json::to_value(&answered).expect("the end as JSON");
        assert_eq!(json["key"], json!("stage/construct/1/ended-short"));
        assert_eq!(
            json["answer"],
            json!({
                "words": "2.4, as deployed",
                "act": "retry",
                "at": 1600,
                "by": "east/ops",
                "key": "stage/construct/1/ended-short"
            })
        );
        // An answer that named no key and no one: its line says neither.
        let mut unnamed = answered;
        if let Some(answer) = unnamed.end.answer.as_mut() {
            answer.by = None;
            answer.key = None;
        }
        let place = PlaceStatus {
            ends: vec![unnamed],
            ..a_place()
        };
        assert!(
            place_lines(&place).contains(
                &"  ended stage/construct/1/ended-short: construct needs an answer; answered at 1600, by a retry: 2.4, as deployed"
                    .to_string()
            ),
            "{:?}",
            place_lines(&place)
        );
    }

    #[test]
    fn a_stop_short_with_the_role_it_named() {
        // 41, 56, 132: the address a stop-short named, whole.
        let stage = StageStatus {
            name: "construct".into(),
            state: flume_seams::StageState::EndedShort,
            starts: 1,
            end: Some(StageEnd {
                words: Some("Which queue is deployed?".into()),
                needs: Some(flume_seams::StopKind::NeedsAnswer),
                addressee: Some("west/ops".into()),
                ..StageEnd::default()
            }),
            refused_part: None,
            refused_setting: None,
            conversation: false,
            group: None,
            answers: vec![],
        };
        assert_eq!(
            stage_lines(&stage),
            vec![
                "  stage construct: ended_short, started 1 times, asking west/ops, saying: Which queue is deployed?"
                    .to_string()
            ]
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
                    sessions: vec![],
                    faults: vec![],
                    priority: 10,
                    running: 4,
                    bound: Some(4),
                    room: Some(0),
                    queue: vec![work],
                    labels: vec![],
                    reserves: Reserves::None,
                    refused: None,
                    view_refused: None,
                    answerer: None,
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
                    sessions: vec![],
                    faults: vec![],
                    priority: 0,
                    running: 2,
                    bound: Some(3),
                    room: Some(1),
                    queue: vec![personal],
                    labels: vec![],
                    reserves: Reserves::None,
                    refused: None,
                    view_refused: None,
                    answerer: None,
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
                    sessions: vec![],
                    faults: vec![],
                    priority: 0,
                    running: 0,
                    bound: None,
                    room: None,
                    queue: vec![],
                    labels: vec![],
                    reserves: Reserves::None,
                    refused: None,
                    view_refused: None,
                    answerer: None,
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
                "  no caller keeps watch: every place settles at each command of flume's that acts".to_string(),
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
            sessions: vec![],
            faults: vec![],
            priority: 0,
            running: 5,
            bound,
            room: bound.map(|_| 0),
            queue: vec![],
            labels: vec![flume_core::LabelStatus {
                name: "operation".into(),
                owed: 2,
                reserved: 1,
                reserve: 2,
                keep_free: 1,
                running: 1,
                waiting: 1,
            }],
            reserves,
            refused: refused.map(str::to_string),
            view_refused: None,
            answerer: None,
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
                    Some("the label operation of the stream nightly holds 2 reserved of the 1 it is owed, and a label holds reserved no more than it is owed"),
                ),
            ],
            ..HostStatus::default()
        };
        let operation = "    label operation: owed 2, reserved 1, 1 agents running, 1 waiting";
        assert_eq!(
            host_lines(&host)[1..host_lines(&host).len() - 1],
            [
                "  stream work, priority 0: 5 agents running of 5, room for 0".to_string(),
                "    its agents open in the session work, the stream's name".to_string(),
                "    multiplexer printed (the host's), view terminal (the default), terminal app printed (the default), title \"work\" (the stream's name)".to_string(),
                operation.to_string(),
                "    waiting for room: /r/places/research--one's work in work, priority 0, short of the stream work's room, the rest held reserved for operation".to_string(),
                "    waiting for room: /r/places/deploy--two's work in work as operation, priority 0, short of the stream work's room, at its bound of 5".to_string(),
                "  stream personal, priority 0: 5 agents running, no bound of its own".to_string(),
                "    its agents open in the session personal, the stream's name".to_string(),
                "    multiplexer printed (the host's), view terminal (the default), terminal app printed (the default), title \"personal\" (the stream's name)".to_string(),
                operation.to_string(),
                "    what its labels are owed is passed over: it states no bound".to_string(),
                "  stream nightly, priority 0: 5 agents running of 1, room for 0".to_string(),
                "    its agents open in the session nightly, the stream's name".to_string(),
                "    multiplexer printed (the host's), view terminal (the default), terminal app printed (the default), title \"nightly\" (the stream's name)".to_string(),
                operation.to_string(),
                "    it starts no stage: the label operation of the stream nightly holds 2 reserved of the 1 it is owed, and a label holds reserved no more than it is owed".to_string(),
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
            open_until: None,
            closed: None,
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
            open_until: None,
            closed: None,
        };
        assert_eq!(
            waiting_line(&dialog, false),
            "  waits on a person to answer the dialog its agent shows in talk (dialog) since 54000:\n    New MCP server found in .mcp.json: docs\n    \u{276f} 1. Use this and all future MCP servers"
        );
    }

    fn listed(repository: &str, name: &str, stage: Option<&str>, rest: &[&str]) -> Listed {
        let line = std::iter::once(name)
            .chain(stage)
            .chain(rest.iter().copied())
            .collect::<Vec<_>>()
            .join(" · ");
        Listed {
            place: PlaceId::new(format!("{repository}/places/{name}")),
            repository: repository.into(),
            name: name.into(),
            kind: "fix@1".into(),
            stage: stage.map(str::to_string),
            state: rest[0].into(),
            waits_on: None,
            kept: None,
            line,
        }
    }

    fn stream_listed(stream: &str, runs: Option<SessionRuns>, line: &str) -> StreamListed {
        StreamListed {
            stream: stream.into(),
            session: SessionHeld {
                name: Some(stream.into()),
                from: SessionFrom::StreamName,
            },
            multiplexer: ProfileHeld {
                profile: "mux".into(),
                from: NamedBy::Neither,
            },
            uncarried: runs
                .is_none()
                .then(|| format!("the stream {stream} names no terminal profile flume carries")),
            runs,
            sessions: vec![],
            line: line.into(),
        }
    }

    /// A stream of each answer runs gives, and one whose multiplexer
    /// names no profile, as the core words their lines (200, 201).
    fn every_stream() -> Vec<StreamListed> {
        let named = |name: &str| name.to_string();
        vec![
            stream_listed(
                "madswan",
                Some(SessionRuns::Runs {
                    name: named("madswan"),
                }),
                "madswan · session madswan · runs",
            ),
            stream_listed(
                "swancloud",
                Some(SessionRuns::NotRunning {
                    name: named("swancloud"),
                }),
                "swancloud · session swancloud · not running",
            ),
            stream_listed(
                "work",
                Some(SessionRuns::NoPart {
                    reason: named("the terminal holds no session"),
                }),
                "work · no session",
            ),
            stream_listed(
                "office",
                Some(SessionRuns::Unread {
                    reason: named("the terminal did not answer"),
                }),
                "office · session office · not read",
            ),
            stream_listed("trial", None, "trial · session trial · no profile for cmux"),
        ]
    }

    #[test]
    fn the_list_by_repository_then_the_streams() {
        // 200, design 7: each repository's path on a line of its own, its
        // places' lines under it, then the streams', each indented by two
        // spaces, every line the answer's own.
        let list = Answer::List {
            places: vec![
                listed(
                    "/code/flume",
                    "change--list",
                    Some("construct"),
                    &["working"],
                ),
                listed(
                    "/code/flume",
                    "chore--rename",
                    None,
                    &["landed", "kept by release"],
                ),
                listed(
                    "/code/boomdrive",
                    "change--records",
                    Some("construct"),
                    &["held", "waits on its caller"],
                ),
            ],
            streams: every_stream(),
        };
        assert_eq!(
            person(&list),
            "/code/flume\n  change--list · construct · working\n  chore--rename · landed · kept by \
             release\n/code/boomdrive\n  change--records · construct · held · waits on its \
             caller\nstreams\n  madswan · session madswan · runs\n  swancloud · session swancloud \
             · not running\n  work · no session\n  office · session office · not read\n  trial · \
             session trial · no profile for cmux"
        );
    }

    #[test]
    fn a_host_with_no_places_lists_none_then_its_streams() {
        // 200, design 7.
        let list = Answer::List {
            places: vec![],
            streams: vec![stream_listed(
                "default",
                Some(SessionRuns::NoPart {
                    reason: "the terminal holds no session".into(),
                }),
                "default · no session",
            )],
        };
        assert_eq!(person(&list), "no places\nstreams\n  default · no session");
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
