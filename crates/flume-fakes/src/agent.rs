use crate::{lock, FakeTerminal, Log};
use flume_seams::{
    Agent, AgentCommand, FolderTrust, Guarded, Identity, Mode, ModeBegan, ModeEntry, Offered,
    PlaceRef, Reading, Received, Secret, Settings, Spent, Unsettled,
};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// The agent seam: each agent's own record by identity. An agent says
/// it started once the fake terminal has opened it, unless set otherwise,
/// and receives a prompt once the fake terminal has sent it, or opened it
/// carried, as many times as it is set to need, and says nothing of its
/// start where it is set to have begun without its settings; every other answer is
/// set, unknowns among them. A record kept in a place's folder is read
/// there alone: a reading handed another folder is unknown (128). It maps
/// no mode until its map is scripted, and reads none back until set to;
/// its command passes the value mapped as `--mode <value>` (169).
pub struct FakeAgent {
    terminal: Arc<FakeTerminal>,
    state: Mutex<AgentModel>,
    pub log: Log,
}

#[derive(Default)]
struct AgentModel {
    agents: BTreeMap<String, Record>,
    no_send_back: bool,
    no_on_quiet: bool,
    no_guard: bool,
    withheld: Vec<String>,
    /// The value each of flume's four maps to, as scripted (169).
    modes: BTreeMap<Mode, String>,
    /// The own values the program names, each passed as written (169).
    own_modes: Vec<String>,
    mode_read_back: bool,
}

#[derive(Clone)]
struct Record {
    asking: Reading,
    own_work: Reading,
    record: Option<String>,
    receives_at: Option<usize>,
    received_unknown: bool,
    refused: Vec<String>,
    ended: Reading,
    folder: Option<String>,
    started: Option<Reading>,
    last_words: Option<String>,
    spent: Spent,
    started_without: Option<Unsettled>,
    /// The mode the agent's word says it began in, as scripted (161).
    began_in: Option<String>,
}

impl Default for Record {
    fn default() -> Self {
        Record {
            asking: Reading::No,
            own_work: Reading::No,
            record: None,
            receives_at: Some(1),
            received_unknown: false,
            refused: vec![],
            ended: Reading::No,
            folder: None,
            started: None,
            last_words: None,
            spent: Spent::No,
            started_without: None,
            began_in: None,
        }
    }
}

impl FakeAgent {
    pub fn new(terminal: Arc<FakeTerminal>) -> FakeAgent {
        FakeAgent {
            terminal,
            state: Mutex::default(),
            log: Log::default(),
        }
    }

    fn with(&self, agent: &Identity, change: impl FnOnce(&mut Record)) {
        let mut state = lock(&self.state);
        change(state.agents.entry(agent.to_string()).or_default());
    }

    pub fn set_asking(&self, agent: &Identity, reading: Reading) {
        self.with(agent, |record| record.asking = reading);
    }

    pub fn set_own_work(&self, agent: &Identity, reading: Reading) {
        self.with(agent, |record| record.own_work = reading);
    }

    /// The record the agent keeps, as one a terminal resumed may differ.
    pub fn set_record(&self, agent: &Identity, record: &str) {
        let record = record.to_string();
        self.with(agent, |model| model.record = Some(record));
    }

    /// The agent receives a prompt at its `sending`th sending, or never.
    pub fn receives_at(&self, agent: &Identity, sending: Option<usize>) {
        self.with(agent, |record| record.receives_at = sending);
    }

    pub fn received_unknown(&self, agent: &Identity) {
        self.with(agent, |record| record.received_unknown = true);
    }

    /// The guard refused an act the agent tried.
    pub fn refuse_act(&self, agent: &Identity, act: &str) {
        let act = act.to_string();
        self.with(agent, |record| record.refused.push(act));
    }

    /// Whether the agent's record shows its session ended (134).
    pub fn set_ended(&self, agent: &Identity, reading: Reading) {
        self.with(agent, |record| record.ended = reading);
    }

    /// Whether the agent says it started, in place of its word once opened
    /// (135).
    pub fn set_started(&self, agent: &Identity, reading: Reading) {
        self.with(agent, |record| record.started = Some(reading));
    }

    /// The agent's last words as its record holds them (43).
    pub fn set_last_words(&self, agent: &Identity, words: &str) {
        let words = words.to_string();
        self.with(agent, |record| record.last_words = Some(words));
    }

    /// Whether the agent's record shows its account reached a usage
    /// limit, and the reset it names (151): a usage limit scripted with
    /// words a test writes.
    pub fn set_spent(&self, agent: &Identity, spent: Spent) {
        self.with(agent, |record| record.spent = spent);
    }

