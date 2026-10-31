//! The agent seam's scenarios (B.4, 88 to 93, 115, 116, 121, 128, 134,
//! 135, 151, 161, 162, 169).

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;
use std::collections::BTreeMap;

pub trait AgentHarness {
    type Seam: Agent + ?Sized;

    fn seam(&self) -> &Self::Seam;

    fn place(&self) -> PlaceRef;

    /// The place's own folder beside its held state, where the agent's
    /// program records its session (128).
    fn folder(&self) -> String;

    /// The agent run with `command` under `identity`, as a terminal runs
    /// it, its session recorded in `folder`.
    fn start(&self, command: &AgentCommand, identity: &Identity, folder: &str);

    /// `text` delivered to the agent as input.
    fn deliver(&self, identity: &Identity, text: &str);

    /// The agent puts a question to a person.
    fn ask(&self, identity: &Identity);

    /// The agent starts own work that runs on without it.
    fn start_own_work(&self, identity: &Identity);

    /// The agent's own record can no longer be read.
    fn hide_record(&self, identity: &Identity);

    /// The agent tries to push its place's branch.
    fn try_push(&self, identity: &Identity);

    /// The agent's session ends.
    fn end_session(&self, identity: &Identity);

    /// The agent's last message is `words`.
    fn say_last(&self, identity: &Identity, words: &str);

    /// The agent's turn fails at its account's usage limit, with words the
    /// harness writes naming a reset; the reset by the host's clock (151).
    fn hit_usage_limit(&self, identity: &Identity) -> Seconds;

    /// The agent begins without the variable `variable` its start
    /// recorded, as the program's own word at its start shows it (161).
    fn begin_without(&self, identity: &Identity, variable: &str);

    /// The profile's map of modes, as its program's release has it or the
    /// harness scripted it (169).
    fn modes(&self) -> Modes;

    /// The agent begins in the mode `began` where its start asked
    /// `asked`, as the program's own word at its start shows it (161,
    /// 162).
    fn begin_in(&self, identity: &Identity, asked: &str, began: &str);
}

/// A profile's map of modes: the value each of flume's four it maps to,
/// none for one its program has no setting for, and its program's own
/// values, each passed as written (169).
#[derive(Clone, Debug, Default)]
pub struct Modes {
    pub four: BTreeMap<Mode, String>,
    pub own: Vec<String>,
}

fn identity(stage: &str) -> Identity {
    Identity {
        repository: "/repository".into(),
        place: "fix--one".into(),
        stage: stage.into(),
        start: 1,
    }
}

fn started<H: AgentHarness>(harness: &H, stage: &str) -> Identity {
    let who = identity(stage);
    let command = harness.seam().command(
        &Settings::default(),
        None,
        &BTreeMap::new(),
        &harness.place(),
        &who,
        &Secret("secret".into()),
        "begin",
    );
    harness.start(&command, &who, &harness.folder());
    who
}

// ---- the agent seam: a profile unit appends its own scenarios at the list's end ----

