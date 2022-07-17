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
        Self { x: MAX_X / 2, y: MAX_Y / 2 }
    }

    pub fn move_by_one(&mut self, dir: &Direction) -> Result<(), ()> {
        match dir {
            Direction::Up => try_add(&mut self.y, -1, MAX_Y),
            Direction::Down => try_add(&mut self.y, 1, MAX_Y),
            Direction::Left => try_add(&mut self.x, -1, MAX_X),
            Direction::Right => try_add(&mut self.x, 1, MAX_X),
        }
    }

    pub fn get_translation(&self) -> Vec3 {
        Vec3::new(self.x as f32 * 80.0 - 640.0, (9 - self.y) as f32 * 80.0 - 360.0, 1.0)
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
