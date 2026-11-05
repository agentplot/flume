//! Each stream's agents run in its own terminal (190; core.49): a
//! start opens in the terminal its stream's multiplexer names as the
//! host's configuration stands then, the host's where the stream names
//! none, and records it; every later call on its agent goes to the
//! terminal it recorded, the host's for a start that records none. A
//! stream whose multiplexer names a terminal profile the resolver does
//! not carry refuses its launches and its starts, naming it, and nothing
//! else (4, 107). The terminals the host binds are its own and each
//! declared stream's, each once: flume runs attended while any of them
//! keeps no watch, the calling command's session is the first that names
//! one, and a recipient is reached in the first that holds its session
//! (7, 43, 119). A place's clues go to the terminal of its live start,
//! else its newest, else its stream's, and its tidy to each terminal its
//! starts recorded (137, 139).

use crate::Engine;
use flume_seams::*;
use std::sync::Arc;

/// A terminal the host binds, with the name of the profile a stream
/// takes it by; none for the host's own where no stream takes it (119,
/// 190).
pub(crate) struct HostTerminal {
    pub profile: Option<String>,
    pub terminal: Arc<dyn Terminal>,
}

/// The terminal that holds a recipient's session, with the session's
/// state (43, 190).
pub(crate) type HeldIn = (Arc<dyn Terminal>, AgentState);

/// Some terminals, each once.
pub(crate) type Terminals = Vec<Arc<dyn Terminal>>;

/// Whether `a` and `b` are one terminal.
fn same(a: &Arc<dyn Terminal>, b: &Arc<dyn Terminal>) -> bool {
    std::ptr::addr_eq(Arc::as_ptr(a), Arc::as_ptr(b))
}

/// `terminal` among `terminals`, added where it is not.
pub(crate) fn add(terminals: &mut Terminals, terminal: Arc<dyn Terminal>) {
    if !terminals.iter().any(|other| same(other, &terminal)) {
        terminals.push(terminal);
    }
}

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

    /// The terminals the host binds, in order: its own, then each stream
    /// it declares whose multiplexer a profile carries, by the stream's
    /// name, each once (119, 190).
    pub(crate) fn host_terminals(&self) -> Vec<HostTerminal> {
        let mut bound = vec![HostTerminal {
            profile: None,
            terminal: self.terminal.clone(),
        }];
        for stream in self.host.streams().keys() {
            let Ok((terminal, named)) = self.stream_terminal(stream) else {
                continue;
            };
            match bound
                .iter_mut()
                .find(|bound| same(&bound.terminal, &terminal))
            {
                Some(bound) => {
                    bound.profile.get_or_insert(named.profile);
                }
                None => bound.push(HostTerminal {
                    profile: Some(named.profile),
                    terminal,
                }),
            }
        }
        bound
    }

    /// Whether flume runs attended: a terminal the host binds, its own or
    /// a stream's, keeps no watch (119, 190).
    pub(crate) fn attended(&self) -> bool {
        self.host_terminals()
            .iter()
            .any(|bound| bound.terminal.watch() == Watch::None)
    }

    /// The calling command's session, as the first terminal the host
    /// binds that names one names it (7, 190).
    pub(crate) fn caller_session(&self) -> Option<String> {
        self.host_terminals()
            .iter()
            .find_map(|bound| bound.terminal.session())
    }

    /// The terminal that holds the recipient's session named `recipient`,
    /// the first the host binds that holds it, with the session's state;
    /// none where none holds it (43, 95, 190).
    pub(crate) fn recipient_terminal(&self, recipient: &str) -> Option<HeldIn> {
        self.host_terminals().into_iter().find_map(|bound| {
            bound
                .terminal
                .session_state(recipient)
                .map(|state| (bound.terminal, state))
        })
    }

    /// The terminal a place's clues go to: its live start's, else its
    /// newest start's, else its stream's as the host's configuration
    /// stands; or why none answers for it (139, 190).
    pub(crate) fn place_terminal(&self, held: &HeldState) -> Result<Arc<dyn Terminal>, String> {
        let starts = || held.stages.iter().filter_map(|stage| stage.start.as_ref());
        match starts()
            .find(|start| start.live)
            .or_else(|| starts().max_by_key(|start| start.started_at))
        {
            Some(start) => self.terminal_of(start),
            None => self
                .stream_terminal(&held.place.stream.name)
                .map(|(terminal, _)| terminal),
        }
    }

    /// The terminals a place's tidy asks, each once: each its stages'
    /// starts recorded, the host's for a start that records none, and its
    /// stream's as the host's configuration stands where it has no start;
    /// and why each the tidy cannot reach does not answer (137, 190).
    pub(crate) fn tidy_terminals(&self, held: &HeldState) -> (Terminals, Vec<String>) {
        let mut terminals = vec![];
        let mut unreached = vec![];
        let mut started = false;
        for start in held.stages.iter().filter_map(|stage| stage.start.as_ref()) {
            started = true;
            match self.terminal_of(start) {
                Ok(terminal) => add(&mut terminals, terminal),
                Err(reason) => unreached.push(reason),
            }
        }
        if !started {
            match self.stream_terminal(&held.place.stream.name) {
                Ok((terminal, _)) => add(&mut terminals, terminal),
                Err(reason) => unreached.push(reason),
            }
        }
        (terminals, unreached)
    }
}