pub fn scenarios<H: AgentHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name: "agent: command starts the agent in its place under its identity and secret",
            cites: &["88", "29", "32", "33"],
            run: command_carries_identity_and_secret::<H>,
        },
        Scenario {
            name: "agent: received reads the prompt from the agent's own record",
            cites: &["89", "38"],
            run: received_reads_the_record::<H>,
        },
        Scenario {
            name: "agent: asking and own work are read from the record",
            cites: &["90", "91", "37", "31"],
            run: asking_and_own_work::<H>,
        },
        Scenario {
            name: "agent: what the record does not give is unknown",
            cites: &["93", "31"],
            run: unknown_when_the_record_cannot_say::<H>,
        },
        Scenario {
            name: "agent: the one way to stop short is given for every prompt, the same at every start of a stage",
            cites: &["92", "28", "41", "141"],
            run: the_way_to_stop_short::<H>,
        },
        Scenario {
            name: "agent: the guard refuses a push and reports it",
            cites: &["116", "108", "I3"],
            run: the_guard_reports::<H>,
        },
        Scenario {
            name: "agent: send-back, on-quiet and the mode read back say whether the program offers them",
            cites: &["115", "121", "110", "161"],
            run: what_the_program_offers::<H>,
        },
        Scenario {
            name: "agent: the readings read the start's record in the place's folder, and nothing in another",
            cites: &["128", "93", "77"],
            run: the_folder_handed_to_the_readings::<H>,
        },
        Scenario {
            name: "agent: ended reads whether the agent's record shows its session ended",
            cites: &["134", "93"],
            run: ended_reads_the_session::<H>,
        },
        Scenario {
            name: "agent: started reads the agent's own word that it started under its identity",
            cites: &["135", "35", "93", "128"],
            run: started_reads_the_agents_word::<H>,
        },
        Scenario {
            name: "agent: last words are read from the record, and none from another folder",
            cites: &["43", "93", "128"],
            run: last_words_from_the_record::<H>,
        },
        Scenario {
            name: "agent: spent reads a usage limit and its reset from the agent's own record, and nothing from another folder",
            cites: &["151", "146", "93", "128"],
            run: spent_reads_the_usage_limit::<H>,
        },
        Scenario {
            name: "agent: started without names each variable the agent began without, read from its own record, and nothing from another folder",
            cites: &["135", "161", "93", "128"],
            run: started_without_names_the_variable::<H>,
        },
        Scenario {
            name: "agent: an agent begun without its settings that has had no turn is read waiting on nothing, its own work not running",
            cites: &["91", "90", "31", "161"],
            run: begun_without_and_no_turn::<H>,
        },
        Scenario {
            name: "agent: mode maps each of flume's four and the program's own values as its program has them, and refuses any other naming it",
            cites: &["88", "169"],
            run: mode_maps_as_its_program_has_them::<H>,
        },
        Scenario {
            name: "agent: the command passes the mode mapped, and none where none is given",
            cites: &["88", "162", "169"],
            run: the_command_passes_the_mode::<H>,
        },
        Scenario {
            name: "agent: started without names the mode asked and the mode begun where the agent began in another, and no variable",
            cites: &["135", "161", "162"],
            run: started_without_names_the_mode::<H>,
        },
    ]
}

fn mode_maps_as_its_program_has_them<H: AgentHarness>(harness: &H) -> Outcome {
    let modes = harness.modes();
    for mode in Mode::ALL {
        match (
            harness.seam().mode(&ModeEntry::Flume(mode)),
            modes.four.get(&mode),
        ) {
            (Ok(value), Some(due)) => ensure_eq!(&value, due, format!("{mode} mapped")),
            (Err(words), None) => ensure!(
                words.contains(mode.word()),
                "the refusal of {mode} does not name it: {words}"
            ),
            (Ok(value), None) => {
                return Err(format!(
                    "{mode} mapped to {value}, where its program has no setting for it"
                ))
            }
            (Err(words), Some(due)) => {
                return Err(format!("{mode} refused, where it maps to {due}: {words}"))
            }
        }
    }
    for own in &modes.own {
        ensure_eq!(
            harness.seam().mode(&ModeEntry::Own(own.clone())),
            Ok::<String, String>(own.clone()),
            format!("the own value {own}")
        );
    }
    let other = "a-mode-of-no-program";
    match harness.seam().mode(&ModeEntry::Own(other.into())) {
        Ok(value) => Err(format!("{other} mapped to {value}")),
        Err(words) => {
            ensure!(
                words.contains(other),
                "the refusal of {other} does not name it: {words}"
            );
            Ok(())
        }
    }
}

