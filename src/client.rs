use crate::shared::shared_config;
use bevy::prelude::*;
use lightyear::connection::netcode::PRIVATE_KEY_BYTES;
use lightyear::prelude::client::*;
use lightyear::prelude::*;

pub(crate) fn lightyear_client_plugin(app: &mut App) {
    let config = ClientConfig {
        shared: shared_config(),
        net: NetConfig::Netcode {
            auth: Authentication::Manual {
                server_addr: "127.0.0.1:8080".parse().unwrap(),
                client_id: 1,
                private_key: [0; PRIVATE_KEY_BYTES],
                protocol_id: 0,
            },
            config: NetcodeConfig::default(),

            io: IoConfig {
                transport: ClientTransport::UdpSocket("127.0.0.1:0".parse().unwrap()),
                // conditioner: server_run_config.multiplayer.conditioner(),
                ..default()
            },
        },
        // Look at the connect code to change these.
        prediction: PredictionConfig::no_input_delay(),
        ..default()
    };
    app.add_plugins(ClientPlugins::new(config));
}
