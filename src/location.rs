use bevy::prelude::*;
use crate::direction::{Direction};
use crate::board::*;

#[derive(Component, Default, Clone)]
pub struct Location {
    pub x: XCoord,
    pub y: YCoord,
}

impl Location {
    pub fn adjacent(&self, dir: &Direction) -> Self {
        let mut result = self.clone();

        match dir {
            Direction::Up => result.y += 1,
            Direction::Down => result.y -= 1,
            Direction::Left => result.x -= 1,
            Direction::Right => result.x += 1,
        }

        result
    }
}