fn the_command_passes_the_mode<H: AgentHarness>(harness: &H) -> Outcome {
    let modes = harness.modes();
    let who = identity("code");
    let command = |mode: Option<&str>| {
        harness.seam().command(
            &Settings::default(),
            mode,
            &BTreeMap::new(),
            &harness.place(),
            &who,
            &Secret("secret".into()),
            "begin",
        )
    };
    let none = command(None);
    for value in modes.four.values().chain(&modes.own) {
        ensure!(
            command(Some(value)).args.iter().any(|arg| arg == value),
            "the command passes no {value}"
        );
        ensure!(
            !none.args.iter().any(|arg| arg == value),
            "a command given no mode passes {value}"
        );
    }
    Ok(())
}

fn started_without_names_the_mode<H: AgentHarness>(harness: &H) -> Outcome {
    // A profile that reads no mode back compares none, so its agent
    // never reads begun in another (169).
    if harness.seam().mode_read_back() == Offered::NotOffered {
        return Ok(());
    }
    let modes = harness.modes();
    let (Some(asked), Some(began)) = (modes.four.get(&Mode::Full), modes.four.get(&Mode::Ask))
    else {
        return Ok(());
    };
    let who = identity("code");
    let folder = harness.folder();
    let command = harness.seam().command(
        &Settings::default(),
        Some(asked),
        &BTreeMap::new(),
        &harness.place(),
        &who,
        &Secret("secret".into()),
        "begin",
    );
    harness.start(&command, &who, &folder);
    ensure_eq!(
        harness.seam().started_without(&folder, &who),
        None,
        "an agent begun in the mode its start asked"
    );
    harness.begin_in(&who, asked, began);
    let Some(unsettled) = harness.seam().started_without(&folder, &who) else {
        return Err(format!(
            "an agent begun in {began} where its start asked {asked} read as begun as asked"
        ));
    };
    ensure_eq!(
        unsettled.mode,
        Some(ModeBegan {
            asked: asked.clone(),
            began: Some(began.clone()),
        }),
        "the modes"
    );
    ensure!(
        unsettled.missing.is_empty() && unsettled.different.is_empty() && !unsettled.program,
        "nothing else differed: {unsettled:?}"
    );
    ensure_eq!(
        harness
            .seam()
            .started_without(&format!("{folder}-of-another-place"), &who),
        None,
        "started without, read in another folder"
    );
    Ok(())
}

fn begun_without_and_no_turn<H: AgentHarness>(harness: &H) -> Outcome {
    // No start was made for this identity: the program's word at its start
    // is all it has said, and it sits at its prompt with no turn taken.
    let who = identity("verify");
    let folder = harness.folder();
    harness.begin_without(&who, "AGENT_HOME");
    ensure!(
        harness.seam().started_without(&folder, &who).is_some(),
        "the agent read begun with its settings"
    );
    ensure_eq!(
        harness.seam().own_work(&folder, &who),
        Reading::No,
        "own work of an agent that has had no turn"
    );
    ensure_eq!(
        harness.seam().asking(&folder, &who),
        Reading::No,
        "asking, of an agent that has had no turn"
    );
    Ok(())
}

fn started_without_names_the_variable<H: AgentHarness>(harness: &H) -> Outcome {
    let who = started(harness, "code");
    let folder = harness.folder();
    ensure_eq!(
        harness.seam().started_without(&folder, &who),
        None,
        "an agent begun as its start recorded"
    );
    harness.begin_without(&who, "AGENT_HOME");
    let Some(unsettled) = harness.seam().started_without(&folder, &who) else {
        return Err("an agent begun without AGENT_HOME read as begun with it".into());
    };
    ensure_eq!(
        unsettled.missing,
        vec!["AGENT_HOME".to_string()],
        "the variable missing"
    );
    ensure!(
        unsettled.different.is_empty() && !unsettled.program,
        "nothing else differed: {unsettled:?}"
    );
    ensure_eq!(
        harness
            .seam()
            .started_without(&format!("{folder}-of-another-place"), &who),
        None,
        "started without, read in another folder"
    );
    Ok(())
}

