//! A pool's accounts as the queue reads them under the room lock (144,
//! 145): each account's agents of flume's running, the time it stays
//! spent until where the host's own document records it spent, its
//! reading as that document holds it, the windows its agent program last
//! reported in a session flume started, and the threshold that reading
//! holds it past, and the choice of an account at a start, the strategy
//! over the usable accounts in order (184, 185, 211; core.32).

use crate::room::Queue;
use crate::{Engine, Past, ReadingStatus, WindowStatus};
use flume_seams::*;
use std::collections::BTreeMap;

/// The whole percentage of a window past which an account takes no new
/// start, where its pool states none (185, C.7).
pub const THRESHOLD: u32 = 90;

/// How long a reading is fresh from when flume took it, by the host's
/// clock (184).
pub const FRESH: Seconds = 15 * 60;

/// Why an account whose program's profile builds no windows, or that no
/// profile binds, has no reading (171, 174, 184).
pub const NO_WINDOWS: &str = "its program's profile gives no windows";

/// Why an account of a program whose profile builds windows has no
/// reading: no session flume started has reported them for it (184).
pub const NONE_REPORTED: &str = "no session flume started has reported its windows";

/// A pool as the queue reads it: its strategy, its threshold, 90 where
/// it states none, and its accounts in order (185).
#[derive(Clone, Debug)]
pub(crate) struct PoolRoom {
    pub strategy: Strategy,
    pub threshold: u32,
    pub accounts: Vec<AccountRoom>,
}

/// An account of a pool: as the configuration declares it, flume's
/// agents running on it, the time it is spent until, none where it is
/// not spent at the host's clock, the words in which flume read no reset
/// where that time is the hour, and its reading, with the host's clock
/// and its pool's threshold, 90 where the pool states none, the queue
/// read it at (144, 146, 184, 185).
#[derive(Clone, Debug)]
pub(crate) struct AccountRoom {
    pub account: Account,
    pub running: u32,
    pub spent_until: Option<Seconds>,
    pub spent_unread: Option<String>,
    pub reading: Reading,
    pub threshold: u32,
    pub now: Seconds,
}

/// An account's reading as the queue reads it from the account's own
/// document (184).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Reading {
    /// The document holds none, with why: [`NO_WINDOWS`] or
    /// [`NONE_REPORTED`].
    None(String),
    /// The reading flume took from a session it started, as the document
    /// holds it.
    Taken(AccountReading),
}

/// The window's use now in hundredths of a percent: none where its reset
/// has passed at `now`, its `used` otherwise (184).
pub(crate) fn used_now(window: &UsageWindow, now: Seconds) -> u32 {
    match window.resets_at {
        Some(resets_at) if resets_at <= now => 0,
        _ => window.used,
    }
}

/// When flume took the reading, a time later than `now` read as `now`
/// (184).
fn taken_at(reading: &AccountReading, now: Seconds) -> Seconds {
    reading.taken_at.min(now)
}

/// Whether the reading is fresh at `now`: less than [`FRESH`] past when
/// flume took it (184).
pub(crate) fn fresh(reading: &AccountReading, now: Seconds) -> bool {
    now < taken_at(reading, now).saturating_add(FRESH)
}

/// Until when `window` of `reading` holds its account past `threshold`,
/// a whole percentage, at `now`: where its use now is at the threshold or
/// past it, until its reset whatever the reading's age, and, where it
/// names none, until the reading stops being fresh, while it is; none
/// otherwise (185).
pub(crate) fn holds_until(
    reading: &AccountReading,
    window: &UsageWindow,
    threshold: u32,
    now: Seconds,
) -> Option<Seconds> {
    if used_now(window, now) < threshold.saturating_mul(100) {
        return None;
    }
    match window.resets_at {
        Some(resets_at) => Some(resets_at),
        None => fresh(reading, now).then(|| taken_at(reading, now).saturating_add(FRESH)),
    }
}

