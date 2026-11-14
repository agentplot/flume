//! The agent seam's answers (88 to 93, 115, 116, 121, 134, 135, 151, 161, 179, 196, 207), an
//! agent's identity and its start's secret (33, 41). Every reading of an agent's
//! own record is handed the place's own folder with the identity, where
//! the agent profile keeps its marks (128).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// The one name flume gives an agent: its repository, its place, its
/// stage and which start of the stage it is (33). The repository is
/// named by its id, taken from its common git directory (C.4).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Identity {
    /// The repository's id.
    pub repository: String,
    /// The place as `<kind>--<name>`.
    pub place: String,
    pub stage: String,
    pub start: u32,
}

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "flume:{}:{}:{}:{}",
            self.repository, self.place, self.stage, self.start
        )
    }
}

/// A secret flume gives one start alone, which a stop-short carries
/// (41).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Secret(pub String);

/// The question the agent program asks before it works in a folder new
/// to it, and how its choices show and are chosen, which the terminal
/// reads off the screen and answers yes (34).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FolderTrust {
    /// The yes choice's text, which the program shows while it asks.
    pub question: String,
    /// The mark the program shows on the line of the choice in focus.
    pub focus: String,
    /// The key that moves the focus to the next choice.
    pub next: String,
    /// The key that confirms the choice in focus, sent only while the
    /// yes choice has it.
    pub confirm: String,
}

/// command: how to start an agent so it works in its place under its
/// identity (88). `prompt_args` carry the prompt as the command's own
/// arguments, for a terminal that cannot send to a running agent (38).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCommand {
    pub program: String,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub prompt_args: Vec<String>,
    /// The program's folder-trust question and its answer, if it asks
    /// one (34).
    #[serde(default)]
    pub folder_trust: Option<FolderTrust>,
    /// The program's kind, by which a terminal that starts a known agent
    /// program names it; none where the profile names none (C.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Each file the agent is started with, by its absolute path under
    /// the place's own folder the command was given, with its text, which
    /// the core writes before the terminal opens the agent; none where the
    /// command needs none, and in a command written before (196).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub files: BTreeMap<String, String>,
}

/// A key of its program's settings that another of flume's settings or
/// the account owns, by its path, a nested key's as
/// `permissions.defaultMode`, with its owner's words: refused where a
/// table of a layer gives it, and left out of a named file (195, 196).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reserved {
    pub key: String,
    pub owner: String,
}

/// What a profile that keeps the agent seam's default says of its
/// program's own settings given it, which it refuses (195, 196).
pub const NO_OWN: &str = "its agent profile passes no settings of its program's own";

/// What a profile that keeps the agent seam's default says of the effort
/// `word` asked of it, which it refuses, since it names no way to give
/// its program an effort (170, 197).
pub fn no_effort(word: &str) -> String {
    format!("its agent profile names no way to give an effort, `{word}` among them")
}

/// A yes or no the agent's own record may not give (93).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reading {
    Yes,
    No,
    Unknown,
}

/// received: whether the agent received a text as a prompt, read from
/// its own record, which the answer names (89, 77).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Received {
    Received { record: String },
    NotReceived { record: String },
    Unknown,
}

/// spent: whether the agent's own record shows its account reached a
/// usage limit, with the reset the record names by the host's clock, none
/// where it names none, and the words it gave; unknown where the record
/// cannot say (146, 151).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Spent {
    Yes {
        until: Option<crate::Seconds>,
        words: String,
    },
    No,
    Unknown,
}

/// windows: the usage windows the agent's program last reported for the
/// account it runs under, with when flume's own part of the program took
/// them, by the host's clock, and the session they came from where the
/// agent's record names one; none yet, where the record is readable and
/// holds no report; or unknown, where the record cannot be read or the
/// profile builds no such part (171, 174, 211).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Windows {
    Reported {
        taken_at: crate::Seconds,
        windows: Vec<crate::UsageWindow>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        session: Option<String>,
    },
    NoneYet,
    Unknown,
}

/// carry: whether the agent program can carry the conversation of the
/// start whose agent stopped at its account's usage limit, or of a
/// conversation's start reported lost, to the account chosen for the next
/// start, with the session that start resumes; why not, naming a variable
/// and never its value; or that the program gives no way to (49, 179;
/// core.30).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Carry {
    Yes { session: String },
    No { why: String },
    NotOffered,
}

