//! The merge-queue profile over sluice (C.6; design 13), every command
//! and field it reads checked against the sluice the tree pins, 0.67.0.
//!
//! submit is `sluice submit <base> --no-wait --json` in the place, with
//! every multiplexer's session stripped from its environment, so sluice
//! records the place's worktree as the submitter, pushes into no pane and
//! answers at once; never `--check`, never `wt submit` (111, 113, 117,
//! core.6). Where sluice finds the submission decided as it takes it in,
//! as on a base that takes pull requests, it answers with the entry's
//! record, exiting as the outcome does, and records the outcome delivered
//! to the place, so the outcome is read from that answer as the inbox
//! reads one, never as a refusal (112, 133). On such a base sluice pushes
//! the place's branch and opens or updates its pull request, and where
//! intake has not taken it in answers `{pull_request, branch, state}`
//! with no entry: the request is held, open and undecided, and an intake
//! on whichever host takes the base's requests in takes it in later; a
//! request answered `closed` was closed without landing (124, C.6). An
//! entry intake took in names its place `pull/<n>` as its `from.worktree`
//! and the branch the request was opened from as its `from.head`, and
//! sluice delivers its outcome to the request, so read-submission reads
//! it, and the inbox only where it is still pending for the place (133).
//! inbox is `sluice inbox
//! --json` in the place: each record landed with its commit, failed or in
//! conflict with sluice's `notice` as its words, a conflict's files from
//! the record's `conflict.paths` and its base revision from its
//! `tested.on`, never parsed from the notice; a record with no notice
//! says its state, and a conflict's files and the entries its
//! `conflict.with` names; one found in conflict before any test names the
//! base's current revision (112, 133). Where sluice could not read a
//! branch's records it answers the others' outcomes with that branch in
//! its `unread`, exiting as a failure: the inbox passes that branch over
//! with its words, whose outcomes sluice leaves pending, and a decided
//! record the profile cannot read is passed over the same way, by its
//! branch, rather than dropped unsaid (133). read-submission reads
//! `sluice status <base> --all --json` for each branch `sluice overview
//! --json` names, the newest entry submitted from the branch, by a
//! worktree on it or from a pull request opened from it, a conflict's
//! base revision its `tested.on` where it holds one; a branch whose
//! status sluice refuses, as it refuses one whose policy it no longer
//! reads, is passed over with sluice's words, and every other branch is
//! read still (99, 101). step is
//! `sluice step <base>`, which takes in the base's pull requests before
//! its round, and beside a live `sluice run` finds the branch's lock held
//! and does nothing, so no liveness is read (124, 125). The base lives at `<remote>/<base>` after one fetch where
//! `.config/sluice.toml` lands it on a remote, and at the local base
//! otherwise (9, 126, C.1). A place is made ready by sluice's equipping
//! of it, `sluice equip <base> --at <revision> --json` run in the place,
//! the revision the place's launch read, so a retry equips it as its
//! launch did: sluice reads `[tests]` of the base's policy at that
//! revision and runs in the place what it runs before its own runs, the
//! toolchain, then its runner's install of what the place pins, its check
//! of its own version and its setup, and answers each step with its
//! outcome and words, which [`equipped`] reads as the readiness's steps
//! (11, 12, 20, 79, C.1, C.6). The profile reads no `[tests]` key and no
//! `.prototools` for a place: the readiness, `sluice release` as the
//! teardown and the remote the policy lands the base on are declared for
//! the resolver to hand the place profile (core.3).

use crate::git::Git;
use crate::shell;
use flume_seams::{
    BaseRevision, Inbox, MergeQueue, PassedOver, PlaceRef, ReadAcross, ReadyStep, Revision,
    Stepped, Submission, SubmissionOutcome, SubmissionRead, SubmissionState, Submitted,
};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The profile's name in a repository's binding, `merge_queue` (105).
pub const NAME: &str = "sluice";

/// Where a repository's sluice policy lives in its tree.
pub const POLICY: &str = ".config/sluice.toml";