/// What holds the account whose reading is `reading` past `threshold` at
/// `now`: of the windows that hold it, the one that holds it latest,
/// equals in the reading's order; none where none holds it (185).
pub(crate) fn past(reading: &AccountReading, threshold: u32, now: Seconds) -> Option<Past> {
    reading
        .windows
        .iter()
        .filter_map(|window| {
            holds_until(reading, window, threshold, now).map(|until| Past {
                window: window.name.clone(),
                until,
            })
        })
        .fold(None, |latest: Option<Past>, past| match latest {
            Some(latest) if latest.until >= past.until => Some(latest),
            _ => Some(past),
        })
}

/// What the account whose reading is `reading` has left at `now`, in
/// hundredths of a percent: 10000 less the most use now of its windows,
/// for a fresh reading with a window; none otherwise (145, 184).
pub(crate) fn left(reading: &AccountReading, now: Seconds) -> Option<u32> {
    if !fresh(reading, now) {
        return None;
    }
    reading
        .windows
        .iter()
        .map(|window| used_now(window, now))
        .max()
        .map(|most| 10_000u32.saturating_sub(most))
}

impl AccountRoom {
    /// What holds it past its threshold, none where nothing does (185).
    pub fn past(&self) -> Option<Past> {
        match &self.reading {
            Reading::Taken(reading) => past(reading, self.threshold, self.now),
            Reading::None(_) => None,
        }
    }

    /// Its reading as status gives it: when flume took it, from which
    /// agent and session, whether it is fresh, and each window with
    /// whether it holds the account past its threshold; or why it has none
    /// (184, 185).
    pub fn reading_status(&self) -> Option<ReadingStatus> {
        Some(match &self.reading {
            Reading::None(why) => ReadingStatus::Unread {
                unread: why.clone(),
            },
            Reading::Taken(reading) => ReadingStatus::Read {
                read_at: reading.taken_at,
                agent: reading.agent.clone(),
                session: reading.session.clone(),
                fresh: fresh(reading, self.now),
                windows: reading
                    .windows
                    .iter()
                    .map(|window| WindowStatus {
                        name: window.name.clone(),
                        used: window.used,
                        resets_at: window.resets_at,
                        past: holds_until(reading, window, self.threshold, self.now).is_some(),
                    })
                    .collect(),
            },
        })
    }

    /// What it has left by a fresh reading, none where it has none (184).
    pub fn left(&self) -> Option<u32> {
        match &self.reading {
            Reading::Taken(reading) => left(reading, self.now),
            Reading::None(_) => None,
        }
    }

    /// Why the account cannot take a start now, none where it can: spent,
    /// past its threshold, or at its own bound (145, 146, 185).
    pub fn short(&self) -> Option<MoveWhy> {
        if self.spent_until.is_some() {
            return Some(MoveWhy::Spent);
        }
        if self.past().is_some() {
            return Some(MoveWhy::Threshold);
        }
        self.account
            .agents
            .is_some_and(|bound| self.running >= bound)
            .then_some(MoveWhy::Full)
    }

    /// Whether a start of `program` may take it: not spent, not past its
    /// threshold, under its own bound, and running that program (144,
    /// 145, 185).
    pub fn usable(&self, program: &str) -> bool {
        self.account.program == program && self.short().is_none()
    }

    /// The move of `on` off it to `to` at `at`, where it cannot take a
    /// start now, with why, `carried` as a stage's move after its agent
    /// stopped at the usage limit gives it, and, off an account past its
    /// threshold, the window that holds it and when that window stops
    /// holding it; none where it can take a start (56, 145 to 147, 185).
    pub fn move_off(
        &self,
        on: MoveOn,
        to: &str,
        at: Seconds,
        carried: Option<bool>,
    ) -> Option<AccountMove> {
        let why = self.short()?;
        let past = match why {
            MoveWhy::Threshold => self.past(),
            MoveWhy::Spent | MoveWhy::Full => None,
        };
        let (window, until) =
            past.map_or((None, None), |past| (Some(past.window), Some(past.until)));
        Some(AccountMove {
            on,
            from: self.account.name.clone(),
            to: to.to_string(),
            why,
            at,
            carried,
            window,
            until,
        })
    }

