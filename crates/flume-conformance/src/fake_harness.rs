//! Each seam's harness over the fakes, so the fakes pass the same seam
//! scenarios a profile does (106).

use crate::seams::{
    agent::{AgentHarness, CarryAccounts, Efforts, LiveSessions, Modes, OwnWay},
    change::ChangeHarness,
    history::HistoryHarness,
    kinds::{KindsHarness, SourceDir},
    merge_queue::MergeQueueHarness,
    place::{Pinned, PlaceHarness},
    store::StoreHarness,
    terminal::TerminalHarness,
    window::WindowHarness,
};
use flume_fakes::*;
use flume_seams::*;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

const REPOSITORY: &str = "/repository";

/// flume's own kinds as Part C's table gives them: fix@1, chore@1,
/// change@1, change@2 and change@3 side by side, each sent back to code,
/// change@2's code stage repeating per task group with its group, notes
/// and words last, and change@3's one agent for the whole change with a
/// code prompt of its own (22, 140, 141, 157, C.2; core.31).
pub fn flumes_own() -> Vec<Kind> {
    let stage = |name: &str, deliverable: Deliverable, prompt: &str| Stage {
        name: name.into(),
        prompt: prompt.into(),
        conversation: false,
        deliverables: vec![deliverable],
        agents: vec![Settings::default()],
        waits: false,
        quiet_limit: None,
        send_back_to: None,
        send_back_limit: None,
        makes: vec![],
        repeats: false,
    };
    let mut code = stage(
        "code",
        Deliverable::Commits,
        "Fix {{name}} on {{branch}}.\n\n{{brief}}\n\n{{#words}}{{words}}\n\n{{/words}}{{unready}}\n\n{{needs}}",
    );
    code.send_back_to = Some("code".into());
    let mut verify = stage(
        "verify",
        Deliverable::ChangeArchived,
        "Verify {{change}} against the work, commit the report, and archive it.\n\n{{#words}}{{words}}\n\n{{/words}}{{needs}}",
    );
    verify.send_back_to = Some("code".into());
    let change = Kind {
        name: "change".into(),
        version: 1,
        branch_prefix: "change".into(),
        starts_unready: false,
        lands: true,
        agents: KindSettings::default(),
        label: None,
        stages: vec![
            stage(
                "construct",
                Deliverable::ChangeWritten,
                "Write the change {{change}}.\n\n{{brief}}\n\n{{#words}}{{words}}\n\n{{/words}}{{needs}}",
            ),
            stage(
                "code",
                Deliverable::TasksTicked,
                "Build {{change}}, ticking each task.\n\n{{#words}}{{words}}\n\n{{/words}}{{needs}}",
            ),
            verify,
        ],
        fix: None,
        fix_limit: None,
    };
    let mut repeating = change.clone();
    repeating.version = 2;
    repeating.stages[1].repeats = true;
    repeating.stages[1].prompt = "Build {{change}}, one task group of it, ticking each task.\n\n{{needs}}\n\n{{group}}\n\nThe notes: {{notes}}{{#words}}\n\n{{words}}{{/words}}".into();
    let mut one_agent = change.clone();
    one_agent.version = 3;
    one_agent.stages[1].prompt = "Build {{change}}, every task of it, running each task's own check and ticking it once the check passes.\n\n{{needs}}{{#words}}\n\n{{words}}{{/words}}".into();
    vec![
        Kind {
            name: "fix".into(),
            version: 1,
            branch_prefix: "fix".into(),
            starts_unready: true,
            lands: true,
            agents: KindSettings::default(),
            label: None,
            stages: vec![code.clone()],
            fix: None,
            fix_limit: None,
        },
        Kind {
            name: "chore".into(),
            version: 1,
            branch_prefix: "chore".into(),
            starts_unready: true,
            lands: true,
            agents: KindSettings::default(),
            label: None,
            stages: vec![code],
            fix: None,
            fix_limit: None,
        },
        change,
        repeating,
        one_agent,
    ]
}

pub struct FakePlaceHarness {
    pub git: Arc<FakeGit>,
    pub place: FakePlace,
}

impl Default for FakePlaceHarness {
    fn default() -> Self {
        let git = FakeGit::new();
        git.init(REPOSITORY, "main");
        FakePlaceHarness {
            place: FakePlace::new(git.clone()),
            git,
        }
    }
}

