//! flume's commands (section 9; design 16; command-line spec): each
//! builds the core for itself, asks it one act, and prints its answer for
//! a person or as its JSON with `--json`, exiting 1 on a refusal (60, 61).
//! No command computes a stage's end, a status, a submission or a tidy of
//! its own (core.14), and none names a tool (core.1).

mod launch;
pub mod mcp;
pub mod place;
mod print;

use clap::{Args, Parser, Subcommand};
use flume_core::{Answer, Answering, Engine, KindsLock, LockOf, Retry, WaitFor};
use flume_seams::{
    read_span, AgentState, EndKey, HoldOn, Launch, PlaceId, Reading, Seconds, StopKind,
};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

/// The variable an agent flume started is named in (33, C.4).
pub const AGENT: &str = "FLUME_AGENT";

/// The variable an agent's start's secret is in (41, C.4).
pub const SECRET: &str = "FLUME_SECRET";

/// The variable an agent flume started names its place in, by which a
/// command run in a role's run finds the role it serves (132, C.4).
pub const PLACE: &str = "FLUME_PLACE";

/// How long `settle --on-quiet` reads the agent for until it is quiet
/// (121; design 14).
pub const ON_QUIET_WITHIN: Seconds = 60;

#[derive(Debug, Parser)]
#[command(
    name = "flume",
    version = crate::VERSION,
    about = "One launch carried into one worktree of its own and worked through its kind's stages."
)]
struct Cli {
    /// Print the answer as its JSON (61).
    #[arg(long, global = true)]
    json: bool,
    /// The repository, where it is not the one the command runs in.
    #[arg(long, global = true, value_name = "DIR")]
    repository: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Make one place and start, hold or queue its first stage (2 to 8).
    Launch(Box<LaunchArgs>),
    /// Every fact of a place, or of every place, and of the host (56 to 58).
    Status {
        /// A location, or `<kind>--<name>` in the repository.
        place: Option<String>,
        /// Each place's one line alone, reading only what a line holds
        /// (57, 60).
        #[arg(long)]
        line: bool,
    },
    /// A line for each place on the host and each stream it declares,
    /// reading only what flume keeps, a place that landed or was dropped
    /// left out unless `--all` is given; `flume status <place>` gives a
    /// place's every fact. Needs no repository and settles nothing (57,
    /// 200).
    List {
        /// Every place, those that landed or were dropped among them.
        #[arg(long)]
        all: bool,
    },
    /// The exact prompt a stage would be sent at its next start (29).
    Prompt { place: String, stage: String },
    /// Message a place's agent or a role's address: a place's agent is
    /// handed the words as a prompt once it waits; a role's address,
    /// `<stream>/<role>`, keeps them as a delivery its session takes at its
    /// next wait (53 to 55, 214).
    #[command(alias = "tell")]
    Message {
        /// A role's address, `<stream>/<role>`; else a place, by its
        /// location or `<kind>--<name>` in the repository.
        target: String,
        words: String,
        /// How long to wait for a place's agent, as `<n>`, `<n>s`, `<n>m`
        /// or `<n>h`.
        #[arg(long)]
        within: Option<String>,
        /// The key of the stop-short the words answer, as the inbox and
        /// status give it, its agent kept open for its answer: taken only
        /// while the place waits on that stop-short and only once, and
        /// otherwise refused naming the answer it took or the move the
        /// place waits on now (42, 53, 198).
        #[arg(long, value_name = "KEY")]
        answers: Option<String>,
    },
    /// End the place's agent and nothing else (51).
    Stop(Forced),
    /// End a conversation, which then counts as delivered (109).
    Finish(Forced),
    /// Give up the place's work (52).
    Drop(Forced),
    /// Declare complete the work of a place whose kind does not land (114).
    Done(Forced),
    /// Start a stage again, or submit the place again (49, 50).
    Retry {
        place: String,
        /// The stage to start again; the current one where none is named.
        #[arg(long, conflicts_with = "submission")]
        stage: Option<String>,
        /// Submit the place's head again.
        #[arg(long)]
        submission: bool,
        /// Words for the stage's agent, after the question it stopped on.
        #[arg(long)]
        words: Option<String>,
        /// End an agent that is not quiet; never one whose stage started
        /// or began under it, which `flume stop` ends first (35, 50).
        #[arg(long)]
        force: bool,
        /// The key of the stop-short the words answer, as the inbox and
        /// status give it: taken only while the place waits on that
        /// stop-short and only once, with words and naming its stage, and
        /// otherwise refused, before any agent is ended, naming the answer
        /// it took or the move the place waits on now (42, 49).
        #[arg(long, value_name = "KEY")]
        answers: Option<String>,
    },
    /// Put the place's next stage, or its submission, on hold with words
    /// saying what for, which only a retry naming it ends; or, with
    /// `--waiting`, hold the move the place waits on its caller for, with
    /// words saying who makes it and how, which a later hold on the move
    /// replaces and which ends once the place waits on another move or
    /// none (47, 49, 130).
    Hold {
        place: String,
        /// The stage held: one not started since the current stage began.
        #[arg(
            long,
            conflicts_with_all = ["submission", "waiting"],
            required_unless_present_any = ["submission", "waiting"]
        )]
        stage: Option<String>,
        /// Hold the place's submission, not yet made.
        #[arg(long, conflicts_with = "waiting")]
        submission: bool,
        /// Hold the move the place waits on its caller for: while the
        /// hold holds, the place is not stalled, so its stall reaches no
        /// one (130, 193).
        #[arg(long)]
        waiting: bool,
        /// How long a hold on the waiting move holds, as `<n>`, `<n>s`,
        /// `<n>m` or `<n>h`: once it passes, the hold lapses and the place
        /// stalls at once; with none, it holds until the move is made. A
        /// hold on a stage or the submission takes none (130).
        #[arg(long = "for", value_name = "TIME")]
        bound: Option<String>,
        /// What the hold waits for, or who makes the move and how, the
        /// caller's to make good.
        words: String,
    },
    /// Remove what is over (63 to 68).
    Tidy { place: Option<String> },
    /// Move a place, or every place, on (46, 119 to 121); with `--every`,
    /// every place as a caller's watch over the host (208).
    Settle {
        place: Option<String>,
        /// Read the place's agent until it is quiet, for up to 60 seconds,
        /// first.
        #[arg(long)]
        on_quiet: bool,
        /// Settle every place as a watch over the host whose settles come
        /// within this bound, as `<n>`, `<n>s`, `<n>m` or `<n>h`: flume
        /// runs unattended while the newest such settle began less than
        /// twice it ago.
        #[arg(long)]
        every: Option<String>,
    },
    /// From an agent flume started: its stage cannot deliver without an
    /// answer, or found work, its end going to the place's recipients, or
    /// to the role `--to` names alone (41, 43, 132, C.4).
    Needs {
        /// The stage found work a stage before its own must do again.
        #[arg(long)]
        found: bool,
        /// The role the words are for, one the stage's prompt lists: a
        /// role alone, `<role>`, is the place's own stream's, and an
        /// address, `<stream>/<role>`, any stream's. The end goes to that
        /// role alone, in place of the place's recipients; a name the
        /// prompt did not list is refused, naming those it did (41, 132).
        #[arg(long, value_name = "ROLE OR ADDRESS")]
        to: Option<String>,
        words: String,
    },
    /// Set the place's priority (70).
    Priority {
        place: String,
        #[arg(allow_negative_numbers = true)]
        priority: i64,
    },
    /// From an agent program's hook: what the hook for `event` does with
    /// the input on standard input, in an agent flume started; nothing
    /// elsewhere. `status-line` reads no input and prints the place's
    /// line. It always exits 0 (C.4).
    Hook {
        /// One of the plugin's hooks; any other does nothing.
        event: String,
    },
    /// From an agent flume started: it started, or it received a prompt
    /// (35, 89, 135).
    Ack {
        #[command(subcommand)]
        ack: Ack,
    },
    /// Wait on a place's stage or submission as a recipient: an end
    /// pending for it is answered and handed to it, one it confirmed is
    /// read, and otherwise the host's state is followed until one comes
    /// (43). A hand-over delivers nothing until its recipient confirms it:
    /// with `--confirm` the end is confirmed as it is answered, and
    /// otherwise it stays pending until `flume confirm` names its key.
    /// Exits 0 for a stage delivered or finished or a submission landed, 2
    /// for any other end, 3 for still waiting at the timeout.
    Wait {
        /// A location, or `<kind>--<name>` in the repository.
        place: String,
        /// The stage whose current start's end is waited on.
        #[arg(
            long,
            conflicts_with = "submission",
            required_unless_present = "submission"
        )]
        stage: Option<String>,
        /// The place's submission's outcome is waited on.
        #[arg(long)]
        submission: bool,
        /// The recipient waited as; `$FLUME_RECIPIENT`, a role's run's
        /// address, or the terminal's session, where none is named.
        #[arg(long)]
        recipient: Option<String>,
        /// How long to follow the host's state, as `<n>`, `<n>s`, `<n>m`
        /// or `<n>h`; with none, until an end comes.
        #[arg(long)]
        timeout: Option<String>,
        /// Confirm the end answered as its recipient, on receipt, so it is
        /// delivered and pending no more.
        #[arg(long)]
        confirm: bool,
    },
    /// Collect every end pending for a recipient, of every place on the
    /// host, each handed to it; an inbox naming a place collects that
    /// place's ends alone (43, 62). A hand-over delivers nothing until its
    /// recipient confirms it: an end the inbox gives stays pending, and is
    /// given again by the next inbox, until `--confirm` confirms it on
    /// receipt or `flume confirm` names its delivery's key.
    Inbox {
        /// The place whose ends alone are collected, named as `flume
        /// status` names one: a location, or `<kind>--<name>` in the
        /// repository, its own document or, once it was tidied, those kept
        /// for it, a tidied document's `<place>@<at>` naming that one
        /// alone; every place on the host where none is named.
        place: Option<String>,
        /// The recipient collected for; `$FLUME_RECIPIENT`, a role's
        /// run's address, or the terminal's session, where none is named.
        #[arg(long)]
        recipient: Option<String>,
        /// Confirm each end given as its recipient, on receipt, so it is
        /// delivered and given no more.
        #[arg(long)]
        confirm: bool,
    },
    /// Confirm, as a recipient, each delivery key given: a notice, a wait,
    /// the inbox and a session's start hand an end over, and a hand-over
    /// delivers nothing until its recipient confirms it by its key, which
    /// `flume inbox`, `flume wait`, `flume status` and the notice give as
    /// the end's delivery (43). A key confirmed before says when it first
    /// was; a key no end is under, or one whose end is another
    /// recipient's, is refused naming it, and the keys after it are not
    /// confirmed. Exits 1 on a refusal.
    Confirm {
        /// The delivery keys, `<kind>--<name>/<token>/<end key>`.
        #[arg(required = true, value_name = "KEY")]
        keys: Vec<String>,
        /// The recipient confirmed as; `$FLUME_RECIPIENT`, a role's run's
        /// address, or the terminal's session, where none is named.
        #[arg(long)]
        recipient: Option<String>,
    },
    /// Wake a role's address, `<stream>/<role>`: a role a source of the
    /// host declares in its roles file, for a stream the host declares.
    /// Its run, as the source's entry gives it, is launched in the
    /// address's stream where none stands, and otherwise nothing is made,
    /// the answer saying the run is live or waits. An address its source
    /// gives a standing agent launches nothing, the answer saying where
    /// that agent lives, what was found of it and the ends pending for
    /// it. An address carried to another host, declared by no source or
    /// by two, or whose stream the host does not declare is refused,
    /// saying why (132, 143, 193).
    Wake {
        /// The role's address, `<stream>/<role>`.
        address: String,
    },
    /// Show a session of a stream's terminal in the stream's view, by the
    /// session's name, never a stream's: a window of the stream's
    /// terminal app running its multiplexer's attach for the session,
    /// titled as the stream declares, else the stream's name, for the
    /// stream's own session, and by the session's name for one the stream
    /// names for a kind or a stage; or the multiplexer's own app; or the
    /// line to run where no terminal app opens a window. Needs no
    /// repository; each asking opens a window of its own (186, 199).
    Open {
        /// A session's name: a stream's own, the one the stream's
        /// `session` names, else the stream's name, or one a stream names
        /// for a kind or a stage; the default stream's own where none is
        /// named.
        session: Option<String>,
    },
    /// flume's status over MCP on standard input and output, one tool,
    /// `status`, settling nothing (57, 60).
    Mcp,
    /// The kinds a lock records, by which a kind from outside flume and
    /// the repository runs (155, C.2).
    Kinds {
        #[command(subcommand)]
        kinds: KindsCommand,
    },
}

