//! A stage's prompt, filled before it is sent (27 to 29): every part it
//! asks for filled, `{{#words}}…{{/words}}` only when there are words,
//! and the one way to stop short always given. The parts are flume's own
//! prompt vocabulary (design 6), not fields of the kind format (core.4).

use flume_seams::PromptPart;

const OPEN: &str = "{{#words}}";
const CLOSE: &str = "{{/words}}";

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
    fn what_is_no_part_is_left_as_written() {
        assert_eq!(
            render("{{other}} {{brief", fill(None)),
            Ok("{{other}} {{brief".into())
        );
        assert!(asks("x {{brief}}", PromptPart::Brief));
        assert!(!asks("x {{#words}}{{/words}}", PromptPart::Words));
    }
}
