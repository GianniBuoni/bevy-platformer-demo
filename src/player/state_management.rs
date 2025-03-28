use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (player_state).in_set(UpdateSets::StateManagement));
}

#[derive(Component, Default, Clone, PartialEq, Eq, Debug)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
    Jump,
    Fall,
}

#[derive(Component, Default)]
pub struct PlayerStateTransition(pub PlayerState);

impl PlayerStateTransition {
    pub fn new(state: PlayerState) -> Self {
        Self(state)
    }
}

/// Inital pass of state management based on input and
/// the character controller.
/// Certain animation dependant states are hanled and validated in the draw stage.
fn player_state(
    mut player: Query<
        (Entity, &TnuaController, &PlayerInput, &mut PlayerState),
        With<Player>,
    >,
    mut commands: Commands,
) {
    let (player, controller, input, state) = get_single_mut!(player);
    let old = state.clone();

    if old == PlayerState::Jump {
        return;
    }

    let mut new = PlayerState::default();
    if input.x != 0. {
        new = PlayerState::Run;
    }
    if let Ok(airborne) = controller.is_airborne() {
        new = if airborne { PlayerState::Fall } else { new };
    }
    if let Some(action) = controller.action_flow_status().ongoing() {
        new = if action == "TnuaBuiltinJump" {
            PlayerState::Jump
        } else {
            new
        }
    }
    if old != new {
        commands
            .entity(player)
            .insert(PlayerStateTransition::new(new));
    }
}
