use crate::{lock, FakeHost, Log};
use flume_seams::{
    AgentCommand, AgentState, Attached, Closed, Clues, Host, Identity, OpenIn, Opened, PlaceRef,
    Seconds, Sent, SessionRuns, Shown, ShownAt, Terminal, TerminalTidy, ToSession, View,
    ViewCommand, Watch,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

/// The terminal seam: the agents it runs by identity, each one's state
/// as set or as scheduled by the fake clock, what each shows and holds
/// typed, what was sent to each, and whether it sends, closes and
/// watches. A send clears what the agent holds typed before it types.
/// Each open is logged with where it was asked to open, beside a pane,
/// in a session or in the terminal's own. An open not beside a pane
/// makes a tab for its place in a workspace for its repository and base,
/// in the session it names or the terminal's own; a tidy closes the
/// place's tab in every session, and each workspace once no tab of a
/// live place and no person's pane is in it (107, 137, 138).
/// What show was given is kept, and a show may be made to fail. It
/// holds recipients' sessions by name, each with a state a test sets,
/// and keeps every text sent to each in order; an agent opened under a
/// session name is that session while it is live (43, 95, 132). Attach
/// answers a terminal view with a command of its own, `fake-terminal
/// attach <session>` or `attach own`, with no environment, and the app's
/// view shown, or either as scripted, and logs each asking with its view
/// (187).
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
    opens_as: OpensAs,
    /// The tabs made, oldest first, each a place's in one session.
    tabs: Vec<Tab>,
    /// The workspaces made, each by its session and `<repository> <base>`.
    workspaces: BTreeSet<Workspace>,
    /// A person's panes, by the workspace they stand in.
    persons: BTreeMap<Workspace, String>,
    shows_fail: bool,
    shown: Vec<(String, Clues)>,
    /// Recipients' sessions by name, each with its state.
    sessions: BTreeMap<String, AgentState>,
    /// The agent each session name an open gave names, by its identity.
    agent_sessions: BTreeMap<String, String>,
    /// Every text sent to a session by its name, in order.
    notices: Vec<(String, String)>,
    /// What every attach for a terminal view answers, and for the app's,
    /// the fake's own answer where none.
    attaches_terminal: Option<Attached>,
    attaches_app: Option<Attached>,
    /// What the terminal's configuration lacks for flume's plugin, as
    /// scripted.
    setup: Vec<String>,
}

/// What the next opens answer.
#[derive(Clone)]
enum OpensAs {
    Running,
    Held(String),
    NotComeUp(String),
}

/// The workspace label of a place's repository and base.
fn workspace(place: &PlaceRef) -> String {
    let repository = place
        .repository
        .rsplit('/')
        .find(|part| !part.is_empty())
        .unwrap_or(&place.repository);
    format!("{repository} {}", place.base)
}

/// A workspace by the session it stands in, none for the terminal's own,
/// and its label.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Workspace {
    session: Option<String>,
    label: String,
}

impl Workspace {
    fn of(place: &PlaceRef, session: Option<&str>) -> Workspace {
        Workspace {
            session: session.map(str::to_string),
            label: workspace(place),
        }
    }

    /// The workspace as the fake names it: its label, and the session it
    /// stands in where that is not the terminal's own.
    fn named(&self) -> String {
        in_session(&self.label, self.session.as_deref())
    }
}

/// `name`, and the session it stands in where that is not the
/// terminal's own.
fn in_session(name: &str, session: Option<&str>) -> String {
    match session {
        Some(session) => format!("{name} in {session}"),
        None => name.to_string(),
    }
}

/// A place's tab in one workspace.
struct Tab {
    location: String,
    name: String,
    workspace: Workspace,
}

/// The session an open made its tab in, none for the terminal's own;
/// none of either beside a pane.
fn session_of(at: &OpenIn) -> Option<Option<&str>> {
    match at {
        OpenIn::Beside { .. } => None,
        OpenIn::Session { name } => Some(Some(name)),
        OpenIn::Own => Some(None),
    }
}

/// Where an open was asked, as the log names it.
fn where_words(at: &OpenIn) -> String {
    match at {
        OpenIn::Beside { pane } => format!("beside {pane}"),
        OpenIn::Session { name } => format!("in session {name}"),
        OpenIn::Own => "in own".into(),
    }
}

