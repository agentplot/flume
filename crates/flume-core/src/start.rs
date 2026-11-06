//! A stage's start (27 to 33): its settings resolved from their eight
//! layers, or the stage refused naming the setting (32), a fresh identity
//! and secret, its prompt filled or the stage refused naming the part,
//! and one agent opened through the agent seam's command in its stream's
//! terminal, never while another of the place's is live, and only under
//! the host's bound (70, 190). Each file the command lists is written
//! into the place's folder before the terminal opens the agent, and a
//! start whose command lists one outside it is not opened (196).

use crate::accounts::Choice;
use crate::live::LiveRead;
use crate::prompt::{asks, group_text, notes_path, render};
use crate::room::Admit;
use crate::start_record::start_record;
use crate::{repository_id, Engine, Step};
use flume_seams::*;
use std::collections::BTreeMap;
use std::sync::Arc;

/// A start written as live, its agent still to open.
pub(crate) struct Claim {
    identity: Identity,
    secret: Secret,
    text: String,
    place: PlaceRef,
    /// The settings the agent starts with.
    settings: Settings,
    /// The environment its account gives, each value read against the
    /// login environment (29, 144).
    env: BTreeMap<String, String>,
    agent: Arc<dyn Agent>,
    /// The session name the agent carries: a role's run's address (132).
    session: Option<String>,
    /// The terminal it opens in, its stream's as the host's
    /// configuration stands now (190).
    terminal: Arc<dyn Terminal>,
    /// The terminal's session the agent opens in, none where it opens
    /// beside the pane its settings name (107, 143).
    opens_in: Option<SessionHeld>,
    /// The account the start runs under (144, 145).
    account: Option<AccountHeld>,
    /// The value its agent profile passes for its mode, none where no
    /// layer gives one (162, 169).
    mode: Option<String>,
    /// The session it resumes, carrying the conversation of the stage's
    /// start whose agent stopped at its account's usage limit, or whose
    /// conversation was reported lost (49, 146, 179).
    resume: Option<String>,
    /// Its program's own settings as the layers resolved them, which its
    /// profile takes, none where they give none (195, 196).
    own: BTreeMap<String, serde_json::Value>,
}

impl Claim {
    /// The account the start runs under, none where it has no pool.
    pub(crate) fn account(&self) -> Option<&AccountHeld> {
        self.account.as_ref()
    }
}

/// What a start came to.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Start {
    Opened(Identity),
    /// The terminal read the start held at a question its program asks
    /// before it starts, which flume does not answer: the start waits on
    /// a person with the question's words (34, 35).
    Held(Identity),
    /// The terminal read the start's program not come up: never begun at
    /// once (35).
    NeverBegun(Identity),
    /// The host has no room, or stages waiting rank ahead of it (70).
    WaitingForRoom,
    /// The prompt asks for a part with nothing to fill it (27).
    Refused(PromptPart),
    /// Another agent of the place's is live (30).
    AgentLive,
    /// The terminal refused to open the agent.
    NotOpened(String),
    /// The stage's settings name a program no agent profile answers for,
    /// or a variable the host user's login environment lacks (27, 32,
    /// 105).
    Unresolved(String),
    NoPlace,
}

/// Why flume starts a stage afresh on its own, with words of its own in
/// its prompt where it asks for `{{words}}`, counted toward no bound of
/// the kind's and waiting on no caller (112).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StartReason {
    /// The stage's earlier agent stopped at its account's usage limit
    /// (146).
    Spent,
    /// The stage's earlier agent began without the settings its start
    /// recorded (161).
    Unsettled,
    /// The stage's earlier agent stopped at its account's usage limit, and
    /// this start resumes its conversation: its prompt is these words and
    /// the way to stop short alone (28, 146, 179).
    Carried,
    /// The stage, a conversation, was reported lost and its caller
    /// retried it, and this start resumes its conversation: its prompt is
    /// these words, the retry's words and the way to stop short (28, 49,
    /// 179).
    Resumed,
    /// The stage, a conversation, was reported lost and its caller
    /// retried it, and its conversation could not be carried: these words
    /// stand before the retry's in the stage's prompt (27, 49, 179).
    Lost,
}