    /// When it is back: the latest of the time it is spent until and the
    /// end of what holds it past its threshold, where it has either; none
    /// where it is only full (145, 146, 185).
    pub fn back(&self) -> Option<Seconds> {
        self.spent_until
            .into_iter()
            .chain(self.past().map(|past| past.until))
            .max()
    }
}

/// What a start of a pool's stage may take.
pub(crate) enum Usable {
    /// An account of the pool runs the program and may take it.
    Some,
    /// No account of the pool runs the program (144).
    NoProgram,
    /// Every account of the pool that runs the program is spent, past its
    /// threshold or full; the soonest time one is back, none where each is
    /// only full (145, 146, 185).
    None(Option<Seconds>),
}

impl PoolRoom {
    /// The pool's accounts that run `program`.
    fn running_program<'a>(&'a self, program: &'a str) -> impl Iterator<Item = &'a AccountRoom> {
        self.accounts
            .iter()
            .filter(move |room| room.account.program == program)
    }

    /// Whether the pool holds an account that runs `program`, spent, full
    /// or neither (145).
    pub fn runs(&self, program: &str) -> bool {
        self.running_program(program).next().is_some()
    }

    /// Whether a start of `program` may take an account of the pool.
    pub fn usable(&self, program: &str) -> Usable {
        if !self.runs(program) {
            return Usable::NoProgram;
        }
        if self.accounts.iter().any(|room| room.usable(program)) {
            return Usable::Some;
        }
        Usable::None(
            self.running_program(program)
                .filter_map(AccountRoom::back)
                .min(),
        )
    }

    /// The account the strategy chooses for a start of `program`: under
    /// fill-first the first usable in order; under least-busy the usable
    /// one with fewest of flume's agents, equals in order; under most-left
    /// the usable one with the most left by a fresh reading, then each with
    /// no fresh reading, equals in either by fewest of flume's agents and
    /// then in order, so that with no fresh reading anywhere it chooses as
    /// least-busy does (145, 184).
    pub fn strategy(&self, program: &str) -> Option<&AccountRoom> {
        let mut usable = self.accounts.iter().filter(|room| room.usable(program));
        match self.strategy {
            Strategy::FillFirst => usable.next(),
            Strategy::LeastBusy => {
                usable.fold(None, |least: Option<&AccountRoom>, room| match least {
                    Some(least) if least.running <= room.running => Some(least),
                    _ => Some(room),
                })
            }
            Strategy::MostLeft => {
                let rank = |room: &AccountRoom| {
                    let left = room.left();
                    (left.is_some(), left, std::cmp::Reverse(room.running))
                };
                usable.fold(None, |most: Option<&AccountRoom>, room| match most {
                    Some(most) if rank(most) >= rank(room) => Some(most),
                    _ => Some(room),
                })
            }
        }
    }

    pub fn account(&self, name: &str) -> Option<&AccountRoom> {
        self.accounts.iter().find(|room| room.account.name == name)
    }

    pub fn account_mut(&mut self, name: &str) -> Option<&mut AccountRoom> {
        self.accounts
            .iter_mut()
            .find(|room| room.account.name == name)
    }
}

/// The account a start takes, as the queue under the room lock finds its
/// pool (144, 145).
pub(crate) enum Choice {
    /// Neither the stream nor the host names a pool: the agent runs under
    /// no account (144).
    NoPool,
    /// The account chosen, and how.
    Chose { held: AccountHeld, account: Account },
    /// No account of the pool is usable: the stage waits for room (145).
    Wait,
    /// The pool named is not declared, naming it.
    Undeclared(String),
    /// The pool holds no account that runs the stage's program, in
    /// [`no_account`]'s words: the stage is refused, since no room the pool
    /// gives could start it (4, 32, 145).
    NoAccount(String),
}

