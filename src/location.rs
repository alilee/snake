use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Location {
    pub coord: UVec2,
}

impl Location {
    pub fn new(coord: UVec2) -> Self {
        Self { coord }
    }
}