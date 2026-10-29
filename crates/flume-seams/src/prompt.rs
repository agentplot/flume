//! A prompt and its parts (27, 28): flume's own prompt vocabulary, not
//! fields of the kind format (core.4).

use serde::{Deserialize, Serialize};

/// A part a stage's prompt may ask for, written `{{<name>}}` (C.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptPart {
    Brief,
    Name,
    Kind,
    Stage,
    Branch,
    Base,
    Place,
    Change,
    Report,
    Words,
    Unready,
    Needs,
    /// The path of `made.jsonl` in the place's own folder, for a stage
    /// that declares what it makes (129).
    Made,
}

impl PromptPart {
    pub const ALL: [PromptPart; 13] = [
        PromptPart::Brief,
        PromptPart::Name,
        PromptPart::Kind,
        PromptPart::Stage,
        PromptPart::Branch,
        PromptPart::Base,
        PromptPart::Place,
        PromptPart::Change,
        PromptPart::Report,
        PromptPart::Words,
        PromptPart::Unready,
        PromptPart::Needs,
        PromptPart::Made,
    ];

    pub fn name(self) -> &'static str {
        match self {
            PromptPart::Brief => "brief",
            PromptPart::Name => "name",
            PromptPart::Kind => "kind",
            PromptPart::Stage => "stage",
            PromptPart::Branch => "branch",
            PromptPart::Base => "base",
            PromptPart::Place => "place",
            PromptPart::Change => "change",
            PromptPart::Report => "report",
            PromptPart::Words => "words",
            PromptPart::Unready => "unready",
            PromptPart::Needs => "needs",
            PromptPart::Made => "made",
        }
    }

    pub fn named(name: &str) -> Option<PromptPart> {
        PromptPart::ALL.into_iter().find(|part| part.name() == name)
    }
}

/// A stage's prompt as flume sends it: the text with every part filled,
/// how many times it was sent and when it was received (38).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prompt {
    pub text: String,
    /// The start carried it in the agent's command (38).
    pub carried: bool,
    pub sendings: Vec<u64>,
    pub received_at: Option<u64>,
}
