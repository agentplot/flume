use crate::{lock, FakeHost, Log};
use flume_seams::{
    AgentCommand, AgentState, Closed, Host, Identity, Opened, PlaceRef, Seconds, Sent, Terminal,
    Watch,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

/// The terminal seam: the agents it runs by identity, each one's state
/// as set or as scheduled by the fake clock, what each shows and holds
/// typed, what was sent to each, and whether it sends, closes and
/// watches. A send clears what the agent holds typed before it types.
pub struct FakeTerminal {
    host: Arc<FakeHost>,
    state: Mutex<TerminalState>,
    pub log: Log,
}

struct TerminalState {
    agents: BTreeMap<String, Running>,
    comes_up: AgentState,
    sends: bool,
    open_refusal: Option<String>,
    close_refused: BTreeSet<String>,
    watch: Watch,
    sent: Vec<(String, String)>,
    session: Option<String>,
}

struct Running {
    state: AgentState,
    schedule: Vec<(Seconds, AgentState)>,
    carried: bool,
    command: AgentCommand,
    beside: Option<String>,
    folder: Option<String>,
    screen: Option<String>,
    typed: String,
    cleared: usize,
    leaves_typed: bool,
}

impl Running {
    fn new(state: AgentState, command: AgentCommand) -> Running {
        Running {
            state,
            schedule: vec![],
            carried: false,
            command,
            beside: None,
            folder: None,
            screen: Some("the agent's screen".into()),
            typed: String::new(),
            cleared: 0,
            leaves_typed: false,
        }
    }
}

impl FakeTerminal {
    pub fn new(host: Arc<FakeHost>) -> FakeTerminal {
        FakeTerminal {
            host,
            state: Mutex::new(TerminalState {
                agents: BTreeMap::new(),
                comes_up: AgentState::Waiting,
                sends: true,
                open_refusal: None,
                close_refused: BTreeSet::new(),
                watch: Watch::None,
                sent: vec![],
                session: None,
            }),
            log: Log::default(),
        }
    }

    /// The state an agent reads in once opened, until set otherwise.
    pub fn comes_up(&self, state: AgentState) {
        lock(&self.state).comes_up = state;
    }

    pub fn set_state(&self, agent: &Identity, state: AgentState) {
        if let Some(running) = lock(&self.state).agents.get_mut(&agent.to_string()) {
            running.state = state;
            running.schedule.clear();
        }
    }

    /// From the clock's `at`, the agent reads `state`.
    pub fn schedule(&self, agent: &Identity, at: Seconds, state: AgentState) {
        if let Some(running) = lock(&self.state).agents.get_mut(&agent.to_string()) {
            running.schedule.push((at, state));
        }
    }

    /// The agent's process ends with no word from it.
    pub fn kill(&self, agent: &Identity) {
        self.set_state(agent, AgentState::Stopped);
    }

    /// The host restarted: no agent the terminal ran survives.
    pub fn forget_all(&self) {
        lock(&self.state).agents.clear();
    }

    /// An agent the terminal resumed under `agent`'s identity, waiting.
    pub fn resume(&self, agent: &Identity) {
        lock(&self.state).agents.insert(
            agent.to_string(),
            Running::new(AgentState::Waiting, AgentCommand::default()),
        );
    }

    /// The terminal cannot send to a running agent: every send is
    /// refused, and every open runs a command carrying its prompt.
    pub fn cannot_send(&self) {
        lock(&self.state).sends = false;
    }

    pub fn refuse_opens(&self, reason: &str) {
        lock(&self.state).open_refusal = Some(reason.into());
    }

    /// The agent does not end when closed.
    pub fn refuse_close(&self, agent: &Identity) {
        lock(&self.state).close_refused.insert(agent.to_string());
    }

    pub fn allow_close(&self, agent: &Identity) {
        lock(&self.state).close_refused.remove(&agent.to_string());
    }

    pub fn set_watch(&self, watch: Watch) {
        lock(&self.state).watch = watch;
    }

    /// The session the calling command runs in, as the terminal names it.
    pub fn set_session(&self, session: Option<&str>) {
        lock(&self.state).session = session.map(str::to_string);
    }

    /// What the terminal shows of the agent, or that it shows nothing.
    pub fn set_screen(&self, agent: &Identity, screen: Option<&str>) {
        if let Some(running) = lock(&self.state).agents.get_mut(&agent.to_string()) {
            running.screen = screen.map(str::to_string);
        }
    }

    /// Text left typed in the agent's input, never submitted, as a
    /// sending the agent did not receive leaves it.
    pub fn leave_typed(&self, agent: &Identity, text: &str) {
        if let Some(running) = lock(&self.state).agents.get_mut(&agent.to_string()) {
            running.typed.push_str(text);
        }
    }

    /// The next send types its text and never submits it, as a prompt
    /// left typed in the pane (S43).
    pub fn leaves_next_typed(&self, agent: &Identity) {
        if let Some(running) = lock(&self.state).agents.get_mut(&agent.to_string()) {
            running.leaves_typed = true;
        }
    }

    /// What the agent holds typed, and how many times a send cleared it.
    pub fn typed(&self, agent: &Identity) -> Option<(String, usize)> {
        lock(&self.state)
            .agents
            .get(&agent.to_string())
            .map(|running| (running.typed.clone(), running.cleared))
    }

    /// The folder the agent's start was opened with (128).
    pub fn folder(&self, agent: &Identity) -> Option<String> {
        lock(&self.state)
            .agents
            .get(&agent.to_string())
            .and_then(|running| running.folder.clone())
    }

    /// The texts sent to the agent, in order.
    pub fn sent_to(&self, agent: &Identity) -> Vec<String> {
        let key = agent.to_string();
        lock(&self.state)
            .sent
            .iter()
            .filter(|(to, _)| *to == key)
            .map(|(_, text)| text.clone())
            .collect()
    }

    /// The command the agent was opened with, and whether it carried its
    /// prompt.
    pub fn opened(&self, agent: &Identity) -> Option<(AgentCommand, bool)> {
        lock(&self.state)
            .agents
            .get(&agent.to_string())
            .map(|running| (running.command.clone(), running.carried))
    }

    pub fn beside(&self, agent: &Identity) -> Option<String> {
        lock(&self.state)
            .agents
            .get(&agent.to_string())
            .and_then(|running| running.beside.clone())
    }

    fn read(&self, key: &str) -> AgentState {
        let now = self.host.now();
        let state = lock(&self.state);
        let Some(running) = state.agents.get(key) else {
            return AgentState::Stopped;
        };
        running
            .schedule
            .iter()
            .filter(|(at, _)| *at <= now)
            .max_by_key(|(at, _)| *at)
            .map(|(_, scheduled)| *scheduled)
            .unwrap_or(running.state)
    }
}

impl Terminal for FakeTerminal {
    fn open(
        &self,
        place: &PlaceRef,
        folder: &str,
        command: &AgentCommand,
        identity: &Identity,
        beside: Option<&str>,
    ) -> Opened {
        self.log.push(format!("open {identity} {}", place.location));
        let key = identity.to_string();
        if self.read(&key) != AgentState::Stopped {
            return Opened::Refused {
                reason: format!("an agent is live under {key}"),
            };
        }
        let mut state = lock(&self.state);
        if let Some(reason) = &state.open_refusal {
            return Opened::Refused {
                reason: reason.clone(),
            };
        }
        let carried = !state.sends;
        let comes_up = state.comes_up;
        let mut running = Running::new(comes_up, command.clone());
        running.carried = carried;
        running.beside = beside.map(str::to_string);
        running.folder = Some(folder.to_string());
        state.agents.insert(key, running);
        Opened::Opened { carried }
    }

    fn send(&self, agent: &Identity, text: &str) -> Sent {
        self.log.push(format!("send {agent}"));
        let key = agent.to_string();
        let live = self.read(&key) != AgentState::Stopped;
        let mut state = lock(&self.state);
        if !state.sends {
            return Sent::Refused {
                reason: "this terminal cannot send to a running agent".into(),
            };
        }
        if !live {
            return Sent::Refused {
                reason: format!("no agent runs under {key}"),
            };
        }
        if let Some(running) = state.agents.get_mut(&key) {
            if !running.typed.is_empty() {
                running.typed.clear();
                running.cleared += 1;
            }
            if std::mem::take(&mut running.leaves_typed) {
                running.typed.push_str(text);
            }
        }
        state.sent.push((key, text.into()));
        Sent::Sent
    }

    fn state(&self, agent: &Identity) -> AgentState {
        self.log.push(format!("state {agent}"));
        self.read(&agent.to_string())
    }

    fn screen(&self, agent: &Identity) -> Option<String> {
        self.log.push(format!("screen {agent}"));
        let state = lock(&self.state);
        if !state.sends {
            return None;
        }
        state
            .agents
            .get(&agent.to_string())
            .and_then(|running| running.screen.clone())
    }

    fn close(&self, agent: &Identity) -> Closed {
        self.log.push(format!("close {agent}"));
        let key = agent.to_string();
        let mut state = lock(&self.state);
        if state.close_refused.contains(&key) {
            return Closed::Refused {
                reason: format!("the agent under {key} did not end"),
            };
        }
        if let Some(running) = state.agents.get_mut(&key) {
            running.state = AgentState::Stopped;
            running.schedule.clear();
        }
        Closed::Closed
    }

    fn watch(&self) -> Watch {
        lock(&self.state).watch
    }

    fn session(&self) -> Option<String> {
        self.log.push("session");
        lock(&self.state).session.clone()
    }
}
