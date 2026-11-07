//! A role's address a standing agent holds (43, 107, 132, 143, 190, 193,
//! C.7; core.11, core.28, core.49; record 0039's amendment). A source's
//! roles file names the agent by the name its terminal knows it by: it is
//! sought in its address's stream, a stream the host does not declare
//! reaching nothing; in that stream's terminal, as a start reads it; and
//! in the session the entry names, else the stream's own, as a start
//! resolves it. The terminal gives the session's agents by name, and the
//! core counts those of the entry's name: one is found, none or more than
//! one leave its ends pending. Each is read as the sources stand at each
//! delivery and each status, and nothing of
//! where the agent was is kept, so an agent revived under its name is
//! found where it is. flume opens, starts, wakes and watches nothing for
//! the address (core.13).

use crate::room::StreamRoom;
use crate::{Engine, StandingFound, StandingStatus};
use flume_seams::*;
use std::collections::BTreeMap;
use std::sync::Arc;

/// The host's standing entries as one delivery or one status reads them:
/// the entries read once, the streams the host declares read at the
/// first entry reached, and each part of a terminal asked for its agents
/// once (132; core.15).
pub(crate) struct Standing {
    entries: BTreeMap<String, StandingAgent>,
    streams: Option<BTreeMap<String, Stream>>,
    asked: Vec<Asked>,
}

/// A part of a terminal asked for its agents, and what it answered.
struct Asked {
    terminal: Arc<dyn Terminal>,
    session: Option<String>,
    agents: AgentsIn,
}

impl Standing {
    /// Each address the entries give a standing agent.
    pub fn addresses(&self) -> Vec<String> {
        self.entries.keys().cloned().collect()
    }

    /// The agents the part `session` of `terminal` holds, asked once for
    /// each terminal and part.
    fn agents_in(&mut self, terminal: &Arc<dyn Terminal>, session: Option<&str>) -> &AgentsIn {
        let at = match self.asked.iter().position(|asked| {
            std::ptr::addr_eq(Arc::as_ptr(&asked.terminal), Arc::as_ptr(terminal))
                && asked.session.as_deref() == session
        }) {
            Some(at) => at,
            None => {
                self.asked.push(Asked {
                    terminal: terminal.clone(),
                    session: session.map(str::to_string),
                    agents: terminal.agents_in(session),
                });
                self.asked.len() - 1
            }
        };
        &self.asked[at].agents
    }
}

/// A standing agent sought: where, and what was found, as status gives
/// it; and the terminal its one agent is reached on, none where none was
/// found (132, 190).
pub(crate) struct Reach {
    pub status: StandingStatus,
    pub terminal: Option<Arc<dyn Terminal>>,
}

/// A recipient a notice can go to: the terminal that holds it, its state
/// there, and the session name the notice is sent to, a standing agent's
/// `at` or a session's own name (43, 95, 132).
#[derive(Clone)]
pub(crate) struct Reached {
    pub terminal: Arc<dyn Terminal>,
    pub state: AgentState,
    pub at: String,
}

/// What the agents `agents` give of the name `agent`: one, none, several
/// or not read (132).
fn count(agents: &AgentsIn, agent: &str) -> StandingFound {
    let agents = match agents {
        AgentsIn::Read { agents } => agents,
        AgentsIn::Unread { reason } => {
            return StandingFound::Unread {
                reason: reason.clone(),
            }
        }
    };
    let named: Vec<&NamedAgent> = agents.iter().filter(|named| named.name == agent).collect();
    match named.as_slice() {
        [] => StandingFound::None,
        [one] => StandingFound::One {
            at: one.at.clone(),
            state: one.state,
        },
        several => StandingFound::Several {
            count: several.len(),
        },
    }
}

impl Standing {
    /// The standing entries `roles` declares, read once for one status
    /// (132, 193; core.15).
    pub(crate) fn of(roles: &HostRoles) -> Standing {
        Standing {
            entries: roles.standing(),
            streams: None,
            asked: vec![],
        }
    }
}

impl Engine {
    /// The host's standing entries as its sources stand now, for one
    /// delivery or one status (132, 193).
    pub(crate) fn standing(&self) -> Standing {
        Standing::of(&self.host.roles())
    }

    /// The standing agent the entries give `address`, sought in its
    /// address's stream's terminal and session; none where the entries
    /// name no standing agent for it (107, 132, 143, 190, 193).
    pub(crate) fn reach(&self, standing: &mut Standing, address: &str) -> Option<Reach> {
        let entry = standing.entries.get(address)?.clone();
        let stream = stream_of(address).to_string();
        let declared = standing
            .streams
            .get_or_insert_with(|| self.host.streams())
            .get(&stream)
            .cloned();
        let session = match &entry.session {
            Some(name) => SessionHeld {
                name: Some(name.clone()),
                from: SessionFrom::Role,
            },
            None => {
                StreamRoom::of(&stream, declared.clone().unwrap_or_default()).own_session(&stream)
            }
        };
        let status = |terminal: Option<String>, found| StandingStatus {
            agent: entry.agent.clone(),
            stream: stream.clone(),
            session: session.clone(),
            terminal,
            found,
        };
        if declared.is_none() {
            return Some(Reach {
                status: status(
                    None,
                    StandingFound::Unread {
                        reason: format!("the host declares no stream {stream}"),
                    },
                ),
                terminal: None,
            });
        }
        let (terminal, named) = match self.stream_terminal(&stream) {
            Ok(found) => found,
            Err(reason) => {
                return Some(Reach {
                    status: status(
                        Some(self.host.display_for(&stream).multiplexer.profile),
                        StandingFound::Unread { reason },
                    ),
                    terminal: None,
                })
            }
        };
        let found = count(
            standing.agents_in(&terminal, session.name.as_deref()),
            &entry.agent,
        );
        let terminal = matches!(found, StandingFound::One { .. }).then_some(terminal);
        Some(Reach {
            status: status(Some(named.profile), found),
            terminal,
        })
    }

    /// Where a notice for `recipient` goes: the one standing agent the
    /// entries give it, where they give it one, and otherwise the first
    /// terminal the host binds that holds a session of its name; none
    /// where neither is found. Which agent holds an address is the
    /// configuration's at each delivery, so a standing agent takes an
    /// address's ends over a live run of an earlier entry (43, 132, 190).
    pub(crate) fn reached(&self, standing: &mut Standing, recipient: &str) -> Option<Reached> {
        match self.reach(standing, recipient) {
            Some(Reach {
                terminal: Some(terminal),
                status:
                    StandingStatus {
                        found: StandingFound::One { at, state },
                        ..
                    },
            }) => Some(Reached {
                terminal,
                state,
                at,
            }),
            Some(_) => None,
            None => self
                .recipient_terminal(recipient)
                .map(|(terminal, state)| Reached {
                    terminal,
                    state,
                    at: recipient.to_string(),
                }),
        }
    }
}