fn spent_reads_the_usage_limit<H: AgentHarness>(harness: &H) -> Outcome {
    let who = started(harness, "code");
    let folder = harness.folder();
    ensure!(
        !matches!(harness.seam().spent(&folder, &who), Spent::Yes { .. }),
        "an agent at work read spent"
    );
    let reset = harness.hit_usage_limit(&who);
    match harness.seam().spent(&folder, &who) {
        Spent::Yes { until, words } => {
            ensure_eq!(until, Some(reset), "the reset");
            ensure!(!words.trim().is_empty(), "a usage limit with no words");
        }
        other => return Err(format!("spent read {other:?} at a usage limit")),
    }
    ensure_eq!(
        harness
            .seam()
            .spent(&format!("{folder}-of-another-place"), &who),
        Spent::Unknown,
        "spent, read in another folder"
    );
    Ok(())
}

fn started_reads_the_agents_word<H: AgentHarness>(harness: &H) -> Outcome {
    let folder = harness.folder();
    ensure!(
        harness.seam().started(&folder, &identity("verify")) != Reading::Yes,
        "an agent never started read as started"
    );
    let who = started(harness, "code");
    ensure_eq!(
        harness.seam().started(&folder, &who),
        Reading::Yes,
        "started, once the agent said so"
    );
    ensure_eq!(
        harness
            .seam()
            .started(&format!("{folder}-of-another-place"), &who),
        Reading::Unknown,
        "started, read in another folder"
    );
    Ok(())
}

fn last_words_from_the_record<H: AgentHarness>(harness: &H) -> Outcome {
    let who = started(harness, "code");
    let folder = harness.folder();
    harness.say_last(&who, "I could not sign in");
    ensure_eq!(
        harness.seam().last_words(&folder, &who),
        Some("I could not sign in".to_string()),
        "the last words"
    );
    ensure_eq!(
        harness
            .seam()
            .last_words(&format!("{folder}-of-another-place"), &who),
        None,
        "the last words, read in another folder"
    );
    Ok(())
}

fn command_carries_identity_and_secret<H: AgentHarness>(harness: &H) -> Outcome {
    let who = identity("code");
    let secret = Secret("s3cret".into());
    let settings = Settings {
        model: Some("large".into()),
        ..Settings::default()
    };
    let env = BTreeMap::from([(
        "AGENT_HOME".to_string(),
        "/scratch/agent-work-1".to_string(),
    )]);
    let command = harness.seam().command(
        &settings,
        None,
        &env,
        &harness.place(),
        &who,
        &secret,
        "begin the stage",
    );
    let carries = |text: &str| {
        command.args.iter().any(|arg| arg.contains(text))
            || command.env.values().any(|value| value.contains(text))
    };
    ensure!(!command.program.is_empty(), "no program");
    ensure!(carries(&who.to_string()), "the command carries no identity");
    ensure!(carries(&secret.0), "the command carries no secret");
    ensure!(carries("large"), "the command carries no model");
    ensure_eq!(
        command.env.get("AGENT_HOME").map(String::as_str),
        Some("/scratch/agent-work-1"),
        "the account's variable in the command's environment"
    );
    ensure!(
        command
            .prompt_args
            .iter()
            .any(|arg| arg.contains("begin the stage")),
        "the command cannot carry its prompt"
    );
    Ok(())
}

fn received_reads_the_record<H: AgentHarness>(harness: &H) -> Outcome {
    let who = started(harness, "code");
    let folder = harness.folder();
    ensure!(
        !matches!(
            harness.seam().received(&folder, &who, "the prompt"),
            Received::Received { .. }
        ),
        "a prompt never sent read as received"
    );
    harness.deliver(&who, "the prompt");
    ensure!(
        matches!(
            harness.seam().received(&folder, &who, "the prompt"),
            Received::Received { .. }
        ),
        "a delivered prompt read as not received"
    );
    Ok(())
}

