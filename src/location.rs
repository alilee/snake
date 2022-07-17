use bevy::prelude::*;
use crate::direction::{Direction};

type Coord = u8;

const MAX_X: Coord = 15;
const MAX_Y: Coord = 8;

#[derive(Component, Debug)]
pub struct Location {
    x: Coord,
    y: Coord,
}

impl Location {
    pub fn new() -> Self {
        Self { x: 7, y: 4 }
    }

    pub fn move_by_one(&mut self, dir: &Direction) -> Result<(), ()> {
        match dir {
            Direction::Up => try_add(&mut self.y, -1, MAX_Y),
            Direction::Down => try_add(&mut self.y, 1, MAX_Y),
            Direction::Left => try_add(&mut self.x, -1, MAX_X),
            Direction::Right => try_add(&mut self.x, 1, MAX_X),
        }
    }
}

fn try_add(coord: &mut Coord, amount: i8, max: Coord) -> Result<(), ()> {
    let new_value = *coord as i8 + amount;

    if new_value >= 0 && new_value <= max as i8 {
        *coord = new_value as Coord;
        Ok(())
    } else {
        Err(())
    }
}