impl PlaceHarness for FakePlaceHarness {
    type Seam = FakePlace;

    fn seam(&self) -> &FakePlace {
        &self.place
    }

    fn repository(&self) -> (String, String, Revision) {
        let head = self.git.branch_head(REPOSITORY, "main").expect("main");
        (REPOSITORY.into(), "main".into(), head)
    }

    fn branch(&self, repository: &str, branch: &str) -> Revision {
        let base = self.git.branch_head(repository, "main").expect("main");
        self.git.set_branch(repository, branch, &base);
        self.git.commit(repository, branch, &["by-hand.txt"])
    }

    fn commit(&self, place: &PlaceRef) -> Revision {
        self.git
            .commit(&place.repository, &place.branch, &["work.rs"])
    }

    fn head(&self, repository: &str, branch: &str) -> Option<Revision> {
        self.git.branch_head(repository, branch)
    }

    fn occupy(&self, location: &str) {
        self.place.occupy(location);
    }

    fn readiness(&self, repository: &str, steps: &[(&str, bool)]) {
        let steps: Vec<(&str, bool, &str)> = steps
            .iter()
            .map(|(step, passed)| (*step, *passed, if *passed { "ok" } else { "failed" }))
            .collect();
        self.place.set_readiness(repository, &steps);
    }

    /// The fake runs the readiness the repository declares, here the
    /// toolchain, the pins' install and the setup that runs what it
    /// installed.
    fn pinned(&self, repository: &str) -> Option<Pinned> {
        self.place.set_readiness(
            repository,
            &[
                ("toolchain", true, "ok"),
                ("install", true, "installed the pins"),
                ("setup", true, "ok"),
            ],
        );
        Some(Pinned {
            steps: vec!["toolchain".into(), "install".into(), "setup".into()],
            install: "install".into(),
            said: "installed the pins".into(),
        })
    }

    fn untracked(&self, place: &PlaceRef, file: &str) {
        self.place.set_holds(
            &place.location,
            Holds {
                untracked: vec![file.into()],
                ..Holds::default()
            },
        );
    }

    fn fail_teardown(&self, place: &PlaceRef, step: &str) {
        self.place.fail_teardown(&place.location, step, "failed");
    }

    fn apply_to_base(&self, _repository: &str, _base: &str, commit: &Revision) {
        self.git.apply(commit);
    }
}

#[derive(Default)]
pub struct FakeKindsHarness {
    pub git: Arc<FakeGit>,
    pub kinds: FakeKinds,
}

impl FakeKindsHarness {
    pub fn new() -> Self {
        let harness = FakeKindsHarness::default();
        harness.kinds.set_own(flumes_own());
        harness
    }
}

impl KindsHarness for FakeKindsHarness {
    type Seam = FakeKinds;

    fn seam(&self) -> &FakeKinds {
        &self.kinds
    }

    fn declaring(&self, kinds: &[Kind]) -> (String, Revision) {
        let revision = self.git.init(REPOSITORY, "main");
        self.kinds
            .set_kinds_at(REPOSITORY, &revision, kinds.to_vec());
        (REPOSITORY.into(), revision)
    }

    fn launch_declaration(&self, kinds: &[Kind]) -> String {
        self.kinds.set_declaration("launch.toml", kinds.to_vec());
        "launch.toml".into()
    }

    fn unreadable(&self) -> (String, Revision) {
        let revision = self.git.init("/broken", "main");
        self.kinds
            .unreadable_kinds("/broken", "the declaration is not valid");
        ("/broken".into(), revision)
    }

    fn binding(&self, binding: &Binding) -> String {
        self.kinds.set_binding(REPOSITORY, binding.clone());
        REPOSITORY.into()
    }

    fn source(&self, kinds: &[Kind]) -> Option<SourceDir> {
        let dir = acme();
        for kind in kinds {
            let hash = packaged(&dir.source, kind.clone(), None).hash;
            self.kinds.set_package(&dir.kinds_dir, kind.clone(), &hash);
        }
        Some(dir)
    }

    fn unreadable_source(&self) -> Option<SourceDir> {
        let dir = SourceDir {
            source: "broken".into(),
            kinds_dir: "/kinds/broken".into(),
        };
        self.kinds
            .unreadable_source(&dir.kinds_dir, "the directory cannot be listed");
        Some(dir)
    }

