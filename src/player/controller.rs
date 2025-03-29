use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, player_controls.in_set(UpdateSets::Update));
}

fn player_controls(
    mut player: Query<
        (&mut TnuaController, &HorizontalDirection, &PlayerInput),
        With<Player>,
    >,
) {
    let (mut controller, dir, input) = get_single_mut!(player);
    let input_vector: Vec3 = Vec3::new(dir.0, 0., 0.);

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: input_vector * 100.,
        desired_forward: Dir3::new(input_vector).ok(),
        float_height: 16.,
        acceleration: 200.,
        ..default()
    });

    if input.jump {
        controller.action(TnuaBuiltinJump {
            height: 64.,
            ..default()
        })
    }
}