#[derive(Debug, Subcommand)]
enum KindsCommand {
    /// Record each kind, named whole as `<source>/<name>@<version>`, at
    /// its content's hash: in `.config/flume.lock` of the working copy the
    /// command runs in, which its caller commits, or in the host's lock
    /// with `--host`. It fetches nothing and commits nothing (155, C.2).
    Lock {
        /// Write the host's lock, beside its configuration.
        #[arg(long)]
        host: bool,
        /// Take each kind's entry out, reading no package.
        #[arg(long, conflicts_with = "hash")]
        remove: bool,
        /// Record the one kind named only where its content hashes so,
        /// and otherwise refuse, the lock left as it was.
        #[arg(long)]
        hash: Option<String>,
        /// Each kind, `<source>/<name>@<version>`.
        #[arg(required = true, value_name = "IDENTITY")]
        kinds: Vec<String>,
    },
}

#[derive(Debug, Subcommand)]
enum Ack {
    /// The agent started under the identity and secret its environment
    /// names, in a session with a transcript (135).
    Started {
        #[arg(long)]
        session: String,
        #[arg(long)]
        transcript: String,
        /// How the session began.
        #[arg(long, default_value = "startup")]
        source: String,
        /// The version of flume's plugin the agent loaded.
        #[arg(long)]
        plugin_version: String,
    },
    /// The agent received a prompt, by the digest of its text, paste tags
    /// taken off and whitespace collapsed (89).
    Received {
        #[arg(long)]
        digest: String,
        /// The mode the session runs in, as its program names it,
        /// compared at the start's first prompt where its start recorded a
        /// mode (161, 162).
        #[arg(long)]
        mode: Option<String>,
    },
}