    fn refused_package(&self) -> Option<(SourceDir, String, u32)> {
        let dir = acme();
        self.kinds.refuse_package(
            &dir.kinds_dir,
            "review",
            3,
            "kinds.review.version: makes the package acme/review@2, but it is held at acme/review@3",
        );
        Some((dir, "review".into(), 3))
    }

    fn lock_at(&self, entries: &[LockEntry]) -> Option<(String, Revision)> {
        let revision = self.git.init("/locked", "main");
        self.kinds
            .set_lock_at("/locked", &revision, entries.to_vec());
        Some(("/locked".into(), revision))
    }

    fn no_lock_at(&self) -> Option<(String, Revision)> {
        Some(("/unlocked".into(), self.git.init("/unlocked", "main")))
    }

    fn lock_file(&self, entries: &[LockEntry]) -> Option<String> {
        let path = "/config/flume/flume.lock";
        self.kinds.set_lock_file(path, entries.to_vec());
        Some(path.into())
    }

    fn no_lock_file(&self) -> Option<String> {
        Some("/work/.config/flume.lock".into())
    }

    fn unreadable_lock_file(&self) -> Option<String> {
        let path = "/broken/flume.lock";
        self.kinds
            .unreadable_lock_file(path, "kinds[0].hash: is no content hash");
        Some(path.into())
    }

    fn unwritable_lock(&self, entries: &[LockEntry]) -> Option<String> {
        let path = "/read-only/flume.lock";
        self.kinds.set_lock_file(path, entries.to_vec());
        self.kinds
            .unwritable_lock(path, "/read-only is not writable");
        Some(path.into())
    }
}

/// The source `acme`, its packages in `/kinds/acme`.
fn acme() -> SourceDir {
    SourceDir {
        source: "acme".into(),
        kinds_dir: "/kinds/acme".into(),
    }
}

pub struct FakeChangeHarness {
    pub git: Arc<FakeGit>,
    pub change: FakeChange,
}

impl Default for FakeChangeHarness {
    fn default() -> Self {
        let git = FakeGit::new();
        git.init(REPOSITORY, "main");
        FakeChangeHarness {
            change: FakeChange::new(git.clone()),
            git,
        }
    }
}

impl ChangeHarness for FakeChangeHarness {
    type Seam = FakeChange;

    fn seam(&self) -> &FakeChange {
        &self.change
    }

    fn committed(&self, name: &str, facts: &ChangeFacts) -> (String, Revision) {
        let path = format!("{}/tasks.md", self.change.path(name));
        let revision = self.git.commit(REPOSITORY, "main", &[&path]);
        self.change.set(REPOSITORY, &revision, name, facts.clone());
        (REPOSITORY.into(), revision)
    }
}

pub struct FakeAgentHarness {
    pub host: Arc<FakeHost>,
    pub terminal: Arc<FakeTerminal>,
    pub agent: FakeAgent,
    /// Whether the harness scripts the fake to carry a conversation and
    /// read a running session, or leaves it carrying none and reading none,
    /// as a profile that keeps the seam's defaults (179, 207).
    carries: bool,
    /// How the harness scripted the fake to take own settings, by
    /// [`fake_own`], or left it keeping the seam's defaults (195, 196).
    own: OwnWay,
    /// The efforts the harness scripted the fake to take, by
    /// [`fake_efforts`], or none where it keeps the seam's defaults (197).
    efforts: Efforts,
}

impl FakeAgentHarness {
    /// The fake left carrying no conversation and reading no running
    /// session, as a profile that keeps the seam's defaults (179, 207).
    pub fn carrying_none() -> Self {
        let harness = FakeAgentHarness {
            carries: false,
            ..FakeAgentHarness::default()
        };
        harness.agent.live_not_read();
        harness
    }

    /// The fake left taking no own settings and reserving no key, as a
    /// profile that keeps the seam's defaults (195, 196).
    pub fn taking_no_own() -> Self {
        let host = Arc::new(FakeHost::default());
        let terminal = Arc::new(FakeTerminal::new(host.clone()));
        FakeAgentHarness {
            agent: scripted(
                FakeAgent::new(terminal.clone()),
                &OwnWay::default(),
                &fake_efforts(),
            ),
            terminal,
            host,
            carries: true,
            own: OwnWay::default(),
            efforts: fake_efforts(),
        }
    }