struct Running {
    state: AgentState,
    schedule: Vec<(Seconds, AgentState)>,
    carried: bool,
    command: AgentCommand,
    at: OpenIn,
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
            at: OpenIn::Own,
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
                opens_as: OpensAs::Running,
                tabs: vec![],
                workspaces: BTreeSet::new(),
                persons: BTreeMap::new(),
                shows_fail: false,
                shown: vec![],
                sessions: BTreeMap::new(),
                agent_sessions: BTreeMap::new(),
                notices: vec![],
                attaches_terminal: None,
                attaches_app: None,
                setup: vec![],
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

    /// Every open from now holds its start at a question the program
    /// asks before it starts, the agent left at it showing `screen` and
    /// read asking, as a terminal reads a program held at its question.
    pub fn hold_opens(&self, screen: &str) {
        lock(&self.state).opens_as = OpensAs::Held(screen.into());
    }

    /// Every open from now finds its program did not come up within the
    /// terminal's bound, the terminal showing `screen`.
    pub fn opens_never_come_up(&self, screen: &str) {
        lock(&self.state).opens_as = OpensAs::NotComeUp(screen.into());
    }

    /// A person opens a pane of their own in the workspace of `place`'s
    /// repository and base, in the terminal's own session.
    pub fn person_pane(&self, place: &PlaceRef, pane: &str) {
        self.person_pane_in(None, place, pane);
    }

    /// A person opens a pane of their own in the workspace of `place`'s
    /// repository and base in the session `session`, none for the
    /// terminal's own.
    pub fn person_pane_in(&self, session: Option<&str>, place: &PlaceRef, pane: &str) {
        lock(&self.state)
            .persons
            .insert(Workspace::of(place, session), pane.into());
    }

    /// The person's pane in `place`'s workspace is closed.
    pub fn close_person_pane(&self, place: &PlaceRef) {
        lock(&self.state)
            .persons
            .remove(&Workspace::of(place, None));
    }

    /// The tabs standing, by place location, oldest first, and the
    /// workspaces, each named with its session where that is not the
    /// terminal's own.
    pub fn made(&self) -> (Vec<String>, Vec<String>) {
        let state = lock(&self.state);
        (
            state.tabs.iter().map(|tab| tab.location.clone()).collect(),
            state.workspaces.iter().map(Workspace::named).collect(),
        )
    }

    /// The sessions `place`'s tabs stand in, oldest first, none for the
    /// terminal's own.
    pub fn tabs_of(&self, place: &PlaceRef) -> Vec<Option<String>> {
        lock(&self.state)
            .tabs
            .iter()
            .filter(|tab| tab.location == place.location)
            .map(|tab| tab.workspace.session.clone())
            .collect()
    }

    /// Every show fails, or none does.
    pub fn shows_fail(&self, fail: bool) {
        lock(&self.state).shows_fail = fail;
    }

    /// The clues each show was given, by place location, in order.
    pub fn shown(&self) -> Vec<(String, Clues)> {
        lock(&self.state).shown.clone()
    }

    /// A recipient's session named `name`, reading `state`, as one the
    /// terminal holds; standing already, its state is set.
    pub fn add_session(&self, name: &str, state: AgentState) {
        lock(&self.state).sessions.insert(name.into(), state);
    }

    /// The recipient's session named `name` is gone, as a session that
    /// exited or restarted under another name.
    pub fn remove_session(&self, name: &str) {
        lock(&self.state).sessions.remove(name);
    }

    /// The recipient's session named `name` reads `state`.
    pub fn set_session_state(&self, name: &str, state: AgentState) {
        if let Some(standing) = lock(&self.state).sessions.get_mut(name) {
            *standing = state;
        }
    }

    /// Every text sent to the session named `name`, in order.
    pub fn notices(&self, name: &str) -> Vec<String> {
        lock(&self.state)
            .notices
            .iter()
            .filter(|(to, _)| to == name)
            .map(|(_, text)| text.clone())
            .collect()
    }

    /// Every text sent to any session by its name, with the name, in
    /// order.
    pub fn every_notice(&self) -> Vec<(String, String)> {
        lock(&self.state).notices.clone()
    }

