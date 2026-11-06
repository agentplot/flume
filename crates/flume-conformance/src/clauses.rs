//! The statement's clauses, invariants and scenarios, as this suite read
//! them at the statement's revision. A clause the statement adds or
//! removes changes this list in the commit that adopts it.

/// The revision of `design/flume/requirements.md` in agentplot/blueprints
/// the list was read at.
pub const REVISION: &str = "ff616eb";

/// Part A: what flume decides.
pub fn part_a() -> Vec<u32> {
    (1..=77)
        .chain(107..=114)
        .chain(118..=120)
        .chain([123, 124])
        .chain(129..=132)
        .chain([137])
        .chain(140..=149)
        .chain(154..=159)
        .chain(161..=163)
        .chain(165..=173)
        .chain(175..=177)
        .chain(180..=186)
        .chain([190, 191])
        .collect()
}

/// Part B: the contracts across the seams.
pub fn part_b() -> Vec<u32> {
    (78..=99)
        .chain(101..=106)
        .chain(115..=117)
        .chain([121, 122])
        .chain(125..=128)
        .chain(133..=136)
        .chain([138, 139])
        .chain(151..=153)
        .chain([160, 164, 174, 178, 179])
        .chain(187..=189)
        .chain([192])
        .collect()
}

/// Every clause, invariant and scenario a scenario of the suite may
/// cite, each as cited: `70`, `I3`, `S12`.
pub fn all() -> Vec<String> {
    let mut all: Vec<String> = part_a()
        .into_iter()
        .chain(part_b())
        .map(|clause| clause.to_string())
        .collect();
    all.extend((1..=8).map(|invariant| format!("I{invariant}")));
    all.extend(
        (1..=SCENARIOS)
            .filter(|scenario| !SCENARIO_GAP.contains(scenario))
            .map(|scenario| format!("S{scenario}")),
    );
    all
}

/// The last scenario of section 11.
pub const SCENARIOS: u32 = 97;

/// The numbers section 11 skips: the revision holds no S92 to S94.
pub const SCENARIO_GAP: std::ops::RangeInclusive<u32> = 92..=94;

/// The change that builds record 0021: the standing prompt in layers and
/// cache use in status.
pub const STANDING: &str = "record 0021's change, the standing prompt, once its plan is approved";

/// The change that builds record 0024's plan stage, after its mode.
pub const PLAN: &str = "record 0024's plan stage's change, built after the mode";

/// The change that builds record 0025's points, a stream's points bound
/// and its pace.
pub const POINTS: &str = "record 0025's change, points, built after the mode";

/// The change that builds record 0026's common profile and the degraded
/// parts of the agent seam.
pub const PROFILES: &str = "record 0026's change, every program the terminal starts, once accepted";

/// The change that builds records 0028 and 0029: statechart kinds, run
/// on the family's engine moved into weir.
pub const STATECHART: &str =
    "records 0028 and 0029's change, statechart kinds on the family's engine, after crew's rewrite";

/// The change that builds records 0031 to 0033: each stream's own
/// session, a kind's label and a stream's reserves and slots, and the
/// host's local layer. It and the three below landed on main after this
/// branch last took main, so the merge that brings them takes main's
/// list for their clauses.
pub const SESSIONS: &str =
    "records 0031 to 0033's change, each stream's own session and labels and the host's local layer";

/// What builds clause 183, an MCP server naming the labels whose places
/// may have it: the answer to section 10's question on how a host's
/// servers reach an agent.
pub const MCP_SERVERS: &str =
    "the answer to section 10's question on how a host's MCP servers reach an agent";

/// The change that builds record 0035: a pool choosing the account with
/// the most of its allowance left by the host's reading, and a pool's
/// threshold past which an account takes no new start.
pub const MOST_LEFT: &str = "record 0035's change, a pool choosing by the allowance left";

/// The change that builds record 0036: a place made ready by its merge
/// queue's equipping of it.
pub const READINESS: &str = "record 0036's change, a place equipped by its merge queue";

/// The change that builds record 0037: each stream's multiplexer, view
/// and terminal app, and the window a person opens its agents in.
pub const MULTIPLEXER: &str =
    "record 0037's change, each stream's multiplexer, view and terminal app";