impl StartReason {
    /// flume's words for the fresh start.
    pub(crate) fn words(self) -> String {
        match self {
            StartReason::Spent => "The stage's earlier agent stopped at its account's usage limit, and this agent starts the stage afresh. What it committed is on the place's branch and what it left is in the place: read them and carry the stage on from there.".into(),
            StartReason::Unsettled => "The stage's earlier agent began without the settings its start recorded and was ended, and this agent starts the stage afresh. Anything it committed is on the place's branch and anything it left is in the place: read them and carry the stage on from there.".into(),
            StartReason::Carried => "Your last turn stopped at your account's usage limit. This session now goes on under an account of the pool, with its conversation as it stood: carry on from where you stopped.".into(),
            StartReason::Resumed => "Your session was lost and goes on here, with its conversation as it stood. Anything you had running then has stopped: read the place again before you carry on.".into(),
            StartReason::Lost => "The stage's earlier agent was lost, and its conversation could not be carried, so this agent starts the stage afresh. What it committed is on the place's branch and what it left is in the place: read them and carry the stage on from there.".into(),
        }
    }
}

/// `first`, then `words` where there are any, joined by a blank line (27,
/// 28, 49).
fn then_words(first: String, words: Option<&str>) -> String {
    match words {
        Some(words) => format!("{first}\n\n{words}"),
        None => first,
    }
}

/// Why a start after a spent agent carries no conversation where its
/// agent profile offers no carry (171, 179).
pub(crate) const CARRIES_NONE: &str = "its agent profile carries no conversation";

/// Whether the start after `after`, the stage's start whose agent stopped
/// at its account's usage limit or whose conversation was reported lost,
/// carries that agent's conversation, as `carry`, its profile's answer
/// for the account chosen, says; or why not: the profile's words, or the
/// wait's where it may not carry, or that no account was chosen to carry
/// it to (49, 146, 179).
fn conversation(after: &CarryFrom, carry: Option<&Carry>) -> Conversation {
    let why = match carry {
        Some(Carry::Yes { .. }) => None,
        Some(Carry::No { why }) => Some(why.clone()),
        Some(Carry::NotOffered) => Some(CARRIES_NONE.to_string()),
        None => Some(
            after
                .why
                .clone()
                .filter(|_| !after.carry)
                .unwrap_or_else(|| "its start runs under no account".to_string()),
        ),
    };
    Conversation {
        from: after.start.start,
        cause: after.cause,
        carried: why.is_none(),
        why,
    }
}

/// What readiness came to, as a prompt's `{{unready}}` gives it (12).
fn unready(readiness: &Readiness) -> String {
    match readiness.failed() {
        Some(step) => format!(
            "The place's readiness failed at {}: {}",
            step.step, step.words
        ),
        None if readiness.steps.is_empty() => "The repository declares no readiness.".into(),
        None => "The place's readiness passed.".into(),
    }
}

impl Engine {
    /// A report's path: under the place's own folder for `{{state}}`, or
    /// inside the place (24, 123).
    pub(crate) fn report_path(&self, place: &PlaceRecord, path: &str) -> String {
        match path.strip_prefix("{{state}}") {
            Some(rest) => format!("{}{rest}", self.store.folder(&place.id)),
            None => format!("{}/{path}", place.location),
        }
    }

    /// A repository's id: of its common git directory, or of its
    /// location where git cannot say (33, 147, C.4).
    pub(crate) fn repository_id_of(&self, repository: &str) -> String {
        let common = self
            .history
            .common_dir(repository)
            .unwrap_or_else(|| repository.to_string());
        repository_id(&common)
    }

    /// The identity of the stage's next start, its repository named by
    /// its id (33, C.4).
    pub(crate) fn next_identity(&self, held: &HeldState, index: usize) -> Identity {
        Identity {
            repository: self.repository_id_of(&held.place.repository),
            place: held.place.short(),
            stage: held.place.kind.stages[index].name.clone(),
            start: held.stages[index].starts + 1,
        }
    }

