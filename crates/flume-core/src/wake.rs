//! Role addresses and the wake (3, 7, 132, 143, 193, S38, S99; design 1,
//! 2; core.28). A role's address is `<stream>/<role>`, each part a word
//! of letters, digits, `.`, `_` and `-`: the role a source of the host
//! declares, in the roles file its declaration names, for a stream the
//! host declares; any other recipient's name is a session's. What a
//! source declares for an address is read as its roles file stands at
//! each wake: a run, a standing agent, or the address carried to another
//! host. A run is a place of its source's own kind, launched as the
//! identity `<source>/<kind>` and resolved and locked as a launch's kind
//! is, its version the locks', carrying no declaration of its own (154,
//! 155; core.34); in its repository, on its base (the repository's
//! default branch where it names none), named by its entry's `name`,
//! else `<stream>-<role>`, so two streams' runs of one role are two
//! places; in its address's stream, at its priority, serving its entry's
//! intent, its agents given its entry's settings as a launch's; with its
//! brief where it gives one, the address as its role and as its agents'
//! session name, and no recipient of its own; so at most one run of an
//! address stands at once (3). A run an earlier flume named by the role
//! alone is found by its role. A settle that finds an end pending for an
//! address with no run standing launches the run inside that settle, and
//! `flume wake <address>` does the same for another tool's carrier; a
//! wake of an address whose run is live or waiting makes nothing and says
//! so, and a launch refused because another settle's run of the address
//! now stands is that run, answered as waiting (75). An address no source
//! declares, two declare, whose stream the host does not declare or whose
//! entry is at fault is woken by nothing, and its wake's refusal and
//! status say why. An address a source declares carried wakes nothing on
//! this host, its ends kept pending for the tool that carries them. flume
//! decides nothing of what a run does (69, 70, 72). An address a source
//! gives a standing agent wakes no run, whatever its terminal finds and
//! whatever run of an earlier entry stands: a settle passes over it,
//! recording nothing, its ends delivered into that agent, and a carrier's
//! wake settles as any act does, launches nothing and answers that the
//! agent holds it, with where it lives and its pending ends; which agent
//! holds an address is its source's entry's at each delivery and each
//! wake (132; core.13, core.28; record 0039's amendment). A role's run
//! that stalls with its own stream's answerer standing behind it, so its
//! stall reaches no one, is named under the host's addresses (130, 193).

use crate::standing::Standing;
use crate::start::Starts;
use crate::{what_ended, AddressStatus, Answer, Engine, PendingEnd, RoleStatus, Step, Woke};
use flume_seams::*;
use std::collections::BTreeMap;

impl Engine {
    /// Wakes the role's address for a carrier: its run launched where
    /// none stands, and otherwise the run that stands, nothing made; and,
    /// for an address its source gives a standing agent, nothing
    /// launched, answered held with where the agent lives and the ends
    /// pending for it (132; core.28).
    pub fn wake(&self, address: &str) -> Answer {
        if !is_address(address) {
            return Answer::refused(format!(
                "{address} names no role's address, which is <stream>/<role>: a role a source of \
                 the host declares for a stream the host declares"
            ));
        }
        self.settle_attended();
        let roles = self.host.roles();
        if let Some(reach) = self.reach(&mut Standing::of(&roles), address) {
            let every = self.every_held();
            return Answer::Held {
                address: address.into(),
                standing: reach.status,
                pending: pending_addresses(&every, &self.owing())
                    .remove(address)
                    .map(|status| status.pending)
                    .unwrap_or_default(),
            };
        }
        match self.wake_address(address, &roles) {
            Ok((woke, held)) => Answer::Woken {
                address: address.into(),
                woke,
                place: self.place_status(&held),
            },
            Err(reason) => Answer::refused(reason),
        }
    }

    /// The wake both the settle and a carrier call, over the roles the
    /// sources declare as `roles` read them: the address's run that
    /// stands, or its run launched, or why none was (132, 193; design 2).
    pub(crate) fn wake_address(
        &self,
        address: &str,
        roles: &HostRoles,
    ) -> Result<(Woke, HeldState), String> {
        let run = match roles.declared(address) {
            Ok(RoleEntry::Run(run)) => *run,
            Ok(RoleEntry::Standing(agent)) => {
                return Err(format!(
                    "{address} wakes no run: {} gives it the standing agent {}",
                    agent.source, agent.agent
                ))
            }
            Ok(RoleEntry::Carried(carried)) => {
                return Err(format!(
                    "{address} wakes nothing on this host: {} declares it carried, its run on \
                     another host",
                    carried.source
                ))
            }
            Err(why) => return Err(format!("{address} wakes nothing: {why}")),
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
            name: run_name(address, &run),
            base,
            brief: run.brief.clone(),
            priority: run.priority,
            stream: Some(stream_of(address).into()),
            intent: run.intent.clone(),
            every_stage: run.agents.every_stage.clone(),
            settings: run.agents.stages.clone(),
            ..Launch::default()
        };
        match self.make_place(&launch, Some(address), None) {
            Ok(held) => Ok((Woke::Launched, held)),
            Err(reason) => match self.run_of(address, &run) {
                Some(held) => Ok((standing(&held), held)),
                None => Err(format!("the run of {address} was not launched: {reason}")),
            },
        }
    }

