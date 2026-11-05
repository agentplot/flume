//! A pool's accounts as the queue reads them under the room lock (144,
//! 145): each account's agents of flume's running, the time it stays
//! spent until where the host's own document records it spent, and the
//! choice of an account at a start, the strategy over the usable accounts
//! in order (core.32).

use crate::room::Queue;
use crate::Engine;
use flume_seams::*;
use std::collections::BTreeMap;

/// A pool as the queue reads it: its strategy and its accounts in order.
#[derive(Clone, Debug)]
pub(crate) struct PoolRoom {
    pub strategy: Strategy,
    pub accounts: Vec<AccountRoom>,
}

/// An account of a pool: as the configuration declares it, flume's
/// agents running on it, and the time it is spent until, none where it is
/// not spent at the host's clock (144, 146).
#[derive(Clone, Debug)]
pub(crate) struct AccountRoom {
    pub account: Account,
    pub running: u32,
    pub spent_until: Option<Seconds>,
}

impl AccountRoom {
    /// Why the account cannot take a start now, none where it can: spent,
    /// or at its own bound (145).
    pub fn short(&self) -> Option<MoveWhy> {
        if self.spent_until.is_some() {
            return Some(MoveWhy::Spent);
        }
        self.account
            .agents
            .is_some_and(|bound| self.running >= bound)
            .then_some(MoveWhy::Full)
    }

    /// Whether a start of `program` may take it: not spent, under its own
    /// bound, and running that program (144, 145).
    pub fn usable(&self, program: &str) -> bool {
        self.account.program == program && self.short().is_none()
    }
}

/// What a start of a pool's stage may take.
pub(crate) enum Usable {
    /// An account of the pool runs the program and may take it.
    Some,
    /// No account of the pool runs the program (144).
    NoProgram,
    /// Every account of the pool that runs the program is spent or full;
    /// the soonest time one stops being spent, none where none is spent
    /// (145, 146).
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
                .filter_map(|room| room.spent_until)
                .min(),
        )
    }

    /// The account the strategy chooses for a start of `program`: under
    /// fill-first the first usable in order, under least-busy the usable
    /// one with fewest of flume's agents, equals in order; under most-left,
    /// with no account's reading read, as least-busy (145, 184).
    pub fn strategy(&self, program: &str) -> Option<&AccountRoom> {
        let mut usable = self.accounts.iter().filter(|room| room.usable(program));
        match self.strategy {
            Strategy::FillFirst => usable.next(),
            Strategy::LeastBusy | Strategy::MostLeft => {
                usable.fold(None, |least: Option<&AccountRoom>, room| match least {
                    Some(least) if least.running <= room.running => Some(least),
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
    /// account for `program` was spent or full, the intent's move to it for
    /// `program`, unless it is an account the affinity for `program` once
    /// left. A program the affinity keeps no account for moves nothing, and
    /// a pool that holds no account of `program` refuses the stage (4, 32,
    /// 144, 145, 147; core.32).
    pub(crate) fn choose_account(
        &self,
        place: &PlaceRecord,
        stage: &str,
        program: Option<&str>,
        affinity: Option<&Affinity>,
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
        if let Some(kept) = kept.filter(|kept| kept.usable(program)) {
            return (chose(kept, AccountFrom::Affinity), None);
        }
        let Some(chosen) = room.strategy(program) else {
            return (Choice::Wait, None);
        };
        let moved = kept.zip(affinity).and_then(|(kept, affinity)| {
            let why = kept.short()?;
            (!affinity.left(program, &chosen.account.name)).then(|| AccountMove {
                on: MoveOn::Intent {
                    program: Some(program.to_string()),
                },
                from: kept.account.name.clone(),
                to: chosen.account.name.clone(),
                why,
                at: now,
                carried: None,
                window: None,
                until: None,
            })
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
    /// ran under to `to`, in the same pool, where that account is spent or
    /// at its own bound now, with whether its conversation went with it,
    /// `carried`, where its agent stopped at its account's usage limit;
    /// none otherwise (56, 145, 146, 179).
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
        let why = queue.account(&from.pool, &from.name)?.short()?;
        Some(AccountMove {
            on: MoveOn::Stage {
                stage: stage.name.clone(),
            },
            from: from.name.clone(),
            to: to.name.clone(),
            why,
            at: now,
            carried,
            window: None,
            until: None,
        })
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