#[derive(Debug, Args)]
struct Forced {
    place: String,
    /// Act on an agent that is not quiet.
    #[arg(long)]
    force: bool,
}

#[derive(Debug, Args)]
struct LaunchArgs {
    /// The kind, as the launch's declaration, the repository's or flume's
    /// own names it, `<name>` or `<name>@<version>`, or by its source,
    /// `<source>/<name>` or `<source>/<name>@<version>`; read whole by
    /// the core (19, 154).
    #[arg(long)]
    kind: String,
    #[arg(long)]
    name: String,
    /// The branch the place is cut from; the default branch where none is
    /// named.
    #[arg(long)]
    base: Option<String>,
    #[arg(long, conflicts_with = "brief_file")]
    brief: Option<String>,
    #[arg(long, value_name = "FILE")]
    brief_file: Option<PathBuf>,
    /// A declaration of kinds the launch carries (19).
    #[arg(long, value_name = "FILE")]
    kinds: Option<PathBuf>,
    /// A stage on hold, with its words (47, 130).
    #[arg(long, value_name = "STAGE=WORDS")]
    hold: Vec<String>,
    #[arg(long)]
    hold_submission: bool,
    /// A recipient of the place's ends: a terminal's session by its name,
    /// or a role's address, `<stream>/<role>`, of any stream, whose run is
    /// woken for them. A name no pane holds is sent no notice: its ends,
    /// a drop's or a done's among them, wait for it, the place's record
    /// kept once it is tidied, until it collects them by `flume inbox` or
    /// `flume wait` as that name. Where none is named, `$FLUME_RECIPIENT`,
    /// else the terminal's session, else the answerer behind the place's
    /// stream, the role a source of the host declares to stand behind it;
    /// a launch none of them gives is refused. Every stall reaches the
    /// answerer behind the place's stream too (7, 43, 62, 130, 132, 193).
    #[arg(long, value_name = "SESSION OR ADDRESS")]
    recipient: Vec<String>,
    #[arg(long)]
    mark: Option<String>,
    #[arg(long, allow_negative_numbers = true)]
    priority: Option<i64>,
    /// How long the place may wait on its caller under no hold's words
    /// before it is stalled, as `<n>`, `<n>s`, `<n>m` or `<n>h`; five
    /// minutes where none is given (130).
    #[arg(long, value_name = "TIME")]
    stall_after: Option<String>,
    /// One setting of every stage's agent, or of one stage's (32): the
    /// field one of program, model, effort, instructions, beside, session,
    /// mode and keep_open, `effort` its program's own word for how hard its
    /// model works, `session` the terminal's session its agents open in,
    /// the mode one of plan, ask, edits and full, or `mode.<program>` for
    /// one program's entry of a mode, and `keep_open` how long the agent
    /// is kept open for an answer once it stops short, as `<n>`, `<n>s`,
    /// `<n>m` or `<n>h`, one time for every program, an hour where no
    /// layer gives one (107, 162, 169, 197, 198).
    #[arg(long, value_name = "[STAGE.]FIELD=VALUE")]
    agent: Vec<String>,
    /// The stream the place runs in, one the host's configuration
    /// declares; the one it names for the repository or the kind, else
    /// `default`, where none is named (143).
    #[arg(long)]
    stream: Option<String>,
    /// The intent the place serves, by its name; the repository and base
    /// where none is named (147).
    #[arg(long)]
    intent: Option<String>,
}

