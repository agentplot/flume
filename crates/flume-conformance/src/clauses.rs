//! The statement's clauses, invariants and scenarios, as this suite read
//! them at the statement's revision. A clause the statement adds or
//! removes changes this list in the commit that adopts it.

/// The revision of `design/flume/requirements.md` in agentplot/blueprints
/// the list was read at.
pub const REVISION: &str = "7bd1891";

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
        .chain([190, 191, 195, 197, 200])
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
        .chain([192, 196, 201])
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
            .filter(|scenario| !SKIPPED.contains(scenario))
            .map(|scenario| format!("S{scenario}")),
    );
    all
}

/// The last scenario of section 11.
pub const SCENARIOS: u32 = 108;

/// The numbers section 11 skips at the revision, which designs the
/// statement does not hold yet keep for their own scenarios.
pub const SKIPPED: [u32; 8] = [93, 94, 98, 99, 104, 105, 106, 107];

/// The change that builds record 0021: the standing prompt in layers and
/// cache use in status.
pub const STANDING: &str = "record 0021's change, the standing prompt, once its plan is approved";

/// The change that builds record 0022's locks and packages from outside.
pub const PACKAGES: &str = "record 0022's change, locks and packages, once its plan is approved";

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

/// What builds clause 183, an MCP server naming the labels whose places
/// may have it: the answer to section 10's question on how a host's
/// servers reach an agent.
pub const MCP_SERVERS: &str =
    "the answer to section 10's question on how a host's MCP servers reach an agent";

/// The change that builds record 0040: the sources of a host's kinds,
/// which landed on main after this list's branch was cut from it.
pub const SOURCES: &str = "a-host-declares-where-its-kinds-come-from, record 0040";

/// The change that built record 0041, a place cut from origin's copy of
/// its base, which landed before the list held S92: no scenario cites it
/// as data yet.
pub const ORIGIN: &str =
    "a-place-is-cut-from-origins-copy-of-its-base, record 0041, landed before the list held S92";

/// The change that builds record 0043: an agent's program settings from
/// flume's layers.
pub const SETTINGS: &str = "an-agents-program-settings-come-from-flumes-layers, record 0043";

/// The change that builds record 0044: an agent's effort, a setting of
/// flume's layers.
pub const EFFORT: &str = "an-agents-effort-is-a-setting-of-flumes-layers, record 0044";

/// Each clause or scenario no change has built yet, with the change that
/// builds it: no scenario of the suite cites one, and the coverage check
/// asks none to be cited. 154 and 157 are built as far as flume's own
/// kinds go, one package per version and a launch naming a version, and
/// their scenarios cite them.
pub fn unbuilt() -> Vec<(String, &'static str)> {
    let standing = [148, 149, 152]
        .map(|clause| clause.to_string())
        .into_iter()
        .chain((57..=58).map(|scenario| format!("S{scenario}")))
        .map(|clause| (clause, STANDING));
    let packages = [155, 156, 158, 159, 160]
        .map(|clause| clause.to_string())
        .into_iter()
        .chain((59..=63).map(|scenario| format!("S{scenario}")))
        .map(|clause| (clause, PACKAGES));
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
    let mcp_servers = std::iter::once(("183".to_string(), MCP_SERVERS));
    let sources = [191, 192]
        .map(|clause| clause.to_string())
        .into_iter()
        .chain((95..=97).map(|scenario| format!("S{scenario}")))
        .map(|clause| (clause, SOURCES));
    let origin = std::iter::once(("S92".to_string(), ORIGIN));
    let settings = [195, 196]
        .map(|clause| clause.to_string())
        .into_iter()
        .chain((100..=102).map(|scenario| format!("S{scenario}")))
        .map(|clause| (clause, SETTINGS));
    let effort = ["197".to_string(), "S103".to_string()]
        .into_iter()
        .map(|clause| (clause, EFFORT));
    standing
        .chain(packages)
        .chain(plan)
        .chain(points)
        .chain(profiles)
        .chain(statechart)
        .chain(mcp_servers)
        .chain(sources)
        .chain(origin)
        .chain(settings)
        .chain(effort)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_clauses_listed() {
        let a = part_a();
        let b = part_b();
        assert_eq!(a.len(), 77 + 8 + 3 + 2 + 4 + 1 + 10 + 6 + 3 + 9 + 3 + 7 + 5);
        assert_eq!(b.len(), 22 + 6 + 3 + 2 + 4 + 4 + 2 + 3 + 5 + 3 + 3);
        for gap in [100, 150, 193, 194, 198, 199] {
            assert!(!a.contains(&gap) && !b.contains(&gap), "{gap} is listed");
        }
        assert!(a.iter().all(|clause| !b.contains(clause)));
        let mut every: Vec<u32> = a.into_iter().chain(b).collect();
        every.sort();
        assert_eq!(
            every,
            (1..=201)
                .filter(|clause| ![100, 150, 193, 194, 198, 199].contains(clause))
                .collect::<Vec<_>>()
        );
        assert_eq!(all().len(), 195 + 8 + 100);
        assert!(all().contains(&"S64".to_string()), "S64 is not listed");
        for gap in SKIPPED.map(|scenario| format!("S{scenario}")) {
            assert!(!all().contains(&gap), "{gap} is listed");
        }
        for scenario in ["S92", "S95", "S97", "S100", "S103", "S108"] {
            assert!(
                all().contains(&scenario.to_string()),
                "{scenario} is not listed"
            );
        }
        assert_eq!(REVISION, "7bd1891");
        for clause in [
            129, 130, 131, 132, 137, 140, 141, 142, 148, 154, 157, 161, 162, 163, 165, 168, 169,
            173, 175, 176, 177, 180, 181, 182, 183, 184, 185, 186, 190, 191, 195, 197, 200,
        ] {
            assert!(part_a().contains(&clause), "{clause} is not in Part A");
        }
        for clause in [
            126, 127, 128, 133, 134, 135, 136, 138, 139, 151, 153, 160, 164, 174, 178, 179, 187,
            188, 189, 192, 196, 201,
        ] {
            assert!(part_b().contains(&clause), "{clause} is not in Part B");
        }
    }

    #[test]
    fn the_clauses_unbuilt_are_named_by_the_changes_that_build_them() {
        let unbuilt = unbuilt();
        assert_eq!(
            unbuilt.len(),
            3 + 2 + 5 + 5 + 3 + 6 + 10 + 6 + 1 + 5 + 1 + 5 + 2
        );
        assert!(unbuilt.iter().all(|(clause, _)| all().contains(clause)));
        for built in [
            "140", "141", "142", "143", "144", "145", "146", "147", "151", "153", "154", "157",
            "161", "162", "169", "179", "S50", "S51", "S52", "S53", "S54", "S55", "S56", "S64",
            "S65", "S66", "S67", "S78", "S79", "S80", "S81", "180", "181", "182", "S82", "S83",
            "S84", "S85", "184", "185", "186", "187", "188", "189", "190", "S86", "S87", "S88",
            "S89", "S90", "S91", "200", "201", "S108",
        ] {
            assert!(
                unbuilt.iter().all(|(clause, _)| clause != built),
                "{built} is named unbuilt"
            );
        }
    }
}