    /// The run of the address that stands, a tidied document of an
    /// earlier run aside: the place serving it, read first, so a run an
    /// earlier flume named by the role alone is found; else a place of no
    /// role's of the repository, kind and name its run would be made as,
    /// the kind's name as the kinds seam reads the role's `kind` (154),
    /// wherever it was made (3, 132, 193).
    fn run_of(&self, address: &str, run: &RoleRun) -> Option<HeldState> {
        let standing: Vec<HeldState> = self
            .every_held()
            .into_iter()
            .filter(|held| !matches!(held.phase, Phase::Tidied { .. }))
            .collect();
        if let Some(held) = standing
            .iter()
            .find(|held| held.role.as_deref() == Some(address))
        {
            return Some(held.clone());
        }
        let identity = role_kind(run);
        let kind = self
            .resolver
            .kinds()
            .read_named(&identity)
            .map_or(identity, |named| named.name);
        let short = format!("{kind}--{}", run_name(address, run));
        standing.into_iter().find(|held| {
            held.role.is_none()
                && held.place.repository == run.repository
                && held.place.short() == short
        })
    }

    /// The settle's wake step (design 2): each role's address an end of
    /// the place stands pending for, and that nothing has handed it to, is
    /// woken, and why one was not is kept
    /// on the place for status, gone once it is woken; an address its
    /// source gives a standing agent or declares carried, the roles read
    /// once, is passed over, nothing launched and nothing kept (43, 132,
    /// 193; core.28).
    pub(crate) fn wake_roles(&self, id: &PlaceId) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        if matches!(held.phase, Phase::Making { .. }) {
            return;
        }
        // A drop's or a done's end stands pending for no role's address, so
        // it wakes no role; and an end anything handed to an address, a
        // notice, a wait or a run's inbox, wakes no run for it again, so a
        // run that never confirms wakes no run after it (43, 52, 114, 132,
        // S122; core.28).
        let owing = self.owing();
        let mut addresses: Vec<String> = vec![];
        for end in &held.ends {
            for address in end.pending(&held.place, &owing) {
                if is_address(&address)
                    && !end.handed_to(&held.place, &owing, &address)
                    && !addresses.contains(&address)
                {
                    addresses.push(address);
                }
            }
        }
        let roles = if addresses.is_empty() {
            HostRoles::default()
        } else {
            self.host.roles()
        };
        let mut unwoken = BTreeMap::new();
        for address in addresses {
            if matches!(
                roles.declared(&address),
                Ok(RoleEntry::Standing(_) | RoleEntry::Carried(_))
            ) {
                continue;
            }
            if let Err(reason) = self.wake_address(&address, &roles) {
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
    pub(crate) fn addresses(&self, roles: &HostRoles, owing: &Owing) -> Vec<AddressStatus> {
        let every = self.every_held();
        let mut by = pending_addresses(&every, owing);
        for (address, stall) in unheard_stalls(&every) {
            by.entry(address.clone())
                .or_insert_with(|| AddressStatus::of(address))
                .stalled = Some(stall);
        }
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
                    Ok(RoleEntry::Run(run)) => {
                        status.role = Some(RoleStatus {
                            run: *run,
                            stream: stream_of(&status.address).to_string(),
                        })
                    }
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
fn pending_addresses(every: &[HeldState], owing: &Owing) -> BTreeMap<String, AddressStatus> {
    let mut by: BTreeMap<String, AddressStatus> = BTreeMap::new();
    for held in every {
        for end in &held.ends {
            for address in end.pending(&held.place, owing) {
                if !is_address(&address) {
                    continue;
                }
                let status = by
                    .entry(address.clone())
                    .or_insert_with(|| AddressStatus::of(address.clone()));
                status.pending.push(PendingEnd {
                    place: held.place.id.clone(),
                    key: end.of.end_key(),
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

/// Each role's run of `every` that stands, by its address, whose newest
/// stall reached no recipient, its run being its own stream's answerer
/// or no role standing behind its stream, with that stall (130, 193).
fn unheard_stalls(every: &[HeldState]) -> Vec<(String, PendingEnd)> {
    every
        .iter()
        .filter(|held| !matches!(held.phase, Phase::Tidied { .. }))
        .filter_map(|held| {
            let address = held.role.clone()?;
            let stall = held
                .ends
                .iter()
                .rev()
                .find(|end| matches!(end.of, EndOf::Stall { .. }))?;
            stall
                .recipients(&held.place.recipients)
                .is_empty()
                .then(|| {
                    (
                        address,
                        PendingEnd {
                            place: held.place.id.clone(),
                            key: stall.of.end_key(),
                            what: what_ended(&stall.of),
                        },
                    )
                })
        })
        .collect()
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
            stalled: None,
        }
    }
}

/// The name a role's run is made under: its entry's `name`, else
/// `<stream>-<role>`, so two streams' runs of one role are two places (3,
/// 132, 193).
fn run_name(address: &str, run: &RoleRun) -> String {
    run.name
        .clone()
        .unwrap_or_else(|| format!("{}-{}", stream_of(address), role_of(address)))
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
