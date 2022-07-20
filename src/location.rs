use bevy::math::uvec2;
use bevy::prelude::*;
use crate::board::Board;

#[derive(Component, Clone)]
pub struct Location {
    pub coord: UVec2,
}

impl Location {
    pub fn new(coord: UVec2) -> Self {
        Self { coord }
    }
}
