use avian2d::prelude::Collider;
use bevy::prelude::*;
use lightyear::prelude::{client::Predicted, server::ReplicateToClient, *};
use std::time::Duration;

pub(crate) const FIXED_TIMESTEP_HZ: f64 = 50.0; // should be 50!
pub(crate) const FIXED_TIMESTEP_DURATION: Duration =
    Duration::from_millis(1000 / FIXED_TIMESTEP_HZ as u64);

pub(crate) fn shared_config() -> SharedConfig {
    SharedConfig {
        client_replication_send_interval: FIXED_TIMESTEP_DURATION,
        server_replication_send_interval: FIXED_TIMESTEP_DURATION,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        },
    }
}

#[derive(Component, Reflect, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub(crate) enum SetCollider {
    /// Radius
    Circle(f32),
    Rectangle(f32, f32),
}

impl Default for SetCollider {
    fn default() -> Self {
        Self::Circle(1.0)
    }
}

pub(crate) fn on_set_collider(
    mut commands: Commands,
    query: Query<
        (Entity, &SetCollider),
        (
            Or<(Added<SetCollider>, Changed<SetCollider>)>,
            Or<(With<ReplicateToClient>, With<Predicted>)>,
        ),
    >,
) {
    for (entity, set_collider) in &query {
        match set_collider {
            SetCollider::Circle(radius) => {
                commands.entity(entity).insert(Collider::circle(*radius));
            }
            SetCollider::Rectangle(width, height) => {
                commands
                    .entity(entity)
                    .insert(Collider::rectangle(*width, *height));
            }
        }
    }
}