    /// The stage's prompt filled for `identity`'s start, or the first
    /// part it asks for with nothing to fill it (27). The way to stop
    /// short is in every prompt (28), as `agent`, the profile of the
    /// stage's program, gives it; and a first stage started in a place
    /// that could not be made ready carries the step and its words (12).
    /// A repeating stage fills `{{group}}` with `group`, the start's,
    /// which none fills where the change could not be read, and
    /// `{{notes}}` with its notes' path; a stage that does not repeat
    /// fills neither (27, 141).
    pub(crate) fn fill(
        &self,
        held: &HeldState,
        index: usize,
        identity: &Identity,
        words: Option<&str>,
        agent: Option<&dyn Agent>,
        group: Option<Option<&GroupHeld>>,
    ) -> Result<String, PromptPart> {
        let place = &held.place;
        let stage = &place.kind.stages[index];
        let change = self.bound(place).ok().and_then(|bound| bound.change);
        let stop_short = agent.map(|agent| agent.stop_short(identity));
        let fill = |part: PromptPart| -> Option<String> {
            match part {
                PromptPart::Brief => place.brief.clone(),
                PromptPart::Name => Some(place.name.clone()),
                PromptPart::Kind => Some(place.kind.name.clone()),
                PromptPart::Stage => Some(stage.name.clone()),
                PromptPart::Branch => Some(place.branch.clone()),
                PromptPart::Base => Some(place.base.clone()),
                PromptPart::Place => Some(place.location.clone()),
                PromptPart::Change => change.as_ref().map(|change| change.path(&place.name)),
                PromptPart::Report => match stage.deliverables.first() {
                    Some(Deliverable::Report { path }) => Some(self.report_path(place, path)),
                    Some(Deliverable::CommittedReport { path }) => Some(path.clone()),
                    _ => None,
                },
                PromptPart::Words => words.map(str::to_string),
                PromptPart::Unready => Some(unready(&held.readiness)),
                PromptPart::Needs => stop_short.clone(),
                PromptPart::Made => (!stage.makes.is_empty()).then(|| self.made_path(&place.id)),
                PromptPart::Group => group.filter(|_| stage.repeats).map(group_text),
                PromptPart::Notes => stage
                    .repeats
                    .then(|| notes_path(&self.store.folder(&place.id), &stage.name)),
            }
        };
        let mut text = render(&stage.prompt, fill)?;
        if !asks(&stage.prompt, PromptPart::Needs) {
            text.push_str("\n\n");
            text.push_str(&stop_short.clone().ok_or(PromptPart::Needs)?);
        }
        if index == 0
            && held.readiness.failed().is_some()
            && !asks(&stage.prompt, PromptPart::Unready)
        {
            text.push_str("\n\n");
            text.push_str(&unready(&held.readiness));
        }
        Ok(text)
    }

    /// The task group a start of the stage at `index` takes at `head`,
    /// for a repeating stage: the first of the change's groups holding a
    /// task unticked there, or none where every task is ticked or the
    /// change holds none (140). None for a stage that does not repeat,
    /// and for a change that cannot be read at `head`, which gives a
    /// repetition nothing to work from.
    pub(crate) fn group_at(
        &self,
        held: &HeldState,
        index: usize,
        head: &Revision,
    ) -> Option<Option<GroupHeld>> {
        let place = &held.place;
        if !place.kind.stages[index].repeats {
            return None;
        }
        let change = self.bound(place).ok()?.change?;
        let ChangeRead::Read { facts } = change.read_change(&place.repository, head, &place.name)
        else {
            return None;
        };
        Some(facts.first_unticked_group().map(|at| {
            let group = &facts.groups[at];
            GroupHeld {
                index: at,
                number: group.number.clone(),
                title: group.title.clone(),
                tasks: facts.tasks_of(at).map(|task| task.text.clone()).collect(),
            }
        }))
    }

    /// Starts the stage at `index` with one fresh agent and `words` in
    /// its prompt, admitted under the host's room lock: the start is
    /// written to held state as live while there is room and no stage
    /// waiting ranks ahead of it, so two acts at once open one agent and
    /// two settles at once never start past the bound (30, 70, 75), and
    /// the agent is opened once the lock is released; otherwise the stage
    /// waits for room with its words.
    pub(crate) fn start_stage(&self, id: &PlaceId, index: usize, words: Option<String>) -> Start {
        let claimed = {
            let _room = self.host.room();
            let queue = self.queue();
            self.claim(id, index, words, Admit::Queue(&queue))
        };
        match claimed {
            Ok(claim) => self.open(id, claim),
            Err(start) => start,
        }
    }

