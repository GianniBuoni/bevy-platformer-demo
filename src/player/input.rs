use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, player_input.in_set(UpdateSets::RecordInput));
}

#[derive(Component, Default)]
pub struct PlayerInput {
    pub x: f32,
    pub jump: bool,
}

fn player_input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut PlayerInput, With<Player>>,
) {
    let mut input = get_single_mut!(player);
    let left: f32 = if kb_input.any_pressed([
        KeyCode::ArrowLeft,
        KeyCode::KeyH,
        KeyCode::KeyW,
    ]) {
        1.
    } else {
        0.
    };
    let right: f32 = if kb_input.any_pressed([
        KeyCode::ArrowRight,
        KeyCode::KeyL,
        KeyCode::KeyS,
    ]) {
        1.
    } else {
        0.
    };
    input.x = right - left;
    input.jump = kb_input.just_pressed(KeyCode::Space);
}
