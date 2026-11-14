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
//!
//! A command that names no recipient, run in a standing agent's pane,
//! acts as the address the agent holds (7, 132; core.28): the agent is
//! the one its address's delivery reaches, at the session name its
//! stream's terminal gives it, and the command runs in its pane where
//! that terminal names the command's session so. An agent its sources
//! give several addresses holds none by itself, and its command names
//! which.

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

/// What the standing agent a command runs in holds: nothing, where the
/// command runs in no pane a standing agent of a declared address is
/// found at; the one address, with the agent and the source that
/// declares it; or, where the sources give the agent several addresses,
/// each of them, sorted (7, 132; core.28).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum StandingHere {
    None,
    One {
        address: String,
        agent: String,
        source: String,
    },
    Several {
        agent: String,
        addresses: Vec<String>,
    },
}

impl StandingHere {
    /// The address held, none where none is; or the refusal of a command
    /// in the pane of an agent standing for several, naming the agent and
    /// each address and asking for `--recipient` (132; core.28).
    pub(crate) fn address(self) -> Result<Option<String>, String> {
        match self {
            StandingHere::None => Ok(None),
            StandingHere::One { address, .. } => Ok(Some(address)),
            StandingHere::Several { agent, addresses } => Err(several_words(&agent, &addresses)),
        }
    }
}

/// The refusal of a command naming no recipient in the pane of `agent`,
/// which stands for each of `addresses` (132; core.28).
pub(crate) fn several_words(agent: &str, addresses: &[String]) -> String {
    let named = match addresses {
        [] => String::new(),
        [one] => one.clone(),
        [before @ .., last] => format!("{} and {last}", before.join(", ")),
    };
    format!(
        "the agent {agent} this command runs in stands for {named}: name which with --recipient"
    )
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

    /// What the standing agent the calling command runs in holds, read
    /// from the entries `standing` gives (7, 132; core.15, core.28; design
    /// 1, 4, 5). Nothing is asked where the entries give no address a
    /// standing agent. For each address that does, in a stream the host
    /// declares, its stream's terminal is asked the session it names for
    /// the command, and an address whose terminal names none is passed
    /// over unasked; else its agent is reached as a delivery reaches it,
    /// each part of a terminal asked once, and the command's pane holds
    /// the address where that one agent is found at the session the
    /// terminal names for the command. An address none or several of
    /// whose agents are found, or whose part is not read, holds nothing,
    /// and nothing is kept.
    pub(crate) fn standing_here(&self, mut standing: Standing) -> StandingHere {
        if standing.entries.is_empty() {
            return StandingHere::None;
        }
        let mut held: Vec<(String, StandingAgent)> = vec![];
        for (address, entry) in standing.entries.clone() {
            let stream = stream_of(&address);
            if !standing
                .streams
                .get_or_insert_with(|| self.host.streams())
                .contains_key(stream)
            {
                continue;
            }
            let Some(session) = self
                .stream_terminal(stream)
                .ok()
                .and_then(|(terminal, _)| terminal.session())
            else {
                continue;
            };
            if let Some(Reach {
                terminal: Some(_),
                status:
                    StandingStatus {
                        found: StandingFound::One { at, .. },
                        ..
                    },
            }) = self.reach(&mut standing, &address)
            {
                if at == session {
                    held.push((address, entry));
                }
            }
        }
        match held.as_slice() {
            [] => StandingHere::None,
            [(address, entry)] => StandingHere::One {
                address: address.clone(),
                agent: entry.agent.clone(),
                source: entry.source.clone(),
            },
            [(_, entry), ..] => StandingHere::Several {
                agent: entry.agent.clone(),
                addresses: held.iter().map(|(address, _)| address.clone()).collect(),
            },
        }
    }

    /// Where a notice for `recipient` goes: the one standing agent the
    /// entries give it, where they give it one, and otherwise the first
    /// terminal the host binds that holds a session of its name; none
    /// where neither is found, and none while the session is a role's
    /// run whose current start has not taken its prompt, which a notice
    /// typed now would come before (37, 38). Which agent holds an
    /// address is the configuration's at each delivery, so a standing
    /// agent takes an address's ends over a live run of an earlier entry
    /// (43, 132, 190).
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
            None if is_address(recipient) && self.awaits_its_prompt(recipient) => None,
            None => self
                .recipient_terminal(recipient)
                .map(|(terminal, state)| Reached {
                    terminal,
                    state,
                    at: recipient.to_string(),
                }),
        }
    }

    /// Whether the address's run stands with its current start live and
    /// its prompt not yet received: starting, or started and its prompt
    /// sent or still to send. Its agent may wait in its session already,
    /// and would take a notice typed into it as its prompt, its stage's
    /// own then never received (37, 38, 43, 132).
    fn awaits_its_prompt(&self, address: &str) -> bool {
        self.store.read_every().into_iter().any(|read| {
            let held = read.state;
            held.role.as_deref() == Some(address)
                && !matches!(held.phase, Phase::Tidied { .. })
                && held.current.is_some_and(|index| {
                    let stage = &held.stages[index];
                    stage.start.as_ref().is_some_and(|start| start.live)
                        && matches!(stage.state, StageState::Starting | StageState::Started)
                })
        })
    }
}