/// live: whether a process of the host runs the conversation of a start,
/// each by its id and when it started; none where none does, or where the
/// start names no conversation; or unknown, with what could not be read,
/// naming a variable or the host's processes and never a path (207;
/// core.30).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Live {
    Runs(Vec<crate::LiveProcess>),
    None,
    Unknown { what: String },
}

/// What [`Agent::live`] answers by default, for a profile that reads no
/// running session (171, 207).
pub const READS_NO_SESSION: &str = "its agent profile reads no running session";

/// started_without: how the agent began otherwise than its start
/// recorded, as its own word that it started shows it, or, for its mode,
/// its word at its start's first prompt: each variable the start's record
/// names that its environment lacks, each it holds with another value,
/// both by name alone, whether it runs another program, whether it came
/// up in another conversation than the one its start resumes, and the
/// mode it began in where it is not the one its start asked, none in a
/// word written before (135, 161, 162, 179).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unsettled {
    #[serde(default)]
    pub missing: Vec<String>,
    #[serde(default)]
    pub different: Vec<String>,
    #[serde(default)]
    pub program: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub session: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ModeBegan>,
}

/// The mode a start asked, as its profile passed it, and the one the
/// agent's word says it began in, none where its word names none (161,
/// 162).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeBegan {
    pub asked: String,
    pub began: Option<String>,
}

impl Unsettled {
    /// How the agent began otherwise, naming each variable and never a
    /// value, no session, and both modes where it began in another (56,
    /// 161, 162).
    pub fn words(&self) -> String {
        let mut words = vec![];
        if !self.missing.is_empty() {
            words.push(format!("without {}", self.missing.join(", ")));
        }
        if !self.different.is_empty() {
            words.push(format!("with another {}", self.different.join(", ")));
        }
        if self.program {
            words.push("under another program".to_string());
        }
        if self.session {
            words.push("in another conversation than the one its start resumes".to_string());
        }
        if let Some(ModeBegan { asked, began }) = &self.mode {
            words.push(match began {
                Some(began) => format!("in the mode `{began}`, where its start asked `{asked}`"),
                None => format!("in a mode its word did not name, where its start asked `{asked}`"),
            });
        }
        if words.is_empty() {
            return "otherwise than its start recorded".to_string();
        }
        words.join(", ")
    }
}

/// Whether the agent program offers an operation (115, 121).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Offered {
    Offered,
    NotOffered,
}

/// guard: each act the agent would have taken that submits, merges or
/// pushes its place's branch, refused and reported, or that the program
/// gives no way to (116, 108).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Guarded {
    Refused { acts: Vec<String> },
    NotOffered,
}

/// What a stop-short says of the stage: it cannot go on without an
/// answer, or it found work a stage before its own must do again, which
/// sends the place back only where the kind's fix round is such a stage
/// and otherwise waits on an answer (41, 92, 131).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopKind {
    #[default]
    NeedsAnswer,
    FoundWork,
}

/// An agent's word that its stage cannot deliver without something, with
/// its start's identity and secret, and whether it needs an answer or
/// found work a stage before its own must do again (41, 92).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopShort {
    pub identity: String,
    pub secret: Secret,
    pub words: String,
    #[serde(default)]
    pub kind: StopKind,
}

/// The agent seam (B.4). The guarantee: each answer is what the agent's
/// own record shows, and an answer the record does not give is unknown,
/// never guessed from the terminal (93).
pub trait Agent: Send + Sync {
    /// How to start the agent with its settings and `env`, its account's
    /// variables read against the login environment, in the place under
    /// its identity and secret, the prompt in `prompt_args` (88, 29, 144);
    /// `folder` is the place's own folder, under which each file the
    /// command lists lies (128, 196); `mode` is the value [`Agent::mode`]
    /// mapped, passed as its program's setting, and nothing is passed
    /// where it is none (162, 169); `settings.effort` is a word
    /// [`Agent::effort`] took, passed as its profile gives its program an
    /// effort, and nothing is passed where it is none (88, 197); `own` is
    /// its program's own settings as the layers resolved them, its
    /// program's alone and taken by [`Agent::own`], passed its program's
    /// way, and nothing is passed where they are empty (195, 196);
    /// `resume` is the session the start
    /// resumes, which [`Agent::carry`] answered yes with, so a profile
    /// that answers no yes is never given one (88, 179); what the start
    /// carries by a file, such as a standing prompt no terminal can type
    /// as an argument, is written beside its record in `folder` (148, C.4,
    /// C.5).
    #[allow(clippy::too_many_arguments)]
    fn command(
        &self,
        folder: &str,
        settings: &crate::Settings,
        mode: Option<&str>,
        own: &BTreeMap<String, serde_json::Value>,
        resume: Option<&str>,
        env: &BTreeMap<String, String>,
        place: &crate::PlaceRef,
        identity: &Identity,
        secret: &Secret,
        prompt: &str,
    ) -> AgentCommand;