/// The variables a multiplexer's session is named by: a submission made
/// with one would be recorded as that session's and its outcome pushed
/// into its pane (C.6, core.6).
fn a_session(name: &str) -> bool {
    name.starts_with("HERDR_")
        || name.starts_with("ZELLIJ")
        || name == "TMUX"
        || name == "TMUX_PANE"
}

/// The merge queue through sluice.
#[derive(Clone, Debug)]
pub struct Sluice {
    program: PathBuf,
    env: Vec<(String, Option<String>)>,
}

impl Default for Sluice {
    fn default() -> Self {
        Sluice::new()
    }
}

impl Sluice {
    /// sluice as `PATH` finds it.
    pub fn new() -> Sluice {
        Sluice {
            program: "sluice".into(),
            env: vec![],
        }
    }

    /// sluice run with `name` set to `value`, or left out where none,
    /// over the command's own environment.
    pub fn with_env(mut self, name: &str, value: Option<&str>) -> Sluice {
        self.env.push((name.to_string(), value.map(str::to_string)));
        self
    }

    /// sluice run in `dir` with `args`: whether it succeeded, its output
    /// and its errors, or why it could not run.
    fn run(&self, dir: &Path, args: &[&str]) -> Result<(bool, String, String), String> {
        let mut command = Command::new(&self.program);
        command
            .args(args)
            .current_dir(dir)
            .stdin(Stdio::null())
            .env("GIT_TERMINAL_PROMPT", "0");
        for (name, value) in &self.env {
            match value {
                Some(value) => command.env(name, value),
                None => command.env_remove(name),
            };
        }
        for (name, _) in std::env::vars_os() {
            if a_session(&name.to_string_lossy()) {
                command.env_remove(&name);
            }
        }
        for (name, _) in &self.env {
            if a_session(name) {
                command.env_remove(name);
            }
        }
        let output = command
            .output()
            .map_err(|error| format!("sluice could not be run: {error}"))?;
        Ok((
            output.status.success(),
            String::from_utf8_lossy(&output.stdout).into_owned(),
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }

    /// sluice's JSON answer, or its words where it refused or could not
    /// be read.
    fn json(&self, dir: &Path, args: &[&str]) -> Result<Value, String> {
        let (success, stdout, stderr) = self.run(dir, args)?;
        let answer: Option<Value> = serde_json::from_str(stdout.trim()).ok();
        match answer {
            Some(answer) if success && answer.get("error").is_none() => Ok(answer),
            _ => Err(words(args, &stdout, &stderr)),
        }
    }

    /// A decided record's outcome: landed with its commit, failed with
    /// its words, or in conflict with the files contested, its words and
    /// the base's revision it was tested on, or where it was found in
    /// conflict before any test, the base's current revision where it
    /// lives, else its tracking copy's (112, 133).
    fn outcome(&self, record: &Value, place: &PlaceRef) -> Result<SubmissionOutcome, String> {
        match state(record)? {
            SubmissionState::Landed { commit } => Ok(SubmissionOutcome::Landed { commit }),
            SubmissionState::Failed { words } => Ok(SubmissionOutcome::Failed { words }),
            SubmissionState::Conflict {
                files,
                base: tested,
                words,
            } => {
                let base = match tested {
                    Some(base) => base,
                    None => match self.base_revision(&place.repository, &place.base) {
                        BaseRevision::At { revision } => revision,
                        BaseRevision::Refused { reason } => base_at(&place.repository, &place.base)
                            .ok_or_else(|| {
                                format!("{words}, and the base's revision is unread: {reason}")
                            })?,
                    },
                };
                Ok(SubmissionOutcome::Conflict { files, base, words })
            }
            other => Err(format!("sluice gave a submission undecided: {other:?}")),
        }
    }

    /// Each branch sluice keeps a queue for in the repository.
    fn branches(&self, dir: &Path) -> Result<Vec<String>, String> {
        let overview = self.json(dir, &["overview", "--json"])?;
        Ok(overview
            .get("branches")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|branch| branch.get("branch").and_then(Value::as_str))
            .map(str::to_string)
            .collect())
    }
}

/// sluice's own words for a refusal, naming the command refused.
fn words(args: &[&str], stdout: &str, stderr: &str) -> String {
    format!(
        "sluice {}: {}",
        args.first().copied().unwrap_or_default(),
        refusal(stdout, stderr)
    )
}

/// What sluice said of a refusal: its JSON error's message where it gave
/// one, its errors otherwise, and what it printed where it gave neither.
fn refusal(stdout: &str, stderr: &str) -> String {
    let message = serde_json::from_str::<Value>(stdout.trim())
        .ok()
        .and_then(|answer| {
            answer
                .get("error")?
                .get("message")?
                .as_str()
                .map(str::to_string)
        });
    message.unwrap_or_else(|| {
        let stderr = stderr.trim();
        if stderr.is_empty() {
            stdout.trim().to_string()
        } else {
            stderr.to_string()
        }
    })
}

fn text(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_string)
}