/// What a command prints, where, and its exit status.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outcome {
    pub text: String,
    /// The words are about the command's own words, for its error stream.
    pub usage: bool,
    pub code: u8,
}

impl Outcome {
    /// Prints the words and gives the exit status.
    pub fn exit(self) -> ExitCode {
        if self.usage {
            eprintln!("{}", self.text.trim_end());
        } else if !self.text.is_empty() {
            println!("{}", self.text);
        }
        ExitCode::from(self.code)
    }
}

/// Runs the command `args` give, the core built by `build` once it is
/// needed (design 16).
pub fn run<I, T>(args: I, build: &dyn Fn() -> Result<Engine, String>) -> Outcome
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(error) => {
            let code = u8::try_from(error.exit_code()).unwrap_or(2);
            return Outcome {
                text: error.render().to_string(),
                usage: code != 0,
                code,
            };
        }
    };
    match &cli.command {
        Command::Hook { event } => {
            let mut input = String::new();
            if crate::profiles::hook_reads_input(event) {
                let _ = std::io::Read::read_to_string(&mut std::io::stdin(), &mut input);
            }
            return Outcome {
                text: crate::profiles::hook(event, &input, build),
                usage: false,
                code: 0,
            };
        }
        Command::Ack { ack } => {
            let acked = match ack {
                Ack::Started {
                    session,
                    transcript,
                    source,
                    plugin_version,
                } => {
                    crate::profiles::ack_started(session, transcript, source, plugin_version, build)
                }
                Ack::Received { digest, mode } => {
                    crate::profiles::ack_received(digest, mode.as_deref(), build)
                }
            };
            return match acked {
                Ok(text) => Outcome {
                    text,
                    usage: false,
                    code: 0,
                },
                Err(reason) => Outcome {
                    text: format!("refused: {reason}"),
                    usage: false,
                    code: 1,
                },
            };
        }
        Command::Mcp => {
            let stdin = std::io::stdin();
            let stdout = std::io::stdout();
            mcp::serve(
                stdin.lock(),
                stdout.lock(),
                build,
                cli.repository.as_deref(),
            );
            return Outcome {
                text: String::new(),
                usage: false,
                code: 0,
            };
        }
        _ => {}
    }
    let answer = act(&cli, build).unwrap_or_else(Answer::refused);
    let text = if cli.json {
        answer.to_json()
    } else {
        print::person(&answer)
    };
    Outcome {
        text,
        usage: false,
        code: answer.exit(),
    }
}

