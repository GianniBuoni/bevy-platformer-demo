use crate::prelude::*;

// TODO define movement and constraints for entity movements

#[derive(Bundle, Default)]
pub struct OrthoMovementBundle {
    velocity: LinearVelocity,
    direction: HorizontalDirection,
}