    /// Whether the agent under `agent` has said it started under its
    /// start's identity, with flume's guard and stop check in force, read
    /// from its record in the place's own `folder` (135, 35, 128).
    fn started(&self, folder: &str, agent: &Identity) -> Reading;

    /// Whether the agent under `agent` received `text` as a prompt, read
    /// from its record in the place's own `folder` (89, 128).
    fn received(&self, folder: &str, agent: &Identity, text: &str) -> Received;

    /// Whether it waits on a question it put to a person (90, 128).
    fn asking(&self, folder: &str, agent: &Identity) -> Reading;

    /// Whether own work it started still runs (91, 128).
    fn own_work(&self, folder: &str, agent: &Identity) -> Reading;

    /// Whether the agent's own record shows its session ended (134, 128).
    fn ended(&self, folder: &str, agent: &Identity) -> Reading;

    /// The agent's last words as its own record holds them, which a stuck,
    /// lost or never-begun end carries; none where the record holds none
    /// or cannot be read (43, 128).
    fn last_words(&self, folder: &str, agent: &Identity) -> Option<String>;

    /// Whether the agent's own record shows its account reached a usage
    /// limit, and the reset it names, read in the place's own `folder`
    /// (151, 146, 128); unknown by default, for a program whose record
    /// says nothing of one. flume asks no program or its maker whether a
    /// limit has reset.
    fn spent(&self, _folder: &str, _agent: &Identity) -> Spent {
        Spent::Unknown
    }

    /// The usage windows the agent's program last reported for the
    /// account it runs under, read in the place's own `folder` as `spent`
    /// is, with when flume's own part of the program took them and the
    /// session they came from; none yet, or unknown (184, 211, 128).
    /// Unknown by default, for a program whose profile builds no such
    /// part, which is degraded there: its accounts have no reading (171,
    /// 174). flume asks no program or its maker, reads no sign-in and
    /// fetches nothing (146).
    fn windows(&self, _folder: &str, _agent: &Identity) -> Windows {
        Windows::Unknown
    }

    /// Whether the profile builds the windows' part, so its accounts can
    /// have a reading (171, 174, 211); not by default.
    fn reports_windows(&self) -> Offered {
        Offered::NotOffered
    }

    /// Whether its program can carry the conversation of `earlier`, the
    /// start whose agent stopped at its account's usage limit or a
    /// conversation's start reported lost, read in the place's own
    /// `folder`, to the account whose environment is `env`, an empty one
    /// where the stage's stream has no pool, so the login environment's,
    /// each value read against the login environment, with the session the
    /// next start resumes, or why not, naming a variable and never its
    /// value (49, 128, 146, 179; core.30); not offered by default, for a
    /// program that carries none. It reads only, so it is safe to ask at
    /// every claim (core.17).
    fn carry(&self, _folder: &str, _earlier: &Identity, _env: &BTreeMap<String, String>) -> Carry {
        Carry::NotOffered
    }

    /// Whether a process of the host runs the conversation of `start`, the
    /// start whose conversation a start would carry or a lost
    /// conversation's start, read in the place's own `folder`, under each
    /// of `envs`, the environments of the accounts the host's configuration
    /// declares for its program, the login environment's among them as an
    /// empty one, and under whatever environment a process runs it: each
    /// process by its id and when it started, none, or unknown with what
    /// could not be read, naming a variable and never its value or a path
    /// (49, 128, 207; core.30). Unknown by default, saying its profile
    /// reads no running session, so a program whose profile cannot read
    /// one never carries a conversation beside a process that runs it
    /// (171). It reads only, so it is safe to ask at every settle
    /// (core.17).
    fn live(&self, _folder: &str, _start: &Identity, _envs: &[BTreeMap<String, String>]) -> Live {
        Live::Unknown {
            what: READS_NO_SESSION.to_string(),
        }
    }

