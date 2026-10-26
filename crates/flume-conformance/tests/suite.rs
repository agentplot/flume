//! The suite's own scenarios, which hold the suite to the statement.

use flume_conformance::scenario::pass_all;
use flume_conformance::suite;

#[test]
fn the_suites_own_scenarios() {
    pass_all("suite", &suite::scenarios(), || ());
}