    /// Claims the stage's start in held state: refused naming the part
    /// its prompt cannot fill, left waiting for room where `admit` says
    /// so, its wait to carry a conversation kept, or written as live;
    /// called under the room lock. A start after its stage's agent stopped
    /// at its account's usage limit, or after its caller's retry of a
    /// conversation reported lost, resumes that agent's conversation where
    /// the stage's profile answers that the account chosen reaches it, its
    /// prompt flume's words, a lost one's retry's words, and the way to
    /// stop short, and its record naming the session; and starts afresh
    /// otherwise, saying why. Such a start, and a lost conversation's
    /// fresh start, waits for room while the profile's live reads a
    /// process running the conversation, or cannot read whether one does,
    /// or while the agent the terminal brought back in the lost start's
    /// pane stands (28, 30, 49, 146, 179, 207).
    pub(crate) fn claim(
        &self,
        id: &PlaceId,
        index: usize,
        words: Option<String>,
        admit: Admit<'_>,
    ) -> Result<Claim, Start> {
        let Some(read) = self.store.read(id) else {
            return Err(Start::NoPlace);
        };
        let place = &read.state.place;
        let queue = admit.queue();
        // A stream whose labels are at fault, or whose multiplexer names a
        // terminal profile the resolver does not carry, starts no stage:
        // its words refuse the start before its settings are resolved (4,
        // 181, 182, 190). Where the settings name no pane and no session,
        // the stream's word for the stage's kind or the stage, read once
        // they are resolved, refuses it where that word is at fault (4,
        // 107, 199).
        let resolution = match (
            queue.stream(&place.stream.name).fault,
            self.stream_terminal(&place.stream.name),
        ) {
            (Some(fault), _) | (None, Err(fault)) => Err(fault),
            (None, Ok(terminal)) => {
                let stage = &place.kind.stages[index];
                self.resolution(place, stage).and_then(|resolution| {
                    let opens_in = queue.session_of(
                        &place.stream.name,
                        &place.kind.name,
                        &stage.name,
                        &resolution.resolved.settings,
                    )?;
                    Ok((resolution, terminal, opens_in))
                })
            }
        };
        let now = self.host.now();
        let affinity = queue
            .pool_of(&place.stream.name)
            .and_then(|_| self.affinity(&place.intent))
            .map(|(_, affinity)| affinity);
        let program = resolution
            .as_ref()
            .ok()
            .and_then(|(resolution, _, _)| resolution.resolved.settings.program.clone());
        let stage_name = &place.kind.stages[index].name;
        // A stage waiting after its agent stopped at its account's usage
        // limit, or after its caller's retry of a conversation reported
        // lost: its profile is asked, under the room lock once an account
        // is chosen, whether that account reaches the agent's conversation
        // (49, 145, 146, 179).
        let after = read.state.stages[index]
            .room
            .as_ref()
            .and_then(|room| room.carry.clone());
        let lost = after
            .as_ref()
            .is_some_and(|after| after.cause == CarryCause::Lost);
        // The lost start, while it is the stage's last: the account it ran
        // under holds its conversation (145, 179).
        let lost_start = read.state.stages[index]
            .start
            .as_ref()
            .filter(|start| {
                lost && after
                    .as_ref()
                    .is_some_and(|after| after.start == start.identity)
            })
            .cloned();
        let (chosen, intent_moved) = match &resolution {
            Ok(_) => self.choose_account(
                place,
                stage_name,
                program.as_deref(),
                affinity.as_ref(),
                lost_start.as_ref().and_then(|start| start.account.as_ref()),
                queue,
                now,
            ),
            Err(_) => (Choice::NoPool, None),
        };
        let env = match &chosen {
            Choice::Chose { account, .. } => self.account_env(account, stage_name),
            _ => Ok(BTreeMap::new()),
        };
        let resolution = match (&chosen, env.as_ref()) {
            (Choice::Undeclared(reason) | Choice::NoAccount(reason), _) | (_, Err(reason)) => {
                Err(reason.clone())
            }
            _ => resolution,
        };
        let env = env.unwrap_or_default();
        // A lost conversation's start under no pool asks the profile of
        // the login environment's, an empty one; a spent agent always ran
        // under an account (146, 179).
        let carry = match (&after, &chosen, &resolution) {
            (Some(after), Choice::Chose { .. }, Ok((resolution, _, _))) if after.carry => Some(
                resolution
                    .agent
                    .carry(&self.store.folder(id), &after.start, &env),
            ),
            (Some(after), Choice::NoPool, Ok((resolution, _, _))) if after.carry && lost => Some(
                resolution
                    .agent
                    .carry(&self.store.folder(id), &after.start, &BTreeMap::new()),
            ),
            _ => None,
        };
        let conversation = after
            .as_ref()
            .map(|after| conversation(after, carry.as_ref()));
        let resume = match carry {
            Some(Carry::Yes { session }) => Some(session),
            _ => None,
        };
        // A start that carries a conversation, and a lost conversation's
        // fresh start, opens only while no process runs that conversation:
        // live is asked of every home the host declares for the program,
        // and the stage waits while one runs it or live cannot read whether
        // one does, flume adopting, ending and sending nothing to it (30,
        // 49, 207; core.10).
        let ask = matches!(
            (&after, &chosen, &resolution),
            (Some(_), Choice::Chose { .. } | Choice::NoPool, Ok(_))
        ) && (resume.is_some() || lost);
        let (session, standing) = match &after {
            Some(after) => self.session_holds(
                id,
                after,
                lost_start.as_ref(),
                program.as_deref().unwrap_or_default(),
                queue,
                ask,
            ),
            None => (None, None),
        };
        let lost_identity = lost_start.as_ref().map(|start| start.identity.clone());
        let fresh = if lost {
            Some(then_words(StartReason::Lost.words(), words.as_deref()))
        } else {
            words.clone()
        };
        let made_from = self.store.made(id).len();
        let label = self.label_of(place);
        let claimed = self.update(id, |held| {
            if let Admit::Waiting(_) = admit {
                if held.phase != Phase::Live
                    || held.stages[index].state != StageState::WaitingForRoom
                {
                    return Step::Keep(Err(Start::NoPlace));
                }
            }
            if let Some(lost) = lost_identity.as_ref().filter(|_| standing == Some(false)) {
                if let Some(start) = held
                    .index_of(lost)
                    .and_then(|at| held.stages[at].start.as_mut())
                {
                    start.live = false;
                }
            }
            match (&session, admit) {
                (Some(session), Admit::Waiting(_)) => {
                    let room = held.stages[index]
                        .room
                        .get_or_insert_with(RoomWait::default);
                    session.hold(room);
                    return Step::Write(Err(Start::WaitingForRoom));
                }
                (Some(session), Admit::Queue(queue)) => {
                    let stage = &mut held.stages[index];
                    stage.state = StageState::WaitingForRoom;
                    let mut room = RoomWait {
                        order: queue.next_order(),
                        words: words.clone(),
                        carry: after.clone(),
                        ..RoomWait::default()
                    };
                    session.hold(&mut room);
                    stage.room = Some(room);
                    held.current = Some(index);
                    return Step::Write(Err(Start::WaitingForRoom));
                }
                (None, _) => {
                    // Read no longer running: the wait holds no process.
                    if let Some(room) = held.stages[index].room.as_mut() {
                        LiveRead::default().hold(room);
                    }
                }
            }
            if held
                .stages
                .iter()
                .any(|stage| stage.start.as_ref().is_some_and(|start| start.live))
            {
                return Step::Write(Err(Start::AgentLive));
            }
            let (resolution, (terminal, named), opens_in) = match &resolution {
                Ok((resolution, terminal, opens_in)) => (resolution, terminal, opens_in),
                Err(reason) => {
                    let stage = &mut held.stages[index];
                    stage.state = StageState::Refused;
                    stage.refused_part = None;
                    stage.refused_setting = Some(reason.clone());
                    stage.room = None;
                    held.current = Some(index);
                    return Step::Write(Err(Start::Unresolved(reason.clone())));
                }
            };
            let head = self
                .history
                .head(&held.place.repository, &held.place.branch)
                .unwrap_or_else(|| held.place.base_revision.clone());
            let group = self.group_at(held, index, &head);
            let identity = self.next_identity(held, index);
            let agent = resolution.agent.as_ref();
            // A start that carries its conversation is sent flume's words,
            // a lost conversation's retry's words after them, and the way
            // to stop short, nothing of the stage's prompt filled; a lost
            // conversation's fresh start is given flume's words before the
            // retry's as its words (27, 28, 49, 146). A lost carried
            // start keeps the retry's words, so a fresh start after it
            // that did not begin is given them again (S79).
            let (text, started_with) = match &resume {
                Some(_) if lost => {
                    let resumed = then_words(StartReason::Resumed.words(), words.as_deref());
                    let text = format!("{resumed}\n\n{}", agent.stop_short(&identity));
                    (text, words.clone())
                }
                Some(_) => {
                    let words = StartReason::Carried.words();
                    let text = format!("{words}\n\n{}", agent.stop_short(&identity));
                    (text, Some(words))
                }
                None => match self.fill(
                    held,
                    index,
                    &identity,
                    fresh.as_deref(),
                    Some(agent),
                    group.as_ref().map(Option::as_ref),
                ) {
                    Err(part) => {
                        let stage = &mut held.stages[index];
                        stage.state = StageState::Refused;
                        stage.refused_part = Some(part.name().into());
                        stage.refused_setting = None;
                        stage.room = None;
                        return Step::Write(Err(Start::Refused(part)));
                    }
                    Ok(text) => (text, fresh.clone()),
                },
            };
            if let (Admit::Waiting(_), Choice::Wait) = (admit, &chosen) {
                return Step::Write(Err(Start::WaitingForRoom));
            }
            if let Admit::Queue(queue) = admit {
                let program = resolution.resolved.settings.program.as_deref();
                if matches!(chosen, Choice::Wait)
                    || queue.must_wait(
                        id,
                        &held.place.stream.name,
                        label.as_ref().map(|label| label.name.as_str()),
                        program,
                        held.priority,
                    )
                {
                    let stage = &mut held.stages[index];
                    stage.state = StageState::WaitingForRoom;
                    stage.room = Some(RoomWait {
                        order: queue.next_order(),
                        words: words.clone(),
                        carry: after.clone(),
                        ..RoomWait::default()
                    });
                    held.current = Some(index);
                    return Step::Write(Err(Start::WaitingForRoom));
                }
            }
            let account = match &chosen {
                Choice::Chose { held, .. } => Some(held.clone()),
                _ => None,
            };
            let carried = conversation
                .as_ref()
                .map(|conversation| conversation.carried);
            if let Some(moved) =
                self.stage_moved(held, index, account.as_ref(), carried, queue, now)
            {
                held.moves.push(moved);
            }
            if let Some(moved) = &intent_moved {
                held.moves.push(moved.clone());
            }
            let mut from = resolution.resolved.from.clone();
            if let Choice::Chose { account, .. } = &chosen {
                for (name, value) in &account.env {
                    from.insert(
                        format!("env.{name}"),
                        Given {
                            layer: Layer::Account,
                            value: value.clone(),
                            file: None,
                        },
                    );
                }
            }
            let secret = self.host.secret();
            let program = resolution
                .resolved
                .settings
                .program
                .clone()
                .unwrap_or_default();
            let withheld = self
                .agent(&program)
                .map(|agent| agent.withheld())
                .unwrap_or_default();
            let start_record = StartRecord {
                mode: resolution.mode.clone(),
                session: resume.clone(),
                ..start_record(
                    &program,
                    account.as_ref(),
                    &env,
                    &withheld,
                    &identity,
                    &held.place.location,
                    &secret,
                )
            };
            let stage = &mut held.stages[index];
            stage.state = StageState::Starting;
            stage.starts = identity.start;
            stage.refused_part = None;
            stage.refused_setting = None;
            stage.room = None;
            stage.end = None;
            stage.start = Some(StartHeld {
                n: identity.start,
                started_at: now,
                start_head: head.clone(),
                made_from,
                words: started_with,
                identity: identity.clone(),
                secret: secret.clone(),
                program,
                settings: resolution.resolved.settings.clone(),
                // Names only: the values reach the agent's command and
                // nothing kept (32, 56).
                from: flume_seams::unvalued_from(&from),
                account: account.clone(),
                start_record,
                record: None,
                prompt: Prompt {
                    text: text.clone(),
                    ..Prompt::default()
                },
                live: true,
                begun_at: None,
                first_quiet: None,
                last_head: head,
                last_head_move: now,
                past_limit: false,
                printed: None,
                group: group.clone().flatten(),
                opening: Some(self.host.command()),
                dialog: None,
                conversation: conversation.clone(),
                session: opens_in.clone(),
                label: label.clone(),
                terminal: Some(named.clone()),
                own: resolution.resolved.own.as_ref().map(|own| OwnHeld {
                    files: own.files.clone(),
                    passed: None,
                }),
            });
            if !held
                .terminals
                .iter()
                .any(|opened| opened.profile == named.profile)
            {
                held.terminals.push(named.clone());
            }
            held.current = Some(index);
            Step::Write(Ok(Claim {
                identity,
                secret,
                text,
                place: held.place.place_ref(),
                settings: resolution.resolved.settings.clone(),
                env: env.clone(),
                agent: resolution.agent.clone(),
                session: held.role.clone(),
                terminal: terminal.clone(),
                opens_in: opens_in.clone(),
                account,
                mode: resolution.mode.clone(),
                resume: resume.clone(),
                own: resolution.own_keys(),
            }))
        });
        let claimed = claimed.unwrap_or(Err(Start::NoPlace));
        if let (Ok(claim), Some(place), Some(program)) = (&claimed, self.store.read(id), &program) {
            if let Some(account) = &claim.account {
                self.affinity_after(&place.state.place, program, account, intent_moved.as_ref());
            }
        }
        claimed
    }