/// A record's id as flume names a submission.
fn id(record: &Value) -> Option<String> {
    match record.get("id")? {
        Value::Number(number) => Some(number.to_string()),
        Value::String(id) => Some(id.clone()),
        _ => None,
    }
}

/// The pull request an answer or a record names: a submit's
/// `pull_request`, or an entry's submitter `pull/<n>`, as sluice names
/// the place of an entry intake took in from a request.
fn request(record: &Value) -> Option<String> {
    if let Some(number) = record.get("pull_request").and_then(Value::as_u64) {
        return Some(number.to_string());
    }
    let from = record.get("from").and_then(|from| text(from, "worktree"))?;
    let number = from.strip_prefix("pull/")?;
    number.parse::<u64>().is_ok().then(|| number.to_string())
}

/// Whether a record was submitted from `branch`: by a worktree on it, or
/// by intake from a pull request opened from it, which sluice records as
/// `from.head` (99).
fn submitted_from(record: &Value, branch: &str) -> bool {
    let from = record.get("from");
    let by = |key| from.and_then(|from| text(from, key));
    by("worktree").as_deref() == Some(branch) || by("head").as_deref() == Some(branch)
}

/// Each branch an answer of sluice's names in its `unread`, as `sluice
/// inbox --json` names a branch whose records it could not read, with
/// sluice's words.
fn unread(answer: &Value) -> Vec<PassedOver> {
    answer
        .get("unread")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|unread| {
            Some(PassedOver {
                branch: text(unread, "branch")?,
                words: format!("sluice inbox: {}", text(unread, "error")?),
            })
        })
        .collect()
}