/// Why a stage whose agent runs `program` cannot start on the pool `pool`,
/// which holds no account of that program: the words a launch and a start
/// refuse it in (4, 32, 145).
pub(crate) fn no_account(stage: &str, pool: &str, program: &str) -> String {
    format!(
        "the stage {stage}'s agent runs {program}, and the pool {pool} holds no account of {program}, so no room it gives could start it (145)"
    )
}

impl Engine {
    /// The account a start of `place`'s stage `stage` whose agent runs
    /// `program` takes: the intent's affinity's for `program` while it is
    /// usable, else the strategy's choice over its pool's usable accounts,
    /// in order; and, where the strategy chose because the affinity's
    /// account for `program` was spent, past its threshold or full, the
    /// intent's move to it for `program`, off a threshold with the window
    /// and its end, unless it is an account the affinity for `program`
    /// once left. A program the affinity keeps no account for moves nothing, and
    /// a pool that holds no account of `program` refuses the stage (4, 32,
    /// 144, 145, 147; core.32). A start that carries a conversation lost
    /// with its terminal or its host takes `conversation`, the account its
    /// lost start ran under, first, where the pool declares it under the
    /// same pool, of `program` and usable, moving no intent's affinity by
    /// it (49, 145, 147, 179).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn choose_account(
        &self,
        place: &PlaceRecord,
        stage: &str,
        program: Option<&str>,
        affinity: Option<&Affinity>,
        conversation: Option<&AccountHeld>,
        queue: &Queue,
        now: Seconds,
    ) -> (Choice, Option<AccountMove>) {
        let stream = &place.stream.name;
        let Some(pool) = queue.pool_of(stream) else {
            return (Choice::NoPool, None);
        };
        let Some(room) = queue.pools.get(pool) else {
            return (
                Choice::Undeclared(format!(
                    "the pool {pool} the stream {stream} takes its accounts from is not declared by the host's configuration"
                )),
                None,
            );
        };
        let Some(program) = program else {
            return (Choice::Wait, None);
        };
        if let Usable::NoProgram = room.usable(program) {
            return (Choice::NoAccount(no_account(stage, pool, program)), None);
        }
        let kept = affinity
            .and_then(|affinity| affinity.accounts.get(program))
            .filter(|kept| kept.pool == pool)
            .and_then(|kept| room.account(&kept.account));
        let chose = |chosen: &AccountRoom, from| Choice::Chose {
            held: AccountHeld {
                pool: pool.to_string(),
                name: chosen.account.name.clone(),
                from,
            },
            account: chosen.account.clone(),
        };
        let holds = conversation
            .filter(|held| held.pool == pool)
            .and_then(|held| room.account(&held.name))
            .filter(|holds| holds.usable(program));
        if let Some(holds) = holds {
            return (chose(holds, AccountFrom::Conversation), None);
        }
        if let Some(kept) = kept.filter(|kept| kept.usable(program)) {
            return (chose(kept, AccountFrom::Affinity), None);
        }
        let Some(chosen) = room.strategy(program) else {
            return (Choice::Wait, None);
        };
        let moved = kept.zip(affinity).and_then(|(kept, affinity)| {
            if affinity.left(program, &chosen.account.name) {
                return None;
            }
            kept.move_off(
                MoveOn::Intent {
                    program: Some(program.to_string()),
                },
                &chosen.account.name,
                now,
                None,
            )
        });
        (chose(chosen, AccountFrom::Strategy), moved)
    }

    /// The intent's affinity after a start of `place` whose agent runs
    /// `program` under `account`: written at the intent's first start
    /// with a pool, its account for `program` kept at the first start on
    /// `program` and moved by `moved`, every other program's as it stands,
    /// and no longer gone (147).
    pub(crate) fn affinity_after(
        &self,
        place: &PlaceRecord,
        program: &str,
        account: &AccountHeld,
        moved: Option<&AccountMove>,
    ) {
        let kept = AffinityAccount {
            pool: account.pool.clone(),
            account: account.name.clone(),
        };
        self.write_affinity(&place.intent, |standing| {
            let mut affinity = standing.unwrap_or_else(|| Affinity {
                stream: place.stream.name.clone(),
                accounts: BTreeMap::new(),
                moves: vec![],
                gone: None,
                pool: None,
                account: None,
            });
            if let Some(moved) = moved {
                affinity.stream = place.stream.name.clone();
                affinity.accounts.insert(
                    program.to_string(),
                    AffinityAccount {
                        account: moved.to.clone(),
                        ..kept.clone()
                    },
                );
                affinity.moves.push(moved.clone());
            }
            affinity
                .accounts
                .entry(program.to_string())
                .or_insert_with(|| kept.clone());
            affinity.gone = None;
            Some(affinity)
        });
    }
}

