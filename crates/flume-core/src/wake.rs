//! Role addresses and the wake (132, C.7, S38; design 8; core.28). A
//! recipient named `<partition>/<role>` is a role's address. A run of the
//! role is a place of the kind the host's configuration names for the
//! address, in its repository, on its base (the repository's default
//! branch where it names none), named by the role, at its priority, with
//! its brief where it gives one, the address as its role and as its
//! agents' session name, and no recipient of its own; so at most one run
//! of an address stands at once (3). A settle that finds an end pending
//! for an address with no run standing launches the run inside that
//! settle, and `flume wake <address>` does the same for another tool's
//! carrier; a wake of an address whose run is live or waiting makes
//! nothing and says so, and a launch refused because another settle's run
//! of the address now stands is that run, answered as waiting (75). An
//! address the configuration names nothing for is woken by nothing, and
//! status names its pending ends and why its run was not woken. A role's
//! kind is its source's own, launched as the identity `<source>/<kind>`
//! and resolved and locked as a launch's kind is, its version the
//! locks', its run carrying no declaration of its own (154, 155, 193;
//! core.34). flume
//! decides nothing of what the run does (69, 70, 72). An address the
//! configuration gives a standing agent wakes no run, whatever its
//! terminal finds and whatever run of an earlier entry stands: a settle
//! passes over it, recording nothing, and a carrier's wake settles as
//! any act does, launches nothing and answers that the agent holds it,
//! with where it lives and its pending ends; which agent holds an
//! address is the configuration's at each delivery and each wake (132;
//! core.13, core.28).

use crate::standing::Standing;
use crate::start::Starts;
use crate::{what_ended, AddressStatus, Answer, Engine, PendingEnd, Step, Woke};
use flume_seams::*;
use std::collections::BTreeMap;

impl Engine {
    /// Wakes the role's address for a carrier: its run launched where
    /// none stands, and otherwise the run that stands, nothing made; and,
    /// for an address the configuration gives a standing agent, nothing
    /// launched, answered held with where the agent lives and the ends
    /// pending for it (132; core.28).
    pub fn wake(&self, address: &str) -> Answer {
        if !is_address(address) {
            return Answer::refused(format!(
                "{address} names no role's address, which is <partition>/<role>"
            ));
        }
        self.settle_attended();
        if let Some(reach) = self.reach(&mut self.standing(), address) {
            let every = self.every_held();
            return Answer::Held {
                address: address.into(),
                standing: reach.status,
                pending: pending_addresses(&every)
                    .remove(address)
                    .map(|status| status.pending)
                    .unwrap_or_default(),
            };
        }
        match self.wake_address(address) {
            Ok((woke, held)) => Answer::Woken {
                address: address.into(),
                woke,
                place: self.place_status(&held),
            },
            Err(reason) => Answer::refused(reason),
        }
    }

    /// The wake both the settle and a carrier call: the address's run
    /// that stands, or its run launched, or why none was (132; design 8).
    pub(crate) fn wake_address(&self, address: &str) -> Result<(Woke, HeldState), String> {
        let Some(run) = self.host.role(address) else {
            return Err(format!(
                "the host's configuration names no role for {address}"
            ));
        };
        if let Some(held) = self.run_of(address, &run) {
            return Ok((standing(&held), held));
        }
        let base = match &run.base {
            Some(base) => base.clone(),
            None => self
                .history
                .default_branch(&run.repository)
                .ok_or_else(|| {
                    format!(
                        "the role {address} names no base, and git names no default branch of {}",
                        run.repository
                    )
                })?,
        };
        let launch = Launch {
            repository: run.repository.clone(),
            kind: role_kind(&run),
            name: role_of(address).into(),
            base,
            brief: run.brief.clone(),
            priority: run.priority,
            stream: Some(stream_of(address).into()),
            ..Launch::default()
        };
        match self.make_place(&launch, Some(address)) {
            Ok(held) => Ok((Woke::Launched, held)),
            Err(reason) => match self.run_of(address, &run) {
                Some(held) => Ok((standing(&held), held)),
                None => Err(format!("the run of {address} was not launched: {reason}")),
            },
        }
    }

    /// The run of the address that stands: the place serving it, or a
    /// place of the repository, kind and name its run would be made as,
    /// the kind's name as the kinds seam reads the role's `kind` (154),
    /// wherever it was made, a tidied document of an earlier run aside
    /// (3, 132).
    fn run_of(&self, address: &str, run: &RoleRun) -> Option<HeldState> {
        let identity = role_kind(run);
        let kind = self
            .resolver
            .kinds()
            .read_named(&identity)
            .map_or(identity, |named| named.name);
        let short = format!("{kind}--{}", role_of(address));
        self.store
            .read_every()
            .into_iter()
            .map(|read| read.state)
            .filter(|held| !matches!(held.phase, Phase::Tidied { .. }))
            .find(|held| {
                held.role.as_deref() == Some(address)
                    || (held.place.repository == run.repository && held.place.short() == short)
            })
    }