/// The files a conflict record contests.
fn contested(record: &Value) -> Vec<String> {
    record
        .get("conflict")
        .and_then(|conflict| conflict.get("paths"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

/// The entries a conflict record conflicts with, each by its id and
/// what submitted it, as sluice's forecast names them.
fn contested_with(record: &Value) -> Vec<String> {
    record
        .get("conflict")
        .and_then(|conflict| conflict.get("with"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let from = entry.get("from");
            let by = |key| from.and_then(|from| text(from, key)).unwrap_or_default();
            Some(format!(
                "#{} ({}, {})",
                id(entry)?,
                by("agent"),
                by("worktree")
            ))
        })
        .collect()
}

/// A decided record's words: sluice's notice, or where a record holds
/// none, its state and the checks that failed, or the files contested
/// and the entries it conflicts with.
fn said(record: &Value) -> String {
    text(record, "notice").unwrap_or_else(|| {
        let state = text(record, "state").unwrap_or_default();
        let named = match state.as_str() {
            "conflict" => {
                let with = contested_with(record);
                let files = contested(record).join(", ");
                match with.is_empty() {
                    true => files,
                    false => format!("{files} with {}", with.join(", ")),
                }
            }
            _ => record
                .get("failed")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|check| text(check, "task"))
                .collect::<Vec<String>>()
                .join(", "),
        };
        let branch = text(record, "branch").unwrap_or_default();
        match named.is_empty() {
            true => format!("{state} on {branch}"),
            false => format!("{state} on {branch}: {named}"),
        }
    })
}

/// A record's state as the seam names it, or the state sluice gave that
/// the seam has no name for.
fn state(record: &Value) -> Result<SubmissionState, String> {
    let name = text(record, "state").unwrap_or_default();
    Ok(match name.as_str() {
        "queued" => SubmissionState::Waiting,
        "testing" => SubmissionState::UnderTest,
        "landed" => SubmissionState::Landed {
            commit: Revision::new(text(record, "landed_as").ok_or("a landing names no commit")?),
        },
        "failed" => SubmissionState::Failed {
            words: said(record),
        },
        "conflict" => SubmissionState::Conflict {
            files: contested(record),
            base: record
                .get("tested")
                .and_then(|tested| text(tested, "on"))
                .map(Revision::new),
            words: said(record),
        },
        "withdrawn" => SubmissionState::Withdrawn,
        other => return Err(format!("sluice gave a submission in the state {other:?}")),
    })
}

/// A branch's settings in the policy: its own table, or the first pattern
/// with one `*` that names it.
fn branch_table<'a>(policy: &'a toml::Table, base: &str) -> Option<&'a toml::Table> {
    let branches = policy.get("branches")?.as_table()?;
    if let Some(table) = branches.get(base).and_then(toml::Value::as_table) {
        return Some(table);
    }
    branches.iter().find_map(|(pattern, table)| {
        let (before, after) = pattern.split_once('*')?;
        (base.len() >= before.len() + after.len()
            && base.starts_with(before)
            && base.ends_with(after))
        .then(|| table.as_table())
        .flatten()
    })
}

/// The policy at the base's revision: the local base's, else a remote's
/// tracking copy's; none where neither holds one.
fn policy(repository: &str, base: &str) -> Result<Option<toml::Table>, String> {
    let git = Git::at(repository);
    let mut revisions = vec![format!("refs/heads/{base}")];
    revisions.extend(
        git.remotes()?
            .into_iter()
            .map(|remote| format!("refs/remotes/{remote}/{base}")),
    );
    for revision in revisions {
        if git.commit(&revision).is_err() {
            continue;
        }
        if let Some(text) = git.file(&revision, POLICY)? {
            return text
                .parse::<toml::Table>()
                .map(Some)
                .map_err(|error| format!("{POLICY} at {revision}: {error}"));
        }
    }
    Ok(None)
}

/// The remote the policy lands `base` on, none where it lands locally.
pub fn base_remote(repository: &str, base: &str) -> Result<Option<String>, String> {
    Ok(policy(repository, base)?.and_then(|policy| {
        branch_table(&policy, base)?
            .get("remote")?
            .as_str()
            .map(str::to_string)
    }))
}

/// The step a place's readiness names where sluice's equip answered no
/// step it could read.
pub const EQUIP: &str = "sluice equip";

/// What sluice says of a place, for the resolver to hand the place
/// profile, which no profile calls (core.2): its readiness, sluice's
/// equipping of it, whose answer [`equipped`] reads; `sluice release`, its
/// teardown; and the remote the base lands on (C.1, C.6, core.3).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ForPlace {
    /// The readiness, as a shell line: `sluice equip <base> --at
    /// <revision> --json`, the revision the place's launch read; none
    /// where no revision was given, as for a teardown.
    pub equip: Option<String>,
    /// The teardown, as a shell line.
    pub teardown: String,
    pub base_remote: Option<String>,
}

