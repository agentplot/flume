use crate::{lock, FakeTerminal, Log};
use flume_seams::{
    no_effort, Agent, AgentCommand, Carry, FolderTrust, Guarded, Identity, Live, LiveProcess, Mode,
    ModeBegan, ModeEntry, Offered, PlaceRef, Reading, Received, Reserved, Secret, Settings, Spent,
    Unsettled, NO_OWN, READS_NO_SESSION,
};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// The agent seam: each agent's own record by identity. An agent says
/// it started once the fake terminal it opened in has opened it, unless
/// set otherwise, and receives a prompt once that terminal has sent it,
/// or opened it carried, as many times as it is set to need, and says nothing of its
/// start where it is set to have begun without its settings; every other answer is
/// set, unknowns among them. A record kept in a place's folder is read
/// there alone: a reading handed another folder is unknown (128). It maps
/// no mode until its map is scripted, and reads none back until set to,
/// in its word that it started or at its start's first prompt; its
/// command passes the value mapped as `--mode <value>` (169). It carries
/// no conversation until its carry is scripted, by each account's
/// [`FakeAgent::HOME`]; its command passes the session a start resumes as
/// `--resume <session>`, and a start's word that it started names the
/// session it resumes unless scripted otherwise (179). Its live answers,
/// by the session a start's word names, the processes or the words
/// scripted for that session, and none otherwise, unless set to read no
/// running session (207). Its agents open in the terminal it is made
/// over, or in any other it is told of (190). It reserves no key of its
/// settings and takes no own settings until scripted to; once it takes
/// them, its command lists them as one JSON object in a file under the
/// place's own folder, [`FakeAgent::own_file`], and names it by `--own
/// <file>`, and every command given own settings logs their keys (195,
/// 196). It takes no effort until the words it takes are scripted,
/// refusing every word in the seam's default words, and names no
/// variable read over the effort until scripted to; its command passes
/// the effort its settings give as `--effort <word>`, and each effort
/// asked of it is logged (197).
pub struct FakeAgent {
    terminal: Arc<FakeTerminal>,
    /// The other terminals its agents may open in, a stream's (190).
    others: Mutex<Vec<Arc<FakeTerminal>>>,
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
    /// The mode is named at the start's first prompt, never in the word
    /// that it started (161).
    mode_at_prompt: bool,
    /// The answer of carry to each account, by its [`FakeAgent::HOME`]
    /// (179).
    carries: BTreeMap<String, Carry>,
    /// Each start whose command resumes a session ends before its word
    /// that it started (S79).
    carried_starts_end: bool,
    /// What live answers for each session, as scripted (207).
    lives: BTreeMap<String, Live>,
    /// Live answers the seam's default, as a profile that reads no
    /// running session (171, 207).
    live_not_read: bool,
    /// The program takes its own settings, by a file its command names
    /// (195, 196).
    takes_own: bool,
    /// The keys of its settings the program reserves, each with its
    /// owner, a nested key by its path (195, 196).
    reserved: Vec<Reserved>,
    /// The file its command lists its own settings in where scripted
    /// apart from its own under the place's folder (196).
    own_at: Option<String>,
    /// The words the program takes as its effort, as scripted (197).
    efforts: Vec<String>,
    /// The variable the program reads over its effort, as scripted (197).
    effort_variable: Option<&'static str>,
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
    /// The session the agent's word that it started names, as scripted
    /// (161, 179).
    session: Option<String>,
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
            session: None,
        }
    }
}

impl FakeAgent {
    /// The variable of an account's environment by which the fake knows
    /// the account, as a program knows its configuration home (179).
    pub const HOME: &'static str = "AGENT_HOME";

    /// What [`FakeAgent::carry_to`] scripts for an environment that names
    /// no [`FakeAgent::HOME`], the login environment's under no pool, as a
    /// program reads its own configuration home there (49, 179).
    pub const LOGIN: &'static str = "no home";

    pub fn new(terminal: Arc<FakeTerminal>) -> FakeAgent {
        FakeAgent {
            terminal,
            others: Mutex::default(),
            state: Mutex::default(),
            log: Log::default(),
        }
    }

