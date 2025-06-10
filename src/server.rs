use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

use crate::{
    protocol::PlayerId,
    shared::{SetCollider, shared_config},
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

pub(crate) fn spawn_multiplayer_scene(mut commands: Commands) {
    let replicate = Replicate {
        sync: SyncTarget {
            prediction: NetworkTarget::All,
            // interpolation: NetworkTarget::All,
            ..default()
        },
        ..default()
    };

    let ship_entity = commands
        .spawn((
            replicate.clone(),
            RigidBody::Dynamic,
            SetCollider::Rectangle(0.5, 1.0),
            // AngularVelocity(10.1),
            // LinearVelocity(Vec2::X * 10.0),
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