    /// The fake left taking no effort and naming no variable read over
    /// one, as a profile that keeps the seam's defaults (197).
    pub fn taking_no_effort() -> Self {
        let host = Arc::new(FakeHost::default());
        let terminal = Arc::new(FakeTerminal::new(host.clone()));
        let own = fake_own();
        FakeAgentHarness {
            agent: scripted(FakeAgent::new(terminal.clone()), &own, &Efforts::default()),
            terminal,
            host,
            carries: true,
            own,
            efforts: Efforts::default(),
        }
    }
}

/// The fake agent's efforts the seam's scenarios run with: `low` and
/// `high` taken, and `FAKE_EFFORT` read over them (197).
pub fn fake_efforts() -> Efforts {
    Efforts {
        words: vec!["low".into(), "high".into()],
        variable: Some("FAKE_EFFORT"),
    }
}

/// The fake agent's own settings the seam's scenarios run with: taken by
/// a file, `env` reserved for the account and `permissions.default`, by
/// its path, for flume's mode (195, 196).
pub fn fake_own() -> OwnWay {
    OwnWay {
        takes: true,
        reserved: vec![
            Reserved {
                key: "env".into(),
                owner: "the account, whose env is an agent's whole environment".into(),
            },
            Reserved {
                key: "permissions.default".into(),
                owner: "flume's mode".into(),
            },
        ],
    }
}

/// `agent` scripted with [`fake_modes`], reading the mode back, taking
/// own settings as `own` says, and taking the efforts `efforts` gives
/// (161, 169, 195, 197).
fn scripted(agent: FakeAgent, own: &OwnWay, efforts: &Efforts) -> FakeAgent {
    let modes = fake_modes();
    for (mode, value) in &modes.four {
        agent.map_mode(*mode, value);
    }
    for value in &modes.own {
        agent.own_mode(value);
    }
    agent.read_mode_back();
    if own.takes {
        agent.take_own();
    }
    for Reserved { key, owner } in &own.reserved {
        agent.reserve(key, owner);
    }
    let words: Vec<&str> = efforts.words.iter().map(String::as_str).collect();
    if !words.is_empty() {
        agent.take_efforts(&words);
    }
    if let Some(variable) = efforts.variable {
        agent.read_effort_over(variable);
    }
    agent
}

/// The fake agent's map of modes the seam's scenarios run with: each of
/// flume's four to a setting of its own, and the own value `careful`
/// (169).
pub fn fake_modes() -> Modes {
    Modes {
        four: BTreeMap::from([
            (Mode::Plan, "planning".to_string()),
            (Mode::Ask, "ask-each".to_string()),
            (Mode::Edits, "edits-only".to_string()),
            (Mode::Full, "all".to_string()),
        ]),
        own: vec!["careful".into()],
    }
}

impl Default for FakeAgentHarness {
    /// The fake agent scripted with [`fake_modes`], reading the mode
    /// back, with [`fake_own`] and with [`fake_efforts`] (161, 169, 195,
    /// 197).
    fn default() -> Self {
        let host = Arc::new(FakeHost::default());
        let terminal = Arc::new(FakeTerminal::new(host.clone()));
        let own = fake_own();
        FakeAgentHarness {
            agent: scripted(FakeAgent::new(terminal.clone()), &own, &fake_efforts()),
            terminal,
            host,
            carries: true,
            own,
            efforts: fake_efforts(),
        }
    }
}

fn a_place() -> PlaceRef {
    PlaceRef {
        repository: REPOSITORY.into(),
        location: place_location(REPOSITORY, "fix--one"),
        branch: "fix/one".into(),
        base: "main".into(),
    }
}

impl AgentHarness for FakeAgentHarness {
    type Seam = FakeAgent;

    fn seam(&self) -> &FakeAgent {
        &self.agent
    }

    fn place(&self) -> PlaceRef {
        a_place()
    }

    fn folder(&self) -> String {
        "/host/state/flume/places/fix--one".into()
    }

    fn start(&self, command: &AgentCommand, identity: &Identity, folder: &str) {
        self.terminal
            .open(&a_place(), folder, command, identity, &OpenIn::Own, None);
        self.agent.keep_record_in(identity, folder);
    }

    fn deliver(&self, identity: &Identity, text: &str) {
        self.terminal.send(identity, text);
    }