/// Each clause or scenario no change has built yet, with the change that
/// builds it: no scenario of the suite cites one, and the coverage check
/// asks none to be cited.
pub fn unbuilt() -> Vec<(String, &'static str)> {
    let standing = [148, 149, 152]
        .map(|clause| clause.to_string())
        .into_iter()
        .chain((57..=58).map(|scenario| format!("S{scenario}")))
        .map(|clause| (clause, STANDING));
    let plan = [163, 164]
        .map(|clause| clause.to_string())
        .into_iter()
        .chain(["S68".to_string()])
        .map(|clause| (clause, PLAN));
    let points = (165..=168)
        .map(|clause| clause.to_string())
        .chain((69..=70).map(|scenario| format!("S{scenario}")))
        .map(|clause| (clause, POINTS));
    let profiles = (170..=174)
        .map(|clause| clause.to_string())
        .chain((71..=75).map(|scenario| format!("S{scenario}")))
        .map(|clause| (clause, PROFILES));
    let statechart = (175..=178)
        .map(|clause| clause.to_string())
        .chain((76..=77).map(|scenario| format!("S{scenario}")))
        .map(|clause| (clause, STATECHART));
    let sessions = (180..=182)
        .map(|clause| clause.to_string())
        .chain((82..=85).map(|scenario| format!("S{scenario}")))
        .map(|clause| (clause, SESSIONS));
    let mcp_servers = std::iter::once(("183".to_string(), MCP_SERVERS));
    let most_left = [184, 185]
        .map(|clause| clause.to_string())
        .into_iter()
        .chain((86..=88).map(|scenario| format!("S{scenario}")))
        .map(|clause| (clause, MOST_LEFT));
    let readiness = std::iter::once(("S89".to_string(), READINESS));
    let multiplexer = (186..=190)
        .map(|clause| clause.to_string())
        .chain((90..=91).map(|scenario| format!("S{scenario}")))
        .map(|clause| (clause, MULTIPLEXER));
    standing
        .chain(plan)
        .chain(points)
        .chain(profiles)
        .chain(statechart)
        .chain(sessions)
        .chain(mcp_servers)
        .chain(most_left)
        .chain(readiness)
        .chain(multiplexer)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_clauses_listed() {
        let a = part_a();
        let b = part_b();
        assert_eq!(a.len(), 77 + 8 + 3 + 2 + 4 + 1 + 10 + 6 + 3 + 9 + 3 + 7 + 2);
        assert_eq!(b.len(), 22 + 6 + 3 + 2 + 4 + 4 + 2 + 3 + 5 + 3 + 1);
        for gap in [100, 150] {
            assert!(!a.contains(&gap) && !b.contains(&gap), "{gap} is listed");
        }
        assert!(a.iter().all(|clause| !b.contains(clause)));
        let mut every: Vec<u32> = a.into_iter().chain(b).collect();
        every.sort();
        assert_eq!(
            every,
            (1..=192)
                .filter(|clause| ![100, 150].contains(clause))
                .collect::<Vec<_>>()
        );
        assert_eq!(all().len(), 190 + 8 + 94);
        assert!(all().contains(&"S64".to_string()), "S64 is not listed");
        for gap in ["S92", "S93", "S94"] {
            assert!(!all().contains(&gap.to_string()), "{gap} is listed");
        }
        for scenario in ["S91", "S95", "S96", "S97"] {
            assert!(
                all().contains(&scenario.to_string()),
                "{scenario} is not listed"
            );
        }
        assert_eq!(REVISION, "ff616eb");
        for clause in [
            129, 130, 131, 132, 137, 140, 141, 142, 148, 154, 155, 156, 157, 158, 159, 161, 162,
            163, 165, 168, 169, 173, 175, 176, 177, 180, 183, 186, 190, 191,
        ] {
            assert!(part_a().contains(&clause), "{clause} is not in Part A");
        }
        for clause in [
            126, 127, 128, 133, 134, 135, 136, 138, 139, 151, 153, 160, 164, 174, 178, 179, 187,
            189, 192,
        ] {
            assert!(part_b().contains(&clause), "{clause} is not in Part B");
        }
    }

    #[test]
    fn the_clauses_unbuilt_are_records_0021_0024s_plan_0025_0026_0028_0029_0031_to_0033_0035_to_0037s_and_183(
    ) {
        let unbuilt = unbuilt();
        assert_eq!(unbuilt.len(), 3 + 2 + 3 + 6 + 10 + 6 + 7 + 1 + 5 + 1 + 7);
        assert!(unbuilt.iter().all(|(clause, _)| all().contains(clause)));
        for built in [
            "140", "141", "142", "143", "144", "145", "146", "147", "151", "153", "154", "157",
            "161", "162", "169", "179", "S50", "S51", "S52", "S53", "S54", "S55", "S56", "S64",
            "S65", "S66", "S67", "S78", "S79", "S80", "S81", "155", "156", "158", "159", "160",
            "191", "192", "S59", "S60", "S61", "S62", "S63", "S95", "S96", "S97",
        ] {
            assert!(
                unbuilt.iter().all(|(clause, _)| clause != built),
                "{built} is named unbuilt"
            );
        }
    }
}