    /// Opens the agent of a claimed start, out of the room lock; an open
    /// the terminal refuses ends the start, freeing its room (70). Once
    /// the open answers, the start is no longer opening, and an agent that
    /// came up is sent its prompt at once where it said it started and
    /// waits, its receipt read by the settles that follow, so the command
    /// that opened it, a launch, a retry or a settle, moves it on with no
    /// wait on the agent (6, 37, 38, 46). A start held at its program's
    /// question stays starting, the question's words kept with it, and
    /// waits on a person (34, 35).
    pub(crate) fn open(&self, id: &PlaceId, claim: Claim) -> Start {
        let Claim {
            identity,
            secret,
            text,
            place,
            settings,
            env,
            agent,
            session,
            terminal,
            opens_in,
            account: _,
            mode,
            resume,
            own,
        } = claim;
        let folder = self.store.folder(id);
        let command = agent.command(
            &folder,
            &settings,
            mode.as_deref(),
            &own,
            resume.as_deref(),
            &env,
            &place,
            &identity,
            &secret,
            &text,
        );
        if let Err(failure) = self.write_files(id, &folder, &command) {
            self.with_start(id, &identity, |start, _| start.opening = None);
            self.not_opened(
                id,
                &identity,
                format!("{identity} was not opened: {failure}"),
            );
            return Start::NotOpened(failure);
        }
        let passed = passed(&command);
        let at = match (&settings.beside, opens_in) {
            (Some(pane), _) => OpenIn::Beside { pane: pane.clone() },
            (
                None,
                Some(SessionHeld {
                    name: Some(name), ..
                }),
            ) => OpenIn::Session { name },
            (None, _) => OpenIn::Own,
        };
        let opened = terminal.open(
            &place,
            &folder,
            &command,
            &identity,
            &at,
            session.as_deref(),
        );
        self.with_start(id, &identity, |start, _| {
            start.opening = None;
            if let Some(own) = start.own.as_mut() {
                own.passed.clone_from(&passed);
            }
            if let Opened::Opened { carried, printed } = &opened {
                start.prompt.carried = *carried;
                start.printed.clone_from(printed);
            }
        });
        match opened {
            Opened::Opened { .. } => {
                self.send_prompt(id);
                Start::Opened(identity)
            }
            Opened::Held { screen } => {
                let since = self.host.now();
                self.with_start(id, &identity, |start, _| {
                    start.dialog = Some(Dialog {
                        words: screen.clone(),
                        since,
                    });
                });
                Start::Held(identity)
            }
            Opened::NotComeUp { screen } => {
                self.never_begun_with(id, &identity, screen);
                Start::NeverBegun(identity)
            }
            Opened::Refused { reason } => {
                let failure = format!("the terminal did not open {identity}: {reason}");
                self.not_opened(id, &identity, failure);
                Start::NotOpened(reason)
            }
        }
    }