/// What sluice says of a place on `base` in `repository`, its readiness
/// asked at `revision`, the revision its launch read, where one is given
/// (12, 20). Only where the base lives is read of the policy here: what
/// equips the place is sluice's to read, at that revision.
pub fn for_place(
    repository: &str,
    base: &str,
    revision: Option<&Revision>,
) -> Result<ForPlace, String> {
    Ok(ForPlace {
        equip: revision
            .map(|revision| shell::words(["sluice", "equip", base, "--at", &revision.0, "--json"])),
        teardown: "sluice release".into(),
        base_remote: base_remote(repository, base)?,
    })
}

/// The readiness's steps from what `sluice equip --json` printed, whether
/// it passed, on its output and on its errors: each of its `steps[]`, in
/// order, with its outcome and its words cut as any step's are. Where it
/// answered no step it could read, a refusal, an answer that is not its
/// form or a sluice that could not run, they are one failed step,
/// [`EQUIP`], with sluice's message, else its errors, else what it
/// printed; where it failed with every step it answered passed, that step
/// follows them (12, 79, C.6).
pub fn equipped(passed: bool, printed: &str, errors: &str) -> Vec<ReadyStep> {
    let answer = serde_json::from_str::<Value>(printed.trim()).ok();
    let steps = answer
        .as_ref()
        .filter(|answer| answer.get("error").is_none())
        .and_then(|answer| answer.get("steps")?.as_array())
        .and_then(|steps| {
            steps
                .iter()
                .map(|step| {
                    Some(ReadyStep {
                        step: text(step, "step")?,
                        passed: step.get("passed")?.as_bool()?,
                        words: shell::tail(step.get("words")?.as_str()?),
                    })
                })
                .collect::<Option<Vec<ReadyStep>>>()
        });
    let refused = || ReadyStep {
        step: EQUIP.into(),
        passed: false,
        words: shell::tail(&refusal(printed, errors)),
    };
    match steps {
        Some(steps) if passed || steps.iter().any(|step| !step.passed) => steps,
        Some(mut steps) => {
            steps.push(refused());
            steps
        }
        None => vec![refused()],
    }
}

/// The base's revision where its tracking copy or the local branch holds
/// it, with no fetch.
fn base_at(repository: &str, base: &str) -> Option<Revision> {
    let git = Git::at(repository);
    let at = match base_remote(repository, base).ok()? {
        Some(remote) => format!("refs/remotes/{remote}/{base}"),
        None => format!("refs/heads/{base}"),
    };
    git.commit(&at).ok().map(Revision::new)
}

impl MergeQueue for Sluice {
    fn submit(&self, place: &PlaceRef, _head: &Revision) -> Submitted {
        let args = ["submit", place.base.as_str(), "--no-wait", "--json"];
        let (success, stdout, stderr) = match self.run(Path::new(&place.location), &args) {
            Ok(output) => output,
            Err(reason) => return Submitted::Refused { reason },
        };
        // A submission sluice found decided as it took it in is answered
        // with its record, exiting as the outcome does, and delivered.
        let entry = serde_json::from_str::<Value>(stdout.trim())
            .ok()
            .filter(|entry| entry.get("error").is_none())
            .filter(|entry| success || entry.get("state").is_some());
        let Some(entry) = entry else {
            return Submitted::Refused {
                reason: words(&args, &stdout, &stderr),
            };
        };
        // On a base that takes pull requests, sluice answers the request
        // it opened or updated and no entry where intake has not taken it
        // in, which a later intake does; and a request closed without
        // landing, exiting as a failure does (124, C.6).
        let Some(id) = id(&entry) else {
            return match (request(&entry), success) {
                (Some(request), true) => Submitted::Requested { request },
                (Some(request), false) => Submitted::Refused {
                    reason: format!(
                        "sluice submit: pull request #{request} is {}",
                        text(&entry, "state").unwrap_or_default()
                    ),
                },
                (None, _) => Submitted::Refused {
                    reason: format!("sluice submit answered no entry: {entry}"),
                },
            };
        };
        let state = match entry.get("state") {
            Some(_) => match state(&entry) {
                Ok(state) => state,
                Err(reason) => return Submitted::Refused { reason },
            },
            None => SubmissionState::Waiting,
        };
        let submission = Submission {
            id,
            branch: place.branch.clone(),
            target: place.base.clone(),
            state,
            request: request(&entry),
        };
        if !matches!(
            submission.state,
            SubmissionState::Landed { .. }
                | SubmissionState::Failed { .. }
                | SubmissionState::Conflict { .. }
        ) {
            return Submitted::Submitted { submission };
        }
        match self.outcome(&entry, place) {
            Ok(outcome) => Submitted::Decided {
                submission,
                outcome,
            },
            Err(reason) => Submitted::Refused { reason },
        }
    }

