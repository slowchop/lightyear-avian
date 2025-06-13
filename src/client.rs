use crate::Ship;
use crate::shared::shared_config;
use avian2d::prelude::{ColliderDisabled, RigidBody, RigidBodyDisabled};
use bevy::color::palettes::css::DARK_SLATE_GREY;
use bevy::color::palettes::tailwind::ORANGE_500;
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
        sync: SyncConfig {
            error_margin: 10.0,
            max_error_margin: 1000.0,
            ..default()
        },
        ..default()
    };
    app.add_plugins(ClientPlugins::new(config));
    app.add_systems(Update, disable_physics_on_confirmed_entities);
}

pub(crate) fn insert_ship_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, Has<Replicated>), (With<Ship>, Without<Mesh2d>)>,
) {
    for (entity, has_replicated) in &query {
        let color = if has_replicated {
            Color::from(DARK_SLATE_GREY).with_alpha(0.1)
        } else {
            Color::from(ORANGE_500).with_alpha(0.5)
        };
        commands.entity(entity).insert((
            Mesh2d(meshes.add(Rectangle::new(0.5, 1.0))),
            MeshMaterial2d(materials.add(color)),
        ));
    }
}

/// Disable physics on confirmed entities to prevent rollback conflicts
/// Physics should only run on predicted entities, not on confirmed server entities
pub(crate) fn disable_physics_on_confirmed_entities(
    mut commands: Commands,
    confirmed_entities: Query<
        Entity,
        (With<Confirmed>, With<RigidBody>, Without<RigidBodyDisabled>),
    >,
) {
    for entity in &confirmed_entities {
        commands
            .entity(entity)
            .insert(RigidBodyDisabled)
            .insert(ColliderDisabled);
    }
}
