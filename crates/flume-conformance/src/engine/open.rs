//! A person opens a stream's agents in its view (186 to 189; core.48):
//! the stream's own session, resolved as a start resolves its stream's,
//! shown through the stream's multiplexer in its view, the stream's else
//! the host's: for a terminal view, the terminal's attach for a window run
//! by the stream's terminal app under the stream's title, else its name;
//! for the app, the terminal's attach for its app alone. An open takes no
//! lock, writes no held state and settles nothing, a second asking opens
//! a second window, and no launch, settle, start or tidy asks for one; an
//! undeclared stream, a multiplexer or a terminal app naming no profile,
//! and a terminal's or a window's refusal refuse it; and its answer names
//! the command's variables, never their values (4, 107, 120, 186 to 190;
//! core.17, core.30).

use super::settling::{archive, construct_begun, fix_begun, write_change};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, StreamOpened, TitleFrom, TitleHeld, WindowOutcome};
use flume_fakes::FakeWindow;
use flume_seams::*;
use std::sync::Arc;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "open: a stream's window runs the terminal's attach of the stream's session under the stream's title, the answer giving each with where it came from",
            cites: &["186", "S90"],
            run: a_streams_window,
        },
        Scenario {
            name: "open: a stream that names its session and no title has that session attached under the stream's name",
            cites: &["107", "143"],
            run: a_stream_that_names_its_session_and_no_title,
        },
        Scenario {
            name: "open: the default stream naming no session has the terminal's own session attached under the title default",
            cites: &["107", "S90"],
            run: the_default_stream,
        },
        Scenario {
            name: "open: a stream under a multiplexer of its own has the same session and terminal attached as its stage's agent opened in",
            cites: &["107", "190", "S90"],
            run: the_same_session_and_terminal_as_a_stages_agent,
        },
        Scenario {
            name: "open: a stream naming its own terminal app is shown by it, and a stream naming none by the host's",
            cites: &["186", "188", "S90"],
            run: a_streams_own_terminal_app,
        },
        Scenario {
            name: "open: a stream whose view is the app has the terminal's app show its session, no title given, no terminal app read and no window asked",
            cites: &["186", "187"],
            run: the_app_shows_the_session,
        },
        Scenario {
            name: "open: a terminal that offers no app refuses an open of a stream whose view is the app in its own words, and no window is asked",
            cites: &["187", "S90"],
            run: a_terminal_with_no_app,
        },
        Scenario {
            name: "open: two askings open two windows, and no lock, held state or settle is touched",
            cites: &["186", "189", "S90"],
            run: two_askings,
        },
        Scenario {
            name: "open: a change run through from its launch to its landing and its tidy asks no window and no terminal's attach",
            cites: &["186"],
            run: a_place_run_through,
        },
        Scenario {
            name: "open: a stream the host does not declare is refused naming it and the streams the host declares, nothing asked of the terminal or the window",
            cites: &["4", "186"],
            run: an_undeclared_stream,
        },
        Scenario {
            name: "open: a stream whose multiplexer names rex is refused naming rex and the terminal profiles carried, nothing attached",
            cites: &["4", "190", "S91"],
            run: a_multiplexer_no_profile_carries,
        },
        Scenario {
            name: "open: a terminal that holds no session refuses the open in its own words, and no window is asked",
            cites: &["187"],
            run: a_terminal_that_holds_no_session,
        },
        Scenario {
            name: "open: a stream whose terminal app names kitty is refused naming kitty and the window profiles carried, before any attach",
            cites: &["4", "186", "188"],
            run: a_terminal_app_no_profile_carries,
        },
        Scenario {
            name: "open: a window that refuses refuses the open in its words",
            cites: &["188"],
            run: a_window_that_refuses,
        },
        Scenario {
            name: "open: a command that needs a variable is answered by the variable's name alone, with the printed line",
            cites: &["186", "56"],
            run: a_command_that_needs_a_variable,
        },
        Scenario {
            name: "open: S90, a stream's view opened before its places run and again, another stream's own terminal app, the default stream's own session, an app refused in the terminal's words, and the line printed where no window opens",
            cites: &["S90", "107", "143", "186", "187", "188", "189"],
            run: s90_a_streams_view,
        },
    ]
}

