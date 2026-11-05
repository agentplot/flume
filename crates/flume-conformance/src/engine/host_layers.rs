//! The host's configuration in two layers as the core gives it (C.7;
//! core.45): status gives every value with the layer it came from, an
//! account's variables by name and `<set>`, whether the local layer was
//! taken, and each local key passed over with its pin (56; core.30); and
//! the next start finds room for the bound the host's configuration
//! gives as it stands then (32, 70).

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, HostStatus, RoomWanted};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "host layers: status gives each value with its layer, the local layer taken and a key passed over with its pin, and the next start finds room for the bound the layers give",
            cites: &["S83", "56", "70"],
            run: s83_both_layers_in_status,
        },
        Scenario {
            name: "host layers: status gives a local layer not taken with each key it may not set, and every value from the managed layer",
            cites: &["S83", "56"],
            run: a_local_layer_not_taken,
        },
    ]
}

/// The host's part of status.
fn host(world: &World) -> Result<HostStatus, String> {
    match world.engine().status(None) {
        Answer::Status { host, .. } => Ok(host),
        other => Err(format!("status answered {other:?}")),
    }
}

fn value(key: &str, value: &str, layer: ConfigLayer) -> ConfigValue {
    ConfigValue {
        key: key.into(),
        value: value.into(),
        layer,
    }
}

/// The managed layer's values: `work`'s pool and its account's token.
fn managed() -> Vec<ConfigValue> {
    vec![
        value(
            "pools.work.accounts[0].env.TOKEN",
            SET,
            ConfigLayer::Managed,
        ),
        value("streams.work.pool", "\"work\"", ConfigLayer::Managed),
    ]
}

/// The fix `name` launched on `work` and settled once.
fn fix_on_work(world: &World, name: &str) -> Result<PlaceId, String> {
    launched(&world.engine.launch(&Launch {
        stream: Some("work".into()),
        ..launch("fix", name, Some("x"))
    }))?;
    let id = World::id("fix", name);
    world.settle(&id);
    Ok(id)
}

fn s83_both_layers_in_status(_: &()) -> Outcome {
    let world = World::new();
    let mut values = managed();
    values.push(value("streams.work.agents", "3", ConfigLayer::Local));
    values.push(value("streams.work.priority", "30", ConfigLayer::Local));
    let configuration = HostConfiguration {
        local: LocalLayer::Taken,
        values,
        passed_over: vec![PinnedKey {
            key: "streams.work.session".into(),
            pin: "streams.work".into(),
        }],
    };
    world.host.set_configuration(configuration.clone());
    world.host.set_stream(
        "work",
        Some(Stream {
            agents: Some(3),
            priority: 30,
            ..Stream::default()
        }),
    );
    let status = host(&world)?;
    ensure_eq!(
        status.configuration,
        configuration,
        "the configuration in status"
    );
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    let given = &json["configuration"];
    ensure_eq!(
        given["local"],
        serde_json::json!({"state": "taken"}),
        "the local layer's state"
    );
    ensure_eq!(
        given["values"][0],
        serde_json::json!({
            "key": "pools.work.accounts[0].env.TOKEN",
            "value": "<set>",
            "layer": "managed",
        }),
        "the account's variable, by name and <set>"
    );
    ensure_eq!(
        given["values"][2],
        serde_json::json!({"key": "streams.work.agents", "value": "3", "layer": "local"}),
        "work's agents from the local layer"
    );
    ensure_eq!(
        given["passed_over"],
        serde_json::json!([{"key": "streams.work.session", "pin": "streams.work"}]),
        "the key passed over with its pin"
    );
    let places: Vec<PlaceId> = ["one", "two", "three", "four"]
        .into_iter()
        .map(|name| fix_on_work(&world, name))
        .collect::<Result<_, _>>()?;
    ensure_eq!(
        world.stage_state(&places[3], "code"),
        Some(StageState::WaitingForRoom),
        "the fourth fix's code, past work's three"
    );
    ensure_eq!(
        host(&world)?
            .queue
            .first()
            .map(|queued| queued.wants.clone()),
        Some(RoomWanted::Stream {
            stream: "work".into(),
            bound: 3,
        }),
        "what the fourth fix waits for"
    );
    world.host.set_stream(
        "work",
        Some(Stream {
            agents: Some(4),
            priority: 30,
            ..Stream::default()
        }),
    );
    world
        .host
        .set_value("streams.work.agents", "4", ConfigLayer::Local);
    world.engine().settle(None);
    ensure!(
        world
            .stage_state(&places[3], "code")
            .is_some_and(StageState::running),
        "the fourth fix did not start once the layers give work four"
    );
    Ok(())
}

fn a_local_layer_not_taken(_: &()) -> Outcome {
    let world = World::new();
    let configuration = HostConfiguration {
        local: LocalLayer::NotTaken {
            keys: vec!["pools.work".into()],
            fault: None,
        },
        values: managed(),
        passed_over: vec![],
    };
    world.host.set_configuration(configuration.clone());
    let status = host(&world)?;
    ensure_eq!(
        status.configuration,
        configuration,
        "the configuration in status"
    );
    ensure!(
        status
            .configuration
            .values
            .iter()
            .all(|value| value.layer == ConfigLayer::Managed),
        "a value from the local layer: {:?}",
        status.configuration.values
    );
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    ensure_eq!(
        json["configuration"]["local"],
        serde_json::json!({"state": "not-taken", "keys": ["pools.work"]}),
        "the local layer not taken, naming its key"
    );
    Ok(())
}
