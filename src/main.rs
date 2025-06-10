pub(crate) mod client;
pub(crate) mod pilot_action;
pub(crate) mod protocol;
pub(crate) mod server;
pub(crate) mod shared;

use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use client::lightyear_client_plugin;
use lightyear::prelude::{client::ClientCommandsExt, *};
use pilot_action::PilotAction;
use server::{
    handle_connections, lightyear_server_plugin, spawn_multiplayer_scene, start_dedicated_server,
};
use shared::on_set_collider;

enum NetMode {
    HostClient,
    Server,
    Client,
}

impl NetMode {
    fn from_arg(arg: &str) -> Self {
        match arg.to_uppercase().as_str() {
            "H" => NetMode::HostClient,
            "S" => NetMode::Server,
            "C" => NetMode::Client,
            _ => panic!("Invalid argument: {arg}"),
        }
    }
}

fn main() -> AppExit {
    let first_arg = std::env::args()
        .nth(1)
        .expect("Missing argument: H, C or S");
    let net_mode = NetMode::from_arg(&first_arg);

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(PhysicsPlugins::default());

    match net_mode {
        NetMode::HostClient => {
            app.add_plugins(lightyear_server_plugin());
            app.add_systems(
                Startup,
                (start_dedicated_server, spawn_multiplayer_scene).chain(),
            );
            app.add_systems(Update, handle_connections);
        }
        NetMode::Server => {
            app.add_plugins(lightyear_server_plugin());
            app.add_systems(
                Startup,
                (start_dedicated_server, spawn_multiplayer_scene).chain(),
            );
            app.add_systems(Update, handle_connections);
        }
        NetMode::Client => {
            app.add_plugins(lightyear_client_plugin);

            app.add_systems(Startup, |mut commands: Commands| {
                commands.connect_client();
            });
        }
    }

    app.add_plugins(LeafwingInputPlugin::<PilotAction> {
        config: InputConfig::<PilotAction> {
            rebroadcast_inputs: true,
            ..default()
        },
    });
    app.add_plugins(protocol::plugin);

    // app.insert_resource(avian2d::sync::SyncConfig {
    //     transform_to_position: true,
    //     position_to_transform: true,
    //     ..default()
    // });

    app.add_systems(Update, on_set_collider);

    app.run()
}
