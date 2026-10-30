//! A pool's accounts as the queue reads them under the room lock (144,
//! 145): each account's agents of flume's running, the time it stays
//! spent until where the host's own document records it spent, and the
//! choice of an account at a start, the strategy over the usable accounts
//! in order (core.32).

use crate::room::Queue;
use crate::Engine;
use flume_seams::*;

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

    /// Whether a start of `program` may take an account of the pool.
    pub fn usable(&self, program: &str) -> Usable {
        let mut of_program = self.running_program(program).peekable();
        if of_program.peek().is_none() {
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
    /// one with fewest of flume's agents, equals in order (145).
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
}

impl Engine {
    /// The account a start of `place`'s stage whose agent runs `program`
    /// takes: the strategy's choice over its pool's usable accounts, in
    /// order (144, 145; core.32).
    pub(crate) fn choose_account(
        &self,
        place: &PlaceRecord,
        program: Option<&str>,
        queue: &Queue,
    ) -> Choice {
        let stream = &place.stream.name;
        let Some(pool) = queue.pool_of(stream) else {
            return Choice::NoPool;
        };
        let Some(room) = queue.pools.get(pool) else {
            return Choice::Undeclared(format!(
                "the pool {pool} the stream {stream} takes its accounts from is not declared by the host's configuration"
            ));
        };
        let Some(program) = program else {
            return Choice::Wait;
        };
        match room.strategy(program) {
            Some(chosen) => Choice::Chose {
                held: AccountHeld {
                    pool: pool.to_string(),
                    name: chosen.account.name.clone(),
                    from: AccountFrom::Strategy,
                },
                account: chosen.account.clone(),
            },
            None => Choice::Wait,
        }
    }
}

impl Engine {
    /// The move of the stage at `index` from the account its last start
    /// ran under to `to`, in the same pool, where that account is spent or
    /// at its own bound now; none otherwise (56, 145, 146).
    pub(crate) fn stage_moved(
        &self,
        held: &HeldState,
        index: usize,
        to: Option<&AccountHeld>,
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
        })
    }
}