/// The core's answer to the command, or the refusal naming what kept it
/// from being asked.
fn act(cli: &Cli, build: &dyn Fn() -> Result<Engine, String>) -> Result<Answer, String> {
    let given = cli.repository.as_deref();
    let named = |place: &str| place::place(place, given);
    let maybe = |place: &Option<String>| place.as_deref().map(named).transpose();
    Ok(match &cli.command {
        Command::Needs { found, to, words } => {
            let identity = from_agent(AGENT)?;
            let secret = from_agent(SECRET)?;
            let kind = if *found {
                StopKind::FoundWork
            } else {
                StopKind::NeedsAnswer
            };
            build()?.needs_to(&identity, &secret, words, kind, to.as_deref())
        }
        Command::Launch(args) => {
            let launch = launch_of(args, given)?;
            build()?.launch(&launch)
        }
        Command::Status { place, line } => {
            let place = maybe(place)?;
            let core = build()?;
            if *line {
                core.lines(place.as_ref())
            } else {
                core.status(place.as_ref())
            }
        }
        Command::List { all } => build()?.list(*all),
        Command::Prompt { place, stage } => {
            let place = named(place)?;
            build()?.prompt(&place, stage)
        }
        Command::Message {
            target,
            words,
            within,
            answers,
        } => {
            let place = named(target)?;
            let within = within.as_deref().map(read_span).transpose()?;
            let key = key_of(answers)?;
            let core = build()?;
            let answering = answering(&core, key);
            core.message_answering(&place, words, within, &answering)
        }
        Command::Stop(forced) => build_for(forced, &named, build, Engine::stop)?,
        Command::Finish(forced) => build_for(forced, &named, build, Engine::finish)?,
        Command::Drop(forced) => build_for(forced, &named, build, Engine::drop_place)?,
        Command::Done(forced) => build_for(forced, &named, build, Engine::done)?,
        Command::Retry {
            place,
            stage,
            submission,
            words,
            force,
            answers,
        } => {
            let place = named(place)?;
            let target = match (stage, submission) {
                (Some(stage), _) => Retry::Stage(stage.clone()),
                (None, true) => Retry::Submission,
                (None, false) => Retry::Current,
            };
            let key = key_of(answers)?;
            let core = build()?;
            let answering = answering(&core, key);
            core.retry_answering(&place, target, words.clone(), *force, &answering)
        }
        Command::Hold {
            place,
            stage,
            submission: _,
            waiting,
            bound,
            words,
        } => {
            let place = named(place)?;
            let on = match (stage, waiting) {
                (Some(stage), _) => HoldOn::Stage {
                    stage: stage.clone(),
                },
                (None, true) => HoldOn::Waiting,
                (None, false) => HoldOn::Submission,
            };
            let bound = bound.as_deref().map(read_span).transpose()?;
            build()?.hold(&place, on, words, bound)
        }
        Command::Tidy { place } => {
            let place = maybe(place)?;
            build()?.tidy(place.as_ref())
        }
        Command::Settle {
            place: Some(given_place),
            every: Some(_),
            ..
        } => {
            return Err(format!(
                "--every settles every place as a caller's watch over the host, and takes no place: {given_place} was named"
            ));
        }
        Command::Settle {
            every: Some(every), ..
        } => {
            let bound = read_span(every)?;
            if bound == 0 {
                return Err(format!(
                    "--every {every} is no bound: a watch's bound is a positive duration"
                ));
            }
            build()?.settle_every(bound)
        }
        Command::Settle {
            place, on_quiet, ..
        } => {
            let place = maybe(place)?;
            let core = build()?;
            if *on_quiet {
                if let Some(place) = &place {
                    until_quiet(&core, place, Duration::from_secs(ON_QUIET_WITHIN));
                }
            }
            core.settle(place.as_ref())
        }
        Command::Priority { place, priority } => {
            let place = named(place)?;
            build()?.priority(&place, *priority)
        }
        Command::Wait {
            place,
            stage,
            submission: _,
            recipient,
            timeout,
            confirm,
        } => {
            let place = named(place)?;
            let of = match stage {
                Some(stage) => WaitFor::Stage(stage.clone()),
                None => WaitFor::Submission,
            };
            let timeout = timeout.as_deref().map(read_span).transpose()?;
            let core = build()?;
            let recipient = recipient_of(&core, recipient)?;
            core.wait(&place, &of, &recipient, timeout, *confirm)
        }
        Command::Inbox {
            place,
            recipient,
            confirm,
        } => {
            let place = maybe(place)?;
            let core = build()?;
            let recipient = recipient_of(&core, recipient)?;
            core.inbox(&recipient, place.as_ref(), *confirm)
        }
        Command::Confirm { keys, recipient } => {
            let core = build()?;
            let recipient = recipient_of(&core, recipient)?;
            core.confirm(&recipient, keys)
        }
        Command::Wake { address } => build()?.wake(address),
        Command::Kinds {
            kinds:
                KindsCommand::Lock {
                    host,
                    remove,
                    hash,
                    kinds,
                },
        } => {
            let of = if *host {
                LockOf::Host
            } else {
                LockOf::WorkingCopy {
                    dir: place::working_copy(given)?,
                }
            };
            // Held until the lock is written, so two writers on the host
            // never lose each other's entry (74, 155; design 6).
            let _writing = crate::host::writing_kinds_lock()?;
            build()?.lock_kinds(&KindsLock {
                of,
                kinds: kinds.clone(),
                remove: *remove,
                hash: hash.clone(),
            })
        }
        Command::Open { session } => build()?.open_session(session.as_deref()),
        Command::Hook { .. } | Command::Ack { .. } | Command::Mcp => {
            unreachable!("answered before the core's acts")
        }
    })
}