/// What the open answered, or why it was not opened.
fn opened(answer: &Answer) -> Result<StreamOpened, String> {
    match answer {
        Answer::Opened(opened) => Ok(opened.clone()),
        other => Err(format!("the open answered {other:?}")),
    }
}

fn profile(name: &str, from: NamedBy) -> ProfileHeld {
    ProfileHeld {
        profile: name.into(),
        from,
    }
}

fn session(name: Option<&str>, from: SessionFrom) -> SessionHeld {
    SessionHeld {
        name: name.map(str::to_string),
        from,
    }
}

fn title(title: &str, from: TitleFrom) -> Option<TitleHeld> {
    Some(TitleHeld {
        title: title.into(),
        from,
    })
}

/// That the open of `stream` was refused with words holding each of
/// `words`, and asked nothing of the terminal's attach or the window.
fn refused_with(world: &World, stream: Option<&str>, words: &[&str], attached: usize) -> Outcome {
    let reason = refused(&world.engine.open_stream(stream))?;
    for word in words {
        ensure!(
            reason.contains(word),
            "the refusal {reason:?} names no {word}"
        );
    }
    ensure_eq!(
        world.terminal.log.count("attach"),
        attached,
        "the terminal's attaches"
    );
    ensure_eq!(world.window.log.count("show"), 0, "the window's shows");
    Ok(())
}

fn a_streams_window(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_title("work", Some("Work Agents"));
    let answer = opened(&world.engine.open_stream(Some("work")))?;
    ensure_eq!(
        world.terminal.log.of("attach"),
        vec!["attach work for terminal".to_string()],
        "the terminal's attaches"
    );
    ensure_eq!(
        world.window.log.of("show"),
        vec!["show fake-terminal attach work titled Work Agents".to_string()],
        "the window's shows"
    );
    ensure_eq!(
        answer.session,
        session(Some("work"), SessionFrom::StreamName),
        "the session"
    );
    ensure_eq!(
        answer.view,
        ViewHeld {
            view: View::Terminal,
            from: NamedBy::Neither,
        },
        "the view"
    );
    ensure_eq!(
        answer.title,
        title("Work Agents", TitleFrom::Stream),
        "the title"
    );
    ensure_eq!(
        answer.multiplexer,
        profile("fake", NamedBy::Neither),
        "the multiplexer"
    );
    ensure_eq!(
        answer.terminal_app,
        Some(profile("fake-window", NamedBy::Neither)),
        "the terminal app"
    );
    ensure_eq!(answer.window, WindowOutcome::Opened, "the window");
    Ok(())
}

fn a_stream_that_names_its_session_and_no_title(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream_session("work", Some("office"));
    let answer = opened(&world.engine.open_stream(Some("work")))?;
    ensure_eq!(
        world.terminal.log.of("attach"),
        vec!["attach office for terminal".to_string()],
        "the terminal's attaches"
    );
    ensure_eq!(
        answer.session,
        session(Some("office"), SessionFrom::Stream),
        "the session"
    );
    ensure_eq!(
        answer.title,
        title("work", TitleFrom::StreamName),
        "the title"
    );
    ensure_eq!(
        world
            .window
            .windows()
            .into_iter()
            .map(|(_, title)| title)
            .collect::<Vec<_>>(),
        vec!["work".to_string()],
        "the windows' titles"
    );
    Ok(())
}

