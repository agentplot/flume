//! The statement's clauses, invariants and scenarios, as this suite read
//! them at the statement's revision. A clause the statement adds or
//! removes changes this list in the commit that adopts it.

/// The revision of `design/flume/requirements.md` in agentplot/blueprints
/// the list was read at.
pub const REVISION: &str = "4d1f88b";

/// Part A: what flume decides.
pub fn part_a() -> Vec<u32> {
    (1..=77)
        .chain(107..=114)
        .chain(118..=120)
        .chain([123, 124])
        .chain(129..=132)
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
    all.extend((1..=44).map(|scenario| format!("S{scenario}")));
    all
}

/// The change that builds what no change has built yet.
pub const NEXT_CHANGE: &str = "the next change, which delivers a place's ends";

/// Each clause or scenario no change has built yet, with the change that
/// builds it: no scenario of the suite cites one, and the coverage check
/// asks none to be cited.
pub fn unbuilt() -> Vec<(String, &'static str)> {
    ["129", "132", "S38"]
        .into_iter()
        .map(|clause| (clause.to_string(), NEXT_CHANGE))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_clauses_listed() {
        let a = part_a();
        let b = part_b();
        assert_eq!(a.len(), 77 + 8 + 3 + 2 + 4);
        assert_eq!(b.len(), 22 + 6 + 3 + 2 + 4 + 4);
        assert!(!a.contains(&100) && !b.contains(&100));
        assert!(a.iter().all(|clause| !b.contains(clause)));
        let mut every: Vec<u32> = a.into_iter().chain(b).collect();
        every.sort();
        assert_eq!(
            every,
            (1..=136)
                .filter(|clause| *clause != 100)
                .collect::<Vec<_>>()
        );
        assert_eq!(all().len(), 135 + 8 + 44);
        assert_eq!(REVISION, "4d1f88b");
        for clause in [129, 130, 131, 132] {
            assert!(part_a().contains(&clause), "{clause} is not in Part A");
        }
        for clause in [126, 127, 128, 133, 134, 135, 136] {
            assert!(part_b().contains(&clause), "{clause} is not in Part B");
        }
    }

    #[test]
    fn what_the_next_change_builds() {
        let unbuilt = unbuilt();
        assert_eq!(
            unbuilt
                .iter()
                .map(|(clause, _)| clause.as_str())
                .collect::<Vec<_>>(),
            ["129", "132", "S38"]
        );
        assert!(unbuilt
            .iter()
            .all(|(clause, change)| all().contains(clause) && *change == NEXT_CHANGE));
    }
}
