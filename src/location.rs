use bevy::prelude::*;
use crate::direction::{Direction};
use crate::board::*;

pub type Coord = usize;
pub type XCoord = Coord;
pub type YCoord = Coord;

#[derive(Component, Default, Clone)]
pub struct Location {
    pub coord: UVec2,
}

impl Location {
    pub fn new(coord: UVec2) -> Self {
        Self {
            coord,
        }
    }
}