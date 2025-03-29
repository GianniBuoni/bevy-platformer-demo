use bevy_tnua::{
    TnuaGhostSensor, TnuaProximitySensor,
    control_helpers::TnuaSimpleFallThroughPlatformsHelper,
};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (platform_jump, platform_fall_through)
            .in_set(TnuaUserControlsSystemSet),
    );
}

const MIN_PROXIMITY: f32 = 10.;
fn platform_jump(
    mut query: Query<(&mut TnuaProximitySensor, &TnuaGhostSensor)>,
) {
    query
        .iter_mut()
        .for_each(|(mut proxitmy_sensor, ghost_sensors)| {
            for platform in ghost_sensors.iter() {
                if platform.proximity >= MIN_PROXIMITY {
                    proxitmy_sensor.output = Some(platform.clone());
                    break;
                }
            }
        });
}

fn platform_fall_through(
    mut query: Query<(
        &mut TnuaSimpleFallThroughPlatformsHelper,
        &mut TnuaProximitySensor,
        &TnuaGhostSensor,
    )>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut helper, mut proximity_sensor, ghost_sensor) =
        get_single_mut!(query);
    let mut handle =
        helper.with(proximity_sensor.as_mut(), ghost_sensor, MIN_PROXIMITY);

    if kb_input.pressed(KeyCode::ArrowDown) {
        handle.try_falling(kb_input.just_pressed(KeyCode::ArrowDown));
    } else {
        handle.dont_fall();
    }
}