    /// The start under `identity` ended never begun, its room freed, and
    /// `failure` kept with the place (35, 70).
    fn not_opened(&self, id: &PlaceId, identity: &Identity, failure: String) {
        self.update(id, |held| {
            if let Some(index) = held.index_of(identity) {
                let stage = &mut held.stages[index];
                stage.state = StageState::NeverBegun;
                if let Some(start) = stage.start.as_mut() {
                    start.live = false;
                }
                held.failures.push(failure.clone());
            }
            Step::Write(())
        });
    }

    /// Writes each file `command` lists into the place's own `folder`
    /// through the store, written beside and renamed over, readable by
    /// the host's user alone; or, writing nothing, why not: a file
    /// outside the folder, named, or the store's words (196; core.16,
    /// core.17).
    fn write_files(
        &self,
        id: &PlaceId,
        folder: &str,
        command: &AgentCommand,
    ) -> Result<(), String> {
        if let Some(outside) = command.files.keys().find(|path| !under(folder, path)) {
            return Err(format!(
                "its command lists the file {outside}, which lies outside the place's folder {folder}"
            ));
        }
        for (path, text) in &command.files {
            self.store.write_file(id, path, text)?;
        }
        Ok(())
    }

    /// Changes the start under `identity`, if it is still the stage's,
    /// and writes `end` as its stage's end in the same write, so the end
    /// is recorded whole with what was read of it (43).
    pub(crate) fn end_start(
        &self,
        id: &PlaceId,
        identity: &Identity,
        mut change: impl FnMut(&mut StartHeld, &mut StageState),
        end: Option<StageEnd>,
    ) {
        self.update(id, |held| {
            if let Some(index) = held.index_of(identity) {
                let stage = &mut held.stages[index];
                if let Some(start) = stage.start.as_mut() {
                    change(start, &mut stage.state);
                }
                if let Some(end) = &end {
                    stage.end = Some(end.clone());
                }
            }
            Step::Write(())
        });
    }

