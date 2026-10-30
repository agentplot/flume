//! The statement's clauses, invariants and scenarios, as this suite read
//! them at the statement's revision. A clause the statement adds or
//! removes changes this list in the commit that adopts it.

/// The revision of `design/flume/requirements.md` in agentplot/blueprints
/// the list was read at.
pub const REVISION: &str = "57a1ba9";

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
        .chain([160])
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

/// The last scenario of section 11.
pub const SCENARIOS: u32 = 63;

/// The change that builds record 0020: streams, pools, spent accounts
/// and an intent's affinity.
pub const STREAMS: &str = "record 0020's change, streams and pools, once its plan is approved";

/// The change that builds record 0021: the standing prompt in layers and
/// cache use in status.
pub const STANDING: &str = "record 0021's change, the standing prompt, once its plan is approved";

/// The change that builds record 0022's locks and packages from outside.
pub const PACKAGES: &str = "record 0022's change, locks and packages, once its plan is approved";

/// Each clause or scenario no change has built yet, with the change that
/// builds it: no scenario of the suite cites one, and the coverage check
/// asks none to be cited. 154 and 157 are built as far as flume's own
/// kinds go, one package per version and a launch naming a version, and
/// their scenarios cite them.
pub fn unbuilt() -> Vec<(String, &'static str)> {
    let streams = [146]
        .map(|clause| clause.to_string())
        .into_iter()
        .chain(["S55".to_string()])
        .map(|clause| (clause, STREAMS));
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
    streams.chain(standing).chain(packages).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_clauses_listed() {
        let a = part_a();
        let b = part_b();
        assert_eq!(a.len(), 77 + 8 + 3 + 2 + 4 + 1 + 10 + 6);
        assert_eq!(b.len(), 22 + 6 + 3 + 2 + 4 + 4 + 2 + 3 + 1);
        for gap in [100, 150] {
            assert!(!a.contains(&gap) && !b.contains(&gap), "{gap} is listed");
        }
        assert!(a.iter().all(|clause| !b.contains(clause)));
        let mut every: Vec<u32> = a.into_iter().chain(b).collect();
        every.sort();
        assert_eq!(
            every,
            (1..=160)
                .filter(|clause| ![100, 150].contains(clause))
                .collect::<Vec<_>>()
        );
        assert_eq!(all().len(), 158 + 8 + 63);
        assert_eq!(REVISION, "57a1ba9");
        for clause in [129, 130, 131, 132, 137, 140, 141, 142, 148, 154, 157] {
            assert!(part_a().contains(&clause), "{clause} is not in Part A");
        }
        for clause in [126, 127, 128, 133, 134, 135, 136, 138, 139, 151, 153, 160] {
            assert!(part_b().contains(&clause), "{clause} is not in Part B");
        }
    }

    #[test]
    fn the_clauses_unbuilt_are_records_0020_to_0022s() {
        let unbuilt = unbuilt();
        assert_eq!(unbuilt.len(), 1 + 1 + 3 + 2 + 5 + 5);
        assert!(unbuilt.iter().all(|(clause, _)| all().contains(clause)));
        for built in [
            "140", "141", "142", "143", "144", "145", "147", "151", "153", "154", "157", "S50",
            "S51", "S52", "S53", "S54", "S56",
        ] {
            assert!(
                unbuilt.iter().all(|(clause, _)| clause != built),
                "{built} is named unbuilt"
            );
        }
    }
}
