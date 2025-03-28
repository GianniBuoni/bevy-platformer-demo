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
pub struct PlayerStateTransition;

fn player_state(
    mut player: Query<
        (Entity, &TnuaController, &PlayerInput, &mut PlayerState),
        With<Player>,
    >,
    mut commands: Commands,
) {
    let (player, controller, input, mut state) = get_single_mut!(player);
    let old = state.clone();
    let mut new = PlayerState::default();

    if input.x != 0. {
        new = PlayerState::Run;
    }
    if let Ok(airborne) = controller.is_airborne() {
        if airborne {
            new = PlayerState::Fall;
        }
    }
    if old != new {
        commands.entity(player).insert(PlayerStateTransition);
    }
    *state = new;
}
