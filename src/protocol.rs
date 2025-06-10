use avian2d::prelude::{
    AngularVelocity, ColliderDisabled, LinearVelocity, Position, RigidBody, RigidBodyDisabled,
    Rotation,
};
use bevy::ecs::component::Component;
use bevy::prelude::*;
use lightyear::{
    prelude::{
        client::{ComponentSyncMode, LerpFn},
        *,
    },
    utils::{
        avian2d::{angular_velocity, linear_velocity, position, rotation},
        bevy::TransformLinearInterpolation,
    },
};

// #[derive(Channel)]
// pub(crate) struct ReliableChannel;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct JoinGameMessage {
    player_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct ChatMessage(pub String);

/// A component that will identify which player the controller is.
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct PlayerId(pub(crate) ClientId);

pub(super) fn plugin(app: &mut App) {
    // app.add_channel::<ReliableChannel>(ChannelSettings {
    //     mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
    //     ..default()
    // });

    app.register_component::<PlayerId>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Once)
        .add_interpolation(ComponentSyncMode::Once);

    app.register_component::<RigidBody>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    // Position and Rotation have a `correction_fn` set, which is used to smear rollback errors
    // over a few frames, just for the rendering part in postudpate.
    //
    // They also set `interpolation_fn` which is used by the VisualInterpolationPlugin to smooth
    // out rendering between fixedupdate ticks.
    // app.register_component::<Position>(ChannelDirection::ServerToClient)
    //     .add_prediction(ComponentSyncMode::Full)
    //     .add_interpolation(ComponentSyncMode::Full)
    //     .add_interpolation_fn(position::lerp)
    //     .add_correction_fn(position::lerp);
    app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(linear_velocity::lerp)
        .add_correction_fn(linear_velocity::lerp);

    // app.register_component::<Rotation>(ChannelDirection::ServerToClient)
    //     .add_prediction(ComponentSyncMode::Full)
    //     .add_interpolation(ComponentSyncMode::Full)
    //     .add_interpolation_fn(rotation::lerp)
    //     .add_correction_fn(rotation::lerp);
    app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(angular_velocity::lerp)
        .add_correction_fn(angular_velocity::lerp);

    app.register_component::<Transform>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(TransformLinearInterpolation::lerp)
        .add_correction_fn(TransformLinearInterpolation::lerp);

    // do not replicate Transform but make sure to register an interpolation function
    // for it so that we can do visual interpolation
    // (another option would be to replicate transform and not use Position/Rotation at all)
    // app.add_interpolation::<Transform>(ComponentSyncMode::None);
    // app.add_interpolation_fn::<Transform>(TransformLinearInterpolation::lerp);
}
