use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub(crate) enum PilotAction {
    Forward,     // eg "w"
    Backward,    // eg "s"
    StrafeLeft,  // eg "q"
    StrafeRight, // eg "e"
    TurnLeft,    // eg "a"
    TurnRight,   // eg "d"
    Turbo,       // eg "shift"
    Shoot,       // eg "lmb"
}

impl PilotAction {
    pub(crate) fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::Forward, KeyCode::KeyW);
        input_map.insert(Self::Backward, KeyCode::KeyS);
        input_map.insert(Self::StrafeLeft, KeyCode::KeyQ);
        input_map.insert(Self::StrafeRight, KeyCode::KeyE);
        input_map.insert(Self::TurnLeft, KeyCode::KeyA);
        input_map.insert(Self::TurnRight, KeyCode::KeyD);
        input_map.insert(Self::Turbo, KeyCode::ShiftLeft);
        input_map.insert(Self::Shoot, MouseButton::Left);

        input_map
    }
}

impl Actionlike for PilotAction {
    fn input_control_kind(&self) -> InputControlKind {
        // We're using a match statement here
        // because in larger projects, you will likely have
        // different input control kinds for different actions
        InputControlKind::Button
    }
}