    /// Every attach for `view` from now answers `answer`, a refusal
    /// among them, or the fake's own answer for the view where none
    /// (187).
    pub fn attaches_as(&self, view: View, answer: Option<Attached>) {
        let mut state = lock(&self.state);
        match view {
            View::Terminal => state.attaches_terminal = answer,
            View::App => state.attaches_app = answer,
        }
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

    /// What the terminal's configuration lacks for flume's plugin, each
    /// in its words, until set otherwise (122).
    pub fn set_setup(&self, setup: &[&str]) {
        lock(&self.state).setup = setup.iter().map(|words| words.to_string()).collect();
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
        self.opened_at(agent).and_then(|at| match at {
            OpenIn::Beside { pane } => Some(pane),
            _ => None,
        })
    }

    /// Where the agent was asked to open (107).
    pub fn opened_at(&self, agent: &Identity) -> Option<OpenIn> {
        lock(&self.state)
            .agents
            .get(&agent.to_string())
            .map(|running| running.at.clone())
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
        at: &OpenIn,
        session: Option<&str>,
    ) -> Opened {
        self.log.push(format!("open {identity} {}", place.location));
        self.log.push(format!("at {identity} {}", where_words(at)));
        let key = identity.to_string();
        if self.read(&key) != AgentState::Stopped {
            return Opened::Refused {
                reason: format!("an agent is live under {key}"),
            };
        }
        if let Some(name) = session {
            let holder = lock(&self.state).agent_sessions.get(name).cloned();
            if let Some(holder) = holder.filter(|holder| self.read(holder) != AgentState::Stopped) {
                return Opened::Refused {
                    reason: format!("the live agent {holder} carries the session name {name}"),
                };
            }
        }
        let mut state = lock(&self.state);
        if let Some(reason) = &state.open_refusal {
            return Opened::Refused {
                reason: reason.clone(),
            };
        }
        if let Some(in_session) = session_of(at) {
            let made = Workspace::of(place, in_session);
            state
                .tabs
                .retain(|tab| tab.location != place.location || tab.workspace != made);
            state.tabs.push(Tab {
                location: place.location.clone(),
                name: identity.place.clone(),
                workspace: made.clone(),
            });
            state.workspaces.insert(made);
        }
        let carried = !state.sends;
        let (comes_up, answer) = match state.opens_as.clone() {
            OpensAs::Running => (
                state.comes_up,
                Opened::Opened {
                    carried,
                    printed: None,
                },
            ),
            OpensAs::Held(screen) => (AgentState::Asking, Opened::Held { screen }),
            OpensAs::NotComeUp(screen) => return Opened::NotComeUp { screen },
        };
        let mut running = Running::new(comes_up, command.clone());
        running.carried = carried;
        running.at = at.clone();
        running.folder = Some(folder.to_string());
        if let Opened::Held { screen } = &answer {
            running.screen = Some(screen.clone());
        }
        if let Some(name) = session {
            state.agent_sessions.insert(name.into(), key.clone());
        }
        state.agents.insert(key, running);
        answer
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

    fn send_to(&self, session: &str, text: &str) -> ToSession {
        self.log.push(format!("send_to {session}"));
        let agent = lock(&self.state).agent_sessions.get(session).cloned();
        if let Some(key) = agent.filter(|key| self.read(key) != AgentState::Stopped) {
            let mut state = lock(&self.state);
            if !state.sends {
                return ToSession::Refused {
                    reason: "this terminal cannot send to a running agent".into(),
                };
            }
            if let Some(running) = state.agents.get_mut(&key) {
                if !running.typed.is_empty() {
                    running.typed.clear();
                    running.cleared += 1;
                }
            }
            state.sent.push((key, text.into()));
            state.notices.push((session.into(), text.into()));
            return ToSession::Sent;
        }
        let mut state = lock(&self.state);
        if !state.sessions.contains_key(session) {
            return ToSession::NotFound;
        }
        if !state.sends {
            return ToSession::Refused {
                reason: "this terminal cannot send to a session".into(),
            };
        }
        state.notices.push((session.into(), text.into()));
        ToSession::Sent
    }

    fn session_state(&self, session: &str) -> Option<AgentState> {
        self.log.push(format!("session_state {session}"));
        let agent = lock(&self.state).agent_sessions.get(session).cloned();
        if let Some(read) = agent
            .map(|key| self.read(&key))
            .filter(|read| *read != AgentState::Stopped)
        {
            return Some(read);
        }
        lock(&self.state).sessions.get(session).copied()
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

    fn tidy(&self, place: &PlaceRef) -> TerminalTidy {
        self.log.push(format!("tidy {}", place.location));
        let mut state = lock(&self.state);
        let mut tidy = TerminalTidy::default();
        let label = workspace(place);
        let (gone, kept): (Vec<Tab>, Vec<Tab>) = std::mem::take(&mut state.tabs)
            .into_iter()
            .partition(|tab| tab.location == place.location);
        state.tabs = kept;
        for tab in gone {
            tidy.closed.push(format!(
                "the tab {}",
                in_session(&tab.name, tab.workspace.session.as_deref())
            ));
        }
        let bases: Vec<Workspace> = state
            .workspaces
            .iter()
            .filter(|made| made.label == label)
            .filter(|made| !state.tabs.iter().any(|tab| tab.workspace == **made))
            .cloned()
            .collect();
        for made in bases {
            let named = made.named();
            match state.persons.get(&made) {
                Some(pane) => tidy
                    .kept
                    .push(format!("the workspace {named}, held by the pane {pane}")),
                None => {
                    state.workspaces.remove(&made);
                    tidy.closed.push(format!("the workspace {named}"));
                }
            }
        }
        tidy
    }

    fn kept(&self) -> Vec<String> {
        let state = lock(&self.state);
        state
            .workspaces
            .iter()
            .filter(|made| !state.tabs.iter().any(|tab| tab.workspace == **made))
            .filter_map(|made| {
                state
                    .persons
                    .get(made)
                    .map(|pane| format!("the workspace {}, held by the pane {pane}", made.named()))
            })
            .collect()
    }

    fn show(&self, place: &PlaceRef, clues: &Clues) -> Shown {
        self.log.push(format!("show {}", place.location));
        let mut state = lock(&self.state);
        if state.shows_fail {
            return Shown::Failed {
                reason: "the terminal refused the clues".into(),
            };
        }
        state.shown.push((place.location.clone(), clues.clone()));
        Shown::Shown
    }

    /// The place's tab in the session its live agent opened in, else its
    /// newest; the agent while it runs; the workspace of its repository
    /// and base in that tab's session, or the terminal's own where it has
    /// no tab, while it stands; and each workspace of its repository and
    /// base in any session; each by the name the fake gives it, with its
    /// session where that is not the terminal's own; nothing logged,
    /// since it asks nothing (107, 139).
    fn shown_at(&self, place: &PlaceRef, agent: Option<&Identity>) -> ShownAt {
        let state = lock(&self.state);
        let running = agent.and_then(|agent| state.agents.get(&agent.to_string()));
        let tabs: Vec<&Tab> = state
            .tabs
            .iter()
            .filter(|tab| tab.location == place.location)
            .collect();
        let tab = running
            .and_then(|running| session_of(&running.at))
            .and_then(|session| {
                tabs.iter()
                    .find(|tab| tab.workspace.session.as_deref() == session)
            })
            .or(tabs.last());
        let base = tab.map_or_else(|| Workspace::of(place, None), |tab| tab.workspace.clone());
        let label = workspace(place);
        ShownAt {
            place: tab.map(|tab| {
                format!(
                    "tab {}",
                    in_session(&tab.name, tab.workspace.session.as_deref())
                )
            }),
            agent: agent
                .filter(|_| running.is_some())
                .map(|agent| format!("pane {agent}")),
            base: state
                .workspaces
                .contains(&base)
                .then(|| format!("workspace {}", base.named())),
            bases: state
                .workspaces
                .iter()
                .filter(|made| made.label == label)
                .map(|made| format!("workspace {}", made.named()))
                .collect(),
        }
    }

    fn watch(&self) -> Watch {
        lock(&self.state).watch
    }

    /// What was scripted, logged so a reading that asks none is seen to.
    fn setup(&self) -> Vec<String> {
        self.log.push("setup");
        lock(&self.state).setup.clone()
    }

    fn session(&self) -> Option<String> {
        self.log.push("session");
        lock(&self.state).session.clone()
    }

    fn attach(&self, session: Option<&str>, view: View) -> Attached {
        let part = session.unwrap_or("own");
        self.log.push(format!("attach {part} for {}", view.name()));
        let state = lock(&self.state);
        let scripted = match view {
            View::Terminal => &state.attaches_terminal,
            View::App => &state.attaches_app,
        };
        scripted.clone().unwrap_or_else(|| match view {
            View::Terminal => Attached::Command {
                command: ViewCommand {
                    program: "fake-terminal".into(),
                    arguments: vec!["attach".into(), part.into()],
                    environment: Default::default(),
                },
            },
            View::App => Attached::Shown,
        })
    }

    /// The part named does not run, its own named `own` as attach names
    /// it (201).
    fn runs(&self, session: Option<&str>) -> SessionRuns {
        SessionRuns::NotRunning {
            name: session.unwrap_or("own").into(),
        }
    }
}