impl Engine {
    /// Why the account `account` has no reading: its program's profile
    /// gives no windows, or no profile binds its program, else no session
    /// flume started has reported them; nothing is asked of the program
    /// (171, 174, 184; core.32).
    pub(crate) fn no_reading(&self, account: &Account) -> String {
        match self.agent(&account.program) {
            Some(agent) if agent.reports_windows() == Offered::Offered => NONE_REPORTED.into(),
            _ => NO_WINDOWS.into(),
        }
    }
}

impl Engine {
    /// Refuses a launch's stage `stage` of a place in `stream` whose agent
    /// runs `program` where the pool the host's configuration names for
    /// the stream, as it stands now, declares no account of that program;
    /// a stream with no pool, or with one the host does not declare, is
    /// refused nothing here (4, 32, 145; core.8, core.30).
    pub(crate) fn pool_runs(
        &self,
        stream: &str,
        stage: &str,
        program: Option<&str>,
    ) -> Result<(), String> {
        let (Some(named), Some(program)) = (self.host.pool_for(stream), program) else {
            return Ok(());
        };
        let Some(pool) = self.host.pool(&named.name) else {
            return Ok(());
        };
        if pool
            .accounts
            .iter()
            .any(|account| account.program == program)
        {
            return Ok(());
        }
        Err(no_account(stage, &named.name, program))
    }
}

impl Engine {
    /// The move of the stage at `index` from the account its last start
    /// ran under to `to`, in the same pool, where that account is spent,
    /// past its threshold or at its own bound now, off a threshold with
    /// the window and its end, with whether its conversation went with it,
    /// `carried`, where its agent stopped at its account's usage limit;
    /// none otherwise (56, 145, 146, 179, 185).
    pub(crate) fn stage_moved(
        &self,
        held: &HeldState,
        index: usize,
        to: Option<&AccountHeld>,
        carried: Option<bool>,
        queue: &Queue,
        now: Seconds,
    ) -> Option<AccountMove> {
        let to = to?;
        let stage = &held.stages[index];
        let from = stage.start.as_ref()?.account.as_ref()?;
        if from.pool != to.pool || from.name == to.name {
            return None;
        }
        queue.account(&from.pool, &from.name)?.move_off(
            MoveOn::Stage {
                stage: stage.name.clone(),
            },
            &to.name,
            now,
            carried,
        )
    }
}

/// How long an intent's affinity outlives its last place (147).
pub const AFFINITY_KEPT: Seconds = 60 * 60;

/// How often a write of an intent's affinity reads it again when its
/// version moved under it (103).
const TRIES: usize = 8;

impl Engine {
    fn intent_key(intent: &IntentKey) -> HostKey {
        HostKey::Intent {
            digest: intent.digest().to_string(),
        }
    }