    /// How the agent under `agent` began otherwise than its start
    /// recorded, as its own record in the place's own `folder` holds its
    /// word, its word that it started or, where its program names its mode
    /// only with a prompt, its word at its start's first prompt: none where
    /// it began as recorded, has said nothing yet, or the program checks
    /// nothing at its start (135, 161, 128).
    fn started_without(&self, _folder: &str, _agent: &Identity) -> Option<Unsettled> {
        None
    }

    /// The value of its program's own that `entry` maps to, which its
    /// command passes, or words naming the mode where its program has no
    /// setting that means it, never read wider or narrower (88, 169); every
    /// entry refused by default, since a profile that maps no mode refuses
    /// one (170).
    fn mode(&self, entry: &crate::ModeEntry) -> Result<String, String> {
        Err(format!(
            "its agent profile maps no mode, `{entry}` among them"
        ))
    }

    /// Whether the agent's program takes `word` as its effort, which its
    /// command passes as written; words naming the effort, the program
    /// and the words it takes where it does not, and every word refused by
    /// default, in [`no_effort`]'s words, since a profile that names no way
    /// to give an effort refuses one (88, 170, 197).
    fn effort(&self, word: &str) -> Result<(), String> {
        Err(no_effort(word))
    }

    /// The variable of the agent's environment its program reads over the
    /// effort its command passes, which status names where an account's
    /// environment sets it, never its value (56, 197; core.30); none by
    /// default.
    fn effort_variable(&self) -> Option<&'static str> {
        None
    }

    /// Whether the program's word at its start, its word that it started
    /// or at its start's first prompt, names the mode it began in, so its
    /// start is compared with the mode it asked (161, 169); not by
    /// default.
    fn mode_read_back(&self) -> Offered {
        Offered::NotOffered
    }

    /// Each key of its program's settings that another of flume's
    /// settings or the account owns, with its owner, which the core
    /// refuses where a table gives it and leaves out of a named file (195,
    /// 196); none by default.
    fn reserved(&self) -> Vec<Reserved> {
        vec![]
    }

    /// Whether the profile passes `given`, its program's own settings as
    /// the layers resolved them, or words saying its program gives no way
    /// (195, 196); an empty entry is always taken, and every other refused
    /// by default, in [`NO_OWN`]'s words, as a profile that maps no mode
    /// refuses one (170).
    fn own(&self, given: &BTreeMap<String, serde_json::Value>) -> Result<(), String> {
        if given.is_empty() {
            Ok(())
        } else {
            Err(NO_OWN.into())
        }
    }

    /// The variables the agent program keeps from every process it
    /// starts, its hooks among them, whatever gives them: its word that it
    /// started cannot show them, so no start's record names them (161);
    /// none by default.
    fn withheld(&self) -> Vec<String> {
        vec![]
    }

    /// What the agent's own record says of its start that no reading
    /// answers and status reports with the agent, such as the agent
    /// program's part of flume being of another version than the one
    /// that started it (135); none by default.
    fn notes(&self, _folder: &str, _agent: &Identity) -> Vec<String> {
        vec![]
    }

    /// The one way the agent tells flume it stops short, filled in for
    /// its place and stage, as its prompt gives it (92, 28): the same for
    /// every start of the stage, naming nothing of `agent` that differs
    /// between starts, so a repetition's prompt matches the one before it
    /// up to its group (141; core.31).
    fn stop_short(&self, agent: &Identity) -> String;

    /// How the way to stop short names the role its words are for, one
    /// of the names its prompt lists after these words, a role of the
    /// place's own stream by the role alone and any other stream's by its
    /// address: the same for every start (41, 92, 132; core.31).
    fn stop_short_to(&self) -> String;

    /// Whether the program lets flume keep an agent about to go quiet at
    /// work, with words naming its deliverable (115).
    fn send_back(&self) -> Offered;

    /// Each act the agent would have taken that submits, merges or pushes
    /// its place's branch, refused and reported (116, 108, 128).
    fn guard(&self, folder: &str, agent: &Identity) -> Guarded;

    /// Whether the program tells flume at once each time the agent goes
    /// quiet, so that flume settles its place (121).
    fn on_quiet(&self) -> Offered;
}
