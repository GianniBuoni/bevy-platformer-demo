use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (validate_state).in_set(UpdateSets::TimersTick));
    app.add_systems(Update, (player_state).in_set(UpdateSets::StateManagement));
    app.register_type::<PlayerActions>();
}

#[derive(Component, Default)]
#[require(PlayerActions, PlayerStateTransition)]
pub struct PlayerState;

#[derive(Component, Default, Clone, PartialEq, Eq, Debug, Reflect)]
pub enum PlayerActions {
    #[default]
    Idle,
    Run,
    Jump,
    Fall,
}

#[derive(Component, Default)]
pub struct PlayerStateTransition(pub PlayerActions);

impl PlayerStateTransition {
    pub fn new(state: PlayerActions) -> Self {
        Self(state)
    }
}

/// System validates states that are determined in the last [`Update`]
/// If ther current player state is no longer valid for animation,
/// i.e. timer has finished, the player state system is allowed to run.
fn validate_state(
    mut player: Query<(Entity, &mut AnimationConfig), With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let (player, mut config) = get_single_mut!(player);
    let mut should_transition = false;
    if let Some(timer) = &config.animation_timer {
        let mut timer = timer.clone();
        timer.tick(time.delta());
        should_transition = timer.finished();
        config.animation_timer = Some(timer);
    }

    // Handle a state change
    if should_transition {
        commands.entity(player).remove::<AnimateOnce>();
    }
}

/// State management based on input and the character controller.
/// This system does not run if thes state is set to animate once.
fn player_state(
    mut player: Query<
        (
            Entity,
            &TnuaController,
            &HorizontalDirection,
            &mut PlayerActions,
        ),
        Without<AnimateOnce>,
    >,
    mut commands: Commands,
) {
    let (player, controller, dir, state) = get_single_mut!(player);
    let old = state.clone();

    let mut new = PlayerActions::default();
    if dir.0 != 0. {
        new = PlayerActions::Run;
    }
    if let Ok(airborne) = controller.is_airborne() {
        new = if airborne { PlayerActions::Fall } else { new };
    }
    if let Some(action) = controller.action_flow_status().ongoing() {
        new = if action == "TnuaBuiltinJump" {
            PlayerActions::Jump
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