    fn ask(&self, identity: &Identity) {
        self.agent.set_asking(identity, Reading::Yes);
    }

    fn start_own_work(&self, identity: &Identity) {
        self.agent.set_own_work(identity, Reading::Yes);
    }

    fn hide_record(&self, identity: &Identity) {
        self.agent.set_own_work(identity, Reading::Unknown);
        self.agent.received_unknown(identity);
    }

    fn try_push(&self, identity: &Identity) {
        self.agent.refuse_act(identity, "push of fix/one");
    }

    fn end_session(&self, identity: &Identity) {
        self.agent.set_ended(identity, Reading::Yes);
    }

    fn say_last(&self, identity: &Identity, words: &str) {
        self.agent.set_last_words(identity, words);
    }

    fn hit_usage_limit(&self, identity: &Identity) -> Seconds {
        let reset = self.host.now() + 2 * 3600;
        self.agent.set_spent(
            identity,
            Spent::Yes {
                until: Some(reset),
                words: "a usage limit this harness writes".into(),
            },
        );
        reset
    }

    fn begin_without(&self, identity: &Identity, variable: &str) {
        self.agent.set_started_without(
            identity,
            Unsettled {
                missing: vec![variable.to_string()],
                ..Unsettled::default()
            },
        );
    }

    fn modes(&self) -> Modes {
        fake_modes()
    }

    /// The fake compares `began` with the mode its command passed, which
    /// is `asked`.
    fn begin_in(&self, identity: &Identity, _asked: &str, began: &str) {
        self.agent.set_began_in(identity, began);
    }

    /// The fake scripted to read `runs`'s conversation run by process
    /// 4242 and `unknown`'s as one it cannot read, by the sessions their
    /// words name; none where the harness leaves it reading no running
    /// session (207).
    fn live_sessions(&self, runs: &Identity, unknown: &Identity) -> Option<LiveSessions> {
        if !self.carries {
            return None;
        }
        let processes = vec![LiveProcess {
            pid: 4242,
            started: self.host.now(),
        }];
        self.agent.set_session(runs, "s-live");
        self.agent.live_runs("s-live", &processes);
        self.agent.set_session(unknown, "s-unread");
        self.agent.live_unknown(
            "s-unread",
            &format!(
                "the sessions under the account's {} cannot be read",
                FakeAgent::HOME
            ),
        );
        let home = BTreeMap::from([(
            FakeAgent::HOME.to_string(),
            "/scratch/agent-work-2".to_string(),
        )]);
        Some(LiveSessions {
            envs: vec![BTreeMap::new(), home.clone()],
            processes,
            unreadable: vec![home],
        })
    }

    /// The fake scripted to carry to one account's home with the session
    /// `s-1`, and to answer no to another's with words naming its
    /// variable; none where the harness leaves it carrying none (179).
    fn carry_accounts(&self, _earlier: &Identity) -> Option<CarryAccounts> {
        if !self.carries {
            return None;
        }
        let home = |home: &str| BTreeMap::from([(FakeAgent::HOME.to_string(), home.to_string())]);
        self.agent.carry_to(
            "/scratch/agent-work-2",
            Carry::Yes {
                session: "s-1".into(),
            },
        );
        self.agent.carry_to(
            "/scratch/agent-work-3",
            Carry::No {
                why: format!(
                    "the account's {} does not reach its conversation",
                    FakeAgent::HOME
                ),
            },
        );
        Some(CarryAccounts {
            reaches: home("/scratch/agent-work-2"),
            session: "s-1".into(),
            reaches_none: home("/scratch/agent-work-3"),
        })
    }

    fn own_way(&self) -> OwnWay {
        self.own.clone()
    }

    /// The JSON object of the file the command names by `--own`, read
    /// from the files it lists; an empty one where it lists no such file,
    /// and none where it names none (196).
    fn efforts(&self) -> Efforts {
        self.efforts.clone()
    }

    fn own_passed(&self, command: &AgentCommand) -> Option<BTreeMap<String, serde_json::Value>> {
        let file = command
            .args
            .iter()
            .skip_while(|arg| *arg != "--own")
            .nth(1)?;
        Some(
            command
                .files
                .get(file)
                .and_then(|text| serde_json::from_str(text).ok())
                .unwrap_or_default(),
        )
    }
}

pub struct FakeTerminalHarness {
    pub host: Arc<FakeHost>,
    pub terminal: Arc<FakeTerminal>,
    sends: bool,
}