/// The recipient named, else the one the core reads for a command that
/// names none: `$FLUME_RECIPIENT`, the address of the role's run
/// `$FLUME_PLACE` names, or the terminal's session (7, 132; design 7).
fn recipient_of(core: &Engine, named: &Option<String>) -> Result<String, String> {
    if let Some(named) = named {
        return Ok(named.clone());
    }
    let here = std::env::var(PLACE)
        .ok()
        .filter(|place| !place.is_empty())
        .map(PlaceId::new);
    core.recipient(here.as_ref()).ok_or_else(|| {
        format!("no recipient is named: name one with --recipient or ${}, or run in a terminal's session or a role's run", crate::host::RECIPIENT)
    })
}

/// What a retry's or a message's answer names beside its words: the key of
/// the stop-short it answers, where `--answers` gives one, and who gives
/// it, the recipient the command acts as where it names none, recorded on
/// the stop-short it answers; none where no recipient is read (42).
fn answering(core: &Engine, key: Option<EndKey>) -> Answering {
    Answering {
        key,
        by: recipient_of(core, &None).ok(),
    }
}

/// The key `--answers` gives, read from its token alone, refused naming
/// the forms a key takes before the core is asked anything (42, 49).
fn key_of(answers: &Option<String>) -> Result<Option<EndKey>, String> {
    answers.as_deref().map(str::parse::<EndKey>).transpose()
}