    /// The intent's affinity and the version it was read at, none where
    /// it has none; one whose last place went an hour ago or more is
    /// removed by compare-and-set and read as none (147). A document
    /// written before an intent kept an account per program reads as the
    /// affinity for its account's program, as the host's configuration
    /// declares that account now, and with its stream alone where it
    /// declares it no more; it is written in the new shape at its next
    /// change (147, 153).
    pub(crate) fn affinity(&self, intent: &IntentKey) -> Option<(u64, Affinity)> {
        let key = Self::intent_key(intent);
        let read = self.store.read_host(&key)?;
        let HostDocument::Intent(affinity) = read.document else {
            return None;
        };
        if let Some(gone) = affinity.gone {
            if self.host.now() >= gone.saturating_add(AFFINITY_KEPT) {
                self.store.write_host(&key, Some(read.version), None);
                return None;
            }
        }
        Some((read.version, self.folded(affinity)))
    }

    /// A document written before an intent kept an account per program,
    /// its one account given its program, as the host's configuration
    /// declares the account in its pool now: the account kept for that
    /// program unless one is kept for it already, and every intent move
    /// with no program given it; the account dropped where the pool no
    /// longer declares it (147).
    fn folded(&self, mut affinity: Affinity) -> Affinity {
        let (Some(pool), Some(account)) = (affinity.pool.take(), affinity.account.take()) else {
            return affinity;
        };
        let program = self.host.pool(&pool).and_then(|declared| {
            declared
                .accounts
                .into_iter()
                .find(|declared| declared.name == account)
                .map(|declared| declared.program)
        });
        let Some(program) = program else {
            return affinity;
        };
        for moved in &mut affinity.moves {
            if let MoveOn::Intent {
                program: none @ None,
            } = &mut moved.on
            {
                *none = Some(program.clone());
            }
        }
        affinity
            .accounts
            .entry(program)
            .or_insert(AffinityAccount { pool, account });
        affinity
    }

    /// Writes the intent's affinity as `change` makes it of what stands,
    /// none where none stands, by compare-and-set, reading it again
    /// where its version moved; `change` keeping it as it is writes
    /// nothing (103, 147).
    pub(crate) fn write_affinity(
        &self,
        intent: &IntentKey,
        change: impl Fn(Option<Affinity>) -> Option<Affinity>,
    ) {
        let key = Self::intent_key(intent);
        for _ in 0..TRIES {
            let (at, standing) = match self.affinity(intent) {
                Some((version, affinity)) => (Some(version), Some(affinity)),
                None => (None, None),
            };
            let Some(next) = change(standing.clone()) else {
                return;
            };
            if standing.as_ref() == Some(&next) {
                return;
            }
            let at = at.or_else(|| self.store.read_host(&key).map(|read| read.version));
            if let Written::Written { .. } =
                self.store
                    .write_host(&key, at, Some(&HostDocument::Intent(next)))
            {
                return;
            }
        }
    }