fn the_default_stream(_: &()) -> Outcome {
    let world = World::new();
    let answer = opened(&world.engine.open_stream(None))?;
    ensure_eq!(
        world.terminal.log.of("attach"),
        vec!["attach own for terminal".to_string()],
        "the terminal's attaches"
    );
    ensure_eq!(answer.stream, DEFAULT_STREAM, "the stream");
    ensure_eq!(
        answer.session,
        session(None, SessionFrom::Own),
        "the session"
    );
    ensure_eq!(
        answer.title,
        title(DEFAULT_STREAM, TitleFrom::StreamName),
        "the title"
    );
    Ok(())
}

fn the_same_session_and_terminal_as_a_stages_agent(_: &()) -> Outcome {
    let world = World::new();
    let other = world.other_terminal("other");
    world.host.set_display("trial", Some("other"), None, None);
    launched(&world.engine.launch(&Launch {
        stream: Some("trial".into()),
        ..launch("fix", "one", Some("x"))
    }))?;
    let agent = World::identity("fix", "one", "code", 1);
    ensure_eq!(
        other.opened_at(&agent),
        Some(OpenIn::Session {
            name: "trial".into()
        }),
        "where the fix's agent opened in other"
    );
    let answer = opened(&world.engine.open_stream(Some("trial")))?;
    ensure_eq!(
        other.log.of("attach"),
        vec!["attach trial for terminal".to_string()],
        "other's attaches"
    );
    ensure_eq!(
        world.terminal.log.count("attach"),
        0,
        "the host's own terminal's attaches"
    );
    ensure_eq!(
        answer.multiplexer,
        profile("other", NamedBy::Stream),
        "the multiplexer"
    );
    ensure_eq!(
        answer.session,
        session(Some("trial"), SessionFrom::StreamName),
        "the session"
    );
    Ok(())
}

fn a_streams_own_terminal_app(_: &()) -> Outcome {
    let world = World::new();
    let other_window = Arc::new(FakeWindow::default());
    world
        .resolver
        .with_window("other-window", other_window.clone());
    world.host.set_host_display(None, None, Some("fake-window"));
    world
        .host
        .set_display("office", None, None, Some("other-window"));
    world.host.set_stream("work", Some(Stream::default()));
    let office = opened(&world.engine.open_stream(Some("office")))?;
    let work = opened(&world.engine.open_stream(Some("work")))?;
    ensure_eq!(
        other_window.log.of("show"),
        vec!["show fake-terminal attach office titled office".to_string()],
        "other-window's shows"
    );
    ensure_eq!(
        world.window.log.of("show"),
        vec!["show fake-terminal attach work titled work".to_string()],
        "fake-window's shows"
    );
    ensure_eq!(
        office.terminal_app,
        Some(profile("other-window", NamedBy::Stream)),
        "office's terminal app"
    );
    ensure_eq!(
        work.terminal_app,
        Some(profile("fake-window", NamedBy::Host)),
        "work's terminal app"
    );
    Ok(())
}

fn the_app_shows_the_session(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_display("work", None, Some(View::App), None);
    let answer = opened(&world.engine.open_stream(Some("work")))?;
    ensure_eq!(
        world.terminal.log.of("attach"),
        vec!["attach work for app".to_string()],
        "the terminal's attaches"
    );
    ensure_eq!(
        answer.view,
        ViewHeld {
            view: View::App,
            from: NamedBy::Stream,
        },
        "the view"
    );
    ensure_eq!(answer.title, None, "the title");
    ensure_eq!(answer.terminal_app, None, "the terminal app");
    ensure_eq!(answer.command, None, "the command");
    ensure_eq!(answer.window, WindowOutcome::App, "the window");
    ensure_eq!(
        world.window.log.calls(),
        Vec::<String>::new(),
        "the window's log"
    );
    Ok(())
}

fn a_terminal_with_no_app(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_display("work", None, Some(View::App), None);
    world.terminal.attaches_as(
        View::App,
        Some(Attached::Refused {
            reason: "no app of its own".into(),
        }),
    );
    ensure_eq!(
        refused(&world.engine.open_stream(Some("work")))?,
        "no app of its own",
        "the refusal"
    );
    ensure_eq!(
        world.window.log.calls(),
        Vec::<String>::new(),
        "the window's log"
    );
    Ok(())
}