    fn inbox(&self, place: &PlaceRef) -> Inbox {
        let args = ["inbox", "--json"];
        let (success, stdout, stderr) = match self.run(Path::new(&place.location), &args) {
            Ok(output) => output,
            Err(reason) => return Inbox::CouldNotRead { reason },
        };
        let answer = serde_json::from_str::<Value>(stdout.trim()).ok();
        let records = answer
            .as_ref()
            .and_then(|answer| answer.get("outcomes").and_then(Value::as_array).cloned());
        let mut passed_over = answer.as_ref().map(unread).unwrap_or_default();
        // sluice exits as a failure where it passed a branch over, having
        // answered every other branch's outcomes and marked them delivered.
        let Some(records) =
            records.filter(|records| success || !records.is_empty() || !passed_over.is_empty())
        else {
            return Inbox::CouldNotRead {
                reason: words(&args, &stdout, &stderr),
            };
        };
        let mut outcomes = vec![];
        for record in records.iter().filter(|record| {
            matches!(
                text(record, "state").as_deref(),
                Some("landed" | "failed" | "conflict")
            )
        }) {
            match self.outcome(record, place) {
                Ok(outcome) => outcomes.push(outcome),
                Err(reason) => passed_over.push(PassedOver {
                    branch: text(record, "branch").unwrap_or_default(),
                    words: match id(record) {
                        Some(id) => format!("#{id}: {reason}"),
                        None => reason,
                    },
                }),
            }
        }
        Inbox::Outcomes {
            outcomes,
            passed_over,
        }
    }

