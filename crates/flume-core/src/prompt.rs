//! A stage's prompt, filled before it is sent (27 to 29): every part it
//! asks for filled, `{{#words}}…{{/words}}` only when there are words,
//! and the one way to stop short always given, saying it is the agent's
//! one way to ask, with the roles a stop-short may name after every part
//! the stage's repetitions share. The parts are flume's own prompt
//! vocabulary (design 6), not fields of the kind format (core.4).

use flume_seams::{Agent, AnsweringRole, GroupHeld, Identity, PromptPart};

const OPEN: &str = "{{#words}}";
const CLOSE: &str = "{{/words}}";

/// What the way to stop short says after the agent profile's words: that
/// it is the agent's one way to ask for what its stage needs, and that the
/// agent writes no card and no message to a person, since whoever answers
/// decides whether a question is a person's (28, 41, 69, 72).
pub const ONE_WAY_TO_ASK: &str = "Stopping short is your one way to ask for what your stage needs: write no card and no message to a person yourself, since whoever answers your question decides whether it is a person's.";

/// The way to stop short as every prompt gives it: `agent`'s, filled in
/// for `identity`'s place and stage, then [`ONE_WAY_TO_ASK`]; the same for
/// every start of the stage (28, 92; core.31).
pub fn way_to_stop_short(agent: &dyn Agent, identity: &Identity) -> String {
    format!("{} {ONE_WAY_TO_ASK}", agent.stop_short(identity))
}

/// The roles a stop-short of a place in `stream` may name, of every role
/// `answering` gives, the host's sources' roles declared with words of
/// what they answer: all but `own`, the place's own address where it is a
/// role's run; `stream`'s first, then every other stream's, each in its
/// addresses' order (28, 41, 132, 193).
pub(crate) fn may_name(
    answering: Vec<AnsweringRole>,
    stream: &str,
    own: Option<&str>,
) -> Vec<AnsweringRole> {
    let (mut listed, others): (Vec<AnsweringRole>, Vec<AnsweringRole>) = answering
        .into_iter()
        .filter(|role| Some(role.address.as_str()) != own)
        .partition(|role| role.stream == stream);
    listed.extend(others);
    listed
}

/// The address a stop-short of a place in `stream` names by `to`: a role
/// alone that role of `stream`, an address as written (41, 132).
pub(crate) fn addressed(to: &str, stream: &str) -> String {
    if to.contains('/') {
        to.to_string()
    } else {
        format!("{stream}/{to}")
    }
}

/// The names `listed`, each a role's address whole, gives a stop-short of
/// a place in `stream`, as its prompt listed them, joined as a refusal
/// names them (41).
pub(crate) fn listed_names(listed: &[String], stream: &str) -> String {
    let names: Vec<String> = listed
        .iter()
        .map(|address| format!("`{}`", listed_name(address, stream)))
        .collect();
    match names.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}

/// A role's address as a prompt of a place in `stream` lists it: a role
/// of `stream` by the role alone, any other stream's whole (28, 41, 132).
pub(crate) fn listed_name<'a>(address: &'a str, stream: &str) -> &'a str {
    match address.split_once('/') {
        Some((of, role)) if of == stream => role,
        _ => address,
    }
}

/// The roles a stop-short may name, as the prompt of a place in `stream`
/// gives them after every part its repetitions share: `agent`'s way of
/// naming one, then each name as listed with its source's words; none
/// where it lists none (28, 41, 193; core.31).
pub(crate) fn roles_text(
    listed: &[AnsweringRole],
    stream: &str,
    agent: &dyn Agent,
) -> Option<String> {
    if listed.is_empty() {
        return None;
    }
    let names: Vec<String> = listed
        .iter()
        .map(|role| {
            format!(
                "- `{}`: {}",
                listed_name(&role.address, stream),
                role.answers
            )
        })
        .collect();
    Some(format!(
        "Where your words are for one of these roles rather than for the place's recipients, {}:\n{}",
        agent.stop_short_to(),
        names.join("\n")
    ))
}