fn two_askings(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    let id = fix_begun(&world, "one")?;
    let before = world.held(&id);
    let places = world.store.places();
    world.host.log.clear();
    world.store.log.clear();
    world.terminal.log.clear();
    for _ in 0..2 {
        opened(&world.engine.open_stream(Some("work")))?;
    }
    ensure_eq!(world.window.log.count("show"), 2, "the window's shows");
    ensure_eq!(world.window.windows().len(), 2, "the windows opened");
    ensure_eq!(world.held(&id), before, "the place's held state");
    ensure_eq!(world.store.places(), places, "the places the store holds");
    ensure_eq!(
        world.store.log.calls(),
        Vec::<String>::new(),
        "the store's log"
    );
    ensure_eq!(world.host.log.count("room"), 0, "the room lock taken");
    ensure_eq!(
        world.terminal.log.of("attach").len(),
        world.terminal.log.calls().len(),
        "the terminal's calls other than attach"
    );
    Ok(())
}

fn a_place_run_through(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    write_change(&world, "plan");
    world.settle(&id);
    world.tick("change/plan", "plan", 2, 2);
    world.settle(&id);
    archive(&world, "plan");
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the place did not land and tidy");
    ensure_eq!(world.terminal.log.count("open"), 3, "agents opened");
    ensure_eq!(
        world.window.log.calls(),
        Vec::<String>::new(),
        "the window's log"
    );
    ensure_eq!(
        world.terminal.log.count("attach"),
        0,
        "the terminal's attaches"
    );
    Ok(())
}

fn an_undeclared_stream(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    refused_with(
        &world,
        Some("nightly"),
        &["nightly", "its streams are: default, work"],
        0,
    )
}

fn a_multiplexer_no_profile_carries(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_display("trial", Some("rex"), None, None);
    refused_with(
        &world,
        Some("trial"),
        &["trial", "rex", "the terminal profiles there are: fake"],
        0,
    )
}

fn a_terminal_that_holds_no_session(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    world.terminal.attaches_as(
        View::Terminal,
        Some(Attached::Refused {
            reason: "no session here".into(),
        }),
    );
    ensure_eq!(
        refused(&world.engine.open_stream(Some("work")))?,
        "no session here",
        "the refusal"
    );
    ensure_eq!(world.window.log.count("show"), 0, "the window's shows");
    Ok(())
}

fn a_terminal_app_no_profile_carries(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_display("work", None, None, Some("kitty"));
    refused_with(
        &world,
        Some("work"),
        &[
            "work",
            "kitty",
            "the window profiles there are: fake-window",
        ],
        0,
    )
}

fn a_window_that_refuses(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    world.window.answers(Some(WindowShown::Refused {
        reason: "no display".into(),
    }));
    ensure_eq!(
        refused(&world.engine.open_stream(Some("work")))?,
        "no display",
        "the refusal"
    );
    Ok(())
}

fn a_command_that_needs_a_variable(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    world.terminal.attaches_as(
        View::Terminal,
        Some(Attached::Command {
            command: ViewCommand {
                program: "/bin/fake-terminal".into(),
                arguments: vec!["attach".into(), "work".into()],
                environment: [("XDG_CONFIG_HOME".to_string(), "/scratch/home".to_string())].into(),
            },
        }),
    );
    let line = "XDG_CONFIG_HOME=<set> /bin/fake-terminal attach work";
    world
        .window
        .answers(Some(WindowShown::Printed { line: line.into() }));
    let answer = world.engine.open_stream(Some("work"));
    let shown = opened(&answer)?;
    let command = shown.command.ok_or("no command in the answer")?;
    ensure_eq!(
        command.environment,
        vec!["XDG_CONFIG_HOME".to_string()],
        "the command's variables"
    );
    ensure_eq!(
        command.program,
        "/bin/fake-terminal",
        "the command's program"
    );
    ensure_eq!(
        command.arguments,
        vec!["attach".to_string(), "work".to_string()],
        "the command's arguments"
    );
    ensure_eq!(
        shown.window,
        WindowOutcome::Printed { line: line.into() },
        "the window"
    );
    let json = serde_json::to_string(&answer).map_err(|error| error.to_string())?;
    ensure!(
        json.contains("XDG_CONFIG_HOME"),
        "the JSON names no variable"
    );
    ensure!(
        !json.contains("/scratch/home"),
        "the JSON holds the variable's value: {json}"
    );
    Ok(())
}