    fn read_submission(&self, repository: &str, branch: &str) -> ReadAcross {
        let git = Git::at(repository);
        let dir = git.path();
        let bases = match self.branches(dir) {
            Ok(bases) => bases,
            Err(reason) => return SubmissionRead::CouldNotRead { reason }.into(),
        };
        let mut newest: Option<(u64, Value)> = None;
        let mut passed_over = vec![];
        for base in bases {
            let status = match self.json(dir, &["status", &base, "--all", "--json"]) {
                Ok(status) => status,
                Err(words) => {
                    passed_over.push(PassedOver {
                        branch: base,
                        words,
                    });
                    continue;
                }
            };
            for record in status
                .get("submissions")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                let number = record.get("id").and_then(Value::as_u64).unwrap_or_default();
                if submitted_from(record, branch)
                    && newest.as_ref().is_none_or(|(id, _)| number > *id)
                {
                    newest = Some((number, record.clone()));
                }
            }
        }
        let read = match newest.map(|(_, record)| record) {
            None => SubmissionRead::None,
            Some(record) => match (id(&record), state(&record)) {
                (Some(id), Ok(state)) => SubmissionRead::Found {
                    submission: Submission {
                        id,
                        branch: branch.into(),
                        target: text(&record, "branch").unwrap_or_default(),
                        state,
                        request: request(&record),
                    },
                },
                (None, _) => SubmissionRead::CouldNotRead {
                    reason: format!("sluice gave a submission with no id: {record}"),
                },
                (_, Err(reason)) => SubmissionRead::CouldNotRead { reason },
            },
        };
        ReadAcross { read, passed_over }
    }

    fn step(&self, repository: &str, base: &str) -> Stepped {
        match self.json(Git::at(repository).path(), &["step", base, "--json"]) {
            Ok(_) => Stepped::Stepped,
            Err(reason) => Stepped::CouldNotRead { reason },
        }
    }

    fn base_revision(&self, repository: &str, base: &str) -> BaseRevision {
        let git = Git::at(repository);
        let remote = match base_remote(repository, base) {
            Ok(remote) => remote,
            Err(reason) => return BaseRevision::Refused { reason },
        };
        let at = match &remote {
            Some(remote) => {
                let refspec = format!("+refs/heads/{base}:refs/remotes/{remote}/{base}");
                if let Err(words) = git.bytes(&["fetch", "--quiet", remote, &refspec]) {
                    return BaseRevision::Refused {
                        reason: format!("the fetch of {base} from {remote} failed: {words}"),
                    };
                }
                format!("refs/remotes/{remote}/{base}")
            }
            None => format!("refs/heads/{base}"),
        };
        match git.commit(&at) {
            Ok(revision) => BaseRevision::At {
                revision: Revision::new(revision),
            },
            Err(reason) => BaseRevision::Refused { reason },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The steps a reading gave, each with whether it passed.
    fn outcomes(steps: &[ReadyStep]) -> Vec<(&str, bool)> {
        steps
            .iter()
            .map(|step| (step.step.as_str(), step.passed))
            .collect()
    }

    #[test]
    fn the_steps_sluice_equipped_with() {
        let long = format!("{}moon setup: the toolchain is unknown", "x".repeat(5000));
        let answer = serde_json::json!({
            "branch": "main",
            "revision": "2eb676c",
            "steps": [
                {"step": "toolchain", "passed": true, "words": "  activated\n"},
                {"step": "proto install", "passed": true, "words": "installed moon 2.6.1"},
                {"step": "moon --version", "passed": true, "words": "moon 2.6.1"},
                {"step": "moon setup", "passed": false, "words": long},
            ],
        });
        let steps = equipped(false, &answer.to_string(), "");
        assert_eq!(
            outcomes(&steps),
            vec![
                ("toolchain", true),
                ("proto install", true),
                ("moon --version", true),
                ("moon setup", false),
            ]
        );
        assert_eq!(steps[0].words, "activated");
        assert_eq!(steps[3].words, shell::tail(&long));
        assert!(steps[3].words.len() < long.len(), "the words were not cut");
        assert!(steps[3].words.ends_with("the toolchain is unknown"));
    }

    #[test]
    fn nothing_to_equip_is_no_step() {
        let answer = r#"{"branch": "main", "revision": "2eb676c", "steps": []}"#;
        assert_eq!(equipped(true, answer, ""), vec![]);
    }

    #[test]
    fn an_answer_with_no_steps() {
        let refused = r#"{"error": {"kind": "refused", "message": "no .config/sluice.toml at 2eb676c", "next": []}}"#;
        let unread = "equipping /r/places/fix--one for main at 2eb676c";
        let readings = [
            (
                equipped(false, refused, "a warning"),
                "no .config/sluice.toml at 2eb676c",
            ),
            (equipped(false, unread, "unexpected"), "unexpected"),
            (equipped(true, unread, ""), unread),
            (
                equipped(false, "", "bash: line 1: sluice: command not found"),
                "bash: line 1: sluice: command not found",
            ),
        ];
        for (steps, words) in readings {
            assert_eq!(outcomes(&steps), vec![(EQUIP, false)], "{steps:?}");
            assert_eq!(steps[0].words, words);
        }
    }

    #[test]
    fn a_failure_after_every_step_passed_is_sluices() {
        let answer = r#"{"steps": [{"step": "toolchain", "passed": true, "words": ""}]}"#;
        let steps = equipped(
            false,
            answer,
            "sluice: the run directory could not be removed",
        );
        assert_eq!(outcomes(&steps), vec![("toolchain", true), (EQUIP, false)]);
        assert_eq!(
            steps[1].words,
            "sluice: the run directory could not be removed"
        );
    }
}
