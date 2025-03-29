use crate::prelude::*;

// TODO define movement and constraints for entity movements

#[derive(Component, Default)]
#[require(LinearVelocity, HorizontalDirection)]
pub struct OrthoMovement;