/// A forced act on the place `forced` names.
fn build_for(
    forced: &Forced,
    named: &dyn Fn(&str) -> Result<PlaceId, String>,
    build: &dyn Fn() -> Result<Engine, String>,
    act: fn(&Engine, &PlaceId, bool) -> Answer,
) -> Result<Answer, String> {
    let place = named(&forced.place)?;
    Ok(act(&build()?, &place, forced.force))
}

/// What an agent's environment gives in `name`, or the refusal saying the
/// command runs only in an agent flume started (41).
fn from_agent(name: &str) -> Result<String, String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            format!("flume needs runs only in an agent flume started, and {name} is not set")
        })
}

/// The launch the words give, in the repository the command runs in.
fn launch_of(args: &LaunchArgs, given: Option<&Path>) -> Result<Launch, String> {
    let repository = place::repository(given)?;
    let base = match &args.base {
        Some(base) => base.clone(),
        None => place::default_base(&repository)?,
    };
    let brief = match (&args.brief, &args.brief_file) {
        (Some(brief), _) => Some(brief.clone()),
        (None, Some(file)) => Some(
            std::fs::read_to_string(file)
                .map_err(|error| format!("the brief {}: {error}", file.display()))?,
        ),
        (None, None) => None,
    };
    let declaration = args
        .kinds
        .as_ref()
        .map(|file| {
            std::path::absolute(file)
                .map(|file| file.to_string_lossy().into_owned())
                .map_err(|error| format!("the declaration {}: {error}", file.display()))
        })
        .transpose()?;
    let mut every_stage = Default::default();
    let mut settings = BTreeMap::new();
    let working = std::env::current_dir()
        .map_err(|error| format!("the working directory could not be read: {error}"))?;
    for given in &args.agent {
        launch::agent(given, &working, &mut every_stage, &mut settings)?;
    }
    let mut hold = vec![];
    let mut hold_words = BTreeMap::new();
    for given in &args.hold {
        let (stage, words) = launch::hold(given)?;
        if let Some(words) = words {
            hold_words.insert(stage.clone(), words);
        }
        hold.push(stage);
    }
    Ok(Launch {
        repository,
        kind: args.kind.clone(),
        name: args.name.clone(),
        base,
        brief,
        declaration,
        settings,
        every_stage,
        hold,
        hold_submission: args.hold_submission,
        mark: args.mark.clone(),
        hold_words,
        priority: args.priority.unwrap_or(0),
        recipients: args.recipient.clone(),
        stall_after: args.stall_after.as_deref().map(read_span).transpose()?,
        stream: args.stream.clone(),
        intent: args.intent.clone(),
    })
}

