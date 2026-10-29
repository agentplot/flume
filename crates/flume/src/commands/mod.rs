//! flume's commands (section 9; design 16; command-line spec): each
//! builds the core for itself, asks it one act, and prints its answer for
//! a person or as its JSON with `--json`, exiting 1 on a refusal (60, 61).
//! No command computes a stage's end, a status, a submission or a tidy of
//! its own (core.14), and none names a tool (core.1).

mod launch;
pub mod place;
mod print;

use clap::{Args, Parser, Subcommand};
use flume_core::{Answer, Engine, Retry};
use flume_seams::{AgentState, Launch, PlaceId, Reading, Seconds, StopKind};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

/// The variable an agent flume started is named in (33, C.4).
pub const AGENT: &str = "FLUME_AGENT";

/// The variable an agent's start's secret is in (41, C.4).
pub const SECRET: &str = "FLUME_SECRET";

/// How long `settle --on-quiet` reads the agent for until it is quiet
/// (121; design 14).
pub const ON_QUIET_WITHIN: Seconds = 60;

#[derive(Debug, Parser)]
#[command(
    name = "flume",
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
    Launch(LaunchArgs),
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
    /// One setting of every stage's agent, or of one stage's (32): the
    /// field one of program, model, instructions, env.<NAME> and beside.
    #[arg(long, value_name = "[STAGE.]FIELD=VALUE")]
    agent: Vec<String>,
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
        } else {
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
    let answer = act(&cli, build).unwrap_or_else(Answer::refused);
    let text = if cli.json {
        answer.to_json()
    } else {
        print::person(&answer)
    };
    Outcome {
        text,
        usage: false,
        code: u8::from(answer.refusal().is_some()),
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
