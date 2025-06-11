use avian2d::prelude::{AngularVelocity, LinearVelocity, RigidBody};
use bevy::{color::palettes::tailwind::ORANGE_500, prelude::*};
use lightyear::prelude::server::*;
use lightyear::prelude::*;

use crate::{
    protocol::PlayerId,
    shared::{Astro, SetCollider, Ship, shared_config},
};

pub(crate) fn lightyear_server_plugin() -> ServerPlugins {
    let config = ServerConfig {
        shared: shared_config(),
        ..default()
    };
    ServerPlugins::new(config)
}

pub(crate) fn start_dedicated_server(
    mut commands: Commands,
    mut server_config: ResMut<ServerConfig>,
) {
    info!("Grabby Aliens Server");

    let net_config = NetConfig::Netcode {
        io: IoConfig {
            transport: ServerTransport::UdpSocket("127.0.0.1:8080".parse().unwrap()),
            // conditioner: server_run_config.multiplayer.conditioner(),
            ..default()
        },
        config: NetcodeConfig::default(),
    };

    *server_config = ServerConfig {
        shared: shared_config(),
        net: vec![net_config],
        ..default()
    };

    commands.start_server();
}

pub(crate) fn spawn_multiplayer_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let replicate = Replicate {
        sync: SyncTarget {
            prediction: NetworkTarget::All,
            // interpolation: NetworkTarget::All,
            ..default()
        },
        ..default()
    };

    /*
    commands.spawn((
                Mesh2d(shape),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    0.0,
                    0.0,
                ),
            ));
    */
    let ship_entity = commands
        .spawn((
            Name::new("Ship---!"),
            Ship,
            replicate.clone(),
            RigidBody::Dynamic,
            Transform::from_translation(Vec3::new(-2.0, 0., 0.)),
            SetCollider::Rectangle(0.5, 1.0),
            AngularVelocity(1.1),
            LinearVelocity(Vec2::X * 0.1),
            Mesh2d(meshes.add(Rectangle::new(0.5, 1.0))),
            MeshMaterial2d(materials.add(Color::from(ORANGE_500).with_alpha(0.5))),
        ))
        .id();
}

pub(crate) fn handle_connections(
    mut commands: Commands,
    mut connections: EventReader<ConnectEvent>,
    // test_ship: Single<Entity, With<TestSpawnInThisShip>>,
) {
    for connection in connections.read() {
        let client_id = connection.client_id;

        info!("spawning crew for {client_id}");
        let replicate = Replicate {
            sync: SyncTarget {
                prediction: NetworkTarget::All,
                // interpolation: NetworkTarget::All,
                ..default()
            },
            controlled_by: ControlledBy {
                target: NetworkTarget::Single(client_id),
                ..default()
            },
            ..default()
        };

        let network_player_entity = commands
            .spawn((
                Name::new("Network PlayerId"),
                PlayerId(client_id),
                replicate.clone(),
            ))
            .id();
        info!(
            "Create entity {:?} for client {:?}",
            network_player_entity, client_id
        );
        // boss_state.insert(client_id, network_player_entity);

        let astro = commands
            .spawn((
                Name::new("Player Owned Astro"),
                Astro,
                replicate.clone(),
                // RelationshipSync::<ChildOf>::from(Some(*test_ship)),
            ))
            .id();

        // commands
        //     .entity(network_player_entity)
        //     .insert((replicate, ControllerOf(astro)));

        // spawn_completed_events.write(SpawnCompletedEvent(Buildable::existing_in_space(
        //     astro,
        //     //add jetpack when in space!
        //     GlobalPosition::random_in_circle_uniform(5.meters()),
        //     blueprints.get("Crew").unwrap().clone(),
        // )));

        // spawn_completed_events.write(SpawnCompletedEvent(Buildable::existing_in_structure(
        //     astro,
        //     *test_ship,
        //     GridPosition::new(5, 0),
        //     blueprints.get("Crew").unwrap().clone(),
        // )));
    }
}
