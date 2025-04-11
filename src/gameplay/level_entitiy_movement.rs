use crate::prelude::*;
use animation::prelude::ConfigDirection;

// TODO define movement and constraints for entity movements

#[derive(Component, Default)]
#[require(LinearVelocity, ConfigDirection)]
pub struct OrthoMovement;
