#![allow(dead_code)]
#![allow(unused_imports)]

pub(crate) use avian2d::prelude::*;
pub(crate) use bevy::prelude::*;
pub(crate) use bevy_ecs_ldtk::prelude::*;

pub(crate) const GAME_W: f32 = 640.;
pub(crate) const GAME_H: f32 = 360.;

#[macro_export]
macro_rules! get_single {
    ($q: expr) => {
        match $q.get_single() {
            Ok(m) => m,
            _ => return,
        }
    };
}

pub(crate) use get_single;
