use avian2d::prelude::{AngularVelocity, LinearVelocity, RigidBody};
use bevy::ecs::component::Component;
use bevy::prelude::*;
use lightyear::{
    prelude::{
        client::{ComponentSyncMode, LerpFn},
        *,
    },
    utils::{
        // avian2d::{angular_velocity, linear_velocity},
        bevy::TransformLinearInterpolation,
    },
};

use crate::{Ship, shared::SetCollider};

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

    app.register_component::<Name>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Once)
        .add_interpolation(ComponentSyncMode::Once);
    app.register_component::<Ship>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Once)
        .add_interpolation(ComponentSyncMode::Once);

    // app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
    //     .add_prediction(ComponentSyncMode::Once)
    //     .add_interpolation(ComponentSyncMode::Once);

    app.register_component::<RigidBody>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<SetCollider>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full)
        // .add_interpolation(ComponentSyncMode::Full)
        // .add_interpolation_fn(linear_velocity::lerp)
        // .add_correction_fn(linear_velocity::lerp);
        ;

    app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full)
        // .add_interpolation(ComponentSyncMode::Full)
        // .add_interpolation_fn(angular_velocity::lerp)
        // .add_correction_fn(angular_velocity::lerp);
        ;

    app.register_component::<Transform>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Full)
        // .add_interpolation(ComponentSyncMode::Full)
        // .add_interpolation_fn(TransformLinearInterpolation::lerp)
        // .add_correction_fn(TransformLinearInterpolation::lerp);
        ;
}