    /// The settle's wake step (design 2): each role's address an end of
    /// the place stands pending for is woken, and why one was not is kept
    /// on the place for status, gone once it is woken; an address the
    /// standing entries name, read once, is passed over, nothing launched
    /// and nothing kept (43, 132; core.28).
    pub(crate) fn wake_roles(&self, id: &PlaceId) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        if matches!(held.phase, Phase::Making { .. }) {
            return;
        }
        let addresses: Vec<String> = held
            .pending_for()
            .into_iter()
            .filter(|name| is_address(name))
            .collect();
        let standing = (!addresses.is_empty()).then(|| self.standing());
        let mut unwoken = BTreeMap::new();
        for address in addresses {
            if standing
                .as_ref()
                .is_some_and(|standing| standing.holds(&address))
            {
                continue;
            }
            if let Err(reason) = self.wake_address(&address) {
                unwoken.insert(address, reason);
            }
        }
        if unwoken != held.unwoken {
            self.update(id, |held| {
                held.unwoken = unwoken.clone();
                Step::Write(())
            });
        }
    }

    /// Each role's address an end stands pending for, of every place on
    /// the host, and each a source gives a standing agent, pending or
    /// not, as `roles`, read once for the status, declares them: with the
    /// run a source declares for it, its run standing and why it was not
    /// woken; for a standing address, where its agent lives, each
    /// terminal and part asked once; for a carried one, its entry; and,
    /// where no source declares anything for it, why (56, 132, 190, 193;
    /// core.15, core.28).
    pub(crate) fn addresses(&self, roles: &HostRoles) -> Vec<AddressStatus> {
        let every = self.every_held();
        let mut by = pending_addresses(&every);
        let mut standing = Standing::of(roles);
        for address in standing.addresses() {
            by.entry(address.clone())
                .or_insert_with(|| AddressStatus::of(address));
        }
        for status in by.values_mut() {
            status.standing = self
                .reach(&mut standing, &status.address)
                .map(|reach| reach.status);
            if status.standing.is_some() {
                status.unwoken = None;
            } else {
                match roles.declared(&status.address) {
                    Ok(RoleEntry::Run(run)) => status.role = Some(*run),
                    Ok(RoleEntry::Carried(carried)) => status.carried = Some(carried),
                    Ok(RoleEntry::Standing(_)) => {}
                    Err(why) => status.wakes_nothing = Some(why),
                }
            }
            status.run = every
                .iter()
                .filter(|held| !matches!(held.phase, Phase::Tidied { .. }))
                .find(|held| held.role.as_deref() == Some(status.address.as_str()))
                .map(|held| held.place.id.clone());
        }
        by.into_values().collect()
    }
}

impl Engine {
    /// Every place's held state on the host, tidied documents among them.
    fn every_held(&self) -> Vec<HeldState> {
        self.store
            .read_every()
            .into_iter()
            .map(|read| read.state)
            .collect()
    }
}

/// Each role's address an end of `every` stands pending for, with those
/// ends and why the last settle did not wake it, by the address (43,
/// 132).
fn pending_addresses(every: &[HeldState]) -> BTreeMap<String, AddressStatus> {
    let mut by: BTreeMap<String, AddressStatus> = BTreeMap::new();
    for held in every {
        for end in &held.ends {
            for address in end.pending(&held.place.recipients) {
                if !is_address(&address) {
                    continue;
                }
                let status = by
                    .entry(address.clone())
                    .or_insert_with(|| AddressStatus::of(address.clone()));
                status.pending.push(PendingEnd {
                    place: held.place.id.clone(),
                    key: end.of.key(),
                    what: what_ended(&end.of),
                });
                if status.unwoken.is_none() {
                    status.unwoken = held.unwoken.get(&address).cloned();
                }
            }
        }
    }
    by
}

impl AddressStatus {
    /// The address with nothing read of it yet.
    fn of(address: String) -> AddressStatus {
        AddressStatus {
            address,
            role: None,
            run: None,
            pending: vec![],
            unwoken: None,
            standing: None,
            carried: None,
            wakes_nothing: None,
        }
    }
}

/// The kind a role's run is launched as: its source's own, by the
/// identity `<source>/<kind>`, resolved and locked as a launch's kind is,
/// its version the locks' (154, 155, 193; core.34).
fn role_kind(run: &RoleRun) -> String {
    format!("{}/{}", run.source, run.kind)
}

/// A standing run as a wake answers it: live while its agent is, and
/// otherwise waiting, for room, on its caller, or for its launch to
/// finish (132).
fn standing(held: &HeldState) -> Woke {
    if held.phase == Phase::Live && held.live_start().is_some() {
        Woke::Live
    } else {
        Woke::Waiting
    }
}