/// Reads the place's current agent, as status gives it, until it is
/// quiet or `within` has passed: neither working nor asking, nor waiting
/// on own work (121). A place with no agent is read once.
fn until_quiet(core: &Engine, place: &PlaceId, within: Duration) {
    let began = Instant::now();
    loop {
        let quiet = match core.status(Some(place)) {
            Answer::Status { places, .. } => places
                .first()
                .and_then(|status| status.agent.as_ref())
                .is_none_or(|agent| {
                    matches!(agent.state, AgentState::Waiting | AgentState::Stopped)
                        && agent.asking == Reading::No
                        && agent.own_work == Reading::No
                }),
            _ => true,
        };
        if quiet || began.elapsed() >= within {
            return;
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_that_are_no_command_are_for_the_error_stream() {
        let outcome = run(["flume", "launch"], &|| Err("not built".into()));
        assert!(outcome.usage);
        assert_eq!(outcome.code, 2);
    }

    #[test]
    fn a_watch_beside_a_place_or_with_no_bound_builds_nothing() {
        // 208: refused before the core is built, so nothing is settled
        // and no watch is recorded.
        for words in [
            ["flume", "settle", "fix--one", "--every", "2m"].as_slice(),
            ["flume", "settle", "--every", "0"].as_slice(),
            ["flume", "settle", "--every", "soon"].as_slice(),
        ] {
            let outcome = run(words.iter().copied(), &|| panic!("the core is built"));
            assert_eq!(outcome.code, 1, "{words:?}: {}", outcome.text);
            assert!(!outcome.usage, "{words:?}");
            assert!(outcome.text.starts_with("refused: "), "{}", outcome.text);
        }
    }

    #[test]
    fn needs_outside_an_agent_builds_nothing() {
        let outcome = run(["flume", "needs", "which?", "--json"], &|| {
            panic!("the core is built")
        });
        if std::env::var_os(AGENT).is_none() {
            assert_eq!(outcome.code, 1);
            assert!(outcome.text.contains(AGENT));
        }
    }
}
