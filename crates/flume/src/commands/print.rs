//! The core's answer printed for a person (61): every fact the JSON holds
//! that a person reads, in lines, and nothing the answer does not hold.
//! A word of the answer's own, a state or a layer, is printed as the JSON
//! spells it.

use flume_core::{
    group_name, AgentStatus, Answer, HostStatus, PlaceStatus, PoolStatus, Queued, StageStatus,
    Tidied, Woke,
};
use flume_seams::{HoldOn, IntentKey, Made, Readiness, UnsettledStart, WaitingOn, Waits};
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
    }
    lines.join("\n")
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

fn place_lines(place: &PlaceStatus) -> Vec<String> {
    let mut lines = vec![format!(
        "{}: {}@{} ({}), {}",
        place.place,
        place.kind,
        place.version,
        word(json!(place.declared)),
        word(json!(place.phase))
    )];
    let mut facts = vec![format!("branch {} from {}", place.branch, place.base)];
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
    lines.push(format!(
        "  stream {} (from {}), {}",
        place.stream.name,
        word(json!(place.stream.from)),
        intent_words(&place.intent)
    ));
    lines.extend(readiness_lines(&place.readiness));
    for stage in &place.stages {
        lines.extend(stage_lines(stage));
    }
    if let Some(agent) = &place.agent {
        lines.push(agent_line(agent));
        for (setting, given) in &agent.settings.from {
            lines.push(format!(
                "    {setting} = {} (from {})",
                given.value,
                word(json!(given.layer))
            ));
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
/// lines give the failed step and the last of its words (12, 56).
fn waiting_line(waiting: &WaitingOn, stalled: bool) -> String {
    let mut line = format!(
        "  waits on its caller to {}{} ({})",
        word(json!(waiting.act)),
        waiting
            .stage
            .as_ref()
            .map(|stage| format!(" {stage}"))
            .unwrap_or_default(),
        word(json!(waiting.why))
    );
    if let Some(since) = waiting.since {
        line.push_str(&format!(" since {since}"));
    }
    if stalled {
        line.push_str(", stalled");
    }
    if let Some(words) = waiting
        .words
        .as_ref()
        .filter(|_| waiting.why != Waits::Unready)
    {
        line.push_str(&format!(": {words}"));
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

/// The intent a place serves, as a person reads it (147).
fn intent_words(intent: &IntentKey) -> String {
    match intent {
        IntentKey::Launch { name, .. } => format!("intent {name} (from the launch)"),
        IntentKey::Base {
            repository, base, ..
        } => format!("intent of {repository} on {base}"),
    }
}

/// The current agent's line: its identity, program, account and state
/// (56, 145).
fn agent_line(agent: &AgentStatus) -> String {
    let mut line = format!("  agent {} ({})", agent.identity, agent.program);
    if let Some(account) = &agent.account {
        line.push_str(&format!(
            " on the account {} of {} (from the {})",
            account.name,
            account.pool,
            word(json!(account.from))
        ));
    }
    line.push_str(&format!(": {}", word(json!(agent.state))));
    if agent.past_limit {
        line.push_str(", past its limit");
    }
    line
}

/// A pool's lines: its strategy, then each account's program, agents
/// running, bound and spent time (56, 144 to 146).
fn pool_lines(pool: &PoolStatus) -> Vec<String> {
    let mut lines = vec![format!("  pool {}, {}:", pool.name, pool.strategy.name())];
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
    }
    lines
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
        for queued in &stream.queue {
            lines.push(format!("    {}", queued_line(queued)));
        }
    }
    for pool in &host.pools {
        lines.extend(pool_lines(pool));
    }
    for kept in &host.terminal_kept {
        lines.push(format!("  the terminal keeps {kept}"));
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
    lines
}

/// A start whose agent began without its settings, each variable by its
/// name alone (56, 161).
fn unsettled_line(unsettled: &UnsettledStart) -> String {
    format!(
        "  without its settings: {}'s start {} began {} at {}, and was not taken as its stage's agent",
        unsettled.stage,
        unsettled.start,
        unsettled.unsettled.words(),
        unsettled.at
    )
}

/// A stage waiting for room, with its stream, its priority and the room
/// it is short of (56, 70, 143).
fn queued_line(queued: &Queued) -> String {
    format!(
        "waiting for room: {}'s {} in {}, priority {}, short of {}",
        queued.place,
        queued.stage,
        queued.stream,
        queued.priority,
        queued.wants.words()
    )
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
        for closed in &terminal.closed {
            lines.push(format!("closed {closed} of {}", terminal.place));
        }
        for kept in &terminal.kept {
            lines.push(format!("kept {kept} of {}", terminal.place));
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use flume_core::{AccountStatus, AgentSettings, MoveStatus, RoomWanted, StreamStatus};
    use flume_seams::{
        AccountFrom, AccountHeld, AccountMove, AgentState, MoveOn, MoveWhy, Reading, ReadyStep,
        Strategy, Unsettled,
    };

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
            printed: None,
            last_words: None,
            notes: vec![],
        };
        assert_eq!(
            agent_line(&agent),
            "  agent flume:r:fix--one:code:2 (agent) on the account work-2 of work (from the strategy): working"
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
            "  without its settings: code's start 1 began without AGENT_HOME at 54000, and was not taken as its stage's agent"
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
    fn the_hosts_pools() {
        let host = HostStatus {
            pools: vec![PoolStatus {
                name: "work".into(),
                strategy: Strategy::FillFirst,
                accounts: vec![
                    AccountStatus {
                        name: "work-1".into(),
                        program: "agent".into(),
                        running: 2,
                        bound: Some(4),
                        spent_until: None,
                    },
                    AccountStatus {
                        name: "work-2".into(),
                        program: "agent".into(),
                        running: 0,
                        bound: None,
                        spent_until: Some(54_000),
                    },
                ],
            }],
            ..HostStatus::default()
        };
        assert_eq!(
            host_lines(&host),
            vec![
                "host: 0 agents running, no bound".to_string(),
                "  pool work, fill-first:".to_string(),
                "    work-1 (agent): 2 agents running of 4".to_string(),
                "    work-2 (agent): 0 agents running, no bound of its own, spent until 54000"
                    .to_string(),
            ]
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
            "no place stands on this host\nhost: 0 agents running, no bound"
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
                    priority: 10,
                    running: 4,
                    bound: Some(4),
                    room: Some(0),
                    queue: vec![work],
                },
                StreamStatus {
                    name: "personal".into(),
                    priority: 0,
                    running: 2,
                    bound: Some(3),
                    room: Some(1),
                    queue: vec![personal],
                },
                StreamStatus {
                    name: "default".into(),
                    priority: 0,
                    running: 0,
                    bound: None,
                    room: None,
                    queue: vec![],
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
                format!("    {w5}"),
                "  stream personal, priority 0: 2 agents running of 3, room for 1".to_string(),
                format!("    {p3}"),
                "  stream default, priority 0: 0 agents running, no bound of its own".to_string(),
            ]
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