impl FakeTerminalHarness {
    /// A terminal that sends, or, as the printed command, one that cannot.
    pub fn new(sends: bool) -> Self {
        let host = Arc::new(FakeHost::default());
        let terminal = Arc::new(FakeTerminal::new(host.clone()));
        if sends {
            terminal.set_session(Some("pane-1"));
        } else {
            terminal.cannot_send();
        }
        FakeTerminalHarness {
            host,
            terminal,
            sends,
        }
    }
}

impl TerminalHarness for FakeTerminalHarness {
    type Seam = FakeTerminal;

    fn seam(&self) -> &FakeTerminal {
        &self.terminal
    }

    fn shared(&self) -> Arc<dyn Terminal> {
        self.terminal.clone()
    }

    fn place(&self) -> PlaceRef {
        a_place()
    }

    fn command(&self) -> AgentCommand {
        AgentCommand {
            program: "agent".into(),
            ..AgentCommand::default()
        }
    }

    fn sends(&self) -> bool {
        self.sends
    }

    fn closes(&self) -> bool {
        true
    }

    fn stubborn(&self, identity: &Identity) {
        self.terminal.refuse_close(identity);
    }

    fn session(&self) -> Option<String> {
        self.sends.then(|| "pane-1".to_string())
    }

    fn folder(&self) -> String {
        "/host/state/flume/places/fix--one".into()
    }

    fn shows(&self) -> bool {
        self.sends
    }

    fn leave_typed(&self, identity: &Identity, text: &str) {
        self.terminal.leave_typed(identity, text);
    }

    fn submitted(&self, identity: &Identity) -> Vec<String> {
        self.terminal.sent_to(identity)
    }

    fn held(&self) -> Option<AgentCommand> {
        self.terminal
            .hold_opens("WARNING: running in Bypass Permissions mode");
        Some(self.command())
    }

    fn never_up(&self) -> Option<AgentCommand> {
        self.terminal
            .opens_never_come_up("sh: agent: command not found");
        Some(self.command())
    }

    fn makes(&self) -> bool {
        true
    }

    fn person_pane(&self, place: &PlaceRef) {
        self.terminal.person_pane(place, "p9");
    }

    /// The fake holds its agents by identity, wherever a pane goes.
    fn move_pane(&self, _identity: &Identity) {}

    fn shows_clues(&self) -> bool {
        true
    }

    fn recipient(&self) -> Option<String> {
        self.terminal.add_session("ops", AgentState::Waiting);
        Some("ops".into())
    }

    fn received(&self, session: &str) -> Vec<String> {
        self.terminal.notices(session)
    }

    fn no_session(&self) -> String {
        "nobody".into()
    }

    fn views(&self) -> Vec<View> {
        vec![View::Terminal, View::App]
    }

    fn parts(&self) -> Vec<Option<String>> {
        vec![Some("work".into()), None]
    }

    fn refuse_attach(&self, view: View) -> bool {
        self.terminal.attaches_as(
            view,
            Some(Attached::Refused {
                reason: format!("no {}", view.name()),
            }),
        );
        true
    }

    fn setup(&self) -> Vec<String> {
        self.terminal.set_setup(&[SETUP]);
        vec![SETUP.to_string()]
    }

    /// `work` scripted running and `desk` unread with `no listing`;
    /// `office`, never scripted, and its own part not running, the own as
    /// `own` (201).
    fn runs_parts(&self) -> Vec<(Option<String>, SessionRuns)> {
        let work = SessionRuns::Runs {
            name: "work".into(),
        };
        let desk = SessionRuns::Unread {
            reason: "no listing".into(),
        };
        self.terminal.runs_as("work", work.clone());
        self.terminal.runs_as("desk", desk.clone());
        vec![
            (Some("work".into()), work),
            (Some("desk".into()), desk),
            (
                Some("office".into()),
                SessionRuns::NotRunning {
                    name: "office".into(),
                },
            ),
            (None, SessionRuns::NotRunning { name: "own".into() }),
        ]
    }

    fn logged(&self) -> Option<Vec<String>> {
        Some(self.terminal.log.calls())
    }

    /// The fake scripts the agent in the part, waiting (132).
    fn named_agent(&self, session: &str, name: &str) -> bool {
        self.terminal
            .add_agent(Some(session), name, AgentState::Waiting);
        true
    }

