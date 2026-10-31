//! The statement's clauses, invariants and scenarios, as this suite read
//! them at the statement's revision. A clause the statement adds or
//! removes changes this list in the commit that adopts it.

/// The revision of `design/flume/requirements.md` in agentplot/blueprints
/// the list was read at.
pub const REVISION: &str = "5caff76";

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
        .chain([160, 164, 174])
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
    all.extend((1..=SCENARIOS).map(|scenario| format!("S{scenario}")));
    all
}

/// The last scenario of section 11, which numbers them with no gap.
pub const SCENARIOS: u32 = 75;

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
    standing
        .chain(packages)
        .chain(plan)
        .chain(points)
        .chain(profiles)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_clauses_listed() {
        let a = part_a();
        let b = part_b();
        assert_eq!(a.len(), 77 + 8 + 3 + 2 + 4 + 1 + 10 + 6 + 3 + 9);
        assert_eq!(b.len(), 22 + 6 + 3 + 2 + 4 + 4 + 2 + 3 + 3);
        for gap in [100, 150] {
            assert!(!a.contains(&gap) && !b.contains(&gap), "{gap} is listed");
        }
        assert!(a.iter().all(|clause| !b.contains(clause)));
        let mut every: Vec<u32> = a.into_iter().chain(b).collect();
        every.sort();
        assert_eq!(
            every,
            (1..=174)
                .filter(|clause| ![100, 150].contains(clause))
                .collect::<Vec<_>>()
        );
        assert_eq!(all().len(), 172 + 8 + 75);
        assert!(all().contains(&"S64".to_string()), "S64 is not listed");
        assert_eq!(REVISION, "5caff76");
        for clause in [
            129, 130, 131, 132, 137, 140, 141, 142, 148, 154, 157, 161, 162, 163, 165, 168, 169,
            173,
        ] {
            assert!(part_a().contains(&clause), "{clause} is not in Part A");
        }
        for clause in [
            126, 127, 128, 133, 134, 135, 136, 138, 139, 151, 153, 160, 164, 174,
        ] {
            assert!(part_b().contains(&clause), "{clause} is not in Part B");
        }
    }

    #[test]
    fn the_clauses_unbuilt_are_records_0021_0022_0024s_plan_0025_and_0026s() {
        let unbuilt = unbuilt();
        assert_eq!(unbuilt.len(), 3 + 2 + 5 + 5 + 3 + 6 + 10);
        assert!(unbuilt.iter().all(|(clause, _)| all().contains(clause)));
        for built in [
            "140", "141", "142", "143", "144", "145", "146", "147", "151", "153", "154", "157",
            "161", "162", "169", "S50", "S51", "S52", "S53", "S54", "S55", "S56", "S64", "S65",
            "S66", "S67",
        ] {
            assert!(
                unbuilt.iter().all(|(clause, _)| clause != built),
                "{built} is named unbuilt"
            );
        }
    }
}
