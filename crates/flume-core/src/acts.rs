//! The caller's acts on a place after its launch (47 to 55, 109, 114),
//! and the prompt a stage would be sent (29).

use crate::start::{Start, Starts};
use crate::{Act, Answer, Engine, Step};
use flume_seams::*;

/// What a retry names (49).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Retry {
    /// The current stage.
    Current,
    Stage(String),
    Submission,
}

impl Engine {
    /// The exact prompt the stage would be sent at its next start,
    /// rendered with nothing started, sent, settled or written (29,
    /// 119).
    pub fn prompt(&self, place: &PlaceId, stage: &str) -> Answer {
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let held = read.state;
        let Some(index) = held.stages.iter().position(|held| held.name == stage) else {
            return Answer::refused(format!("the place {place} has no stage {stage}"));
        };
        let identity = self.next_identity(&held, index);
        match self.fill(&held, index, &identity, None) {
            Ok(text) => Answer::Prompt {
                place: place.clone(),
                stage: stage.into(),
                text,
            },
            Err(part) => Answer::refused(format!(
                "the stage {stage}'s prompt asks for {}, which has nothing to fill it",
                part.name()
            )),
        }
    }

    /// Starts the stage a retry names with a fresh agent and the retry's
    /// words, the stages after it then running again in order (49).
    pub fn retry(
        &self,
        place: &PlaceId,
        target: Retry,
        words: Option<String>,
        force: bool,
    ) -> Answer {
        let _ = force;
        self.settle_attended();
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let held = read.state;
        if !matches!(held.phase, Phase::Live) {
            return Answer::refused(format!("the work of the place {place} is over"));
        }
        if let Some((_, start)) = held.live_start() {
            return Answer::refused(format!(
                "the agent {} of the place {place} is live",
                start.identity
            ));
        }
        let index = match &target {
            Retry::Current => held.current.unwrap_or(0),
            Retry::Stage(name) => match held.stages.iter().position(|stage| &stage.name == name) {
                Some(index) => index,
                None => return Answer::refused(format!("the place {place} has no stage {name}")),
            },
            Retry::Submission => {
                return Answer::refused(format!("the place {place} cannot be submitted yet"))
            }
        };
        self.update(place, |held| {
            for stage in held.stages.iter_mut().skip(index + 1) {
                stage.state = StageState::NotReached;
            }
            Step::Write(())
        });
        match self.start_stage(place, index, words) {
            Start::Opened(_) => self.acted(Act::Retry, place),
            Start::Refused(part) => Answer::refused(format!(
                "the stage {}'s prompt asks for {}, which has nothing to fill it",
                held.stages[index].name,
                part.name()
            )),
            Start::AgentLive => Answer::refused(format!("an agent of the place {place} is live")),
            Start::NotOpened(reason) => {
                Answer::refused(format!("the terminal did not open the agent: {reason}"))
            }
            Start::NoAgent(program) => {
                Answer::refused(format!("no agent profile runs the program {program}"))
            }
            Start::NoPlace => Answer::refused(format!("no place {place} stands on this host")),
        }
    }

    /// The answer to an act taken: the place's status as it then stands.
    pub(crate) fn acted(&self, act: Act, place: &PlaceId) -> Answer {
        Answer::Acted {
            act,
            place: place.clone(),
            status: self
                .store
                .read(place)
                .map(|read| self.place_status(&read.state)),
            tidy: None,
        }
    }
}