/// What `{{group}}` renders for a repetition: its group's number and
/// title, `Group <n>. <title>`, and its tasks' lines as the change gives
/// them; for the tasks above every heading, a group with neither, words
/// naming them so; and for a repetition with no group, words saying every
/// task is ticked, that the words below say what to fix, and that the
/// fix is recorded as a new task ticked (140, 141).
pub(crate) fn group_text(group: Option<&GroupHeld>) -> String {
    let Some(group) = group else {
        return "There is no task group for you: every task of the change is ticked. The words below say what to fix; record the fix as a new task in the change, ticked.".into();
    };
    let heading = match (&group.number, &group.title) {
        (Some(number), Some(title)) => format!("Group {number}. {title}"),
        (Some(number), None) => format!("Group {number}."),
        (None, Some(title)) => format!("Group {title}"),
        (None, None) => "The tasks above every heading of the change".into(),
    };
    let tasks: Vec<String> = group.tasks.iter().map(|task| format!("- {task}")).collect();
    format!("{heading}\n\n{}", tasks.join("\n"))
}

/// A task group named in a line, as status, an end's notice and a
/// person's form give it: `group <n>. <title>`, or for the tasks above
/// every heading words naming them so (142).
pub fn group_name(number: Option<&str>, title: Option<&str>) -> String {
    match (number, title) {
        (Some(number), Some(title)) => format!("group {number}. {title}"),
        (Some(number), None) => format!("group {number}"),
        (None, Some(title)) => format!("group {title}"),
        (None, None) => "the tasks above every heading".into(),
    }
}

/// The path of a repeating stage's notes in the place's own folder,
/// which `{{notes}}` renders: flume names it and never creates, reads or
/// removes what is there; it goes with the folder (62, 141; core.31).
pub(crate) fn notes_path(folder: &str, stage: &str) -> String {
    format!("{folder}/notes/{stage}.md")
}

/// Whether `template` asks for `part` bare.
pub(crate) fn asks(template: &str, part: PromptPart) -> bool {
    template.contains(&format!("{{{{{}}}}}", part.name()))
}

