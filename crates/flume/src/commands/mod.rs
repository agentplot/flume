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
use flume_core::{Answer, Engine, Retry, WaitFor};
use flume_seams::{AgentState, HoldOn, Launch, PlaceId, Reading, Seconds, StopKind};
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
    },
    /// The exact prompt a stage would be sent at its next start (29).
    Prompt { place: String, stage: String },
    /// Hand the place's agent words as a prompt (53 to 55).
    Tell {
        place: String,
        words: String,
        /// How long to wait for the agent, as `<n>`, `<n>s`, `<n>m` or `<n>h`.
        #[arg(long)]
        within: Option<String>,
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
        #[arg(long)]
        force: bool,
    },
    /// Put the place's next stage, or its submission, on hold with words
    /// saying what for; only a retry naming it ends the hold (47, 130).
    Hold {
        place: String,
        /// The stage held: one not started since the current stage began.
        #[arg(
            long,
            conflicts_with = "submission",
            required_unless_present = "submission"
        )]
        stage: Option<String>,
        /// Hold the place's submission, not yet made.
        #[arg(long)]
        submission: bool,
        /// What the hold waits for, the caller's to make good.
        words: String,
    },
    /// Remove what is over (63 to 68).
    Tidy { place: Option<String> },
    /// Move a place, or every place, on (46, 119 to 121).
    Settle {
        place: Option<String>,
        /// Read the place's agent until it is quiet, for up to 60 seconds,
        /// first.
        #[arg(long)]
        on_quiet: bool,
    },
    /// From an agent flume started: its stage cannot deliver without an
    /// answer, or found work (41, C.4).
    Needs {
        /// The stage found work it cannot deliver past.
        #[arg(long)]
        found: bool,
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
    /// elsewhere. It always exits 0 (C.4).
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
    /// pending for it is answered and delivered, one already delivered is
    /// read, and otherwise the host's state is followed until one comes
    /// (43). Exits 0 for a stage delivered or finished or a submission
    /// landed, 2 for any other end, 3 for still waiting at the timeout.
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
    },
    /// Collect every end pending for a recipient, of every place on the
    /// host, each then delivered to it (43).
    Inbox {
        /// The recipient collected for; `$FLUME_RECIPIENT`, a role's
        /// run's address, or the terminal's session, where none is named.
        #[arg(long)]
        recipient: Option<String>,
    },
    /// Wake a role's address, `<partition>/<role>`: its run, the kind the
    /// host's `[roles]` table names for it, launched where none stands,
    /// and otherwise nothing made, the answer saying the run is live or
    /// waits (132).
    Wake {
        /// The role's address, `<partition>/<role>`.
        address: String,
    },
    /// flume's status over MCP on standard input and output, one tool,
    /// `status`, settling nothing (57, 60).
    Mcp,
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
    /// A recipient of the place's ends; `$FLUME_RECIPIENT`, or the
    /// terminal's session, where none is named (7).
    #[arg(long)]
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
    /// field one of program, model, instructions, beside and mode, the
    /// mode one of plan, ask, edits and full, or `mode.<program>` for one
    /// program's entry of a mode (162, 169).
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
            let _ = std::io::Read::read_to_string(&mut std::io::stdin(), &mut input);
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
                Ack::Received { digest } => crate::profiles::ack_received(digest, build),
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
        Command::Needs { found, words } => {
            let identity = from_agent(AGENT)?;
            let secret = from_agent(SECRET)?;
            let kind = if *found {
                StopKind::FoundWork
            } else {
                StopKind::NeedsAnswer
            };
            build()?.needs(&identity, &secret, words, kind)
        }
        Command::Launch(args) => {
            let launch = launch_of(args, given)?;
            build()?.launch(&launch)
        }
        Command::Status { place } => {
            let place = maybe(place)?;
            build()?.status(place.as_ref())
        }
        Command::Prompt { place, stage } => {
            let place = named(place)?;
            build()?.prompt(&place, stage)
        }
        Command::Tell {
            place,
            words,
            within,
        } => {
            let place = named(place)?;
            let within = within.as_deref().map(seconds).transpose()?;
            build()?.tell(&place, words, within)
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
        } => {
            let place = named(place)?;
            let target = match (stage, submission) {
                (Some(stage), _) => Retry::Stage(stage.clone()),
                (None, true) => Retry::Submission,
                (None, false) => Retry::Current,
            };
            build()?.retry(&place, target, words.clone(), *force)
        }
        Command::Hold {
            place,
            stage,
            submission: _,
            words,
        } => {
            let place = named(place)?;
            let on = match stage {
                Some(stage) => HoldOn::Stage {
                    stage: stage.clone(),
                },
                None => HoldOn::Submission,
            };
            build()?.hold(&place, on, words)
        }
        Command::Tidy { place } => {
            let place = maybe(place)?;
            build()?.tidy(place.as_ref())
        }
        Command::Settle { place, on_quiet } => {
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
        } => {
            let place = named(place)?;
            let of = match stage {
                Some(stage) => WaitFor::Stage(stage.clone()),
                None => WaitFor::Submission,
            };
            let timeout = timeout.as_deref().map(seconds).transpose()?;
            let core = build()?;
            let recipient = recipient_of(&core, recipient)?;
            core.wait(&place, &of, &recipient, timeout)
        }
        Command::Inbox { recipient } => {
            let core = build()?;
            let recipient = recipient_of(&core, recipient)?;
            core.inbox(&recipient)
        }
        Command::Wake { address } => build()?.wake(address),
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
    for given in &args.agent {
        launch::agent(given, &mut every_stage, &mut settings)?;
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
        stall_after: args.stall_after.as_deref().map(seconds).transpose()?,
        stream: args.stream.clone(),
        intent: args.intent.clone(),
    })
}

/// A time given as `<n>`, `<n>s`, `<n>m` or `<n>h`, in seconds.
fn seconds(given: &str) -> Result<Seconds, String> {
    let (number, unit) = match given.char_indices().last() {
        Some((at, unit)) if unit.is_ascii_alphabetic() => (&given[..at], unit),
        _ => (given, 's'),
    };
    let scale = match unit {
        's' => 1,
        'm' => 60,
        'h' => 60 * 60,
        _ => 0,
    };
    number
        .parse::<Seconds>()
        .ok()
        .filter(|_| scale > 0)
        .map(|number| number * scale)
        .ok_or_else(|| format!("{given} is no time: it is <n>, <n>s, <n>m or <n>h"))
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
    fn a_time_in_seconds() {
        assert_eq!(seconds("90"), Ok(90));
        assert_eq!(seconds("90s"), Ok(90));
        assert_eq!(seconds("10m"), Ok(600));
        assert_eq!(seconds("1h"), Ok(3600));
        assert!(seconds("soon").is_err());
        assert!(seconds("5d").is_err());
    }

    #[test]
    fn words_that_are_no_command_are_for_the_error_stream() {
        let outcome = run(["flume", "launch"], &|| Err("not built".into()));
        assert!(outcome.usage);
        assert_eq!(outcome.code, 2);
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
