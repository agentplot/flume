//! The core's answer printed for a person (61): every fact the JSON holds
//! that a person reads, in lines, and nothing the answer does not hold.
//! A word of the answer's own, a state or a layer, is printed as the JSON
//! spells it.

use flume_core::{Answer, HostStatus, PlaceStatus, Tidied};
use flume_seams::{HoldOn, Made};
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
    for step in &place.readiness.steps {
        let outcome = if step.passed { "passed" } else { "failed" };
        lines.push(
            format!("  readiness {}: {outcome} {}", step.step, step.words)
                .trim_end()
                .to_string(),
        );
    }
    for stage in &place.stages {
        let mut line = format!(
            "  stage {}: {}, started {} times",
            stage.name,
            word(json!(stage.state)),
            stage.starts
        );
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
        lines.push(line);
        for made in stage.end.iter().flat_map(|end| &end.made) {
            let line = match made {
                Made::Thing(thing) => json!(thing).to_string(),
                Made::Text(text) => text.clone(),
            };
            lines.push(format!("    made: {line}"));
        }
    }
    if let Some(agent) = &place.agent {
        let mut line = format!(
            "  agent {} ({}): {}",
            agent.identity,
            agent.program,
            word(json!(agent.state))
        );
        if agent.past_limit {
            line.push_str(", past its limit");
        }
        lines.push(line);
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
        if place.stalled {
            line.push_str(", stalled");
        }
        if let Some(words) = &waiting.words {
            line.push_str(&format!(": {words}"));
        }
        lines.push(line);
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

fn host_lines(host: &HostStatus) -> Vec<String> {
    let mut lines = vec![match (host.bound, host.room) {
        (Some(bound), Some(room)) => format!(
            "host: {} agents running of {bound}, room for {room}",
            host.running
        ),
        _ => format!("host: {} agents running, no bound", host.running),
    }];
    for queued in &host.queue {
        lines.push(format!(
            "  waiting for room: {}'s {}, priority {}",
            queued.place, queued.stage, queued.priority
        ));
    }
    for kept in &host.terminal_kept {
        lines.push(format!("  the terminal keeps {kept}"));
    }
    lines
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
    fn a_word_as_the_json_spells_it() {
        assert_eq!(word(json!("on_hold")), "on_hold");
        assert_eq!(word(json!({"live": {}})), "live");
        assert_eq!(
            word(json!({"over": {"over": "done"}})),
            r#"over {"over":"done"}"#
        );
    }
}