    /// Marks the intent's affinity gone at the host's clock where no
    /// place of the intent but `leaving` still stands (147).
    pub(crate) fn intent_left(&self, intent: &IntentKey, leaving: &PlaceId) {
        let stands = self.store.read_every().iter().any(|read| {
            let held = &read.state;
            &held.place.id != leaving
                && held.place.intent.digest() == intent.digest()
                && !matches!(held.phase, Phase::Gone { .. } | Phase::Tidied { .. })
        });
        if stands {
            return;
        }
        let now = self.host.now();
        self.write_affinity(intent, |standing| {
            standing.map(|affinity| Affinity {
                gone: affinity.gone.or(Some(now)),
                ..affinity
            })
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: Seconds = 1_000_000;

    fn window(name: &str, used: u32, resets_at: Option<Seconds>) -> UsageWindow {
        UsageWindow {
            name: name.into(),
            used,
            resets_at,
        }
    }

    /// A reading flume took at `taken_at` from a fix's code agent.
    fn taken(taken_at: Seconds, windows: Vec<UsageWindow>) -> AccountReading {
        AccountReading {
            taken_at,
            agent: "flume:r:fix--one:code:1".into(),
            session: Some("s-1".into()),
            windows,
        }
    }

    #[test]
    fn a_window_past_its_reset_reads_as_nothing_used() {
        let passed = window("5-hour", 9_500, Some(NOW - 3600));
        assert_eq!(used_now(&passed, NOW), 0);
        assert_eq!(used_now(&window("5-hour", 9_500, Some(NOW)), NOW), 0);
        assert_eq!(
            used_now(&window("5-hour", 9_500, Some(NOW + 1)), NOW),
            9_500
        );
        assert_eq!(used_now(&window("7-day", 9_500, None), NOW), 9_500);
        let read = taken(NOW - 2 * 3600, vec![passed]);
        assert_eq!(past(&read, THRESHOLD, NOW), None);
    }

    #[test]
    fn a_reading_is_fresh_for_fifteen_minutes_from_when_flume_took_it() {
        assert!(fresh(&taken(NOW - 120, vec![]), NOW));
        assert!(fresh(&taken(NOW - FRESH + 1, vec![]), NOW));
        assert!(!fresh(&taken(NOW - FRESH, vec![]), NOW));
        assert!(!fresh(&taken(NOW - 20 * 60, vec![]), NOW));
        assert!(fresh(&taken(NOW + 3600, vec![]), NOW), "a taken_at to come");
    }

    #[test]
    fn a_window_with_a_reset_holds_until_it_whatever_the_readings_age() {
        let monday = NOW + 3 * 24 * 3600;
        let old = taken(NOW - 3 * 3600, vec![window("7-day", 9_500, Some(monday))]);
        assert_eq!(
            past(&old, THRESHOLD, NOW),
            Some(Past {
                window: "7-day".into(),
                until: monday
            })
        );
        assert_eq!(past(&old, THRESHOLD, monday), None);
    }

    #[test]
    fn a_window_with_no_reset_holds_while_the_reading_is_fresh() {
        let read = taken(NOW, vec![window("weekly", 9_500, None)]);
        assert_eq!(
            past(&read, THRESHOLD, NOW + 60),
            Some(Past {
                window: "weekly".into(),
                until: NOW + FRESH
            })
        );
        assert_eq!(past(&read, THRESHOLD, NOW + FRESH), None);
    }

    #[test]
    fn the_threshold_is_held_at_its_own_share() {
        let at = |used| taken(NOW, vec![window("5-hour", used, Some(NOW + 3600))]);
        assert!(past(&at(9_000), THRESHOLD, NOW).is_some(), "90% at 90");
        assert!(past(&at(8_999), THRESHOLD, NOW).is_none(), "89.99% at 90");
        assert!(past(&at(9_900), 100, NOW).is_none(), "99% at 100");
        assert!(past(&at(10_000), 100, NOW).is_some(), "100% at 100");
        assert!(past(&at(5_000), 50, NOW).is_some(), "50% at 50");
        assert!(past(&at(4_999), 50, NOW).is_none(), "49.99% at 50");
    }

    #[test]
    fn the_window_that_holds_latest_is_named() {
        let read = taken(
            NOW,
            vec![
                window("5-hour", 9_200, Some(NOW + 3600)),
                window("7-day", 9_600, Some(NOW + 3 * 24 * 3600)),
                window("other", 9_900, Some(NOW + 3 * 24 * 3600)),
                window("under", 1_000, Some(NOW + 9 * 24 * 3600)),
            ],
        );
        assert_eq!(
            past(&read, THRESHOLD, NOW),
            Some(Past {
                window: "7-day".into(),
                until: NOW + 3 * 24 * 3600
            })
        );
    }

    #[test]
    fn what_a_fresh_reading_leaves() {
        let read = |taken_at| {
            taken(
                taken_at,
                vec![
                    window("5-hour", 4_000, Some(NOW + 3600)),
                    window("7-day", 8_000, Some(NOW + 3 * 24 * 3600)),
                ],
            )
        };
        assert_eq!(left(&read(NOW - 120), NOW), Some(2_000), "20 left");
        assert_eq!(left(&read(NOW - 20 * 60), NOW), None, "stale");
        assert_eq!(left(&taken(NOW, vec![]), NOW), None, "no window");
        let reset = taken(NOW, vec![window("5-hour", 9_000, Some(NOW))]);
        assert_eq!(left(&reset, NOW), Some(10_000), "a window reset");
    }

    fn room(spent_until: Option<Seconds>, reading: Reading, agents: Option<u32>) -> AccountRoom {
        AccountRoom {
            account: Account {
                name: "work-1".into(),
                program: "agent".into(),
                env: BTreeMap::new(),
                agents,
            },
            running: 1,
            spent_until,
            spent_unread: None,
            reading,
            threshold: THRESHOLD,
            now: NOW,
        }
    }

    #[test]
    fn short_answers_spent_then_threshold_then_full() {
        let past = Reading::Taken(taken(NOW, vec![window("5-hour", 9_200, Some(NOW + 3600))]));
        let under = Reading::Taken(taken(NOW, vec![window("5-hour", 4_000, Some(NOW + 3600))]));
        let spent = Some(NOW + 600);
        assert_eq!(
            room(spent, past.clone(), Some(1)).short(),
            Some(MoveWhy::Spent)
        );
        assert_eq!(
            room(None, past.clone(), Some(1)).short(),
            Some(MoveWhy::Threshold)
        );
        assert_eq!(
            room(None, under.clone(), Some(1)).short(),
            Some(MoveWhy::Full)
        );
        assert_eq!(room(None, under, None).short(), None);
        assert_eq!(
            room(None, Reading::None(NONE_REPORTED.into()), None).short(),
            None
        );
        assert_eq!(room(spent, past.clone(), None).back(), Some(NOW + 3600));
        assert_eq!(room(Some(NOW + 7200), past, None).back(), Some(NOW + 7200));
        assert_eq!(
            room(None, Reading::None(NO_WINDOWS.into()), Some(1)).back(),
            None
        );
    }

    #[test]
    fn status_gives_the_reading_with_its_agent_or_why_it_has_none() {
        let read = taken(
            NOW - 120,
            vec![window("five_hour", 4_200, Some(NOW + 3600))],
        );
        assert_eq!(
            room(None, Reading::Taken(read), None).reading_status(),
            Some(ReadingStatus::Read {
                read_at: NOW - 120,
                agent: "flume:r:fix--one:code:1".into(),
                session: Some("s-1".into()),
                fresh: true,
                windows: vec![WindowStatus {
                    name: "five_hour".into(),
                    used: 4_200,
                    resets_at: Some(NOW + 3600),
                    past: false,
                }],
            })
        );
        let stale = taken(NOW - 20 * 60, vec![]);
        assert!(matches!(
            room(None, Reading::Taken(stale), None).reading_status(),
            Some(ReadingStatus::Read { fresh: false, .. })
        ));
        assert_eq!(
            room(None, Reading::None(NONE_REPORTED.into()), None).reading_status(),
            Some(ReadingStatus::Unread {
                unread: "no session flume started has reported its windows".into(),
            })
        );
        let json = serde_json::to_value(
            room(
                None,
                Reading::Taken(AccountReading {
                    session: None,
                    ..taken(NOW, vec![])
                }),
                None,
            )
            .reading_status(),
        )
        .unwrap();
        assert!(json.get("session").is_none(), "{json}");
        assert_eq!(json["agent"], "flume:r:fix--one:code:1");
    }
}