    /// The agent began without the settings its start recorded, as its
    /// own word shows, which then says it started nothing (135, 161).
    pub fn set_started_without(&self, agent: &Identity, unsettled: Unsettled) {
        self.with(agent, |record| record.started_without = Some(unsettled));
    }

    /// The agent's word that it started names `mode` as the mode it began
    /// in, which is read where the fake reads the mode back and its
    /// command passed another (161, 162).
    pub fn set_began_in(&self, agent: &Identity, mode: &str) {
        let mode = mode.to_string();
        self.with(agent, |record| record.began_in = Some(mode));
    }

    /// The agent's record is kept in `folder`, as the agent's program
    /// records its session in the place's own folder (128).
    pub fn keep_record_in(&self, agent: &Identity, folder: &str) {
        let folder = folder.to_string();
        self.with(agent, |record| record.folder = Some(folder));
    }

    pub fn no_send_back(&self) {
        lock(&self.state).no_send_back = true;
    }

    pub fn no_on_quiet(&self) {
        lock(&self.state).no_on_quiet = true;
    }

    pub fn no_guard(&self) {
        lock(&self.state).no_guard = true;
    }

    /// The program's setting `value` means `mode`, which the fake maps to
    /// it (169).
    pub fn map_mode(&self, mode: Mode, value: &str) {
        lock(&self.state).modes.insert(mode, value.to_string());
    }

    /// The program names `value` among its own modes, passed as written
    /// (169).
    pub fn own_mode(&self, value: &str) {
        lock(&self.state).own_modes.push(value.to_string());
    }

    /// The program's word that it started names the mode it began in
    /// (161, 169).
    pub fn read_mode_back(&self) {
        lock(&self.state).mode_read_back = true;
    }

    /// The program keeps `names` from every process it starts (161).
    pub fn withhold(&self, names: &[&str]) {
        lock(&self.state).withheld = names.iter().map(|name| name.to_string()).collect();
    }

    fn record(&self, agent: &Identity) -> Record {
        lock(&self.state)
            .agents
            .get(&agent.to_string())
            .cloned()
            .unwrap_or_default()
    }

    /// How the agent began otherwise than its start, as scripted: the
    /// variables set, and its mode where it began in another (161, 162).
    fn unsettled(&self, agent: &Identity, record: &Record) -> Option<Unsettled> {
        match (
            record.started_without.clone(),
            self.began_otherwise(agent, record),
        ) {
            (scripted, None) => scripted,
            (scripted, Some(mode)) => Some(Unsettled {
                mode: Some(mode),
                ..scripted.unwrap_or_default()
            }),
        }
    }

    /// The mode the agent began in and the one its command passed, where
    /// the fake reads the mode back and they differ; none where its
    /// command passed none, since the start then compares none (161, 162).
    fn began_otherwise(&self, agent: &Identity, record: &Record) -> Option<ModeBegan> {
        if !lock(&self.state).mode_read_back {
            return None;
        }
        let began = record.began_in.clone()?;
        let (command, _) = self.terminal.opened(agent)?;
        let asked = command
            .args
            .iter()
            .skip_while(|arg| *arg != "--mode")
            .nth(1)?
            .clone();
        (asked != began).then_some(ModeBegan {
            asked,
            began: Some(began),
        })
    }

    /// The agent's record as read in `folder`: none where it is kept in
    /// another.
    fn read_in(&self, operation: &str, folder: &str, agent: &Identity) -> Option<Record> {
        self.log.push(format!("{operation} {agent} in {folder}"));
        let record = self.record(agent);
        match &record.folder {
            Some(kept) if kept != folder => None,
            _ => Some(record),
        }
    }
}

impl Agent for FakeAgent {
    fn command(
        &self,
        settings: &Settings,
        mode: Option<&str>,
        env: &BTreeMap<String, String>,
        place: &PlaceRef,
        identity: &Identity,
        secret: &Secret,
        prompt: &str,
    ) -> AgentCommand {
        self.log.push(format!("command {identity}"));
        let mut args = vec![];
        if let Some(model) = &settings.model {
            args.extend(["--model".to_string(), model.clone()]);
        }
        if let Some(mode) = mode {
            args.extend(["--mode".to_string(), mode.to_string()]);
        }
        if let Some(instructions) = &settings.instructions {
            args.extend(["--instructions".to_string(), instructions.clone()]);
        }
        let mut env = env.clone();
        env.insert("AGENT_IDENTITY".into(), identity.to_string());
        env.insert("AGENT_SECRET".into(), secret.0.clone());
        env.insert("AGENT_PLACE".into(), place.location.clone());
        env.insert("AGENT_STAGE".into(), identity.stage.clone());
        AgentCommand {
            program: settings.program.clone().unwrap_or_else(|| "agent".into()),
            args,
            env,
            prompt_args: vec![prompt.to_string()],
            folder_trust: Some(FolderTrust {
                question: "Yes, I trust this folder".into(),
                focus: ">".into(),
                next: "down".into(),
                confirm: "enter".into(),
            }),
            kind: Some("agent".into()),
        }
    }