    /// The fake relays while it sends, and, as the printed command, not
    /// where it cannot (221, 222).
    fn relays(&self) -> bool {
        self.sends
    }
}

/// The fake window, its shows answered as each scenario scripts them.
#[derive(Default)]
pub struct FakeWindowHarness {
    pub window: FakeWindow,
}

impl WindowHarness for FakeWindowHarness {
    type Seam = FakeWindow;

    fn seam(&self) -> &FakeWindow {
        &self.window
    }

    fn command(&self) -> ViewCommand {
        ViewCommand {
            program: "/bin/fake-terminal".into(),
            arguments: vec!["attach".into(), "work".into()],
            environment: BTreeMap::from([("XDG_CONFIG_HOME".into(), "/scratch/home".into())]),
        }
    }

    fn opens(&self) -> bool {
        true
    }

    fn opened(&self) -> Vec<(ViewCommand, String)> {
        self.window.windows()
    }

    fn will_print(&self) -> bool {
        self.window.answers(Some(WindowShown::Printed {
            line: "XDG_CONFIG_HOME=<set> /bin/fake-terminal attach work".into(),
        }));
        true
    }

    fn will_refuse(&self) -> Option<String> {
        let words = "no display";
        self.window.answers(Some(WindowShown::Refused {
            reason: words.into(),
        }));
        Some(words.into())
    }

    /// The fake answers at once, and has no program to hold.
    fn will_hang(&self) -> Option<Duration> {
        None
    }
}

/// What the fake terminal's configuration is scripted to lack for
/// flume's plugin (122).
pub const SETUP: &str =
    "no key opens flume's places: bind one to the plugin's places in the terminal's configuration";

pub struct FakeMergeQueueHarness {
    pub git: Arc<FakeGit>,
    pub queue: FakeMergeQueue,
}

impl Default for FakeMergeQueueHarness {
    fn default() -> Self {
        let git = FakeGit::new();
        git.init(REPOSITORY, "main");
        FakeMergeQueueHarness {
            queue: FakeMergeQueue::new(git.clone()),
            git,
        }
    }
}

impl MergeQueueHarness for FakeMergeQueueHarness {
    type Seam = FakeMergeQueue;

    fn seam(&self) -> &FakeMergeQueue {
        &self.queue
    }

    fn place(&self) -> (PlaceRef, Revision) {
        let place = a_place();
        let base = self.git.branch_head(REPOSITORY, "main").expect("main");
        self.git.set_branch(REPOSITORY, &place.branch, &base);
        let head = self.git.commit(REPOSITORY, &place.branch, &["work.rs"]);
        (place, head)
    }

    fn will_fail(&self, words: &str) {
        self.queue.plan_submits(vec![Plan::Fail(words.into())]);
    }

    fn will_conflict(&self, place: &PlaceRef, file: &str) {
        self.git.commit(&place.repository, &place.base, &[file]);
        self.queue.plan_submits(vec![Plan::Conflict(
            vec![file.into()],
            format!("{file} conflicts with {}", place.base),
        )]);
    }

    fn will_hold(&self) {
        self.queue.plan_submits(vec![Plan::Hold]);
    }

    fn until_decided(&self, _place: &PlaceRef) {}

    fn withdraw(&self, place: &PlaceRef) {
        self.queue
            .decide(&place.repository, &place.branch, Plan::Withdraw);
    }

    fn land_elsewhere(&self, place: &PlaceRef) {
        let head = self
            .git
            .branch_head(&place.repository, &place.branch)
            .expect("branch");
        self.git.set_branch(&place.repository, "other", &head);
        self.git.commit(&place.repository, "other", &["other.rs"]);
        self.queue
            .hand_landing(&place.repository, "other", &place.base);
    }

    fn open_submission(&self, place: &PlaceRef) -> Submission {
        self.queue.hand_submission(
            &place.repository,
            &place.branch,
            &place.base,
            SubmissionState::Waiting,
        )
    }

    fn base_elsewhere(&self, repository: &str, base: &str) -> Revision {
        let remote = Revision::new("remote-head");
        self.queue.set_base(
            repository,
            base,
            BaseRevision::At {
                revision: remote.clone(),
            },
        );
        remote
    }

