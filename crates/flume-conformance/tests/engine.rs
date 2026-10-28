//! The engine's scenarios over the fakes.

use flume_conformance::engine;
use flume_conformance::scenario::pass_all;

#[test]
fn the_engine_passes_its_scenarios() {
    pass_all("engine", &engine::scenarios(), || ());
}
