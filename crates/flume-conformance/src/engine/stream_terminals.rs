//! A stream's terminal and the window it is viewed in, given by name
//! (105, 190; core.49): the resolver gives a terminal profile and a
//! window profile by name, none for a name it carries none for, and
//! lists the names it carries, the one the host's multiplexer names
//! being the very terminal the core is built with.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_fakes::{FakeTerminal, FakeWindow};
use flume_seams::*;
use std::sync::Arc;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![Scenario {
        name: "stream terminals: the resolver gives terminals and windows by name, the core's own among them, and none for a name it carries none for",
        cites: &["105", "190"],
        run: named_terminals_and_windows,
    }]
}

/// Whether `given` is the very fake `fake`.
fn is<T: ?Sized, F>(given: &Arc<T>, fake: &Arc<F>) -> bool {
    std::ptr::addr_eq(Arc::as_ptr(given), Arc::as_ptr(fake))
}

fn named_terminals_and_windows(_: &()) -> Outcome {
    let world = World::new();
    let other = Arc::new(FakeTerminal::new(world.host.clone()));
    let other_window = Arc::new(FakeWindow::default());
    world.resolver.with_terminal("other", other.clone());
    world
        .resolver
        .with_window("other-window", other_window.clone());
    let resolver = &world.resolver;
    ensure!(
        resolver
            .terminal("fake")
            .is_some_and(|terminal| is(&terminal, &world.terminal)),
        "fake is not the terminal the core is built with"
    );
    ensure!(
        resolver
            .terminal("other")
            .is_some_and(|terminal| is(&terminal, &other)),
        "other is not the terminal given"
    );
    ensure!(resolver.terminal("rex").is_none(), "a terminal named rex");
    ensure!(
        resolver
            .window("fake-window")
            .is_some_and(|window| is(&window, &world.window)),
        "fake-window is not the fake window"
    );
    ensure!(resolver.window("kitty").is_none(), "a window named kitty");
    ensure_eq!(
        resolver.terminal_profiles(),
        vec!["fake".to_string(), "other".to_string()],
        "the terminal profiles"
    );
    ensure_eq!(
        resolver.window_profiles(),
        vec!["fake-window".to_string(), "other-window".to_string()],
        "the window profiles"
    );
    Ok(())
}
