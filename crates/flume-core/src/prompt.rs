//! A stage's prompt, filled before it is sent (27 to 29): every part it
//! asks for filled, `{{#words}}…{{/words}}` only when there are words,
//! and the one way to stop short always given. The parts are flume's own
//! prompt vocabulary (design 6), not fields of the kind format (core.4).

use flume_seams::{GroupHeld, PromptPart};

const OPEN: &str = "{{#words}}";
const CLOSE: &str = "{{/words}}";

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