    fn fetch_fails(&self, repository: &str, base: &str) {
        self.queue.set_base(
            repository,
            base,
            BaseRevision::Refused {
                reason: "the fetch failed".into(),
            },
        );
    }

    fn base_unlanded(&self, repository: &str) -> String {
        self.queue
            .set_base(repository, "trial", BaseRevision::Unlanded);
        "trial".into()
    }

    fn unreadable(&self) {
        self.queue.unreadable("the merge queue is down");
    }

    fn refuse_another(&self, place: &PlaceRef) -> String {
        let other = "bolt/x";
        let base = self
            .git
            .branch_head(&place.repository, &place.base)
            .expect("the base");
        self.git.set_branch(&place.repository, other, &base);
        self.git.set_branch(&place.repository, "unit/x", &base);
        self.git.commit(&place.repository, "unit/x", &["x.rs"]);
        self.queue
            .hand_submission(&place.repository, "unit/x", other, SubmissionState::Waiting);
        self.queue.refuse_branch(
            &place.repository,
            other,
            &format!("{other}'s policy cannot be read"),
        );
        other.into()
    }

    fn branch_head(&self, repository: &str, branch: &str) -> Option<Revision> {
        self.git.branch_head(repository, branch)
    }

    fn change_on(&self, repository: &str, branch: &str, file: &str) -> Revision {
        self.git.commit(repository, branch, &[file])
    }

    /// The fake forms no merge: its forecast of `branch` is scripted in
    /// conflict on `file` with the base's new revision (210).
    fn move_base_against(
        &self,
        repository: &str,
        base: &str,
        branch: &str,
        file: &str,
    ) -> Revision {
        let moved = self.git.commit(repository, base, &[file]);
        let head = self
            .git
            .branch_head(repository, branch)
            .expect("the branch");
        self.queue.answer_forecast(
            repository,
            branch,
            Forecast::Conflict {
                head,
                onto: moved.clone(),
                files: vec![file.into()],
                with: vec![],
                words: format!("would conflict: {file} with {base}"),
            },
        );
        moved
    }
}

#[derive(Default)]
pub struct FakeStoreHarness {
    pub store: Arc<FakeStore>,
}

impl StoreHarness for FakeStoreHarness {
    type Seam = FakeStore;

    fn seam(&self) -> &FakeStore {
        &self.store
    }

    fn another(&self) -> Arc<dyn Store> {
        self.store.clone()
    }

    fn repository(&self) -> String {
        REPOSITORY.into()
    }

    fn record_made(&self, place: &PlaceId, line: &str) {
        self.store.record_made(place, line);
    }
}

pub struct FakeHistoryHarness {
    pub git: Arc<FakeGit>,
    pub history: FakeHistory,
}

impl Default for FakeHistoryHarness {
    fn default() -> Self {
        let git = FakeGit::new();
        FakeHistoryHarness {
            history: FakeHistory::new(git.clone()),
            git,
        }
    }
}

impl HistoryHarness for FakeHistoryHarness {
    type Seam = FakeHistory;

    fn seam(&self) -> &FakeHistory {
        &self.history
    }

    fn branch(&self) -> (String, String, Revision) {
        let first = self.git.init(REPOSITORY, "fix/one");
        (REPOSITORY.into(), "fix/one".into(), first)
    }

    fn commit(&self, repository: &str, branch: &str, path: &str) -> Revision {
        self.git.commit(repository, branch, &[path])
    }

    fn reset(&self, repository: &str, branch: &str, revision: &Revision) {
        self.git.set_branch(repository, branch, revision);
    }

    fn made_default(&self) -> String {
        "fix/one".into()
    }

    fn origin_ahead(&self, repository: &str, branch: &str) -> Revision {
        let own = self
            .git
            .branch_head(repository, branch)
            .expect("the branch");
        self.git.set_branch(repository, "origin-copy", &own);
        self.git.commit(repository, "origin-copy", &["one.rs"]);
        let ahead = self.git.commit(repository, "origin-copy", &["two.rs"]);
        self.git.set_origin(repository, branch, &ahead);
        ahead
    }

    fn origin_gone(&self, repository: &str) {
        self.git
            .origin_fails(repository, "origin does not appear to be a git repository");
    }

    fn origin_named(&self, repository: &str, url: &str) {
        self.git.set_origin_url(repository, url);
    }

    fn origin_removed(&self, repository: &str) {
        self.git.remove_origin_url(repository);
    }
}