    /// Changes the start under `identity`, if it is still the stage's.
    pub(crate) fn with_start(
        &self,
        id: &PlaceId,
        identity: &Identity,
        mut change: impl FnMut(&mut StartHeld, &mut StageState),
    ) {
        self.update(id, |held| {
            if let Some(index) = held.index_of(identity) {
                let stage = &mut held.stages[index];
                if let Some(start) = stage.start.as_mut() {
                    change(start, &mut stage.state);
                }
            }
            Step::Write(())
        });
    }
}

/// Whether `path` is a file under `folder`, by its components, none of
/// them `..` (196).
fn under(folder: &str, path: &str) -> bool {
    let path = std::path::Path::new(path);
    path.starts_with(folder)
        && path != std::path::Path::new(folder)
        && !path
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
}

/// How `command` passed its program's own settings: each of its
/// arguments naming a file it lists, with the argument before it; none
/// where it names none (195, 196; design 12).
fn passed(command: &AgentCommand) -> Option<String> {
    let words: Vec<String> = command
        .args
        .iter()
        .enumerate()
        .filter(|(_, arg)| command.files.contains_key(*arg))
        .map(
            |(at, file)| match at.checked_sub(1).map(|before| &command.args[before]) {
                Some(flag) => format!("{flag} {file}"),
                None => file.clone(),
            },
        )
        .collect();
    (!words.is_empty()).then(|| words.join(" "))
}

/// Finding a start in held state by its identity.
pub(crate) trait Starts {
    fn index_of(&self, identity: &Identity) -> Option<usize>;
    fn live_start(&self) -> Option<(usize, &StartHeld)>;
}

impl Starts for HeldState {
    fn index_of(&self, identity: &Identity) -> Option<usize> {
        self.stages.iter().position(|stage| {
            stage
                .start
                .as_ref()
                .is_some_and(|start| &start.identity == identity)
        })
    }

    fn live_start(&self) -> Option<(usize, &StartHeld)> {
        self.stages.iter().enumerate().find_map(|(index, stage)| {
            stage
                .start
                .as_ref()
                .filter(|start| start.live)
                .map(|start| (index, start))
        })
    }
}