    fn started(&self, folder: &str, agent: &Identity) -> Reading {
        let Some(model) = self.read_in("started", folder, agent) else {
            return Reading::Unknown;
        };
        if model.started.is_none() && self.unsettled(agent, &model).is_some() {
            return Reading::No;
        }
        model
            .started
            .unwrap_or(if self.terminal.opened(agent).is_some() {
                Reading::Yes
            } else {
                Reading::No
            })
    }

    fn received(&self, folder: &str, agent: &Identity, text: &str) -> Received {
        let Some(model) = self.read_in("received", folder, agent) else {
            return Received::Unknown;
        };
        let Some((command, carried)) = self.terminal.opened(agent) else {
            return Received::Unknown;
        };
        if model.received_unknown {
            return Received::Unknown;
        }
        let record = model
            .record
            .clone()
            .unwrap_or_else(|| format!("record:{agent}"));
        let mut sendings = self
            .terminal
            .sent_to(agent)
            .iter()
            .filter(|sent| *sent == text)
            .count();
        if carried && command.prompt_args.iter().any(|arg| arg == text) {
            sendings += 1;
        }
        if model.receives_at.is_some_and(|needed| sendings >= needed) {
            Received::Received { record }
        } else {
            Received::NotReceived { record }
        }
    }

    fn asking(&self, folder: &str, agent: &Identity) -> Reading {
        self.read_in("asking", folder, agent)
            .map_or(Reading::Unknown, |record| record.asking)
    }

    fn own_work(&self, folder: &str, agent: &Identity) -> Reading {
        self.read_in("own_work", folder, agent)
            .map_or(Reading::Unknown, |record| record.own_work)
    }

    fn ended(&self, folder: &str, agent: &Identity) -> Reading {
        self.read_in("ended", folder, agent)
            .map_or(Reading::Unknown, |record| record.ended)
    }

    fn last_words(&self, folder: &str, agent: &Identity) -> Option<String> {
        self.read_in("last_words", folder, agent)
            .and_then(|record| record.last_words)
    }

    fn spent(&self, folder: &str, agent: &Identity) -> Spent {
        self.read_in("spent", folder, agent)
            .map_or(Spent::Unknown, |record| record.spent)
    }

    fn started_without(&self, folder: &str, agent: &Identity) -> Option<Unsettled> {
        self.read_in("started_without", folder, agent)
            .and_then(|record| self.unsettled(agent, &record))
    }

    /// The value scripted for `entry`, or words naming it, each mapping
    /// logged (169).
    fn mode(&self, entry: &ModeEntry) -> Result<String, String> {
        self.log.push(format!("mode {entry}"));
        let state = lock(&self.state);
        match entry {
            ModeEntry::Flume(mode) => state
                .modes
                .get(mode)
                .cloned()
                .ok_or_else(|| format!("the fake agent has no setting that means `{mode}`")),
            ModeEntry::Own(value) if state.own_modes.contains(value) => Ok(value.clone()),
            ModeEntry::Own(value) => Err(format!(
                "`{value}` is none of the fake agent's own modes, which are {}",
                if state.own_modes.is_empty() {
                    "none".to_string()
                } else {
                    state.own_modes.join(", ")
                }
            )),
        }
    }

    fn mode_read_back(&self) -> Offered {
        if lock(&self.state).mode_read_back {
            Offered::Offered
        } else {
            Offered::NotOffered
        }
    }

    fn withheld(&self) -> Vec<String> {
        lock(&self.state).withheld.clone()
    }

    fn stop_short(&self, _agent: &Identity) -> String {
        "to stop short needing an answer, run `flume needs \"<what you need>\"`; having found work the stage cannot deliver past, run `flume needs --found \"<the work you found>\"`".into()
    }

    fn send_back(&self) -> Offered {
        if lock(&self.state).no_send_back {
            Offered::NotOffered
        } else {
            Offered::Offered
        }
    }

    fn guard(&self, folder: &str, agent: &Identity) -> Guarded {
        let record = self.read_in("guard", folder, agent);
        if lock(&self.state).no_guard {
            return Guarded::NotOffered;
        }
        Guarded::Refused {
            acts: record.map(|record| record.refused).unwrap_or_default(),
        }
    }

    fn on_quiet(&self) -> Offered {
        if lock(&self.state).no_on_quiet {
            Offered::NotOffered
        } else {
            Offered::Offered
        }
    }
}
