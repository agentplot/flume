//! Each seam's harness over the fakes, so the fakes pass the same seam
//! scenarios a profile does (106).

use crate::seams::{
    agent::AgentHarness, change::ChangeHarness, history::HistoryHarness, kinds::KindsHarness,
    merge_queue::MergeQueueHarness, place::PlaceHarness, store::StoreHarness,
    terminal::TerminalHarness,
};
use flume_fakes::*;
use flume_seams::*;
use std::sync::Arc;

const REPOSITORY: &str = "/repository";

/// flume's own kinds as Part C's table gives them: fix@1, chore@1 and
/// change@1, each sent back to code (22, C.2).
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
    vec![
        Kind {
            name: "fix".into(),
            version: 1,
            branch_prefix: "fix".into(),
            starts_unready: true,
            lands: true,
            agents: KindSettings::default(),
            stages: vec![code.clone()],
        },
        Kind {
            name: "chore".into(),
            version: 1,
            branch_prefix: "chore".into(),
            starts_unready: true,
            lands: true,
            agents: KindSettings::default(),
            stages: vec![code],
        },
        Kind {
            name: "change".into(),
            version: 1,
            branch_prefix: "change".into(),
            starts_unready: false,
            lands: true,
            agents: KindSettings::default(),
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
        },
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

    fn location(&self, repository: &str, name: &str) -> String {
        format!("{repository}/places/fix--{name}")
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
}

impl Default for FakeAgentHarness {
    fn default() -> Self {
        let host = Arc::new(FakeHost::default());
        let terminal = Arc::new(FakeTerminal::new(host.clone()));
        FakeAgentHarness {
            agent: FakeAgent::new(terminal.clone()),
            terminal,
            host,
        }
    }
}

fn a_place() -> PlaceRef {
    PlaceRef {
        repository: REPOSITORY.into(),
        location: format!("{REPOSITORY}/places/fix--one"),
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
            .open(&a_place(), folder, command, identity, None, None);
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
}

pub struct FakeTerminalHarness {
    pub host: Arc<FakeHost>,
    pub terminal: FakeTerminal,
    sends: bool,
}

impl FakeTerminalHarness {
    /// A terminal that sends, or, as the printed command, one that cannot.
    pub fn new(sends: bool) -> Self {
        let host = Arc::new(FakeHost::default());
        let terminal = FakeTerminal::new(host.clone());
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
}

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

    fn unreadable(&self) {
        self.queue.unreadable("the merge queue is down");
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
}
