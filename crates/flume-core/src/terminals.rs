//! Each stream's agents run in its own terminal (190; core.49): a
//! start opens in the terminal its stream's multiplexer names as the
//! host's configuration stands then, the host's where the stream names
//! none, and records it; every later call on its agent goes to the
//! terminal it recorded, the host's for a start that records none. A
//! stream whose multiplexer names a terminal profile the resolver does
//! not carry refuses its launches and its starts, naming it, and nothing
//! else (4, 107).

use crate::Engine;
use flume_seams::*;
use std::sync::Arc;

/// Why the stream `stream` runs no agent: its multiplexer, `named`,
/// names a terminal profile none of `carried` is (4, 190).
pub(crate) fn uncarried(stream: &str, named: &ProfileHeld, carried: &[String]) -> String {
    let profile = &named.profile;
    let names = match named.from {
        NamedBy::Stream => format!("the stream {stream} names the multiplexer {profile}"),
        NamedBy::Host => {
            format!("the stream {stream} takes the host's multiplexer {profile}")
        }
        NamedBy::Neither => format!("the stream {stream} takes the multiplexer {profile}"),
    };
    format!(
        "{names}, and no terminal profile is named {profile}; the terminal profiles there are: {}",
        carried.join(", ")
    )
}

/// A start's agent on the terminal it opened in, or why no profile
/// answers for that terminal now: its state is then unknown, and every
/// call on it is refused naming the profile (190).
pub(crate) struct AgentOn<'a> {
    identity: &'a Identity,
    terminal: Result<Arc<dyn Terminal>, String>,
}

impl AgentOn<'_> {
    pub fn send(&self, text: &str) -> Sent {
        match &self.terminal {
            Ok(terminal) => terminal.send(self.identity, text),
            Err(reason) => Sent::Refused {
                reason: reason.clone(),
            },
        }
    }

    pub fn state(&self) -> AgentState {
        match &self.terminal {
            Ok(terminal) => terminal.state(self.identity),
            Err(_) => AgentState::Unknown,
        }
    }

    pub fn screen(&self) -> Option<String> {
        self.terminal.as_ref().ok()?.screen(self.identity)
    }

    pub fn answer_trust(&self) -> bool {
        self.terminal
            .as_ref()
            .is_ok_and(|terminal| terminal.answer_trust(self.identity))
    }

    pub fn close(&self) -> Closed {
        match &self.terminal {
            Ok(terminal) => terminal.close(self.identity),
            Err(reason) => Closed::Refused {
                reason: reason.clone(),
            },
        }
    }
}

impl Engine {
    /// The terminal the start's agent opened in: the profile it recorded
    /// through the resolver, the host's own where it records none, as a
    /// start written before does; or why none answers for it, a profile
    /// the resolver no longer carries (56, 190).
    pub(crate) fn terminal_of(&self, start: &StartHeld) -> Result<Arc<dyn Terminal>, String> {
        let Some(held) = &start.terminal else {
            return Ok(self.terminal.clone());
        };
        self.resolver.terminal(&held.profile).ok_or_else(|| {
            format!(
                "the agent {} opened in the terminal {}, and no terminal profile is named {} now; the terminal profiles there are: {}",
                start.identity,
                held.profile,
                held.profile,
                self.resolver.terminal_profiles().join(", ")
            )
        })
    }

    /// The start's agent on the terminal it opened in (190).
    pub(crate) fn agent_on<'a>(&self, start: &'a StartHeld) -> AgentOn<'a> {
        AgentOn {
            identity: &start.identity,
            terminal: self.terminal_of(start),
        }
    }

    /// The terminal the stream `stream`'s agents open in as the host's
    /// configuration stands now, with where its name came from; or why
    /// none does, its multiplexer naming a profile the resolver does not
    /// carry (4, 107, 190).
    pub(crate) fn stream_terminal(
        &self,
        stream: &str,
    ) -> Result<(Arc<dyn Terminal>, ProfileHeld), String> {
        let named = self.host.display_for(stream).multiplexer;
        match self.resolver.terminal(&named.profile) {
            Some(terminal) => Ok((terminal, named)),
            None => Err(uncarried(
                stream,
                &named,
                &self.resolver.terminal_profiles(),
            )),
        }
    }
}