fn s90_a_streams_view(_: &()) -> Outcome {
    let world = World::new();
    let office_window = Arc::new(FakeWindow::default());
    world
        .resolver
        .with_window("office-window", office_window.clone());
    world.host.set_host_display(None, None, Some("fake-window"));
    world.host.set_title("work", Some("Work Agents"));
    world
        .host
        .set_display("office", None, None, Some("office-window"));
    world.host.set_title("office", Some("Office Agents"));
    world
        .host
        .set_display("tablet", None, Some(View::App), None);

    let first = opened(&world.engine.open_stream(Some("work")))?;
    ensure_eq!(
        first.session,
        session(Some("work"), SessionFrom::StreamName),
        "work's session"
    );
    launched(&world.engine.launch(&Launch {
        stream: Some("work".into()),
        ..launch("fix", "one", Some("x"))
    }))?;
    ensure_eq!(
        world
            .terminal
            .opened_at(&World::identity("fix", "one", "code", 1)),
        Some(OpenIn::Session {
            name: "work".into()
        }),
        "where the fix on work opened its agent"
    );
    opened(&world.engine.open_stream(Some("work")))?;
    ensure_eq!(
        world.window.windows(),
        vec![
            (
                ViewCommand {
                    program: "fake-terminal".into(),
                    arguments: vec!["attach".into(), "work".into()],
                    environment: Default::default(),
                },
                "Work Agents".to_string(),
            );
            2
        ],
        "work's windows"
    );

    let office = opened(&world.engine.open_stream(Some("office")))?;
    ensure_eq!(
        office_window.log.of("show"),
        vec!["show fake-terminal attach office titled Office Agents".to_string()],
        "office's window"
    );
    ensure_eq!(
        office.title,
        title("Office Agents", TitleFrom::Stream),
        "office's title"
    );

    let default = opened(&world.engine.open_stream(None))?;
    ensure_eq!(
        default.session,
        session(None, SessionFrom::Own),
        "the default stream's session"
    );
    ensure_eq!(
        world.window.log.of("show").last().cloned(),
        Some("show fake-terminal attach own titled default".to_string()),
        "the default stream's window"
    );

    world.terminal.attaches_as(
        View::App,
        Some(Attached::Refused {
            reason: "the terminal has no app of its own".into(),
        }),
    );
    let shows = world.window.log.count("show") + office_window.log.count("show");
    ensure_eq!(
        refused(&world.engine.open_stream(Some("tablet")))?,
        "the terminal has no app of its own",
        "tablet's refusal"
    );
    ensure_eq!(
        world.window.log.count("show") + office_window.log.count("show"),
        shows,
        "the windows asked for tablet"
    );

    world.host.set_host_display(None, None, None);
    world.window.answers(Some(WindowShown::Printed {
        line: "fake-terminal attach work".into(),
    }));
    let printed = opened(&world.engine.open_stream(Some("work")))?;
    ensure_eq!(
        printed.terminal_app,
        Some(profile("fake-window", NamedBy::Neither)),
        "work's terminal app where the host names none"
    );
    ensure_eq!(
        printed.window,
        WindowOutcome::Printed {
            line: "fake-terminal attach work".into(),
        },
        "work's view where no window opens"
    );
    Ok(())
}