    /// Its agents may open in `terminal` too, each read in the terminal
    /// that opened it (190).
    pub fn also_on(&self, terminal: Arc<FakeTerminal>) {
        lock(&self.others).push(terminal);
    }

    /// The terminal `agent` opened in: the one it is made over, else the
    /// first other that opened it, else the one it is made over.
    fn on(&self, agent: &Identity) -> Arc<FakeTerminal> {
        if self.terminal.opened(agent).is_some() {
            return self.terminal.clone();
        }
        lock(&self.others)
            .iter()
            .find(|terminal| terminal.opened(agent).is_some())
            .cloned()
            .unwrap_or_else(|| self.terminal.clone())
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

    /// The agent's word that it started names `session`, which is
    /// compared with the session its command resumes, where it resumes
    /// one (161, 179).
    pub fn set_session(&self, agent: &Identity, session: &str) {
        let session = session.to_string();
        self.with(agent, |record| record.session = Some(session));
    }

    /// The program carries a stopped or lost start's conversation to the
    /// account whose [`FakeAgent::HOME`] is `home`, or to the login
    /// environment's where `home` is [`FakeAgent::LOGIN`], as `carry`
    /// says: yes with the session it resumes, or no with the words (49,
    /// 179).
    pub fn carry_to(&self, home: &str, carry: Carry) {
        lock(&self.state).carries.insert(home.to_string(), carry);
    }

    /// The terminal brought `agent` back bare, as a terminal's restore
    /// does: its pane holds an agent again, waiting, whose word that it
    /// started reads no, logged `bring back bare <agent>` (77, 106). The
    /// test scripts its state after, by the terminal's `set_state`.
    pub fn bring_back_bare(&self, agent: &Identity) {
        self.log.push(format!("bring back bare {agent}"));
        self.on(agent).resume(agent);
        self.set_started(agent, Reading::No);
    }

    /// Processes `processes` run the conversation `session`, logged (207).
    pub fn live_runs(&self, session: &str, processes: &[LiveProcess]) {
        self.log
            .push(format!("live_runs {session}: {}", pids(processes)));
        lock(&self.state)
            .lives
            .insert(session.to_string(), Live::Runs(processes.to_vec()));
    }

    /// Whether a process runs the conversation `session` cannot be read,
    /// for `what`, logged (207).
    pub fn live_unknown(&self, session: &str, what: &str) {
        self.log.push(format!("live_unknown {session}: {what}"));
        lock(&self.state).lives.insert(
            session.to_string(),
            Live::Unknown {
                what: what.to_string(),
            },
        );
    }

    /// No process runs the conversation `session` any longer, logged
    /// (207).
    pub fn live_ended(&self, session: &str) {
        self.log.push(format!("live_ended {session}"));
        lock(&self.state).lives.remove(session);
    }

    /// Live answers the seam's default, as a profile that reads no
    /// running session (171, 207).
    pub fn live_not_read(&self) {
        lock(&self.state).live_not_read = true;
    }

    /// Each start whose command resumes a session finds no such
    /// conversation and ends before its word that it started, as a program
    /// does whose account cannot reach it (146, S79).
    pub fn carried_starts_end(&self) {
        lock(&self.state).carried_starts_end = true;
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

    /// The program names the mode it began in only at its start's first
    /// prompt: its word that it started names none, and where it began in
    /// another, its word names the mode once a prompt reached it, which it
    /// does not receive (161, 169).
    pub fn read_mode_at_prompt(&self) {
        let mut state = lock(&self.state);
        state.mode_read_back = true;
        state.mode_at_prompt = true;
    }

    /// The program takes its own settings, by a file under the place's
    /// own folder its command lists and names by `--own <file>` (195,
    /// 196).
    pub fn take_own(&self) {
        lock(&self.state).takes_own = true;
    }

    /// The program's setting `key`, a nested key by its path as
    /// `permissions.default`, is `owner`'s, so the fake reserves it (195,
    /// 196).
    pub fn reserve(&self, key: &str, owner: &str) {
        lock(&self.state).reserved.push(Reserved {
            key: key.to_string(),
            owner: owner.to_string(),
        });
    }

    /// The program's own settings go to the file at `path`, wherever it
    /// lies, as a profile that lists a file outside the place's folder
    /// would (196).
    pub fn own_file_at(&self, path: &str) {
        lock(&self.state).own_at = Some(path.to_string());
    }

    /// The file under the place's own `folder` that a command for `agent`
    /// lists its own settings in (196).
    pub fn own_file(folder: &str, agent: &Identity) -> String {
        format!(
            "{folder}/agent/{}/own.json",
            agent.to_string().replace('/', "%")
        )
    }

    /// The program takes `words` as its effort, each passed as written,
    /// and refuses any other naming it and them (197).
    pub fn take_efforts(&self, words: &[&str]) {
        lock(&self.state).efforts = words.iter().map(|word| word.to_string()).collect();
    }

    /// The program reads the variable `name` of its environment over the
    /// effort its command passes (197).
    pub fn read_effort_over(&self, name: &'static str) {
        lock(&self.state).effort_variable = Some(name);
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
    /// variables set, another session than its command resumes, and its
    /// mode where it began in another, once a prompt reached it where it
    /// names its mode only then (161, 162, 179).
    fn unsettled(&self, agent: &Identity, record: &Record) -> Option<Unsettled> {
        let at_prompt = lock(&self.state).mode_at_prompt;
        let mode = self
            .began_otherwise(agent, record)
            .filter(|_| !at_prompt || self.prompted(agent));
        let mut unsettled = match (record.started_without.clone(), mode) {
            (scripted, None) => scripted,
            (scripted, Some(mode)) => Some(Unsettled {
                mode: Some(mode),
                ..scripted.unwrap_or_default()
            }),
        };
        if self.in_another_session(agent, record) {
            unsettled.get_or_insert_with(Unsettled::default).session = true;
        }
        unsettled
    }

    /// Whether its word that it started shows it began otherwise: the
    /// variables set, another session than its command resumes, or its
    /// mode where that word names it (161, 179).
    fn unsettled_at_start(&self, agent: &Identity, record: &Record) -> bool {
        let at_prompt = lock(&self.state).mode_at_prompt;
        record.started_without.is_some()
            || self.in_another_session(agent, record)
            || (!at_prompt && self.began_otherwise(agent, record).is_some())
    }

    /// The session the agent's command resumes, none where it resumes
    /// none (179).
    fn resumed(&self, agent: &Identity) -> Option<String> {
        let (command, _) = self.on(agent).opened(agent)?;
        command
            .args
            .iter()
            .skip_while(|arg| *arg != "--resume")
            .nth(1)
            .cloned()
    }

    /// Whether its word that it started names another session than the
    /// one its command resumes; none is compared where it resumes none
    /// (161, 179).
    fn in_another_session(&self, agent: &Identity, record: &Record) -> bool {
        match (self.resumed(agent), &record.session) {
            (Some(resumed), Some(session)) => resumed != *session,
            _ => false,
        }
    }

    /// Whether the agent resumes a session where carried starts are
    /// scripted to end before their word that they started (S79).
    fn ends_unfound(&self, agent: &Identity) -> bool {
        let end = lock(&self.state).carried_starts_end;
        end && self.resumed(agent).is_some()
    }

    /// Whether its start's first prompt was held back, its program naming
    /// another mode than its command passed only then (161).
    fn held_back(&self, agent: &Identity, record: &Record) -> bool {
        let at_prompt = lock(&self.state).mode_at_prompt;
        at_prompt && self.began_otherwise(agent, record).is_some()
    }

    /// Whether a prompt reached the agent: sent by the terminal, or
    /// carried by the command it was opened with.
    fn prompted(&self, agent: &Identity) -> bool {
        !self.on(agent).sent_to(agent).is_empty()
            || self
                .on(agent)
                .opened(agent)
                .is_some_and(|(command, carried)| carried && !command.prompt_args.is_empty())
    }

    /// The mode the agent began in and the one its command passed, where
    /// the fake reads the mode back and they differ; none where its
    /// command passed none, since the start then compares none (161, 162).
    fn began_otherwise(&self, agent: &Identity, record: &Record) -> Option<ModeBegan> {
        if !lock(&self.state).mode_read_back {
            return None;
        }
        let began = record.began_in.clone()?;
        let (command, _) = self.on(agent).opened(agent)?;
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
    /// another. A start scripted to end before its word has ended by the
    /// first reading, its terminal reading it stopped (S79).
    fn read_in(&self, operation: &str, folder: &str, agent: &Identity) -> Option<Record> {
        self.log.push(format!("{operation} {agent} in {folder}"));
        if self.ends_unfound(agent) {
            self.on(agent).kill(agent);
        }
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
        folder: &str,
        settings: &Settings,
        mode: Option<&str>,
        own: &BTreeMap<String, serde_json::Value>,
        resume: Option<&str>,
        env: &BTreeMap<String, String>,
        place: &PlaceRef,
        identity: &Identity,
        secret: &Secret,
        prompt: &str,
    ) -> AgentCommand {
        self.log.push(match resume {
            Some(session) => format!("command {identity} --resume {session}"),
            None => format!("command {identity}"),
        });
        let mut args = vec![];
        if let Some(model) = &settings.model {
            args.extend(["--model".to_string(), model.clone()]);
        }
        if let Some(effort) = &settings.effort {
            args.extend(["--effort".to_string(), effort.clone()]);
        }
        if let Some(mode) = mode {
            args.extend(["--mode".to_string(), mode.to_string()]);
        }
        let mut files = BTreeMap::new();
        if !own.is_empty() {
            let keys: Vec<&str> = own.keys().map(String::as_str).collect();
            self.log
                .push(format!("own {identity}: {}", keys.join(", ")));
            let (takes, at) = {
                let state = lock(&self.state);
                (state.takes_own, state.own_at.clone())
            };
            if takes {
                let file = at.unwrap_or_else(|| Self::own_file(folder, identity));
                args.extend(["--own".to_string(), file.clone()]);
                files.insert(
                    file,
                    serde_json::to_string(own).expect("JSON values are written as JSON"),
                );
            }
        }
        if let Some(session) = resume {
            args.extend(["--resume".to_string(), session.to_string()]);
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
            files,
        }
    }

    fn started(&self, folder: &str, agent: &Identity) -> Reading {
        let Some(model) = self.read_in("started", folder, agent) else {
            return Reading::Unknown;
        };
        if self.ends_unfound(agent) {
            return Reading::No;
        }
        if model.started.is_none() && self.unsettled_at_start(agent, &model) {
            return Reading::No;
        }
        model
            .started
            .unwrap_or(if self.on(agent).opened(agent).is_some() {
                Reading::Yes
            } else {
                Reading::No
            })
    }

    fn received(&self, folder: &str, agent: &Identity, text: &str) -> Received {
        let Some(model) = self.read_in("received", folder, agent) else {
            return Received::Unknown;
        };
        let Some((command, carried)) = self.on(agent).opened(agent) else {
            return Received::Unknown;
        };
        if model.received_unknown {
            return Received::Unknown;
        }
        let record = model
            .record
            .clone()
            .unwrap_or_else(|| format!("record:{agent}"));
        if self.held_back(agent, &model) {
            return Received::NotReceived { record };
        }
        let mut sendings = self
            .on(agent)
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
            .map_or(Reading::Unknown, |record| {
                if self.ends_unfound(agent) {
                    Reading::Yes
                } else {
                    record.ended
                }
            })
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

    /// The answer scripted for the account by its [`FakeAgent::HOME`],
    /// each logged with its answer: not offered where no account's is
    /// scripted, as the seam's default; otherwise no, naming the variable,
    /// to an account none is scripted for, or for a stopped start whose
    /// record is kept in another folder (128, 179).
    fn carry(&self, folder: &str, earlier: &Identity, env: &BTreeMap<String, String>) -> Carry {
        let home = env.get(Self::HOME);
        let known = self
            .record(earlier)
            .folder
            .is_none_or(|kept| kept == folder);
        let answer = {
            let state = lock(&self.state);
            if state.carries.is_empty() {
                Carry::NotOffered
            } else if !known {
                Carry::No {
                    why: "the agent that stopped is not known in this place, so its \
                          conversation is not known"
                        .into(),
                }
            } else {
                state
                    .carries
                    .get(home.map_or(Self::LOGIN, String::as_str))
                    .cloned()
                    .unwrap_or_else(|| Carry::No {
                        why: format!(
                            "the account's {} reaches no conversation of the fake agent's",
                            Self::HOME
                        ),
                    })
            }
        };
        self.log.push(format!(
            "carry {earlier} in {folder} to {}: {}",
            home.map_or(Self::LOGIN, String::as_str),
            match &answer {
                Carry::Yes { session } => format!("yes {session}"),
                Carry::No { .. } => "no".to_string(),
                Carry::NotOffered => "not offered".to_string(),
            }
        ));
        answer
    }

    /// The answer scripted for the session `start`'s word names, as
    /// scripted or as its command resumes it, each logged with its answer:
    /// none where no session is scripted live, where the start names none,
    /// or where its record is kept in another folder, so its word is not
    /// read there; the seam's default where set to read none (128, 207).
    fn live(&self, folder: &str, start: &Identity, envs: &[BTreeMap<String, String>]) -> Live {
        let record = self.record(start);
        let session = record
            .session
            .clone()
            .or_else(|| self.resumed(start))
            .filter(|_| record.folder.as_ref().is_none_or(|kept| kept == folder));
        let answer = {
            let state = lock(&self.state);
            if state.live_not_read {
                Live::Unknown {
                    what: READS_NO_SESSION.to_string(),
                }
            } else {
                session
                    .as_ref()
                    .and_then(|session| state.lives.get(session))
                    .cloned()
                    .unwrap_or(Live::None)
            }
        };
        let homes: Vec<&str> = envs
            .iter()
            .map(|env| env.get(Self::HOME).map_or("no home", String::as_str))
            .collect();
        self.log.push(format!(
            "live {start} in {folder} under {}: {}",
            if homes.is_empty() {
                "no account".to_string()
            } else {
                homes.join(", ")
            },
            match &answer {
                Live::Runs(processes) => format!("runs {}", pids(processes)),
                Live::None => "none".to_string(),
                Live::Unknown { .. } => "unknown".to_string(),
            }
        ));
        answer
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

    /// Each word scripted taken, any other refused naming it and the
    /// words taken, and every word refused in the seam's default words
    /// where none is scripted, each asking logged (197).
    fn effort(&self, word: &str) -> Result<(), String> {
        self.log.push(format!("effort {word}"));
        let state = lock(&self.state);
        if state.efforts.is_empty() {
            Err(no_effort(word))
        } else if state.efforts.iter().any(|taken| taken == word) {
            Ok(())
        } else {
            Err(format!(
                "`{word}` is no effort the fake agent takes, which are {}",
                state.efforts.join(", ")
            ))
        }
    }

    fn effort_variable(&self) -> Option<&'static str> {
        lock(&self.state).effort_variable
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

    fn reserved(&self) -> Vec<Reserved> {
        lock(&self.state).reserved.clone()
    }

    /// Every entry where the fake is scripted to take own settings; a
    /// non-empty one refused in the seam's default words otherwise (170,
    /// 195).
    fn own(&self, given: &BTreeMap<String, serde_json::Value>) -> Result<(), String> {
        if given.is_empty() || lock(&self.state).takes_own {
            Ok(())
        } else {
            Err(NO_OWN.into())
        }
    }

    fn stop_short(&self, _agent: &Identity) -> String {
        "to stop short needing an answer, run `flume needs \"<what you need>\"`; having found work the stage cannot deliver past, run `flume needs --found \"<the work you found>\"`".into()
    }

    fn stop_short_to(&self) -> String {
        "add `--to <name>` to `flume needs`".into()
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

/// Each process's id, joined by commas.
fn pids(processes: &[LiveProcess]) -> String {
    processes
        .iter()
        .map(|process| process.pid.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
