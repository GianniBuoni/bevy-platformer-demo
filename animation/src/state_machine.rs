use std::time::Duration;

use crate::prelude::*;

/// Implement this trait to include this into the
/// animation plugin's state managment.
/// Each compoment that implement this should also be
/// used whith the `validate_animation` system.
pub trait StateMachine {
    fn validate(&self, delta: Duration) -> bool;
    fn next(&self) -> Transition;
}

/// System for validating an animation state,
/// the User must add a new system for each state component
/// that they wish to check and validate.
pub fn validate_animation<T>(
    mut query: Query<(Entity, &T)>,
    mut commands: Commands,
    time: Res<Time>,
) where
    T: StateMachine + Component,
{
    query.iter_mut().for_each(|(entity, state)| {
        let delta = time.delta();
        if !state.validate(delta) {
            commands.entity(entity).insert(state.next()).remove::<T>();
        }
    });
}