/// The template with every part filled by `fill`, or the first part it
/// asks for with nothing to fill it (27).
pub(crate) fn render(
    template: &str,
    fill: impl Fn(PromptPart) -> Option<String>,
) -> Result<String, PromptPart> {
    let words = fill(PromptPart::Words);
    let mut sectioned = String::new();
    let mut rest = template;
    while let Some(open) = rest.find(OPEN) {
        sectioned.push_str(&rest[..open]);
        let after = &rest[open + OPEN.len()..];
        let Some(close) = after.find(CLOSE) else {
            sectioned.push_str(&rest[open..]);
            rest = "";
            break;
        };
        if words.is_some() {
            sectioned.push_str(&after[..close]);
        }
        rest = &after[close + CLOSE.len()..];
    }
    sectioned.push_str(rest);

    let mut text = String::new();
    let mut rest = sectioned.as_str();
    while let Some(open) = rest.find("{{") {
        text.push_str(&rest[..open]);
        let after = &rest[open + 2..];
        let Some(close) = after.find("}}") else {
            text.push_str(&rest[open..]);
            rest = "";
            break;
        };
        let name = &after[..close];
        match PromptPart::named(name) {
            Some(part) => text.push_str(&fill(part).ok_or(part)?),
            None => text.push_str(&rest[open..open + 2 + close + 2]),
        }
        rest = &after[close + 2..];
    }
    text.push_str(rest);
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fill(words: Option<&str>) -> impl Fn(PromptPart) -> Option<String> + '_ {
        move |part| match part {
            PromptPart::Brief => Some("the brief, whole".into()),
            PromptPart::Words => words.map(str::to_string),
            PromptPart::Needs => Some("stop short so".into()),
            _ => None,
        }
    }

    #[test]
    fn every_part_asked_for_is_filled() {
        assert_eq!(
            render("{{brief}} / {{needs}}", fill(None)),
            Ok("the brief, whole / stop short so".into())
        );
    }

    #[test]
    fn a_words_section_renders_only_with_words() {
        let template = "a{{#words}} [{{words}}]{{/words}} b";
        assert_eq!(render(template, fill(None)), Ok("a b".into()));
        assert_eq!(render(template, fill(Some("w"))), Ok("a [w] b".into()));
    }

    #[test]
    fn a_part_with_nothing_to_fill_it_is_named() {
        assert_eq!(
            render("{{brief}} {{words}}", fill(None)),
            Err(PromptPart::Words)
        );
        assert_eq!(render("{{change}}", fill(None)), Err(PromptPart::Change));
    }

    #[test]
    fn the_group_renders_its_number_title_and_tasks() {
        let group = GroupHeld {
            index: 1,
            number: Some("2".into()),
            title: Some("Task groups".into()),
            tasks: vec!["2.1 add the groups".into(), "2.2 read them".into()],
        };
        let text = group_text(Some(&group));
        assert_eq!(
            text,
            "Group 2. Task groups\n\n- 2.1 add the groups\n- 2.2 read them"
        );
        let fill = |part| match part {
            PromptPart::Group => Some(text.clone()),
            _ => None,
        };
        assert_eq!(
            render("Build it.\n\n{{group}}", fill),
            Ok(format!("Build it.\n\n{text}"))
        );
        let above = GroupHeld {
            tasks: vec!["do first".into()],
            ..GroupHeld::default()
        };
        assert_eq!(
            group_text(Some(&above)),
            "The tasks above every heading of the change\n\n- do first"
        );
    }

    #[test]
    fn a_repetition_with_no_group_is_told_every_task_is_ticked() {
        let text = group_text(None);
        assert!(
            text.contains("every task of the change is ticked"),
            "{text}"
        );
        assert!(text.contains("a new task"), "{text}");
        assert!(text.contains("words below"), "{text}");
    }

    #[test]
    fn the_notes_are_named_in_the_places_folder() {
        assert_eq!(
            notes_path("/host/state/flume/r%places%change--plan", "code"),
            "/host/state/flume/r%places%change--plan/notes/code.md"
        );
        let fill = |part| match part {
            PromptPart::Notes => Some(notes_path("/f", "code")),
            _ => None,
        };
        assert_eq!(
            render("Read {{notes}} first.", fill),
            Ok("Read /f/notes/code.md first.".into())
        );
        assert_eq!(render("{{notes}}", |_| None), Err(PromptPart::Notes));
        assert_eq!(render("{{group}}", |_| None), Err(PromptPart::Group));
    }

    fn role(address: &str, answers: &str) -> AnsweringRole {
        AnsweringRole {
            address: address.into(),
            stream: flume_seams::stream_of(address).into(),
            answers: answers.into(),
        }
    }

    fn agent() -> flume_fakes::FakeAgent {
        let host = std::sync::Arc::new(flume_fakes::FakeHost::default());
        flume_fakes::FakeAgent::new(std::sync::Arc::new(flume_fakes::FakeTerminal::new(host)))
    }

    #[test]
    fn the_roles_listed_are_the_streams_first_then_every_others_but_its_own() {
        let answering = vec![
            role("east/design", "design"),
            role("east/ops", "east's systems"),
            role("north/ops", "north's systems"),
            role("west/ops", "west's systems"),
        ];
        let addresses = |listed: Vec<AnsweringRole>| -> Vec<String> {
            listed.into_iter().map(|role| role.address).collect()
        };
        assert_eq!(
            addresses(may_name(answering.clone(), "west", None)),
            ["west/ops", "east/design", "east/ops", "north/ops"]
        );
        assert_eq!(
            addresses(may_name(answering, "east", Some("east/ops"))),
            ["east/design", "north/ops", "west/ops"]
        );
        assert_eq!(listed_name("east/ops", "east"), "ops");
        assert_eq!(listed_name("west/ops", "east"), "west/ops");
    }

    #[test]
    fn the_list_names_each_role_with_its_words_or_is_none() {
        let agent = agent();
        assert_eq!(roles_text(&[], "east", &agent), None);
        let text = roles_text(
            &[
                role("east/ops", "east's systems"),
                role("west/ops", "west's systems"),
            ],
            "east",
            &agent,
        )
        .expect("a list");
        assert!(text.contains(&agent.stop_short_to()), "{text}");
        assert!(
            text.ends_with("\n- `ops`: east's systems\n- `west/ops`: west's systems"),
            "{text}"
        );
    }

    #[test]
    fn the_way_to_stop_short_says_it_is_the_one_way_to_ask() {
        let agent = agent();
        let identity = Identity {
            repository: "r".into(),
            place: "fix--one".into(),
            stage: "code".into(),
            start: 1,
        };
        let way = way_to_stop_short(&agent, &identity);
        assert!(way.starts_with(&agent.stop_short(&identity)), "{way}");
        assert!(way.ends_with(ONE_WAY_TO_ASK), "{way}");
        assert_eq!(
            way,
            way_to_stop_short(
                &agent,
                &Identity {
                    start: 2,
                    ..identity
                }
            ),
            "the way at the stage's second start"
        );
    }

    #[test]
    fn what_is_no_part_is_left_as_written() {
        assert_eq!(
            render("{{other}} {{brief", fill(None)),
            Ok("{{other}} {{brief".into())
        );
        assert!(asks("x {{brief}}", PromptPart::Brief));
        assert!(!asks("x {{#words}}{{/words}}", PromptPart::Words));
    }
}
