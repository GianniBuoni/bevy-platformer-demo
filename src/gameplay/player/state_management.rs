use crate::prelude::*;

use super::{Player, PlayerInput};

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
pub struct PlayerStateTransition;

fn player_state(
    mut player: Query<
        (Entity, &TnuaController, &mut PlayerState),
        With<Player>,
    >,
    mut commands: Commands,
    input: Res<PlayerInput>,
) {
    let (player, controller, mut state) = get_single_mut!(player);
    let mut new: PlayerState = PlayerState::default();
    let old = state.clone();

    if input.movement_vector.x != 0. {
        new = PlayerState::Run;
    }
    let Ok(airborne) = controller.is_airborne() else {
        return;
    };
    if airborne {
        new = PlayerState::Fall;
    }
    if input.jump {
        new = PlayerState::Jump;
    }
    if new != old {
        commands.entity(player).insert(PlayerStateTransition);
    } else {
        commands.entity(player).remove::<PlayerStateTransition>();
    }
    *state = new;
}