fn asking_and_own_work<H: AgentHarness>(harness: &H) -> Outcome {
    let who = started(harness, "code");
    let folder = harness.folder();
    ensure_eq!(harness.seam().asking(&folder, &who), Reading::No, "asking");
    ensure_eq!(
        harness.seam().own_work(&folder, &who),
        Reading::No,
        "own work"
    );
    harness.ask(&who);
    harness.start_own_work(&who);
    ensure_eq!(harness.seam().asking(&folder, &who), Reading::Yes, "asking");
    ensure_eq!(
        harness.seam().own_work(&folder, &who),
        Reading::Yes,
        "own work"
    );
    Ok(())
}

fn unknown_when_the_record_cannot_say<H: AgentHarness>(harness: &H) -> Outcome {
    let who = started(harness, "code");
    let folder = harness.folder();
    harness.hide_record(&who);
    ensure_eq!(
        harness.seam().own_work(&folder, &who),
        Reading::Unknown,
        "own work"
    );
    ensure_eq!(
        harness.seam().received(&folder, &who, "anything"),
        Received::Unknown,
        "received"
    );
    Ok(())
}

fn the_way_to_stop_short<H: AgentHarness>(harness: &H) -> Outcome {
    let first = identity("construct");
    let way = harness.seam().stop_short(&first);
    ensure!(!way.trim().is_empty(), "no way to stop short");
    let second = Identity { start: 2, ..first };
    ensure_eq!(
        harness.seam().stop_short(&second),
        way,
        "the way to stop short at the stage's second start"
    );
    Ok(())
}

fn the_guard_reports<H: AgentHarness>(harness: &H) -> Outcome {
    let who = started(harness, "code");
    harness.try_push(&who);
    match harness.seam().guard(&harness.folder(), &who) {
        Guarded::Refused { acts } => {
            ensure!(!acts.is_empty(), "a refused push is not reported");
            Ok(())
        }
        Guarded::NotOffered => Ok(()),
    }
}

fn what_the_program_offers<H: AgentHarness>(harness: &H) -> Outcome {
    let offered = [
        harness.seam().send_back(),
        harness.seam().on_quiet(),
        harness.seam().mode_read_back(),
    ];
    ensure!(
        offered
            .iter()
            .all(|answer| matches!(answer, Offered::Offered | Offered::NotOffered)),
        "an answer neither offered nor not"
    );
    Ok(())
}

fn the_folder_handed_to_the_readings<H: AgentHarness>(harness: &H) -> Outcome {
    let who = started(harness, "code");
    let folder = harness.folder();
    let elsewhere = format!("{folder}-of-another-place");
    harness.deliver(&who, "the prompt");
    ensure!(
        matches!(
            harness.seam().received(&folder, &who, "the prompt"),
            Received::Received { .. }
        ),
        "the prompt delivered read as not received in the place's folder"
    );
    ensure_eq!(
        harness.seam().received(&elsewhere, &who, "the prompt"),
        Received::Unknown,
        "received, read in another folder"
    );
    ensure_eq!(
        harness.seam().asking(&elsewhere, &who),
        Reading::Unknown,
        "asking, read in another folder"
    );
    ensure_eq!(
        harness.seam().own_work(&elsewhere, &who),
        Reading::Unknown,
        "own work, read in another folder"
    );
    ensure_eq!(
        harness.seam().ended(&elsewhere, &who),
        Reading::Unknown,
        "ended, read in another folder"
    );
    Ok(())
}

fn ended_reads_the_session<H: AgentHarness>(harness: &H) -> Outcome {
    let who = started(harness, "code");
    let folder = harness.folder();
    ensure_eq!(
        harness.seam().ended(&folder, &who),
        Reading::No,
        "ended, while the session runs"
    );
    harness.end_session(&who);
    ensure_eq!(
        harness.seam().ended(&folder, &who),
        Reading::Yes,
        "ended, once the session ended"
    );
    Ok(())
}
