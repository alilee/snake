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

    pub(crate) fn starting(board: &Board) -> Self {
        Self::new(board.limit / 2)
    }
}
