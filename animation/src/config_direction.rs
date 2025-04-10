use crate::prelude::*;

/// Hooks into any component that records/reads input.
/// It's required that this be updated in the main
/// app itself since thir modules does not keep track of inputs
#[derive(Component, Reflect, Default)]
pub struct ConfigDirection(Vec2);

impl ConfigDirection {
    /// Returns configured x direction from input
    pub fn x(&self) -> f32 {
        return self.0.x;
    }
    /// Returns configured y direction from input
    pub fn y(&self) -> f32 {
        return self.0.y;
    }
    /// Set the intenal data of the direction configuration
    pub fn set(&mut self, x: Option<f32>, y: Option<f32>) {
        if let Some(x) = x {
            self.0.x = x;
        }
        if let Some(y) = y {
            self.0.y = y;
        }
    }
}
